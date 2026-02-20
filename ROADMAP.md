# HL7 Forge – Roadmap

## Vision

HL7 Forge ersetzt den HL7 Inspector als primäres Testwerkzeug im Schnittstellen-Team. Es läuft als zentraler Service auf dem Dev-Server (Windows Server) und wird von mehreren Entwicklern gleichzeitig über den Browser genutzt – kein lokales Setup, keine RDP-exklusiven Fenster, keine Performance-Einbrüche bei hohem Nachrichtenvolumen.

## Einsatzkontext

- **Team:** Schnittstellen-Team (Mehrere Entwickler gleichzeitig)
- **Software:** Orchestra (Schnittstellenentwicklung)
- **Infrastruktur:** Windows Server, Zugriff per RDP + Browser
- **Protokoll:** HL7 v2.x über MLLP, perspektivisch FHIR R4

---

## Phase 1 – Solides Fundament (MVP) ✅

*Status: Grundgerüst fertig, erste Nachrichten werden empfangen*

- [x] MLLP Server mit Tokio (async TCP, korrekte `0x0B`/`0x1C 0x0D` Framing)
- [x] ACK/NAK Antworten (AA, AE)
- [x] HL7 v2.x Parser (dynamische Delimiter-Erkennung, MSH/PID-Extraktion)
- [x] In-Memory Message Store mit Broadcast-Channel
- [x] Web UI mit Echtzeit-Nachrichtenliste (WebSocket)
- [x] Segment/Feld-Detailansicht (Parsed, Raw, JSON Tabs)
- [x] Suchfilter (Nachrichtentyp, Patient, Facility, ID)
- [x] JSON-Export
- [ ] Route-Fix für Message-Detail-Ansicht (`:id` statt `{id}`)
- [ ] Compiler-Warnings bereinigen

## Phase 2 – Team-Tauglichkeit

*Fokus: Mehrere Entwickler arbeiten gleichzeitig gegen denselben Server*

### Multi-User & Sessions
- [ ] **Session-basierte Ansichten** – jeder Entwickler sieht seine eigene Filterkonfiguration, Scroll-Position und Auswahl, ohne andere zu beeinflussen
- [ ] **Farbcodierte Quell-Markierung** – Nachrichten nach Absender-System/IP visuell unterscheidbar (z.B. Orchestra Dev vs. Orchestra Test)
- [ ] **Nachrichten-Tagging** – manuelles Taggen von Nachrichten (z.B. "Bug #1234", "Test-Szenario A") zur Zuordnung bei gemeinsamer Nutzung
- [ ] **Bookmark/Pin** – wichtige Nachrichten markieren, damit sie nicht im Strom untergehen

### Windows Server Deployment
- [ ] **Windows Service** – HL7 Forge als Windows-Dienst (`sc create` / NSSM), automatischer Start beim Serverboot
- [ ] **Konfigurationsdatei** (`hl7-forge.toml`) – Ports, Speicherlimits, Log-Level, Retention konfigurierbar ohne Neucompilierung
- [ ] **Startup-Banner im Event-Log** – Windows Event Log Integration für Ops-Monitoring
- [ ] **Portable Binary** – Single `.exe`, keine Abhängigkeiten, xcopy-Deployment

### Stabilität & Performance
- [ ] **Backpressure-Handling** – wenn der Store voll wird, älteste Nachrichten evicten statt OOM
- [ ] **Memory-Budget konfigurierbar** – z.B. max 512 MB RAM, automatische Eviction
- [ ] **Connection Limits** – maximale gleichzeitige MLLP-Verbindungen begrenzen
- [ ] **Graceful Shutdown** – laufende Verbindungen sauber beenden bei Dienst-Stop

## Phase 3 – Orchestra-Integration & Workflow

*Fokus: Nahtlose Integration in den Schnittstellenentwicklungs-Workflow mit Orchestra*

### Nachrichten-Analyse
- [ ] **HL7-Feldwörterbuch** – Hover-Tooltips zeigen Feld-Beschreibungen (z.B. "PID-5: Patient Name", "OBR-4: Universal Service Identifier") basierend auf HL7 v2.5/v2.6 Spec
- [ ] **Nachrichtentyp-Erkennung** – ADT, ORM, ORU, SIU, MDM etc. mit Kurzbeschreibung und typischen Segmenten
- [ ] **Validierung** – Pflichtfelder pro Nachrichtentyp prüfen, Warnungen anzeigen (z.B. "PID-3 fehlt in ADT^A01")
- [ ] **Segment-Vergleich (Diff)** – zwei Nachrichten nebeneinander vergleichen, Unterschiede hervorheben (ideal zum Testen von Orchestra-Transformationen)

