# HL7 Forge

High-performance HL7 MLLP server with real-time Web UI, built in Rust.

HL7 Forge replaces the HL7 Inspector as the central testing tool for the integration team. It runs as a service on the dev server and can be used by multiple developers simultaneously via the browser — no local setup, no RDP-exclusive windows.

## Features

- **MLLP Server** — Receives HL7 v2.x messages with correct MLLP framing, responds with ACK/NACK
- **Real-time Web UI** — Browser-based SPA with WebSocket updates, no installation required
- **Message Details** — Parsed, raw, and JSON views per message
- **Search & Filter** — Message type, patient, facility, ID
- **JSON Export** — Export messages as JSON
- **Single Binary** — No dependencies, frontend is embedded in the binary

## Quick Start

```bash
# Build
cargo build --release

# Run (defaults: MLLP on port 2575, Web UI on port 8080)
cargo run --release

# Custom ports
MLLP_PORT=4000 WEB_PORT=9090 cargo run --release
```

Open the Web UI at: **http://localhost:8080**

## Windows Deployment

HL7 Forge is designed as a portable single binary for Windows. The `.exe` can be run directly or installed as a Windows service.

### Download

The Windows binary is built automatically via GitHub Actions:

- **Releases:** The `hl7-forge.exe` is available for download under [GitHub Releases](../../releases) with each release
- **Artifacts:** Every push to `main` produces a build artifact (`hl7-forge-windows`) under [Actions](../../actions)

### Direct Start

```powershell
# Simply run
.\hl7-forge.exe

# With custom ports
$env:MLLP_PORT="4000"; $env:WEB_PORT="9090"; .\hl7-forge.exe
```

### As a Windows Service (using NSSM)

```powershell
# Install NSSM (https://nssm.cc)
nssm install HL7Forge C:\Tools\hl7-forge.exe
nssm set HL7Forge AppEnvironmentExtra MLLP_PORT=2575 WEB_PORT=8080
nssm start HL7Forge
```

The service starts automatically on server boot and is accessible at `http://<server>:8080`.

## Architecture

```
┌─────────────────────┐    ┌──────────────┐    ┌──────────────┐
│  HL7 Sender (HIS,   │───▶│  MLLP Server │───▶│  Message     │
│  RIS, PACS, etc.)   │◀───│  (Tokio TCP)  │    │  Store       │
└─────────────────────┘ ACK└──────────────┘    │  (In-Memory) │
                                                └──────┬───────┘
                                                       │ broadcast
                                                ┌──────▼───────┐
                                                │  Web Server   │
                                                │  (Axum)       │
                                                │  REST + WS    │
                                                └──────┬───────┘
                                                       │
                                                ┌──────▼───────┐
                                                │  Browser SPA  │
                                                │  (WebSocket)  │
                                                └──────────────┘
```

## API

| Endpoint | Method | Description |
|---|---|---|
| `/api/messages?offset=0&limit=100` | GET | Message list (newest first) |
| `/api/messages/{id}` | GET | Message with all segments |
| `/api/search?q=ADT&limit=100` | GET | Search messages |
| `/api/stats` | GET | Server statistics |
| `/api/clear` | POST | Delete all messages |
| `/ws` | WS | Real-time updates |

## Testing

```bash
# Unit tests
cargo test

# Send a test message via MLLP (netcat)
printf '\x0bMSH|^~\\&|TESTSYS|TESTFAC|HL7FORGE|HL7FORGE|20240101120000||ADT^A01|MSG001|P|2.5\rPID|||12345||Doe^John||19900101|M\rPV1||I|ICU^101^A\x1c\r' | nc localhost 2575

# Or use the included test script
./test.sh
```

## Roadmap

The full roadmap is described in [ROADMAP.md](ROADMAP.md), the derived milestones in [MILESTONES.md](MILESTONES.md).

| Milestone | Description |
|---|---|
| 1 – Team-Ready Server | Windows service, config file, backpressure, graceful shutdown |
| 2 – Multi-User Experience | Sessions, source tagging, tags, bookmarks |
| 3 – Message Analysis | Field dictionary, validation, message type detection, diff |
| 4 – Workflow & Testing | Replay, message editor, test generator |
| 5 – Persistence | SQLite, retention policy, extended exports |
| 6 – FHIR & Monitoring | FHIR R4 preview, dashboard, alerting, health endpoint |

## Technology

| Topic | Decision |
|---|---|
| Language | Rust |
| Async Runtime | Tokio |
| Web Framework | Axum |
| UI | Embedded SPA (HTML/JS/CSS) |
| CI/CD | GitHub Actions (Windows build) |
| Deployment | Single `.exe`, xcopy-deploy |
