//! Validador de respuestas de agentes Mistral

use crate::ai::schemas::SchemaManager;
use serde_json::Value;

/// Validador de respuestas de agentes
#[derive(Debug, Clone)]
pub struct AgentResponseValidator {
    schema_manager: SchemaManager,
}

impl AgentResponseValidator {
    pub fn new() -> Self {
        Self {
            schema_manager: SchemaManager::new(),
        }
    }

    /// Validar respuesta de agente por sección
    pub fn validate_response(
        &self,
        section: &str,
        response: &Value,
    ) -> Result<(), Vec<String>> {
        // Primero, validar estructura básica
        self.validate_basic_structure(section, response)?;
        
        // Luego, validar contra schema específico
        self.schema_manager.validate_response(section, response)
    }

    /// Validar estructura básica (status, section, action, data)
    fn validate_basic_structure(
        &self,
        expected_section: &str,
        response: &Value,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Validar que es un objeto
        if !response.is_object() {
            errors.push("Response must be a JSON object".to_string());
            return Err(errors);
        }
        
        // Validar status
        if let Some(status) = response.get("status") {
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
        if let Some(section) = response.get("section") {
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
        if response.get("action").is_none() {
            errors.push("Missing required field: 'action'".to_string());
        }
        
        // Validar data
        if response.get("data").is_none() {
            errors.push("Missing required field: 'data'".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validar que todos los IDs en la respuesta existen en los catálogos
    pub fn validate_ids_against_catalogs(
        &self,
        section: &str,
        response: &Value,
        catalogs: &crate::catalog::Catalogs,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        match section {
            "narrative_elements" => {
                self.validate_narrative_elements_ids(response, catalogs, &mut errors);
            }
            "events" => {
                self.validate_event_ids(response, catalogs, &mut errors);
            }
            "narrative" => {
                // Narrativa no requiere validación de IDs
            }
            _ => {
                // Para otras secciones, no validamos IDs por ahora
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validar IDs en narrative_elements
    fn validate_narrative_elements_ids(
        &self,
        response: &Value,
        catalogs: &crate::catalog::Catalogs,
        errors: &mut Vec<String>,
    ) {
        if let Some(data) = response.get("data").and_then(|d| d.as_object()) {
            // Validar themes
            if let Some(themes) = data.get("themes").and_then(|t| t.as_array()) {
                for theme in themes {
                    if let Some(id) = theme.get("id").and_then(|id| id.as_str()) {
                        if !catalogs.themes.themes.contains_key(&crate::domain::ids::ThemeId(id.to_string())) {
                            errors.push(format!("Theme ID '{}' not found in catalog", id));
                        }
                    }
                }
            }
            
            // Validar protagonists
            if let Some(protagonists) = data.get("protagonists").and_then(|p| p.as_array()) {
                for protagonist in protagonists {
                    if let Some(id) = protagonist.get("id").and_then(|id| id.as_str()) {
                        if !catalogs.protagonists.protagonists.contains_key(&crate::domain::ids::ProtagonistId(id.to_string())) {
                            errors.push(format!("Protagonist ID '{}' not found in catalog", id));
                        }
                    }
                }
            }
            
            // Validar antagonists
            if let Some(antagonists) = data.get("antagonists").and_then(|a| a.as_array()) {
                for antagonist in antagonists {
                    if let Some(id) = antagonist.get("id").and_then(|id| id.as_str()) {
                        if !catalogs.antagonists.antagonists.contains_key(&crate::domain::ids::AntagonistId(id.to_string())) {
                            errors.push(format!("Antagonist ID '{}' not found in catalog", id));
                        }
                    }
                }
            }
            
            // Validar scenarios
            if let Some(scenarios) = data.get("scenarios").and_then(|s| s.as_array()) {
                for scenario in scenarios {
                    if let Some(id) = scenario.get("id").and_then(|id| id.as_str()) {
                        if !catalogs.scenarios.scenarios.contains_key(&crate::domain::ids::ScenarioId(id.to_string())) {
                            errors.push(format!("Scenario ID '{}' not found in catalog", id));
                        }
                    }
                }
            }
            
            // Validar procedures
            if let Some(procedures) = data.get("procedures").and_then(|p| p.as_array()) {
                for procedure in procedures {
                    if let Some(id) = procedure.get("id").and_then(|id| id.as_str()) {
                        if !catalogs.procedures.procedures.contains_key(&crate::domain::ids::ProcedureId(id.to_string())) {
                            errors.push(format!("Procedure ID '{}' not found in catalog", id));
                        }
                    }
                }
            }
        }
    }

    /// Validar IDs en events
    fn validate_event_ids(
        &self,
        response: &Value,
        catalogs: &crate::catalog::Catalogs,
        errors: &mut Vec<String>,
    ) {
        if let Some(data) = response.get("data").and_then(|d| d.as_object()) {
            // Validar theme_id
            if let Some(theme_id) = data.get("story_element_id").and_then(|id| id.as_str()) {
                if !catalogs.themes.themes.contains_key(&crate::domain::ids::ThemeId(theme_id.to_string())) {
                    errors.push(format!("Theme ID '{}' not found in catalog", theme_id));
                }
            }
        }
    }

    /// Validar que la respuesta tiene el formato esperado
    pub fn validate_response_format(
        &self,
        section: &str,
        response: &Value,
    ) -> Result<(), Vec<String>> {
        // Validar estructura básica
        self.validate_basic_structure(section, response)?;
        
        // Validar contra schema
        self.schema_manager.validate_response(section, response)
    }
}

impl Default for AgentResponseValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::catalog::Catalogs;

    #[test]
    fn test_validator_creation() {
        let validator = AgentResponseValidator::new();
        assert!(!validator.schema_manager.available_sections().is_empty());
    }

    #[test]
    fn test_validate_basic_structure() {
        let validator = AgentResponseValidator::new();
        
        let valid_response = json!({
            "status": "success",
            "section": "narrative",
            "action": "generate",
            "data": {}
        });
        
        let result = validator.validate_basic_structure("narrative", &valid_response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_missing_fields() {
        let validator = AgentResponseValidator::new();
        
        let invalid_response = json!({
            "status": "success",
            // Falta section, action, data
        });
        
        let result = validator.validate_basic_structure("narrative", &invalid_response);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 3); // Al menos 3 campos faltantes
    }

    #[test]
    fn test_validate_wrong_section() {
        let validator = AgentResponseValidator::new();
        
        let response = json!({
            "status": "success",
            "section": "wrong_section",
            "action": "generate",
            "data": {}
        });
        
        let result = validator.validate_basic_structure("narrative", &response);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Expected section")));
    }

    #[test]
    fn test_validate_full_response() {
        let validator = AgentResponseValidator::new();
        
        let valid_response = json!({
            "status": "success",
            "section": "narrative",
            "action": "generate",
            "data": {
                "title": "Test",
                "summary": "Test Summary",
                "act": "Act1",
                "tone": "Ambiguous"
            },
            "warnings": []
        });
        
        let result = validator.validate_response("narrative", &valid_response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_ids_against_catalogs() {
        let validator = AgentResponseValidator::new();
        let mut catalogs = Catalogs::new();
        
        // Añadir un tema válido
        let mut theme = crate::domain::structs::Theme::new("valid_theme");
        theme.base.label = "Valid Theme".to_string();
        catalogs.themes.add(theme);
        
        let response = json!({
            "status": "success",
            "section": "narrative_elements",
            "action": "generate",
            "data": {
                "themes": [{
                    "id": "valid_theme",
                    "type": "Theme",
                    "label": "Valid Theme",
                    "description": "Test"
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
        
        let result = validator.validate_ids_against_catalogs("narrative_elements", &response, &catalogs);
        assert!(result.is_ok());
        
        // Test con ID inválido
        let response_with_invalid_id = json!({
            "status": "success",
            "section": "narrative_elements",
            "action": "generate",
            "data": {
                "themes": [{
                    "id": "invalid_theme",
                    "type": "Theme",
                    "label": "Invalid Theme",
                    "description": "Test"
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
        
        let result = validator.validate_ids_against_catalogs("narrative_elements", &response_with_invalid_id, &catalogs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("invalid_theme")));
    }
}
