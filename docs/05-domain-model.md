# 05 — Domain Model (UML)

> Logical model. Field lists are indicative: enough to reason about the design, not the final SQL.
> Bounded contexts (→ Rust modules, see doc 06): **Identity**, **Organization**, **Catalog & Stock**, **Care** (cases, consults), **Availability** (opening hours, presence), **Reservations**, **Notifications**, **Audit**.

---

## 1. UML class diagram

```mermaid
classDiagram
    direction LR

    %% ---------- Identity ----------
    class User {
        +Uuid id
        +String phone
        +String? email
        +DateTime created_at
        +ConsentSet consents
        +delete_account()
    }
    class PatientProfile {
        +Uuid id
        +Uuid owner_user_id
        +String display_name
        +Relation relation
        +u16 birth_year
        +Sex sex
        +bool pregnant_or_breastfeeding
    }
    class HealthItem {
        +Uuid id
        +HealthItemKind kind
        +String label
        +String? code
    }
    class StaffMember {
        +Uuid id
        +Uuid user_id
        +Uuid tenant_id
        +StaffRole role
        +String? cfr_license_no
        +bool verified
    }

    %% ---------- Organization ----------
    class Tenant {
        +Uuid id
        +String name
        +TenantSettings settings
    }
    class Pharmacy {
        +Uuid id
        +Uuid tenant_id
        +String name
        +Address address
        +GeoPoint location
        +Vec~OpeningHours~ hours
        +bool non_stop
        +is_open_at(DateTime) bool
    }

    %% ---------- Catalog & Stock ----------
    class Product {
        +Uuid id
        +String name
        +String active_substance
        +String form
        +String strength
        +bool rx_required
        +String? anm_code
        +String? leaflet_url
    }
    class StockEntry {
        +Uuid pharmacy_id
        +Uuid product_id
        +StockLevel level
        +i32? quantity
        +StockSource source
        +bool manual_out_of_stock
        +DateTime synced_at
    }

    %% ---------- Care ----------
    class Case {
        +Uuid id
        +Uuid patient_profile_id
        +CaseStatus status
        +bool red_flag_shown
        +DateTime opened_at
        +DateTime? closed_at
    }
    class Message {
        +Uuid id
        +Uuid case_id
        +SenderKind sender
        +Uuid? sender_staff_id
        +String body
        +Vec~Attachment~ attachments
        +DateTime sent_at
    }
    class CaseSummary {
        +Uuid case_id
        +u32 version
        +String subjective
        +String objective
        +String assessment
        +String plan
        +Vec~String~ red_flags
        +DateTime generated_at
    }
    class Consult {
        +Uuid id
        +Uuid case_id
        +ConsultStatus status
        +Uuid? staff_id
        +StaffRole? required_role
        +Uuid? transferred_from_staff_id
        +String? transfer_note
        +DateTime requested_at
        +DateTime? accepted_at
        +DateTime? ended_at
        +ConsultOutcome? outcome
    }
    class ConsultOffer {
        +Uuid id
        +Uuid consult_id
        +Uuid staff_id
        +OfferStatus status
        +DateTime expires_at
    }
    class Recommendation {
        +Uuid id
        +Uuid case_id
        +RecSource source
        +RecStatus status
        +Uuid? validated_by
        +String advice
    }
    class RecommendationItem {
        +Uuid product_id
        +String dosage_instructions
    }

    %% ---------- Availability (MVP: opening hours + presence) ----------
    class Presence {
        +Uuid staff_id
        +PresenceStatus status
        +u8 active_consults
        +DateTime updated_at
    }
    class RolePermission {
        +Uuid tenant_id
        +StaffRole role
        +Permission permission
        +bool allowed
    }

    %% ---------- Reservations ----------
    class Reservation {
        +Uuid id
        +Uuid user_id
        +Uuid pharmacy_id
        +ReservationStatus status
        +String pickup_code
        +DateTime? hold_until
        +Uuid? prescription_id
        +Uuid? from_recommendation_id
    }
    class ReservationItem {
        +Uuid product_id
        +u16 quantity
        +ItemPickupStatus pickup_status
    }
    class Prescription {
        +Uuid id
        +Uuid patient_profile_id
        +PrescriptionKind kind
        +String? series_number
        +String? image_key
        +DateTime? issued_on
        +PrescriptionStatus status
    }

    %% ---------- Cross-cutting ----------
    class Device {
        +Uuid id
        +Uuid user_id
        +Platform platform
        +String push_token
    }
    class CallbackTicket {
        +Uuid id
        +Uuid case_id
        +TicketStatus status
        +DateTime created_at
    }
    class AuditEvent {
        +Uuid id
        +Uuid actor_user_id
        +String action
        +String resource
        +DateTime at
    }

    %% ---------- Relations ----------
    User "1" --> "1..*" PatientProfile : owns
    User "1" --> "0..1" StaffMember : may be
    User "1" --> "*" Device
    PatientProfile "1" --> "*" HealthItem : allergies, conditions, meds
    Tenant "1" --> "*" Pharmacy
    Tenant "1" --> "*" StaffMember
    StaffMember "*" --> "0..1" Pharmacy : home pharmacy
    Pharmacy "1" --> "*" StockEntry
    Product "1" --> "*" StockEntry
    PatientProfile "1" --> "*" Case
    Case "1" *-- "*" Message
    Case "1" *-- "*" CaseSummary : versions
    Case "1" --> "*" Consult
    Case "1" --> "*" Recommendation
    Case "1" --> "0..1" CallbackTicket
    Consult "1" *-- "*" ConsultOffer
    Recommendation "1" *-- "*" RecommendationItem
    RecommendationItem "*" --> "1" Product
    StaffMember "1" --> "1" Presence
    Tenant "1" --> "*" RolePermission
    User "1" --> "*" Reservation
    Pharmacy "1" --> "*" Reservation
    Reservation "1" *-- "1..*" ReservationItem
    ReservationItem "*" --> "1" Product
    Reservation "*" --> "0..1" Prescription
    PatientProfile "1" --> "*" Prescription
```

