#[macro_use]
extern crate serde;
use std::{borrow::Cow, cell::RefCell, fmt::format};

use candid::{Decode, Encode, Principal};
use ic_cdk::{query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap, Cell, DefaultMemoryImpl,
};
mod article;
mod user;
use article::Article;
use user::User;
type Memory = VirtualMemory<DefaultMemoryImpl>;

type IdCell = Cell<u64, Memory>;
thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));


    static ARTICLES: RefCell<BTreeMap< u64,Article,Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
    static USERS: RefCell<BTreeMap< u64,User,Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );



}

fn _signup(name: Option<String>) -> String {
    let id = ID_COUNTER.with(|counter| {
        let counter_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(counter_value + 1);
        counter_value
    });
    let user = User {
        principal: ic_cdk::caller(),
        name: name.clone(),
        id,
    };

    USERS.with(|storage| storage.borrow_mut().insert(id, user));

    format!(
        "{}, was signed up!",
        name.unwrap_or_else(|| "No name was provided".to_string())
    )
}

fn _publish_article(content: String, user_principal: Principal) -> Option<Article> {
    let id = ID_COUNTER.with(|counter| {
        let counter_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(counter_value + 1);
        counter_value
    });
    let users: Vec<_> = USERS.with(|storage| {
        storage
            .borrow()
            .iter()
            .filter(|user| user.clone().1.principal == user_principal)
            .collect()
    });
    let publisher = users.get(0).unwrap().1.principal;
    let article = Article {
        publisher,
        content,
        votes: 0,
        id,
    };
    ARTICLES.with(|db| db.borrow_mut().insert(id, article))
}

fn _get_all_articles() -> Vec<(u64, Article)> {
    let articles: Vec<_> = ARTICLES.with(|storage| storage.borrow().iter().collect());
    articles
}

fn get_single_article(id: u64) -> Option<Article> {
    ARTICLES.with(|storage| storage.borrow().get(&id))
}

fn get_all_writers() -> Vec<(u64, User)> {
    let writers: Vec<_> = USERS.with(|storage| storage.borrow().iter().collect());
    writers
}
fn get_single_writer(id: u64) -> Option<User> {
    USERS.with(|storage| storage.borrow().get(&id))
}

fn upvote_article(id: u64) -> Option<Article> {
    let mut article = ARTICLES.with(|storage| storage.borrow().get(&id));
    let rest_article = article.as_ref().unwrap().clone();
    let new_article = Article {
        votes: rest_article.votes + 1,
        ..rest_article
    };
    article.replace(new_article)
}

fn delete_article(id: u64) -> String {
    let article = get_single_article(id).unwrap();
    if article.publisher != ic_cdk::caller() {
        format!("You are not Authorized to delete this article")
    } else {
        ARTICLES.with(|storage| storage.borrow_mut().remove(&id));
        format!("Article with ID {} deleted!", id)
    }
}
