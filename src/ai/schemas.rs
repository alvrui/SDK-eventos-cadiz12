//! Schemas JSON para validación de respuestas de agentes

use jsonschema::JSONSchema;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Schema para respuesta de generación de narrativa
pub const NARRATIVE_SCHEMA_STR: &str = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {
            "type": "string",
            "enum": ["success", "error"]
        },
        "section": {
            "type": "string",
            "enum": ["project", "narrative", "narrative_elements", "plots", "story_elements", "events", "characters", "validation", "export"]
        },
        "action": {
            "type": "string"
        },
        "data": {
            "type": "object",
            "required": ["title", "summary"],
            "properties": {
                "title": {"type": "string", "minLength": 1},
                "summary": {"type": "string"},
                "description": {"type": "string"},
                "act": {
                    "type": "string",
                    "enum": ["Act1", "Act2", "Act3", "Act4"]
                },
                "tone": {
                    "type": "string",
                    "enum": ["Festive", "Satirical", "Anxious", "Solemn", "Intimate", "Conspiratorial", "Patriotic", "Sordid", "Tragic", "Ambiguous", "Tense", "Polemical", "Funereal", "Resilient", "Combative", "Compassionate"]
                },
                "historical_scope": {
                    "type": "string",
                    "enum": ["StrictHistorical", "PlausibleDocumented", "PlausibleInferred", "ExceptionalButVerisimilar", "Discarded"]
                },
                "spaces": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "factions": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "stakes": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "tags": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "extendedNotes": {"type": "string"}
            },
            "additionalProperties": false
        },
        "warnings": {
            "type": "array",
            "items": {"type": "string"}
        }
    },
    "additionalProperties": false
}"#;

/// Schema para respuesta de narrative_elements
pub const NARRATIVE_ELEMENTS_SCHEMA_STR: &str = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {
            "type": "string",
            "enum": ["success", "error"]
        },
        "section": {
            "type": "string",
            "enum": ["narrative_elements"]
        },
        "action": {
            "type": "string"
        },
        "data": {
            "type": "object",
            "properties": {
                "themes": {
                    "type": "array",
                    "items": {"$ref": "#/definitions/theme"}
                },
                "protagonists": {
                    "type": "array",
                    "items": {"$ref": "#/definitions/protagonist"}
                },
                "antagonists": {
                    "type": "array",
                    "items": {"$ref": "#/definitions/antagonist"}
                },
                "secondaries": {
                    "type": "array",
                    "items": {"$ref": "#/definitions/secondary"}
                },
                "scenarios": {
                    "type": "array",
                    "items": {"$ref": "#/definitions/scenario"}
                },
                "procedures": {
                    "type": "array",
                    "items": {"$ref": "#/definitions/procedure"}
                },
                "dramatic_resources": {
                    "type": "array",
                    "items": {"$ref": "#/definitions/dramatic_resource"}
                },
                "social_pressures": {
                    "type": "array",
                    "items": {"$ref": "#/definitions/social_pressure"}
                }
            },
            "additionalProperties": false
        },
        "warnings": {
            "type": "array",
            "items": {"type": "string"}
        }
    },
    "definitions": {
        "theme": {
            "type": "object",
            "required": ["id", "type", "label", "description"],
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "type": {"type": "string", "enum": ["Theme"]},
                "label": {"type": "string", "minLength": 1},
                "description": {"type": "string"},
                "tone": {
                    "type": "string",
                    "enum": ["Festive", "Satirical", "Anxious", "Solemn", "Intimate", "Conspiratorial", "Patriotic", "Sordid", "Tragic", "Ambiguous", "Tense", "Polemical", "Funereal", "Resilient", "Combative", "Compassionate"]
                },
                "historical_scope": {
                    "type": "string",
                    "enum": ["StrictHistorical", "PlausibleDocumented", "PlausibleInferred", "ExceptionalButVerisimilar", "Discarded"]
                },
                "time_window": {
                    "type": "array",
                    "items": {
                        "type": "string",
                        "enum": ["Y1805_1808", "Y1809", "Y1810", "Y1811", "Y1812", "Y1813", "Y1814", "Y1815_1816"]
                    }
                },
                "act_bias": {
                    "type": "array",
                    "items": {
                        "type": "string",
                        "enum": ["Act1", "Act2", "Act3", "Act4"]
                    }
                },
                "stakes_axis": {
                    "type": "array",
                    "items": {
                        "type": "string",
                        "enum": ["Personal", "Political", "Urban", "Institutional", "Imperial", "Moral", "Economic", "Religious", "Military", "Media"]
                    }
                },
                "faction_vectors": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "space_vectors": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "compatibility_tags": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "blocking_tags": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "unlock_tags": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "generated_tags": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "additionalProperties": false
        },
        "protagonist": {
            "type": "object",
            "required": ["id", "type", "label", "description"],
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "type": {"type": "string", "enum": ["Protagonist"]},
                "label": {"type": "string", "minLength": 1},
                "description": {"type": "string"},
                "eligible_profiles": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "eligible_positions": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "additionalProperties": false
        },
        "antagonist": {
            "type": "object",
            "required": ["id", "type", "label", "description"],
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "type": {"type": "string", "enum": ["Antagonist"]},
                "label": {"type": "string", "minLength": 1},
                "description": {"type": "string"}
            },
            "additionalProperties": false
        },
        "secondary": {
            "type": "object",
            "required": ["id", "type", "label", "description"],
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "type": {"type": "string", "enum": ["Secondary"]},
                "label": {"type": "string", "minLength": 1},
                "description": {"type": "string"}
            },
            "additionalProperties": false
        },
        "scenario": {
            "type": "object",
            "required": ["id", "type", "label", "description"],
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "type": {"type": "string", "enum": ["Scenario"]},
                "label": {"type": "string", "minLength": 1},
                "description": {"type": "string"}
            },
            "additionalProperties": false
        },
        "procedure": {
            "type": "object",
            "required": ["id", "type", "label", "description", "kind"],
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "type": {"type": "string", "enum": ["Procedure"]},
                "label": {"type": "string", "minLength": 1},
                "description": {"type": "string"},
                "kind": {
                    "type": "string",
                    "enum": ["DebatePlenary", "TechnicalCommission", "EmergencySession", "HonorSession", "DecreeVote", "PriorPositioningCall", "PrivateNegotiation", "PetitionSubmission", "DocumentReading", "PressPublication", "PressDenunciation", "StrategicLeak", "StrategicDelay", "AmendmentProposal", "SecretVoteRequest", "SafeConductProcessing", "HousingAssignment", "HealthDeclaration", "NeighborhoodRelief", "SignatureCirculation"]
                }
            },
            "additionalProperties": false
        },
        "dramatic_resource": {
            "type": "object",
            "required": ["id", "type", "label", "description"],
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "type": {"type": "string", "enum": ["DramaticResource"]},
                "label": {"type": "string", "minLength": 1},
                "description": {"type": "string"}
            },
            "additionalProperties": false
        },
        "social_pressure": {
            "type": "object",
            "required": ["id", "type", "label", "description"],
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "type": {"type": "string", "enum": ["SocialPressure"]},
                "label": {"type": "string", "minLength": 1},
                "description": {"type": "string"}
            },
            "additionalProperties": false
        }
    },
    "additionalProperties": false
}"#;