---

## 2. Enumerations

| Enum | Values |
|---|---|
| `Relation` | Self, Child, Parent, Partner, Other |
| `HealthItemKind` | Allergy, Condition, CurrentMedication |
| `StaffRole` | Pharmacist, PharmacyAssistant, ChainAdmin |
| `StockSource` | Connector, CsvUpload, ManualFlag |
| `StockLevel` | Available, Low, OutOfStock, Unknown |
| `CaseStatus` | OpenAi, EmergencyShown, WaitingHuman, WithHuman, CallbackPending, Closed |
| `SenderKind` | Patient, Ai, Staff, System |
| `ConsultStatus` | Requested, Waiting, Offering, Active, Transferring, Completed, Cancelled, Expired, CallbackPending |
| `Permission` | AcceptConsult, ValidateOtcRecommendation, AnswerRxQuestions, VerifyRxAtPickup, ProcessReservations, FlagStock |
| `OfferStatus` | Pending, Accepted, Declined, TimedOut, Withdrawn |
| `ConsultOutcome` | Resolved, ReferredDoctor, ReferredEmergency, NoResponse |
| `RecSource` | Ai, Pharmacist |
| `RecStatus` | Draft, PendingValidation, Validated, Overridden |
| `PresenceStatus` | Offline, Available, Busy, Break |
| `ReservationStatus` | Submitted, Confirmed, Ready, PickedUp, PartiallyPickedUp, Rejected, Cancelled, Expired |
| `ItemPickupStatus` | Pending, Handed, RefusedRxInvalid, Unavailable |
| `PrescriptionKind` | Paper, Electronic |
| `PrescriptionStatus` | Registered, VerifiedAtCounter, Refused, Used |
| `TicketStatus` | Open, Claimed, Done |

> **Rust implementation note:** these map naturally to Rust `enum`s. Look at `serde` (`#[serde(rename_all = "snake_case")]`) for JSON, and at `sqlx::Type` for Postgres enums. Enums with data (e.g., an outcome with a reason) are where Rust shines compared to C++.

---

## 3. ER diagram (persistence view)

