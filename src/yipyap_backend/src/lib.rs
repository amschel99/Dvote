#[macro_use]
extern crate serde;
use candid::Principal;
use ic_cdk::{query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap, Cell, DefaultMemoryImpl,
};

use std::cell::RefCell;
mod article;
mod user;
use article::{Article, VOTE};
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
    let votes = VOTE {
        count: 0,
        principals: None,
    };
    let article = Article {
        publisher,
        content,
        votes,
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
    let mut rest_article = article.as_ref().unwrap().clone();
    let mut temp_principal_store = rest_article.clone().votes.principals.unwrap();

    /*Check that a user does not vote twice */

    let p: Vec<_> = rest_article
        .clone()
        .votes
        .principals
        .unwrap()
        .into_iter()
        .filter(|p| *p == ic_cdk::caller())
        .collect();

    if p.len() != 0 {
        // RETURN NOTHING
        None
    } else {
        temp_principal_store.push(ic_cdk::caller());

        rest_article.votes.count += 1;
        rest_article.votes.principals = Some(temp_principal_store);

        article.replace(rest_article.clone())
    }
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
