# HL7 Forge – Milestones

Abgeleitet aus der [ROADMAP.md](ROADMAP.md), Phasen 2–4. Phase 1 (MVP) ist abgeschlossen, Phase 5 (Nice-to-Have) bewusst ausgeklammert.

---

## Milestone 1 – Team-Ready Server

**Goal:** HL7 Forge läuft stabil als Windows-Dienst auf dem Dev-Server, ist ohne Neucompilierung konfigurierbar und hält hoher Last stand.

### Requirements

- Phase 1 (MVP) komplett abgeschlossen

### Tasks

- [ ] **Konfigurationsdatei** (`hl7-forge.toml`) – Ports, Speicherlimits, Log-Level, Retention konfigurierbar
- [ ] **Windows Service** – als Windows-Dienst installierbar (`sc create` / NSSM), automatischer Start beim Serverboot
- [ ] **Startup-Banner im Event-Log** – Windows Event Log Integration für Ops-Monitoring
- [ ] **Portable Binary** – Single `.exe` ohne Abhängigkeiten, xcopy-Deployment
- [ ] **Backpressure-Handling** – bei vollem Store älteste Nachrichten evicten statt OOM
- [ ] **Memory-Budget konfigurierbar** – z.B. max 512 MB RAM, automatische Eviction
- [ ] **Connection Limits** – maximale gleichzeitige MLLP-Verbindungen begrenzen
- [ ] **Graceful Shutdown** – laufende Verbindungen sauber beenden bei Dienst-Stop

### Akzeptanzkriterien

- [ ] Server startet über `hl7-forge.toml` mit konfigurierten Ports und Limits
- [ ] Server läuft als Windows-Dienst und startet nach Reboot automatisch
- [ ] Bei Erreichen des Memory-Budgets werden alte Nachrichten evictet, kein OOM
- [ ] `Ctrl+C` bzw. Dienst-Stop beendet laufende MLLP-Verbindungen sauber

---

## Milestone 2 – Multi-User Experience

**Goal:** Mehrere Entwickler arbeiten gleichzeitig produktiv gegen denselben Server, ohne sich gegenseitig zu stören.

### Requirements

- Milestone 1 abgeschlossen (stabiler Server mit Konfiguration)

### Tasks

- [ ] **Session-basierte Ansichten** – jeder Entwickler sieht eigene Filterkonfiguration, Scroll-Position und Auswahl
- [ ] **Farbcodierte Quell-Markierung** – Nachrichten nach Absender-System/IP visuell unterscheidbar
- [ ] **Nachrichten-Tagging** – manuelles Taggen (z.B. "Bug #1234", "Test-Szenario A") zur Zuordnung
- [ ] **Bookmark/Pin** – wichtige Nachrichten markieren, damit sie nicht im Strom untergehen

### Akzeptanzkriterien

- [ ] Zwei Browser-Tabs zeigen unabhängige Filter und Auswahl
- [ ] Nachrichten von unterschiedlichen Quell-IPs sind visuell unterscheidbar
- [ ] Tags und Bookmarks bleiben über Page-Reload erhalten (Session-Scope)

---

## Milestone 3 – Nachrichten-Analyse

**Goal:** Entwickler verstehen HL7-Nachrichten direkt in der UI – Feldnamen, Validierung, Diff.

### Requirements

- Milestone 1 abgeschlossen (stabiler Server)

### Tasks

- [ ] **HL7-Feldwörterbuch** – Hover-Tooltips mit Feld-Beschreibungen (z.B. "PID-5: Patient Name") basierend auf HL7 v2.5/v2.6 Spec
- [ ] **Nachrichtentyp-Erkennung** – ADT, ORM, ORU, SIU, MDM etc. mit Kurzbeschreibung und typischen Segmenten
- [ ] **Validierung** – Pflichtfelder pro Nachrichtentyp prüfen, Warnungen anzeigen (z.B. "PID-3 fehlt in ADT^A01")
- [ ] **Segment-Vergleich (Diff)** – zwei Nachrichten nebeneinander vergleichen, Unterschiede hervorheben

### Akzeptanzkriterien

- [ ] Hover über ein HL7-Feld zeigt Namen und Beschreibung aus der Spec
- [ ] Validierungs-Warnungen erscheinen bei fehlenden Pflichtfeldern
- [ ] Diff-Ansicht zeigt Feldunterschiede zwischen zwei Nachrichten farblich markiert

