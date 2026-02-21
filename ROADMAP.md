# HL7 Forge – Roadmap

## Vision

HL7 Forge replaces HL7 Inspector as the primary testing tool for the integration team. It runs as a central service on the dev server (Windows Server) and is used by multiple developers simultaneously via browser — no local setup, no RDP-exclusive windows, no performance degradation at high message volumes.

## Usage Context

- **Team:** Integration team (multiple developers simultaneously)
- **Software:** Orchestra (interface development)
- **Infrastructure:** Windows Server, access via RDP + browser
- **Protocol:** HL7 v2.x over MLLP, FHIR R4 on the horizon

---

## Phase 1 – Solid Foundation (MVP) ✅

*Status: Core structure complete, first messages are being received*

- [x] MLLP server with Tokio (async TCP, correct `0x0B`/`0x1C 0x0D` framing)
- [x] ACK/NAK responses (AA, AE)
- [x] HL7 v2.x parser (dynamic delimiter detection, MSH/PID extraction)
- [x] In-memory message store with broadcast channel
- [x] Web UI with real-time message list (WebSocket)
- [x] Segment/field detail view (Parsed, Raw, JSON tabs)
- [x] Search filter (message type, patient, facility, ID)
- [x] JSON export
- [x] Route fix for message detail view (`:id` instead of `{id}`)
- [x] Clean up compiler warnings

## Phase 2 – Team Readiness

*Focus: Multiple developers working against the same server simultaneously*

### Multi-User & Sessions
- [ ] **Session-based views** – each developer sees their own filter configuration, scroll position and selection without affecting others
- [ ] **Color-coded source markers** – messages visually distinguishable by sender system/IP (e.g. Orchestra Dev vs. Orchestra Test)
- [ ] **Message tagging** – manual tagging of messages (e.g. "Bug #1234", "Test scenario A") for attribution during shared use
- [ ] **Bookmark/Pin** – mark important messages so they don't get lost in the stream

### Deployment & Configuration
- [ ] **Configuration file** (`hl7-forge.toml`) – ports, memory limits, log level, retention configurable without recompilation
- [ ] **File Logging** – standard rotating log files for operation monitoring
- [ ] **Portable binary** – single `.exe`, no dependencies, xcopy deployment

### Stability & Performance
- [ ] **Memory Management** – configurable memory budget (e.g. max 512 MB RAM) with automatic eviction of oldest messages to prevent OOM
- [ ] **Connection limits** – cap maximum concurrent MLLP connections
- [ ] **Graceful shutdown** – cleanly terminate active connections on service stop

## Phase 3 – Orchestra Integration & Workflow

*Focus: Seamless integration into the interface development workflow with Orchestra*

### Message Analysis
- [ ] **HL7 field dictionary** – hover tooltips showing field descriptions (e.g. "PID-5: Patient Name", "OBR-4: Universal Service Identifier") based on HL7 v2.5/v2.6 spec
- [ ] **Message type detection** – ADT, ORM, ORU, SIU, MDM etc. with short description and typical segments
- [ ] **Validation** – check required fields per message type, show warnings (e.g. "PID-3 missing in ADT^A01")
- [ ] **Segment diff** – compare two messages side by side, highlight differences (ideal for testing Orchestra transformations)

### Workflow Features
- [ ] **Message replay** – resend stored messages to a configurable target address/port (MLLP client mode)
- [ ] **Test message generator** – templates for common message types (ADT^A01, ORM^O01, ORU^R01) with editable fields, send directly from the UI
- [ ] **Message editor** – edit raw HL7 directly in the UI and send (rapid testing against Orchestra channels)
- [ ] **Auto-refresh trigger** – WebSocket-based notification when new messages arrive, optionally with desktop notification

### Persistence
- [ ] **SQLite backend** – optional persistence so messages survive server restarts
- [ ] **Retention policy** – automatic deletion after X days / X messages
- [ ] **Extended export** – CSV export, HL7 file export (`.hl7`), filtered exports

## Phase 4 – FHIR & Extended Analysis

*Focus: Future-proofing and deeper insights*

### FHIR R4 Preview
- [ ] **HL7 v2 → FHIR R4 mapping** – display ADT messages as FHIR bundles (Patient, Encounter, etc.)
- [ ] **FHIR JSON view** – additional tab in the detail view
- [ ] **FHIR HTTP endpoint** – REST endpoint that can receive FHIR bundles/resources (for future Orchestra FHIR channels)

### Monitoring & Statistics
- [ ] **Dashboard view** – messages per minute/hour, message type distribution, error rate as charts
- [ ] **Latency tracking** – time difference between MSH-7 (message timestamp) and receive time
- [ ] **Alerting** – configurable warnings at error rate > X% or message gap > Y minutes
- [ ] **Health endpoint** – `/api/health` for monitoring tools (Zabbix, PRTG etc.)

### Extended Features
- [ ] **Multi-port listener** – multiple MLLP ports simultaneously, e.g. port 2575 for ADT, 2576 for ORM (test separate Orchestra channels)
- [ ] **TLS support** – encrypted MLLP connections (MLLP/S)
- [ ] **ACK configuration** – customizable ACK responses (e.g. always send NAK to test Orchestra retry logic)
- [ ] **Regex filter** – extended search with regular expressions across all fields
- [ ] **Dark/Light theme toggle**

## Phase 5 – Nice-to-Have

*No priority, but useful when time allows*

- [ ] **Plugin system** – load custom parsers/transformers as WASM modules
- [ ] **REST API for CI/CD** – send messages programmatically and verify results (automated integration tests for Orchestra channels)
- [ ] **Audit log** – who viewed/sent which message and when
- [ ] **Import from HL7 Inspector** – migrate existing message collections
- [ ] **Orchestra log correlation** – link message IDs to Orchestra channel logs

---

## Technical Decisions

| Topic | Decision | Rationale |
|---|---|---|
| Language | Rust | Performance, memory safety, single binary |
| Async runtime | Tokio | Proven, high throughput, low latency |
| Web framework | Axum | Tokio-native, type-safe, performant |
| UI | Embedded SPA (HTML/JS) | Zero dependencies, browser-based, multi-user |
| Persistence | In-memory (Phase 1–2), SQLite (Phase 3+) | Simple start, persistence when needed |
| Deployment | Single portable `.exe` | No installer, no runtime, xcopy deploy |

## Non-Goals

- **No complex visual HL7 editor** – simple raw text editing for testing is provided, but it's not a full replacement for Orchestra's mapping UI
- **No external database servers** – does not require heavy databases like PostgreSQL/MSSQL; uses lightweight local SQLite instead
- **No HL7 router** – message routing and transformation stays in Orchestra

## Development Guidelines

- **Commits:** Always create a git commit after completing a reasonable amount of code changes or finishing a task. Maintain atomic, well-described commits.
