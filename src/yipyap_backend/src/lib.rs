use std::{borrow::BorrowMut, cell::RefCell};

use ic_cdk::{query, update};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap, DefaultMemoryImpl,
};

type Memory = VirtualMemory<DefaultMemoryImpl>;

type VOTE = (String, u32);

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));


    static MAP: RefCell<BTreeMap<String, u32, Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
    static QUESTION:RefCell<String>=RefCell::new(String::from("What is your favorite programming languange?"));


}

#[query]
fn get_question() -> String {
    let res = QUESTION.with(|q| q.borrow().to_string());
    res
}

#[query]

fn get_votes() -> Option<Vec<VOTE>> {
    let borrowed_value: Vec<_> = MAP.with(|map| map.borrow().iter().collect());
    Some(borrowed_value)
}

#[update]
fn vote(entry: String) {
    let borrowed_value: Vec<_> = MAP.with(|map| map.borrow().iter().collect());
    let matching_val: Vec<_> = borrowed_value.iter().filter(|val| val.0 == entry).collect();
    if matching_val.len() == 0 {
        MAP.with(|m| m.borrow_mut().insert(entry, 1));
        //there is no such key so create it and add 1 vote
    }
}

#[test]
fn question_test() {
    let res = QUESTION.with(|q| q.borrow().to_string());

    assert_eq!("What is your favorite programming languange?", dbg!(res));
}
#[test]
fn vote_for_new_languange_works() {
    /** test for adding new lang */
    let borrowed_value: Vec<_> = MAP.with(|map| map.borrow().iter().collect());
    let matching_val: Vec<_> = borrowed_value
        .iter()
        .filter(|val| val.0 == "Rust".to_string())
        .collect();
    if matching_val.len() == 0 {
        MAP.with(|m| m.borrow_mut().insert("Rust".to_string(), 1));
        let new_votes: Vec<_> = MAP.with(|map| map.borrow().iter().collect());

        assert_eq!(new_votes.get(0), Some(&("Rust".to_string(), 1)));
    }
}
