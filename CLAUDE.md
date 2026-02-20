# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo run --release            # Run server (MLLP + Web)
cargo test                     # Run all unit tests
cargo test <test_name>         # Run a single test
```

Environment variables: `MLLP_PORT` (default 2575), `WEB_PORT` (default 8080), `RUST_LOG` for tracing.

Manual MLLP testing: `./test.sh` sends sample ADT^A01, ORU^R01, SIU^S12 messages via netcat.

## Architecture

HL7 Forge is an MLLP server with a real-time web UI for inspecting HL7 v2.x messages. Two async Tokio tasks run concurrently via `tokio::select!` in `main.rs`:

1. **MLLP Server** (`mllp.rs`) — TCP listener accepting HL7 messages wrapped in MLLP framing (VT `0x0B` start, FS `0x1C` + CR `0x0D` end). Parses each message, stores it, and returns ACK/NACK.

2. **Web Server** (`web.rs`) — Axum HTTP server serving a REST API and WebSocket endpoint. The SPA frontend (`static/index.html`) is embedded in the binary via `rust-embed`.

**Shared state** flows through `MessageStore` (`store.rs`) — an `Arc<RwLock<>>` in-memory store with a `tokio::sync::broadcast` channel that pushes new messages to WebSocket subscribers in real-time. Capacity is 100k messages with 10% eviction on overflow.

### HL7 Parsing (`src/hl7/`)

- `parser.rs` — Parses raw HL7 text by extracting delimiters from the MSH segment, splitting on `\r`/`\n` into segments, and decomposing fields/components. Also builds ACK responses.
- `types.rs` — Data structures: `Hl7Message`, `Hl7MessageSummary`, `Hl7Segment`, `Hl7Field`, `Delimiters`.

### Web API Routes

- `GET /api/messages?offset=&limit=` — Paginated message list (newest first)
- `GET /api/messages/{id}` — Full message with segments
- `GET /api/search?q=&limit=` — Search messages
- `GET /api/stats` — Server statistics
- `POST /api/clear` — Clear all messages
- `WS /ws` — Real-time updates ("init", "new_message", "lagged" events)

### Frontend

`static/index.html` is a self-contained vanilla JS/HTML/CSS SPA (no frameworks). Two-panel layout with message list and detail view supporting parsed segments, raw, and JSON views.
