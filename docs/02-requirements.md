# 02 — Requirements & MVP Scope

> MoSCoW: **M**ust · **S**hould · **C**ould · **W**on't (for MVP).
> IDs are stable so diagrams, tickets and tests can reference them.

---

## 1. MVP goal

Release a **free** app to the public in the area of the **2–3 family pharmacies**. Users can:
1. talk to an AI health assistant,
2. hand over to a real pharmacist who already has the summary,
3. see stock in the pilot pharmacies and **reserve** OTC and Rx medicines for pickup.

Then **measure** whether it's worth continuing (see the SOAR results in PROJECT_IDEA §10).

---

## 2. Functional requirements: Patient app

### 2.1 Account & profile
| ID | User story | Prio |
|---|---|---|
| P-ACC-1 | As a user, I can sign up / log in with my **phone number + SMS code** (Q9) | M |
| P-ACC-2 | As a user, I give explicit **consent** for health-data processing (GDPR art. 9) before using the AI | M |
| P-ACC-3 | As a user, I can fill a **health profile**: birth year, sex, allergies, chronic conditions, current meds, pregnancy/breastfeeding | M |
| P-ACC-4 | As a user, I can add **family profiles** (child, parent) and choose who a case is for | S |
| P-ACC-5 | As a user, I can **delete my account and data** | M |
| P-ACC-6 | As a user, I can export my data | C |

### 2.2 AI Health Assistant
| ID | User story | Prio |
|---|---|---|
| P-AI-1 | As a user, I can start a **case** and describe my symptoms in a chat; the AI asks structured follow-up questions | M |
| P-AI-2 | As a user, I can ask **follow-up questions about possible causes / diagnostics** and get plain-language explanations | M |
| P-AI-3 | As a user, I get **OTC suggestions + self-care advice**, checked against my profile (allergies, interactions, age, pregnancy). Final wording depends on the regulatory option (Q4) | M |
| P-AI-4 | As a user, if I describe a **red flag**, I immediately see a 112 / UPU screen with a call button | M |
| P-AI-5 | As a user, I can **photograph / scan** medicines I have at home so the AI knows them | S |
| P-AI-6 | As a user, I see AI answers **streamed** (word by word) | S |
| P-AI-7 | As a user, I can see my **case history** | S |
| P-AI-8 | As a user, I can rate the AI answer (thumbs up / down) | S |

### 2.3 Human handoff
| ID | User story | Prio |
|---|---|---|
| P-HO-1 | As a user, I can tap **"Talk to a pharmacist"** at any moment in a case | M |
| P-HO-2 | As a user, I can **keep using the app** (stock, reservations) while waiting; a banner shows the queue status | M |
| P-HO-3 | As a user, I get an **in-app banner or push** when the pharmacist joins | M |
| P-HO-4 | As a user, I see the queue position / estimated wait, or "the pharmacy team is available daily 08:00–20:00, next: tomorrow 08:00" | M |
| P-HO-5 | As a user, I can leave a **callback request** when nobody is available (overnight, or after waiting too long) | M |
| P-HO-6 | As a user, I can **chat** with the pharmacist (text + photos) | M |
| P-HO-7 | As a user, I can accept a pharmacist's **video call** invitation | S |
| P-HO-8 | As a user, I can cancel the request | M |
| P-HO-9 | As a user, I can rate the consult | S |

### 2.4 Stock & pharmacies
| ID | User story | Prio |
|---|---|---|
| P-ST-1 | As a user, I can **search a product** (name, active substance) and see in which pilot pharmacies it's available (available / low / out) | M |
| P-ST-2 | As a user, I can see the pharmacies on a **map/list** with opening hours, open now, distance, and a route button (Google/Apple Maps) | M |
| P-ST-3 | As a user, I can see product info (form, strength, OTC/Rx, leaflet link) | S |
| P-ST-4 | As a user, I see alternative products with the same active substance (generic) if out of stock | C |

### 2.5 Reservations
| ID | User story | Prio |
|---|---|---|
| P-RS-1 | As a user, I can **reserve OTC products** at a pharmacy for pickup; I pay at the counter | M |
| P-RS-2 | As a user, I can **reserve Rx products** by first **registering a prescription** (photo + series/number, or e-prescription code); I present the original at the counter | M |
| P-RS-3 | As a user, I get a **pickup code / QR** and the hold expiry time | M |
| P-RS-4 | As a user, I get notifications: confirmed, ready, about to expire, expired, rejected | M |
| P-RS-5 | As a user, I can cancel a reservation | M |
| P-RS-6 | As a user, I can reserve directly from the AI's or pharmacist's recommendation | S |
| P-RS-7 | As a user, I see my reservation history | S |

---

## 3. Functional requirements: Staff app (pharmacy team: pharmacists + assistants)

> Pharmacy assistants answer in the app as they would at the counter. Exact permissions are **configurable per role** (doc 03 §3, Q10).

