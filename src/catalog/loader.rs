//! Cargador de catálogos desde archivos YAML/JSON

use crate::catalog::{Catalogs, ThemeBindingsCatalog};
use crate::catalog::catalog::{ThemeCatalog, ProtagonistCatalog, AntagonistCatalog, SecondaryCatalog, ScenarioCatalog, ProcedureCatalog};
use crate::catalog::theme_bindings::ThemeBindings;
use crate::domain::enums::*;
use crate::domain::ids::*;
use crate::domain::structs::*;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Estructura para cargar temas desde YAML
#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeData {
    pub id: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub historical_scope: Option<String>,
    pub time_window: Vec<String>,
    pub act_bias: Vec<String>,
    pub faction_vectors: Option<Vec<String>>,
    pub space_vectors: Option<Vec<String>>,
    pub stakes_axis: Option<Vec<String>>,
    pub tone: Option<String>,
    pub chain_roles: Option<Vec<String>>,
    pub repeatability: Option<String>,
    pub meter_affinity: Option<Vec<String>>,
    pub visibility_profile: Option<String>,
    pub information_profile: Option<String>,
    pub compatibility_tags: Option<Vec<String>>,
    pub blocking_tags: Option<Vec<String>>,
    pub unlock_tags: Option<Vec<String>>,
    pub generated_tags: Option<Vec<String>>,
}

impl ThemeData {
    pub fn to_theme(&self) -> Result<Theme> {
        let mut base = ScriptElementBase::new(&self.id);
        
        if let Some(label) = &self.label {
            base = base.with_label(label);
        }
        if let Some(description) = &self.description {
            base = base.with_description(description);
        }
        if let Some(scope) = &self.historical_scope {
            base.historical_scope = scope.parse::<HistoricalScope>()?;
        }
        
        base.time_window = self.time_window.iter()
            .map(|s| s.parse::<TimeSlice>().unwrap_or(TimeSlice::Y1810))
            .collect();
        
        base.act_bias = self.act_bias.iter()
            .map(|s| s.parse::<Act>().unwrap_or(Act::Act1))
            .collect();
        
        if let Some(factions) = &self.faction_vectors {
            base.faction_vectors = factions.iter().map(|s| FactionId(s.clone())).collect();
        }
        if let Some(spaces) = &self.space_vectors {
            base.space_vectors = spaces.iter().map(|s| SpaceId(s.clone())).collect();
        }
        if let Some(stakes) = &self.stakes_axis {
            base.stakes_axis = stakes.iter()
                .map(|s| s.parse::<StakesAxis>().unwrap_or(StakesAxis::Political))
                .collect();
        }
        if let Some(tone) = &self.tone {
            base.tone = tone.parse::<Tone>()?;
        }
        if let Some(roles) = &self.chain_roles {
            base.chain_roles = roles.iter()
                .map(|s| s.parse::<ChainRole>().unwrap_or(ChainRole::Seed))
                .collect();
        }
        if let Some(repeat) = &self.repeatability {
            base.repeatability = repeat.parse::<Repeatability>()?;
        }
        if let Some(meters) = &self.meter_affinity {
            base.meter_affinity = meters.iter()
                .map(|s| s.parse::<MeterType>().unwrap_or(MeterType::Influence))
                .collect();
        }
        if let Some(visibility) = &self.visibility_profile {
            base.visibility_profile = visibility.parse::<VisibilityProfile>()?;
        }
        if let Some(info) = &self.information_profile {
            base.information_profile = info.parse::<InformationProfile>()?;
        }
        if let Some(tags) = &self.compatibility_tags {
            base.compatibility_tags = tags.iter().map(|s| TagId(s.clone())).collect();
        }
        if let Some(tags) = &self.blocking_tags {
            base.blocking_tags = tags.iter().map(|s| TagId(s.clone())).collect();
        }
        if let Some(tags) = &self.unlock_tags {
            base.unlock_tags = tags.iter().map(|s| TagId(s.clone())).collect();
        }
        if let Some(tags) = &self.generated_tags {
            base.generated_tags = tags.iter().map(|s| TagId(s.clone())).collect();
        }
        
        Ok(Theme {
            base,
            category: ScriptElementCategory::ThemeEvent,
        })
    }
}

/// Cargador de catálogos
#[derive(Debug, Clone)]
pub struct CatalogLoader {
    data_dir: PathBuf,
}

impl CatalogLoader {
    pub fn new(data_dir: impl Into<PathBuf>) -> Self {
        Self {
            data_dir: data_dir.into(),
        }
    }

