# 09 — API Outline

> First sketch. The final contract will be an **OpenAPI 3.1** spec generated from Rust (e.g., `utoipa`) and used to generate the Flutter client.
> Base URL: `/api/v1`. Auth: `Authorization: Bearer <jwt>`. JSON, `snake_case`.

---

## 1. REST endpoints

### Identity
| Method | Path | Description | Req |
|---|---|---|---|
| POST | `/auth/otp/request` | Send SMS code to a phone | P-ACC-1 |
| POST | `/auth/otp/verify` | Verify code → access + refresh tokens | P-ACC-1 |
| POST | `/auth/refresh` | Rotate tokens | — |
| POST | `/me/consents` | Give / withdraw consents | P-ACC-2 |
| DELETE | `/me` | Delete account and data | P-ACC-5 |
| POST | `/me/devices` | Register push token | — |

### Profiles
| Method | Path | Description |
|---|---|---|
| GET/POST | `/profiles` | List / create patient profiles (self, family) |
| GET/PATCH/DELETE | `/profiles/{id}` | Manage a profile |
| POST/DELETE | `/profiles/{id}/health-items[/{item_id}]` | Allergies, conditions, current meds |

### Care (AI + consults)
| Method | Path | Description | Req |
|---|---|---|---|
| POST | `/cases` | Start a case `{profile_id}` | P-AI-1 |
| GET | `/cases` / `/cases/{id}` | History / detail (messages, latest summary) | P-AI-7 |
| POST | `/cases/{id}/messages` | Patient message → triggers safety + AI (streamed over WS) | P-AI-1 |
| POST | `/cases/{id}/attachments` | Get a signed upload URL (photo of meds) | P-AI-5 |
| POST | `/cases/{id}/consults` | Request a pharmacist | P-HO-1 |
| DELETE | `/consults/{id}` | Cancel a request | P-HO-8 |
| POST | `/consults/{id}/video/join` | Get a LiveKit token | P-HO-7 |
| POST | `/consults/{id}/rating` | Rate | P-HO-9 |
| POST | `/cases/{id}/callback` | Leave a callback request | P-HO-5 |

### Catalog & pharmacies
| Method | Path | Description |
|---|---|---|
| GET | `/products/search?q=&lat=&lng=` | Products + stock per pharmacy + distance |
| GET | `/products/{id}` | Product detail |
| GET | `/pharmacies?lat=&lng=&open_now=` | Nearby pharmacies |
| GET | `/pharmacies/{id}` | Detail, hours, staff availability summary |
| GET | `/availability` | "Pharmacists available now?" / next available time |

### Reservations & prescriptions
| Method | Path | Description |
|---|---|---|
| POST | `/prescriptions` | Register a prescription → signed upload URL |
| GET | `/prescriptions` | List |
| POST | `/reservations` | Create `{pharmacy_id, items[], prescription_id?, from_recommendation_id?}` |
| GET | `/reservations` / `/reservations/{id}` | List / detail |
| DELETE | `/reservations/{id}` | Cancel |

### Staff
| Method | Path | Description |
|---|---|---|
| PUT | `/staff/me/presence` | Available / Busy / Break / Offline (Available only 08:00–20:00) |
| POST | `/consults/{id}/accept` · `/decline` | Respond to an offer |
| POST | `/consults/{id}/transfer` | Assistant → pharmacist `{note}` |
| PUT | `/staff/stock-flags/{pharmacy_id}/{product_id}` | Manual "out of stock" flag |
| GET | `/staff/consults/{id}` | Full case for an assigned consult (audited) |
| POST | `/consults/{id}/video` | Start video |
| POST | `/consults/{id}/recommendations` | Validate / override / create a recommendation |
| POST | `/consults/{id}/close` | Close with outcome |
| GET | `/staff/callbacks` · POST `/staff/callbacks/{id}/claim` | Callback queue |
| GET | `/staff/reservations?pharmacy_id=&status=` | Pharmacy reservation queue |
| POST | `/reservations/{id}/confirm` · `/reject` · `/ready` | Reservation lifecycle |
| GET | `/reservations/by-code/{code}` | Lookup at pickup |
| POST | `/reservations/{id}/pickup` | Pickup (+ Rx verification per item) |

### Chain admin
| Method | Path | Description |
|---|---|---|
| CRUD | `/admin/pharmacies` | Pharmacies |
| CRUD | `/admin/staff` | Staff + roles |
| CRUD | `/admin/role-permissions` | What each role may do (doc 03 §3) |
| POST | `/admin/stock/upload` | Manual CSV stock upload (MVP, doc 11) |
| GET | `/admin/stock/unmatched` | Unmatched product codes for manual mapping |
| GET | `/admin/reports/*` | Reports |

---

## 2. WebSocket

- Endpoint: `GET /api/v1/ws` (upgrade). Auth with the JWT as the first message (or a query token).
- Envelope:

```json
{ "type": "consult.accepted", "id": "evt_01H…", "ts": "2026-10-29T08:00:00Z", "data": { } }
```

- The client acknowledges important events. On reconnect it sends `last_event_id` so the server replays missed events (from the outbox table, for a limited window).

### Server → patient app
| Event | When |
|---|---|
| `ai.chunk` / `ai.done` | AI streaming |
| `emergency.show` | Red flag detected |
| `summary.updated` | New summary version |
| `consult.queued` | Queue position / ETA changes |
| `consult.long_wait` | Waited 15 min → keep waiting / callback / route to pharmacy |
| `consult.transferring` | Assistant is bringing in a pharmacist |
| `consult.accepted` | Team member (or pharmacist after transfer) joined |
| `consult.no_staff` | Outside 08:00–20:00 / closing → callback / nearby pharmacies |
| `consult.closed` | Consult ended with outcome |
| `message.new` / `message.read` / `typing` | Chat |
| `video.invite` / `video.ended` | Video |
| `recommendation.validated` | Pharmacist validated / overrode |
| `reservation.updated` | Any reservation status change |

### Server → staff app
| Event | When |
|---|---|
| `consult.offer` / `consult.offer_withdrawn` / `consult.reminder` | New offer / someone else took it / still waiting |
| `consult.released` | Transfer completed, assistant released |
| `callback.new` / `callbacks.ready` | New callback ticket / overnight tickets at 08:00 |
| `reservation.new` / `reservation.cancelled` | Pharmacy queue |
| `stock.warning` | Reserved product went out of stock |
| `closing.reminder` | 19:50: pharmacy closes soon |
| `message.new` / `typing` | Chat |

### Client → server
| Event | Purpose |
|---|---|
| `auth` | Authenticate the socket |
| `heartbeat` | Keep-alive (staff heartbeat also refreshes presence) |
| `message.send` / `message.read` / `typing` | Chat |
| `ack` | Acknowledge event id |
| `resume` | `{last_event_id}` after reconnect |
