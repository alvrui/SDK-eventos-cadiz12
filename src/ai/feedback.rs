//! Sistema de feedback para agentes Mistral

use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use chrono::{DateTime, Local};

/// Feedback para agentes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFeedback {
    pub section: String,
    pub action: String,
    pub agent: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
    pub timestamp: String,
    pub response_id: Option<String>,
}

impl AgentFeedback {
    /// Crear nuevo feedback
    pub fn new(section: impl Into<String>, action: impl Into<String>, agent: impl Into<String>) -> Self {
        Self {
            section: section.into(),
            action: action.into(),
            agent: agent.into(),
            errors: vec![],
            warnings: vec![],
            suggestions: vec![],
            timestamp: Local::now().to_rfc3339(),
            response_id: None,
        }
    }

    /// Añadir error
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.errors.push(error.into());
        self
    }

    /// Añadir warning
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }

    /// Añadir sugerencia
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    /// Añadir ID de respuesta
    pub fn with_response_id(mut self, response_id: impl Into<String>) -> Self {
        self.response_id = Some(response_id.into());
        self
    }

    /// Convertir a JSON
    pub fn to_json(&self) -> Value {
        json!({
            "section": self.section,
            "action": self.action,
            "agent": self.agent,
            "errors": self.errors,
            "warnings": self.warnings,
            "suggestions": self.suggestions,
            "timestamp": self.timestamp,
            "response_id": self.response_id
        })
    }

    /// Verificar si tiene errores
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Verificar si tiene warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Obtener mensaje de error principal
    pub fn get_main_error(&self) -> Option<&str> {
        self.errors.first().map(|e| e.as_str())
    }
}

impl Default for AgentFeedback {
    fn default() -> Self {
        Self::new("", "", "")
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
            self.add_suggestions_for_error(&mut feedback, error);
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
        feedback = feedback.with_suggestion("Usa un validador de JSON antes de enviar la respuesta");
        
        // Truncar raw_response si es muy largo
        let truncated_response = if raw_response.len() > 500 {
            format!("{}... (truncated)", &raw_response[..500])
        } else {
            raw_response.to_string()
        };
        
        feedback = feedback.with_suggestion(format!("Respuesta recibida: {}", truncated_response));
        
        feedback
    }

    /// Generar feedback para error de schema
    pub fn generate_schema_feedback(
        &self,
        section: &str,
        action: &str,
        agent: &str,
        schema_errors: &[String],
    ) -> AgentFeedback {
        let mut feedback = AgentFeedback::new(section, action, agent);
        
        for error in schema_errors {
            feedback = feedback.with_error(error.clone());
            
            // Generar sugerencias específicas para errores de schema
            if error.contains("missing field") || error.contains("required") {
                feedback = feedback.with_suggestion("Asegúrate de incluir todos los campos requeridos en la respuesta");
            } else if error.contains("invalid type") {
                feedback = feedback.with_suggestion("Verifica que los tipos de datos sean correctos (string, array, number, etc.)");
            } else if error.contains("invalid enum value") {
                feedback = feedback.with_suggestion("Usa solo valores válidos para los enums (Act1, Act2, Act3, Act4, etc.)");
            } else if error.contains("additional property") {
                feedback = feedback.with_suggestion("No incluyas campos adicionales no definidos en el schema");
            }
        }
        
        feedback
    }

    /// Generar feedback para IDs no encontrados
    pub fn generate_id_not_found_feedback(
        &self,
        section: &str,
        action: &str,
        agent: &str,
        invalid_ids: &[String],
        element_type: &str,
    ) -> AgentFeedback {
        let mut feedback = AgentFeedback::new(section, action, agent);
        
        for id in invalid_ids {
            feedback = feedback.with_error(format!(
                "{} ID '{}' not found in catalog",
                element_type, id
            ));
        }
        
        feedback = feedback.with_suggestion(format!(
            "Verifica que los IDs de {} existan en los catálogos o usa 'unresolved': true para IDs no definitivos",
            element_type
        ));
        
        feedback = feedback.with_suggestion("Puedes consultar los IDs disponibles usando el endpoint /api/catalog");
        
        feedback
    }

