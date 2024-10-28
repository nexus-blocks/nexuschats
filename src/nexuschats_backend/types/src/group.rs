use serde::{Deserialize, Serialize};

use crate::chat::MessageBody;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub profile_picture: Option<String>,
    pub creator: GroupCreator,
    pub members: Vec<String>,
    pub admins: Vec<String>,
    pub is_private: bool,
    pub created: i64,
    pub messages: Option<String>,
    pub last_message: Option<String>,
    pub last_message_time: Option<i64>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct GroupCreator {
    pub user_name: String,
    pub profile_picture: Option<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct GroupMessage {
    pub id: String,
    pub sender: String,
    pub body: MessageBody,
    pub created: i64,
    pub edited: bool,
}