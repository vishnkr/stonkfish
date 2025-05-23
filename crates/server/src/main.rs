
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::{net::TcpListener};
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use std::sync::Arc;
use futures::{StreamExt, SinkExt}; 


mod state;
mod models;
mod ws;
mod auth;


#[tokio::main]
async fn main() {
    // Create a shared broadcast channel for messages
    let (tx, _rx) = broadcast::channel::<String>(100);
    let state = Arc::new(Mutex::new(tx));
    // Build the Axum application
    let app = Router::new()
        .route("/ws", get({
            let state: Arc<Mutex<broadcast::Sender<String>>> = state.clone();
            move |ws: WebSocketUpgrade| ws_handler(ws, state.clone())
        }));
    // Start the server
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener,app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    state: Arc<Mutex<broadcast::Sender<String>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}


async fn handle_socket(
    socket: WebSocket,
    state: Arc<Mutex<broadcast::Sender<String>>>,
) {
    let tx = state.lock().await.clone();
    let mut rx = tx.subscribe();


    let (mut sender, mut receiver) = socket.split();


    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("sending {}",msg);
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });


    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            println!("got {}",text);
            let _ = tx.send(text);
        }
    }


    send_task.abort();
}