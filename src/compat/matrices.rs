//! Matrices de compatibilidad

use crate::domain::enums::*;
use crate::domain::ids::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Conjunto completo de matrices de compatibilidad
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompatibilitySet {
    /// protagonista × tema
    pub protagonist_vs_theme: HashMap<ProtagonistId, HashMap<ThemeId, CompatibilityScore>>,
    /// escenario × tema
    pub scenario_vs_theme: HashMap<ScenarioId, HashMap<ThemeId, CompatibilityScore>>,
    /// antagonista × tema
    pub antagonist_vs_theme: HashMap<AntagonistId, HashMap<ThemeId, CompatibilityScore>>,
    /// secundario × tema
    pub secondary_vs_theme: HashMap<SecondaryId, HashMap<ThemeId, CompatibilityScore>>,
    /// procedimiento × tema
    pub procedure_vs_theme: HashMap<ProcedureId, HashMap<ThemeId, CompatibilityScore>>,
}

impl CompatibilitySet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Obtener puntuación de compatibilidad protagonista-tema
    pub fn get_protagonist_theme_score(&self, protagonist_id: &ProtagonistId, theme_id: &ThemeId) -> CompatibilityScore {
        self.protagonist_vs_theme
            .get(protagonist_id)
            .and_then(|m| m.get(theme_id))
            .copied()
            .unwrap_or(0)
    }

    /// Obtener puntuación de compatibilidad escenario-tema
    pub fn get_scenario_theme_score(&self, scenario_id: &ScenarioId, theme_id: &ThemeId) -> CompatibilityScore {
        self.scenario_vs_theme
            .get(scenario_id)
            .and_then(|m| m.get(theme_id))
            .copied()
            .unwrap_or(0)
    }

    /// Obtener puntuación de compatibilidad antagonista-tema
    pub fn get_antagonist_theme_score(&self, antagonist_id: &AntagonistId, theme_id: &ThemeId) -> CompatibilityScore {
        self.antagonist_vs_theme
            .get(antagonist_id)
            .and_then(|m| m.get(theme_id))
            .copied()
            .unwrap_or(0)
    }

    /// Obtener puntuación de compatibilidad secundario-tema
    pub fn get_secondary_theme_score(&self, secondary_id: &SecondaryId, theme_id: &ThemeId) -> CompatibilityScore {
        self.secondary_vs_theme
            .get(secondary_id)
            .and_then(|m| m.get(theme_id))
            .copied()
            .unwrap_or(0)
    }

    /// Obtener puntuación de compatibilidad procedimiento-tema
    pub fn get_procedure_theme_score(&self, procedure_id: &ProcedureId, theme_id: &ThemeId) -> CompatibilityScore {
        self.procedure_vs_theme
            .get(procedure_id)
            .and_then(|m| m.get(theme_id))
            .copied()
            .unwrap_or(0)
    }

    /// Validar todas las puntuaciones de compatibilidad (0-5)
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for (prot_id, theme_map) in &self.protagonist_vs_theme {
            for (theme_id, score) in theme_map {
                if *score > 5 {
                    errors.push(format!(
                        "protagonist_vs_theme: {}->{} has invalid score {}",
                        prot_id.0, theme_id.0, score
                    ));
                }
            }
        }

        for (scen_id, theme_map) in &self.scenario_vs_theme {
            for (theme_id, score) in theme_map {
                if *score > 5 {
                    errors.push(format!(
                        "scenario_vs_theme: {}->{} has invalid score {}",
                        scen_id.0, theme_id.0, score
                    ));
                }
            }
        }

        for (ant_id, theme_map) in &self.antagonist_vs_theme {
            for (theme_id, score) in theme_map {
                if *score > 5 {
                    errors.push(format!(
                        "antagonist_vs_theme: {}->{} has invalid score {}",
                        ant_id.0, theme_id.0, score
                    ));
                }
            }
        }

        for (sec_id, theme_map) in &self.secondary_vs_theme {
            for (theme_id, score) in theme_map {
                if *score > 5 {
                    errors.push(format!(
                        "secondary_vs_theme: {}->{} has invalid score {}",
                        sec_id.0, theme_id.0, score
                    ));
                }
            }
        }

        for (proc_id, theme_map) in &self.procedure_vs_theme {
            for (theme_id, score) in theme_map {
                if *score > 5 {
                    errors.push(format!(
                        "procedure_vs_theme: {}->{} has invalid score {}",
                        proc_id.0, theme_id.0, score
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Obtener el umbral mínimo de compatibilidad (default: 3)
    pub fn get_minimum_threshold(&self) -> CompatibilityScore {
        3
    }

    /// Verificar si un tema cumple el umbral mínimo con un protagonista
    pub fn theme_meets_protagonist_threshold(&self, protagonist_id: &ProtagonistId, theme_id: &ThemeId) -> bool {
        self.get_protagonist_theme_score(protagonist_id, theme_id) >= self.get_minimum_threshold()
    }

    /// Verificar si un tema cumple el umbral mínimo con un escenario
    pub fn theme_meets_scenario_threshold(&self, scenario_id: &ScenarioId, theme_id: &ThemeId) -> bool {
        self.get_scenario_theme_score(scenario_id, theme_id) >= self.get_minimum_threshold()
    }
}

/// Cargador de matrices de compatibilidad desde YAML
#[derive(Debug, Clone)]
pub struct CompatibilityLoader;

impl CompatibilityLoader {
    pub fn new() -> Self {
        Self
    }

    /// Cargar matriz protagonista × tema desde YAML
    pub fn load_protagonist_vs_theme(data: &HashMap<String, HashMap<String, u8>>) -> HashMap<ProtagonistId, HashMap<ThemeId, CompatibilityScore>> {
        data.iter()
            .map(|(prot_id, theme_map)| {
                let prot_id = ProtagonistId(prot_id.clone());
                let theme_map = theme_map.iter()
                    .map(|(theme_id, score)| (ThemeId(theme_id.clone()), *score))
                    .collect();
                (prot_id, theme_map)
            })
            .collect()
    }

    /// Cargar matriz escenario × tema desde YAML
    pub fn load_scenario_vs_theme(data: &HashMap<String, HashMap<String, u8>>) -> HashMap<ScenarioId, HashMap<ThemeId, CompatibilityScore>> {
        data.iter()
            .map(|(scen_id, theme_map)| {
                let scen_id = ScenarioId(scen_id.clone());
                let theme_map = theme_map.iter()
                    .map(|(theme_id, score)| (ThemeId(theme_id.clone()), *score))
                    .collect();
                (scen_id, theme_map)
            })
            .collect()
    }

    /// Cargar matriz antagonista × tema desde YAML
    pub fn load_antagonist_vs_theme(data: &HashMap<String, HashMap<String, u8>>) -> HashMap<AntagonistId, HashMap<ThemeId, CompatibilityScore>> {
        data.iter()
            .map(|(ant_id, theme_map)| {
                let ant_id = AntagonistId(ant_id.clone());
                let theme_map = theme_map.iter()
                    .map(|(theme_id, score)| (ThemeId(theme_id.clone()), *score))
                    .collect();
                (ant_id, theme_map)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compatibility_set_creation() {
        let set = CompatibilitySet::new();
        assert!(set.protagonist_vs_theme.is_empty());
        assert!(set.scenario_vs_theme.is_empty());
    }

    #[test]
    fn test_compatibility_set_get_scores() {
        let mut set = CompatibilitySet::new();
        
        let mut prot_map = HashMap::new();
        prot_map.insert(ThemeId("tema_1".to_string()), 5);
        prot_map.insert(ThemeId("tema_2".to_string()), 3);
        set.protagonist_vs_theme.insert(ProtagonistId("prot_1".to_string()), prot_map);
        
        assert_eq!(
            set.get_protagonist_theme_score(&ProtagonistId("prot_1".to_string()), &ThemeId("tema_1".to_string())),
            5
        );
        assert_eq!(
            set.get_protagonist_theme_score(&ProtagonistId("prot_1".to_string()), &ThemeId("tema_2".to_string())),
            3
        );
        // Tema no presente
        assert_eq!(
            set.get_protagonist_theme_score(&ProtagonistId("prot_1".to_string()), &ThemeId("tema_3".to_string())),
            0
        );
    }

    #[test]
    fn test_compatibility_set_validation() {
        let mut set = CompatibilitySet::new();
        
        let mut prot_map = HashMap::new();
        prot_map.insert(ThemeId("tema_1".to_string()), 6); // Invalido: > 5
        set.protagonist_vs_theme.insert(ProtagonistId("prot_1".to_string()), prot_map);
        
        let result = set.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("invalid score 6")));
    }

    #[test]
    fn test_compatibility_set_thresholds() {
        let mut set = CompatibilitySet::new();
        
        let mut prot_map = HashMap::new();
        prot_map.insert(ThemeId("tema_1".to_string()), 4);
        prot_map.insert(ThemeId("tema_2".to_string()), 2);
        set.protagonist_vs_theme.insert(ProtagonistId("prot_1".to_string()), prot_map);
        
        assert!(set.theme_meets_protagonist_threshold(
            &ProtagonistId("prot_1".to_string()),
            &ThemeId("tema_1".to_string())
        ));
        assert!(!set.theme_meets_protagonist_threshold(
            &ProtagonistId("prot_1".to_string()),
            &ThemeId("tema_2".to_string())
        ));
    }

    #[test]
    fn test_compatibility_loader() {
        let mut data = HashMap::new();
        let mut theme_map = HashMap::new();
        theme_map.insert("tema_1".to_string(), 5);
        theme_map.insert("tema_2".to_string(), 4);
        data.insert("prot_1".to_string(), theme_map);
        
        let result = CompatibilityLoader::load_protagonist_vs_theme(&data);
        assert_eq!(result.len(), 1);
        assert!(result.contains_key(&ProtagonistId("prot_1".to_string())));
    }
}
