use std::borrow::Cow;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_cdk::update;
use crate::USERS;

use crate::service::user_types::{UserProfile, UserProfilePayload};
use ic_stable_structures::Storable;
use ic_stable_structures::storable::Bound;


impl Storable for UserProfile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}


#[update]
fn update_user_profile(user_profile: UserProfile) -> UserProfile {
    USERS.with(|s| {
        s.borrow_mut()
            .insert(user_profile.principal_id.clone(), user_profile.clone())
    });
    user_profile
}

#[update]
fn delete_user_profile(id: String) -> bool {
    USERS.with(|s| s.borrow_mut().remove(&id));
    true
}

#[update]
fn add_user_profile(_user_profile: UserProfilePayload) -> UserProfile {
    let user_profile = UserProfile {
        principal_id: ic_cdk::caller().to_string(),
        username: _user_profile.username,
        email: None,
        created_at: time().to_string(),
        updated_at: None,
    };
    USERS.with(|s| {
        s.borrow_mut()
            .insert(user_profile.principal_id.clone(), user_profile.clone())
    });
    user_profile
}