use crate::models::triage::{PatientCategory, PatientIntakePayload, RedFlagAlert, UrgencyLevel};
use crate::models::incident::SbarBrief;

pub struct AiTriageEngine;

impl AiTriageEngine {
    pub fn evaluate(intake: &PatientIntakePayload) -> (UrgencyLevel, Option<RedFlagAlert>, SbarBrief) {
        let combined_text = format!(
            "{} {}",
            intake.main_complaint.to_lowercase(),
            intake.symptom_tags.join(" ").to_lowercase()
        );

        // 1. Red Flag & Toxicity Rules Engine (Human & Vet)
        let red_flag = Self::check_red_flags(intake, &combined_text);
        
        let urgency = if red_flag.is_some() {
            UrgencyLevel::RedEmergency
        } else if intake.pain_scale_1_to_10 >= 8 || intake.duration_hours > 72 {
            UrgencyLevel::OrangeUrgent
        } else if intake.pain_scale_1_to_10 >= 4 || !intake.symptom_tags.is_empty() {
            UrgencyLevel::YellowModerate
        } else {
            UrgencyLevel::GreenMild
        };

        // 2. SBAR Briefing Synthesis (Designed for Romanian Pharmacists / Vets)
        let sbar = Self::generate_sbar(intake, &urgency, &combined_text);

        (urgency, red_flag, sbar)
    }

    fn check_red_flags(intake: &PatientIntakePayload, text: &str) -> Option<RedFlagAlert> {
        let is_pet = matches!(
            intake.patient_category,
            PatientCategory::PetDog | PatientCategory::PetCat | PatientCategory::PetOther
        );

        if is_pet {
            let vet_toxins = ["ciocolata", "antigel", "otrava", "crin", "paracetamol", "xilitol", "struguri", "ceapa"];
            for toxin in vet_toxins {
                if text.contains(toxin) {
                    return Some(RedFlagAlert {
                        is_critical: true,
                        warning_message_ro: format!(
                            "ALERTA VETERINARA CRITICA: Posibila ingestie de substanta toxica ({toxin}). Ingestia de {toxin} la animale poate cauza blocaj renal sau insuficienta hepatica acuta.",
                        ),
                        recommended_action: "Deplasati-va imediat la cea mai apropiata clinica veterinara non-stop cu dotari de terapie intensiva.".to_string(),
                    });
                }
            }

            if text.contains("torsiune") || text.contains("varsaturi repetate") || text.contains("inconstient") {
                return Some(RedFlagAlert {
                    is_critical: true,
                    warning_message_ro: "ALERTA VETERINARA CRITICA: Simptome sugestive pentru torsiune gastrica sau colaps acut.".to_string(),
                    recommended_action: "Prezentare de urgenta la spital veterinar non-stop!".to_string(),
                });
            }
        } else {
            // Human red flags
            let human_emergencies = [
                ("durere in piept", "Posibil sindrom coronarian acut / infarct miocardic"),
                ("dificultati de respiratie", "Posibil edem pulmonar, bronhospasm sever sau soc anafilactic"),
                ("asfixie", "Obstructie de cai aeriene"),
                ("paralizie", "Semne posibile de accident vascular cerebral (AVC - protocol FAST)"),
                ("lesin", "Sincopa sau pierdere a starii de constienta"),
                ("hemoragie", "Sangerare activa necontrolata"),
            ];

            for (keyword, desc) in human_emergencies {
                if text.contains(keyword) {
                    return Some(RedFlagAlert {
                        is_critical: true,
                        warning_message_ro: format!("URGENTA MEDICALA VITALA: {desc}."),
                        recommended_action: "Apelati imediat 112! Nu asteptati consultatia farmacistului.".to_string(),
                    });
                }
            }
        }

        None
    }

    fn generate_sbar(intake: &PatientIntakePayload, urgency: &UrgencyLevel, _text: &str) -> SbarBrief {
        let target_str = match intake.patient_category {
            PatientCategory::AdultSelf => "Adult",
            PatientCategory::Child => "Copil / Minor",
            PatientCategory::Elderly => "Persoana varstnica",
            PatientCategory::PetDog => "Animal de companie (Caine)",
            PatientCategory::PetCat => "Animal de companie (Pisica)",
            PatientCategory::PetOther => "Animal de companie (Alte specii)",
        };

        let age_info = intake.patient_age
            .map(|a| format!(", varsta: {a} ani"))
            .unwrap_or_default();

        let situation = format!(
            "Pacient: {target_str}{age_info}. Acuza principala: \"{}\". Nivel durere raportat: {}/10.",
            intake.main_complaint, intake.pain_scale_1_to_10
        );

        let allergies_str = if intake.known_allergies.is_empty() {
            "Fara alergii medicamentoase declarate".to_string()
        } else {
            format!("ALERGII DECLARATE: {}", intake.known_allergies.join(", "))
        };

        let chronic_str = if intake.chronic_conditions.is_empty() {
            "Fara afectiuni cronice active".to_string()
        } else {
            format!("Comorbiditati: {}", intake.chronic_conditions.join(", "))
        };

        let background = format!(
            "Debut de cca {} ore. Simptome asociate: [{}]. {} | {}.",
            intake.duration_hours,
            intake.symptom_tags.join(", "),
            allergies_str,
            chronic_str
        );

        let cabinet_drugs = if intake.current_medications_at_home.is_empty() {
            "Trusa de prim-ajutor la domiciliu: nicio substanta specificata de pacient.".to_string()
        } else {
            format!(
                "Medicamente disponibile la domiciliu (foto/ambalaje verificate): [{}].",
                intake.current_medications_at_home.join(", ")
            )
        };

        let assessment = format!(
            "Triaj preliminar AI: {urgency:?}. {cabinet_drugs} Se recomanda confirmarea compatibilitatii substantelor active si excluderea interactiunilor."
        );

        let recommendation = match urgency {
            UrgencyLevel::RedEmergency => {
                "URGENTA: Redirectionare prioritara catre 112 sau unitatea UPU / UPU-Vet cea mai apropiata.".to_string()
            }
            UrgencyLevel::OrangeUrgent => {
                "Validare schema OTC de prima intentie + recomandare programare medic specialist/de familie in 24h.".to_string()
            }
            _ => {
                "Confirmati posologia pentru medicamentele din trusa sau indicati produsul OTC optim disponibil la farmaciile partenere.".to_string()
            }
        };

        SbarBrief {
            situation,
            background,
            assessment,
            recommendation,
        }
    }
}

