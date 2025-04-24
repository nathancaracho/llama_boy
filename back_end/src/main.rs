mod domain;
mod infrastructure;
mod io;
use io::server::start_server;
use tracing_subscriber::EnvFilter;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();
    start_server().await?;
    Ok(())
}