/// Schema para respuesta de evento
pub const EVENT_SCHEMA_STR: &str = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {
            "type": "string",
            "enum": ["success", "error"]
        },
        "section": {
            "type": "string",
            "enum": ["events"]
        },
        "action": {
            "type": "string"
        },
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
                "consequences": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "assets": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "additionalProperties": false
        },
        "warnings": {
            "type": "array",
            "items": {"type": "string"}
        }
    },
    "additionalProperties": false
}"#;

/// Schema para respuesta de validación
pub const VALIDATION_SCHEMA_STR: &str = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {
            "type": "string",
            "enum": ["success", "error"]
        },
        "section": {
            "type": "string",
            "enum": ["validation"]
        },
        "action": {
            "type": "string"
        },
        "data": {
            "type": "object",
            "required": ["summary", "issues"],
            "properties": {
                "summary": {"type": "string"},
                "issues": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "additionalProperties": false
        },
        "warnings": {
            "type": "array",
            "items": {"type": "string"}
        }
    },
    "additionalProperties": false
}"#;

/// Schema para respuesta de personajes
pub const CHARACTERS_SCHEMA_STR: &str = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {
            "type": "string",
            "enum": ["success", "error"]
        },
        "section": {
            "type": "string",
            "enum": ["characters"]
        },
        "action": {
            "type": "string"
        },
        "data": {
            "type": "array",
            "items": {
                "type": "object",
                "required": ["id", "name", "projectRoleType"],
                "properties": {
                    "id": {"type": "string", "minLength": 1},
                    "name": {"type": "string", "minLength": 1},
                    "projectRoleType": {
                        "type": "string",
                        "enum": ["protagonist", "antagonist", "supporting", "flex"]
                    },
                    "summary": {"type": "string"},
                    "description": {"type": "string"},
                    "extendedNotes": {"type": "string"}
                }
            }
        },
        "warnings": {
            "type": "array",
            "items": {"type": "string"}
        }
    },
    "additionalProperties": false
}"#;

