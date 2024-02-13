use std::borrow::Cow;

use candid::{Decode, Encode, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct Article {
    publisher: Principal,
    content: String,
    votes: u128,
}
impl Article {
    pub fn new(content: String, publisher: Principal) -> Self {
        Self {
            content,
            votes: 0,
            publisher,
        }
    }
}
impl Storable for Article {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Article {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}