    /// Generar feedback para respuesta vacía
    pub fn generate_empty_response_feedback(
        &self,
        section: &str,
        action: &str,
        agent: &str,
    ) -> AgentFeedback {
        let mut feedback = AgentFeedback::new(section, action, agent);
        
        feedback = feedback.with_error("La respuesta no contiene datos");
        feedback = feedback.with_suggestion("Asegúrate de incluir el campo 'data' con la información solicitada");
        feedback = feedback.with_suggestion("Si no hay datos disponibles, devuelve un array vacío o un objeto con valores por defecto");
        
        feedback
    }

    /// Añadir sugerencias basadas en el error
    fn add_suggestions_for_error(&self, feedback: &mut AgentFeedback, error: &str) {
        // Sugerencias generales
        if error.contains("status") {
            feedback.suggestions.push("El campo 'status' debe ser 'success' o 'error'".to_string());
        }
        
        if error.contains("section") {
            feedback.suggestions.push("El campo 'section' debe coincidir con la sección solicitada".to_string());
        }
        
        if error.contains("action") {
            feedback.suggestions.push("El campo 'action' debe ser una cadena no vacía".to_string());
        }
        
        if error.contains("data") {
            feedback.suggestions.push("El campo 'data' debe ser un objeto JSON válido".to_string());
        }
    }

    /// Generar feedback genérico para error desconocido
    pub fn generate_generic_error_feedback(
        &self,
        section: &str,
        action: &str,
        agent: &str,
        error: &str,
    ) -> AgentFeedback {
        let mut feedback = AgentFeedback::new(section, action, agent);
        
        feedback = feedback.with_error(error.to_string());
        feedback = feedback.with_suggestion("Revisa el formato de la respuesta");
        feedback = feedback.with_suggestion("Consulta la documentación para el formato esperado");
        
        feedback
    }

    /// Generar feedback de éxito
    pub fn generate_success_feedback(
        &self,
        section: &str,
        action: &str,
        agent: &str,
    ) -> AgentFeedback {
        AgentFeedback::new(section, action, agent)
    }
}

impl Default for FeedbackGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Colección de feedback para análisis
#[derive(Debug, Clone, Default)]
pub struct FeedbackCollector {
    feedbacks: Vec<AgentFeedback>,
}

impl FeedbackCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Añadir feedback
    pub fn add_feedback(&mut self, feedback: AgentFeedback) {
        self.feedbacks.push(feedback);
    }

    /// Obtener todos los feedbacks
    pub fn get_all(&self) -> &[AgentFeedback] {
        &self.feedbacks
    }

    /// Obtener feedbacks con errores
    pub fn get_errors(&self) -> Vec<&AgentFeedback> {
        self.feedbacks.iter().filter(|f| f.has_errors()).collect()
    }

    /// Obtener feedbacks con warnings
    pub fn get_warnings(&self) -> Vec<&AgentFeedback> {
        self.feedbacks.iter().filter(|f| f.has_warnings()).collect()
    }

    /// Obtener estadísticas
    pub fn get_stats(&self) -> FeedbackStats {
        let total = self.feedbacks.len();
        let with_errors = self.get_errors().len();
        let with_warnings = self.get_warnings().len();
        let without_issues = total - with_errors - with_warnings;
        
        FeedbackStats {
            total,
            with_errors,
            with_warnings,
            without_issues,
            error_rate: if total > 0 { (with_errors as f32 / total as f32) * 100.0 } else { 0.0 },
        }
    }

    /// Limpiar feedbacks antiguos
    pub fn clear_old(&mut self, max_age: std::time::Duration) {
        let cutoff = Local::now() - chrono::Duration::from_std(max_age).unwrap_or_default();
        
        self.feedbacks.retain(|f| {
            if let Ok(ts) = DateTime::parse_from_rfc3339(&f.timestamp) {
                ts > cutoff
            } else {
                true
            }
        });
    }

    /// Exportar a JSON
    pub fn to_json(&self) -> Value {
        json!(self.feedbacks.iter().map(|f| f.to_json()).collect::<Vec<_>>())
    }
}

