use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

async fn websocket_connection(socket: WebSocket, state: AppState) {
    let client_id = Uuid::new_v4();
    info!("WebSocket client {} connected", client_id);

    let mut weather_rx = state.weather_tx.subscribe();
    let (ws_sender, mut ws_receiver) = socket.split();

    // Create a channel to communicate between tasks
    let (sender_tx, mut sender_rx) = mpsc::unbounded_channel::<Message>();

    // Task to handle outgoing messages
    let sender_task = tokio::spawn(async move {
        let mut ws_sender = ws_sender;
        while let Some(msg) = sender_rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Clone sender_tx for the ping task
    let ping_sender = sender_tx.clone();

    // Task to handle incoming messages from client
    let ping_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if text == "ping" {
                        if ping_sender.send(Message::Text("pong".to_string())).is_err() {
                            break;
                        }
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Task to handle outgoing weather updates
    let broadcast_task = tokio::spawn(async move {
        while let Ok(weather_update) = weather_rx.recv().await {
            if sender_tx.send(Message::Text(weather_update)).is_err() {
                break;
            }
        }
    });

    // Wait for any task to complete
    tokio::select! {
        _ = ping_task => warn!("WebSocket client {} ping task ended", client_id),
        _ = broadcast_task => warn!("WebSocket client {} broadcast task ended", client_id),
        _ = sender_task => warn!("WebSocket client {} sender task ended", client_id),
    }

    info!("WebSocket client {} disconnected", client_id);
}
