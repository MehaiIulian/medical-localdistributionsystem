# Local Distribution System (LDS România) — Sistem Local de Distribuție & Ghidare Medicală de Urgență

> **Concept & Viziune de Proiect (Iulian)**: Un sistem local de distribuție medicală și farmaceutică (LDS — *Local Distribution System*) cu integrare **Google Maps**, inspirat de arhitectura sistemelor de distribuție globală (GDS / Amadeus), care conectează pacienții aflați în dificultate cu farmaciști și medici veterinari de gardă disponibili în timp real.

---

## 💡 Conceptul Central: Local Distribution System (LDS)

În aviație, sistemele de distribuție decuplează inventarul companiilor aeriene de canalele de vânzare printr-un protocol unificat.

În România, **Local Distribution System (LDS)** aplică același model la nivel local pentru urgențe medicale și farmaceutice:
1. **Pacienți derutați**: Când cineva se simte rău noaptea sau are o urgență veterinară cu un animal de companie, nu știe ce medicament poate lua în siguranță din dulapul propriu și la ce unitate non-stop să se deplaseze.
2. **Camere de gardă (UPU) sufocate**: Peste 60% dintre cazuri sunt de gravitate minoră (cod verde/alb) și pot fi rezolvate de un farmacist licențiat sau un medic de gardă.
3. **Rutare Smart pe Google Maps**:
   - Traseul tradițional pe GPS te duce la cea mai apropiată farmacie sau clinică, ignorând dacă acolo este o coadă de 40 de minute.
   - **LDS Google Maps Router** combină timpul de deplasare rutieră furnizat de Google Maps cu **gradul de aglomerare în timp real** al fiecărei unități:
     $$\text{Timp Total} = \text{Timp Conducere Google Maps} + \text{Timp Așteptare la Rând}$$
     Astfel, sistemul te poate îndruma către o farmacie situată cu 500m mai departe, dar care are zero coadă, economisind peste 20 de minute!

---

## 🗺️ Integrarea cu Google Maps

Aplicația oferă suport complet pentru Google Maps:
* **Hartă interactivă live**: Afișează poziția GPS a pacientului și pinii colorați corespunzători farmaciilor umane și clinicilor veterinare de urgență.
* **Deep-Link Turn-by-Turn Navigation**: Fiecărui card de unitate îi este atașat un buton direct care deschide Google Maps:
  `https://www.google.com/maps/dir/?api=1&origin={lat},{lng}&destination={farmacie_lat},{farmacie_lng}&travelmode=driving`
* **Filtrare rapidă pe hartă**:
  - 💊 Toate unitățile
  - 🌙 Doar farmacii non-stop de gardă
  - 🐕 Doar urgențe veterinare (clinici non-stop cu ATI)
  - ⚡ Unități cu timp redus de așteptare (<5 min)

---

## 🏛️ Arhitectura Tehnică a Sistemului

```
                   ┌─────────────────────────────────────────┐
                   │    Client Web / Mobile (Pacient)        │
                   │  - Wizard ghidat pas cu pas             │
                   │  - Hartă interactivă Google Maps        │
                   │  - Video inspecție ambalaje la cameră   │
                   └────────────────────┬────────────────────┘
                                        │ REST / WebSocket
                                        ▼
                   ┌─────────────────────────────────────────┐
                   │           LDS Core Engine               │
                   │         (Dezvoltat în Rust)             │
                   │                                         │
                   │  ┌───────────────────────────────────┐  │
                   │  │ AI Triage & SBAR Engine           │  │
                   │  │ - Detecție semnale 112 (infarct)  │  │
                   │  │ - Verificator toxicitate vet      │  │
                   │  └─────────────────┬─────────────────┘  │
                   │                    ▼                    │
                   │  ┌───────────────────────────────────┐  │
                   │  │ Google Maps & Busy Hour Router    │  │
                   │  │ - Calcul timp condus + timp coadă │  │
                   │  │ - Rutare către unitatea optimă    │  │
                   │  └─────────────────┬─────────────────┘  │
                   │                    ▼                    │
                   │  ┌───────────────────────────────────┐  │
                   │  │ Local Name Record (LDS Store)     │  │
                   │  └───────────────────────────────────┘  │
                   └────────────────────┬────────────────────┘
                                        │
           ┌────────────────────────────┴────────────────────────────┐
           ▼                                                         ▼
┌──────────────────────────────┐                         ┌──────────────────────────────┐
│  Cockpit Farmacie de Gardă   │                         │  Rețeaua LDS & Google Maps   │
│  - Sumar SBAR pre-consult    │                         │  - Lanțuri (Catena, Dr.Max)  │
│  - Video / Chat în timp real │                         │  - Farmacii Independente     │
│  - Emitere recomandare OTC   │                         │  - Spitale Veterinare NonStop│
└──────────────────────────────┘                         └──────────────────────────────┘
```

---

## 📁 Structura Fișierelor

```
medgds/
├── Cargo.toml               # Configurație Rust: package 'local-distribution-system'
├── README.md                # Această documentație
├── run_mock_server.py       # Server prototip Python 3 cu API REST și Google Maps metadata
├── static/
│   └── index.html           # Aplicație completă cu tab dedicat Google Maps & Smart Routing
└── src/
    ├── main.rs              # Server Axum cu rute REST & WebSockets
    ├── models/
    │   ├── gds.rs           # Modele LDS: Provider, BusyLevel, ResourceAgent, Google Maps URL
    │   ├── incident.rs      # MedicalNameRecord (MNR), SbarBrief, Recomandări
    │   └── triage.rs        # Schema de triaj, UrgencyLevel, RedFlagAlert
    ├── services/
    │   ├── ai_triage.rs     # Evaluator SBAR și detecție urgențe 112 / toxicitate vet
    │   ├── gds_router.rs    # Algoritm de dispatch, calcul timp așteptare & Google Maps URLs
    │   └── signaling.rs     # WebSockets & WebRTC signaling pentru video
    └── api/
        ├── patient.rs       # Endpoint-uri pacient (/triage, /mnr/:id)
        ├── pharmacist.rs    # Endpoint-uri cockpit farmacist (/queue, /claim, /recommend)
        └── gds.rs           # Endpoint-uri rețea LDS (/providers, /stats)
```

---

## 🚀 Rulare & Testare Imediată

### Pasul 1: Lansarea Serverului
În terminal, rulați:
```bash
cd /home/mihai/workspace/projects/medgds
python3 run_mock_server.py
```

### Pasul 2: Deschiderea Aplicației
Deschideți în browser **`http://localhost:3000`**

### Ce puteți testa:
1. **Tab-ul 🗺️ Harta Google Maps & Rutare**:
   - Vedeți unitățile pe hartă, pinii colorați pentru farmacii și clinici veterinare.
   - Apăsați **"Localizează-mă pe hartă"** pentru a centra pe GPS-ul dvs.
   - Testați banner-ul **"Optimizare LDS Smart Routing"** care compară timpul de condus cu timpul de coadă.
   - Apăsați pe orice farmacie și dați click pe **"Navighează cu Google Maps"** pentru a deschide aplicația Google Maps oficială cu traseul calculat.
2. **Tab-ul 🩺 Ghidare Pacient**:
   - Parcurgeți wizard-ul pas cu pas, adăugați medicamentele din casă și primiți fișa SBAR + farmacia recomandată pe Google Maps.
3. **Tab-ul 💊 Cockpit Farmacist**:
   - Preluați dosarele pacienților direct informati, fără întrebări redundante.
