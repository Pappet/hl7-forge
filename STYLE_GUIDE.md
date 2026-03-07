# HL7 Forge — Style Guide

This is the reference for all appearance and functioning rules of HL7 Forge.

---

## 1. Color Palette (Dark Theme)

| Role | Value |
|---|---|
| **Primary Background** | `#0f1117` |
| **Secondary Background** (panels, headers) | `#1a1d27` |
| **Tertiary Background** (hover, active, popups) | `#242736` |
| **Accent Hover** | `#2a2e3f` |
| **Borders** | `#2e3247` |
| **Primary Text** | `#e4e6f0` |
| **Secondary Text** | `#8b8fa3` |
| **Muted Text** | `#5c6078` |
| **Accent (Blue)** | `#6c8cff` (dimmed: `#4a62b3`) |
| **Success (Green)** | `#4caf84` |
| **Warning (Yellow)** | `#e5a54b` |
| **Error (Red)** | `#e05454` |

### Source Marker Palette

Used for color-coded dots in the message list (mapped by sender IP/port):

```
hsl(210, 90%, 65%)  — Blue       hsl(150, 70%, 55%)  — Green
hsl(30,  85%, 60%)  — Orange     hsl(280, 75%, 65%)  — Purple
hsl(0,   80%, 62%)  — Red        hsl(180, 70%, 50%)  — Teal
hsl(50,  85%, 55%)  — Gold       hsl(330, 75%, 62%)  — Pink
hsl(200, 80%, 55%)  — Sky        hsl(100, 60%, 50%)  — Lime
hsl(260, 65%, 60%)  — Indigo     hsl(15,  90%, 58%)  — Coral
```

---

## 2. Typography

- **UI elements:** System fonts — `-apple-system, system-ui, sans-serif`
- **Data / code:** Monospace — `SF Mono, Cascadia Code, Consolas, monospace`
- **Base font size:** 12–13px for data-dense views
- **Headers:** Subtle uppercase with small letter-spacing for grid/table headers

---

## 3. Spacing & Density

- **List row padding:** `9px 16px` — compact for data-heavy interfaces
- **Panel gaps:** Use consistent CSS variable spacing
- **Borders:** 1px solid `var(--border)` between sections

---

## 4. Animations & Transitions

- **Hover/focus transitions:** `0.15s` for background color changes on buttons and list rows
- **New message flash:** Row flashes `rgba(108, 140, 255, 0.15)` and fades out over `0.6s`
- **Toast notifications:** Slide in, auto-dismiss after a short delay

---

## 5. Input Controls

- **No native browser checkboxes or radios** — use custom CSS toggle switches
- Toggle switches use `var(--bg-tertiary)` background and `var(--border)` for resting state
- Active toggles use `var(--accent)` color with `rgba(108, 140, 255, 0.15)` background glow
- Search inputs are debounced at `300ms`

---

## 6. Frontend Architecture Rules

- **No frameworks:** Vanilla JavaScript only. No React, Vue, Svelte.
- **No build toolchain:** No npm, webpack, vite. Write pure `app.js`, `style.css`, `index.html`.
- **Embedded distribution:** Static files are baked into the binary via `rust-embed`. Changes require recompilation.
- **DOM batching:** Buffer incoming messages and flush to the DOM every `250ms` to prevent freeze at high throughput.
- **Client-side filtering:** Search filters the in-memory `messages[]` array directly. Debounce search input at `300ms`.

---

## 7. Backend Coding Rules (Rust)

- **Async runtime:** Tokio with `tokio::select!` for concurrent task management.
- **Web framework:** Axum for HTTP API and WebSocket.
- **State management:** `Arc<RwLock<T>>` with `tokio::sync::broadcast` for real-time push to WebSocket clients.
- **Memory safeguards:** Explicit count limits and byte-size limits on stores. Evict oldest 10% when either limit is hit.
- **Logging:** Use the `tracing` crate. Never use `println!`.
- **File structure:** One responsibility per file (`web.rs` for routing, `store.rs` for state, `parser.rs` for decoding).

---

## 8. Commit Message Format

Conventional commit format with type prefix:

```
feat: add configurable MLLP port via environment variable
fix: handle missing MSH segment without panic
docs: update API examples
refactor: ...
test: ...
chore: ...
```

---

## 9. Branch Naming

```
feat/<name>
fix/<name>
docs/<name>
refactor/<name>
```

---

## 10. CI Checks (Run Before Every Push)

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

All three must pass. Zero tolerance for warnings.