---

## Milestone 4 – Workflow & Testing

**Goal:** Rapid-Testing gegen Orchestra – Nachrichten senden, editieren, wiederholen, direkt aus der UI.

### Requirements

- Milestone 1 abgeschlossen (stabiler Server)

### Tasks

- [ ] **Nachrichten-Replay** – gespeicherte Nachrichten erneut an konfigurierbare Zieladresse/Port senden (MLLP Client)
- [ ] **Test-Nachricht-Generator** – Templates für gängige Typen (ADT^A01, ORM^O01, ORU^R01) mit editierbaren Feldern
- [ ] **Nachrichten-Editor** – Raw-HL7 in der UI bearbeiten und absenden
- [ ] **Auto-Refresh Trigger** – Desktop-Notification bei neuen Nachrichten (optional)

### Akzeptanzkriterien

- [ ] Eine empfangene Nachricht kann per Klick an eine konfigurierte Zieladresse replayed werden
- [ ] Ein Template kann in der UI befüllt, abgeschickt und die Antwort (ACK/NAK) angezeigt werden
- [ ] Raw-HL7 kann editiert und direkt gesendet werden

---

## Milestone 5 – Persistenz

**Goal:** Nachrichten überleben Server-Neustarts. Automatische Bereinigung nach konfigurierbarer Retention.

### Requirements

- Milestone 1 abgeschlossen (Konfigurationsdatei vorhanden)

### Tasks

- [ ] **SQLite-Backend** – optionale Persistenz, aktivierbar über `hl7-forge.toml`
- [ ] **Retention-Policy** – automatisches Löschen nach X Tagen oder X Nachrichten
- [ ] **Export-Erweiterung** – CSV-Export, HL7-Datei-Export (`.hl7`), gefilterte Exports

### Akzeptanzkriterien

- [ ] Nach Server-Neustart sind persistierte Nachrichten wieder sichtbar
- [ ] Retention löscht Nachrichten automatisch nach konfiguriertem Alter/Anzahl
- [ ] Export als CSV und `.hl7` funktioniert für gefilterte Ergebnisse

---

## Milestone 6 – FHIR & Monitoring

**Goal:** Zukunftssicherheit durch FHIR-Preview und Observability durch Monitoring-Dashboard.

### Requirements

- Milestone 3 abgeschlossen (Nachrichten-Analyse als Basis für FHIR-Mapping)

### Tasks

**FHIR R4 Preview**
- [ ] **HL7 v2 → FHIR R4 Mapping** – ADT-Nachrichten als FHIR Bundle anzeigen (Patient, Encounter)
- [ ] **FHIR JSON-Ansicht** – zusätzlicher Tab in der Detailansicht
- [ ] **FHIR HTTP Endpoint** – REST-Endpunkt für FHIR Bundles/Ressourcen

**Monitoring & Statistiken**
- [ ] **Dashboard-Ansicht** – Nachrichten pro Minute/Stunde, Typ-Verteilung, Error-Rate als Charts
- [ ] **Latenz-Tracking** – Zeitdifferenz zwischen MSH-7 und Empfangszeitpunkt
- [ ] **Alerting** – konfigurierbare Warnungen bei Fehlerquote > X% oder Ausfall > Y Minuten
- [ ] **Health-Endpoint** – `/api/health` für Monitoring-Tools (Zabbix, PRTG)

**Erweiterte Features**
- [ ] **Multi-Port-Listener** – mehrere MLLP-Ports gleichzeitig (getrennte Orchestra-Channels testen)
- [ ] **TLS-Support** – verschlüsselte MLLP-Verbindungen (MLLP/S)
- [ ] **Acknowledgement-Konfiguration** – anpassbare ACK-Antworten (z.B. immer NAK zum Testen von Retry-Logik)
- [ ] **Regex-Filter** – erweiterte Suche mit regulären Ausdrücken
- [ ] **Dark/Light Theme Toggle**

### Akzeptanzkriterien

- [ ] ADT-Nachrichten können als FHIR Bundle (JSON) angezeigt werden
- [ ] Dashboard zeigt Nachrichtenvolumen und Fehlerrate als Diagramm
- [ ] `/api/health` liefert Serverstatus für externe Monitoring-Tools
- [ ] Multi-Port-Listener empfängt auf konfigurierten Ports parallel