    /// Cargar todos los catálogos desde el directorio de datos
    pub fn load_all(&self) -> Result<Catalogs> {
        let mut catalogs = Catalogs::new();
        
        // Cargar temas
        if self.data_dir.join("themes.yaml").exists() {
            catalogs.themes = self.load_themes()?;
        }
        
        // Cargar protagonistas
        if self.data_dir.join("protagonists.yaml").exists() {
            catalogs.protagonists = self.load_protagonists()?;
        }
        
        // Cargar antagonistas
        if self.data_dir.join("antagonists.yaml").exists() {
            catalogs.antagonists = self.load_antagonists()?;
        }
        
        // Cargar secundarios
        if self.data_dir.join("secondaries.yaml").exists() {
            catalogs.secondaries = self.load_secondaries()?;
        }
        
        // Cargar escenarios
        if self.data_dir.join("scenarios.yaml").exists() {
            catalogs.scenarios = self.load_scenarios()?;
        }
        
        // Cargar procedimientos
        if self.data_dir.join("procedures.yaml").exists() {
            catalogs.procedures = self.load_procedures()?;
        }
        
        // Cargar theme bindings
        if self.data_dir.join("bindings_theme.yaml").exists() {
            catalogs.theme_bindings = self.load_theme_bindings()?;
        }
        
        Ok(catalogs)
    }

    pub fn load_themes(&self) -> Result<ThemeCatalog> {
        let path = self.data_dir.join("themes.yaml");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read themes file: {:?}", path))?;
        
        let themes_data: Vec<ThemeData> = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse themes YAML: {:?}", path))?;
        
        let mut catalog = ThemeCatalog::new();
        for data in themes_data {
            let theme = data.to_theme()?;
            catalog.add(theme);
        }
        
        Ok(catalog)
    }

    pub fn load_protagonists(&self) -> Result<ProtagonistCatalog> {
        let path = self.data_dir.join("protagonists.yaml");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read protagonists file: {:?}", path))?;
        
        let protagonists_data: Vec<ProtagonistData> = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse protagonists YAML: {:?}", path))?;
        
        let mut catalog = ProtagonistCatalog::new();
        for data in protagonists_data {
            let protagonist = data.to_protagonist()?;
            catalog.add(protagonist);
        }
        
        Ok(catalog)
    }

    pub fn load_antagonists(&self) -> Result<AntagonistCatalog> {
        let path = self.data_dir.join("antagonists.yaml");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read antagonists file: {:?}", path))?;
        
        let antagonists_data: Vec<AntagonistData> = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse antagonists YAML: {:?}", path))?;
        
        let mut catalog = AntagonistCatalog::new();
        for data in antagonists_data {
            let antagonist = data.to_antagonist()?;
            catalog.add(antagonist);
        }
        
        Ok(catalog)
    }

    pub fn load_secondaries(&self) -> Result<SecondaryCatalog> {
        let path = self.data_dir.join("secondaries.yaml");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read secondaries file: {:?}", path))?;
        
        let secondaries_data: Vec<SecondaryData> = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse secondaries YAML: {:?}", path))?;
        
        let mut catalog = SecondaryCatalog::new();
        for data in secondaries_data {
            let secondary = data.to_secondary()?;
            catalog.add(secondary);
        }
        
        Ok(catalog)
    }

    pub fn load_scenarios(&self) -> Result<ScenarioCatalog> {
        let path = self.data_dir.join("scenarios.yaml");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read scenarios file: {:?}", path))?;
        
        let scenarios_data: Vec<ScenarioData> = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse scenarios YAML: {:?}", path))?;
        
        let mut catalog = ScenarioCatalog::new();
        for data in scenarios_data {
            let scenario = data.to_scenario()?;
            catalog.add(scenario);
        }
        
        Ok(catalog)
    }

    pub fn load_procedures(&self) -> Result<ProcedureCatalog> {
        let path = self.data_dir.join("procedures.yaml");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read procedures file: {:?}", path))?;
        
        let procedures_data: Vec<ProcedureData> = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse procedures YAML: {:?}", path))?;
        
        let mut catalog = ProcedureCatalog::new();
        for data in procedures_data {
            let procedure = data.to_procedure()?;
            catalog.add(procedure);
        }
        
        Ok(catalog)
    }

    pub fn load_theme_bindings(&self) -> Result<ThemeBindingsCatalog> {
        let path = self.data_dir.join("bindings_theme.yaml");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read theme bindings file: {:?}", path))?;
        
        let bindings_data: HashMap<String, ThemeBindingsData> = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse theme bindings YAML: {:?}", path))?;
        
        let mut catalog = ThemeBindingsCatalog::new();
        for (theme_id, data) in bindings_data {
            let bindings = data.to_theme_bindings(&theme_id)?;
            catalog.add(ThemeId(theme_id), bindings);
        }
        
        Ok(catalog)
    }
}

/// Datos para cargar protagonistas
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtagonistData {
    pub id: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub eligible_profiles: Option<Vec<String>>,
    pub eligible_positions: Option<Vec<String>>,
    // Campos de ScriptElementBase
    pub historical_scope: Option<String>,
    pub time_window: Option<Vec<String>>,
    pub act_bias: Option<Vec<String>>,
    // ... otros campos opcionales
}

impl ProtagonistData {
    pub fn to_protagonist(&self) -> Result<ProtagonistArchetype> {
        let mut base = ScriptElementBase::new(&self.id);
        
        if let Some(label) = &self.label {
            base = base.with_label(label);
        }
        if let Some(description) = &self.description {
            base = base.with_description(description);
        }
        
        let eligible_profiles = self.eligible_profiles.clone()
            .unwrap_or_default()
            .into_iter()
            .map(PlayerProfileRef)
            .collect();
        
        let eligible_positions = self.eligible_positions.clone()
            .unwrap_or_default()
            .into_iter()
            .map(FormalPositionId)
            .collect();
        
        Ok(ProtagonistArchetype {
            base,
            eligible_profiles,
            eligible_positions,
        })
    }
}

