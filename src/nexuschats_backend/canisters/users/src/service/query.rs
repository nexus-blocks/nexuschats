use ic_cdk::query;
use crate::USERS;

use crate::service::user_types::{UserProfile, Error};

fn _get_user_profile(id: &String) -> Option<UserProfile> {
    USERS.with(|s| s.borrow().get(id))
}


#[query]
fn get_user_profile(id: String) -> Result<UserProfile, Error> {
    match _get_user_profile(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("a user with id={} not found", id),
        }),
    }
}

#[query]
fn get_all_users() -> Vec<UserProfile> {
    USERS.with(|s| s.borrow().iter().map(|(_, v)| v.clone()).collect())
}