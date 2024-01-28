use std::cell::RefCell;

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
    let votes_list: Option<Vec<_>> = Some(MAP.with(|map| map.borrow().iter().collect()));
    match votes_list {
        Some(votes) => Ok(votes),
        None => Err(Error::NotFound {
            msg: format!("There are no votes for now!"),
        }),
    }
}

#[update]
fn vote(entry: String) -> Result<Vec<VOTE>, Error> {
    match _vote(entry) {
        Some(votes) => Ok(votes),
        None => Err(Error::NotFound {
            msg: format!("An error occured!"),
        }),
    }
}
#[update]
fn reset_votes() -> Result<Vec<VOTE>, Error> {
    match _reset_votes() {
        Some(votes) => Ok(votes),
        None => Err(Error::NotFound {
            msg: format!("An error occured!"),
        }),
    }
}
fn _vote(entry: String) -> Option<Vec<VOTE>> {
    let votes_list: Vec<_> = MAP.with(|map| map.borrow().iter().collect());
    let matching_vote: Vec<_> = votes_list.iter().filter(|val| val.0 == entry).collect();
    if matching_vote.len() == 0 {
        MAP.with(|m| m.borrow_mut().insert(entry, 1));
        Some(MAP.with(|map| map.borrow().iter().collect()))
    } else {
        let current_vote_count = MAP.with(|m| m.borrow_mut().get(&entry));

        MAP.with(|m| {
            m.borrow_mut()
                .insert(entry, current_vote_count.unwrap() + 1)
        });
        Some(MAP.with(|map| map.borrow().iter().collect()))
    }
}

fn _reset_votes() -> Option<Vec<VOTE>> {
    let reset_votes: Vec<_> = MAP.with(|map| {
        let map_ref = map.borrow_mut();
        map_ref.iter().map(|value| (value.0, 0)).collect()
    });

    reset_votes.iter().for_each(|vote| {
        MAP.with(|m| m.borrow_mut().insert(vote.clone().0, 0));
    });
    Some(reset_votes)
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
    _vote("Rust".to_string());

    let new_votes: Vec<_> = MAP.with(|map| map.borrow().iter().collect());

    assert_eq!(new_votes.get(0), Some(&("Rust".to_string(), 1)));
}
#[test]
fn vote_for_existing_languange_works() {
    _vote("Motoko".to_string());
    _vote("Motoko".to_string());

    let new_votes: Vec<_> = MAP.with(|map| map.borrow().iter().collect());

    assert_eq!(new_votes.get(0), Some(&("Motoko".to_string(), 2)));
}

#[test]
fn reset_votes_works() {
    _vote("Typescript".to_string());
    _vote("Typescript".to_string());
    _reset_votes();
    let new_votes: Vec<_> = MAP.with(|map| map.borrow().iter().collect());
    assert_eq!(new_votes.get(0), Some(&("Typescript".to_string(), 0)));
}
