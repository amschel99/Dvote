#[macro_use]
extern crate serde;
use candid::Principal;

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
    static USER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a  user counter")
    );
    static ARTICLE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create am articles  counter")
    );




}
#[ic_cdk::update]
fn _signup(name: Option<String>) -> String {
    let id = USER_ID_COUNTER.with(|counter| {
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
#[ic_cdk::update]
fn _publish_article(content: String, user_principal: Principal) -> Result<Article, Error> {
    let id = ARTICLE_ID_COUNTER.with(|counter| {
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
    match ARTICLES.with(|db| db.borrow_mut().insert(id, article)) {
        Some(article) => Ok(article),
        None => Err(Error::NotFound {
            msg: format!("Could not publish the article!"),
        }),
    }
}
#[ic_cdk::query]
fn _get_all_articles() -> Vec<(u64, Article)> {
    let articles: Vec<_> = ARTICLES.with(|storage| storage.borrow().iter().collect());
    articles
}
#[ic_cdk::query]
fn get_single_article(id: u64) -> Result<Article, Error> {
    match ARTICLES.with(|storage| storage.borrow().get(&id)) {
        Some(article) => Ok(article),
        None => Err(Error::NotFound {
            msg: format!("That particular article was not found!"),
        }),
    }
}
#[ic_cdk::query]

fn get_all_writers() -> Vec<(u64, User)> {
    let writers: Vec<_> = USERS.with(|storage| storage.borrow().iter().collect());
    writers
}
#[ic_cdk::query]
fn get_single_writer(id: u64) -> Result<User, Error> {
    match USERS.with(|storage| storage.borrow().get(&id)) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("That particular user was not found!"),
        }),
    }
}
#[ic_cdk::update]
fn upvote_article(id: u64) -> String {
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
        "You can not upvote more than once".to_string()
    } else {
        temp_principal_store.push(ic_cdk::caller());

        rest_article.votes.count += 1;
        rest_article.votes.principals = Some(temp_principal_store);

        article.replace(rest_article.clone());
        "succesfully upvoted the article".to_string()
    }
}
#[ic_cdk::update]
fn delete_article(id: u64) -> String {
    let article = get_single_article(id).unwrap();
    if article.publisher != ic_cdk::caller() {
        format!("You are not Authorized to delete this article")
    } else {
        ARTICLES.with(|storage| storage.borrow_mut().remove(&id));
        format!("Article with ID {} deleted!", id)
    }
}
#[derive(candid::CandidType, Deserialize, Serialize, Debug)]
enum Error {
    NotFound { msg: String },
}

ic_cdk::export_candid!();
