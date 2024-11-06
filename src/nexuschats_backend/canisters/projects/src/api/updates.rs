use std::borrow::Cow;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_cdk::update;
use crate::PROJECTS;

use crate::common::types::{Project, ProjectPayload};
use ic_stable_structures::Storable;
use ic_stable_structures::storable::Bound;

impl Storable for Project {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}


#[update]
fn create_project(project : ProjectPayload) -> Project {
    let project = Project {
        id: project.id,
        name: project.name,
        user_id: ic_cdk::caller().to_string(),
        chat_canister_id: None,
        group_canister_id: None,
        description: project.description,
        created_at: time().to_string(),
        updated_at: None,
    };
    PROJECTS.with(|s| {
        s.borrow_mut()
            .insert(project.id.clone(), project.clone())
    });
    project
}

