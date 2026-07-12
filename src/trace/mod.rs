//! Trace module - Trazabilidad de selección

use crate::domain::enums::*;
use crate::domain::ids::*;
use crate::domain::structs::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Registro de selección completo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectionTrace {
    pub candidate_themes: Vec<ThemeCandidateTrace>,
    pub chosen_theme: ThemeId,
    pub chosen_secondary_reasons: Vec<String>,
    pub chosen_procedure_reasons: Vec<String>,
    pub rejected_reasons: Vec<String>,
    pub final_score_breakdown: Vec<ScoreBreakdown>,
}

impl SelectionTrace {
    pub fn new() -> Self {
        Self {
            candidate_themes: vec![],
            chosen_theme: ThemeId("default".to_string()),
            chosen_secondary_reasons: vec![],
            chosen_procedure_reasons: vec![],
            rejected_reasons: vec![],
            final_score_breakdown: vec![],
        }
    }

    /// Convertir a string legible
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        output += "=== Selection Trace ===\n\n";
        
        output += &format!("Chosen Theme: {}\n", self.chosen_theme);
        output += "\n--- Candidate Themes ---\n";
        
        for (i, candidate) in self.candidate_themes.iter().enumerate() {
            output += &format!("  {}. {}: score = {:.2}\n", i + 1, candidate.theme_id, candidate.score);
            for reason in &candidate.reasons {
                output += &format!("     - {}\n", reason);
            }
        }
        
        output += "\n--- Selection Reasons ---\n";
        output += &format!("Secondary Reasons:\n");
        for reason in &self.chosen_secondary_reasons {
            output += &format!("  - {}\n", reason);
        }
        
        output += &format!("Procedure Reasons:\n");
        for reason in &self.chosen_procedure_reasons {
            output += &format!("  - {}\n", reason);
        }
        
        output += "\n--- Rejected Reasons ---\n";
        for reason in &self.rejected_reasons {
            output += &format!("  - {}\n", reason);
        }
        
        output += "\n--- Score Breakdown ---\n";
        for breakdown in &self.final_score_breakdown {
            output += &format!(
                "  {}: {:.2} * {:.1} = {:.2}\n",
                breakdown.component,
                breakdown.value,
                breakdown.weight,
                breakdown.weighted_value
            );
        }
        
        output
    }

    /// Añadir candidato de tema
    pub fn add_candidate(mut self, candidate: ThemeCandidateTrace) -> Self {
        self.candidate_themes.push(candidate);
        self
    }

    /// Configurar tema elegido
    pub fn with_chosen_theme(mut self, theme_id: impl Into<ThemeId>) -> Self {
        self.chosen_theme = theme_id.into();
        self
    }

    /// Añadir razón para secundarios
    pub fn add_secondary_reason(mut self, reason: impl Into<String>) -> Self {
        self.chosen_secondary_reasons.push(reason.into());
        self
    }

    /// Añadir razón para procedimiento
    pub fn add_procedure_reason(mut self, reason: impl Into<String>) -> Self {
        self.chosen_procedure_reasons.push(reason.into());
        self
    }

    /// Añadir razón de rechazo
    pub fn add_rejected_reason(mut self, reason: impl Into<String>) -> Self {
        self.rejected_reasons.push(reason.into());
        self
    }

    /// Configurar desglose de puntuación
    pub fn with_score_breakdown(mut self, breakdown: Vec<ScoreBreakdown>) -> Self {
        self.final_score_breakdown = breakdown;
        self
    }
}

/// Traza de candidato de tema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThemeCandidateTrace {
    pub theme_id: ThemeId,
    pub score: f32,
    pub reasons: Vec<String>,
}

impl ThemeCandidateTrace {
    pub fn new(theme_id: impl Into<ThemeId>, score: f32) -> Self {
        Self {
            theme_id: theme_id.into(),
            score,
            reasons: vec![],
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reasons.push(reason.into());
        self
    }

    pub fn with_reasons(mut self, reasons: Vec<impl Into<String>>) -> Self {
        for reason in reasons {
            self.reasons.push(reason.into());
        }
        self
    }
}

/// Desglose de puntuación
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreBreakdown {
    pub component: String,
    pub value: f32,
    pub weight: f32,
    pub weighted_value: f32,
}

impl ScoreBreakdown {
    pub fn new(component: impl Into<String>, value: f32, weight: f32) -> Self {
        Self {
            component: component.into(),
            value,
            weight,
            weighted_value: value * weight,
        }
    }
}

/// Formateador de traza para depuración
#[derive(Debug, Clone)]
pub struct TraceFormatter;

impl TraceFormatter {
    pub fn new() -> Self {
        Self
    }

