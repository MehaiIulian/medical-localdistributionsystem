# 06 — Architecture

> Style: **modular monolith in Rust** (one deployable API server + one worker binary, sharing crates).
> Split into services only when a real need appears (scale, team, isolation).

---

## 1. C4 Level 1: System context

```mermaid
flowchart TB
    Patient["🧍 Patient<br/>(mobile app)"]
    Staff["🧑‍⚕️ Pharmacist / staff<br/>(staff app)"]
    Admin["🏢 Chain admin"]

    LDS["LDS Platform"]

    LLM["🤖 LLM provider<br/>(EU region)"]
    ERP["🗄️ Pharmacy ERP<br/>(family chain)"]
    Push["📲 FCM / APNs"]
    LK["🎥 LiveKit<br/>(video)"]
    SMS["✉️ SMS provider<br/>(OTP)"]
    Maps["🗺️ Google / Apple Maps<br/>(deep links)"]

    Patient --> LDS
    Staff --> LDS
    Admin --> LDS
    LDS --> LLM
    LDS --> ERP
    LDS --> Push
    LDS --> LK
    LDS --> SMS
    Patient -.-> Maps
```

## 2. C4 Level 2: Containers

```mermaid
flowchart TB
    subgraph Clients
        PA["📱 Patient app<br/>Flutter · Android + iOS"]
        SA["💻 Staff app<br/>Flutter · tablet / phone / web"]
    end

    subgraph Backend["LDS Backend (Rust)"]
        API["⚙️ API server<br/>Axum + Tokio<br/>REST + WebSocket"]
        WRK["⏱️ Worker<br/>Tokio<br/>timers, jobs, stock sync"]
    end

    subgraph Data
        PG[("🐘 PostgreSQL + PostGIS")]
        RD[("🟥 Redis<br/>presence · pub/sub · queues")]
        OBJ[("🪣 Object storage<br/>S3-compatible, EU")]
    end

    subgraph External
        LLM["🤖 LLM API"]
        LK["🎥 LiveKit"]
        PUSH["📲 FCM / APNs"]
        SMS["✉️ SMS OTP"]
        ERP["🗄️ Chain ERP"]
    end

    PA -- "HTTPS REST" --> API
    PA -- "WSS events" --> API
    SA -- "HTTPS REST" --> API
    SA -- "WSS events" --> API
    PA -. "WebRTC" .-> LK
    SA -. "WebRTC" .-> LK

    API --> PG
    API --> RD
    API --> OBJ
    API --> LLM
    API --> LK
    API --> SMS
    WRK --> PG
    WRK --> RD
    WRK --> PUSH
    WRK --> ERP
    API -- "enqueue jobs" --> RD
```

Stock: the worker reads from whatever source the chain's software allows (API, DB, export file), or from an admin CSV upload. See [doc 11](11-stock-integration.md).

**Why two binaries?** The API server handles requests and WebSockets. The worker runs background work (timers, sync, push sending) so slow jobs never block user requests. Both are stateless and scale horizontally.

## 3. C4 Level 3: Components (modules in the API server)

```mermaid
flowchart LR
    subgraph API["API server (Axum)"]
        direction TB
        GW["HTTP router + middleware<br/>auth · tenant · rate limit · tracing"]
        WS["WebSocket hub<br/>connections · subscriptions"]

        ID["identity<br/>OTP login, JWT, consents"]
        ORG["organization<br/>tenants, pharmacies, staff"]
        CAT["catalog<br/>products, stock search"]
        CARE["care<br/>cases, messages, summaries"]
        SAFE["safety<br/>red-flag engine"]
        AIO["ai<br/>prompt building, LLM client, streaming"]
        RT["routing<br/>presence, offers, escalation"]
        SCH["availability<br/>opening hours, role permissions"]
        RES["reservations<br/>OTC / Rx, pickup codes"]
        NOTIF["notifications<br/>push / in-app"]
        AUD["audit"]
    end

    GW --> ID
    GW --> ORG
    GW --> CAT
    GW --> CARE
    GW --> RES
    GW --> SCH
    WS --> CARE
    WS --> RT
    CARE --> SAFE
    CARE --> AIO
    CARE --> RT
    RT --> SCH
    RT --> NOTIF
    RES --> CAT
    RES --> NOTIF
    CARE --> AUD
    RES --> AUD
```

---

## 4. The async model (the important part)

Requirement: *"chat connecting while the user is navigating the app"* plus many concurrent things (AI streaming, offers, timers, push).

### 4.1 Principles
1. **One WebSocket per logged-in app**, opened at app start and **kept independent of screens**. Any screen can react to events (banner "Pharmacist joined").
2. **Events, not polling.** The server pushes typed events (`consult.accepted`, `message.new`, `reservation.ready`…). See doc 09.
3. **Push notifications as a fallback** when the app is in the background or the socket is down.
4. **Every async process is a state machine persisted in Postgres** (doc 08), so a crash or restart never loses a consult or a reservation.
5. **Timers live in the worker** (offer reminders, max-wait, reservation expiry, opening 08:00 / closing 20:00), backed by Redis / Postgres, never in a single process's memory only.
6. **Multi-instance ready:** when there are 2+ API nodes, a message for user X may arrive at node 1 while X is connected to node 2 → **Redis pub/sub** fans events out to all nodes.

