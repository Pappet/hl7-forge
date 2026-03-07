# HL7 Forge — Project Overview

## What Is It

HL7 Forge is a high-performance MLLP server with a real-time web UI for inspecting HL7 v2.x messages. It is built in Rust and deployed as a single portable binary. It serves as a central service for interface integration teams to view and inspect MLLP messages in real time without local setup.

## Project Status

- **Language:** Rust
- **Latest Release:** v0.3.0 — Milestones 1 & 2 complete
- **Next Milestone:** Milestone 3 (Message Analysis)

---

## Usage Context

- **Team:** Integration team (multiple developers simultaneously)
- **Software:** Orchestra (healthcare integration platform for interface development)
- **Infrastructure:** Windows Server, access via RDP + browser
- **Protocol:** HL7 v2.x over MLLP, FHIR R4 on the horizon
- **Traffic:** Includes ADT, ORU, ORM, SIU, and MDM message types. MDM with Base64-encoded PDF attachments is daily traffic, not an edge case.
- **Network:** Hospital internal network. No internet access from the server.

---

## Technical Decisions

| Topic | Decision | Rationale |
|---|---|---|
| Language | Rust | Performance, memory safety, single binary |
| Async runtime | Tokio | Proven, high throughput, low latency |
| Web framework | Axum | Tokio-native, type-safe, performant |
| UI | Embedded SPA (vanilla HTML/JS/CSS) | Zero dependencies, browser-based, multi-user capable |
| Static embedding | `rust-embed` | Frontend baked into binary at compile time — no separate web server |
| State sharing | `Arc<RwLock<>>` + broadcast channel | Simple, correct, sufficient for current scale |
| Persistence | In-memory (Milestones 0–4), SQLite (Milestone 5+) | Simple start, persistence when needed |
| Deployment | Single portable `.exe` | No installer, no runtime, xcopy deploy |
| CI/CD | GitHub Actions | Cross-platform builds: Windows, macOS, Linux |

## Non-Goals

- **No complex visual HL7 editor** — simple raw text editing for testing is provided, but not a full replacement for Orchestra's mapping UI
- **No external database servers** — no PostgreSQL/MSSQL; uses lightweight local SQLite when persistence is needed
- **No HL7 router** — message routing and transformation stays in Orchestra
- **No CSS/JS frameworks** — the vanilla SPA approach is intentional for zero-dependency deployment

---

## Architecture

Two independent async Tokio tasks share a single `MessageStore` via `Arc<RwLock<>>`:

```
+---------------------+    +------------------+    +------------------+
|  HL7 Sender         |--->|  MLLP Server     |--->|  Message Store   |
|  (HIS / RIS / PACS) |<---|  (Tokio TCP)     |    |  Arc<RwLock<>>   |
+---------------------+ ACK+------------------+    |  + broadcast ch. |
                                                    +--------+---------+
                                                             | push
                                                    +--------v---------+
                                                    |  Web Server      |
                                                    |  (Axum)          |
                                                    |  REST + WebSocket|
                                                    +--------+---------+
                                                             |
                                                    +--------v---------+
                                                    |  Browser SPA     |
                                                    |  (embedded HTML) |
                                                    +------------------+
```

### MLLP Server (`src/mllp.rs`)

Listens on a TCP port, handles MLLP framing (`0x0B` start, `0x1C 0x0D` end), parses HL7 messages, sends ACKs/NACKs, and inserts the result into the MessageStore.

- **ACK storm prevention:** incoming messages with `message_type.starts_with("ACK")` are stored but never ACK'd back — prevents infinite ping-pong with Orchestra.
- **DoS hardening:** 10 MB payload limit, 60s read timeout, 30s write timeout.
- **Connection limits:** configurable `max_connections` via `hl7-forge.toml`.

### Web Server (`src/web.rs`)

An Axum-based HTTP server providing REST API endpoints and a WebSocket endpoint for real-time updates.

### Message Store (`src/store.rs`)

A thread-safe in-memory buffer with dual eviction:

- **Count limit:** configurable capacity (default 10,000 messages)
- **Size limit:** configurable memory budget (default 512 MB)
- **Trigger:** either limit hit → evict oldest 10% of non-bookmarked messages
- **Rationale:** MDM messages with Base64-encoded attachments can be several MB each; count-only eviction is insufficient.

### HL7 Parser & Dictionary (`src/hl7/`, `src/dictionary.rs`)

- `hl7/parser.rs` — Parses raw HL7 text by extracting delimiters from the MSH segment, splitting on `\r`/`\n` into segments, and decomposing fields/components. Also builds ACK responses.
- `hl7/types.rs` — Data structures: `Hl7Message`, `Hl7MessageSummary`, `Hl7Segment`, `Hl7Field`, `Delimiters`.
- `dictionary.rs` — A compiled-in, high-speed `OnceLock` in-memory JSON dictionary engine providing field definitions based on the HL7 v2.5.1 specification.

**MSH field indexing quirk:** MSH-1 is the field separator character itself (`|`). The parser inserts a synthetic `Hl7Field { index: 1, value: "|" }` and shifts all other fields up by 1, so that `get_field_value(msh, 3)` correctly returns Sending Application per the HL7 standard.

