

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct UserProfile {
    pub principal_id: String,
    pub id: String,
    pub profile_body: Option<ProfileBody>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct UserProfilePayload {
    pub id: String,
    pub principal_id: String,
    pub profile_body: Option<ProfileBody>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct ProfileBody {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub canister_id: Option<String>,
    pub topics: Vec<Topic>,
    pub platforms_following: Vec<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Topic {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cover_image: String,
    pub created_at: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct TopicPayload {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cover_image: String,
}
