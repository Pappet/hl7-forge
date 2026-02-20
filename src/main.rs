mod hl7;
mod mllp;
mod store;
mod web;

use mllp::MllpStats;
use store::MessageStore;
use tracing::info;
use tracing_subscriber::EnvFilter;
use web::{create_router, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let mllp_port: u16 = std::env::var("MLLP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(2575);

    let web_port: u16 = std::env::var("WEB_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let store = MessageStore::new();
    let stats = MllpStats::new();

    info!("╔══════════════════════════════════════════╗");
    info!("║          HL7 Forge v0.1.0                ║");
    info!("╠══════════════════════════════════════════╣");
    info!("║  MLLP Server:  0.0.0.0:{}             ║", mllp_port);
    info!("║  Web UI:       http://localhost:{}     ║", web_port);
    info!("╚══════════════════════════════════════════╝");

    // Start MLLP server
    let mllp_store = store.clone();
    let mllp_stats = stats.clone();
    let mllp_handle = tokio::spawn(async move {
        let addr = format!("0.0.0.0:{}", mllp_port);
        mllp::start_mllp_server(&addr, mllp_store, mllp_stats)
            .await
            .expect("MLLP server failed");
    });

    // Start Web server
    let app_state = AppState {
        store: store.clone(),
        stats: stats.clone(),
    };
    let app = create_router(app_state);
    let web_addr = format!("0.0.0.0:{}", web_port);
    let listener = tokio::net::TcpListener::bind(&web_addr).await?;
    let web_handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("Web server failed");
    });

    // Wait for both servers
    tokio::select! {
        _ = mllp_handle => {},
        _ = web_handle => {},
    }

    Ok(())
}
