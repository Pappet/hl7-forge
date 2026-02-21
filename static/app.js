// --- State ---
let messages = [];
let selectedId = null;
let selectedMessage = null;
let activeTab = 'parsed';
let autoscroll = true;
let searchQuery = '';
let ws = null;
let collapsedSegments = new Set();

// Task 2: batching state
let paused = false;
let pendingMessages = [];
let renderScheduled = false;

// --- WebSocket ---
function connectWs() {
    const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
    ws = new WebSocket(`${proto}//${location.host}/ws`);

    ws.onopen = () => {
        document.getElementById('ws-dot').className = 'stat-dot green';
        document.getElementById('ws-status').textContent = 'Connected';
    };

    ws.onclose = () => {
        document.getElementById('ws-dot').className = 'stat-dot red';
        document.getElementById('ws-status').textContent = 'Disconnected';
        setTimeout(connectWs, 2000);
    };

    ws.onerror = (event) => {
        console.error('WebSocket error:', event);
        document.getElementById('ws-dot').className = 'stat-dot red';
        document.getElementById('ws-status').textContent = 'Error';
    };

    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        if (data.type === 'init') {
            document.getElementById('stat-total').textContent = data.total;
            loadMessages();
        } else if (data.type === 'new_message') {
            addMessage(data.data);
        } else if (data.type === 'lagged') {
            console.warn(`Missed ${data.missed} messages, reloading...`);
            loadMessages();
        } else if (data.type === 'cleared') {
            console.info("Server cleared messages via Web UI or API");
            messages = [];
            pendingMessages = [];
            selectedId = null;
            selectedMessage = null;
            renderMessageList();
            document.getElementById('detail-content').innerHTML = '<div class="empty-state"><p>No message selected</p></div>';
            document.getElementById('detail-title').textContent = 'Select a message';
            document.getElementById('detail-meta').textContent = '';
            document.getElementById('stat-total').textContent = '0';
        }
    };
}

// Task 2: buffer incoming messages, flush at most every 250 ms
function addMessage(summary) {
    pendingMessages.unshift(summary);
    document.getElementById('stat-total').textContent =
        messages.length + pendingMessages.length;
    if (!paused) {
        scheduleRender();
    }
}

function scheduleRender() {
    if (renderScheduled) return;
    renderScheduled = true;
    setTimeout(() => {
        renderScheduled = false;
        flushAndRender();
    }, 250);
}

function flushAndRender() {
    if (pendingMessages.length > 0) {
        messages = [...pendingMessages, ...messages];
        pendingMessages = [];
        document.getElementById('stat-total').textContent = messages.length;
    }
    renderMessageList();
}

async function loadMessages() {
    try {
        const resp = await fetch('/api/messages?limit=1000');
        if (!resp.ok) return;
        messages = await resp.json();
        pendingMessages = [];
        document.getElementById('stat-total').textContent = messages.length;
        renderMessageList();
    } catch (e) {
        console.error('Failed to load messages:', e);
    }
}

// --- Stats polling ---
async function pollStats() {
    try {
        const resp = await fetch('/api/stats');
        if (!resp.ok) return;
        const stats = await resp.json();
        document.getElementById('stat-total').textContent = stats.total_messages;
        document.getElementById('stat-connections').textContent = stats.active_connections;
        document.getElementById('stat-errors').textContent = stats.parse_errors;
        if (stats.mllp_port) {
            document.getElementById('mllp-port').textContent = stats.mllp_port;
        }
    } catch (e) { }
}

