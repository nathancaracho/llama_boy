use axum::{
    Extension,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use core::keypad;
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tracing::info;

pub async fn handle_socket(
    ws: WebSocketUpgrade,
    Extension(tx): Extension<Arc<Sender<keypad::Keys>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_control_socket(socket, tx))
}

async fn handle_control_socket(mut socket: WebSocket, tx: Arc<Sender<keypad::Keys>>) {
    while let Some(Ok(Message::Text(text))) = socket.next().await {
        match text.as_str() {
            "up" => {
                let _ = tx.send(keypad::Keys::Up).await;
            }
            "down" => {
                let _ = tx.send(keypad::Keys::Down).await;
            }
            "left" => {
                let _ = tx.send(keypad::Keys::Left).await;
            }
            "right" => {
                let _ = tx.send(keypad::Keys::Right).await;
            }
            "a" => {
                info!("press A");
                let _ = tx.send(keypad::Keys::ButtonA).await;
            }
            "b" => {
                let _ = tx.send(keypad::Keys::ButtonB).await;
            }
            "l" => {
                let _ = tx.send(keypad::Keys::ButtonL).await;
            }
            "r" => {
                let _ = tx.send(keypad::Keys::ButtonR).await;
            }
            "start" => {
                let _ = tx.send(keypad::Keys::Start).await;
            }
            "select" => {
                let _ = tx.send(keypad::Keys::Select).await;
            }
            _ => eprintln!("Comando desconhecido: {}", text),
        }
    }
}
