#[macro_use]
extern crate serde;
use std::{borrow::Cow, cell::RefCell};

use candid::{Decode, Encode, Principal};
use ic_cdk::{query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap, DefaultMemoryImpl,
};
mod article;
mod user;
use article::Article;
use user::User;
type Memory = VirtualMemory<DefaultMemoryImpl>;

type VOTE = (String, u32);

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



}
