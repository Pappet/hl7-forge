# 🚀 The HL7 Forge Style Guide

## 1. Visual & UI Design (The "Forge" Aesthetic)
HL7 Forge uses a beautiful, zero-dependency dark theme built on raw css variables with subtle accents and smooth micro-animations.

**Color Palette (Dark Theme First):**
*   **Backgrounds:** 
    *   Primary: `#0f1117` (Deep dark blue/black)
    *   Secondary: `#1a1d27` (Slightly lighter panels, headers)
    *   Tertiary: `#242736` (Hover states, active elements, popups)
    *   Accent Hover: `#2a2e3f`
*   **Borders & Lines:** `#2e3247`
*   **Text & Typography:**
    *   Primary Text: `#e4e6f0`
    *   Secondary Text: `#8b8fa3`
    *   Muted/Hints: `#5c6078`
*   **Accents (Status & Highlights):**
    *   Primary Accent (Blue): `#6c8cff` (Dimmed: `#4a62b3`)
    *   Success (Green): `#4caf84`
    *   Warning (Yellow): `#e5a54b`
    *   Error (Red): `#e05454`

**Design Principles:**
*   **Typography:** System fonts for UI elements (`-apple-system, system-ui, sans-serif`) to feel native, and strict monospace for data/code elements (`SF Mono, Cascadia Code, Consolas, monospace`).
*   **Micro-Animations:** Keep interactions smooth but extremely fast. Use `0.15s` transitions for background color changes on buttons and list rows.
*   **Feedback:** Flash animations for new incoming data (e.g., a row flashing `rgba(108, 140, 255, 0.15)` and fading out over `0.6s`).
*   **Density:** Data-heavy interfaces should be compact. Use `12px` to `13px` base font sizes with small paddings (e.g., `9px 16px` for list rows) and subtle uppercase headers for grids.

## 2. Frontend Architecture (The "No-Build Vanilla" Rule)
The hallmark of HL7 Forge's frontend is its extremely fast, zero-dependency nature.
*   **No Frameworks:** Avoid heavy frameworks like React, Vue, or Svelte. Use intentional **Vanilla JavaScript**.
*   **No Build Toolchain:** No `npm`, no `webpack`, no `vite`. Write pure `app.js`, `style.css`, and `index.html`. 
*   **Embedded Distribution:** Bake the `static` directory directly into your backend binary (e.g., via `rust-embed` in Rust) so the application ships as a single, self-contained executable.
*   **Performance via Batching:** For high-throughput apps, don't update the DOM on every event. Buffer incoming messages and flush them to the DOM in batches (e.g., every 250ms).
*   **Client-Side Filtering:** Prioritize instant client-side searching for small-to-medium datasets to avoid network overhead. Debounce search inputs by `300ms`.

## 3. Backend & Coding Guidelines (Rust)
The backend conventions are strictly defined to guarantee reliability and performance.
*   **Architecture Stack:** Use Tokio for async concurrency and `tokio::select!` for managing concurrent tasks. Use Axum for HTTP API and WebSockets.
*   **State Management:** Flow shared state evenly using tools like `Arc<RwLock<T>>` combined with broadcast channels (`tokio::sync::broadcast`) to push real-time updates to connected WebSocket clients.
*   **Memory Safeguards:** Always implement explicit count limits and byte-size limits on in-memory stores. Apply eviction rules (e.g., evict the oldest 10% when hitting the limit) to prevent Out-Of-Memory (OOM) crashes.
*   **Logging:** Never use `println!`. Standardize on the `tracing` ecosystem for all application logging.
*   **Modularity:** Keep files focused (e.g., `web.rs` for routing, `store.rs` for state, `parser.rs` for decoding logic). Add doc comments (`///`) to all public types and functions.

## 4. Source Control & Workflow
HL7 Forge maintains an iron-clad CI/CD pipeline and PR workflow.

**Commit Messages (Conventional Format):**
Start commits with a type and short imperative summary:
*   `feat: add configurable MLLP port via environment variable`
*   `fix: handle missing MSH segment without panic`
*   `docs: update API examples`
*   `refactor: ...`, `test: ...`, `chore: ...`

**Changelog Enforcement:**
*   Always update `CHANGELOG.md` **in the same commit** as the code change.
*   Follow the [Keep a Changelog](https://keepachangelog.com/) format.
*   Row format pattern: `| [\`<short-hash>\`](<hash-link>) | \`<type>:\` Short description |`

**Branch Naming Strategy:**
*   `feature/<name>`
*   `fix/<name>`
*   `docs/<name>`
*   `refactor/<name>`

**CI & Guardrails (Run Locally Before Push):**
*   **Formatting Check:** `cargo fmt --check`
*   **Strict Linting:** `cargo clippy -- -D warnings` (zero tolerance for warnings)
*   **Testing:** `cargo test`

---
*Tip: When kicking off a new project based on these principles, use this style guide as a reference for the visual language and architecture.*