### 4.2 Event flow

```mermaid
flowchart LR
    A["Domain action<br/>(e.g., staff accepts consult)"] --> DB[("Postgres<br/>state change + outbox row")]
    DB --> OB["Outbox relay"]
    OB --> PS[("Redis pub/sub<br/>channel user:ID")]
    PS --> N1["API node 1<br/>WS hub"]
    PS --> N2["API node 2<br/>WS hub"]
    N2 --> C["📱 Patient app<br/>(connected to node 2)"]
    OB --> Q[("Redis queue<br/>push jobs")]
    Q --> W["Worker"] --> P["FCM / APNs"]
```

The **transactional outbox** (state change and event row written in the same DB transaction) guarantees that no event is lost and none is emitted for a rolled-back change.

### 4.3 Inside one API node (Tokio)

```mermaid
flowchart TB
    subgraph Node["API node (Tokio runtime)"]
        L["Axum listener"] --> H["Request handlers<br/>(one async task each)"]
        L --> WST["WebSocket tasks<br/>(1 per connection)<br/>reader + writer"]
        HUB["Hub: map user_id → sender channels"]
        SUB["Redis subscriber task"]
        SUB --> HUB
        HUB --> WST
        H --> AI["LLM streaming task"]
        AI --> HUB
    end
```

> 🦀 **Rust learning hints (no code yet):**
> - `tokio::spawn`: lightweight tasks (like goroutines). Thousands are cheap.
> - `tokio::sync::mpsc`: one channel per WebSocket connection (hub → writer task).
> - `tokio::sync::broadcast`: one-to-many in-process fan-out.
> - `Arc<...>` + `DashMap` or `RwLock<HashMap>` for the hub's shared map. Compare `Arc` with C++ `std::shared_ptr`.
> - `tokio::select!` waits on "new WS message OR new hub event OR shutdown" at the same time.
> - `axum::extract::ws` for WebSockets, `axum::extract::State` to share app state.
> - LLM streaming: `reqwest` + a byte stream (`futures::StreamExt`) → forward chunks to the hub.

---

## 5. Proposed Rust workspace layout

```
lds/
├── Cargo.toml                # [workspace]
├── crates/
│   ├── domain/               # pure types + business rules: entities, enums, state machines (no I/O)
│   ├── app/                  # use cases / services, orchestrating domain + ports (traits)
│   ├── infra/                # adapters: Postgres (sqlx), Redis, LLM client, LiveKit, FCM, S3, ERP
│   ├── api/                  # binary: Axum router, handlers, WebSocket hub, auth middleware
│   ├── worker/               # binary: timers, outbox relay, push sender, stock sync
│   └── safety/               # red-flag engine (rules + test suite), isolated and heavily tested
├── migrations/               # SQL migrations (sqlx migrate)
└── tests/                    # integration tests (testcontainers: Postgres, Redis)
```

**Hexagonal / ports & adapters:** `domain` and `app` define **traits** (e.g., `StockRepository`, `LlmClient`, `PushSender`). `infra` implements them. This keeps the business logic testable without a database, and it's a great way to learn Rust traits and generics (≈ C++ abstract classes / concepts).

## 6. Mobile app architecture (Flutter, proposed)

| Layer | Choice (proposal) |
|---|---|
| State management | Riverpod |
| Navigation | go_router |
| HTTP | dio |
| WebSocket | `web_socket_channel` + a global **RealtimeService** (singleton, auto-reconnect, exponential backoff) |
| Push | firebase_messaging (FCM + APNs) |
| Video | livekit_client |
| Maps | google_maps_flutter / apple maps via url_launcher deep links |
| Local storage | drift (SQLite) for offline cache + outbox of unsent messages |
| Models | Generated from the API's OpenAPI spec (shared contract) |

The **global RealtimeService** is what makes "chat connects while I browse" work: it lives above all screens, receives events and updates the app state. Any screen shows a banner or badge.

## 7. Deployment (pilot)

| Component | Pilot proposal |
|---|---|
| Hosting | EU cloud (e.g., Hetzner / OVH / AWS eu-central-1), one region |
| Runtime | Docker containers: `api` ×2, `worker` ×1 |
| DB | Managed PostgreSQL 16 + PostGIS, daily backups, PITR |
| Redis | Managed Redis 7 |
| Video | LiveKit Cloud (start) → self-host later if cost requires |
| CI/CD | GitHub Actions: fmt, clippy, tests, build images, deploy |
| Observability | OpenTelemetry → Grafana / Loki / Tempo (or a managed equivalent) |
| Secrets | Cloud secret manager. Never in the repo |

## 8. Security & privacy architecture

- **Auth:** phone OTP → short-lived access JWT (15 min) + rotating refresh token. Staff: same, plus a role and tenant claims. 2FA for admins.
- **Authorization:** RBAC + tenant isolation enforced in the `app` layer (and optionally Postgres row-level security).
- **Health data access:** staff can read a Case **only** if they have an active/assigned Consult or callback for it. Every read is audited.
- **AI provider:** EU data residency, zero data retention agreement, pseudonymized context (no phone or name sent to the LLM).
- **Encryption:** TLS 1.2+, encryption at rest (DB, object storage). Prescription images get short-lived signed URLs.
- **Red-flag engine runs locally** (no external dependency), so it works even when the LLM is down.
