# Plan de Implementación - Fase 3
# Mejoras en Gestión de Agentes Mistral

## Objetivo
Mejorar la robustez, trazabilidad y calidad de las interacciones con los agentes Mistral que generan contenido narrativo.

## Contexto
Actualmente:
- Los agentes devuelven JSON pero no siempre en el formato esperado
- No hay validación estricta de las respuestas
- No hay logging de interacciones para debugging
- No hay feedback estructurado para mejorar las respuestas de los agentes
- Los prompts podrían ser más específicos y estructurados

## Alcance

### 1. Validación Estricta de Schemas
- Crear schemas JSON para cada tipo de respuesta de agente
- Validar respuestas antes de procesarlas
- Devolver errores claros y accionables

### 2. Feedback Estructurado
- Crear sistema de feedback para agentes
- Registrar errores de validación
- Proporcionar sugerencias para mejorar respuestas

### 3. Logging de Interacciones
- Registrar todas las solicitudes a agentes
- Guardar respuestas (éxito y error)
- Permitir debugging y análisis

### 4. Mejorar Prompts
- Prompts más específicos por acción
- Contexto estructurado y completo
- Instrucciones claras de formato

---

## Tareas Detalladas

### Tarea 1: Crear Schemas JSON para Validación
**Archivo**: `src/ai/schemas.rs` (nuevo)

```rust
/// Schema para respuesta de generación de narrativa
pub const NARRATIVE_SCHEMA: &str = r#"{
    "type": "object",
    "required": ["status", "section", "action", "data"],
    "properties": {
        "status": {"type": "string", "enum": ["success", "error"]},
        "section": {"type": "string"},
        "action": {"type": "string"},
        "data": {
            "type": "object",
            "required": ["title", "summary", "act", "tone"],
            "properties": {
                "title": {"type": "string"},
                "summary": {"type": "string"},
                "act": {"type": "string", "enum": ["Act1", "Act2", "Act3", "Act4"]},
                "tone": {"type": "string"},
                "historical_scope": {"type": "string"},
                "spaces": {"type": "array", "items": {"type": "string"}},
                "factions": {"type": "array", "items": {"type": "string"}},
                "stakes": {"type": "array", "items": {"type": "string"}},
                "tags": {"type": "array", "items": {"type": "string"}}
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
        "section": {"type": "string"},
        "action": {"type": "string"},
        "data": {
            "type": "object",
            "properties": {
                "themes": {"type": "array", "items": {"$ref": "#/definitions/theme"}},
                "protagonists": {"type": "array", "items": {"$ref": "#/definitions/protagonist"}},
                "antagonists": {"type": "array", "items": {"$ref": "#/definitions/antagonist"}},
                "secondaries": {"type": "array", "items": {"$ref": "#/definitions/secondary"}},
                "scenarios": {"type": "array", "items": {"$ref": "#/definitions/scenario"}},
                "procedures": {"type": "array", "items": {"$ref": "#/definitions/procedure"}},
                "dramatic_resources": {"type": "array", "items": {"$ref": "#/definitions/dramatic_resource"}},
                "social_pressures": {"type": "array", "items": {"$ref": "#/definitions/social_pressure"}}
            }
        },
        "warnings": {"type": "array", "items": {"type": "string"}}
    },
    "definitions": {
        "theme": {
            "type": "object",
            "required": ["id", "type", "label", "description"],
            "properties": {
                "id": {"type": "string"},
                "type": {"type": "string", "enum": ["Theme"]},
                "label": {"type": "string"},
                "description": {"type": "string"},
                "tone": {"type": "string"},
                "historical_scope": {"type": "string"},
                "time_window": {"type": "array", "items": {"type": "string"}},
                "act_bias": {"type": "array", "items": {"type": "string"}},
                "stakes_axis": {"type": "array", "items": {"type": "string"}},
                "faction_vectors": {"type": "array", "items": {"type": "string"}},
                "space_vectors": {"type": "array", "items": {"type": "string"}}
            }
        },
        // Definiciones similares para otros tipos...
    }
}"#;
```

**Prioridad**: Alta
**Esfuerzo**: Alto

---

### Tarea 2: Crear Validador de Schemas
**Archivo**: `src/ai/validator.rs` (nuevo)

