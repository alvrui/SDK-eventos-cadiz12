//! Catálogo de ThemeBindings (bindings de tema -> secundarios/procedimientos)

use crate::domain::ids::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Bindings de tema (secundarios y procedimientos asociados)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThemeBindings {
    pub secondary_ids: Vec<SecondaryId>,
    pub procedure_ids: Vec<ProcedureId>,
}

impl ThemeBindings {
    pub fn new() -> Self {
        Self {
            secondary_ids: vec![],
            procedure_ids: vec![],
        }
    }

    pub fn with_secondary(mut self, secondary: impl Into<SecondaryId>) -> Self {
        self.secondary_ids.push(secondary.into());
        self
    }

    pub fn with_procedure(mut self, procedure: impl Into<ProcedureId>) -> Self {
        self.procedure_ids.push(procedure.into());
        self
    }
}

/// Catálogo de bindings de tema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeBindingsCatalog {
    pub bindings: HashMap<ThemeId, ThemeBindings>,
}

impl ThemeBindingsCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, theme_id: ThemeId, bindings: ThemeBindings) {
        self.bindings.insert(theme_id, bindings);
    }

    pub fn get(&self, theme_id: &ThemeId) -> Option<&ThemeBindings> {
        self.bindings.get(theme_id)
    }

    pub fn get_secondaries(&self, theme_id: &ThemeId) -> Vec<SecondaryId> {
        self.bindings.get(theme_id)
            .map(|b| b.secondary_ids.clone())
            .unwrap_or_default()
    }

    pub fn get_procedures(&self, theme_id: &ThemeId) -> Vec<ProcedureId> {
        self.bindings.get(theme_id)
            .map(|b| b.procedure_ids.clone())
            .unwrap_or_default()
    }

    pub fn contains(&self, theme_id: &ThemeId) -> bool {
        self.bindings.contains_key(theme_id)
    }
}

/// Struct para datos de compatibilidad de tema (para carga desde YAML)
#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeCompatibilityData {
    pub protagonist_compat: Option<HashMap<String, u8>>,
    pub scenario_compat: Option<HashMap<String, u8>>,
    pub antagonist_compat: Option<HashMap<String, u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_bindings_catalog() {
        let mut catalog = ThemeBindingsCatalog::new();
        
        let theme_id = ThemeId("tema_test".to_string());
        let bindings = ThemeBindings::new()
            .with_secondary("sec_1")
            .with_secondary("sec_2")
            .with_procedure("proc_1");
        
        catalog.add(theme_id.clone(), bindings);
        
        assert!(catalog.contains(&theme_id));
        assert_eq!(catalog.get_secondaries(&theme_id).len(), 2);
        assert_eq!(catalog.get_procedures(&theme_id).len(), 1);
    }

    #[test]
    fn test_theme_bindings_empty() {
        let catalog = ThemeBindingsCatalog::new();
        let theme_id = ThemeId("tema_inexistente".to_string());
        
        assert!(!catalog.contains(&theme_id));
        assert!(catalog.get(&theme_id).is_none());
        assert!(catalog.get_secondaries(&theme_id).is_empty());
        assert!(catalog.get_procedures(&theme_id).is_empty());
    }
}
