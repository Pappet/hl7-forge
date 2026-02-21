use crate::store::{MessageStore, StoreEvent};
use crate::mllp::MllpStats;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Json, Router};
use rust_embed::Embed;
use serde::Deserialize;
use std::sync::atomic::Ordering;

#[derive(Embed)]
#[folder = "static/"]
struct StaticAssets;

#[derive(Clone)]
pub struct AppState {
    pub store: MessageStore,
    pub stats: MllpStats,
    pub mllp_port: u16,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // API routes
        .route("/api/messages", get(list_messages))
        .route("/api/messages/:id", get(get_message))
        .route("/api/search", get(search_messages))
        .route("/api/stats", get(get_stats))
        .route("/api/clear", axum::routing::post(clear_messages))
        // WebSocket
        .route("/ws", get(ws_handler))
        // Static files (SPA)
        .fallback(get(static_handler))
        .with_state(state)
}

// --- API Handlers ---

#[derive(Deserialize)]
struct ListParams {
    offset: Option<usize>,
    limit: Option<usize>,
}

async fn list_messages(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(100).min(1000);
    let summaries = state.store.list_summaries(offset, limit).await;
    Json(summaries)
}

async fn get_message(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.store.get_by_id(&id).await {
        Some(msg) => Json(serde_json::to_value(msg).unwrap()).into_response(),
        None => (StatusCode::NOT_FOUND, "Message not found").into_response(),
    }
}

#[derive(Deserialize)]
struct SearchParams {
    q: String,
    limit: Option<usize>,
}

async fn search_messages(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(100).min(1000);
    let results = state.store.search(&params.q, limit).await;
    Json(results)
}

async fn get_stats(State(state): State<AppState>) -> impl IntoResponse {
    let count = state.store.count().await;
    Json(serde_json::json!({
        "total_messages": count,
        "received": state.stats.received.load(Ordering::Relaxed),
        "parsed_ok": state.stats.parsed_ok.load(Ordering::Relaxed),
        "parse_errors": state.stats.parse_errors.load(Ordering::Relaxed),
        "active_connections": state.stats.active_connections.load(Ordering::Relaxed),
        "mllp_port": state.mllp_port,
    }))
}

async fn clear_messages(State(state): State<AppState>) -> impl IntoResponse {
    state.store.clear().await;
    Json(serde_json::json!({"status": "cleared"}))
}

// --- WebSocket ---

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: AppState) {
    let mut rx = state.store.subscribe();

    // Send current stats on connect
    let count = state.store.count().await;
    let _ = socket
        .send(Message::Text(
            serde_json::json!({"type": "init", "total": count}).to_string(),
        ))
        .await;

    // Forward broadcast messages to WebSocket client
    loop {
        tokio::select! {
            result = rx.recv() => {
                match result {
                    Ok(StoreEvent::NewMessage(summary)) => {
                        let payload = serde_json::json!({
                            "type": "new_message",
                            "data": summary,
                        });
                        if socket.send(Message::Text(payload.to_string())).await.is_err() {
                            break; // client disconnected
                        }
                    }
                    Ok(StoreEvent::Cleared) => {
                        let payload = serde_json::json!({
                            "type": "cleared"
                        });
                        if socket.send(Message::Text(payload.to_string())).await.is_err() {
                            break; // client disconnected
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        let _ = socket.send(Message::Text(
                            serde_json::json!({"type": "lagged", "missed": n}).to_string()
                        )).await;
                    }
                    Err(_) => break,
                }
            }
            // Also handle incoming WebSocket messages (ping/pong, close)
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {} // ignore other client messages for now
                }
            }
        }
    }
}

// --- Static File Serving ---

async fn static_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match StaticAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                content.data.to_vec(),
            )
                .into_response()
        }
        None => {
            // SPA fallback: serve index.html for unknown routes
            match StaticAssets::get("index.html") {
                Some(content) => Html(String::from_utf8_lossy(&content.data).to_string()).into_response(),
                None => (StatusCode::NOT_FOUND, "Not found").into_response(),
            }
        }
    }
}
