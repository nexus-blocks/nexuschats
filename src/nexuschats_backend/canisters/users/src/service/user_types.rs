use serde::{Deserialize, Serialize};

#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum Error {
    NotFound { msg: String },
}

pub type UserId = String;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct UserProfile {
    pub principal_id: String,
    pub username: String,
    pub email: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct UserProfilePayload {
   pub username: String,
}