```rust
use jsonschema::JSONSchema;
use serde_json::Value;

/// Validador de respuestas de agentes
#[derive(Debug, Clone)]
pub struct AgentResponseValidator {
    schemas: HashMap<String, JSONSchema>,
}

impl AgentResponseValidator {
    pub fn new() -> Self {
        let mut schemas = HashMap::new();
        
        // Cargar schemas
        schemas.insert("narrative".to_string(), self.compile_schema(NARRATIVE_SCHEMA));
        schemas.insert("narrative_elements".to_string(), self.compile_schema(NARRATIVE_ELEMENTS_SCHEMA));
        // ... otros schemas
        
        Self { schemas }
    }
    
    /// Validar respuesta de agente
    pub fn validate_response(&self, section: &str, response: &Value) -> Result<(), Vec<String>> {
        let schema = self.schemas.get(section)
            .ok_or_else(|| vec![format!("No schema found for section: {}", section)])?;
        
        if let Err(errors) = schema.validate(response) {
            let error_messages: Vec<String> = errors
                .map(|e| format!("Validation error: {}", e))
                .collect();
            return Err(error_messages);
        }
        
        Ok(())
    }
    
    /// Compilar schema desde string
    fn compile_schema(&self, schema_str: &str) -> JSONSchema {
        // Implementación usando jsonschema crate
    }
}
```

**Prioridad**: Alta
**Esfuerzo**: Alto

---

### Tarea 3: Crear Sistema de Logging
**Archivo**: `src/ai/logger.rs` (nuevo)

```rust
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use chrono::Local;

/// Logger de interacciones con agentes
#[derive(Debug, Clone)]
pub struct AgentInteractionLogger {
    log_dir: String,
    enabled: bool,
}

impl AgentInteractionLogger {
    pub fn new(log_dir: impl Into<String>) -> Self {
        let log_dir = log_dir.into();
        std::fs::create_dir_all(&log_dir).ok();
        
        Self {
            log_dir,
            enabled: true,
        }
    }
    
    /// Registrar solicitud a agente
    pub fn log_request(&self, section: &str, action: &str, agent: &str, prompt: &str) -> String {
        if !self.enabled {
            return String::new();
        }
        
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let filename = format!("{}/agent_{}_{}_{}.log", self.log_dir, section, action, timestamp);
        
        let log_entry = format!(
            "[{}] REQUEST\nSection: {}\nAction: {}\nAgent: {}\nPrompt:\n{}\n\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            section,
            action,
            agent,
            prompt
        );
        
        self.write_to_file(&filename, &log_entry);
        filename
    }
    
    /// Registrar respuesta de agente
    pub fn log_response(&self, filename: &str, response: &Value, is_success: bool) {
        if !self.enabled {
            return;
        }
        
        let status = if is_success { "SUCCESS" } else { "ERROR" };
        let log_entry = format!(
            "[{}] RESPONSE ({})\n{}\n\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            status,
            serde_json::to_string_pretty(response).unwrap_or_default()
        );
        
        self.write_to_file(filename, &log_entry);
    }
    
    /// Registrar error
    pub fn log_error(&self, section: &str, action: &str, error: &str) {
        if !self.enabled {
            return;
        }
        
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let filename = format!("{}/error_{}_{}_{}.log", self.log_dir, section, action, timestamp);
        
        let log_entry = format!(
            "[{}] ERROR\nSection: {}\nAction: {}\nError: {}\n\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            section,
            action,
            error
        );
        
        self.write_to_file(&filename, &log_entry);
    }
    
    fn write_to_file(&self, filename: &str, content: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
        {
            let _ = file.write_all(content.as_bytes());
        }
    }
}
```

**Prioridad**: Alta
**Esfuerzo**: Medio

---

### Tarea 4: Crear Sistema de Feedback
**Archivo**: `src/ai/feedback.rs` (nuevo)

