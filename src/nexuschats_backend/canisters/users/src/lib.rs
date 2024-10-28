#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

use types::*;
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct UserProfile(Profile);

type Memory = VirtualMemory<DefaultMemoryImpl>;

impl Storable for UserProfile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

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

// GET VALUE BY ID FUNCTIONS

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

#[ic_cdk::update]
fn add_user_profile(_user_profile: UserProfilePayload) -> UserProfile {
    let user_profile = UserProfile {
        id: _user_profile.id,
        principal_id: _user_profile.principal_id,
        profile_body: None,
        created_at: time().to_string(),
        updated_at: None,
    };
    USERS.with(|s| {
        s.borrow_mut()
            .insert(user_profile.id.clone(), user_profile.clone())
    });
    user_profile
}

// UPDATE VALUE IN STATE FUNCTIONS

#[ic_cdk::update]
fn update_user_profile(user_profile: UserProfile) -> UserProfile {
    USERS.with(|s| {
        s.borrow_mut()
            .insert(user_profile.id.clone(), user_profile.clone())
    });
    user_profile
}

#[ic_cdk::update]
fn delete_user_profile(id: String) -> bool {
    USERS.with(|s| s.borrow_mut().remove(&id));
    true
}

// need this to generate candid
ic_cdk::export_candid!();
