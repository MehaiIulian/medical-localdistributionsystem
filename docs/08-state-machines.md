# 08 — State Machines

> Each of these is persisted in Postgres. Transitions happen in the `domain` crate as pure functions:
> `fn transition(state, event) -> Result<NewState, DomainError>`.
> 🦀 **Rust hint:** model states as an `enum` and transitions with `match (state, event)`. The compiler forces you to handle every combination, which C++ `switch` won't.

---

## 1. Case

```mermaid
stateDiagram-v2
    [*] --> OpenAi : patient starts case
    OpenAi --> EmergencyShown : red flag detected
    EmergencyShown --> OpenAi : patient continues
    OpenAi --> WaitingHuman : request pharmacist
    EmergencyShown --> WaitingHuman : request pharmacist
    WaitingHuman --> WithHuman : consult accepted
    WaitingHuman --> CallbackPending : no staff available
    WaitingHuman --> OpenAi : request cancelled
    CallbackPending --> WithHuman : staff claims ticket
    WithHuman --> OpenAi : consult ended, patient continues with AI
    OpenAi --> Closed : patient closes / inactivity 24h
    WithHuman --> Closed : consult closed with outcome
    CallbackPending --> Closed : ticket done / expired
    Closed --> [*]
```

## 2. Consult (handoff)

```mermaid
stateDiagram-v2
    [*] --> Requested
    Requested --> Offering : within 08-20 and team member Available
    Requested --> Waiting : within 08-20, nobody Available
    Requested --> CallbackPending : outside opening hours
    Waiting --> Offering : someone becomes Available
    Offering --> Offering : reminder every 2 min
    Offering --> Active : team member accepts
    Offering --> Waiting : all offers declined
    Waiting --> CallbackPending : patient chooses callback / closing at 20h
    Offering --> CallbackPending : closing at 20h
    Offering --> Cancelled : patient cancels
    Waiting --> Cancelled : patient cancels
    Active --> Active : video started / ended
    Active --> Transferring : assistant transfers to pharmacist
    Transferring --> Active : pharmacist accepts
    Transferring --> Active : no pharmacist, assistant keeps consult
    Active --> Completed : staff closes with outcome
    Active --> Completed : patient leaves + timeout
    CallbackPending --> Active : team member claims callback
    CallbackPending --> Expired : not claimed within SLA
    Completed --> [*]
    Cancelled --> [*]
    Expired --> [*]
```

## 3. Consult offer (per staff member)

```mermaid
stateDiagram-v2
    [*] --> Pending
    Pending --> Accepted : staff accepts (first wins)
    Pending --> Declined : staff declines
    Pending --> TimedOut : timer fires
    Pending --> Withdrawn : another staff accepted / patient cancelled
    Accepted --> [*]
    Declined --> [*]
    TimedOut --> [*]
    Withdrawn --> [*]
```

> ⚠️ **Concurrency:** two pharmacists can tap "Accept" at the same time. Solve it in the DB:
> `UPDATE consults SET status='active', staff_id=$1 WHERE id=$2 AND status='offering'`. Exactly one gets `rows_affected = 1`. (This is a nice first exercise in optimistic concurrency with `sqlx`.)

## 4. Reservation

```mermaid
stateDiagram-v2
    [*] --> Submitted : patient reserves
    Submitted --> Confirmed : pharmacy confirms
    Submitted --> Rejected : out of stock / invalid
    Submitted --> Cancelled : patient cancels
    Submitted --> Expired : not confirmed in X min
    Confirmed --> Ready : prepared, hold timer starts
    Confirmed --> Cancelled : patient cancels
    Ready --> PickedUp : all items handed
    Ready --> PartiallyPickedUp : Rx item refused at counter
    Ready --> Cancelled : patient cancels
    Ready --> Expired : hold_until passed
    PickedUp --> [*]
    PartiallyPickedUp --> [*]
    Rejected --> [*]
    Cancelled --> [*]
    Expired --> [*]
```

## 5. Prescription

```mermaid
stateDiagram-v2
    [*] --> Registered : patient uploads
    Registered --> VerifiedAtCounter : pharmacist checks original
    Registered --> Refused : invalid / expired
    VerifiedAtCounter --> Used : medicine dispensed
    Refused --> [*]
    Used --> [*]
```

## 6. Staff presence

```mermaid
stateDiagram-v2
    [*] --> Offline
    Offline --> Available : toggle (only during opening hours 08-20)
    Available --> Busy : manual / max consults reached
    Busy --> Available : manual / consult finished
    Available --> Break : manual
    Break --> Available : manual
    Available --> Offline : closing at 20h / heartbeat lost
    Busy --> Offline : closing at 20h and no active consult
    Break --> Offline : closing at 20h
```

## 7. Recommendation (human-in-the-loop option)

```mermaid
stateDiagram-v2
    [*] --> Draft : AI generates
    Draft --> PendingValidation : shown to patient as "pending pharmacist review"
    Draft --> Validated : regulatory option 2 (info only, no validation)
    PendingValidation --> Validated : pharmacist approves
    PendingValidation --> Overridden : pharmacist edits / replaces
    Validated --> [*]
    Overridden --> [*]
```
