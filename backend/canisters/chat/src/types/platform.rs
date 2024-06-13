#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum Error {
    NotFound { msg: String },
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Platform {
    pub id: String,
    pub name: String,
    pub cover_image: String,
    pub canister_id: String,
    pub created_at: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct PlatformPayload {
    pub id: String,
    pub name: String,
    pub cover_image: String,
}

