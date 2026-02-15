use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub to: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub signature: Vec<u8>,
    pub is_encrypted: bool,
    pub nonce: Option<String>,
    pub message_number: Option<u64>,
    pub reply_to: Option<String>,
    pub edited_at: Option<DateTime<Utc>>,
    pub reactions: Vec<Reaction>,
    pub forwarded_from: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub user_id: String,
    pub emoji: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum MessengerEvent {
    MessageReceived(Message),
    MessageSent(Message),
    UserOnline(String),
    UserOffline(String),
    TypingIndicator(String),
    ReactionAdded { message_id: String, reaction: Reaction },
    MessageEdited(Message),
    SessionEstablished(String),
}
