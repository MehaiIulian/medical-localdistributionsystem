# 07 — Sequence Diagrams

> Participants: **App** (Flutter patient app), **Staff** (staff app), **API** (Axum server), **WS** (WebSocket hub), **Safety** (red-flag engine), **LLM**, **DB** (Postgres), **Redis**, **Worker**, **Push** (FCM/APNs), **LK** (LiveKit), **ERP**.

---

## SD-1 AI consultation with red-flag check (UC-03, UC-04)

```mermaid
sequenceDiagram
    autonumber
    actor P as Patient
    participant App
    participant API
    participant Safety
    participant LLM
    participant DB

    P->>App: Types symptoms
    App->>API: POST /cases/{id}/messages
    API->>DB: Insert message (sender=Patient)
    API->>Safety: evaluate(message, profile)
    alt Red flag detected
        Safety-->>API: RedFlag(kind=chest_pain)
        API->>DB: case.red_flag_shown = true, status = EmergencyShown
        API-->>App: 200 + event emergency.show
        App-->>P: Full-screen 112 / UPU with call button
    else No red flag
        Safety-->>API: Ok
        API-->>App: 202 Accepted
        API->>LLM: Stream completion (pseudonymized context + history)
        loop For each chunk
            LLM-->>API: token chunk
            API-->>App: WS ai.chunk
        end
        API->>DB: Insert AI message
        API->>Safety: evaluate(AI output) as guardrail
        API->>DB: Insert CaseSummary v(n+1)
        API-->>App: WS ai.done + summary.updated
    end
```

## SD-2 Async handoff to a pharmacist while the user browses (UC-05)

```mermaid
sequenceDiagram
    autonumber
    actor P as Patient
    participant App
    participant API
    participant DB
    participant Redis
    participant Worker
    participant Push
    participant Staff
    actor Ph as Pharmacist

    P->>App: Tap "Talk to the pharmacy"
    App->>API: POST /cases/{id}/consults
    API->>DB: Insert Consult(Requested) + outbox
    API->>API: Check opening hours 08:00-20:00
    API->>Redis: Find Available team members chain-wide (pharmacists + assistants)
    API->>DB: Insert ConsultOffers (broadcast) + Consult = Offering
    API-->>App: 201 {consult_id, queue_pos, eta}
    App-->>P: Banner "Searching for the pharmacy team..."
    Note over App,P: User navigates freely: stock, reservations.<br/>The RealtimeService keeps the WS open globally.

    par Notify staff
        API->>Redis: publish staff:{id} consult.offer
        Redis-->>Staff: WS consult.offer (summary preview)
    and Push fallback
        Worker->>Push: Push to staff devices
    and Reminder timer
        Worker->>Worker: schedule reminder(consult, 2 min) + max_wait(15 min)
    end

    alt Team member accepts
        Ph->>Staff: Accept
        Staff->>API: POST /consults/{id}/accept
        API->>DB: Tx: Consult = Active, staff_id, other offers Withdrawn + outbox
        API-->>Staff: 200 + full case (summary, transcript, profile)
        API->>Redis: publish user:{id} consult.accepted
        alt App in foreground (WS connected)
            Redis-->>App: WS consult.accepted
            App-->>P: Banner "Ana (pharmacy team) joined. Open chat"
        else App in background
            Worker->>Push: Push "The pharmacy team joined"
        end
    else Nobody accepts
        Worker->>Redis: Re-notify Available staff every 2 min
        opt Max wait reached (15 min)
            Worker->>Redis: publish user:{id} consult.long_wait
            Redis-->>App: WS consult.long_wait (keep waiting / callback / route to pharmacy)
        end
        opt 20:00 closing time
            Worker->>DB: Consult = CallbackPending, create CallbackTicket for 08:00
            Worker->>Redis: publish user:{id} consult.no_staff
            Redis-->>App: WS consult.no_staff (next: 08:00, nearby open pharmacies)
        end
    end
```

## SD-2b Transfer from assistant to pharmacist (UC-31)

```mermaid
sequenceDiagram
    autonumber
    participant AsApp as Assistant staff app
    participant API
    participant DB
    participant Redis
    participant PhApp as Pharmacist staff app
    participant App as Patient app

    AsApp->>API: POST /consults/{id}/transfer {note}
    API->>DB: Tx: Consult = Transferring, required_role = Pharmacist, transferred_from = assistant
    API->>Redis: publish to Available pharmacists consult.offer (with note)
    API->>Redis: publish user:{id} consult.transferring
    Redis-->>App: "A pharmacist will join your conversation"
    Redis-->>PhApp: WS consult.offer
    PhApp->>API: POST /consults/{id}/accept
    API->>DB: Tx: Consult = Active, staff_id = pharmacist
    API->>Redis: publish consult.accepted to user + consult.released to assistant
    Redis-->>App: Banner "Pharmacist Ion joined"
    Redis-->>AsApp: Consult released
```

## SD-3 Chat and upgrade to video (UC-06)

