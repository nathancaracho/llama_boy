use axum::{
    Extension,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use tokio::sync::broadcast;
use tracing::info;

use std::sync::Arc;

use crate::domain::emulator::queries::EmulatorQuery;

pub async fn handle_socket(
    ws: WebSocketUpgrade,
    Extension(tx): Extension<Arc<broadcast::Sender<EmulatorQuery>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| socket_handler(socket, tx))
}

async fn socket_handler(mut socket: WebSocket, tx: Arc<broadcast::Sender<EmulatorQuery>>) {
    info!("Video Start websocket");

    let mut rx = tx.subscribe();

    while let Ok(event) = rx.recv().await {
        match event {
            EmulatorQuery::Frame(frame_data) => {
                let encoded = base64::encode(&frame_data);
                if socket.send(Message::Text(encoded.into())).await.is_err() {
                    info!("Video WebSocket send error, closing connection");
                    break;
                }
            }
            _ => {}
        }
    }

    info!("Video WebSocket disconnected");
}
