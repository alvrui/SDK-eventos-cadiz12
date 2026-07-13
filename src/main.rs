use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use std::time::Instant;
use tiny_http::{Header, Method, Response, Server, StatusCode};

// Importar módulo AI
use crate::ai::schemas::SchemaManager;
use crate::ai::validator::AgentResponseValidator;
use crate::ai::logger::AgentInteractionLogger;
use crate::ai::feedback::FeedbackGenerator;
use crate::ai::prompts::PromptBuilder;

const INDEX_HTML: &str = include_str!("../ui/index.html");
const APP_JS: &str = include_str!("../ui/app.js");
const APP_CSS: &str = include_str!("../ui/styles.css");

const UI_HOST: &str = "0.0.0.0:7879";
const SECRETARIO_BASE_URL: &str = "http://127.0.0.1:8000";
const AI_LOG_DIR: &str = "logs/agents";

fn json_response(body: String) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string(body)
        .with_header(Header::from_bytes("Content-Type", "application/json; charset=UTF-8").unwrap())
}

fn text_response(
    status: u16,
    body: &str,
    content_type: &str,
) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string(body.to_string())
        .with_status_code(StatusCode(status))
        .with_header(Header::from_bytes("Content-Type", content_type).unwrap())
}

fn load_project_json() -> String {
    let path = Path::new("project_data/project.json");
    if path.exists() {
        let raw = fs::read_to_string(path).unwrap_or_else(|_| default_project_json());
        if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&raw) {
            migrate_project_shape(&mut v);
            return serde_json::to_string(&v).unwrap_or_else(|_| default_project_json());
        }
        raw
    } else {
        default_project_json()
    }
}

fn default_project_json() -> String {
    r#"{
  "projectMeta": {
    "id": "",
    "title": "Nuevo Proyecto Cadiz12",
    "summary": "",
    "format": "theatre_play",
    "worldContext": "",
    "allowedGenres": [],
    "toneProfile": { "seriousnessMin": 1, "seriousnessMax": 5, "darknessMin": 1, "darknessMax": 5 },
    "contentLimits": { "maxRating": "PG-13", "blockedSensitivityTags": [] },
    "productionConstraints": { "maxCastSize": null, "maxLocations": null, "budgetBand": "medium" },
    "notes": ""
  },
  "projectCharacters": [],
  "narratives": [
    {
      "id": "nar_001",
      "title": "Primera narrativa",
      "summary": "",
      "description": "",
      "act": "Act1",
      "tone": "Ambiguous",
      "historicalscope": "PlausibleInferred",
      "spaces": [],
      "factions": [],
      "stakes": [],
      "tags": [],
      "extendedNotes": "",
      "cast": {
        "rules": { "protagonistMode": "single", "maxAntagonists": 2, "maxSupporting": 4, "allowExternalCharacters": true, "allowRoleOverride": true },
        "entries": []
      }
    }
  ],
  "plots": [],
  "events": [],
  "review": { "summary": "", "issues": [] },
  "aiHistory": [],
  "settings": {
    "agents": {
      "project": "CoordinadorNarrativo",
      "narrative": "CoordinadorNarrativo",
      "characters": "CoordinadorNarrativo",
      "plots": "DiseñadorDeStoryElements",
      "events": "DiseñadorDeEventos",
      "validation": "RevisorNarrativo",
      "export": "EditorDeExportacion"
    }
  }
}"#
    .to_string()
}

