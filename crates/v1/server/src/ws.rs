// src/ws.rs

/*use axum::extract::{WebSocketUpgrade, Path, State};
use axum::response::IntoResponse;

use crate::state::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(game_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        // handle socket connection
    })
}
*/