| ID | User story | Prio |
|---|---|---|
| S-1 | As staff, I log in with my staff account (chain-issued). Role: **Pharmacist** (verified **CFR license** number) or **Pharmacy assistant** | M |
| S-2 | As staff, I set my **presence**: Available / Busy / Break / Offline. Available only during opening hours (08:00–20:00), auto-Offline at closing | M |
| S-3 | As staff, I receive **consult offers** (in-app sound + push) with a preview of the summary, and I accept or decline | M |
| S-4 | As staff, I see the **AI summary, full transcript, profile and meds at hand** before replying | M |
| S-5 | As staff, I chat with the patient and can **start a video call** | M (chat) / S (video) |
| S-6 | As staff, I **validate / edit / override** the AI recommendation and send a final recommendation | M |
| S-7 | As staff, I can propose a **reservation** to the patient (prefilled products) | S |
| S-8 | As staff, I close the consult with an outcome (resolved, referred to doctor, referred to UPU, no response) | M |
| S-9 | As staff, I see the **callback tickets** queue (overnight tickets at the top at 08:00) | M |
| S-10 | ~~As staff, I opt into on-call windows (Mode C)~~ → *Later* (no on-call in MVP) | W |
| S-11 | As pharmacy staff, I see **incoming reservations**, confirm / reject, mark *ready* and *picked up* (by scanning the QR) | M |
| S-12 | As pharmacy staff, at pickup of an Rx reservation I mark **prescription verified** or **refused** | M |
| S-13 | As a pharmacy assistant, I can **transfer a consult to a pharmacist** (Rx questions, when unsure); the pharmacist gets the same summary and chat | M |
| S-14 | As staff, I can set a product to **"out of stock"** quickly (manual stock flag) | S |

## 4. Functional requirements: Chain admin

| ID | User story | Prio |
|---|---|---|
| A-1 | Manage pharmacies (address, geo, **opening hours**, contact) | M |
| A-2 | Manage staff accounts and roles (Pharmacist / Pharmacy assistant / Chain admin) | M |
| A-3 | ~~Shift planner~~ → *Later*. MVP availability = opening hours + presence toggle | W |
| A-4 | **Stock**: manual **CSV upload** (MVP) + connector to the chain's software once known (doc 11) | M |
| A-5 | Reservation settings: hold times, limits | S |
| A-6 | Reports: consults, response / waiting times, reservations, conversion, workload per staff, callbacks created after hours | S |
| A-7 | Coverage view (hours without a human) | C |
| A-8 | Role permissions configuration (what assistants may do, doc 03 §3) | M |
| A-9 | Unmatched stock product codes report + manual mapping | S |

## 5. Platform admin / system

| ID | Requirement | Prio |
|---|---|---|
| SYS-1 | **Red-flag engine** (rule-based) evaluates every user message before the AI | M |
| SYS-2 | **AI guardrails**: no Rx prescribing, escalation on low confidence, mandatory disclaimers | M |
| SYS-3 | **Audit log** of every access to health data by staff | M |
| SYS-4 | **Multi-tenant** data model (chain = tenant) from day one | M |
| SYS-5 | Reservation **auto-expiry** job | M |
| SYS-6 | Consult **offer reminders**, max-wait suggestion, conversion of waiting requests to callbacks at closing time (20:00) | M |
| SYS-7 | Product catalog import (ANMDMR nomenclature) | M |
| SYS-8 | AI conversation review tool (sample conversations for safety QA) | S |

---

## 6. Won't have in MVP

- **On-call / free-day availability (Mode C)**, **dedicated app slot (Mode B)**, **shift planner** (doc 03 §6).
- Human consults outside **08:00–20:00** (AI + callback only).
- Online payments, delivery, insurance.
- Doctor consults / e-prescriptions issued by the platform.
- Integration with SIUI / CNAS e-prescription systems (manual registration only).
- Other chains / independent pharmacies (the data model supports it, the onboarding doesn't yet).
- Veterinary.
- Queue-aware routing (`travel time + queue time`).
- Web app for patients.

---

## 7. Non-functional requirements

| ID | Category | Requirement |
|---|---|---|
| NFR-1 | Latency | API p95 < 200 ms (excluding AI). First AI token < 2 s. Chat message delivery < 500 ms |
| NFR-2 | Async UX | Handoff, chat and notifications work while the user navigates. Reconnects automatically after network loss |
| NFR-3 | Scalability | Stateless API nodes behind a load balancer. Real-time fan-out via Redis pub/sub. Target 10k concurrent WebSockets per node |
| NFR-4 | Availability | 99.5% for pilot. Graceful degradation: if the AI is down, go straight to human / Mode D |
| NFR-5 | Security | TLS everywhere, encryption at rest, JWT with short expiry + refresh, RBAC per tenant, OWASP ASVS L2 |
| NFR-6 | Privacy | GDPR: consent, minimization, EU hosting, DPIA, retention policy (e.g., cases kept N years, configurable), right to erasure |
| NFR-7 | Auditability | All staff reads/writes of health data are logged (who, what, when) |
| NFR-8 | Safety | Red-flag recall target 100% on the test suite. AI outputs logged for review |
| NFR-9 | i18n | Romanian for MVP. All strings externalized (Q8) |
| NFR-10 | Accessibility | Large-font support, screen readers, high contrast |
| NFR-11 | Observability | Structured logs, metrics, tracing (OpenTelemetry) |
| NFR-12 | Mobile | Android 8+ / iOS 15+. Offline: show the last data, queue outgoing messages |