fn migrate_project_shape(v: &mut serde_json::Value) {
    if let Some(legacy) = v.get("project").cloned() {
        if v.get("projectMeta").is_none() {
            let meta = serde_json::json!({
                "id": "",
                "title": legacy.get("title").cloned().unwrap_or(serde_json::json!("")),
                "summary": legacy.get("summary").cloned().unwrap_or(serde_json::json!("")),
                "format": "theatre_play",
                "worldContext": "",
                "allowedGenres": [],
                "toneProfile": { "seriousnessMin": 1, "seriousnessMax": 5, "darknessMin": 1, "darknessMax": 5 },
                "contentLimits": { "maxRating": "PG-13", "blockedSensitivityTags": [] },
                "productionConstraints": { "maxCastSize": null, "maxLocations": null, "budgetBand": "medium" },
                "notes": ""
            });
            v["projectMeta"] = meta;
        }
        if v.get("narratives").is_none() || v["narratives"].as_array().map(|a| a.is_empty()).unwrap_or(true) {
            let nar = serde_json::json!([{
                "id": "nar_001",
                "title": legacy.get("title").cloned().unwrap_or(serde_json::json!("Primera narrativa")),
                "summary": legacy.get("summary").cloned().unwrap_or(serde_json::json!("")),
                "description": "",
                "act": legacy.get("act").cloned().unwrap_or(serde_json::json!("Act1")),
                "tone": legacy.get("tone").cloned().unwrap_or(serde_json::json!("Ambiguous")),
                "historicalscope": legacy.get("historicalscope").cloned().unwrap_or(serde_json::json!("PlausibleInferred")),
                "spaces": legacy.get("spaces").cloned().unwrap_or(serde_json::json!([])),
                "factions": legacy.get("factions").cloned().unwrap_or(serde_json::json!([])),
                "stakes": legacy.get("stakes").cloned().unwrap_or(serde_json::json!([])),
                "tags": legacy.get("tags").cloned().unwrap_or(serde_json::json!([])),
                "extendedNotes": "",
                "cast": {
                    "rules": { "protagonistMode": "single", "maxAntagonists": 2, "maxSupporting": 4, "allowExternalCharacters": true, "allowRoleOverride": true },
                    "entries": []
                }
            }]);
            v["narratives"] = nar;
        }
        if v.get("plots").is_none() {
            if let Some(se) = v.get("story_elements").cloned() {
                // Migrar story_elements a plots
                v["plots"] = se;
                // También guardar en legacy por retrocompatibilidad
                v["legacyStoryElements"] = se;
            } else {
                v["plots"] = serde_json::json!([]);
            }
        }
        if v.get("projectCharacters").is_none() {
            v["projectCharacters"] = serde_json::json!([]);
        }
        
        // Migrar a narrative_elements (unificación de plots y story_elements)
        migrate_to_narrative_elements(v);
    }
}

