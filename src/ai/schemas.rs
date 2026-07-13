//! Schemas JSON para validación de respuestas de agentes
//!
//! Este módulo proporciona schemas JSON en formato string para validación manual
//! sin depender de la crate jsonschema (que tiene muchas dependencias)

use serde_json::{Value, Map};
use std::collections::HashSet;

/// Schema para respuesta de generación de narrativa
pub const NARRATIVE_SCHEMA: &str = r#"{
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {"type": "string", "enum": ["success", "error"]},
        "section": {"type": "string", "enum": ["project", "narrative", "narrative_elements", "plots", "story_elements", "events", "characters", "validation", "export"]},
        "action": {"type": "string"},
        "data": {
            "type": "object",
            "required": ["title", "summary"],
            "properties": {
                "title": {"type": "string", "minLength": 1},
                "summary": {"type": "string"},
                "description": {"type": "string"},
                "act": {"type": "string", "enum": ["Act1", "Act2", "Act3", "Act4"]},
                "tone": {"type": "string", "enum": ["Festive", "Satirical", "Anxious", "Solemn", "Intimate", "Conspiratorial", "Patriotic", "Sordid", "Tragic", "Ambiguous", "Tense", "Polemical", "Funereal", "Resilient", "Combative", "Compassionate"]},
                "historical_scope": {"type": "string", "enum": ["StrictHistorical", "PlausibleDocumented", "PlausibleInferred", "ExceptionalButVerisimilar", "Discarded"]},
                "spaces": {"type": "array", "items": {"type": "string"}},
                "factions": {"type": "array", "items": {"type": "string"}},
                "stakes": {"type": "array", "items": {"type": "string"}},
                "tags": {"type": "array", "items": {"type": "string"}},
                "extendedNotes": {"type": "string"}
            }
        },
        "warnings": {"type": "array", "items": {"type": "string"}}
    }
}"#;

