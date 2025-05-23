use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::Player;

pub struct Game {
    pub players: DashMap<String, Player>,
}

#[derive(Clone)]
pub struct AppState {
    pub games: Arc<DashMap<String, Arc<Mutex<Game>>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            games: Arc::new(DashMap::new()),
        }
    }

    pub fn get_or_create_game(&self, game_id: &str) -> Arc<Mutex<Game>> {
        self.games
            .entry(game_id.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(Game {
                players: DashMap::new(),
            })))
            .clone()
    }
}