```rust
use serde_json::{json, Value};

/// Feedback para agentes
#[derive(Debug, Clone)]
pub struct AgentFeedback {
    pub section: String,
    pub action: String,
    pub agent: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
    pub timestamp: String,
}

impl AgentFeedback {
    pub fn new(section: impl Into<String>, action: impl Into<String>, agent: impl Into<String>) -> Self {
        Self {
            section: section.into(),
            action: action.into(),
            agent: agent.into(),
            errors: vec![],
            warnings: vec![],
            suggestions: vec![],
            timestamp: chrono::Local::now().to_rfc3339(),
        }
    }
    
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.errors.push(error.into());
        self
    }
    
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }
    
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
    
    pub fn to_json(&self) -> Value {
        json!({
            "section": self.section,
            "action": self.action,
            "agent": self.agent,
            "errors": self.errors,
            "warnings": self.warnings,
            "suggestions": self.suggestions,
            "timestamp": self.timestamp
        })
    }
}

/// Generador de feedback automático
#[derive(Debug, Clone)]
pub struct FeedbackGenerator;

impl FeedbackGenerator {
    pub fn new() -> Self {
        Self
    }
    
    /// Generar feedback para error de validación
    pub fn generate_validation_feedback(
        &self,
        section: &str,
        action: &str,
        agent: &str,
        errors: &[String],
    ) -> AgentFeedback {
        let mut feedback = AgentFeedback::new(section, action, agent);
        
        for error in errors {
            feedback = feedback.with_error(error.clone());
            
            // Generar sugerencias basadas en el error
            if error.contains("missing field") {
                feedback = feedback.with_suggestion("Asegúrate de incluir todos los campos requeridos en la respuesta");
            } else if error.contains("invalid type") {
                feedback = feedback.with_suggestion("Verifica que los tipos de datos sean correctos (string, array, etc.)");
            } else if error.contains("invalid enum value") {
                feedback = feedback.with_suggestion("Usa solo valores válidos para los enums (Act1, Act2, etc.)");
            }
        }
        
        feedback
    }
    
    /// Generar feedback para error de parsing
    pub fn generate_parsing_feedback(
        &self,
        section: &str,
        action: &str,
        agent: &str,
        raw_response: &str,
    ) -> AgentFeedback {
        let mut feedback = AgentFeedback::new(section, action, agent);
        
        feedback = feedback.with_error("No se pudo parsear la respuesta como JSON válido");
        feedback = feedback.with_suggestion("Devuelve SOLO JSON válido, sin markdown ni texto adicional");
        feedback = feedback.with_suggestion(format!("Respuesta recibida: {}", raw_response));
        
        feedback
    }
}
```

**Prioridad**: Media
**Esfuerzo**: Medio

---

### Tarea 5: Mejorar Prompts
**Archivo**: `src/ai/prompts.rs` (nuevo)

```rust
/// Generador de prompts mejorados
#[derive(Debug, Clone)]
pub struct PromptBuilder {
    context_builder: ContextBuilder,
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            context_builder: ContextBuilder::new(),
        }
    }
    
    /// Construir prompt para narrative_elements
    pub fn build_narrative_elements_prompt(
        &self,
        action: &str,
        project: &Value,
        existing_elements: Option<&NarrativeElements>,
    ) -> String {
        let context = self.context_builder.build_context(project, existing_elements);
        
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de elementos narrativos unificados.

== CONTEXTO ==
{context}

== ACCIÓN ==
Acción solicitada: {action}

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. No inventes IDs definitivos. Si no conoces un ID exacto, usa un objeto con campo "unresolved": true
3. Usa solo valores de enum válidos (Act1, Act2, Act3, Act4, etc.)
4. Incluye todos los campos requeridos en cada tipo de elemento
5. Prioriza coherencia local, causalidad jugable y tensión política

== FORMATO DE SALIDA ==
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
        "tone": "string (enum)",
        "historical_scope": "string (enum)",
        "time_window": ["string (enum)"],
        "act_bias": ["string (enum)"],
        "stakes_axis": ["string (enum)"],
        "faction_vectors": ["string"],
        "space_vectors": ["string"]
      }}
    ],
    "protagonists": [...],
    "antagonists": [...],
    "secondaries": [...],
    "scenarios": [...],
    "procedures": [...],
    "dramatic_resources": [...],
    "social_pressures": [...]
  }},
  "warnings": ["string"]
}}

== NOTAS ==
- Si la acción es "generate", devuelve entre 3 y 8 elementos por tipo
- Si la acción es "refine", enfócate en mejorar elementos existentes
- Si la acción es "suggest", sugiere elementos que complementen los existentes"#,
            action = action,
            context = context
        )
    }
}

/// Constructor de contexto
#[derive(Debug, Clone)]
pub struct ContextBuilder;

impl ContextBuilder {
    pub fn new() -> Self {
        Self
    }
    
    /// Construir contexto para prompt
    pub fn build_context(
        &self,
        project: &Value,
        existing_elements: Option<&NarrativeElements>,
    ) -> String {
        let mut context_parts = Vec::new();
        
        // Información del proyecto
        if let Some(meta) = project.get("projectMeta") {
            context_parts.push(format!("Título: {}", meta.get("title").and_then(|v| v.as_str()).unwrap_or("")));
            context_parts.push(format!("Resumen: {}", meta.get("summary").and_then(|v| v.as_str()).unwrap_or("")));
            context_parts.push(format!("Formato: {}", meta.get("format").and_then(|v| v.as_str()).unwrap_or("")));
        }
        
        // Narrativas
        if let Some(narratives) = project.get("narratives").and_then(|v| v.as_array()) {
            if !narratives.is_empty() {
                context_parts.push("\n== NARRATIVAS ==".to_string());
                for nar in narratives {
                    context_parts.push(format!(
                        "- ID: {}, Título: {}, Acto: {}, Tono: {}",
                        nar.get("id").and_then(|v| v.as_str()).unwrap_or(""),
                        nar.get("title").and_then(|v| v.as_str()).unwrap_or(""),
                        nar.get("act").and_then(|v| v.as_str()).unwrap_or(""),
                        nar.get("tone").and_then(|v| v.as_str()).unwrap_or("")
                    ));
                }
            }
        }
        
        // Elementos existentes
        if let Some(elements) = existing_elements {
            context_parts.push("\n== ELEMENTOS EXISTENTES ==".to_string());
            context_parts.push(format!("Themes: {}", elements.themes.len()));
            context_parts.push(format!("Protagonists: {}", elements.protagonists.len()));
            context_parts.push(format!("Antagonists: {}", elements.antagonists.len()));
            context_parts.push(format!("Secondaries: {}", elements.secondaries.len()));
            context_parts.push(format!("Scenarios: {}", elements.scenarios.len()));
            context_parts.push(format!("Procedures: {}", elements.procedures.len()));
        }
        
        context_parts.join("\n")
    }
}
```

