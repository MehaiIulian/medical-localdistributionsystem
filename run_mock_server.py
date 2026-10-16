#!/usr/bin/env python3
"""
MedGDS Local Prototype Server (Zero-Dependency Python Runner)
Allows running the full MedGDS system immediately using standard library Python 3.
When Rust/Cargo is installed, `cargo run` runs the production Axum backend.
"""

import http.server
import socketserver
import json
import os
import uuid
from urllib.parse import urlparse

PORT = int(os.environ.get("PORT", 3000))
STATIC_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "static")

# In-memory storage for prototype
INCIDENTS = {}
PROVIDERS = [
    {
        "id": "p1",
        "name": "Farmacia Tei - Barbu Văcărescu",
        "chain": "FarmaciaTei",
        "city": "București (Sector 2)",
        "address": "Bulevardul Barbu Văcărescu 120",
        "latitude": 44.4623,
        "longitude": 26.1044,
        "phone": "+40 21 230 40 50",
        "is_non_stop": True,
        "is_active_on_call": True,
        "current_queue_load": 1,
        "busy_level": "Low",
        "estimated_wait_minutes": 4,
        "google_maps_url": "https://www.google.com/maps/dir/?api=1&destination=44.4623,26.1044"
    },
    {
        "id": "p2",
        "name": "Catena Non-Stop Piața Romană",
        "chain": "Catena",
        "city": "București (Sector 1)",
        "address": "Piața Romană 9",
        "latitude": 44.4448,
        "longitude": 26.0975,
        "phone": "+40 800 800 020",
        "is_non_stop": True,
        "is_active_on_call": True,
        "current_queue_load": 0,
        "busy_level": "Low",
        "estimated_wait_minutes": 2,
        "google_maps_url": "https://www.google.com/maps/dir/?api=1&destination=44.4448,26.0975"
    },
    {
        "id": "p3",
        "name": "Dr. Max Farmacie Cluj Central",
        "chain": "DrMax",
        "city": "Cluj-Napoca",
        "address": "Piața Unirii 4",
        "latitude": 46.7702,
        "longitude": 23.5898,
        "phone": "+40 364 400 500",
        "is_non_stop": True,
        "is_active_on_call": True,
        "current_queue_load": 2,
        "busy_level": "Moderate",
        "estimated_wait_minutes": 12,
        "google_maps_url": "https://www.google.com/maps/dir/?api=1&destination=46.7702,23.5898"
    },
    {
        "id": "p4",
        "name": "Spitalul Veterinar Non-Stop București",
        "chain": "Independent",
        "city": "București (Sector 3)",
        "address": "Șoseaua Mihai Bravu 245",
        "latitude": 44.4265,
        "longitude": 26.1325,
        "phone": "+40 722 000 999",
        "is_non_stop": True,
        "is_active_on_call": True,
        "current_queue_load": 1,
        "busy_level": "Low",
        "estimated_wait_minutes": 5,
        "google_maps_url": "https://www.google.com/maps/dir/?api=1&destination=44.4265,26.1325"
    }
]

class MedGdsRequestHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=STATIC_DIR, **kwargs)

    def do_GET(self):
        parsed = urlparse(self.path)
        path = parsed.path

        if path == "/api/v1/pharmacist/queue":
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Access-Control-Allow-Origin", "*")
            self.end_headers()
            pending = [inc for inc in INCIDENTS.values() if inc.get("status") in ("AiTriagedWaitingPharmacist", "IntakeSubmitted")]
            self.wfile.write(json.dumps(pending).encode("utf-8"))
            return

        elif path == "/api/v1/gds/providers":
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Access-Control-Allow-Origin", "*")
            self.end_headers()
            self.wfile.write(json.dumps(PROVIDERS).encode("utf-8"))
            return

        elif path.startswith("/api/v1/patient/mnr/"):
            token_or_id = path.split("/")[-1]
            match = next((inc for inc in INCIDENTS.values() if inc["id"] == token_or_id or inc["access_token"] == token_or_id), None)
            if match:
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Access-Control-Allow-Origin", "*")
                self.end_headers()
                self.wfile.write(json.dumps(match).encode("utf-8"))
            else:
                self.send_response(404)
                self.send_header("Content-Type", "application/json")
                self.end_headers()
                self.wfile.write(json.dumps({"error": "Incident not found"}).encode("utf-8"))
            return

        super().do_GET()

    def do_POST(self):
        parsed = urlparse(self.path)
        path = parsed.path
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length) if length > 0 else b"{}"
        data = json.loads(body.decode("utf-8")) if body else {}

        if path == "/api/v1/patient/triage":
            inc_id = str(uuid.uuid4())
            token = f"MNR-{inc_id[:6].upper()}"

            complaint = data.get("main_complaint", "")
            lower_complaint = complaint.lower()
            is_red = any(k in lower_complaint for k in ["piept", "respiratie", "ciocolata", "antigel", "asfixie"])

            urgency = "RedEmergency" if is_red else ("OrangeUrgent" if data.get("pain_scale_1_to_10", 0) >= 8 else "YellowModerate")

            sbar = {
                "situation": f"Pacient: {data.get('patient_category')}, vârstă: {data.get('patient_age')} ani. Acuză: \"{complaint}\". Scor durere: {data.get('pain_scale_1_to_10', 0)}/10.",
                "background": f"Debut de cca {data.get('duration_hours', 1)} ore. Simptome asociate: [{', '.join(data.get('symptom_tags', []))}]. Alergii: {', '.join(data.get('known_allergies', [])) or 'Fără alergii declarate'}.",
                "assessment": f"Evaluare AI: {urgency}. Trusă acasă: [{', '.join(data.get('current_medications_at_home', []))}].",
                "recommendation": "Confirmare posologie trusă proprie sau orientare către farmacie parteneră."
            }

            mnr = {
                "id": inc_id,
                "access_token": token,
                "intake": data,
                "urgency": urgency,
                "red_flags": {
                    "is_critical": True,
                    "warning_message_ro": "ALERTA 112: Simptome critice identificate.",
                    "recommended_action": "Sunați imediat la 112!"
                } if is_red else None,
                "sbar_brief": sbar,
                "status": "EscalatedToEmergency112" if is_red else "AiTriagedWaitingPharmacist",
                "assigned_pharmacist_name": None,
                "assigned_pharmacy_name": None,
                "recommendation": None
            }

            INCIDENTS[inc_id] = mnr

            self.send_response(201)
            self.send_header("Content-Type", "application/json")
            self.send_header("Access-Control-Allow-Origin", "*")
            self.end_headers()
            self.wfile.write(json.dumps(mnr).encode("utf-8"))
            return

        elif "/pharmacist/claim/" in path:
            inc_id = path.split("/")[-1]
            if inc_id in INCIDENTS:
                INCIDENTS[inc_id]["status"] = "LiveSessionActive"
                INCIDENTS[inc_id]["assigned_pharmacist_name"] = data.get("pharmacist_name", "Farmacist de Gardă")
                INCIDENTS[inc_id]["assigned_pharmacy_name"] = data.get("pharmacy_name", "Farmacie Parteneră")
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Access-Control-Allow-Origin", "*")
                self.end_headers()
                self.wfile.write(json.dumps(INCIDENTS[inc_id]).encode("utf-8"))
                return

        self.send_response(404)
        self.end_headers()

if __name__ == "__main__":
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("", PORT), MedGdsRequestHandler) as httpd:
        print(f"🚀 Local Distribution System (LDS) pornit pe http://localhost:{PORT}")
        print(f"📁 Servind fișierele din: {STATIC_DIR}")
        print("🗺️  Integrare Google Maps activată")
        print("Apăsați Ctrl+C pentru oprire.")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nServer oprit.")

