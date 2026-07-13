//! Validation module - Validación de datos y reglas

use crate::catalog::Catalogs;
use crate::compat::matrices::CompatibilitySet;
use crate::domain::enums::*;
use crate::domain::ids::*;
use crate::domain::structs::*;
use anyhow::{Context, Result};
use std::collections::HashSet;

/// Validador de catálogos y configuración
#[derive(Debug, Clone)]
pub struct CatalogValidator;

impl CatalogValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validar todos los catálogos
    pub fn validate_catalogs(&self, catalogs: &Catalogs) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validar catálogo de temas
        self.validate_themes(&catalogs.themes, &mut errors);

        // Validar catálogo de protagonistas
        self.validate_protagonists(&catalogs.protagonists, &mut errors);

        // Validar catálogo de antagonistas
        self.validate_antagonists(&catalogs.antagonists, &mut errors);

        // Validar catálogo de secundarios
        self.validate_secondaries(&catalogs.secondaries, &mut errors);

        // Validar catálogo de escenarios
        self.validate_scenarios(&catalogs.scenarios, &mut errors);

        // Validar catálogo de procedimientos
        self.validate_procedures(&catalogs.procedures, &mut errors);

        // Validar theme bindings
        self.validate_theme_bindings(&catalogs.theme_bindings, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_themes(&self, catalog: &crate::catalog::catalog::ThemeCatalog, errors: &mut Vec<String>) {
        for (id, theme) in &catalog.themes {
            // Validar que tiene time_window
            if theme.base.time_window.is_empty() {
                errors.push(format!("Theme {} has no time_window", id.0));
            }

            // Validar que tiene act_bias
            if theme.base.act_bias.is_empty() {
                errors.push(format!("Theme {} has no act_bias", id.0));
            }

            // Validar que scope histórico no es Discarded
            if theme.base.historical_scope == HistoricalScope::Discarded {
                errors.push(format!("Theme {} has Discarded historical_scope", id.0));
            }

            // Validar que time_window contiene valores válidos
            for time_slice in &theme.base.time_window {
                match time_slice {
                    TimeSlice::Y1805_1808 | TimeSlice::Y1809 | TimeSlice::Y1810 | 
                    TimeSlice::Y1811 | TimeSlice::Y1812 | TimeSlice::Y1813 | 
                    TimeSlice::Y1814 | TimeSlice::Y1815_1816 => {}
                }
            }
        }
    }

    fn validate_protagonists(&self, catalog: &crate::catalog::catalog::ProtagonistCatalog, errors: &mut Vec<String>) {
        for (id, protagonist) in &catalog.protagonists {
            // Validar que tiene eligible_profiles o eligible_positions
            if protagonist.eligible_profiles.is_empty() && protagonist.eligible_positions.is_empty() {
                errors.push(format!("Protagonist {} has no eligible_profiles or eligible_positions", id.0));
            }
        }
    }

    fn validate_antagonists(&self, _catalog: &crate::catalog::catalog::AntagonistCatalog, _errors: &mut Vec<String>) {
        // Por ahora, no hay validaciones específicas para antagonistas
    }

    fn validate_secondaries(&self, _catalog: &crate::catalog::catalog::SecondaryCatalog, _errors: &mut Vec<String>) {
        // Por ahora, no hay validaciones específicas para secundarios
    }

    fn validate_scenarios(&self, _catalog: &crate::catalog::catalog::ScenarioCatalog, _errors: &mut Vec<String>) {
        // Por ahora, no hay validaciones específicas para escenarios
    }

    fn validate_procedures(&self, catalog: &crate::catalog::catalog::ProcedureCatalog, _errors: &mut Vec<String>) {
        for (_id, procedure) in &catalog.procedures {
            // Validar que kind es válido
            match procedure.kind {
                ProcedureKind::DebatePlenary | ProcedureKind::TechnicalCommission | 
                ProcedureKind::EmergencySession | ProcedureKind::HonorSession | 
                ProcedureKind::DecreeVote | ProcedureKind::PriorPositioningCall | 
                ProcedureKind::PrivateNegotiation | ProcedureKind::PetitionSubmission | 
                ProcedureKind::DocumentReading | ProcedureKind::PressPublication | 
                ProcedureKind::PressDenunciation | ProcedureKind::StrategicLeak | 
                ProcedureKind::StrategicDelay | ProcedureKind::AmendmentProposal | 
                ProcedureKind::SecretVoteRequest | ProcedureKind::SafeConductProcessing | 
                ProcedureKind::HousingAssignment | ProcedureKind::HealthDeclaration | 
                ProcedureKind::NeighborhoodRelief | ProcedureKind::SignatureCirculation => {}
            }
        }
    }

    fn validate_theme_bindings(&self, catalog: &crate::catalog::theme_bindings::ThemeBindingsCatalog, errors: &mut Vec<String>) {
        for (theme_id, bindings) in &catalog.bindings {
            // Validar que tiene al menos un secundario o procedimiento
            if bindings.secondary_ids.is_empty() && bindings.procedure_ids.is_empty() {
                errors.push(format!("Theme bindings for {} has no secondaries or procedures", theme_id.0));
            }
        }
    }
}

