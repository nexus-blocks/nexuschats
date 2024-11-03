#[macro_use]
extern crate serde;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

mod service;

use service::updates::UserProfile;

use types::{Error, UserProfilePayload};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static USERS: RefCell<StableBTreeMap<String, UserProfile, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

}

fn _get_user_profile(id: &String) -> Option<UserProfile> {
    USERS.with(|s| s.borrow().get(id))
}


#[ic_cdk::query]
fn get_user_profile(id: String) -> Result<UserProfile, Error> {
    match _get_user_profile(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("a user with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_all_users() -> Vec<UserProfile> {
    USERS.with(|s| s.borrow().iter().map(|(_, v)| v.clone()).collect())
}


// need this to generate candid
ic_cdk::export_candid!();