/// Migrar proyectos antiguos a la estructura unificada narrative_elements
fn migrate_to_narrative_elements(v: &mut serde_json::Value) {
    // Si ya existe narrative_elements, no hacer nada
    if v.get("narrative_elements").is_some() {
        return;
    }
    
    // Crear estructura narrative_elements
    let mut narrative_elements = serde_json::json!({
        "themes": [],
        "protagonists": [],
        "antagonists": [],
        "secondaries": [],
        "scenarios": [],
        "procedures": [],
        "dramatic_resources": [],
        "social_pressures": []
    });
    
    // Migrar desde story_elements (si existe)
    if let Some(story_elements) = v.get("story_elements").and_then(|v| v.as_array()) {
        for element in story_elements {
            if let Some(element_type) = element.get("type").and_then(|t| t.as_str()) {
                match element_type {
                    "Theme" => {
                        if let Some(arr) = narrative_elements.get_mut("themes").and_then(|v| v.as_array_mut()) {
                            arr.push(element.clone());
                        }
                    }
                    "Protagonist" => {
                        if let Some(arr) = narrative_elements.get_mut("protagonists").and_then(|v| v.as_array_mut()) {
                            arr.push(element.clone());
                        }
                    }
                    "Antagonist" => {
                        if let Some(arr) = narrative_elements.get_mut("antagonists").and_then(|v| v.as_array_mut()) {
                            arr.push(element.clone());
                        }
                    }
                    "Secondary" => {
                        if let Some(arr) = narrative_elements.get_mut("secondaries").and_then(|v| v.as_array_mut()) {
                            arr.push(element.clone());
                        }
                    }
                    "Scenario" => {
                        if let Some(arr) = narrative_elements.get_mut("scenarios").and_then(|v| v.as_array_mut()) {
                            arr.push(element.clone());
                        }
                    }
                    "Procedure" => {
                        if let Some(arr) = narrative_elements.get_mut("procedures").and_then(|v| v.as_array_mut()) {
                            arr.push(element.clone());
                        }
                    }
                    "DramaticResource" => {
                        if let Some(arr) = narrative_elements.get_mut("dramatic_resources").and_then(|v| v.as_array_mut()) {
                            arr.push(element.clone());
                        }
                    }
                    "SocialPressure" => {
                        if let Some(arr) = narrative_elements.get_mut("social_pressures").and_then(|v| v.as_array_mut()) {
                            arr.push(element.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    // Migrar desde plots (si existe)
    if let Some(plots) = v.get("plots").and_then(|v| v.as_array()) {
        for plot in plots {
            // Asumir que plots son principalmente themes
            if let Some(arr) = narrative_elements.get_mut("themes").and_then(|v| v.as_array_mut()) {
                arr.push(plot.clone());
            }
        }
    }
    
    // Guardar narrative_elements en el proyecto
    v["narrative_elements"] = narrative_elements;
}

fn save_project_json(body: &str) -> Result<(), String> {
    let dir = Path::new("project_data");
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    fs::write(dir.join("project.json"), body).map_err(|e| e.to_string())
}

fn load_json_body(request: &mut tiny_http::Request) -> Value {
    let mut body = String::new();
    let _ = request.as_reader().read_to_string(&mut body);
    serde_json::from_str(&body).unwrap_or_else(|_| json!({}))
}

fn call_secretario(agent_name: &str, prompt: &str, force_new: bool) -> Result<Value, String> {
    let client = Client::new();

    let response = client
        .post(format!("{}/enviar_mensaje", SECRETARIO_BASE_URL))
        .json(&json!({
            "agente": agent_name,
            "mensaje": prompt,
            "force_new": force_new
        }))
        .send()
        .map_err(|e| format!("Error llamando a secretario.py: {}", e))?;

    let status = response.status();
    let payload: Value = response
        .json()
        .map_err(|e| format!("Respuesta no JSON desde secretario.py: {}", e))?;

    if !status.is_success() {
        return Err(format!("secretario.py devolvió HTTP {}: {}", status, payload));
    }

    Ok(payload)
}

fn fetch_secretario_agents() -> Result<Value, String> {
    let client = Client::new();
    let response = client
        .get(format!("{}/agentes", SECRETARIO_BASE_URL))
        .send()
        .map_err(|e| format!("Error consultando agentes en secretario.py: {}", e))?;

    let status = response.status();
    let payload: Value = response
        .json()
        .map_err(|e| format!("Respuesta inválida al pedir /agentes: {}", e))?;

    if !status.is_success() {
        return Err(format!("secretario.py devolvió HTTP {} en /agentes", status));
    }

    Ok(payload)
}

fn extract_inner_json(text: &str) -> Option<String> {
    // Busca el primer patrón típico de tu contrato
    let marker = r#""status": "success""#;
    let start = text.find('{')?;
    let mut best_start = None;

    // Buscamos el primer '{' antes del marker
    if let Some(m_pos) = text.find(marker) {
        let mut i = m_pos;
        while i > 0 {
            if &text[i..=i] == "{" {
                best_start = Some(i);
                break;
            }
            i -= 1;
        }
    }

    let s = best_start.unwrap_or(start);
    let bytes = text.as_bytes();
    let mut depth = 0;
    let mut end = None;

    for (idx, &b) in bytes.iter().enumerate().skip(s) {
        if b == b'{' {
            depth += 1;
        } else if b == b'}' {
            depth -= 1;
            if depth == 0 {
                end = Some(idx);
                break;
            }
        }
    }

    if let Some(e) = end {
        Some(text[s..=e].to_string())
    } else {
        None
    }
}

fn parse_agent_json_payload(
    payload: serde_json::Value,
    section: &str,
    action: &str,
    validator: &AgentResponseValidator,
    logger: &AgentInteractionLogger,
    feedback_generator: &FeedbackGenerator,
) -> Result<serde_json::Value, serde_json::Value> {
    // 1. Registrar la respuesta recibida (para debugging)
    logger.log_response("parse_agent_json_payload", &payload, true, None);
    
    // 1b. Validar estructura básica
    if let Err(errors) = validator.validate_basic_structure(section, &payload) {
        let feedback = feedback_generator.generate_validation_feedback(section, action, "secretario.py", &errors);
        logger.log_feedback(&feedback);
        return Err(json!({
            "status": "error",
            "section": section,
            "action": action,
            "message": "Estructura de respuesta inválida",
            "errors": errors,
            "raw_payload": payload
        }));
    }
    
    // 1c. Validar contra schema específico
    if let Err(errors) = validator.validate_response(section, &payload) {
        let feedback = feedback_generator.generate_schema_feedback(section, action, "secretario.py", &errors);
        logger.log_feedback(&feedback);
        return Err(json!({
            "status": "error",
            "section": section,
            "action": action,
            "message": "Respuesta no cumple con el schema",
            "schema_errors": errors,
            "raw_payload": payload
        }));
    }

    // 2. Validar status global de secretario
    if payload.get("status").and_then(|v| v.as_str()) != Some("success") {
        let message = payload.get("message").and_then(|v| v.as_str()).unwrap_or("secretario.py devolvió error");
        let feedback = feedback_generator.generate_generic_error_feedback(section, action, "secretario.py", message);
        logger.log_feedback(&feedback);
        return Err(json!({
            "status": "error",
            "section": section,
            "action": action,
            "message": message,
            "raw_payload": payload
        }));
    }

    // 2. Intentar extraer data directamente si existe
    if let Some(data) = payload.get("data").cloned() {
        if let Ok(parsed_data) = serde_json::from_value::<serde_json::Value>(data) {
            let mut enriched = parsed_data;
            if enriched.get("status").is_none() {
                enriched["status"] = json!("success");
            }
            if enriched.get("section").is_none() {
                enriched["section"] = json!(section);
            }
            if enriched.get("action").is_none() {
                enriched["action"] = json!(action);
            }
            if enriched.get("warnings").is_none() {
                enriched["warnings"] = json!([]);
            }
            return Ok(enriched);
        }
    }

    // 2b. Candidatos de texto: text y outputs[0]
    let mut candidates: Vec<String> = Vec::new();

    if let Some(t) = payload.get("text").and_then(|v| v.as_str()) {
        candidates.push(t.trim().to_string());
    }

    if let Some(arr) = payload.get("outputs").and_then(|v| v.as_array()) {
        if let Some(first) = arr.get(0).and_then(|v| v.as_str()) {
            candidates.push(first.trim().to_string());
        }
    }

    // 3. Intento 1: parsear candidato completo como JSON
    for cand in &candidates {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(cand) {
            // ¿Ya tiene la forma final?
            if parsed.get("data").is_some() {
                let mut enriched = parsed.clone();
                if enriched.get("status").is_none() {
                    enriched["status"] = json!("success");
                }
                if enriched.get("section").is_none() {
                    enriched["section"] = json!(section);
                }
                if enriched.get("action").is_none() {
                    enriched["action"] = json!(action);
                }
                if enriched.get("warnings").is_none() {
                    enriched["warnings"] = json!([]);
                }
                return Ok(enriched);
            }
        }
    }

    // 4. Intento 2: extraer inner JSON dentro del texto
    for cand in &candidates {
        if let Some(inner) = extract_inner_json(cand) {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&inner) {
                let mut enriched = parsed.clone();
                if enriched.get("status").is_none() {
                    enriched["status"] = json!("success");
                }
                if enriched.get("section").is_none() {
                    enriched["section"] = json!(section);
                }
                if enriched.get("action").is_none() {
                    enriched["action"] = json!(action);
                }
                if enriched.get("warnings").is_none() {
                    enriched["warnings"] = json!([]);
                }
                return Ok(enriched);
            }
        }
    }

    // 5. Si seguimos aquí, no hubo forma de sacar JSON; devolvemos error rico
    Err(json!({
        "status": "error",
        "section": section,
        "action": action,
        "message": "El texto del agente no contenía JSON utilizable",
        "raw_text": candidates,
        "raw_payload": payload
    }))
}

fn prompt_for_project_action(action: &str, project: &Value) -> String {
    format!(
        r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.
Estás trabajando para Cadiz12 en la sección de proyecto (lore y restricciones globales).
Acción solicitada: {action}
Contexto del proyecto:
{project_context}

Devuelve este formato exacto:
{{
  "status": "success",
  "section": "project",
  "action": "{action}",
  "data": {{
    "title": "string",
    "summary": "string",
    "worldContext": "string",
    "allowedGenres": [],
    "toneProfile": {{ "seriousnessMin": 1, "seriousnessMax": 5 }},
    "notes": "string"
  }},
  "warnings": []
}}"#,
        action = action,
        project_context = serde_json::to_string_pretty(project).unwrap_or_default()
    )
}

fn prompt_for_characters_action(action: &str, project: &Value) -> String {
    format!(
        r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.
Estás trabajando para Cadiz12 en la sección de personajes.
Acción solicitada: {action}
Contexto del proyecto:
{project_context}

Devuelve este formato exacto:
{{
  "status": "success",
  "section": "characters",
  "action": "{action}",
  "data": [
    {{
      "id": "string",
      "name": "string",
      "projectRoleType": "protagonist|antagonist|supporting|flex",
      "summary": "string",
      "description": "string",
      "extendedNotes": "string",
      "futureCompat": {{}},
      "narrativeProfile": {{
        "dramaticFunctions": [],
        "coreDrives": [],
        "traits": [],
        "toneFit": []
      }},
      "worldFit": {{
        "allowedPeriods": [],
        "preferredSettings": [],
        "genreAffinity": []
      }},
      "relationships": []
    }}
  ],
  "warnings": []
}}"#,
        action = action,
        project_context = serde_json::to_string_pretty(project).unwrap_or_default()
    )
}

fn prompt_for_plots_action(action: &str, project: &Value) -> String {
    format!(
        r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.
Estás trabajando para Cadiz12 en la sección de tramas.
Acción solicitada: {action}
Contexto del proyecto:
{project_context}

Devuelve este formato exacto:
{{
  "status": "success",
  "section": "plots",
  "action": "{action}",
  "data": [
    {{
      "id": "string",
      "title": "string",
      "narrativeId": "string",
      "summary": "string",
      "extendedNotes": "string",
      "futureCompat": {{}},
      "characterSelection": {{
        "protagonistCharacterId": null,
        "antagonistCharacterIds": [],
        "supportingCharacterIds": [],
        "entries": []
      }},
      "selection": {{
        "themeIds": [],
        "eventIds": [],
        "settingIds": [],
        "finaleId": null,
        "genreIds": []
      }}
    }}
  ],
  "warnings": []
}}"#,
        action = action,
        project_context = serde_json::to_string_pretty(project).unwrap_or_default()
    )
}

fn prompt_for_narrative_elements(action: &str, project: &Value) -> String {
    format!(
        r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de elementos narrativos unificados.
Acción solicitada: {action}

Contexto del proyecto:
{project_context}

Devuelve este formato exacto:
{{
  "status": "success",
  "section": "narrative_elements",
  "action": "{action}",
  "data": {{
    "themes": [
      {{
        "id": "string",
        "type": "Theme",
        "label": "string",
        "description": "string",
        "tone": "string",
        "historical_scope": "string",
        "time_window": ["string"],
        "act_bias": ["string"],
        "stakes_axis": ["string"],
        "faction_vectors": ["string"],
        "space_vectors": ["string"]
      }}
    ],
    "protagonists": [
      {{
        "id": "string",
        "type": "Protagonist",
        "label": "string",
        "description": "string",
        "eligible_profiles": ["string"],
        "eligible_positions": ["string"]
      }}
    ],
    "antagonists": [
      {{
        "id": "string",
        "type": "Antagonist",
        "label": "string",
        "description": "string"
      }}
    ],
    "secondaries": [
      {{
        "id": "string",
        "type": "Secondary",
        "label": "string",
        "description": "string"
      }}
    ],
    "scenarios": [
      {{
        "id": "string",
        "type": "Scenario",
        "label": "string",
        "description": "string"
      }}
    ],
    "procedures": [
      {{
        "id": "string",
        "type": "Procedure",
        "label": "string",
        "description": "string",
        "kind": "string"
      }}
    ],
    "dramatic_resources": [
      {{
        "id": "string",
        "type": "DramaticResource",
        "label": "string",
        "description": "string"
      }}
    ],
    "social_pressures": [
      {{
        "id": "string",
        "type": "SocialPressure",
        "label": "string",
        "description": "string"
      }}
    ]
  }},
  "warnings": []
}}

Reglas:
- Usa solo IDs que existan en los catálogos o marca como unresolved
- No inventes enums fuera del vocabulario dado
- Prioriza coherencia local y causalidad jugable
- Devuelve entre 3 y 8 elementos por tipo si la acción es de propuesta general"#,
        action = action,
        project_context = serde_json::to_string_pretty(project).unwrap_or_default()
    )
}

fn get_agent_name(project: &Value, key: &str, fallback: &str) -> String {
    // Intentar con la clave exacta primero
    if let Some(agent) = project
        .get("settings")
        .and_then(|v| v.get("agents"))
        .and_then(|v| v.get(key))
        .and_then(|v| v.as_str()) {
        return agent.to_string();
    }
    
    // Intentar con clave alternativa (storyelements -> story_elements)
    let alt_key = match key {
        "story_elements" => "storyelements",
        "storyelements" => "story_elements",
        _ => key,
    };
    
    if let Some(agent) = project
        .get("settings")
        .and_then(|v| v.get("agents"))
        .and_then(|v| v.get(alt_key))
        .and_then(|v| v.as_str()) {
        return agent.to_string();
    }
    
    fallback.to_string()
}

fn prompt_for_narrative(action: &str, project: &Value) -> String {
    format!(
        r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de narrativa.
Acción solicitada: {action}

Contexto del proyecto:
{project_context}

Devuelve este formato exacto:
{{
  "status": "success",
  "section": "narrative",
  "action": "{action}",
  "data": {{
    "title": "string",
    "summary": "string",
    "act": "string",
    "tone": "string",
    "historical_scope": "string",
    "spaces": ["string"],
    "factions": ["string"],
    "stakes": ["string"],
    "tags": ["string"]
  }},
  "warnings": []
}}

Si falta contexto, usa propuestas plausibles pero útiles. No devuelvas texto fuera del JSON."#,
        action = action,
        project_context = serde_json::to_string_pretty(project).unwrap_or_else(|_| "{}".to_string())
    )
}

fn prompt_for_story_elements(action: &str, project: &Value) -> String {
    format!(
        r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de story elements.
Acción solicitada: {action}

Contexto del proyecto:
{project_context}

Devuelve este formato exacto:
{{
  "status": "success",
  "section": "story_elements",
  "action": "{action}",
  "data": [
    {{
      "id": "string",
      "type": "Theme",
      "label": "string",
      "description": "string",
      "tone": "string"
    }}
  ],
  "warnings": []
}}

En type puedes usar valores como Theme, Scenario, Procedure, Antagonist, Secondary, Protagonist, DramaticResource.
Devuelve entre 3 y 8 elementos si la acción es de propuesta general."#,
        action = action,
        project_context = serde_json::to_string_pretty(project).unwrap_or_else(|_| "{}".to_string())
    )
}

fn prompt_for_event(action: &str, project: &Value) -> String {
    format!(
        r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de eventos.
Acción solicitada: {action}

Contexto del proyecto:
{project_context}

Devuelve este formato exacto:
{{
  "status": "success",
  "section": "events",
  "action": "{action}",
  "data": {{
    "id": "string",
    "label": "string",
    "title": "string",
    "story_element_id": "string",
    "body_text": "string",
    "flavor_text": "string",
    "choices": [
      {{
        "id": "string",
        "label": "string",
        "outcome": "string"
      }}
    ],
    "consequences": ["string"],
    "assets": ["string"]
  }},
  "warnings": []
}}

Si la acción es proponer textos o decisiones, sigue devolviendo un evento completo para que la UI lo pueda aplicar sin lógica extra."#,
        action = action,
        project_context = serde_json::to_string_pretty(project).unwrap_or_else(|_| "{}".to_string())
    )
}

fn prompt_for_review(action: &str, project: &Value) -> String {
    format!(
        r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en revisión editorial global.
Acción solicitada: {action}

Contexto del proyecto:
{project_context}

Devuelve este formato exacto:
{{
  "status": "success",
  "section": "validation",
  "action": "{action}",
  "data": {{
    "summary": "string",
    "issues": ["string"]
  }},
  "warnings": []
}}"#,
        action = action,
        project_context = serde_json::to_string_pretty(project).unwrap_or_else(|_| "{}".to_string())
    )
}

fn handle_ai_request(
    section: &str,
    default_agent: &str,
    action: &str,
    project: &Value,
    prompt_builder: fn(&str, &Value) -> String,
    validator: &AgentResponseValidator,
    logger: &AgentInteractionLogger,
    feedback_generator: &FeedbackGenerator,
) -> String {
    let configured_agent = get_agent_name(project, section, default_agent);
    let prompt = prompt_builder(action, project);
    
    // Registrar la solicitud
    let start_time = Instant::now();
    let log_filename = logger.log_request(section, action, &configured_agent, &prompt);

    match call_secretario(&configured_agent, &prompt, false) {
        Ok(payload) => {
            // Registrar la respuesta
            let duration = start_time.elapsed().as_millis() as u64;
            logger.log_response(&log_filename.unwrap_or_default(), &payload, true, Some(duration));
            
            match parse_agent_json_payload(payload, section, action, validator, logger, feedback_generator) {
                Ok(parsed) => {
                    let enriched = json!({
                        "status": parsed.get("status").cloned().unwrap_or(json!("success")),
                        "section": parsed.get("section").cloned().unwrap_or(json!(section)),
                        "action": parsed.get("action").cloned().unwrap_or(json!(action)),
                        "data": parsed.get("data").cloned().unwrap_or(json!({})),
                        "warnings": parsed.get("warnings").cloned().unwrap_or(json!([])),
                        "meta": {
                            "agent": configured_agent,
                            "source": "secretario.py",
                            "processing_time_ms": duration
                        }
                    });
                    enriched.to_string()
                }
                Err(err_payload) => {
                    // Registrar error
                    logger.log_error(section, action, &configured_agent, &err_payload.to_string(), Some(&prompt));
                    err_payload.to_string()
                }
            }
        }
        Err(err) => {
            // Registrar error
            logger.log_error(section, action, &configured_agent, &err, Some(&prompt));
            json!({
                "status": "error",
                "section": section,
                "action": action,
                "message": err,
                "meta": {
                    "agent": configured_agent,
                    "source": "secretario.py"
                }
            })
            .to_string()
        }
    }
}

fn main() {
    // Inicializar componentes de AI
    let schema_manager = SchemaManager::new();
    let response_validator = AgentResponseValidator::new();
    let ai_logger = AgentInteractionLogger::new(AI_LOG_DIR);
    let feedback_generator = FeedbackGenerator::new();
    let prompt_builder = PromptBuilder::new();
    
    // Verificar que el directorio de logs existe
    std::fs::create_dir_all(AI_LOG_DIR).ok();
    
    let server = Server::http(UI_HOST).expect("No se pudo iniciar el servidor");
    println!("Cadiz12 Narrative UI en http://{}", UI_HOST);

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().clone();

        match (method, url.as_str()) {
            (Method::Get, "/") => {
                let response = text_response(200, INDEX_HTML, "text/html; charset=UTF-8");
                let _ = request.respond(response);
            }
            (Method::Get, "/app.js") => {
                let response = text_response(200, APP_JS, "application/javascript; charset=UTF-8");
                let _ = request.respond(response);
            }
            (Method::Get, "/styles.css") => {
                let response = text_response(200, APP_CSS, "text/css; charset=UTF-8");
                let _ = request.respond(response);
            }
            (Method::Get, "/api/project") => {
                let response = json_response(load_project_json());
                let _ = request.respond(response);
            }
            (Method::Post, "/api/project") => {
                let mut body = String::new();
                let _ = request.as_reader().read_to_string(&mut body);

                let response = match save_project_json(&body) {
                    Ok(_) => json_response(r#"{"status":"success"}"#.to_string()),
                    Err(err) => text_response(
                        500,
                        &format!(r#"{{"status":"error","message":"{}"}}"#, err),
                        "application/json; charset=UTF-8",
                    ),
                };
                let _ = request.respond(response);
            }
            (Method::Get, "/api/agents") => {
                let response_body = match fetch_secretario_agents() {
                    Ok(payload) => json!({
                        "status": "success",
                        "data": payload
                    })
                    .to_string(),
                    Err(err) => json!({
                        "status": "error",
                        "message": err,
                        "data": []
                    })
                    .to_string(),
                };
                let _ = request.respond(json_response(response_body));
            }
            (Method::Post, "/api/ai/narrative") => {
                let incoming = load_json_body(&mut request);
                let action = incoming
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("generate");
                let project = incoming.get("project").cloned().unwrap_or_else(|| json!({}));

                let response_body = handle_ai_request(
                    "narrative",
                    "CoordinadorNarrativo",
                    action,
                    &project,
                    prompt_for_narrative,
                    &response_validator,
                    &ai_logger,
                    &feedback_generator,
                );

                let _ = request.respond(json_response(response_body));
            }
            (Method::Post, "/api/ai/story-elements") => {
                let incoming = load_json_body(&mut request);
                let action = incoming
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("propose");
                let project = incoming.get("project").cloned().unwrap_or_else(|| json!({}));

                let response_body = handle_ai_request(
                    "story_elements",
                    "DiseñadorDeStoryElements",
                    action,
                    &project,
                    prompt_for_story_elements,
                    &response_validator,
                    &ai_logger,
                    &feedback_generator,
                );

                let _ = request.respond(json_response(response_body));
            }
            (Method::Post, "/api/ai/event") => {
                let incoming = load_json_body(&mut request);
                let action = incoming
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("generate_from_story_element");
                let project = incoming.get("project").cloned().unwrap_or_else(|| json!({}));

                let response_body = handle_ai_request(
                    "events",
                    "DiseñadorDeEventos",
                    action,
                    &project,
                    prompt_for_event,
                    &response_validator,
                    &ai_logger,
                    &feedback_generator,
                );

                let _ = request.respond(json_response(response_body));
            }
            (Method::Post, "/api/ai/review") => {
                let incoming = load_json_body(&mut request);
                let action = incoming
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("review");
                let project = incoming.get("project").cloned().unwrap_or_else(|| json!({}));

                let response_body = handle_ai_request(
                    "validation",
                    "RevisorNarrativo",
                    action,
                    &project,
                    prompt_for_review,
                    &response_validator,
                    &ai_logger,
                    &feedback_generator,
                );

                let _ = request.respond(json_response(response_body));
            }
            (Method::Post, "/api/ai/project") => {
                let incoming = load_json_body(&mut request);
                let action = incoming
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("generate");
                let project = incoming.get("project").cloned().unwrap_or_else(|| json!({}));
                let response_body = handle_ai_request(
                    "project",
                    "CoordinadorNarrativo",
                    action,
                    &project,
                    prompt_for_project_action,
                    &response_validator,
                    &ai_logger,
                    &feedback_generator,
                );
                let _ = request.respond(json_response(response_body));
            }
            (Method::Post, "/api/ai/characters") => {
                let incoming = load_json_body(&mut request);
                let action = incoming
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("propose");
                let project = incoming.get("project").cloned().unwrap_or_else(|| json!({}));
                let response_body = handle_ai_request(
                    "characters",
                    "CoordinadorNarrativo",
                    action,
                    &project,
                    prompt_for_characters_action,
                    &response_validator,
                    &ai_logger,
                    &feedback_generator,
                );
                let _ = request.respond(json_response(response_body));
            }
            (Method::Post, "/api/ai/plots") => {
                let incoming = load_json_body(&mut request);
                let action = incoming
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("propose");
                let project = incoming.get("project").cloned().unwrap_or_else(|| json!({}));
                let response_body = handle_ai_request(
                    "plots",
                    "DiseñadorDeStoryElements",
                    action,
                    &project,
                    prompt_for_plots_action,
                    &response_validator,
                    &ai_logger,
                    &feedback_generator,
                );
                let _ = request.respond(json_response(response_body));
            }
            (Method::Post, "/api/ai/narrative-elements") => {
                let incoming = load_json_body(&mut request);
                let action = incoming
                    .get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("generate");
                let project = incoming.get("project").cloned().unwrap_or_else(|| json!({}));
                let response_body = handle_ai_request(
                    "narrative_elements",
                    "CoordinadorNarrativo",
                    action,
                    &project,
                    prompt_for_narrative_elements,
                    &response_validator,
                    &ai_logger,
                    &feedback_generator,
                );
                let _ = request.respond(json_response(response_body));
            }
            _ => {
                let response = text_response(404, "Not Found", "text/plain; charset=UTF-8");
                let _ = request.respond(response);
            }
        }
    }
}
