//! Cargador de matrices de compatibilidad desde archivos

use crate::compat::matrices::*;
use anyhow::{Context, Result};
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Estructura para cargar matrices desde YAML
#[derive(Debug, Clone)]
pub struct CompatibilitySetLoader {
    data_dir: std::path::PathBuf,
}

impl CompatibilitySetLoader {
    pub fn new(data_dir: impl Into<std::path::PathBuf>) -> Self {
        Self {
            data_dir: data_dir.into(),
        }
    }

    /// Cargar todas las matrices de compatibilidad
    pub fn load_all(&self) -> Result<CompatibilitySet> {
        let mut set = CompatibilitySet::new();
        
        // Cargar protagonista × tema
        if self.data_dir.join("compatibility_protagonist_theme.yaml").exists() {
            let data = self.load_matrix("compatibility_protagonist_theme.yaml")?;
            set.protagonist_vs_theme = CompatibilityLoader::load_protagonist_vs_theme(&data);
        }
        
        // Cargar escenario × tema
        if self.data_dir.join("compatibility_scenario_theme.yaml").exists() {
            let data = self.load_matrix("compatibility_scenario_theme.yaml")?;
            set.scenario_vs_theme = CompatibilityLoader::load_scenario_vs_theme(&data);
        }
        
        // Cargar antagonista × tema
        if self.data_dir.join("compatibility_antagonist_theme.yaml").exists() {
            let data = self.load_matrix("compatibility_antagonist_theme.yaml")?;
            set.antagonist_vs_theme = CompatibilityLoader::load_antagonist_vs_theme(&data);
        }
        
        Ok(set)
    }

    /// Cargar una matriz desde archivo YAML
    pub fn load_matrix(&self, filename: impl AsRef<Path>) -> Result<HashMap<String, HashMap<String, u8>>> {
        let path = self.data_dir.join(filename);
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {:?}", path))?;
        
        let data: HashMap<String, HashMap<String, u8>> = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML file: {:?}", path))?;
        
        Ok(data)
    }

    /// Cargar y convertir a CompatibilitySet
    pub fn load_and_convert(&self) -> Result<CompatibilitySet> {
        let mut set = CompatibilitySet::new();
        
        // Cargar y convertir protagonista × tema
        if self.data_dir.join("compatibility_protagonist_theme.yaml").exists() {
            let data = self.load_matrix("compatibility_protagonist_theme.yaml")?;
            set.protagonist_vs_theme = CompatibilityLoader::load_protagonist_vs_theme(&data);
        }
        
        // Cargar y convertir escenario × tema
        if self.data_dir.join("compatibility_scenario_theme.yaml").exists() {
            let data = self.load_matrix("compatibility_scenario_theme.yaml")?;
            set.scenario_vs_theme = CompatibilityLoader::load_scenario_vs_theme(&data);
        }
        
        // Cargar y convertir antagonista × tema
        if self.data_dir.join("compatibility_antagonist_theme.yaml").exists() {
            let data = self.load_matrix("compatibility_antagonist_theme.yaml")?;
            set.antagonist_vs_theme = CompatibilityLoader::load_antagonist_vs_theme(&data);
        }
        
        Ok(set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::domain::ids::*;

    #[test]
    fn test_load_matrix_from_yaml() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_matrix.yaml");
        
        let yaml_content = r#"
prot_1:
  tema_1: 5
  tema_2: 4
prot_2:
  tema_1: 3
  tema_2: 5
"#;
        fs::write(&path, yaml_content).unwrap();
        
        let loader = CompatibilitySetLoader::new(dir.path());
        let data = loader.load_matrix("test_matrix.yaml").unwrap();
        
        assert_eq!(data.len(), 2);
        assert_eq!(data["prot_1"]["tema_1"], 5);
        assert_eq!(data["prot_2"]["tema_2"], 5);
    }

    #[test]
    fn test_load_and_convert() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("compatibility_protagonist_theme.yaml");
        
        let yaml_content = r#"
prot_1:
  tema_1: 5
  tema_2: 4
"#;
        fs::write(&path, yaml_content).unwrap();
        
        let loader = CompatibilitySetLoader::new(dir.path());
        let set = loader.load_and_convert().unwrap();
        
        assert_eq!(
            set.get_protagonist_theme_score(&ProtagonistId("prot_1".to_string()), &ThemeId("tema_1".to_string())),
            5
        );
    }
}