// --- Rendering ---
function renderMessageList() {
    const list = document.getElementById('message-list');
    const empty = document.getElementById('empty-state');
    const filtered = searchQuery
        ? messages.filter(m => matchesSearch(m, searchQuery))
        : messages;

    if (filtered.length === 0) {
        empty.style.display = 'flex';
        list.querySelectorAll('.message-row').forEach(r => r.remove());
        return;
    }

    empty.style.display = 'none';

    const fragment = document.createDocumentFragment();
    for (const msg of filtered) {
        const row = document.createElement('div');
        row.className = 'message-row' + (msg.id === selectedId ? ' selected' : '');
        row.dataset.id = msg.id;
        row.onclick = () => selectMessage(msg.id);

        const time = new Date(msg.received_at);
        const timeStr = time.toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit', second: '2-digit' });

        // Task 1: red marker for messages that failed to parse
        const typeHtml = msg.parse_error
            ? `<span class="msg-type" style="color:var(--error)" title="${esc(msg.parse_error)}">⚠ PARSE ERROR</span>`
            : `<span class="msg-type">${esc(msg.message_type)}</span>`;

        row.innerHTML = `
            ${typeHtml}
            <span class="msg-source">${esc(msg.sending_facility)}</span>
            <span class="msg-patient">${esc(msg.patient_name || msg.patient_id || '—')}</span>
            <span class="msg-time">${timeStr}</span>
            <span class="msg-segs">${msg.segment_count}</span>
        `;
        fragment.appendChild(row);
    }

    list.querySelectorAll('.message-row').forEach(r => r.remove());
    list.appendChild(fragment);

    if (autoscroll) {
        list.scrollTop = 0;
    }
}

function matchesSearch(msg, query) {
    const q = query.toLowerCase();
    return (
        (msg.message_type || '').toLowerCase().includes(q) ||
        (msg.sending_facility || '').toLowerCase().includes(q) ||
        (msg.patient_name || '').toLowerCase().includes(q) ||
        (msg.patient_id || '').toLowerCase().includes(q) ||
        (msg.message_control_id || '').toLowerCase().includes(q) ||
        (msg.source_addr || '').toLowerCase().includes(q)
    );
}

async function selectMessage(id) {
    selectedId = id;
    renderMessageList();

    try {
        const resp = await fetch(`/api/messages/${id}`);
        if (!resp.ok) return;
        selectedMessage = await resp.json();
        renderDetail();
    } catch (e) {
        console.error('Failed to load message:', e);
    }
}

function renderDetail() {
    if (!selectedMessage) return;
    const msg = selectedMessage;

    document.getElementById('detail-title').textContent =
        `${msg.message_type} — ${msg.patient_name || msg.patient_id || 'Unknown'}`;
    document.getElementById('detail-meta').textContent =
        `${msg.source_addr} | ${msg.message_control_id} | v${msg.version}`;

    renderTab();
}

function switchTab(tab) {
    activeTab = tab;
    document.querySelectorAll('.detail-tab').forEach(t => {
        t.classList.toggle('active', t.dataset.tab === tab);
    });
    renderTab();
}

function renderTab() {
    const content = document.getElementById('detail-content');
    if (!selectedMessage) return;
    const msg = selectedMessage;

    if (activeTab === 'parsed') {
        // Task 1: show parse error banner instead of empty segment table
        if (msg.parse_error) {
            content.innerHTML = `<div style="color:var(--error);font-family:var(--font-mono);padding:16px;line-height:1.6;">
                ⚠ Parse Error<br><br>
                <span style="color:var(--text-secondary)">${esc(msg.parse_error)}</span><br><br>
                <span style="color:var(--text-muted)">Raw message is available in the Raw tab.</span>
            </div>`;
            return;
        }
        content.innerHTML = msg.segments.map((seg, segIdx) => {
            const key = `${msg.message_control_id}-${segIdx}`;
            const collapsed = collapsedSegments.has(key);
            const icon = collapsed ? '▸' : '▾';
            return `
            <div class="segment-block">
                <div class="segment-name" onclick="toggleSegment('${key}')">
                    <span class="collapse-icon">${icon}</span>
                    ${esc(seg.name)}
                    <span class="field-count">(${seg.fields.length})</span>
                </div>
                ${collapsed ? '' : `<table class="field-table">
                    <thead><tr><th style="width:70px">Field</th><th>Value</th><th>Components</th></tr></thead>
                    <tbody>
                    ${seg.fields.map(f => `
                        <tr>
                            <td class="field-idx">${seg.name}-${f.index}</td>
                            <td class="field-val">${esc(f.value) || '<span class="field-empty">empty</span>'}</td>
                            <td class="field-components">${f.components.length > 1
                    ? f.components.map((c, i) => `<span title="${seg.name}-${f.index}.${i + 1}">${esc(c)}</span>`).join(' <span style="color:var(--text-muted)">^</span> ')
                    : ''
                }</td>
                        </tr>
                    `).join('')}
                    </tbody>
                </table>`}
            </div>`;
        }).join('');
    } else if (activeTab === 'raw') {
        const lines = msg.raw.split(/\r?\n|\r/).filter(l => l.trim());
        content.innerHTML = `<div class="raw-view">${lines.map(line => {
            const segName = line.substring(0, 3);
            return `<div class="segment-line"><span style="color:var(--accent);font-weight:600">${esc(segName)}</span>${esc(line.substring(3))}</div>`;
        }).join('')
            }</div>`;
    } else if (activeTab === 'json') {
        content.innerHTML = `<pre class="raw-view">${esc(JSON.stringify(msg, null, 2))}</pre>`;
    }
}

