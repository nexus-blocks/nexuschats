use candid::CandidType;
use serde::{Deserialize, Serialize};


#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub chat_canister_id: Option<String>,
    pub group_canister_id: Option<String>,
    pub description: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectPayload {
    pub name: String,
    pub id: String,
    pub description: String,
}