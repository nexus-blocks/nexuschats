use serde::{Deserialize, Serialize};

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct DirectMessage {
    pub sender: String,
    pub receiver: String,
    pub body: MessageBody,
    pub message_id: String,
    pub created: i64,
    pub edited: bool,
    pub chat_id: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct MessageBody {
    pub text: Option<String>,
    pub image: Option<String>,
    pub video: Option<String>,
}