function toggleSegment(key) {
    if (collapsedSegments.has(key)) {
        collapsedSegments.delete(key);
    } else {
        collapsedSegments.add(key);
    }
    renderTab();
}

// --- Actions ---
function toggleAutoscroll() {
    autoscroll = !autoscroll;
    const btn = document.getElementById('btn-autoscroll');
    btn.style.borderColor = autoscroll ? 'var(--success)' : 'var(--border)';
    btn.style.color = autoscroll ? 'var(--success)' : 'var(--text-primary)';
}

// Task 2: pause/resume live updates
function togglePause() {
    paused = !paused;
    const btn = document.getElementById('btn-pause');
    if (paused) {
        btn.textContent = '▶ Live';
        btn.style.borderColor = 'var(--warning)';
        btn.style.color = 'var(--warning)';
    } else {
        btn.textContent = '⏸ Pause';
        btn.style.borderColor = '';
        btn.style.color = '';
        flushAndRender();
    }
}

async function exportMessages() {
    try {
        const resp = await fetch('/api/messages?limit=100000');
        if (!resp.ok) throw new Error(`Server error: ${resp.status}`);
        const data = await resp.json();
        const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `hl7-forge-export-${new Date().toISOString().slice(0, 19)}.json`;
        a.click();
        URL.revokeObjectURL(url);
    } catch (e) {
        showToast('Export failed: ' + (e.message || e));
    }
}

async function clearMessages() {
    if (!confirm('Delete all messages?')) return;
    try {
        const resp = await fetch('/api/clear', { method: 'POST' });
        if (!resp.ok) throw new Error(`Server error: ${resp.status}`);
        messages = [];
        pendingMessages = [];
        selectedId = null;
        selectedMessage = null;
        renderMessageList();
        document.getElementById('detail-content').innerHTML = '<div class="empty-state"><p>No message selected</p></div>';
        document.getElementById('detail-title').textContent = 'Select a message';
        document.getElementById('detail-meta').textContent = '';
    } catch (e) {
        console.error('Failed to clear messages:', e);
    }
}

// --- Utility ---
function showToast(message, type = 'error') {
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;
    document.body.appendChild(toast);
    setTimeout(() => toast.remove(), 4000);
}

function esc(str) {
    if (!str) return '';
    const div = document.createElement('div');
    div.textContent = str;
    return div.innerHTML;
}

// --- Init ---
// Search is purely client-side (filters the local `messages` array via matchesSearch).
// The debounce is a forward-looking safeguard: if a future /api/search call is added,
// rapid keystrokes would otherwise hammer the server and cause RwLock contention on the
// Rust side. Local renderMessageList() remains immediately reactive inside the handler.
let _searchDebounceTimer = null;
document.getElementById('search-input').addEventListener('input', (e) => {
    clearTimeout(_searchDebounceTimer);
    _searchDebounceTimer = setTimeout(() => {
        searchQuery = e.target.value;
        renderMessageList();
    }, 300);
});

toggleAutoscroll(); // set initial visual state
connectWs();
setInterval(pollStats, 3000);
