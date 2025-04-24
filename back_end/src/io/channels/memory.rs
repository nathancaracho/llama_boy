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
    info!("Start memory websocket");

    let mut rx = tx.subscribe();

    while let Ok(event) = rx.recv().await {
        match event {
            EmulatorQuery::Memory(data) => {
                let json = serde_json::to_string(&data).unwrap_or_else(|_| "{}".into());
                if socket.send(Message::Text(json.into())).await.is_err() {
                    info!("The memory WebSocket send error, closing connection");
                    break;
                }
            }
            _ => {}
        }
    }

    info!("Memory WebSocket disconnected");
}