### Workflow-Features
- [ ] **Nachrichten-Replay** – gespeicherte Nachrichten erneut an eine konfigurierbare Zieladresse/Port senden (MLLP Client-Modus)
- [ ] **Test-Nachricht-Generator** – Templates für gängige Nachrichtentypen (ADT^A01, ORM^O01, ORU^R01) mit editierbaren Feldern, direkt aus der UI absenden
- [ ] **Nachrichten-Editor** – Raw-HL7 direkt in der UI bearbeiten und absenden (Rapid-Testing gegen Orchestra-Channels)
- [ ] **Auto-Refresh Trigger** – WebSocket-basierte Benachrichtigung, wenn neue Nachrichten eintreffen, optional mit Desktop-Notification

### Persistenz
- [ ] **SQLite-Backend** – optionale Persistenz, damit Nachrichten einen Server-Neustart überleben
- [ ] **Retention-Policy** – automatisches Löschen nach X Tagen / X Nachrichten
- [ ] **Export-Erweiterung** – CSV-Export, HL7-Datei-Export (`.hl7`), gefilterte Exports

## Phase 4 – FHIR & Erweiterte Analyse

*Fokus: Zukunftssicherheit und tiefere Einblicke*

### FHIR R4 Preview
- [ ] **HL7 v2 → FHIR R4 Mapping** – ADT-Nachrichten als FHIR Bundle anzeigen (Patient, Encounter, etc.)
- [ ] **FHIR JSON-Ansicht** – zusätzlicher Tab in der Detailansicht
- [ ] **FHIR HTTP Endpoint** – REST-Endpunkt der FHIR Bundles/Ressourcen empfangen kann (für zukünftige Orchestra-FHIR-Channels)

### Monitoring & Statistiken
- [ ] **Dashboard-Ansicht** – Nachrichten pro Minute/Stunde, Nachrichtentyp-Verteilung, Error-Rate als Charts
- [ ] **Latenz-Tracking** – Zeitdifferenz zwischen MSH-7 (Message Timestamp) und Empfangszeitpunkt
- [ ] **Alerting** – konfigurierbare Warnungen bei Fehlerquote > X% oder Nachrichtenausfall > Y Minuten
- [ ] **Health-Endpoint** – `/api/health` für Monitoring-Tools (Zabbix, PRTG etc.)

### Erweiterte Features
- [ ] **Multi-Port-Listener** – mehrere MLLP-Ports gleichzeitig, z.B. Port 2575 für ADT, 2576 für ORM (getrennte Orchestra-Channels testen)
- [ ] **TLS-Support** – verschlüsselte MLLP-Verbindungen (MLLP/S)
- [ ] **Acknowledgement-Konfiguration** – anpassbare ACK-Antworten (z.B. immer NAK senden zum Testen von Orchestra-Retry-Logik)
- [ ] **Regex-Filter** – erweiterte Suche mit regulären Ausdrücken über alle Felder
- [ ] **Dark/Light Theme Toggle**

## Phase 5 – Nice-to-Have

*Keine Priorität, aber nützlich wenn Zeit da ist*

- [ ] **Plugin-System** – eigene Parser/Transformer als WASM-Module laden
- [ ] **REST API für CI/CD** – Nachrichten programmatisch senden und Ergebnisse prüfen (Automated Integration Tests für Orchestra-Channels)
- [ ] **Audit-Log** – wer hat wann welche Nachricht angesehen/gesendet
- [ ] **Import aus HL7 Inspector** – bestehende Nachrichtensammlungen übernehmen
- [ ] **Orchestra Log-Korrelation** – Nachrichten-ID mit Orchestra Channel-Logs verknüpfen

---

## Technische Entscheidungen

| Thema | Entscheidung | Begründung |
|---|---|---|
| Sprache | Rust | Performance, Memory Safety, Single Binary |
| Async Runtime | Tokio | Bewährt, hoher Durchsatz, geringe Latenz |
| Web Framework | Axum | Tokio-nativ, typsicher, performant |
| UI | Embedded SPA (HTML/JS) | Zero Dependencies, Browser-basiert, Multi-User |
| Persistenz | In-Memory (Phase 1-2), SQLite (Phase 3+) | Einfacher Start, Persistenz wenn nötig |
| Deployment | Single `.exe` als Windows Service | Kein Installer, kein Runtime, xcopy-deploy |

## Nicht-Ziele

- **Kein vollständiger HL7-Editor** – HL7 Forge ist primär ein Empfangs- und Analyse-Tool, kein Ersatz für Orchestra
- **Keine Datenbank-Integration** – wir speichern keine Nachrichten dauerhaft in SQL-Datenbanken
- **Kein HL7 Router** – Nachrichten-Routing und Transformation bleibt in Orchestra
