# HL7 Forge – Milestones

Derived from [ROADMAP.md](ROADMAP.md).

---

## Milestone 1 – Team-Ready Server

**Goal:** HL7 Forge runs stably as a Windows service on the dev server, is configurable without recompilation, and holds up under high load.

### Requirements

- Phase 1 (MVP) fully complete

### Tasks

- [x] **Configuration file** (`hl7-forge.toml`) – ports, memory limits, log level, retention configurable
- [x] **File Logging** – standard rotating log files for operation monitoring
- [ ] **Windows Service** – installable as a Windows service (`sc create` / NSSM), automatic start on server boot
- [ ] **Startup banner in Event Log** – Windows Event Log integration for ops monitoring
- [x] **Portable binary** – single `.exe` without dependencies, xcopy deployment
- [x] **Backpressure handling** – evict oldest messages when the store is full instead of OOM
- [x] **Memory Management** – e.g. max 512 MB RAM, automatic eviction
- [x] **Connection limits** – cap maximum concurrent MLLP connections
- [x] **Graceful shutdown** – cleanly terminate active connections on service stop

### Acceptance Criteria

- [x] Server starts via `hl7-forge.toml` with configured ports and limits
- [ ] Server runs as a Windows service and starts automatically after reboot
- [x] When the memory budget is reached, old messages are evicted — no OOM
- [x] `Ctrl+C` or service stop terminates active MLLP connections cleanly

---

## Milestone 2 – Multi-User Experience

**Goal:** Multiple developers work productively against the same server simultaneously without interfering with each other.

### Requirements

- Milestone 1 complete (stable server with configuration)

### Tasks

- [x] **Session-based views** – each developer sees their own filter configuration, scroll position and selection
- [x] **Color-coded source markers** – messages visually distinguishable by sender system/IP
- [x] **Message tagging** – manual tagging (e.g. "Bug #1234", "Test scenario A") for attribution
- [ ] **Bookmark/Pin** – mark important messages so they don't get lost in the stream

### Acceptance Criteria

- [x] Two browser tabs show independent filters and selections
- [x] Messages from different source IPs are visually distinguishable
- [x] Tags persist across page reloads (session scope)
- [ ] Bookmarks persist across page reloads (session scope)

---

## Milestone 3 – Message Analysis

**Goal:** Developers understand HL7 messages directly in the UI — field names, validation, diff.

### Requirements

- Milestone 1 complete (stable server)

### Tasks

- [ ] **HL7 field dictionary** – hover tooltips with field descriptions (e.g. "PID-5: Patient Name") based on HL7 v2.5/v2.6 spec
- [ ] **Message type detection** – ADT, ORM, ORU, SIU, MDM etc. with short description and typical segments
- [ ] **Validation** – check required fields per message type, show warnings (e.g. "PID-3 missing in ADT^A01")
- [ ] **Segment diff** – compare two messages side by side, highlight differences

### Acceptance Criteria

- [ ] Hovering over an HL7 field shows its name and description from the spec
- [ ] Validation warnings appear for missing required fields
- [ ] Diff view shows field differences between two messages highlighted in color

---

## Milestone 4 – Workflow & Testing

**Goal:** Rapid testing against Orchestra — send, edit, replay messages directly from the UI.

### Requirements

- Milestone 1 complete (stable server)

### Tasks

- [ ] **Message replay** – resend stored messages to a configurable target address/port (MLLP client)
- [ ] **Test message generator** – templates for common types (ADT^A01, ORM^O01, ORU^R01) with editable fields
- [ ] **Message editor** – edit raw HL7 in the UI and send
- [ ] **Auto-refresh trigger** – desktop notification on new messages (optional)

### Acceptance Criteria

- [ ] A received message can be replayed to a configured target address with a single click
- [ ] A template can be filled out in the UI, sent, and the response (ACK/NAK) displayed
- [ ] Raw HL7 can be edited and sent directly

---

## Milestone 5 – Persistence

**Goal:** Messages survive server restarts. Automatic cleanup after configurable retention.

### Requirements

- Milestone 1 complete (configuration file in place)

### Tasks

- [ ] **SQLite backend** – optional persistence, enabled via `hl7-forge.toml`
- [ ] **Retention policy** – automatic deletion after X days or X messages
- [ ] **Extended export** – CSV export, HL7 file export (`.hl7`), filtered exports

### Acceptance Criteria

- [ ] After a server restart, persisted messages are visible again
- [ ] Retention deletes messages automatically after configured age/count
- [ ] Export as CSV and `.hl7` works for filtered results

---

## Milestone 6 – FHIR & Monitoring

**Goal:** Future-proofing through FHIR preview and observability through a monitoring dashboard.

### Requirements

- Milestone 3 complete (message analysis as basis for FHIR mapping)

### Tasks

**FHIR R4 Preview**
- [ ] **HL7 v2 → FHIR R4 mapping** – display ADT messages as FHIR bundles (Patient, Encounter)
- [ ] **FHIR JSON view** – additional tab in the detail view
- [ ] **FHIR HTTP endpoint** – REST endpoint for FHIR bundles/resources

**Monitoring & Statistics**
- [ ] **Dashboard view** – messages per minute/hour, type distribution, error rate as charts
- [ ] **Latency tracking** – time difference between MSH-7 and receive time
- [ ] **Alerting** – configurable warnings at error rate > X% or outage > Y minutes
- [ ] **Health endpoint** – `/api/health` for monitoring tools (Zabbix, PRTG)

**Extended Features**
- [ ] **Multi-port listener** – multiple MLLP ports simultaneously (test separate Orchestra channels)
- [ ] **TLS support** – encrypted MLLP connections (MLLP/S)
- [ ] **ACK configuration** – customizable ACK responses (e.g. always send NAK to test retry logic)
- [ ] **Regex filter** – extended search with regular expressions
- [ ] **Dark/Light theme toggle**

### Acceptance Criteria

- [ ] ADT messages can be displayed as a FHIR bundle (JSON)
- [ ] Dashboard shows message volume and error rate as a chart
- [ ] `/api/health` returns server status for external monitoring tools
- [ ] Multi-port listener receives on configured ports in parallel
