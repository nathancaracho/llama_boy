use core::keypad;
use std::{
    fs::File,
    io::Read,
    sync::{Arc},
};
use tokio::sync::mpsc;
use axum::{Extension, Router, routing::get};

use tokio::sync::broadcast;
use tracing::info;

use crate::{
    domain::emulator::{engine::start_emulator_loop, queries::EmulatorQuery},
    io::channels::{command, memory, video},
};

pub async fn start_server() -> anyhow::Result<()> {
    let (tx_query, _) = broadcast::channel::<EmulatorQuery>(16);
    let (tx_command, mut rx_command) = tokio::sync::mpsc::channel::<keypad::Keys>(16);
    start_emulator_loop(tx_query.clone(), rx_command)?;

    let app = Router::new()
        .route("/health", get(|| async { "I’m alive :p" }))
        .route("/video/ws", get(video::handle_socket))
        .route("/memory/ws", get(memory::handle_socket))
        .route("/command/ws", get(command::handle_socket))
        .layer(Extension(Arc::new(tx_query)))
        .layer(Extension(Arc::new(tx_command))); // ✅ Arc para compartilhar entre conexões

    let url = "0.0.0.0:8080";
    info!("🚀 The Llama Boy server was started at http://{}", url);

    let listener = tokio::net::TcpListener::bind(url).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
