
#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{ DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
pub mod types;

use crate::types::platform::*;
use crate::types::user::*;

type Memory = VirtualMemory<DefaultMemoryImpl>;



impl Storable for Platform {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for UserProfile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for Topic {
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
    static PLATFORMS: RefCell<StableBTreeMap<String, Platform, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static USERS: RefCell<StableBTreeMap<String, UserProfile, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static TOPICS: RefCell<StableBTreeMap<String, Topic, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

}

// GET VALUE BY ID HELPER FUNCTIONS

fn _get_user_profile(id: &String) -> Option<UserProfile> {
    USERS.with(|s| s.borrow().get(id))
}

fn _get_platform(id: &String) -> Option<Platform> {
    PLATFORMS.with(|s| s.borrow().get(id))
}

fn _get_topic(id: &String) -> Option<Topic> {
    TOPICS.with(|s| s.borrow().get(id))
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
fn get_platform(id: String) -> Result<Platform, Error> {
    match _get_platform(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("a platform with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_topic(id: String) -> Result<Topic, Error> {
    match _get_topic(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("a topic with id={} not found", id),
        }),
    }
}

// GET ALL VALUES FUNCTIONS

#[ic_cdk::query]
fn get_all_platforms() -> Vec<Platform> {
    PLATFORMS.with(|s| s.borrow().iter().map(|(_, v)| v.clone()).collect())
}

#[ic_cdk::query]
fn get_all_topics() -> Vec<Topic> {
    TOPICS.with(|s| s.borrow().iter().map(|(_, v)| v.clone()).collect())
}

#[ic_cdk::query]
fn get_all_users() -> Vec<UserProfile> {
    USERS.with(|s| s.borrow().iter().map(|(_, v)| v.clone()).collect())
}

// ADD VALUE TO STATE FUNCTIONS

#[ic_cdk::update]
fn add_platform(platform: PlatformPayload) -> Platform {
    let platform = Platform {
        id: platform.id,
        name: platform.name,
        cover_image: platform.cover_image,
        canister_id: "".to_string(),
        created_at: time().to_string(),
    };
    PLATFORMS.with(|s| s.borrow_mut().insert(platform.id.clone(), platform.clone()));
    platform
}

#[ic_cdk::update]
pub fn add_topic(topic: TopicPayload) -> Topic {
    let topic = Topic {
        id: topic.id,
        name: topic.name,
        description: topic.description,
        cover_image: topic.cover_image,
        created_at: time().to_string(),
    };
    TOPICS.with(|s| s.borrow_mut().insert(topic.id.clone(), topic.clone()));
    topic
}

#[ic_cdk::update]
fn add_user_profile(_user_profile: UserProfilePayload) -> UserProfile {
    let user_profile = UserProfile {
        id: _user_profile.id,
        principal_id: _user_profile.principal_id,
        profile_body: _user_profile.profile_body,
        created_at: time().to_string(),
        updated_at: None,
    };
    USERS.with(|s| s.borrow_mut().insert(user_profile.id.clone(), user_profile.clone()));
    user_profile
}

// UPDATE VALUE IN STATE FUNCTIONS

#[ic_cdk::update]
fn update_user_profile(user_profile: UserProfile) -> UserProfile {
    USERS.with(|s| s.borrow_mut().insert(user_profile.id.clone(), user_profile.clone()));
    user_profile
}

#[ic_cdk::update]
fn update_platform(platform: Platform) -> Platform {
    PLATFORMS.with(|s| s.borrow_mut().insert(platform.id.clone(), platform.clone()));
    platform
}

#[ic_cdk::update]
fn update_topic(topic: Topic) -> Topic {
    TOPICS.with(|s| s.borrow_mut().insert(topic.id.clone(), topic.clone()));
    topic
}

#[ic_cdk::update]
fn delete_user_profile(id: String) -> bool {
    USERS.with(|s| s.borrow_mut().remove(&id));
    true
}

#[ic_cdk::update]
fn delete_platform(id: String) -> bool {
    PLATFORMS.with(|s| s.borrow_mut().remove(&id));
    true
}

#[ic_cdk::update]
fn delete_topic(id: String) -> bool {
    TOPICS.with(|s| s.borrow_mut().remove(&id));
    true
}


// need this to generate candid
ic_cdk::export_candid!();

