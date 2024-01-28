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

fn get_votes() -> Result<Vec<VOTE>, Error> {
    let borrowed_value: Option<Vec<_>> = Some(MAP.with(|map| map.borrow().iter().collect()));
    match borrowed_value {
        Some(votes) => Ok(votes),
        None => Err(Error::NotFound {
            msg: format!("There are no votes for now!"),
        }),
    }
}

#[update]
fn vote(entry: String) {
    let borrowed_value: Vec<_> = MAP.with(|map| map.borrow().iter().collect());
    let matching_val: Vec<_> = borrowed_value.iter().filter(|val| val.0 == entry).collect();
    if matching_val.len() == 0 {
        MAP.with(|m| m.borrow_mut().insert(entry, 1));
    }
}
#[derive(candid::CandidType)]
enum Error {
    NotFound { msg: String },
}
#[test]
fn question_test() {
    let res = QUESTION.with(|q| q.borrow().to_string());

    assert_eq!("What is your favorite programming languange?", dbg!(res));
}
#[test]
fn vote_for_new_languange_works() {
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