```mermaid
erDiagram
    USERS ||--o{ PATIENT_PROFILES : owns
    USERS ||--o| STAFF_MEMBERS : "may be"
    USERS ||--o{ DEVICES : has
    PATIENT_PROFILES ||--o{ HEALTH_ITEMS : has
    TENANTS ||--o{ PHARMACIES : has
    TENANTS ||--o{ STAFF_MEMBERS : employs
    PHARMACIES ||--o{ STOCK_ENTRIES : stocks
    PRODUCTS ||--o{ STOCK_ENTRIES : "stocked as"
    PATIENT_PROFILES ||--o{ CASES : opens
    CASES ||--o{ MESSAGES : contains
    CASES ||--o{ CASE_SUMMARIES : versions
    CASES ||--o{ CONSULTS : has
    CONSULTS ||--o{ CONSULT_OFFERS : offers
    CASES ||--o{ RECOMMENDATIONS : has
    RECOMMENDATIONS ||--o{ RECOMMENDATION_ITEMS : lists
    TENANTS ||--o{ ROLE_PERMISSIONS : configures
    USERS ||--o{ RESERVATIONS : makes
    PHARMACIES ||--o{ RESERVATIONS : receives
    RESERVATIONS ||--|{ RESERVATION_ITEMS : contains
    PRODUCTS ||--o{ RESERVATION_ITEMS : "reserved as"
    PATIENT_PROFILES ||--o{ PRESCRIPTIONS : holds
    PRESCRIPTIONS ||--o{ RESERVATIONS : "used by"
    CASES ||--o| CALLBACK_TICKETS : may_have
    USERS ||--o{ AUDIT_EVENTS : performs

    USERS {
        uuid id PK
        text phone UK
        text email
        jsonb consents
        timestamptz created_at
    }
    PHARMACIES {
        uuid id PK
        uuid tenant_id FK
        text name
        geography location
        jsonb opening_hours
        bool non_stop
    }
    PRODUCTS {
        uuid id PK
        text name
        text active_substance
        bool rx_required
        text anm_code UK
    }
    STOCK_ENTRIES {
        uuid pharmacy_id PK
        uuid product_id PK
        text level
        int quantity
        timestamptz synced_at
    }
    CASES {
        uuid id PK
        uuid patient_profile_id FK
        text status
        bool red_flag_shown
        timestamptz opened_at
    }
    CONSULTS {
        uuid id PK
        uuid case_id FK
        uuid staff_id FK
        text status
        text required_role
        uuid transferred_from_staff_id FK
        timestamptz requested_at
    }
    RESERVATIONS {
        uuid id PK
        uuid user_id FK
        uuid pharmacy_id FK
        uuid prescription_id FK
        text status
        text pickup_code UK
        timestamptz hold_until
    }
    ROLE_PERMISSIONS {
        uuid tenant_id PK
        text role PK
        text permission PK
        bool allowed
    }
```

---

## 4. Design notes

- **Multi-tenancy:** every org-owned table has a `tenant_id` (directly or through `pharmacy_id`). Patients are **platform-level**, not tenant-level: one patient can reserve at any chain later (GDS vision).
- **Case ≈ PNR:** the Case is the central record that travels between the AI, staff and reservations.
- **Summary versions:** every AI update creates a new `CaseSummary` version. Staff see the latest one and can view the history (traceability).
- **Presence is hot data:** stored in **Redis** (fast, expiring), with a snapshot in Postgres for reports.
- **Availability in MVP = opening hours + presence.** No `Shift` / `OnCallWindow` entities yet. They return with Modes B/C (doc 03 §6).
- **Pharmacy team:** pharmacists and assistants are both `StaffMember`s. What each role may do is data (`RolePermission`), not code, so the chain can adjust it after legal confirmation (Q10).
- **Transfer:** the same `Consult` is re-offered with `required_role = Pharmacist`. `transferred_from_staff_id` keeps the trace.
- **Prescription images:** stored in object storage (S3-compatible, EU) with encryption. Only `image_key` lives in the DB. Short-lived signed URLs for staff.
- **Stock:** `level` is what the user sees. `quantity` stays internal (and is optional if the ERP doesn't expose it).
- **Soft vs hard delete:** account deletion = hard delete of personal data + anonymized aggregates kept for statistics (to confirm with legal).