/// Schema para respuesta de narrative_elements
pub const NARRATIVE_ELEMENTS_SCHEMA: &str = r#"{
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {"type": "string", "enum": ["success", "error"]},
        "section": {"type": "string", "enum": ["narrative_elements"]},
        "action": {"type": "string"},
        "data": {
            "type": "object",
            "properties": {
                "themes": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "type", "label", "description"],
                        "properties": {
                            "id": {"type": "string", "minLength": 1},
                            "type": {"type": "string", "enum": ["Theme"]},
                            "label": {"type": "string", "minLength": 1},
                            "description": {"type": "string"},
                            "tone": {"type": "string", "enum": ["Festive", "Satirical", "Anxious", "Solemn", "Intimate", "Conspiratorial", "Patriotic", "Sordid", "Tragic", "Ambiguous", "Tense", "Polemical", "Funereal", "Resilient", "Combative", "Compassionate"]},
                            "historical_scope": {"type": "string", "enum": ["StrictHistorical", "PlausibleDocumented", "PlausibleInferred", "ExceptionalButVerisimilar", "Discarded"]},
                            "time_window": {"type": "array", "items": {"type": "string", "enum": ["Y1805_1808", "Y1809", "Y1810", "Y1811", "Y1812", "Y1813", "Y1814", "Y1815_1816"]}},
                            "act_bias": {"type": "array", "items": {"type": "string", "enum": ["Act1", "Act2", "Act3", "Act4"]}},
                            "stakes_axis": {"type": "array", "items": {"type": "string", "enum": ["Personal", "Political", "Urban", "Institutional", "Imperial", "Moral", "Economic", "Religious", "Military", "Media"]}},
                            "faction_vectors": {"type": "array", "items": {"type": "string"}},
                            "space_vectors": {"type": "array", "items": {"type": "string"}},
                            "compatibility_tags": {"type": "array", "items": {"type": "string"}},
                            "blocking_tags": {"type": "array", "items": {"type": "string"}},
                            "unlock_tags": {"type": "array", "items": {"type": "string"}},
                            "generated_tags": {"type": "array", "items": {"type": "string"}}
                        }
                    }
                },
                "protagonists": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "type", "label", "description"],
                        "properties": {
                            "id": {"type": "string", "minLength": 1},
                            "type": {"type": "string", "enum": ["Protagonist"]},
                            "label": {"type": "string", "minLength": 1},
                            "description": {"type": "string"},
                            "eligible_profiles": {"type": "array", "items": {"type": "string"}},
                            "eligible_positions": {"type": "array", "items": {"type": "string"}}
                        }
                    }
                },
                "antagonists": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "type", "label", "description"],
                        "properties": {
                            "id": {"type": "string", "minLength": 1},
                            "type": {"type": "string", "enum": ["Antagonist"]},
                            "label": {"type": "string", "minLength": 1},
                            "description": {"type": "string"}
                        }
                    }
                },
                "secondaries": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "type", "label", "description"],
                        "properties": {
                            "id": {"type": "string", "minLength": 1},
                            "type": {"type": "string", "enum": ["Secondary"]},
                            "label": {"type": "string", "minLength": 1},
                            "description": {"type": "string"}
                        }
                    }
                },
                "scenarios": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "type", "label", "description"],
                        "properties": {
                            "id": {"type": "string", "minLength": 1},
                            "type": {"type": "string", "enum": ["Scenario"]},
                            "label": {"type": "string", "minLength": 1},
                            "description": {"type": "string"}
                        }
                    }
                },
                "procedures": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "type", "label", "description", "kind"],
                        "properties": {
                            "id": {"type": "string", "minLength": 1},
                            "type": {"type": "string", "enum": ["Procedure"]},
                            "label": {"type": "string", "minLength": 1},
                            "description": {"type": "string"},
                            "kind": {"type": "string", "enum": ["DebatePlenary", "TechnicalCommission", "EmergencySession", "HonorSession", "DecreeVote", "PriorPositioningCall", "PrivateNegotiation", "PetitionSubmission", "DocumentReading", "PressPublication", "PressDenunciation", "StrategicLeak", "StrategicDelay", "AmendmentProposal", "SecretVoteRequest", "SafeConductProcessing", "HousingAssignment", "HealthDeclaration", "NeighborhoodRelief", "SignatureCirculation"]}
                        }
                    }
                },
                "dramatic_resources": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "type", "label", "description"],
                        "properties": {
                            "id": {"type": "string", "minLength": 1},
                            "type": {"type": "string", "enum": ["DramaticResource"]},
                            "label": {"type": "string", "minLength": 1},
                            "description": {"type": "string"}
                        }
                    }
                },
                "social_pressures": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "type", "label", "description"],
                        "properties": {
                            "id": {"type": "string", "minLength": 1},
                            "type": {"type": "string", "enum": ["SocialPressure"]},
                            "label": {"type": "string", "minLength": 1},
                            "description": {"type": "string"}
                        }
                    }
                }
            }
        },
        "warnings": {"type": "array", "items": {"type": "string"}}
    }
}"#;

/// Schema para respuesta de evento
pub const EVENT_SCHEMA: &str = r#"{
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {"type": "string", "enum": ["success", "error"]},
        "section": {"type": "string", "enum": ["events"]},
        "action": {"type": "string"},
        "data": {
            "type": "object",
            "required": ["id", "label", "title", "story_element_id", "body_text"],
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "label": {"type": "string", "minLength": 1},
                "title": {"type": "string", "minLength": 1},
                "story_element_id": {"type": "string", "minLength": 1},
                "body_text": {"type": "string"},
                "flavor_text": {"type": "string"},
                "choices": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["id", "label", "outcome"],
                        "properties": {
                            "id": {"type": "string", "minLength": 1},
                            "label": {"type": "string", "minLength": 1},
                            "outcome": {"type": "string"}
                        }
                    }
                },
                "consequences": {"type": "array", "items": {"type": "string"}},
                "assets": {"type": "array", "items": {"type": "string"}}
            }
        },
        "warnings": {"type": "array", "items": {"type": "string"}}
    }
}"#;

/// Schema para respuesta de validación
pub const VALIDATION_SCHEMA: &str = r#"{
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {"type": "string", "enum": ["success", "error"]},
        "section": {"type": "string", "enum": ["validation"]},
        "action": {"type": "string"},
        "data": {
            "type": "object",
            "required": ["summary", "issues"],
            "properties": {
                "summary": {"type": "string"},
                "issues": {"type": "array", "items": {"type": "string"}}
            }
        },
        "warnings": {"type": "array", "items": {"type": "string"}}
    }
}"#;

