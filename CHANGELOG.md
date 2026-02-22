# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project follows [Semantic Versioning](https://semver.org/lang/en/).

---

## [Unreleased]

### Added
- Templates for Bugs and Feature Requests
- CONTRIBUTING, SECURITY, and Pull Request templates

### Changed
- Translated all German text to English across the codebase
- Updated ROADMAP with completed Phase 1 tasks, specific sections, and guidelines
- Added detailed issue comment preferences for AI agents

### Fixed
- Fixed UI sync on clear database

### Commit History (chronological)

#### 2026-02-22

| Commit | Description |
|--------|-------------|
| [`29e5e8c`](https://github.com/Pappet/hl7-forge/commit/29e5e8c6a7336400a1e02e05687ad1976cfec330) | `docs:` add CONTRIBUTING, SECURITY, and PR template |
| [`89ba306`](https://github.com/Pappet/hl7-forge/commit/89ba3063590a0ee3aef05f0e3ebf5b07921dcfd4) | `chore:` Add Templates for Bugs and Feature Requests |

#### 2026-02-21

| Commit | Description |
|--------|-------------|
| [`efb0bcc`](https://github.com/Pappet/hl7-forge/commit/efb0bcc2025500ba7e26e56494dfc8aefbbd0e33) | `docs:` Update ROADMAP with completed Phase 1 tasks, refine deployment and memory management sections, clarify non-goals, and add development guidelines. |
| [`8128ab4`](https://github.com/Pappet/hl7-forge/commit/8128ab456f0a0ff23c3739481fd2194a8934a3aa) | `docs:` add detailed issue comment preferences for AI agents |
| [`a1837a4`](https://github.com/Pappet/hl7-forge/commit/a1837a45d7af2898ee1f693e78d509b56688f51a) | `fix:` Fix UI sync on clear database |
| [`09121a5`](https://github.com/Pappet/hl7-forge/commit/09121a50a319c494c233f050df7eb561aa4c49f0) | `docs:` translate all German text to English across the codebase |

### Planned – Milestone 1: Team-Ready Server

> Goal: HL7 Forge runs stably as a Windows service on the dev server, is configurable without recompilation, and holds up under high load.

- **Configuration file** (`hl7-forge.toml`) — ports, memory limits, log level, and retention configurable without recompilation
- **Windows Service** — installable via `sc create` / NSSM, automatic start on server reboot
- **Windows Event Log integration** — startup banner in Windows Event Log for ops monitoring
- **Portable binary** — single `.exe` without runtime dependencies, xcopy deployment
- **Backpressure handling** — evict oldest messages when the store is full instead of OOM
- **Memory budget** — configurable RAM limit (e.g. 512 MB), automatic eviction on excess
- **Connection limits** — cap maximum concurrent MLLP connections
- **Graceful shutdown** — cleanly terminate active connections on `Ctrl+C` or service stop

### Planned – Milestone 2: Multi-User Experience

> Goal: Multiple developers work productively against the same server simultaneously without interfering with each other.

- **Session-based views** — each developer sees their own filter configuration, scroll position and selection without affecting others
- **Color-coded source markers** — messages visually distinguishable by sender system/IP
- **Message tagging** — manual tagging of messages (e.g. "Bug #1234", "Test scenario A") for attribution during shared use
- **Bookmark/Pin** — mark important messages so they don't get lost in the live stream

### Planned – Milestone 3: Message Analysis

> Goal: Developers understand HL7 messages directly in the UI — field names, validation, diff.

- **HL7 field dictionary** — hover tooltips with field descriptions (e.g. "PID-5: Patient Name") based on HL7 v2.5/v2.6 spec
- **Message type detection** — ADT, ORM, ORU, SIU, MDM etc. with short description and typical segments
- **Validation** — check required fields per message type, show warnings (e.g. "PID-3 missing in ADT^A01")
- **Segment diff** — compare two messages side by side, highlight differences in color

### Planned – Milestone 4: Workflow & Testing

> Goal: Rapid testing against Orchestra — send, edit, replay messages directly from the UI.

- **Message replay** — resend stored messages to a configurable target address/port (MLLP client)
- **Test message generator** — templates for ADT^A01, ORM^O01, ORU^R01 with editable fields
- **Message editor** — edit raw HL7 in the UI and send directly
- **Auto-refresh trigger** — desktop notification on incoming messages (optional)

### Planned – Milestone 5: Persistence

> Goal: Messages survive server restarts.

- **SQLite backend** — optional persistence, enabled via `hl7-forge.toml`
- **Retention policy** — automatic deletion after X days or X messages
- **Extended export** — CSV export, HL7 file export (`.hl7`), filtered exports

### Planned – Milestone 6: FHIR & Monitoring

- **HL7 v2 → FHIR R4 mapping** — display ADT messages as FHIR bundles (Patient, Encounter)
- **FHIR JSON view** — additional tab in the detail view
- **FHIR HTTP endpoint** — REST endpoint for FHIR bundles/resources
- **Dashboard view** — messages per minute/hour, type distribution, error rate as charts
- **Latency tracking** — time difference between MSH-7 (message timestamp) and receive time
- **Alerting** — configurable warnings at error rate > X% or message gap > Y minutes
- **Health endpoint** — `/api/health` for monitoring tools (Zabbix, PRTG etc.)
- **Multi-port listener** — multiple MLLP ports simultaneously (e.g. 2575 for ADT, 2576 for ORM)
- **TLS support** — encrypted MLLP connections (MLLP/S)
- **ACK configuration** — customizable ACK responses (e.g. always send NAK to test retry logic)
- **Regex filter** — extended search with regular expressions across all fields
- **Dark/Light theme toggle**

---

## [0.2.0] – 2026-02-21 – Stabilization & Robustness

> ACK storm prevention, size-based store eviction, UI improvements and error handling for production use with Orchestra/MDM traffic.

---

### Commit History (chronological)

#### 2026-02-21

| Commit | Description |
|--------|-------------|
| [`868b440`](https://github.com/Pappet/hl7-forge/commit/868b440) | `docs:` fix commit hash in CHANGELOG for fdb6e05 |
| [`fdb6e05`](https://github.com/Pappet/hl7-forge/commit/fdb6e05) | `docs:` document mandatory changelog workflow in CLAUDE.md and AGENTS.md |
| [`79cec90`](https://github.com/Pappet/hl7-forge/commit/79cec901f6141ba32c719dda2829e15d20f9f5d3) | `docs:` update CLAUDE.md, add AGENTS.md with architecture and deployment context |
| [`041cb04`](https://github.com/Pappet/hl7-forge/commit/041cb0417f821b354fce1c021148d4e18a78cd01) | `fix:` ACK storm prevention, search debounce (300 ms), size-based store eviction (`MAX_STORE_BYTES`) |
| [`44afeb9`](https://github.com/Pappet/hl7-forge/commit/44afeb9364ce45d2073da84cd840d72ccd6e1882) | `.gitignore` updated |

#### 2026-02-20

| Commit | Description |
|--------|-------------|
| [`1ee69f3`](https://github.com/Pappet/hl7-forge/commit/1ee69f3655a286ea6dfeb2bec71d49f6f56a5270) | `.gitignore` extended |
| [`2f45c7e`](https://github.com/Pappet/hl7-forge/commit/2f45c7e19f1f8c966e857396bc1df178bc0abb4f) | `ux:` various UX optimizations |
| [`f10cd40`](https://github.com/Pappet/hl7-forge/commit/f10cd40efab007e5d629942ed6cd91611488d468) | `feat:` store failed messages; introduce UI batching (250 ms); split static assets into separate files (`index.html`, `style.css`, `app.js`) |
| [`ee0dcda`](https://github.com/Pappet/hl7-forge/commit/ee0dcda6cd673733bd37e9ab01e9b6933625264b) | `docs:` add PowerShell test script (`tests/test.ps1`); document both test runners |
| [`1bd7c9f`](https://github.com/Pappet/hl7-forge/commit/1bd7c9f841fee412d172599e137e8a91bb06eb16) | create PowerShell test script for Windows load tests (1000 messages, persistent TCP connection) |
| [`7a08611`](https://github.com/Pappet/hl7-forge/commit/7a0861102763aed1898eb4c24f1d344978a92cb7) | rename social card image (`hl7-forge-card.png` → `social-card.png`) |
| [`fd7178a`](https://github.com/Pappet/hl7-forge/commit/fd7178ae69eb4dfe90fb8e5725c2d51db15e9da6) | `docs:` completely revamp README.md |
| [`a6860b1`](https://github.com/Pappet/hl7-forge/commit/a6860b125d40031c131a9c8dc4a9f7e64c3d7b10) | `.gitignore` extended |
| [`71eb33c`](https://github.com/Pappet/hl7-forge/commit/71eb33cdb4d232977b4045c26531de2e4f3b8b0a) | `ci:` migrate release upload from `softprops/action-gh-release` to `gh release upload` |
| [`564aba3`](https://github.com/Pappet/hl7-forge/commit/564aba33437afc23e8657f8518982c89366414a8) | `ci:` simplify build pipeline to three independent jobs: Windows, macOS Apple Silicon, Linux |
| [`c8711e8`](https://github.com/Pappet/hl7-forge/commit/c8711e840d6e1db31867f0577be01a4d2ec03fda) | `ci:` cross-compile Intel macOS binary on Apple Silicon runner |
| [`acc9908`](https://github.com/Pappet/hl7-forge/commit/acc9908b5c222fbfa7956d87c8cabe0083c27dd1) | `ci:` add macOS builds for Intel and Apple Silicon |
| [`8c0db5f`](https://github.com/Pappet/hl7-forge/commit/8c0db5fd3821715e03bae3c8a7f17ac90cfeff7d) | `ci:` set `contents: write` permission for release asset upload |
| [`9a14291`](https://github.com/Pappet/hl7-forge/commit/9a14291ba9281c9b135cf8d3b07ae73934db7cbd) | add MIT license (`LICENSE`) |
| [`5bd3579`](https://github.com/Pappet/hl7-forge/commit/5bd357946f568dc7806ea0ca38f4367023c014a4) | `docs:` clarify ACK behavior for unknown message types; revise `test.sh` |
| [`9ef8ed9`](https://github.com/Pappet/hl7-forge/commit/9ef8ed9cf631b442ebf6e10d7767fcdff5b6e3c1) | `fix:` align MSH field indices with HL7 standard (correct +1 offset); add graceful shutdown via `Ctrl+C` signal handler |
| [`bbde980`](https://github.com/Pappet/hl7-forge/commit/bbde98060ccc46fbe5a9ed238907a73ebcd9af21) | `fix:` harden MLLP server and message store against load spikes and DoS (connection timeouts, 10 MB payload limit) |
| [`fa11aa4`](https://github.com/Pappet/hl7-forge/commit/fa11aa42ae65bde3926349bea3ee81d2b3d9714c) | `polish:` add Cargo metadata; clean up Tokio features; introduce toast notifications in UI |
| [`f33ccfc`](https://github.com/Pappet/hl7-forge/commit/f33ccfcccdfb12e16f4879e06fb8a3a9b8802919) | `fix:` UI polish and pre-release fixes (correct Axum route `:id`, clean up compiler warnings) |
| [`696522c`](https://github.com/Pappet/hl7-forge/commit/696522c4126fc45096687fdb5ef38d6462f593b2) | `docs:` revamp README with feature overview, Windows deployment guide, and milestone table |
| [`679aad3`](https://github.com/Pappet/hl7-forge/commit/679aad3e02d1d0c89299ad8f22b38e95b10bb37c) | `ci:` initial GitHub Actions build workflow |
| [`0c46811`](https://github.com/Pappet/hl7-forge/commit/0c468114f5080b9da02ea6e3b4a22796e56337f2) | `docs:` add ROADMAP.md as strategic planning document |
| [`6cedfc6`](https://github.com/Pappet/hl7-forge/commit/6cedfc6b8cd47c94cf475d6905a00b53f7540fc1) | `docs:` create MILESTONES.md with 6 structured milestones from ROADMAP phases 2–4 |
| [`f087a62`](https://github.com/Pappet/hl7-forge/commit/f087a62b435ecf3d8e6e7d9dc7c5902f4d9d8b82) | `docs:` add CLAUDE.md with build commands and architecture overview for AI agents |
| [`f6fef07`](https://github.com/Pappet/hl7-forge/commit/f6fef074115caf756797f5257578349c583c7bec) | **Initial commit:** HL7 Forge MLLP server with real-time web UI |

---

### Added

#### MLLP Server (Backend)
- Asynchronous TCP listener based on **Tokio** (`rt-multi-thread`)
- Correct MLLP framing: start block `0x0B`, end block `0x1C 0x0D` per the HL7 MLLP standard
- **ACK/NAK generation**: automatic response with `AA` (Application Accept) for valid messages and unknown message types; `AE` (Application Error) for missing or malformed MSH segments
- **ACK storm prevention**: incoming ACK messages are never ACK'd back
- Parallel client connections via `tokio::spawn` per connection
- **DoS hardening**: 10 MB payload limit, connection timeouts
- **Graceful shutdown**: clean termination of active connections on `Ctrl+C` via signal handler

#### HL7 v2.x Parser
- Dynamic delimiter detection from the MSH segment (field separator, component, subcomponent, escape, and repetition separators)
- Extraction of key MSH fields: message type, trigger event, sending/receiving facility & application, message ID, timestamp
- MSH field indices correctly aligned with the HL7 standard (+1 offset)
- PID segment extraction: patient ID (PID-3), patient name (PID-5)
- Robust error handling: parse errors result in a `⚠ PARSE ERROR` marker in the UI rather than a server crash; failed messages are stored in the store
- Structured segment and field representation (`Hl7Message`, `Hl7Segment`, `Hl7Field`, `Delimiters`)

#### In-Memory Message Store
- Central store with `Arc<RwLock<>>` for thread-safe access
- **Dual eviction**: messages are evicted when either the maximum count (`DEFAULT_CAPACITY`) or the maximum byte size (`MAX_STORE_BYTES`) is exceeded — no OOM
- Broadcast channel (`tokio::sync::broadcast`) for real-time notification of all active WebSocket clients on new messages
- Each message receives a unique UUID v4 and an ISO 8601 receive timestamp

#### Web API (Axum)
- `GET /api/messages` — list of all stored messages (paginated, as `Hl7MessageSummary`)
- `GET /api/messages/:id` — full message details including parsed segments and raw HL7
- `GET /api/search` — search endpoint with query parameter `q`
- `GET /api/stats` — live statistics: received messages, parse errors, active connections, MLLP port
- `POST /api/clear` — clear the store
- `GET /ws` — WebSocket endpoint for real-time message push
- CORS middleware for browser access
- Embedded static files via `rust-embed` (no separate web server required)

#### Web UI (Embedded SPA, Vanilla JS)
- **Real-time message list** via WebSocket — new messages appear instantly without page reload
- **Batch rendering** every 250 ms — prevents DOM freezing at high message volumes
- **Pause / Live mode** — `⏸` button buffers incoming messages; `▶` button flushes the buffer and returns to live mode
- **Toast notifications** — subtle in-app notifications for relevant events
- **Detail view** with three tabs: `Parsed`, `Raw`, `JSON`
- **Client-side search filter** (debounced, 300 ms) — filter by message type, patient, facility, message control ID
- **JSON export** — download individual messages as `.json`
- Dark theme with CSS variables
- `⚠ PARSE ERROR` marker in red for failed messages

#### Build & Deployment
- Single Rust binary, no external runtime dependencies
- **GitHub Actions CI/CD**: three independent build jobs for Windows (`.exe`), macOS Apple Silicon, and Linux on every push to `main`
- Build artifacts are automatically attached to GitHub Releases
- MIT license
- Test scripts: `tests/test.sh` (Linux/macOS, netcat, 100 messages) and `tests/test.ps1` (Windows, .NET TcpClient, 1000 messages, persistent connection)

### Fixed

- MSH field indices aligned with the correct HL7 standard offset (`9ef8ed9`)
- Route for the message detail view corrected from `{id}` to `:id` (correct Axum syntax) (`f33ccfc`)
- Compiler warnings (`dead_code`, `unused_variables`) cleaned up (`f33ccfc`)
- ACK storm prevention: incoming ACK messages are detected and never ACK'd back (`041cb04`)

### Technical Stack

| Component         | Crate / Technology              | Version |
|-------------------|---------------------------------|---------|
| Async runtime     | `tokio`                         | 1.x     |
| Web framework     | `axum`                          | 0.7     |
| HTTP middleware   | `tower-http` (CORS, static FS)  | 0.5     |
| Serialization     | `serde` + `serde_json`          | 1.x     |
| Timestamps        | `chrono`                        | 0.4     |
| UUID generation   | `uuid` (v4)                     | 1.x     |
| Logging           | `tracing` + `tracing-subscriber`| 0.1/0.3 |
| Static files      | `rust-embed` + `mime_guess`     | 8.x/2.x |
| Error handling    | `anyhow`                        | 1.x     |
| Frontend          | Vanilla JS / HTML / CSS         | —       |

### Known Issues

- Memory limit (`MAX_STORE_BYTES`) is currently hardcoded — will be configurable via `hl7-forge.toml` in Milestone 1

---

[Unreleased]: https://github.com/Pappet/hl7-forge/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Pappet/hl7-forge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Pappet/hl7-forge/releases/tag/v0.1.0
