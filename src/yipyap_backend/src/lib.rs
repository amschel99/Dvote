use std::cell::RefCell;

use ic_cdk::{query,update};
use ic_stable_structures::{BTreeMap, memory_manager::{MemoryId, MemoryManager, VirtualMemory},DefaultMemoryImpl};

type Memory = VirtualMemory<DefaultMemoryImpl>;

type VOTE=(String, u32);


thread_local! {
   
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

   
    static MAP: RefCell<BTreeMap<String, u32, Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

#[query]
fn get_question()->String {

  String::from("What is your favorite Programming languange")

}

#[query]

fn get_votes() -> Option<Vec<VOTE>> {
    let borrowed_value:Vec<_>=MAP.with(|map|map.borrow().iter().collect());
    Some(borrowed_value)
  
 
}

