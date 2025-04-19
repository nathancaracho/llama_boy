use std::{
    fs::File,
    io::Read,
    sync::{Arc, mpsc},
};

use axum::{Extension, Router, routing::get};

use tokio::sync::broadcast;
use tracing::info;

use crate::{domain::emulator_cmd::spawn_emulator_task, io::channels::video::handle_socket};

pub async fn start_server() -> anyhow::Result<()> {
    // Cria canal de broadcast de frames (saída do GBA)
    let (tx_frame, _) = broadcast::channel::<Vec<u8>>(8);

    // Spawn da thread do emulador
    spawn_emulator_task(tx_frame.clone())?;

    let app = Router::new()
        .route("/health", get(|| async { "I’m alive :p" }))
        .route("/video/ws", get(handle_socket))
        .layer(Extension(Arc::new(tx_frame))); // ✅ Arc para compartilhar entre conexões

    let url = "0.0.0.0:8001";
    info!("🚀 The Llama Boy server was started at http://{}", url);

    let listener = tokio::net::TcpListener::bind(url).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
