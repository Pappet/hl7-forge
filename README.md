# HL7 Forge

High-performance HL7 MLLP server with real-time Web UI, built in Rust.

HL7 Forge ersetzt den HL7 Inspector als zentrales Testwerkzeug im Schnittstellen-Team. Es läuft als Service auf dem Dev-Server und wird von mehreren Entwicklern gleichzeitig über den Browser genutzt – kein lokales Setup, keine RDP-exklusiven Fenster.

## Features

- **MLLP Server** – Empfängt HL7 v2.x Nachrichten mit korrektem MLLP-Framing, antwortet mit ACK/NAK
- **Echtzeit Web UI** – Browser-basierte SPA mit WebSocket-Updates, keine Installation nötig
- **Nachrichtendetails** – Parsed-, Raw- und JSON-Ansicht pro Nachricht
- **Suche & Filter** – Nachrichtentyp, Patient, Facility, ID
- **JSON-Export** – Nachrichten als JSON exportieren
- **Single Binary** – Keine Abhängigkeiten, Frontend ist in die Binary eingebettet

## Quick Start

```bash
# Build
cargo build --release

# Run (Defaults: MLLP auf Port 2575, Web UI auf Port 8080)
cargo run --release

# Custom Ports
MLLP_PORT=4000 WEB_PORT=9090 cargo run --release
```

Web UI öffnen: **http://localhost:8080**

## Betrieb unter Windows

HL7 Forge ist als portable Single-Binary für Windows konzipiert. Die `.exe` kann direkt ausgeführt oder als Windows-Dienst installiert werden.

### Download

Die Windows-Binary wird automatisch per GitHub Actions gebaut. Download:

- **Releases:** Unter [GitHub Releases](../../releases) steht die `hl7-forge.exe` bei jedem Release zum Download
- **Artifacts:** Jeder Push auf `main` erzeugt ein Build-Artifact (`hl7-forge-windows`) unter [Actions](../../actions)

### Direktstart

```powershell
# Einfach starten
.\hl7-forge.exe

# Mit angepassten Ports
$env:MLLP_PORT="4000"; $env:WEB_PORT="9090"; .\hl7-forge.exe
```

### Als Windows-Dienst (mit NSSM)

```powershell
# NSSM installieren (https://nssm.cc)
nssm install HL7Forge C:\Tools\hl7-forge.exe
nssm set HL7Forge AppEnvironmentExtra MLLP_PORT=2575 WEB_PORT=8080
nssm start HL7Forge
```

Der Dienst startet automatisch beim Serverboot und ist unter `http://<server>:8080` erreichbar.

## Architektur

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

| Endpoint | Method | Beschreibung |
|---|---|---|
| `/api/messages?offset=0&limit=100` | GET | Nachrichtenliste (neueste zuerst) |
| `/api/messages/{id}` | GET | Nachricht mit allen Segmenten |
| `/api/search?q=ADT&limit=100` | GET | Nachrichten durchsuchen |
| `/api/stats` | GET | Server-Statistiken |
| `/api/clear` | POST | Alle Nachrichten löschen |
| `/ws` | WS | Echtzeit-Updates |

## Testen

```bash
# Unit Tests
cargo test

# Test-Nachricht per MLLP senden (netcat)
printf '\x0bMSH|^~\\&|TESTSYS|TESTFAC|HL7FORGE|HL7FORGE|20240101120000||ADT^A01|MSG001|P|2.5\rPID|||12345||Doe^John||19900101|M\rPV1||I|ICU^101^A\x1c\r' | nc localhost 2575

# Oder das mitgelieferte Testskript
./test.sh
```

## Roadmap

Die vollständige Roadmap ist in [ROADMAP.md](ROADMAP.md) beschrieben, die daraus abgeleiteten Milestones in [MILESTONES.md](MILESTONES.md).

| Milestone | Beschreibung |
|---|---|
| 1 – Team-Ready Server | Windows-Dienst, Konfigurationsdatei, Backpressure, Graceful Shutdown |
| 2 – Multi-User Experience | Sessions, Quell-Markierung, Tagging, Bookmarks |
| 3 – Nachrichten-Analyse | Feldwörterbuch, Validierung, Nachrichtentyp-Erkennung, Diff |
| 4 – Workflow & Testing | Replay, Nachricht-Editor, Test-Generator |
| 5 – Persistenz | SQLite, Retention-Policy, erweiterte Exports |
| 6 – FHIR & Monitoring | FHIR R4 Preview, Dashboard, Alerting, Health-Endpoint |

## Technologie

| Thema | Entscheidung |
|---|---|
| Sprache | Rust |
| Async Runtime | Tokio |
| Web Framework | Axum |
| UI | Embedded SPA (HTML/JS/CSS) |
| CI/CD | GitHub Actions (Windows Build) |
| Deployment | Single `.exe`, xcopy-deploy |