/// Schema para respuesta de proyecto
pub const PROJECT_SCHEMA_STR: &str = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {
            "type": "string",
            "enum": ["success", "error"]
        },
        "section": {
            "type": "string",
            "enum": ["project"]
        },
        "action": {
            "type": "string"
        },
        "data": {
            "type": "object",
            "required": ["title", "summary"],
            "properties": {
                "title": {"type": "string", "minLength": 1},
                "summary": {"type": "string"},
                "worldContext": {"type": "string"},
                "allowedGenres": {
                    "type": "array",
                    "items": {"type": "string"}
                },
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
                        "blockedSensitivityTags": {
                            "type": "array",
                            "items": {"type": "string"}
                        }
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
            },
            "additionalProperties": false
        },
        "warnings": {
            "type": "array",
            "items": {"type": "string"}
        }
    },
    "additionalProperties": false
}"#;

/// Schema para respuesta de exportación
pub const EXPORT_SCHEMA_STR: &str = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {
            "type": "string",
            "enum": ["success", "error"]
        },
        "section": {
            "type": "string",
            "enum": ["export"]
        },
        "action": {
            "type": "string"
        },
        "data": {
            "type": "object",
            "required": ["summary", "checklist"],
            "properties": {
                "summary": {"type": "string"},
                "checklist": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "issues": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "additionalProperties": false
        },
        "warnings": {
            "type": "array",
            "items": {"type": "string"}
        }
    },
    "additionalProperties": false
}"#;

/// Schema Manager - Gestiona todos los schemas
#[derive(Debug, Clone)]
pub struct SchemaManager {
    schemas: HashMap<String, JSONSchema>,
}

impl SchemaManager {
    pub fn new() -> Self {
        let mut schemas = HashMap::new();
        
        // Compilar y almacenar todos los schemas
        schemas.insert("narrative".to_string(), Self::compile_schema(NARRATIVE_SCHEMA_STR));
        schemas.insert("narrative_elements".to_string(), Self::compile_schema(NARRATIVE_ELEMENTS_SCHEMA_STR));
        schemas.insert("event".to_string(), Self::compile_schema(EVENT_SCHEMA_STR));
        schemas.insert("validation".to_string(), Self::compile_schema(VALIDATION_SCHEMA_STR));
        schemas.insert("characters".to_string(), Self::compile_schema(CHARACTERS_SCHEMA_STR));
        schemas.insert("project".to_string(), Self::compile_schema(PROJECT_SCHEMA_STR));
        schemas.insert("export".to_string(), Self::compile_schema(EXPORT_SCHEMA_STR));
        
        Self { schemas }
    }
    
    /// Obtener schema por sección
    pub fn get_schema(&self, section: &str) -> Option<&JSONSchema> {
        self.schemas.get(section)
    }
    
    /// Compilar schema desde string
    fn compile_schema(schema_str: &str) -> JSONSchema {
        // Usar JSONSchema::options() para configurar validación estricta
        let compiled = JSONSchema::options()
            .with_draft/jsonschema::Draft::Draft7)
            .compile(&serde_json::from_str::<serde_json::Value>(schema_str).unwrap())
            .expect("Failed to compile schema");
        
        compiled
    }
    
    /// Validar respuesta contra schema
    pub fn validate_response(&self, section: &str, response: &serde_json::Value) -> Result<(), Vec<String>> {
        let schema = self.get_schema(section)
            .ok_or_else(|| vec![format!("No schema found for section: {}", section)])?;
        
        if let Err(errors) = schema.validate(response) {
            let error_messages: Vec<String> = errors
                .map(|e| {
                    let instance_path = e.instance_path.to_string();
                    let schema_path = e.schema_path.to_string();
                    format!("Validation error at {}: {} (schema: {})", instance_path, e.kind, schema_path)
                })
                .collect();
            return Err(error_messages);
        }
        
        Ok(())
    }
    
    /// Obtener todas las secciones disponibles
    pub fn available_sections(&self) -> Vec<&str> {
        self.schemas.keys().map(|s| s.as_str()).collect()
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
                    "description": "Test Description",
                    "tone": "Ambiguous",
                    "historical_scope": "PlausibleInferred",
                    "time_window": ["Y1810"],
                    "act_bias": ["Act1"],
                    "stakes_axis": ["Political"],
                    "faction_vectors": [],
                    "space_vectors": [],
                    "compatibility_tags": [],
                    "blocking_tags": [],
                    "unlock_tags": [],
                    "generated_tags": []
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
}
