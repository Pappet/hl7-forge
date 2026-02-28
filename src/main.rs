mod hl7;
mod mllp;
mod store;
mod web;

use mllp::MllpStats;
use store::MessageStore;
use tracing::{info, warn};
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
    info!(
        "║          HL7 Forge v{}                ║",
        env!("CARGO_PKG_VERSION")
    );
    info!("╠══════════════════════════════════════════╣");
    info!("║  MLLP Server:  0.0.0.0:{}              ║", mllp_port);
    info!("║  Web UI:       http://localhost:{}     ║", web_port);
    info!("╚══════════════════════════════════════════╝");

    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

    // Start MLLP server
    let mllp_store = store.clone();
    let mllp_stats = stats.clone();
    let mllp_shutdown = shutdown_rx.clone();
    let mllp_handle = tokio::spawn(async move {
        let addr = format!("0.0.0.0:{}", mllp_port);
        if let Err(e) = mllp::start_mllp_server(&addr, mllp_store, mllp_stats, mllp_shutdown).await
        {
            warn!("MLLP server error: {}", e);
        }
    });

    // Start Web server
    let app_state = AppState {
        store: store.clone(),
        stats: stats.clone(),
        mllp_port,
    };
    let app = create_router(app_state);
    let web_addr = format!("0.0.0.0:{}", web_port);
    let listener = tokio::net::TcpListener::bind(&web_addr).await?;
    let web_shutdown = shutdown_rx.clone();
    let web_handle = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let mut rx = web_shutdown;
                let _ = rx.changed().await;
            })
            .await
            .expect("Web server failed");
    });

    // Wait for a shutdown signal or an unexpected server exit
    tokio::select! {
        _ = mllp_handle => {
            warn!("MLLP server stopped unexpectedly, initiating shutdown");
        }
        _ = web_handle => {
            warn!("Web server stopped unexpectedly, initiating shutdown");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down gracefully");
        }
    }

    let _ = shutdown_tx.send(true);
    info!("HL7 Forge stopped.");
    Ok(())
}
