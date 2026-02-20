<div align="center">

<img src="https://raw.githubusercontent.com/Pappet/hl7-forge/main/assets/social-card.png" alt="HL7 Forge" width="100%">

<br/>
<br/>

[![Build](https://img.shields.io/github/actions/workflow/status/Pappet/hl7-forge/build.yml?branch=main&style=flat-square&logo=github&label=build&color=4caf84)](https://github.com/Pappet/hl7-forge/actions)
[![Release](https://img.shields.io/github/v/release/Pappet/hl7-forge?style=flat-square&color=6c8cff&logo=rust)](https://github.com/Pappet/hl7-forge/releases)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square&color=525775)](LICENSE)
[![Rust](https://img.shields.io/badge/built%20with-Rust-b7410e?style=flat-square&logo=rust)](https://www.rust-lang.org/)

<br/>

**HL7 Forge** is a high-performance MLLP server with a real-time web UI for inspecting HL7 v2.x messages — built in Rust, deployed as a single binary.

Designed as a drop-in replacement for HL7 Inspector: runs as a central service, accessible by your entire team via browser. No local setup. No RDP-exclusive windows.

<br/>

[**Download**](https://github.com/Pappet/hl7-forge/releases) · [**Quickstart**](#-quick-start) · [**API**](#-api-reference) · [**Roadmap**](#-roadmap)

</div>

---

## ✦ Features

| | |
|---|---|
| ⚡ **MLLP Server** | Async TCP listener with correct `0x0B`/`0x1C 0x0D` framing, auto ACK/NACK |
| 🖥️ **Real-time Web UI** | Browser SPA with WebSocket push — no page reload, no framework |
| 🔍 **Deep Parser** | Dynamic delimiter detection, full segment/field decomposition |
| 📋 **Three Views** | Parsed segments, raw HL7, and JSON per message |
| 🔎 **Search & Filter** | By message type, patient name, facility, message control ID, source IP |
| 💾 **Smart Store** | In-memory, 100k message capacity with automatic 10% eviction |
| 📥 **JSON Export** | Export full message list with one click |
| 🛡️ **Hardened** | 10 MB payload cap, 60s read timeout, graceful shutdown on `Ctrl+C` |
| 📦 **Single Binary** | Frontend embedded via `rust-embed` — zero runtime dependencies |

---

## 🚀 Quick Start

**Prerequisites:** Rust toolchain ([rustup.rs](https://rustup.rs))

```bash
# Clone and build
git clone https://github.com/Pappet/hl7-forge.git
cd hl7-forge
cargo build --release

# Run (defaults: MLLP :2575, Web UI :8080)
cargo run --release

# Custom ports via environment variables
MLLP_PORT=4000 WEB_PORT=9090 cargo run --release
```

Open **[http://localhost:8080](http://localhost:8080)** — the UI connects automatically.

---

## 🪟 Windows Deployment

HL7 Forge is designed as a portable Windows service. The `.exe` is built automatically via GitHub Actions on every push to `main`.

### Download

| Channel | Link |
|---|---|
| **Latest Release** | [GitHub Releases](../../releases) — `hl7-forge.exe` attached to each release tag |
| **Latest Build** | [GitHub Actions](../../actions) — `hl7-forge-windows` artifact from the last CI run |

### Direct Start

```powershell
# Run directly
.\hl7-forge.exe

# With custom ports
$env:MLLP_PORT="4000"; $env:WEB_PORT="9090"; .\hl7-forge.exe
```

### Install as Windows Service (NSSM)

```powershell
# Download NSSM from https://nssm.cc, then:
nssm install HL7Forge C:\Tools\hl7-forge.exe
nssm set HL7Forge AppEnvironmentExtra MLLP_PORT=2575 WEB_PORT=8080
nssm start HL7Forge
```

The service starts automatically on reboot and is accessible at `http://<server-ip>:8080`.

---

## 🏗️ Architecture

Two independent async Tokio tasks share a single `MessageStore` via `Arc<RwLock<>>`:

```
┌─────────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│  HL7 Sender         │───▶│  MLLP Server     │───▶│  Message Store   │
│  (HIS / RIS / PACS) │◀───│  (Tokio TCP)     │    │  Arc<RwLock<>>   │
└─────────────────────┘ ACK└──────────────────┘    │  + broadcast ch. │
                                                    └────────┬─────────┘
                                                             │ push
                                                    ┌────────▼─────────┐
                                                    │  Web Server      │
                                                    │  (Axum)          │
                                                    │  REST + WebSocket│
                                                    └────────┬─────────┘
                                                             │
                                                    ┌────────▼─────────┐
                                                    │  Browser SPA     │
                                                    │  (embedded HTML) │
                                                    └──────────────────┘
```

### Source Layout

```
src/
├── main.rs          # Entry point, tokio::select! over MLLP + Web tasks
├── mllp.rs          # TCP listener, MLLP framing, ACK/NACK dispatch
├── store.rs         # In-memory store with broadcast channel
├── web.rs           # Axum router, REST handlers, WebSocket handler
└── hl7/
    ├── parser.rs    # Raw HL7 → Hl7Message, delimiter extraction, ACK builder
    └── types.rs     # Hl7Message, Hl7Segment, Hl7Field, Delimiters
static/
└── index.html       # Self-contained SPA (vanilla JS, no framework)
```

---

## 📡 API Reference

| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/messages?offset=0&limit=100` | Paginated message list, newest first |
| `GET` | `/api/messages/{id}` | Full message with all segments and fields |
| `GET` | `/api/search?q=ADT&limit=100` | Search by type, patient, facility, ID, IP |
| `GET` | `/api/stats` | Live server stats (messages, connections, errors) |
| `POST` | `/api/clear` | Delete all messages from store |
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

---

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run a single test by name
cargo test test_parse_adt_a01

# Send a test message manually via netcat
printf '\x0bMSH|^~\\&|TESTSYS|TESTFAC|HL7FORGE|HL7FORGE|20240101120000||ADT^A01|MSG001|P|2.5\rPID|||12345||Doe^John||19900101|M\rPV1||I|ICU^101^A\x1c\r' | nc localhost 2575
```

### Manual Test Scripts

Both scripts send the same set of HL7 messages: three valid types (ADT^A01, ORU^R01, SIU^S12), three error cases, followed by a load test.

**Linux / macOS** — requires `nc` (netcat):

```bash
./test.sh
```

Load test: 100 messages, one `nc` process per message.

**Windows** — no external tools required, uses .NET `TcpClient` directly:

```powershell
.\tests\test.ps1
```

Load test: 1000 messages over a **single persistent TCP connection** for more accurate throughput measurement.

### Error Handling Behavior

| Scenario | Response |
|---|---|
| Valid HL7 message | `MSA\|AA` — Application Accept |
| Unknown message type (e.g. `ZZZ^Z99`) | `MSA\|AA` — type-agnostic acceptance |
| Missing or malformed MSH segment | `MSA\|AE` — Application Error (NACK) |
| Payload > 10 MB | Connection closed immediately |

---

## 🗺️ Roadmap

| Milestone | Status | Description |
|---|---|---|
| **MVP** | ✅ Done | MLLP server, parser, real-time UI, search, export |
| **1 — Team-Ready** | 🔲 Planned | `hl7-forge.toml` config, Windows service, graceful shutdown, backpressure |
| **2 — Multi-User** | 🔲 Planned | Session-isolated views, source color coding, message tagging, bookmarks |
| **3 — Message Analysis** | 🔲 Planned | HL7 field dictionary tooltips, validation, segment diff |
| **4 — Workflow & Testing** | 🔲 Planned | Message replay, raw editor, test message generator |
| **5 — Persistence** | 🔲 Planned | Optional SQLite backend, retention policy, CSV/HL7 export |
| **6 — FHIR & Monitoring** | 🔲 Planned | FHIR R4 preview, dashboard charts, health endpoint, alerting |

Full details in [ROADMAP.md](ROADMAP.md) and [MILESTONES.md](MILESTONES.md).

---

## ⚙️ Technology

| Concern | Choice | Reason |
|---|---|---|
| Language | **Rust** | Memory safety, performance, single binary |
| Async runtime | **Tokio** | Proven, high-throughput, low latency |
| Web framework | **Axum** | Tokio-native, type-safe, ergonomic |
| Frontend | **Embedded SPA** | Zero dependencies, multi-user via browser |
| Static files | **rust-embed** | Frontend baked into the binary at compile time |
| CI/CD | **GitHub Actions** | Cross-platform builds: Windows, macOS, Linux |

---

## 🤝 Contributing

Contributions are welcome. Please open an issue first for larger changes so we can discuss the approach.

```bash
# Fork, then clone your fork
git clone https://github.com/<your-username>/hl7-forge.git

# Create a feature branch
git checkout -b feature/my-feature

# Make your changes, then run tests and check for warnings
cargo test
cargo clippy -- -D warnings

# Submit a pull request against main
```

Please keep commits focused and include tests for new behavior where applicable.

---

## 📄 License

MIT — see [LICENSE](LICENSE) for the full text.

```
Copyright (c) 2026 Pappet
```

---

<div align="center">
<sub>Built with ⚡ and Rust · <a href="https://github.com/Pappet/hl7-forge">github.com/Pappet/hl7-forge</a></sub>
</div>