/// Schema para respuesta de personajes
pub const CHARACTERS_SCHEMA: &str = r#"{
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {"type": "string", "enum": ["success", "error"]},
        "section": {"type": "string", "enum": ["characters"]},
        "action": {"type": "string"},
        "data": {
            "type": "array",
            "items": {
                "type": "object",
                "required": ["id", "name", "projectRoleType"],
                "properties": {
                    "id": {"type": "string", "minLength": 1},
                    "name": {"type": "string", "minLength": 1},
                    "projectRoleType": {"type": "string", "enum": ["protagonist", "antagonist", "supporting", "flex"]},
                    "summary": {"type": "string"},
                    "description": {"type": "string"},
                    "extendedNotes": {"type": "string"}
                }
            }
        },
        "warnings": {"type": "array", "items": {"type": "string"}}
    }
}"#;

/// Schema para respuesta de proyecto
pub const PROJECT_SCHEMA: &str = r#"{
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {"type": "string", "enum": ["success", "error"]},
        "section": {"type": "string", "enum": ["project"]},
        "action": {"type": "string"},
        "data": {
            "type": "object",
            "required": ["title", "summary"],
            "properties": {
                "title": {"type": "string", "minLength": 1},
                "summary": {"type": "string"},
                "worldContext": {"type": "string"},
                "allowedGenres": {"type": "array", "items": {"type": "string"}},
                "toneProfile": {
                    "type": "object",
                    "properties": {
                        "seriousnessMin": {"type": "integer", "minimum": 1, "maximum": 5},
                        "seriousnessMax": {"type": "integer", "minimum": 1, "maximum": 5},
                        "darknessMin": {"type": "integer", "minimum": 1, "maximum": 5},
                        "darknessMax": {"type": "integer", "minimum": 1, "maximum": 5}
                    }
                },
                "contentLimits": {
                    "type": "object",
                    "properties": {
                        "maxRating": {"type": "string"},
                        "blockedSensitivityTags": {"type": "array", "items": {"type": "string"}}
                    }
                },
                "productionConstraints": {
                    "type": "object",
                    "properties": {
                        "maxCastSize": {"type": ["integer", "null"]},
                        "maxLocations": {"type": ["integer", "null"]},
                        "budgetBand": {"type": "string"}
                    }
                },
                "notes": {"type": "string"}
            }
        },
        "warnings": {"type": "array", "items": {"type": "string"}}
    }
}"#;

/// Schema para respuesta de exportación
pub const EXPORT_SCHEMA: &str = r#"{
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {"type": "string", "enum": ["success", "error"]},
        "section": {"type": "string", "enum": ["export"]},
        "action": {"type": "string"},
        "data": {
            "type": "object",
            "required": ["summary", "checklist"],
            "properties": {
                "summary": {"type": "string"},
                "checklist": {"type": "array", "items": {"type": "string"}},
                "issues": {"type": "array", "items": {"type": "string"}}
            }
        },
        "warnings": {"type": "array", "items": {"type": "string"}}
    }
}"#;

/// Validador de schemas manual (sin dependencia de jsonschema)
#[derive(Debug, Clone)]
pub struct SchemaValidator;

