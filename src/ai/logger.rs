//! Logger de interacciones con agentes Mistral

use serde_json::Value;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::Local;

/// Logger de interacciones con agentes
#[derive(Debug, Clone)]
pub struct AgentInteractionLogger {
    log_dir: PathBuf,
    enabled: bool,
    max_log_size: usize,  // Max size in bytes before rotating
}

impl AgentInteractionLogger {
    /// Crear nuevo logger
    pub fn new(log_dir: impl AsRef<Path>) -> Self {
        let log_dir = log_dir.as_ref().to_path_buf();
        
        // Crear directorio si no existe
        if let Err(e) = fs::create_dir_all(&log_dir) {
            log::warn!("Failed to create log directory {}: {}", log_dir.display(), e);
        }
        
        Self {
            log_dir,
            enabled: true,
            max_log_size: 10 * 1024 * 1024, // 10MB
        }
    }

    /// Habilitar/deshabilitar logging
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Verificar si el logging está habilitado
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Registrar solicitud a agente
    pub fn log_request(
        &self,
        section: &str,
        action: &str,
        agent: &str,
        prompt: &str,
    ) -> Option<String> {
        if !self.enabled {
            return None;
        }
        
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = self.log_dir.join(format!("agent_{}_{}_{}.log", section, action, timestamp));
        
        let log_entry = format!(
            "[{}] REQUEST\nSection: {}\nAction: {}\nAgent: {}\nPrompt Length: {} bytes\nPrompt:\n{}\n\n",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            section,
            action,
            agent,
            prompt.len(),
            prompt
        );
        
        if let Err(e) = self.write_to_file(&filename, &log_entry) {
            log::error!("Failed to write agent request log: {}", e);
            return None;
        }
        
        Some(filename.to_string_lossy().into_owned())
    }

    /// Registrar respuesta de agente
    pub fn log_response(
        &self,
        filename: &str,
        response: &Value,
        is_success: bool,
        duration_ms: Option<u64>,
    ) {
        if !self.enabled {
            return;
        }
        
        let status = if is_success { "SUCCESS" } else { "ERROR" };
        let duration_str = duration_ms.map(|d| format!(" ({}ms)", d)).unwrap_or_default();
        
        let log_entry = format!(
            "[{}] RESPONSE ({}){}\n{}\n\n",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            status,
            duration_str,
            serde_json::to_string_pretty(response).unwrap_or_else(|_| "Invalid JSON".to_string())
        );
        
        if let Err(e) = self.write_to_file(Path::new(filename), &log_entry) {
            log::error!("Failed to write agent response log: {}", e);
        }
    }

    /// Registrar error
    pub fn log_error(
        &self,
        section: &str,
        action: &str,
        agent: &str,
        error: &str,
        prompt: Option<&str>,
    ) {
        if !self.enabled {
            return;
        }
        
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = self.log_dir.join(format!("error_{}_{}_{}.log", section, action, timestamp));
        
        let mut log_entry = format!(
            "[{}] ERROR\nSection: {}\nAction: {}\nAgent: {}\nError: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            section,
            action,
            agent,
            error
        );
        
        if let Some(prompt) = prompt {
            log_entry.push_str(&format!("\nPrompt:\n{}", prompt));
        }
        
        log_entry.push('\n');
        
        if let Err(e) = self.write_to_file(&filename, &log_entry) {
            log::error!("Failed to write agent error log: {}", e);
        }
    }

    /// Registrar interacción completa (solicitud + respuesta)
    pub fn log_interaction(
        &self,
        section: &str,
        action: &str,
        agent: &str,
        prompt: &str,
        response: &Value,
        is_success: bool,
        duration_ms: Option<u64>,
    ) {
        if !self.enabled {
            return;
        }
        
        let filename = self.log_request(section, action, agent, prompt);
        if let Some(filename) = filename {
            self.log_response(&filename, response, is_success, duration_ms);
        }
    }

    /// Registrar feedback para agente
    pub fn log_feedback(&self, feedback: &crate::ai::feedback::AgentFeedback) {
        if !self.enabled {
            return;
        }
        
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = self.log_dir.join(format!(
            "feedback_{}_{}_{}.log",
            feedback.section, feedback.action, timestamp
        ));
        
        let log_entry = format!(
            "[{}] FEEDBACK\nSection: {}\nAction: {}\nAgent: {}\nErrors: {:?}\nWarnings: {:?}\nSuggestions: {:?}\n\n",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            feedback.section,
            feedback.action,
            feedback.agent,
            feedback.errors,
            feedback.warnings,
            feedback.suggestions
        );
        
        if let Err(e) = self.write_to_file(&filename, &log_entry) {
            log::error!("Failed to write feedback log: {}", e);
        }
    }