```mermaid
sequenceDiagram
    autonumber
    participant App
    participant API
    participant Redis
    participant Staff
    participant LK as LiveKit

    Staff->>API: WS message.send
    API->>API: Persist message + audit
    API->>Redis: publish user:{id} message.new
    Redis-->>App: WS message.new
    App->>API: WS message.read

    Staff->>API: POST /consults/{id}/video
    API->>LK: Create room + tokens (staff, patient)
    API-->>Staff: {room, token}
    API->>Redis: publish user:{id} video.invite
    Redis-->>App: WS video.invite
    App->>API: POST /consults/{id}/video/join
    API-->>App: {room, token}
    App->>LK: Connect WebRTC
    Staff->>LK: Connect WebRTC
    Note over App,Staff: Media goes App ↔ LiveKit ↔ Staff, not through our API
```

## SD-4 Stock search + OTC reservation (UC-08, UC-10)

```mermaid
sequenceDiagram
    autonumber
    actor P as Patient
    participant App
    participant API
    participant DB
    participant Redis
    participant Staff
    participant Worker

    P->>App: Search "ibuprofen"
    App->>API: GET /products/search?q=ibuprofen&lat&lng
    API->>DB: Full-text product search + stock join + PostGIS distance
    API-->>App: Products with per-pharmacy level + distance + open_now
    P->>App: Reserve 1 box at Pharmacy X
    App->>API: POST /reservations {pharmacy, items}
    API->>DB: Tx: Reservation(Submitted) + pickup_code + outbox
    API-->>App: 201 {reservation}
    API->>Redis: publish pharmacy:{X} reservation.new
    Redis-->>Staff: WS reservation.new
    Staff->>API: POST /reservations/{id}/confirm
    API->>DB: status = Confirmed
    Staff->>API: POST /reservations/{id}/ready
    API->>DB: status = Ready, hold_until = now + hold
    API->>Redis: publish user reservation.ready
    Worker->>Worker: schedule expiry(hold_until) + reminder
    Note over P,Staff: At the counter
    Staff->>API: POST /reservations/pickup {pickup_code}
    API->>DB: status = PickedUp
```

## SD-5 Rx reservation with prescription at the counter (UC-11, UC-30)

```mermaid
sequenceDiagram
    autonumber
    actor P as Patient
    participant App
    participant API
    participant OBJ as Object storage
    participant DB
    participant Staff
    actor Ph as Pharmacist

    P->>App: Reserve Rx product
    App-->>P: "This needs a prescription. Register it."
    P->>App: Photo + series/number (or e-Rx code)
    App->>API: POST /prescriptions (metadata) → upload URL
    App->>OBJ: PUT image (signed URL)
    API->>DB: Prescription(Registered)
    App->>API: POST /reservations {items, prescription_id}
    API->>DB: Reservation(Submitted, prescription_id)
    Note over Staff: Staff sees the Rx badge + prescription preview
    Staff->>API: confirm → ready
    P->>Ph: Comes to the counter with the original prescription
    Ph->>Staff: Scan pickup QR
    Staff->>API: GET /reservations/by-code/{code}
    Ph->>Ph: Verifies the original prescription
    alt Valid
        Staff->>API: POST /reservations/{id}/pickup {rx_verified: true}
        API->>DB: Prescription = Used, Reservation = PickedUp
    else Invalid
        Staff->>API: POST /reservations/{id}/pickup {rx_items: refused, reason}
        API->>DB: Rx items RefusedRxInvalid, status PartiallyPickedUp or Cancelled
    end
```

## SD-6 Stock synchronization (worker ↔ ERP)

```mermaid
sequenceDiagram
    autonumber
    participant Worker
    participant ERP
    participant DB
    participant Redis

    loop Every N minutes (configurable, e.g. 5–15)
        Worker->>ERP: Fetch stock delta (API / DB view / export file) (Q1)
        ERP-->>Worker: [(pharmacy, product_code, qty)]
        Worker->>Worker: Map ERP codes → Product.id. qty → level (thresholds)
        Worker->>DB: Upsert stock_entries (synced_at)
        opt Product became OutOfStock with active reservations
            Worker->>Redis: publish pharmacy staff stock.warning
        end
    end
```

## SD-7 Opening / closing time and presence

```mermaid
sequenceDiagram
    autonumber
    participant Worker
    participant DB
    participant Redis
    participant Staff
    actor T as Team member

    Note over Worker: 08:00 pharmacy opens
    Worker->>DB: Open callback tickets created overnight
    Worker->>Redis: publish pharmacy team callbacks.ready
    T->>Staff: Toggle Available (allowed only 08:00-20:00)
    Staff->>Redis: presence = Available (TTL heartbeat 30s)
    Note over Staff,Redis: Heartbeat every 15s keeps presence alive.<br/>No heartbeat → auto Offline.
    Worker->>Redis: Waiting consults? Offer them to the newly Available member
    Note over Worker: 19:50 closing reminder to staff
    Note over Worker: 20:00 pharmacy closes
    Worker->>DB: Waiting consults → CallbackPending (for 08:00)
    Worker->>Redis: presence = Offline for all (active consults may finish)
```
