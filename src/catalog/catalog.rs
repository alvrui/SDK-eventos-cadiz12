//! Catálogos de elementos

use crate::catalog::theme_bindings::ThemeBindingsCatalog;
use crate::domain::enums::*;
use crate::domain::ids::*;
use crate::domain::structs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Catálogo de temas
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeCatalog {
    pub themes: HashMap<ThemeId, Theme>,
}

impl ThemeCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, theme: Theme) {
        self.themes.insert(ThemeId::from(theme.base.id.clone()), theme);
    }

    pub fn get(&self, id: &ThemeId) -> Option<&Theme> {
        self.themes.get(id)
    }

    pub fn contains(&self, id: &ThemeId) -> bool {
        self.themes.contains_key(id)
    }
}

/// Catálogo de protagonistas
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProtagonistCatalog {
    pub protagonists: HashMap<ProtagonistId, ProtagonistArchetype>,
}

impl ProtagonistCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, protagonist: ProtagonistArchetype) {
        self.protagonists.insert(ProtagonistId::from(protagonist.base.id.clone()), protagonist);
    }

    pub fn get(&self, id: &ProtagonistId) -> Option<&ProtagonistArchetype> {
        self.protagonists.get(id)
    }
}

/// Catálogo de antagonistas
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AntagonistCatalog {
    pub antagonists: HashMap<AntagonistId, Antagonist>,
}

impl AntagonistCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, antagonist: Antagonist) {
        self.antagonists.insert(AntagonistId::from(antagonist.base.id.clone()), antagonist);
    }

    pub fn get(&self, id: &AntagonistId) -> Option<&Antagonist> {
        self.antagonists.get(id)
    }
}

/// Catálogo de secundarios
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecondaryCatalog {
    pub secondaries: HashMap<SecondaryId, Secondary>,
}

impl SecondaryCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, secondary: Secondary) {
        self.secondaries.insert(SecondaryId::from(secondary.base.id.clone()), secondary);
    }

    pub fn get(&self, id: &SecondaryId) -> Option<&Secondary> {
        self.secondaries.get(id)
    }
}

/// Catálogo de escenarios
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScenarioCatalog {
    pub scenarios: HashMap<ScenarioId, Scenario>,
}

impl ScenarioCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, scenario: Scenario) {
        self.scenarios.insert(ScenarioId::from(scenario.base.id.clone()), scenario);
    }

    pub fn get(&self, id: &ScenarioId) -> Option<&Scenario> {
        self.scenarios.get(id)
    }
}

/// Catálogo de procedimientos
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcedureCatalog {
    pub procedures: HashMap<ProcedureId, Procedure>,
}

impl ProcedureCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, procedure: Procedure) {
        self.procedures.insert(ProcedureId::from(procedure.base.id.clone()), procedure);
    }

    pub fn get(&self, id: &ProcedureId) -> Option<&Procedure> {
        self.procedures.get(id)
    }
}

/// Catálogo de recursos dramáticos
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DramaticResourceCatalog {
    pub resources: HashMap<ResourceId, DramaticResource>,
}

impl DramaticResourceCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, resource: DramaticResource) {
        self.resources.insert(ResourceId::from(resource.base.id.clone()), resource);
    }

    pub fn get(&self, id: &ResourceId) -> Option<&DramaticResource> {
        self.resources.get(id)
    }
}

/// Catálogo de presiones sociales
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SocialPressureCatalog {
    pub pressures: HashMap<ElementId, SocialPressure>,
}

impl SocialPressureCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, pressure: SocialPressure) {
        self.pressures.insert(pressure.base.id.clone(), pressure);
    }

    pub fn get(&self, id: &ElementId) -> Option<&SocialPressure> {
        self.pressures.get(id)
    }
}

/// Catálogos completos
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Catalogs {
    pub themes: ThemeCatalog,
    pub protagonists: ProtagonistCatalog,
    pub antagonists: AntagonistCatalog,
    pub secondaries: SecondaryCatalog,
    pub scenarios: ScenarioCatalog,
    pub procedures: ProcedureCatalog,
    pub dramatic_resources: DramaticResourceCatalog,
    pub social_pressures: SocialPressureCatalog,
    pub theme_bindings: ThemeBindingsCatalog,
}

impl Catalogs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validar que no hay IDs duplicados
        let mut all_ids = std::collections::HashSet::new();
        
        for (id, _) in &self.themes.themes {
            if !all_ids.insert(&id.0) {
                errors.push(format!("Duplicate ID in themes: {}", id.0));
            }
        }
        
        for (id, _) in &self.protagonists.protagonists {
            if !all_ids.insert(&id.0) {
                errors.push(format!("Duplicate ID in protagonists: {}", id.0));
            }
        }
        
        for (id, _) in &self.antagonists.antagonists {
            if !all_ids.insert(&id.0) {
                errors.push(format!("Duplicate ID in antagonists: {}", id.0));
            }
        }
        
        for (id, _) in &self.secondaries.secondaries {
            if !all_ids.insert(&id.0) {
                errors.push(format!("Duplicate ID in secondaries: {}", id.0));
            }
        }
        
        for (id, _) in &self.scenarios.scenarios {
            if !all_ids.insert(&id.0) {
                errors.push(format!("Duplicate ID in scenarios: {}", id.0));
            }
        }
        
        for (id, _) in &self.procedures.procedures {
            if !all_ids.insert(&id.0) {
                errors.push(format!("Duplicate ID in procedures: {}", id.0));
            }
        }

        // Validar que todos los temas tienen time_window y act_bias
        for (id, theme) in &self.themes.themes {
            if theme.base.time_window.is_empty() {
                errors.push(format!("Theme {} has no time_window", id.0));
            }
            if theme.base.act_bias.is_empty() {
                errors.push(format!("Theme {} has no act_bias", id.0));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalogs_creation() {
        let catalogs = Catalogs::new();
        assert!(catalogs.themes.themes.is_empty());
        assert!(catalogs.protagonists.protagonists.is_empty());
    }

    #[test]
    fn test_theme_catalog_add_get() {
        let mut catalog = ThemeCatalog::new();
        let mut theme = Theme::new("tema_test");
        theme.base.label = "Test Theme".to_string();
        
        catalog.add(theme);
        
        assert!(catalog.contains(&ThemeId("tema_test".to_string())));
        let retrieved = catalog.get(&ThemeId("tema_test".to_string())).unwrap();
        assert_eq!(retrieved.base.label, "Test Theme");
    }

    #[test]
    fn test_catalogs_validation() {
        let mut catalogs = Catalogs::new();
        
        // Añadir tema sin time_window
        let mut theme = Theme::new("tema_invalido");
        theme.base.time_window = vec![];
        theme.base.act_bias = vec![];
        catalogs.themes.add(theme);
        
        let result = catalogs.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("tema_invalido")));
    }

    #[test]
    fn test_catalogs_validation_duplicate_ids() {
        let mut catalogs = Catalogs::new();
        
        // Add theme with id "id_duplicado"
        let theme = Theme::new("tema_1");
        catalogs.themes.add(theme);
        
        // Add protagonist with same id "tema_1" (this checks cross-catalog duplicates)
        let protagonist = ProtagonistArchetype::new("tema_1");
        catalogs.protagonists.add(protagonist);
        
        let result = catalogs.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Duplicate ID")));
    }
}