/// Validador de matrices de compatibilidad
#[derive(Debug, Clone)]
pub struct CompatibilityValidator;

impl CompatibilityValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validar conjunto de matrices de compatibilidad
    pub fn validate_compatibility_set(&self, set: &CompatibilitySet) -> Result<(), Vec<String>> {
        set.validate()
    }

    /// Validar que todos los IDs referenciados existen en los catálogos
    pub fn validate_references(
        &self,
        set: &CompatibilitySet,
        catalogs: &Catalogs,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validar protagonista × tema
        for (prot_id, theme_map) in &set.protagonist_vs_theme {
            // Verificar que el protagonista existe
            if !catalogs.protagonists.protagonists.contains_key(prot_id) {
                errors.push(format!("Protagonist {} referenced in compatibility but not in catalog", prot_id.0));
            }
            
            for theme_id in theme_map.keys() {
                // Verificar que el tema existe
                if !catalogs.themes.themes.contains_key(theme_id) {
                    errors.push(format!("Theme {} referenced in protagonist compatibility but not in catalog", theme_id.0));
                }
            }
        }

        // Validar escenario × tema
        for (scen_id, theme_map) in &set.scenario_vs_theme {
            if !catalogs.scenarios.scenarios.contains_key(scen_id) {
                errors.push(format!("Scenario {} referenced in compatibility but not in catalog", scen_id.0));
            }
            
            for theme_id in theme_map.keys() {
                if !catalogs.themes.themes.contains_key(theme_id) {
                    errors.push(format!("Theme {} referenced in scenario compatibility but not in catalog", theme_id.0));
                }
            }
        }

        // Validar antagonista × tema
        for (ant_id, theme_map) in &set.antagonist_vs_theme {
            if !catalogs.antagonists.antagonists.contains_key(ant_id) {
                errors.push(format!("Antagonist {} referenced in compatibility but not in catalog", ant_id.0));
            }
            
            for theme_id in theme_map.keys() {
                if !catalogs.themes.themes.contains_key(theme_id) {
                    errors.push(format!("Theme {} referenced in antagonist compatibility but not in catalog", theme_id.0));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Validador de elementos narrativos unificados
#[derive(Debug, Clone)]
pub struct NarrativeElementsValidator;

impl NarrativeElementsValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validar elementos narrativos
    pub fn validate_narrative_elements(&self, elements: &NarrativeElements) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Validar IDs únicos
        self.validate_unique_ids(elements, &mut errors);
        
        // Validar que todos los themes tienen time_window y act_bias
        self.validate_themes_required_fields(elements, &mut errors);
        
        // Validar que todos los protagonists tienen eligible_profiles o eligible_positions
        self.validate_protagonists_required_fields(elements, &mut errors);
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validar que todos los elementos tienen IDs únicas
    fn validate_unique_ids(&self, elements: &NarrativeElements, errors: &mut Vec<String>) {
        let mut ids = HashSet::new();
        
        for theme in &elements.themes {
            if !ids.insert(&theme.base.id.0) {
                errors.push(format!("Duplicate ID in themes: {}", theme.base.id.0));
            }
        }
        
        for protagonist in &elements.protagonists {
            if !ids.insert(&protagonist.base.id.0) {
                errors.push(format!("Duplicate ID in protagonists: {}", protagonist.base.id.0));
            }
        }
        
        for antagonist in &elements.antagonists {
            if !ids.insert(&antagonist.base.id.0) {
                errors.push(format!("Duplicate ID in antagonists: {}", antagonist.base.id.0));
            }
        }
        
        for secondary in &elements.secondaries {
            if !ids.insert(&secondary.base.id.0) {
                errors.push(format!("Duplicate ID in secondaries: {}", secondary.base.id.0));
            }
        }
        
        for scenario in &elements.scenarios {
            if !ids.insert(&scenario.base.id.0) {
                errors.push(format!("Duplicate ID in scenarios: {}", scenario.base.id.0));
            }
        }
        
        for procedure in &elements.procedures {
            if !ids.insert(&procedure.base.id.0) {
                errors.push(format!("Duplicate ID in procedures: {}", procedure.base.id.0));
            }
        }
        
        for resource in &elements.dramatic_resources {
            if !ids.insert(&resource.base.id.0) {
                errors.push(format!("Duplicate ID in dramatic_resources: {}", resource.base.id.0));
            }
        }
        
        for pressure in &elements.social_pressures {
            if !ids.insert(&pressure.base.id.0) {
                errors.push(format!("Duplicate ID in social_pressures: {}", pressure.base.id.0));
            }
        }
    }

    /// Validar que todos los themes tienen time_window y act_bias
    fn validate_themes_required_fields(&self, elements: &NarrativeElements, errors: &mut Vec<String>) {
        for theme in &elements.themes {
            if theme.base.time_window.is_empty() {
                errors.push(format!("Theme {} has no time_window", theme.base.id.0));
            }
            if theme.base.act_bias.is_empty() {
                errors.push(format!("Theme {} has no act_bias", theme.base.id.0));
            }
        }
    }

    /// Validar que todos los protagonists tienen eligible_profiles o eligible_positions
    fn validate_protagonists_required_fields(&self, elements: &NarrativeElements, errors: &mut Vec<String>) {
        for protagonist in &elements.protagonists {
            if protagonist.eligible_profiles.is_empty() && protagonist.eligible_positions.is_empty() {
                errors.push(format!(
                    "Protagonist {} has no eligible_profiles or eligible_positions",
                    protagonist.base.id.0
                ));
            }
        }
    }
}

/// Validador de instancias de evento
#[derive(Debug, Clone)]
pub struct EventInstanceValidator;

impl EventInstanceValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validar una instancia de evento
    pub fn validate_event_instance(&self, event: &EventInstance, catalogs: &Catalogs) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validar que el tema existe
        if !catalogs.themes.themes.contains_key(&event.main_theme) {
            errors.push(format!("Theme {} not found in catalog", event.main_theme.0));
        }

        // Validar que el protagonista existe
        if !catalogs.protagonists.protagonists.contains_key(&event.protagonist) {
            errors.push(format!("Protagonist {} not found in catalog", event.protagonist.0));
        }

        // Validar que el escenario existe
        if !catalogs.scenarios.scenarios.contains_key(&event.scenario) {
            errors.push(format!("Scenario {} not found in catalog", event.scenario.0));
        }

        // Validar que el procedimiento existe
        if !catalogs.procedures.procedures.contains_key(&event.procedure) {
            errors.push(format!("Procedure {} not found in catalog", event.procedure.0));
        }

        // Validar secundarios
        for sec_id in &event.secondaries {
            if !catalogs.secondaries.secondaries.contains_key(sec_id) {
                errors.push(format!("Secondary {} not found in catalog", sec_id.0));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Validador completo
#[derive(Debug, Clone)]
pub struct FullValidator {
    catalog_validator: CatalogValidator,
    compatibility_validator: CompatibilityValidator,
    narrative_elements_validator: NarrativeElementsValidator,
    event_validator: EventInstanceValidator,
}

impl FullValidator {
    pub fn new() -> Self {
        Self {
            catalog_validator: CatalogValidator::new(),
            compatibility_validator: CompatibilityValidator::new(),
            narrative_elements_validator: NarrativeElementsValidator::new(),
            event_validator: EventInstanceValidator::new(),
        }
    }

    /// Validar todo el sistema
    pub fn validate_all(
        &self,
        catalogs: &Catalogs,
        compatibility_set: &CompatibilitySet,
    ) -> Result<(), Vec<String>> {
        let mut all_errors = Vec::new();

        // Validar catálogos
        if let Err(errors) = self.catalog_validator.validate_catalogs(catalogs) {
            all_errors.extend(errors);
        }

        // Validar matrices de compatibilidad
        if let Err(errors) = self.compatibility_validator.validate_compatibility_set(compatibility_set) {
            all_errors.extend(errors);
        }

        // Validar referencias
        if let Err(errors) = self.compatibility_validator.validate_references(compatibility_set, catalogs) {
            all_errors.extend(errors);
        }

        if all_errors.is_empty() {
            Ok(())
        } else {
            Err(all_errors)
        }
    }

    /// Validar elementos narrativos
    pub fn validate_narrative_elements(&self, elements: &NarrativeElements) -> Result<(), Vec<String>> {
        self.narrative_elements_validator.validate_narrative_elements(elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_catalog_validator() {
        let validator = CatalogValidator::new();
        
        let mut catalogs = Catalogs::new();
        
        // Añadir tema válido
        let mut theme = Theme::new("tema_valido");
        theme.base.time_window = vec![TimeSlice::Y1810];
        theme.base.act_bias = vec![Act::Act1];
        catalogs.themes.add(theme);
        
        // Añadir tema inválido (sin time_window)
        let mut invalid_theme = Theme::new("tema_invalido");
        invalid_theme.base.time_window = vec![];
        catalogs.themes.add(invalid_theme);
        
        let result = validator.validate_catalogs(&catalogs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("tema_invalido")));
    }

    #[test]
    fn test_compatibility_validator() {
        let validator = CompatibilityValidator::new();
        
        let mut set = CompatibilitySet::new();
        
        // Añadir puntuación válida
        let mut prot_map = HashMap::new();
        prot_map.insert(ThemeId("tema_1".to_string()), 5);
        set.protagonist_vs_theme.insert(ProtagonistId("prot_1".to_string()), prot_map);
        
        // Añadir puntuación inválida (> 5)
        let mut invalid_prot_map = HashMap::new();
        invalid_prot_map.insert(ThemeId("tema_2".to_string()), 6);
        set.protagonist_vs_theme.insert(ProtagonistId("prot_2".to_string()), invalid_prot_map);
        
        let result = validator.validate_compatibility_set(&set);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("invalid score")));
    }

    #[test]
    fn test_event_instance_validator() {
        let validator = EventInstanceValidator::new();
        let catalogs = Catalogs::new();
        
        let mut event = EventInstance::new("event_test");
        event.main_theme = ThemeId("tema_inexistente".to_string());
        
        let result = validator.validate_event_instance(&event, &catalogs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("not found in catalog")));
    }

    #[test]
    fn test_narrative_elements_validator() {
        let validator = NarrativeElementsValidator::new();
        
        let mut elements = NarrativeElements::new();
        elements = elements.with_theme(Theme::new("theme_1"));
        elements = elements.with_theme(Theme::new("theme_2"));
        
        let result = validator.validate_narrative_elements(&elements);
        assert!(result.is_ok());
        
        // Test con IDs duplicados
        let mut elements_with_dup = NarrativeElements::new();
        elements_with_dup = elements_with_dup.with_theme(Theme::new("id_1"));
        elements_with_dup = elements_with_dup.with_theme(Theme::new("id_1"));
        
        let result = validator.validate_narrative_elements(&elements_with_dup);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Duplicate ID")));
    }

    #[test]
    fn test_full_validator() {
        let validator = FullValidator::new();
        let catalogs = Catalogs::new();
        let set = CompatibilitySet::new();
        
        let result = validator.validate_all(&catalogs, &set);
        assert!(result.is_ok());
    }
}