    /// Escribir en archivo con rotación si es necesario
    fn write_to_file(&self, filename: &Path, content: &str) -> std::io::Result<()> {
        // Verificar si el archivo existe y es muy grande
        if filename.exists() {
            if let Ok(metadata) = fs::metadata(filename) {
                if metadata.len() >= self.max_log_size as u64 {
                    // Rotar el archivo
                    let backup_filename = format!("{}.old", filename.to_string_lossy());
                    fs::rename(filename, backup_filename)?;
                }
            }
        }
        
        // Abrir archivo en modo append
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;
        
        file.write_all(content.as_bytes())?;
        
        Ok(())
    }

    /// Limpiar logs antiguos (más de N días)
    pub fn cleanup_old_logs(&self, max_age_days: u32) -> std::io::Result<usize> {
        if !self.enabled {
            return Ok(0);
        }
        
        let cutoff = Local::now() - chrono::Duration::days(max_age_days as i64);
        let mut deleted_count = 0;
        
        for entry in fs::read_dir(&self.log_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map(|e| e.to_string_lossy() == "log").unwrap_or(false) {
                if let Ok(modified) = entry.metadata().and_then(|m| m.modified()) {
                    let modified_time = chrono::DateTime::<Local>::from(std::time::SystemTime::from(modified));
                    
                    if modified_time < cutoff {
                        fs::remove_file(&path)?;
                        deleted_count += 1;
                        log::info!("Deleted old log file: {}", path.display());
                    }
                }
            }
        }
        
        Ok(deleted_count)
    }

    /// Obtener lista de archivos de log
    pub fn list_log_files(&self) -> std::io::Result<Vec<String>> {
        let mut files = Vec::new();
        
        for entry in fs::read_dir(&self.log_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map(|e| e.to_string_lossy() == "log").unwrap_or(false) {
                files.push(path.to_string_lossy().into_owned());
            }
        }
        
        files.sort();
        Ok(files)
    }
}

impl Default for AgentInteractionLogger {
    fn default() -> Self {
        Self::new("logs/agents")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;

    #[test]
    fn test_logger_creation() {
        let dir = tempdir().unwrap();
        let logger = AgentInteractionLogger::new(dir.path());
        assert!(logger.is_enabled());
    }

    #[test]
    fn test_log_request() {
        let dir = tempdir().unwrap();
        let logger = AgentInteractionLogger::new(dir.path());
        
        let filename = logger.log_request("narrative", "generate", "CoordinadorNarrativo", "Test prompt");
        assert!(filename.is_some());
        
        let files = logger.list_log_files().unwrap();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_log_response() {
        let dir = tempdir().unwrap();
        let logger = AgentInteractionLogger::new(dir.path());
        
        let filename = logger.log_request("narrative", "generate", "CoordinadorNarrativo", "Test prompt");
        assert!(filename.is_some());
        
        let response = json!({
            "status": "success",
            "section": "narrative",
            "action": "generate",
            "data": {}
        });
        
        logger.log_response(&filename.unwrap(), &response, true, Some(100));
        
        let files = logger.list_log_files().unwrap();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_log_error() {
        let dir = tempdir().unwrap();
        let logger = AgentInteractionLogger::new(dir.path());
        
        logger.log_error("narrative", "generate", "CoordinadorNarrativo", "Test error", Some("Test prompt"));
        
        let files = logger.list_log_files().unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].contains("error"));
    }

    #[test]
    fn test_log_interaction() {
        let dir = tempdir().unwrap();
        let logger = AgentInteractionLogger::new(dir.path());
        
        let response = json!({
            "status": "success",
            "section": "narrative",
            "action": "generate",
            "data": {}
        });
        
        logger.log_interaction(
            "narrative",
            "generate",
            "CoordinadorNarrativo",
            "Test prompt",
            &response,
            true,
            Some(100),
        );
        
        let files = logger.list_log_files().unwrap();
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_disabled_logger() {
        let dir = tempdir().unwrap();
        let mut logger = AgentInteractionLogger::new(dir.path());
        logger.set_enabled(false);
        
        let filename = logger.log_request("narrative", "generate", "CoordinadorNarrativo", "Test prompt");
        assert!(filename.is_none());
        
        logger.log_error("narrative", "generate", "CoordinadorNarrativo", "Test error", None);
        
        let files = logger.list_log_files().unwrap();
        assert_eq!(files.len(), 0);
    }
}
