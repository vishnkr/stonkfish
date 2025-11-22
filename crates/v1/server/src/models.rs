// models.rs
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct ConnectPayload {
    pub token: String,
    pub colorPref: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Join,
    Move,
    Resign,
    DrawOffer,
    DrawAccept,
    DrawReject,
    GameOver,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub gid: String,
    pub uid: String,
    pub t: EventType,
    pub d: serde_json::Value,
}

