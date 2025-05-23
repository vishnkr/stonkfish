// models.rs
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub userId: String,
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

#[derive(Clone)]
pub struct Player {
    pub user_id: String,
    pub game_id: String,
    pub color_pref: Option<String>,
    //pub sender: UnboundedSender<tungstenite::Message>,
}
