# Changelog

Alle nennenswerten Änderungen an diesem Projekt werden in dieser Datei dokumentiert.

Das Format basiert auf [Keep a Changelog](https://keepachangelog.com/de/1.0.0/)
und dieses Projekt folgt [Semantic Versioning](https://semver.org/lang/de/).

---

## [Unreleased]

### Geplant – Milestone 1: Team-Ready Server

> Ziel: HL7 Forge läuft stabil als Windows-Dienst auf dem Dev-Server, ist ohne Neucompilierung konfigurierbar und hält hoher Last stand.

- **Konfigurationsdatei** (`hl7-forge.toml`) — Ports, Speicherlimits, Log-Level und Retention ohne Neucompilierung konfigurierbar
- **Windows Service** — Installierbar via `sc create` / NSSM, automatischer Start nach Serverreboot
- **Windows Event Log Integration** — Startup-Banner im Windows Event Log für Ops-Monitoring
- **Portable Binary** — Single `.exe` ohne Laufzeitabhängigkeiten, xcopy-Deployment
- **Backpressure-Handling** — Bei vollem Store werden älteste Nachrichten evictet statt OOM
- **Memory-Budget** — Konfigurierbares RAM-Limit (z. B. 512 MB), automatische Eviction bei Überschreitung
- **Connection Limits** — Maximale gleichzeitige MLLP-Verbindungen konfigurierbar begrenzen
- **Graceful Shutdown** — Laufende Verbindungen sauber beenden bei `Ctrl+C` oder Dienst-Stop

### Geplant – Milestone 2: Multi-User Experience

> Ziel: Mehrere Entwickler arbeiten gleichzeitig produktiv gegen denselben Server, ohne sich gegenseitig zu stören.

- **Session-basierte Ansichten** — Jeder Entwickler sieht eigene Filterkonfiguration, Scroll-Position und Auswahl
- **Farbcodierte Quell-Markierung** — Nachrichten nach Absender-System / IP visuell unterscheidbar
- **Nachrichten-Tagging** — Manuelles Taggen von Nachrichten (z. B. „Bug #1234", „Test-Szenario A")
- **Bookmark / Pin** — Wichtige Nachrichten markieren, damit sie nicht im Live-Stream untergehen

### Geplant – Milestone 3: Nachrichten-Analyse

> Ziel: Entwickler verstehen HL7-Nachrichten direkt in der UI – Feldnamen, Validierung, Diff.

- **HL7-Feldwörterbuch** — Hover-Tooltips mit Feldbeschreibungen (z. B. „PID-5: Patient Name") basierend auf HL7 v2.5/v2.6 Spec
- **Nachrichtentyp-Erkennung** — ADT, ORM, ORU, SIU, MDM etc. mit Kurzbeschreibung und typischen Segmenten
- **Validierung** — Pflichtfelder pro Nachrichtentyp prüfen, Warnungen anzeigen (z. B. „PID-3 fehlt in ADT^A01")
- **Segment-Vergleich (Diff)** — Zwei Nachrichten nebeneinander vergleichen, Unterschiede farblich hervorheben

### Geplant – Milestone 4: Workflow & Testing

> Ziel: Rapid-Testing gegen Orchestra – Nachrichten senden, editieren, wiederholen, direkt aus der UI.

- **Nachrichten-Replay** — Gespeicherte Nachrichten erneut an konfigurierbare Zieladresse/Port senden (MLLP-Client)
- **Test-Nachricht-Generator** — Templates für ADT^A01, ORM^O01, ORU^R01 mit editierbaren Feldern
- **Nachrichten-Editor** — Raw-HL7 in der UI bearbeiten und direkt absenden
- **Auto-Refresh Trigger** — Desktop-Notification bei eingehenden Nachrichten (optional)

### Geplant – Milestone 5: Persistenz

> Ziel: Nachrichten überleben Server-Neustarts.

- **SQLite-Backend** — Optionale Persistenz, aktivierbar über `hl7-forge.toml`
- **Retention-Policy** — Automatisches Löschen nach X Tagen oder X Nachrichten
- **Export-Erweiterung** — CSV-Export, HL7-Datei-Export (`.hl7`), gefilterte Exports

### Geplant – Milestone 6: FHIR & Monitoring

- **HL7 v2 → FHIR R4 Mapping** — ADT-Nachrichten als FHIR Bundle anzeigen (Patient, Encounter)
- **FHIR JSON-Ansicht** — Zusätzlicher Tab in der Detailansicht
- **FHIR HTTP Endpoint** — REST-Endpunkt für FHIR Bundles / Ressourcen
- **Dashboard-Ansicht** — Nachrichten pro Minute/Stunde, Nachrichtentyp-Verteilung, Error-Rate als Charts
- **Latenz-Tracking** — Zeitdifferenz zwischen MSH-7 (Message Timestamp) und Empfangszeitpunkt
- **Alerting** — Konfigurierbare Warnungen bei Fehlerquote > X % oder Nachrichtenausfall > Y Minuten
- **Health-Endpoint** — `/api/health` für Monitoring-Tools (Zabbix, PRTG etc.)
- **Multi-Port-Listener** — Mehrere MLLP-Ports gleichzeitig (z. B. 2575 für ADT, 2576 für ORM)
- **TLS-Support** — Verschlüsselte MLLP-Verbindungen (MLLP/S)
- **Acknowledgement-Konfiguration** — Anpassbare ACK-Antworten (z. B. immer NAK zum Testen von Retry-Logik)
- **Regex-Filter** — Erweiterte Suche mit regulären Ausdrücken über alle Felder
- **Dark/Light Theme Toggle**

---

## [0.1.0] – 2026-02 – MVP / Phase 1 ✅

> Erstes lauffähiges Release. Grundgerüst fertig, HL7-Nachrichten werden über MLLP empfangen und in der Web-UI in Echtzeit angezeigt.

---

### Commit-Historie (chronologisch)

#### 2026-02-21

| Commit | Beschreibung |
|--------|-------------|
| [`c967107`](https://github.com/Pappet/hl7-forge/commit/c967107) | `docs:` Changelog-Pflicht in CLAUDE.md und AGENTS.md dokumentiert |
| [`79cec90`](https://github.com/Pappet/hl7-forge/commit/79cec901f6141ba32c719dda2829e15d20f9f5d3) | `docs:` CLAUDE.md aktualisiert, AGENTS.md mit Architektur- und Deployment-Kontext hinzugefügt |
| [`041cb04`](https://github.com/Pappet/hl7-forge/commit/041cb0417f821b354fce1c021148d4e18a78cd01) | `fix:` ACK-Storm-Prevention, Search-Debounce (300 ms), größenbasierte Store-Eviction (`MAX_STORE_BYTES`) |
| [`44afeb9`](https://github.com/Pappet/hl7-forge/commit/44afeb9364ce45d2073da84cd840d72ccd6e1882) | `.gitignore` angepasst |

#### 2026-02-20

| Commit | Beschreibung |
|--------|-------------|
| [`1ee69f3`](https://github.com/Pappet/hl7-forge/commit/1ee69f3655a286ea6dfeb2bec71d49f6f56a5270) | `.gitignore` erweitert |
| [`2f45c7e`](https://github.com/Pappet/hl7-forge/commit/2f45c7e19f1f8c966e857396bc1df178bc0abb4f) | `ux:` Diverse UX-Optimierungen |
| [`f10cd40`](https://github.com/Pappet/hl7-forge/commit/f10cd40efab007e5d629942ed6cd91611488d468) | `feat:` Fehlerhafte Nachrichten werden im Store gespeichert; UI-Batching (250 ms) eingeführt; statische Assets in separate Dateien ausgelagert (`index.html`, `style.css`, `app.js`) |
| [`ee0dcda`](https://github.com/Pappet/hl7-forge/commit/ee0dcda6cd673733bd37e9ab01e9b6933625264b) | `docs:` PowerShell-Testskript (`tests/test.ps1`) hinzugefügt; beide Test-Runner dokumentiert |
| [`1bd7c9f`](https://github.com/Pappet/hl7-forge/commit/1bd7c9f841fee412d172599e137e8a91bb06eb16) | PowerShell-Testskript für Windows-Load-Tests erstellt (1000 Nachrichten, persistente TCP-Verbindung) |
| [`7a08611`](https://github.com/Pappet/hl7-forge/commit/7a0861102763aed1898eb4c24f1d344978a92cb7) | Social-Card-Bild umbenannt (`hl7-forge-card.png` → `social-card.png`) |
| [`fd7178a`](https://github.com/Pappet/hl7-forge/commit/fd7178ae69eb4dfe90fb8e5725c2d51db15e9da6) | `docs:` README.md vollständig überarbeitet |
| [`a6860b1`](https://github.com/Pappet/hl7-forge/commit/a6860b125d40031c131a9c8dc4a9f7e64c3d7b10) | `.gitignore` erweitert |
| [`71eb33c`](https://github.com/Pappet/hl7-forge/commit/71eb33cdb4d232977b4045c26531de2e4f3b8b0a) | `ci:` Release-Upload von `softprops/action-gh-release` auf `gh release upload` umgestellt |
| [`564aba3`](https://github.com/Pappet/hl7-forge/commit/564aba33437afc23e8657f8518982c89366414a8) | `ci:` Build-Pipeline auf drei unabhängige Jobs vereinfacht: Windows, macOS Apple Silicon, Linux |
| [`c8711e8`](https://github.com/Pappet/hl7-forge/commit/c8711e840d6e1db31867f0577be01a4d2ec03fda) | `ci:` Intel-macOS-Binary wird cross-compiled auf Apple-Silicon-Runner |
| [`acc9908`](https://github.com/Pappet/hl7-forge/commit/acc9908b5c222fbfa7956d87c8cabe0083c27dd1) | `ci:` macOS-Builds für Intel und Apple Silicon hinzugefügt |
| [`8c0db5f`](https://github.com/Pappet/hl7-forge/commit/8c0db5fd3821715e03bae3c8a7f17ac90cfeff7d) | `ci:` `contents: write`-Permission für Release-Asset-Upload gesetzt |
| [`9a14291`](https://github.com/Pappet/hl7-forge/commit/9a14291ba9281c9b135cf8d3b07ae73934db7cbd) | MIT-Lizenz (`LICENSE`) zum Projekt hinzugefügt |
| [`5bd3579`](https://github.com/Pappet/hl7-forge/commit/5bd357946f568dc7806ea0ca38f4367023c014a4) | `docs:` ACK-Verhalten für unbekannte Nachrichtentypen klargestellt; `test.sh` überarbeitet |
| [`9ef8ed9`](https://github.com/Pappet/hl7-forge/commit/9ef8ed9cf631b442ebf6e10d7767fcdff5b6e3c1) | `fix:` MSH-Feldindizes auf HL7-Standard ausgerichtet (+1-Offset korrigiert); Graceful Shutdown via `Ctrl+C`-Signal-Handler eingebaut |
| [`bbde980`](https://github.com/Pappet/hl7-forge/commit/bbde98060ccc46fbe5a9ed238907a73ebcd9af21) | `fix:` MLLP-Server und Message Store gegen Lastspitzen und DoS gehärtet (Connection-Timeouts, Payload-Limit 10 MB) |
| [`fa11aa4`](https://github.com/Pappet/hl7-forge/commit/fa11aa42ae65bde3926349bea3ee81d2b3d9714c) | `polish:` Cargo-Metadaten ergänzt; Tokio-Features bereinigt; Toast-Benachrichtigungen in der UI eingeführt |
| [`f33ccfc`](https://github.com/Pappet/hl7-forge/commit/f33ccfcccdfb12e16f4879e06fb8a3a9b8802919) | `fix:` UI-Polish und Pre-Release-Fixes (Axum-Route `:id` korrigiert, Compiler-Warnings bereinigt) |
| [`696522c`](https://github.com/Pappet/hl7-forge/commit/696522c4126fc45096687fdb5ef38d6462f593b2) | `docs:` README mit Feature-Übersicht, Windows-Deployment-Guide und Milestone-Tabelle überarbeitet |
| [`679aad3`](https://github.com/Pappet/hl7-forge/commit/679aad3e02d1d0c89299ad8f22b38e95b10bb37c) | `ci:` GitHub Actions Build-Workflow initial eingerichtet |
| [`0c46811`](https://github.com/Pappet/hl7-forge/commit/0c468114f5080b9da02ea6e3b4a22796e56337f2) | `docs:` ROADMAP.md als strategisches Planungsdokument hinzugefügt |
| [`6cedfc6`](https://github.com/Pappet/hl7-forge/commit/6cedfc6b8cd47c94cf475d6905a00b53f7540fc1) | `docs:` MILESTONES.md mit 6 strukturierten Meilensteinen aus ROADMAP Phasen 2–4 erstellt |
| [`f087a62`](https://github.com/Pappet/hl7-forge/commit/f087a62b435ecf3d8e6e7d9dc7c5902f4d9d8b82) | `docs:` CLAUDE.md mit Build-Kommandos und Architekturübersicht für AI-Agenten hinzugefügt |
| [`f6fef07`](https://github.com/Pappet/hl7-forge/commit/f6fef074115caf756797f5257578349c583c7bec) | **Initial Commit:** HL7 Forge MLLP-Server mit Echtzeit-Web-UI |

---

### Added

#### MLLP Server (Backend)
- Asynchroner TCP-Listener auf Basis von **Tokio** (`rt-multi-thread`)
- Korrektes MLLP-Framing: Start-Block `0x0B`, End-Block `0x1C 0x0D` gemäß HL7 MLLP-Standard
- **ACK/NAK-Generierung**: Automatische Antwort mit `AA` (Application Accept) bei gültigen Nachrichten und unbekannten Nachrichtentypen; `AE` (Application Error) bei fehlendem oder fehlerhaftem MSH-Segment
- **ACK-Loop-Schutz**: Eingehende ACK-Nachrichten werden nie selbst mit ACK beantwortet
- Parallele Client-Verbindungen über `tokio::spawn` pro Verbindung
- **DoS-Härtung**: Payload-Limit von 10 MB, Connection-Timeouts
- **Graceful Shutdown**: Sauberes Beenden laufender Verbindungen bei `Ctrl+C` via Signal-Handler

#### HL7 v2.x Parser
- Dynamische Delimiter-Erkennung aus MSH-Segment (Feldtrennzeichen, Komponenten-, Subkomponenten-, Escape- und Wiederholungs-Separator)
- Extraktion der wichtigsten MSH-Felder: Nachrichtentyp, Trigger-Event, Sending/Receiving Facility & Application, Message ID, Timestamp
- MSH-Feldindizes korrekt nach HL7-Standard ausgerichtet (+1-Offset)
- PID-Segmentextraktion: Patient-ID (PID-3), Patientenname (PID-5)
- Robuste Fehlerbehandlung: Parse-Fehler führen zu `⚠ PARSE ERROR`-Markierung in der UI, nicht zu einem Server-Absturz; fehlerhafte Nachrichten werden im Store gespeichert
- Strukturierte Segment- und Feld-Repräsentation (`Hl7Message`, `Hl7Segment`, `Hl7Field`, `Delimiters`)

#### In-Memory Message Store
- Zentraler Store mit `Arc<RwLock<>>` für thread-sicheren Zugriff
- **Dual-Eviction**: Nachrichten werden evictet, wenn entweder die maximale Anzahl (`MAX_STORE_CAPACITY`) oder das maximale Speichervolumen (`MAX_STORE_BYTES`) überschritten wird — kein OOM
- Broadcast-Channel (`tokio::sync::broadcast`) für Echtzeit-Benachrichtigung aller aktiven WebSocket-Clients bei neuen Nachrichten
- Jede Nachricht erhält eine eindeutige UUID v4 und einen ISO 8601-Empfangszeitpunkt

#### Web API (Axum)
- `GET /api/messages` — Liste aller gespeicherten Nachrichten (paginiert, als `Hl7MessageSummary`)
- `GET /api/messages/:id` — Vollständige Nachrichtendetails inklusive geparster Segmente und Raw-HL7
- `GET /api/search` — Suchendpunkt mit Query-Parameter `q`
- `GET /api/stats` — Live-Statistiken: Empfangene Nachrichten, Parse-Fehler, aktive Verbindungen, MLLP-Port
- `POST /api/clear` — Store leeren
- `GET /ws` — WebSocket-Endpunkt für Echtzeit-Nachrichten-Push
- CORS-Middleware für Browser-Zugriff
- Eingebettete statische Dateien via `rust-embed` (kein separater Webserver notwendig)

#### Web-UI (Embedded SPA, Vanilla JS)
- **Echtzeit-Nachrichtenliste** via WebSocket — neue Nachrichten erscheinen sofort ohne Page-Reload
- **Batch-Rendering** alle 250 ms — verhindert DOM-Einfrieren bei hohem Nachrichtenvolumen
- **Pause / Live-Modus** — `⏸`-Button puffert eingehende Nachrichten; `▶`-Button leert den Buffer und kehrt in den Live-Modus zurück
- **Toast-Benachrichtigungen** — diskrete In-App-Notifications bei relevanten Ereignissen
- **Detailansicht** mit drei Tabs: `Parsed`, `Raw`, `JSON`
- **Client-seitiger Suchfilter** (debounced, 300 ms) — Filterung nach Nachrichtentyp, Patient, Facility, Message Control ID
- **JSON-Export** — einzelne Nachrichten als `.json`-Datei herunterladen
- Dark Theme mit CSS-Variablen
- `⚠ PARSE ERROR`-Markierung in Rot für fehlerhafte Nachrichten

#### Build & Deployment
- Single Rust Binary, keine externen Laufzeitabhängigkeiten
- **GitHub Actions CI/CD**: Drei unabhängige Build-Jobs für Windows (`.exe`), macOS Apple Silicon und Linux bei jedem Push auf `main`
- Build-Artefakte werden automatisch an GitHub Releases angehängt
- MIT-Lizenz
- Testskripte: `tests/test.sh` (Linux/macOS, netcat, 100 Nachrichten) und `tests/test.ps1` (Windows, .NET TcpClient, 1000 Nachrichten, persistente Verbindung)

### Fixed

- MSH-Feldindizes auf korrekten HL7-Standard-Offset ausgerichtet (`9ef8ed9`)
- Route für die Message-Detail-Ansicht von `{id}` auf `:id` (korrekte Axum-Syntax) korrigiert (`f33ccfc`)
- Compiler-Warnings (`dead_code`, `unused_variables`) bereinigt (`f33ccfc`)
- ACK-Storm-Prevention: Eingehende ACK-Nachrichten werden erkannt und nicht erneut beantwortet (`041cb04`)

### Technical Stack

| Komponente        | Crate / Technologie             | Version |
|-------------------|---------------------------------|---------|
| Async Runtime     | `tokio`                         | 1.x     |
| Web Framework     | `axum`                          | 0.7     |
| HTTP Middleware   | `tower-http` (CORS, static FS)  | 0.5     |
| Serialisierung    | `serde` + `serde_json`          | 1.x     |
| Zeitstempel       | `chrono`                        | 0.4     |
| UUID-Generierung  | `uuid` (v4)                     | 1.x     |
| Logging           | `tracing` + `tracing-subscriber`| 0.1/0.3 |
| Static Files      | `rust-embed` + `mime_guess`     | 8.x/2.x |
| Fehlerbehandlung  | `anyhow`                        | 1.x     |
| Frontend          | Vanilla JS / HTML / CSS         | —       |

### Known Issues

- Speicherlimit (`MAX_STORE_BYTES`) ist derzeit hardcodiert — wird in Milestone 1 über `hl7-forge.toml` konfigurierbar

---

[Unreleased]: https://github.com/Pappet/hl7-forge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Pappet/hl7-forge/releases/tag/v0.1.0