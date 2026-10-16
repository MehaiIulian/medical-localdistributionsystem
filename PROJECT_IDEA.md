# Iulian's project idea for a medical app in Romania (request nearby pharmacist/doctor to guide an emergency) — scope and status

## Details
- Wants to build a medical project in Romania: an app that requests an available nearby pharmacist or doctor to guide someone through a medical emergency
- Going with teleconsult plus orientation to the nearest medical unit, and wants to extend it to animals too (veterinary teleconsult/orientation)
- Decided to start with the veterinary side and scale to human medicine later
- Wants to keep costs and resources for this project as low as possible
- Says TelePaws (Romanian vet telemedicine competitor) offers live consultation too, so he is looking for what to add on top of it to differentiate
- Wants an AI bot that talks with the user while waiting for a vet to connect (holds the conversation in an emergency) and then gives the vet a short case summary on connection
- Now leaning toward an app (no online consult option) combining: AI receptionist for vet clinics, emergency vet map, route guidance that factors in how busy each clinic is at that time of day, and a quick toxicity checker / urgent quick recommendations
- Brainstorming further ideas for Romania: wants a scalable project with a large, open audience (medicine preferred, but any domain works); observes people prefer being gently guided step by step in an app to reach the end result
- New direction: wants to invent a common medical distribution layer, a "GDS for medicine" that many providers can connect to, inspired by Amadeus (a travel-booking GDS he says he works with)
- Leaning towards Rust for the project's implementation, mainly to learn Rust for the future
- Concrete concept: app connecting people who feel unwell and don't know what medication to take with a pharmacist; user writes symptoms quickly while waiting, the pharmacist joins already briefed and gives a direct recommendation without re-asking; user can start live video to show what they have at hand (medicines, packaging)
- Wants pharmacy profiles on the platform to benefit the pharmacy (win clients) and to support pharmacy chains/networks too, not only independents
- Wants the platform to be a scalable GDS-style layer, planning to expand from pharmacies to hospitals and veterinary pharmacies later
