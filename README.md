# HL7 Forge ⚡

High-performance HL7 MLLP server with real-time Web UI, built in Rust.

## Quick Start

```bash
# Build
cargo build --release

# Run (defaults: MLLP on 2575, Web UI on 8080)
cargo run --release

# Custom ports
MLLP_PORT=4000 WEB_PORT=9090 cargo run --release
```

Open **http://localhost:8080** in your browser.

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
| `/api/messages?offset=0&limit=100` | GET | List messages (newest first) |
| `/api/messages/{id}` | GET | Full message with segments |
| `/api/search?q=ADT&limit=100` | GET | Search messages |
| `/api/stats` | GET | Server statistics |
| `/api/clear` | POST | Clear all messages |
| `/ws` | WS | Real-time message stream |

## Testing

```bash
# Run unit tests
cargo test

# Send a test HL7 message via MLLP (using netcat)
printf '\x0bMSH|^~\\&|TESTSYS|TESTFAC|HL7FORGE|HL7FORGE|20240101120000||ADT^A01|MSG001|P|2.5\rPID|||12345||Doe^John||19900101|M\rPV1||I|ICU^101^A\x1c\r' | nc localhost 2575
```

## Roadmap

- [ ] FHIR R4 preview (JSON conversion)
- [ ] SQLite persistence option
- [ ] Message replay / resend
- [ ] Dark/Light theme toggle
- [ ] HL7 field dictionary (hover tooltips)
- [ ] TCP client mode (connect to remote MLLP)
- [ ] Message diffing
