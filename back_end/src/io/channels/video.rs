use axum::{
    Extension, Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use tokio::sync::broadcast;
use tracing::info;

use std::sync::Arc;

pub async fn handle_socket(
    ws: WebSocketUpgrade,
    Extension(tx): Extension<Arc<broadcast::Sender<Vec<u8>>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| socket_handler(socket, tx))
}

async fn socket_handler(mut socket: WebSocket, tx: Arc<broadcast::Sender<Vec<u8>>>) {
    info!("🔌 WebSocket connected");

    // Cada client ganha um receiver próprio
    let mut rx = tx.subscribe();

    loop {
        match rx.recv().await {
            Ok(frame_data) => {
                let encoded = base64::encode(&frame_data);
                if socket.send(Message::Text(encoded.into())).await.is_err() {
                    info!("❌ WebSocket send error, closing connection");
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                tracing::warn!("⚠️ Skipped {} frames", skipped);
            }
            Err(_) => break, // closed
        }
    }

    info!("📴 WebSocket disconnected");
}