### Frontend (`static/`)

Three files, all embedded into the binary at compile time via `rust-embed`:

- `index.html` — HTML structure only
- `style.css` — All styles (dark theme, CSS variables)
- `app.js` — All logic (WebSocket, rendering, search, batching)

Key behaviors:
- Messages are batched and rendered at most every 250ms
- Pause/Live button buffers incoming messages without displaying them
- Search is purely client-side (filters `messages[]` array via `matchesSearch()`)
- Parse errors are shown with PARSE ERROR in red in the message list

---

## Source Layout

```
src/
├── main.rs          # Entry point, tokio::select! over MLLP + Web tasks
├── config.rs        # Configuration loading (hl7-forge.toml + env vars)
├── mllp.rs          # TCP listener, MLLP framing, ACK/NACK dispatch
├── store.rs         # In-memory store with broadcast channel, dual eviction
├── web.rs           # Axum router, REST handlers, WebSocket handler
├── dictionary.rs    # In-memory embedded HL7 dictionary engine 
└── hl7/
    ├── mod.rs
    ├── parser.rs    # Raw HL7 → Hl7Message, delimiter extraction, ACK builder
    └── types.rs     # Hl7Message, Hl7MessageSummary, Hl7Segment, Hl7Field, Delimiters
static/
├── index.html       # HTML skeleton
├── style.css        # Dark theme, CSS variables
└── app.js           # SPA logic (vanilla JS)
assets/
└── hl7/
    └── v2.5.1.json  # Highly optimized canonical source of truth for dictionary fields
tests/
├── test.sh          # Linux/macOS functional + load test (netcat, 100 messages)
└── test.ps1         # Windows functional + load test (.NET TcpClient, 1000 messages)
```

---

## API Reference

| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/messages?offset=0&limit=100` | Paginated message list, newest first |
| `GET` | `/api/messages/{id}` | Full message with all segments and fields |
| `GET` | `/api/search?q=ADT&limit=100` | Search by type, patient, facility, ID, IP |
| `GET` | `/api/stats` | Live server stats (messages, connections, errors) |
| `POST` | `/api/clear` | Delete all messages from store |
| `POST` | `/api/messages/{id}/bookmark` | Toggle bookmark on a message |
| `POST` | `/api/messages/{id}/tags` | Add a tag to a message |
| `DELETE` | `/api/messages/{id}/tags/{tag}` | Remove a tag from a message |
| `WS` | `/ws` | Real-time updates: `init`, `new_message`, `lagged` events |

### WebSocket Events

```json
// On connect
{ "type": "init", "total": 42 }

// On new message
{ "type": "new_message", "data": { /* Hl7MessageSummary */ } }

// When client falls behind the broadcast buffer
{ "type": "lagged", "missed": 12 }
```

### Error Handling

| Scenario | Response |
|---|---|
| Valid HL7 message | `MSA\|AA` — Application Accept |
| Unknown message type (e.g. `ZZZ^Z99`) | `MSA\|AA` — type-agnostic acceptance |
| Missing or malformed MSH segment | `MSA\|AE` — Application Error (NACK) |
| Payload > 10 MB | Connection closed immediately |

---

## Technology Stack

| Component | Crate / Technology | Version |
|---|---|---|
| Async runtime | `tokio` | 1.x |
| Web framework | `axum` | 0.7 |
| HTTP middleware | `tower-http` (CORS, static FS) | 0.5 |
| Serialization | `serde` + `serde_json` | 1.x |
| Config parsing | `toml` | latest |
| Timestamps | `chrono` | 0.4 |
| UUID generation | `uuid` (v4) | 1.x |
| Logging | `tracing` + `tracing-subscriber` | 0.1/0.3 |
| Static files | `rust-embed` + `mime_guess` | 8.x/2.x |
| Error handling | `anyhow` | 1.x |
| Frontend | Vanilla JS / HTML / CSS | — |

---

## Deployment

- **Primary target:** Windows Server (`.exe`), run as a Windows Service via NSSM
- **Build pipeline:** GitHub Actions builds Windows, macOS (Apple Silicon), and Linux binaries on every push to `main` and attaches them to releases
- **Users:** Multiple developers simultaneously via browser — no local setup, no RDP window
- **Deployment style:** Single binary, xcopy. No installer, no runtime dependencies, no Docker.

---

## Testing

```bash
# Unit tests
cargo test

# Functional + load test (Linux/macOS, requires nc)
./test.sh

# Functional + load test (Windows, no external tools required)
.\tests\test.ps1
```

Both test scripts send the same set of HL7 messages: three valid types (ADT^A01, ORU^R01, SIU^S12), three error cases, followed by a load test. The PowerShell test uses a persistent TCP connection for load testing (1000 messages). The shell script spawns a new `nc` process per message (100 messages).

---

## Additional References

- [README.md](README.md) — Quick start and feature overview
- [ROADMAP.md](ROADMAP.md) — Milestones and planned features
- [STYLE_GUIDE.md](STYLE_GUIDE.md) — Appearance and functioning rules
- [CHANGELOG.md](CHANGELOG.md) — Full history of every change