/// Estadísticas de feedback
#[derive(Debug, Clone)]
pub struct FeedbackStats {
    pub total: usize,
    pub with_errors: usize,
    pub with_warnings: usize,
    pub without_issues: usize,
    pub error_rate: f32,
}

impl FeedbackStats {
    pub fn to_json(&self) -> Value {
        json!({
            "total": self.total,
            "with_errors": self.with_errors,
            "with_warnings": self.with_warnings,
            "without_issues": self.without_issues,
            "error_rate": self.error_rate
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_feedback_creation() {
        let feedback = AgentFeedback::new("narrative", "generate", "CoordinadorNarrativo");
        
        assert_eq!(feedback.section, "narrative");
        assert_eq!(feedback.action, "generate");
        assert_eq!(feedback.agent, "CoordinadorNarrativo");
        assert!(feedback.errors.is_empty());
        assert!(feedback.warnings.is_empty());
        assert!(feedback.suggestions.is_empty());
    }

    #[test]
    fn test_feedback_with_errors() {
        let mut feedback = AgentFeedback::new("narrative", "generate", "CoordinadorNarrativo");
        feedback = feedback.with_error("Error 1");
        feedback = feedback.with_error("Error 2");
        
        assert_eq!(feedback.errors.len(), 2);
        assert!(feedback.has_errors());
        assert!(!feedback.has_warnings());
    }

    #[test]
    fn test_feedback_with_suggestions() {
        let mut feedback = AgentFeedback::new("narrative", "generate", "CoordinadorNarrativo");
        feedback = feedback.with_error("Error 1");
        feedback = feedback.with_suggestion("Suggestion 1");
        feedback = feedback.with_suggestion("Suggestion 2");
        
        assert_eq!(feedback.suggestions.len(), 2);
    }

    #[test]
    fn test_feedback_to_json() {
        let feedback = AgentFeedback::new("narrative", "generate", "CoordinadorNarrativo")
            .with_error("Test error");
        
        let json = feedback.to_json();
        assert!(json.get("section").is_some());
        assert!(json.get("errors").is_some());
    }

    #[test]
    fn test_feedback_generator_validation() {
        let generator = FeedbackGenerator::new();
        
        let errors = vec![
            "Missing field: title".to_string(),
            "Invalid type for act".to_string(),
        ];
        
        let feedback = generator.generate_validation_feedback(
            "narrative", "generate", "CoordinadorNarrativo", &errors
        );
        
        assert!(feedback.has_errors());
        assert_eq!(feedback.errors.len(), 2);
        assert!(!feedback.suggestions.is_empty());
    }

    #[test]
    fn test_feedback_generator_parsing() {
        let generator = FeedbackGenerator::new();
        
        let feedback = generator.generate_parsing_feedback(
            "narrative", "generate", "CoordinadorNarrativo", "invalid json"
        );
        
        assert!(feedback.has_errors());
        assert!(!feedback.suggestions.is_empty());
    }

    #[test]
    fn test_feedback_collector() {
        let mut collector = FeedbackCollector::new();
        
        collector.add_feedback(AgentFeedback::new("narrative", "generate", "Agent1"));
        collector.add_feedback(AgentFeedback::new("narrative", "generate", "Agent1").with_error("Error"));
        collector.add_feedback(AgentFeedback::new("narrative", "generate", "Agent1").with_warning("Warning"));
        
        assert_eq!(collector.get_all().len(), 3);
        assert_eq!(collector.get_errors().len(), 1);
        assert_eq!(collector.get_warnings().len(), 1);
        
        let stats = collector.get_stats();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.with_errors, 1);
        assert_eq!(stats.with_warnings, 1);
        assert_eq!(stats.without_issues, 1);
    }

    #[test]
    fn test_feedback_stats() {
        let stats = FeedbackStats {
            total: 10,
            with_errors: 2,
            with_warnings: 3,
            without_issues: 5,
            error_rate: 20.0,
        };
        
        let json = stats.to_json();
        assert_eq!(json["total"], 10);
        assert_eq!(json["error_rate"], 20.0);
    }
}
