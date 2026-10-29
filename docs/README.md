# LDS — Design Documentation

> **Phase:** Design. **No code until these documents are reviewed and agreed.**
> The vision and the SOAR are in [../PROJECT_IDEA.md](../PROJECT_IDEA.md).

## Documents

| # | Document | What it contains | Status |
|---|---|---|---|
| 01 | [PROJECT_IDEA.md](../PROJECT_IDEA.md) | Vision, problem, pillars, competitors, SOAR, business model, stack options | ✅ Draft v3 |
| 02 | [Requirements & MVP scope](02-requirements.md) | User stories, MoSCoW, non-functional requirements | 📝 Draft v1 |
| 03 | [Pharmacy team availability](03-pharmacist-availability.md) | Minimal model (counter + AI/callback), roles, routing rules, schedule | ✅ Decided v2 |
| 04 | [Use cases](04-use-cases.md) | UML use case diagrams + use case descriptions | 📝 Draft v2 |
| 05 | [Domain model](05-domain-model.md) | UML class diagram, ER diagram, entity glossary | 📝 Draft v2 |
| 06 | [Architecture](06-architecture.md) | C4 context/container/component, Rust workspace layout, async model | 📝 Draft v2 |
| 07 | [Sequence diagrams](07-sequence-diagrams.md) | AI consult, async handoff, reservations (OTC/Rx), stock sync, video | 📝 Draft v2 |
| 08 | [State machines](08-state-machines.md) | Case, Consult, Reservation, Staff presence | 📝 Draft v2 |
| 09 | [API outline](09-api-outline.md) | REST endpoints + WebSocket events | 📝 Draft v2 |
| 10 | [Decision log (ADRs)](10-decisions.md) | Architecture decision records | 📝 Living |
| 11 | [Stock integration](11-stock-integration.md) | Checklist for the chain's software, connector options, CSV fallback | ❓ Waiting for Q1 |

## Diagram conventions

- All diagrams are **Mermaid** (rendered natively by GitHub, GitLab, VS Code and most Markdown viewers).
- Mermaid has no native UML *use case* diagram, so use cases are drawn as flowcharts: actors on the sides, use cases as rounded nodes.
- Class / sequence / state / ER diagrams use the native Mermaid UML types.

## Glossary

| Term | Meaning |
|---|---|
| **LDS** | Local Distribution System, the platform (working name) |
| **Tenant / Chain** | A pharmacy organization on the platform. The family chain is tenant #1 |
| **Case** | One health episode of a patient: AI conversation + summary + consults + recommendations (like a PNR in travel) |
| **Consult** | A live session between the patient and a human (pharmacist / assistant) inside a Case |
| **Handoff** | Transfer of a Case from the AI to a human, with the summary |
| **Summary** | AI-generated structured brief (SOAP-like) of the Case |
| **Red flag** | Symptom pattern requiring immediate 112 / UPU escalation |
| **OTC** | Over-the-counter medicine (no prescription) |
| **Rx** | Prescription-only medicine |
| **Reservation** | A hold on products at a pharmacy for pickup; paid at the counter |
| **Shift** | *(Later)* A scheduled time window for app consults. MVP uses opening hours instead |
| **Pharmacy team** | Pharmacists + pharmacy assistants who answer in the app, like at the counter |
| **Transfer** | An assistant hands a consult to a pharmacist (same case, same history) |
| **Presence** | Real-time status of a staff member: offline / available / busy / break |
| **CFR** | Colegiul Farmaciștilor din România (pharmacist licensing body) |
| **ANMDMR** | Romanian National Agency for Medicines and Medical Devices |
| **MDR / AI Act** | EU Medical Device Regulation / EU Artificial Intelligence Act |

## Open questions

| # | Question | Owner | Doc | Status |
|---|---|---|---|---|
| Q1 | How does the chain's pharmacy software expose stock (API, DB, export)? Use the checklist | Founder | 11 | ❓ Founder verifying |
| Q2 | Hours: **08:00–20:00** ✅. Same at weekends? Team members on duty per pharmacy? Tablet per pharmacy? | Founder | 03 | 🟡 Partly answered |
| Q3 | Availability model | Founder | 03 | ✅ Minimal (counter + AI/callback), no on-call |
| Q4 | AI regulatory option (human-in-the-loop recommended) | Founder + adviser | 02 | ❓ |
| Q5 | Reservation hold time (OTC / Rx), max items, max active reservations per user | Founder | 02 | ❓ |
| Q6 | Mobile stack | Founder | 10 | ✅ Flutter (front) + Rust (back) |
| Q7 | Brand name, separate from the chain? | Founder | — | ❓ |
| Q8 | App languages (RO only for MVP?) | Founder | 02 | ❓ |
| Q9 | Login method: phone + SMS OTP, email, Google/Apple sign-in? | Founder | 02 | ❓ |
| Q10 | Legal scope of pharmacy assistants (advice on Rx? Rx verification at pickup?) | Adviser | 03 | ❓ |
