# Project Overview

## What it is?
HL7 Forge is a high-performance MLLP server with a real-time web UI for inspecting HL7 v2.x messages. It is built in Rust and deployed as a single portable binary. It serves as a central service for interface integration teams to view and inspect MLLP messages in real time without local setup.

## Project Stats
- **Language**: Rust
- **Status**: Milestone 2 (Multi-User Experience) in progress

## Architectural Decisions
1. **Single Binary + Embedded SPA**: The frontend (HTML/JS/CSS) is embedded into the Rust binary at compile time via `rust-embed`. This ensures zero runtime dependencies.
2. **In-Memory Store**: Uses an `Arc<RwLock<>>` for fast access, with size-based and count-based dual eviction to prevent OOM errors.
3. **Async I/O**: Uses `tokio` for handling multiple concurrent MLLP TCP connections and websockets without blocking.
4. **Message Broadcast**: A `tokio::sync::broadcast` channel is used to push new messages safely to all connected WebSocket clients.

## Detailed Architecture
The system consists of independent async Tokio tasks sharing a single `MessageStore`:

1. **MLLP Server (`src/mllp.rs`)**: Listens on a TCP port, handles MLLP framing, parses HL7 messages, sends ACKs/NACKs, and inserts the result into the MessageStore.
2. **Web Server (`src/web.rs`)**: An Axum-based HTTP server providing REST API endpoints (search, stats) and a WebSocket endpoint for real-time updates.
3. **Message Store (`src/store.rs`)**: A thread-safe buffer that limits memory usage based on configuration.

## Source Files Description
- `src/main.rs`: Application entry point, config loading, and task orchestration.
- `src/config.rs`: Structures for reading `hl7-forge.toml` and env variables.
- `src/mllp.rs`: TCP handling and MLLP framing implementation.
- `src/store.rs`: Concurrency-safe message storage and eviction logic.
- `src/web.rs`: Axum routing, REST API handlers, and WebSocket logic.
- `src/hl7/parser.rs`: HL7 parsing logic, delimiter detection, and ACK generation.
- `src/hl7/types.rs`: Data structures representing parsed HL7 messages and segments.

## Dependencies and their purpose
- **tokio**: Async runtime for network and task management.
- **axum**: Web framework for the REST API and WebSocket server.
- **tower-http**: Middleware for CORS and static file serving.
- **serde / serde_json**: Serialization for API responses.
- **toml**: Configuration file parsing.
- **rust-embed**: Embedding `static/` files into the compiled binary.
- **tracing**: High-performance structured logging.
- **chrono / uuid**: Timestamp handling and unique message identification.

## Additional References
- [ROADMAP.md](ROADMAP.md): Future planning and milestones.
- [MILESTONES.md](MILESTONES.md): Breakdown of planned tasks and criteria.
- [STYLE_GUIDE.md](STYLE_GUIDE.md): Project coding conventions.