impl SchemaValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validar respuesta contra schema (versión simplificada)
    pub fn validate_response(
        &self,
        section: &str,
        response: &Value,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Validar estructura básica
        self.validate_basic_structure(section, response, &mut errors)?;
        
        // Validar según la sección
        match section {
            "narrative" => self.validate_narrative(response, &mut errors),
            "narrative_elements" => self.validate_narrative_elements(response, &mut errors),
            "events" => self.validate_event(response, &mut errors),
            "validation" => self.validate_validation(response, &mut errors),
            "characters" => self.validate_characters(response, &mut errors),
            "project" => self.validate_project(response, &mut errors),
            "export" => self.validate_export(response, &mut errors),
            _ => Ok(()), // Secciones desconocidas no se validan
        }?;
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validar estructura básica
    fn validate_basic_structure(
        &self,
        expected_section: &str,
        response: &Value,
        errors: &mut Vec<String>,
    ) -> Result<(), Vec<String>> {
        // Validar que es un objeto
        if !response.is_object() {
            errors.push("Response must be a JSON object".to_string());
            return Err(errors.clone());
        }
        
        let obj = response.as_object().unwrap();
        
        // Validar status
        if let Some(status) = obj.get("status") {
            if !status.is_string() {
                errors.push("Field 'status' must be a string".to_string());
            } else if let Some(status_str) = status.as_str() {
                if status_str != "success" && status_str != "error" {
                    errors.push(format!(
                        "Field 'status' must be 'success' or 'error', got: {}",
                        status_str
                    ));
                }
            }
        } else {
            errors.push("Missing required field: 'status'".to_string());
        }
        
        // Validar section
        if let Some(section) = obj.get("section") {
            if !section.is_string() {
                errors.push("Field 'section' must be a string".to_string());
            } else if section.as_str() != Some(expected_section) {
                errors.push(format!(
                    "Expected section '{}', got: {}",
                    expected_section,
                    section.as_str().unwrap_or("")
                ));
            }
        } else {
            errors.push("Missing required field: 'section'".to_string());
        }
        
        // Validar action
        if obj.get("action").is_none() {
            errors.push("Missing required field: 'action'".to_string());
        }
        
        // Validar data
        if obj.get("data").is_none() {
            errors.push("Missing required field: 'data'".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.clone())
        }
    }

    /// Validar respuesta de narrativa
    fn validate_narrative(&self, response: &Value, errors: &mut Vec<String>) -> Result<(), Vec<String>> {
        if let Some(data) = response.get("data").and_then(|d| d.as_object()) {
            // Validar campos requeridos
            for field in ["title", "summary"] {
                if data.get(field).is_none() {
                    errors.push(format!("Missing required field in data: {}", field));
                }
            }
            
            // Validar campos opcionales
            self.validate_optional_fields(data, &["description", "act", "tone", "historical_scope"], errors);
            
            // Validar enums
            self.validate_enum(data, "act", &["Act1", "Act2", "Act3", "Act4"], errors);
            self.validate_enum(data, "tone", &["Festive", "Satirical", "Anxious", "Solemn", "Intimate", "Conspiratorial", "Patriotic", "Sordid", "Tragic", "Ambiguous", "Tense", "Polemical", "Funereal", "Resilient", "Combative", "Compassionate"], errors);
            self.validate_enum(data, "historical_scope", &["StrictHistorical", "PlausibleDocumented", "PlausibleInferred", "ExceptionalButVerisimilar", "Discarded"], errors);
        }
        
        Ok(())
    }

    /// Validar respuesta de narrative_elements
    fn validate_narrative_elements(&self, response: &Value, errors: &mut Vec<String>) -> Result<(), Vec<String>> {
        if let Some(data) = response.get("data").and_then(|d| d.as_object()) {
            // Validar que al menos un tipo de elemento existe
            let has_elements = ["themes", "protagonists", "antagonists", "secondaries", 
                               "scenarios", "procedures", "dramatic_resources", "social_pressures"]
                .iter()
                .any(|&field| data.get(field).and_then(|v| v.as_array()).map_or(false, |a| !a.is_empty()));
            
            if !has_elements {
                errors.push("At least one element type must be present in data".to_string());
            }
            
            // Validar cada tipo de elemento
            self.validate_element_array(data, "themes", errors);
            self.validate_element_array(data, "protagonists", errors);
            self.validate_element_array(data, "antagonists", errors);
            self.validate_element_array(data, "secondaries", errors);
            self.validate_element_array(data, "scenarios", errors);
            self.validate_element_array(data, "procedures", errors);
            self.validate_element_array(data, "dramatic_resources", errors);
            self.validate_element_array(data, "social_pressures", errors);
        }
        
        Ok(())
    }

    /// Validar array de elementos
    fn validate_element_array(&self, data: &Map<String, Value>, field: &str, errors: &mut Vec<String>) {
        if let Some(arr) = data.get(field).and_then(|v| v.as_array()) {
            for (idx, element) in arr.iter().enumerate() {
                if let Some(obj) = element.as_object() {
                    // Validar campos requeridos
                    for required_field in ["id", "type", "label", "description"] {
                        if obj.get(required_field).is_none() {
                            errors.push(format!(
                                "Missing required field '{}' in {}[{}]",
                                required_field, field, idx
                            ));
                        }
                    }
                    
                    // Validar tipo
                    if let Some(element_type) = obj.get("type").and_then(|v| v.as_str()) {
                        let expected_type = match field {
                            "themes" => "Theme",
                            "protagonists" => "Protagonist",
                            "antagonists" => "Antagonist",
                            "secondaries" => "Secondary",
                            "scenarios" => "Scenario",
                            "procedures" => "Procedure",
                            "dramatic_resources" => "DramaticResource",
                            "social_pressures" => "SocialPressure",
                            _ => "",
                        };
                        
                        if !expected_type.is_empty() && element_type != expected_type {
                            errors.push(format!(
                                "Invalid type '{}' in {}[{}], expected '{}'",
                                element_type, field, idx, expected_type
                            ));
                        }
                    }
                    
                    // Validar enums específicos
                    if field == "procedures" {
                        if let Some(kind) = obj.get("kind").and_then(|v| v.as_str()) {
                            let valid_kinds = ["DebatePlenary", "TechnicalCommission", "EmergencySession", 
                                             "HonorSession", "DecreeVote", "PriorPositioningCall", "PrivateNegotiation",
                                             "PetitionSubmission", "DocumentReading", "PressPublication", "PressDenunciation",
                                             "StrategicLeak", "StrategicDelay", "AmendmentProposal", "SecretVoteRequest",
                                             "SafeConductProcessing", "HousingAssignment", "HealthDeclaration", 
                                             "NeighborhoodRelief", "SignatureCirculation"];
                            if !valid_kinds.contains(&kind) {
                                errors.push(format!("Invalid kind '{}' in procedures[{}]", kind, idx));
                            }
                        }
                    }
                }
            }
        }
    }

    /// Validar respuesta de evento
    fn validate_event(&self, response: &Value, errors: &mut Vec<String>) -> Result<(), Vec<String>> {
        if let Some(data) = response.get("data").and_then(|d| d.as_object()) {
            // Validar campos requeridos
            for field in ["id", "label", "title", "story_element_id", "body_text"] {
                if data.get(field).is_none() {
                    errors.push(format!("Missing required field in data: {}", field));
                }
            }
            
            // Validar choices
            if let Some(choices) = data.get("choices").and_then(|v| v.as_array()) {
                for (idx, choice) in choices.iter().enumerate() {
                    if let Some(obj) = choice.as_object() {
                        for field in ["id", "label", "outcome"] {
                            if obj.get(field).is_none() {
                                errors.push(format!("Missing required field '{}' in choices[{}]", field, idx));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Validar respuesta de validación
    fn validate_validation(&self, response: &Value, errors: &mut Vec<String>) -> Result<(), Vec<String>> {
        if let Some(data) = response.get("data").and_then(|d| d.as_object()) {
            for field in ["summary", "issues"] {
                if data.get(field).is_none() {
                    errors.push(format!("Missing required field in data: {}", field));
                }
            }
        }
        
        Ok(())
    }

    /// Validar respuesta de personajes
    fn validate_characters(&self, response: &Value, errors: &mut Vec<String>) -> Result<(), Vec<String>> {
        if let Some(data) = response.get("data").and_then(|v| v.as_array()) {
            for (idx, character) in data.iter().enumerate() {
                if let Some(obj) = character.as_object() {
                    for field in ["id", "name", "projectRoleType"] {
                        if obj.get(field).is_none() {
                            errors.push(format!("Missing required field '{}' in data[{}]", field, idx));
                        }
                    }
                    
                    // Validar enum projectRoleType
                    if let Some(role) = obj.get("projectRoleType").and_then(|v| v.as_str()) {
                        if !["protagonist", "antagonist", "supporting", "flex"].contains(&role) {
                            errors.push(format!("Invalid projectRoleType '{}' in data[{}]", role, idx));
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Validar respuesta de proyecto
    fn validate_project(&self, response: &Value, errors: &mut Vec<String>) -> Result<(), Vec<String>> {
        if let Some(data) = response.get("data").and_then(|d| d.as_object()) {
            for field in ["title", "summary"] {
                if data.get(field).is_none() {
                    errors.push(format!("Missing required field in data: {}", field));
                }
            }
        }
        
        Ok(())
    }

    /// Validar respuesta de exportación
    fn validate_export(&self, response: &Value, errors: &mut Vec<String>) -> Result<(), Vec<String>> {
        if let Some(data) = response.get("data").and_then(|d| d.as_object()) {
            for field in ["summary", "checklist"] {
                if data.get(field).is_none() {
                    errors.push(format!("Missing required field in data: {}", field));
                }
            }
        }
        
        Ok(())
    }

    /// Validar campos opcionales
    fn validate_optional_fields(&self, data: &Map<String, Value>, fields: &[&str], errors: &mut Vec<String>) {
        for field in fields {
            if let Some(value) = data.get(*field) {
                if value.is_null() {
                    errors.push(format!("Field '{}' cannot be null", field));
                }
            }
        }
    }

    /// Validar enum
    fn validate_enum(&self, data: &Map<String, Value>, field: &str, valid_values: &[&str], errors: &mut Vec<String>) {
        if let Some(value) = data.get(field).and_then(|v| v.as_str()) {
            if !valid_values.contains(&value) {
                errors.push(format!(
                    "Invalid value '{}' for field '{}'. Valid values: {}",
                    value,
                    field,
                    valid_values.join(", ")
                ));
            }
        }
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Schema Manager - Gestiona todos los schemas
#[derive(Debug, Clone)]
pub struct SchemaManager {
    validator: SchemaValidator,
}

impl SchemaManager {
    pub fn new() -> Self {
        Self {
            validator: SchemaValidator::new(),
        }
    }

    /// Obtener schema por sección (como string)
    pub fn get_schema_str(&self, section: &str) -> Option<&'static str> {
        match section {
            "narrative" => Some(NARRATIVE_SCHEMA),
            "narrative_elements" => Some(NARRATIVE_ELEMENTS_SCHEMA),
            "event" | "events" => Some(EVENT_SCHEMA),
            "validation" => Some(VALIDATION_SCHEMA),
            "characters" => Some(CHARACTERS_SCHEMA),
            "project" => Some(PROJECT_SCHEMA),
            "export" => Some(EXPORT_SCHEMA),
            _ => None,
        }
    }

    /// Validar respuesta contra schema
    pub fn validate_response(&self, section: &str, response: &Value) -> Result<(), Vec<String>> {
        self.validator.validate_response(section, response)
    }

    /// Obtener todas las secciones disponibles
    pub fn available_sections(&self) -> Vec<&'static str> {
        vec!["narrative", "narrative_elements", "events", "validation", "characters", "project", "export"]
    }
}

impl Default for SchemaManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_schema_manager_creation() {
        let manager = SchemaManager::new();
        assert!(!manager.available_sections().is_empty());
    }

    #[test]
    fn test_validate_valid_narrative_response() {
        let manager = SchemaManager::new();
        
        let valid_response = json!({
            "status": "success",
            "section": "narrative",
            "action": "generate",
            "data": {
                "title": "Test Title",
                "summary": "Test Summary",
                "act": "Act1",
                "tone": "Ambiguous",
                "historical_scope": "PlausibleInferred",
                "spaces": [],
                "factions": [],
                "stakes": [],
                "tags": []
            },
            "warnings": []
        });
        
        let result = manager.validate_response("narrative", &valid_response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_narrative_response() {
        let manager = SchemaManager::new();
        
        let invalid_response = json!({
            "status": "success",
            "section": "narrative",
            "action": "generate",
            "data": {
                "title": "Test Title",
                // Falta summary (required)
                "act": "Act1"
            },
            "warnings": []
        });
        
        let result = manager.validate_response("narrative", &invalid_response);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("summary")));
    }

    #[test]
    fn test_validate_narrative_elements_response() {
        let manager = SchemaManager::new();
        
        let valid_response = json!({
            "status": "success",
            "section": "narrative_elements",
            "action": "generate",
            "data": {
                "themes": [{
                    "id": "theme_1",
                    "type": "Theme",
                    "label": "Test Theme",
                    "description": "Test Description"
                }],
                "protagonists": [],
                "antagonists": [],
                "secondaries": [],
                "scenarios": [],
                "procedures": [],
                "dramatic_resources": [],
                "social_pressures": []
            },
            "warnings": []
        });
        
        let result = manager.validate_response("narrative_elements", &valid_response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_enum() {
        let manager = SchemaManager::new();
        
        let invalid_response = json!({
            "status": "success",
            "section": "narrative",
            "action": "generate",
            "data": {
                "title": "Test Title",
                "summary": "Test Summary",
                "act": "InvalidAct"  // Acto inválido
            },
            "warnings": []
        });
        
        let result = manager.validate_response("narrative", &invalid_response);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Invalid value")));
    }
}