**Prioridad**: Media
**Esfuerzo**: Alto

---

### Tarea 6: Integrar Todo en main.rs
**Archivo**: `src/main.rs`

Modificar las funciones de manejo de agentes para usar:
- Validación de schemas
- Logging de interacciones
- Feedback estructurado
- Prompts mejorados

---

## Cronograma Estimado

| Tarea | Prioridad | Esfuerzo | Tiempo Estimado |
|-------|-----------|----------|-----------------|
| 1. Schemas JSON | Alta | Alto | 3 horas |
| 2. Validador de Schemas | Alta | Alto | 3 horas |
| 3. Logger de Interacciones | Alta | Medio | 2 horas |
| 4. Sistema de Feedback | Media | Medio | 2 horas |
| 5. Prompts Mejorados | Media | Alto | 3 horas |
| 6. Integración en main.rs | Alta | Medio | 2 horas |

**Total Estimado**: 15 horas

---

## Dependencias

Nuevas dependencias necesarias:
```toml
[dependencies]
jsonschema = "0.17"  # Para validación de schemas JSON
chrono = "0.4"      # Para timestamps en logging
```

---

## Criterios de Aceptación

1. ✅ Todos los tests pasan
2. ✅ No hay errores de compilación
3. ✅ Validación de schemas funciona para todas las secciones
4. ✅ Logging de interacciones funciona
5. ✅ Feedback se genera correctamente
6. ✅ Prompts mejorados generan respuestas válidas
7. ✅ No se rompe funcionalidad existente

---

## Riesgos

1. **Dependencias nuevas**: jsonschema y chrono podrían tener conflictos
   - **Mitigación**: Verificar compatibilidad, probar en entorno de desarrollo

2. **Rendimiento**: Validación de schemas podría ser lenta
   - **Mitigación**: Cachear schemas compilados, validar solo en modo debug

3. **Cambios en formato**: Agentes podrían necesitar ajustes
   - **Mitigación**: Mantener retrocompatibilidad, feedback claro a agentes

---

## Métricas de Éxito

- [ ] Todos los tests pasan
- [ ] No hay errores de compilación
- [ ] Validación de schemas funciona
- [ ] Logging funciona
- [ ] Feedback se genera
- [ ] Prompts mejorados
- [ ] No se rompe funcionalidad existente