    /// Formatear traza como JSON
    pub fn format_as_json(&self, trace: &SelectionTrace) -> String {
        serde_json::to_string_pretty(trace).unwrap_or_else(|_| "{}".to_string())
    }

    /// Formatear traza como texto simple
    pub fn format_as_text(&self, trace: &SelectionTrace) -> String {
        trace.to_string()
    }

    /// Formatear solo el desglose de puntuación
    pub fn format_score_breakdown(&self, breakdown: &[ScoreBreakdown]) -> String {
        let mut output = String::new();
        output += "Score Breakdown:\n";
        
        for b in breakdown {
            output += &format!(
                "  {}: {:.2} * {:.1} = {:.2}\n",
                b.component, b.value, b.weight, b.weighted_value
            );
        }
        
        output
    }
}

/// Registro de depuración para selección de eventos
#[derive(Debug, Clone)]
pub struct SelectionDebugLogger;

impl SelectionDebugLogger {
    pub fn new() -> Self {
        Self
    }

    /// Registrar inicio de selección
    pub fn log_selection_start(&self, context_description: &str) {
        log::debug!("=== Event Selection Started ===");
        log::debug!("Context: {}", context_description);
    }

    /// Registrar filtrado de temas
    pub fn log_filtering(&self, initial_count: usize, filtered_count: usize, filter_type: &str) {
        log::debug!(
            "Filtering: {} -> {} themes ({} filter)",
            initial_count, filtered_count, filter_type
        );
    }

    /// Registrar selección final
    pub fn log_selection_result(&self, event: &EventInstance) {
        log::debug!("=== Event Selection Result ===");
        log::debug!("Event ID: {}", event.id);
        log::debug!("Theme: {}", event.main_theme);
        log::debug!("Template: {:?}", event.scene_template);
        log::debug!("Procedure: {}", event.procedure);
        log::debug!("Secondaries: {:?}", event.secondaries);
    }

    /// Registrar traza completa
    pub fn log_trace(&self, trace: &SelectionTrace) {
        log::debug!("\n{}", trace.to_string());
    }
}

/// Implementación de Display para SelectionTrace
impl fmt::Display for SelectionTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_trace_creation() {
        let trace = SelectionTrace::new();
        assert_eq!(trace.candidate_themes.len(), 0);
        assert_eq!(trace.chosen_theme.0, "default");
    }

    #[test]
    fn test_selection_trace_to_string() {
        let mut trace = SelectionTrace::new();
        trace.chosen_theme = ThemeId("tema_1".to_string());
        
        let candidate = ThemeCandidateTrace::new("tema_1", 42.5)
            .with_reason("High compatibility score");
        trace = trace.add_candidate(candidate);
        
        let breakdown = vec![
            ScoreBreakdown::new("protagonist", 5.0, 3.0),
            ScoreBreakdown::new("scenario", 4.0, 2.0),
        ];
        trace = trace.with_score_breakdown(breakdown);
        
        let trace_str = trace.to_string();
        assert!(trace_str.contains("tema_1"));
        assert!(trace_str.contains("42.50"));
        assert!(trace_str.contains("High compatibility score"));
    }

    #[test]
    fn test_trace_formatter() {
        let formatter = TraceFormatter::new();
        
        let mut trace = SelectionTrace::new();
        trace.chosen_theme = ThemeId("tema_1".to_string());
        
        let json_str = formatter.format_as_json(&trace);
        assert!(json_str.contains("chosen_theme"));
        
        let text_str = formatter.format_as_text(&trace);
        assert!(text_str.contains("Selection Trace"));
    }

    #[test]
    fn test_theme_candidate_trace() {
        let mut candidate = ThemeCandidateTrace::new("tema_1", 42.5);
        candidate = candidate.with_reason("Reason 1");
        candidate = candidate.with_reason("Reason 2");
        
        assert_eq!(candidate.theme_id.0, "tema_1");
        assert_eq!(candidate.score, 42.5);
        assert_eq!(candidate.reasons.len(), 2);
    }

    #[test]
    fn test_score_breakdown() {
        let breakdown = ScoreBreakdown::new("protagonist", 5.0, 3.0);
        assert_eq!(breakdown.weighted_value, 15.0);
    }
}
