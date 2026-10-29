# 10 — Decision Log (ADRs)

> Format: lightweight ADR. Status: **Proposed** → **Accepted** → (**Superseded**).
> Add a new entry for each significant decision. Never rewrite history: supersede instead.

---

## ADR-001 Backend in Rust
- **Status:** ✅ Accepted (2026-10-29)
- **Context:** we need a fast, scalable, async-first backend (chat, presence, AI streaming, timers). The founder wants to learn Rust (C++ background).
- **Decision:** Rust with **Tokio** (async runtime), **Axum** (HTTP/WebSocket), **sqlx** (Postgres), **redis-rs** / **fred** (Redis), **serde**, **tracing**.
- **Consequences:** top performance and memory safety. A slower start (learning curve), mitigated by a slow, guided pace (the founder implements, the assistant gives hints) and by keeping the architecture simple (modular monolith).

## ADR-002 Mobile in Flutter
- **Status:** ✅ Accepted (2026-10-29)
- **Context:** one product for Android + iOS, a small team, real-time chat + video + maps + push.
- **Decision:** Flutter (Dart) for the patient app **and** the staff app (tablet/phone/web builds).
- **Alternatives:** React Native (TS), Kotlin Multiplatform, native ×2 (see PROJECT_IDEA §11).
- **Consequences:** one codebase, good LiveKit / Firebase / Maps support. Dart is a new (easy) language to learn.

## ADR-003 Modular monolith, two binaries
- **Status:** 🟡 Proposed
- **Decision:** one Cargo workspace; `api` and `worker` binaries share the `domain`, `app` and `infra` crates. Hexagonal architecture (traits as ports).
- **Consequences:** simple deploys and debugging. Modules can later be split into services along the existing boundaries.

## ADR-004 Free launch, business model decided after traction
- **Status:** ✅ Accepted (2026-10-29)
- **Decision:** the app is free for users. The pilot chain covers the running costs. Measure SOAR results for ~3–6 months, then choose among the PROJECT_IDEA §9 options.
- **Consequences:** keep infrastructure costs low (small instances, LiveKit Cloud free/low tier, a cheap LLM model where possible). Instrument analytics from day one to support the later decision.

## ADR-005 Rx reservations: register in the app, present at the counter
- **Status:** ✅ Accepted (2026-10-29), ⚠️ pending regulatory confirmation
- **Decision:** users may reserve Rx products by registering a prescription (photo + identifiers). Dispensing happens only after the pharmacist verifies the **original** prescription at the counter. No online payment.
- **Consequences:** prescription images are sensitive health data: encrypted storage, short retention, audited access.

## ADR-006 AI with human-in-the-loop for treatment recommendations
- **Status:** 🟡 Proposed (recommended), ⚠️ regulatory adviser to confirm
- **Decision:** the AI may inform, explain and draft OTC suggestions. Treatment recommendations are marked **pending pharmacist validation** unless the adviser confirms a lighter option is acceptable.
- **Consequences:** reduces MDR / AI Act exposure. Requires the pharmacist validation flow (doc 08 §7).

## ADR-007 Real-time via WebSocket + Redis pub/sub + transactional outbox
- **Status:** 🟡 Proposed
- **Decision:** one global WebSocket per app session. Events published via a Postgres outbox → Redis pub/sub → all API nodes. Push notifications as the fallback.
- **Consequences:** no lost events, horizontal scaling, and the "chat connects while browsing" UX.

## ADR-008 Video via LiveKit
- **Status:** 🟡 Proposed
- **Decision:** LiveKit (open-source SFU). Start on LiveKit Cloud, with the option to self-host.
- **Consequences:** no media through our servers. Flutter SDK available.

## ADR-009 Design-first process
- **Status:** ✅ Accepted (2026-10-29)
- **Decision:** requirements, UML (use case, class, sequence, state), architecture and API outline are reviewed before coding. All diagrams in Mermaid inside `docs/`.

## ADR-010 Minimal availability model, no on-call
- **Status:** ✅ Accepted (2026-10-29)
- **Context:** pilot of 2–3 pharmacies open 08:00–20:00. Free launch, zero extra staff cost wanted.
- **Decision:** human consults only during opening hours, answered by the team on duty between counter clients (Mode A). Outside opening hours: AI + callback ticket for 08:00 + nearby open pharmacies (Mode D). **No on-call, no dedicated slot, no shift planner** in MVP.
- **Consequences:** a simpler system (no scheduling module). Weaker coverage in the evening, when people often feel unwell. Measure callbacks created after 20:00 to decide on Modes B/C later (doc 03 §6).

## ADR-011 Pharmacy assistants answer consults; permissions are configurable
- **Status:** ✅ Accepted (2026-10-29), ⚠️ legal scope to confirm (Q10)
- **Decision:** pharmacists **and** pharmacy assistants receive and answer consults, mirroring the counter. Assistants can **transfer** a consult to a pharmacist. What each role may do is stored as **data** (`RolePermission`) per tenant, not hard-coded.
- **Consequences:** more people can answer, so faster responses. Requires a transfer flow (SD-2b) and a permission check in routing and in the staff UI.

## ADR-012 Stock via a connector abstraction, CSV fallback first
- **Status:** ✅ Accepted (2026-10-29)
- **Context:** it isn't yet known how the chain's pharmacy software exposes stock (Q1).
- **Decision:** a `StockSource` port with multiple adapters (vendor API, read-only DB, export file). MVP ships with **admin CSV upload + manual staff "out of stock" flags**. The real connector is added once Q1 is answered (doc 11).
- **Consequences:** development isn't blocked by the vendor. Stock freshness in the MVP depends on how often someone uploads, so the app shows `synced_at` and a "call to confirm" hint when the data is stale.
