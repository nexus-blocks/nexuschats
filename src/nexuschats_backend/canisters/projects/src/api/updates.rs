use std::borrow::Cow;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::time;
use ic_cdk::update;
use crate::PROJECTS;

use ic_cdk::api::management_canister::main::{CanisterInstallMode, CreateCanisterArgument, InstallCodeArgument};
use ic_cdk::api::{call, id};
use candid::{encode_args, Principal};

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

#[update]
fn update_project(project : ProjectPayload) -> Project {
    let project = Project {
        id: project.id,
        name: project.name,
        user_id: ic_cdk::caller().to_string(),
        chat_canister_id: None,
        group_canister_id: None,
        description: project.description,
        created_at: time().to_string(),
        updated_at: Some(time().to_string()),
    };
    PROJECTS.with(|s| {
        s.borrow_mut()
            .insert(project.id.clone(), project.clone())
    });
    project
}

#[ic_cdk::update]
async fn create_new_canister() -> Result<Principal, String> {
    let create_args = CreateCanisterArgument {
        settings: None,
    };

    match call(Principal::management_canister(), "create_canister", (create_args,)).await {
        Ok((canister_id,)) => Ok(canister_id),
        Err((_, err)) => Err(format!("Failed to create canister: {}", err)),
    }
}