/// Datos para cargar antagonistas
#[derive(Debug, Deserialize, Serialize)]
pub struct AntagonistData {
    pub id: String,
    pub label: Option<String>,
    pub description: Option<String>,
}

impl AntagonistData {
    pub fn to_antagonist(&self) -> Result<Antagonist> {
        let mut base = ScriptElementBase::new(&self.id);
        
        if let Some(label) = &self.label {
            base = base.with_label(label);
        }
        if let Some(description) = &self.description {
            base = base.with_description(description);
        }
        
        Ok(Antagonist { base })
    }
}

/// Datos para cargar secundarios
#[derive(Debug, Deserialize, Serialize)]
pub struct SecondaryData {
    pub id: String,
    pub label: Option<String>,
    pub description: Option<String>,
}

impl SecondaryData {
    pub fn to_secondary(&self) -> Result<Secondary> {
        let mut base = ScriptElementBase::new(&self.id);
        
        if let Some(label) = &self.label {
            base = base.with_label(label);
        }
        if let Some(description) = &self.description {
            base = base.with_description(description);
        }
        
        Ok(Secondary { base })
    }
}

/// Datos para cargar escenarios
#[derive(Debug, Deserialize, Serialize)]
pub struct ScenarioData {
    pub id: String,
    pub label: Option<String>,
    pub description: Option<String>,
}

impl ScenarioData {
    pub fn to_scenario(&self) -> Result<Scenario> {
        let mut base = ScriptElementBase::new(&self.id);
        
        if let Some(label) = &self.label {
            base = base.with_label(label);
        }
        if let Some(description) = &self.description {
            base = base.with_description(description);
        }
        
        Ok(Scenario { base })
    }
}

/// Datos para cargar procedimientos
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcedureData {
    pub id: String,
    pub kind: String,
    pub label: Option<String>,
    pub description: Option<String>,
}

impl ProcedureData {
    pub fn to_procedure(&self) -> Result<Procedure> {
        let mut base = ScriptElementBase::new(&self.id);
        
        if let Some(label) = &self.label {
            base = base.with_label(label);
        }
        if let Some(description) = &self.description {
            base = base.with_description(description);
        }
        
        let kind = self.kind.parse::<ProcedureKind>()
            .map_err(|_| anyhow::anyhow!("Invalid procedure kind: {}", self.kind))?;
        
        Ok(Procedure { base, kind })
    }
}

/// Datos para cargar theme bindings
#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeBindingsData {
    pub secondary_ids: Option<Vec<String>>,
    pub procedure_ids: Option<Vec<String>>,
}

impl ThemeBindingsData {
    pub fn to_theme_bindings(&self, _theme_id: &str) -> Result<ThemeBindings> {
        let secondaries = self.secondary_ids.clone()
            .unwrap_or_default()
            .into_iter()
            .map(SecondaryId)
            .collect();
        
        let procedures = self.procedure_ids.clone()
            .unwrap_or_default()
            .into_iter()
            .map(ProcedureId)
            .collect();
        
        Ok(ThemeBindings {
            secondary_ids: secondaries,
            procedure_ids: procedures,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_themes_from_yaml() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("themes.yaml");
        
        let yaml_content = r#"
- id: tema_test
  label: Test_Theme
  description: A test theme
  time_window: [Y1810, Y1811]
  act_bias: [Act1, Act2]
"#;
        std::fs::write(&path, yaml_content).unwrap();
        
        let loader = CatalogLoader::new(dir.path());
        let catalog = loader.load_themes().unwrap();
        
        assert!(catalog.contains(&ThemeId("tema_test".to_string())));
        let theme = catalog.get(&ThemeId("tema_test".to_string())).unwrap();
        assert_eq!(theme.base.label, "Test_Theme");
        assert_eq!(theme.base.time_window.len(), 2);
    }

    #[test]
    fn test_theme_data_to_theme() {
        let data = ThemeData {
            id: "tema_1".to_string(),
            label: Some("Tema_Uno".to_string()),
            description: Some("Descripcion".to_string()),
            historical_scope: Some("plausible_documented".to_string()),
            time_window: vec!["Y1810".to_string(), "Y1811".to_string()],
            act_bias: vec!["Act1".to_string()],
            faction_vectors: None,
            space_vectors: None,
            stakes_axis: None,
            tone: None,
            chain_roles: None,
            repeatability: None,
            meter_affinity: None,
            visibility_profile: None,
            information_profile: None,
            compatibility_tags: None,
            blocking_tags: None,
            unlock_tags: None,
            generated_tags: None,
        };
        
        let theme = data.to_theme().unwrap();
        assert_eq!(theme.base.id.0, "tema_1");
        assert_eq!(theme.base.label, "Tema_Uno");
        assert_eq!(theme.base.time_window.len(), 2);
    }
}
