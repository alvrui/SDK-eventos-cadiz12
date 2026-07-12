//! Templates module - Plantillas de escena A-F

use crate::domain::enums::*;
use crate::domain::ids::*;
use crate::domain::structs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Definición de una plantilla de escena
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneTemplate {
    pub id: SceneTemplateId,
    pub template_type: SceneTemplateType,
    pub description: String,
    pub functions: Vec<SceneFunction>,
    pub mechanical_properties: Vec<MechanicalProperty>,
    pub narrative_properties: Vec<NarrativeProperty>,
    pub meter_effects: Vec<MeterDelta>,
    pub relationship_effects: Vec<RelationshipDelta>,
    pub default_consequences: EventOutcomePrototype,
}

impl SceneTemplate {
    pub fn new(id: impl Into<SceneTemplateId>, template_type: SceneTemplateType) -> Self {
        Self {
            id: id.into(),
            template_type,
            description: String::new(),
            functions: vec![],
            mechanical_properties: vec![],
            narrative_properties: vec![],
            meter_effects: vec![],
            relationship_effects: vec![],
            default_consequences: EventOutcomePrototype::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_function(mut self, function: SceneFunction) -> Self {
        self.functions.push(function);
        self
    }
}

/// Función de una escena
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SceneFunction {
    InstitutionalDecisionMaking,
    UrbanInteraction,
    PrivateNegotiation,
    InformationAcquisition,
    PublicConflictResolution,
    PersonalConsequenceHandling,
    PoliticalTension,
    WorldBuilding,
    CharacterDevelopment,
    HighStakes,
    MoralDilemma,
}

/// Propiedad mecánica
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MechanicalProperty {
    RequiresFormalPosition,
    RequiresResources,
    TimeSensitive,
    FactionSpecific,
    Repeatable,
}

/// Propiedad narrativa
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NarrativeProperty {
    HighStakes,
    CharacterDevelopment,
    WorldBuilding,
    MoralDilemma,
    PoliticalTension,
}

/// Catálogo de plantillas de escena
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SceneTemplateCatalog {
    pub templates: HashMap<SceneTemplateId, SceneTemplate>,
}

impl SceneTemplateCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, template: SceneTemplate) {
        self.templates.insert(template.id.clone(), template);
    }

    pub fn get(&self, id: &SceneTemplateId) -> Option<&SceneTemplate> {
        self.templates.get(id)
    }

    /// Obtener plantilla por tipo
    pub fn get_by_type(&self, template_type: SceneTemplateType) -> Option<&SceneTemplate> {
        self.templates.values()
            .find(|t| t.template_type == template_type)
    }

    /// Crear las plantillas A-F por defecto
    pub fn create_default_templates() -> Self {
        let mut catalog = Self::new();

        // Plantilla A: Sesión Institucional
        let template_a = SceneTemplate::new("A", SceneTemplateType::AInstitutionalSession)
            .with_description("Sesión formal en una institución (Cortes, Comisión, etc.)")
            .with_function(SceneFunction::InstitutionalDecisionMaking)
            .with_function(SceneFunction::PoliticalTension);
        catalog.add(template_a);

        // Plantilla B: Encuentro Urbano
        let template_b = SceneTemplate::new("B", SceneTemplateType::BUrbanEncounter)
            .with_description("Interacción en espacios públicos (calles, plazas, mercados)")
            .with_function(SceneFunction::UrbanInteraction)
            .with_function(SceneFunction::WorldBuilding);
        catalog.add(template_b);

        // Plantilla C: Visita Privada
        let template_c = SceneTemplate::new("C", SceneTemplateType::CPrivateVisit)
            .with_description("Reunión en privado (casa, café, oficina)")
            .with_function(SceneFunction::PrivateNegotiation)
            .with_function(SceneFunction::CharacterDevelopment);
        catalog.add(template_c);

        // Plantilla D: Lectura de Documento
        let template_d = SceneTemplate::new("D", SceneTemplateType::DDocumentReading)
            .with_description("Acceso a información a través de documentos")
            .with_function(SceneFunction::InformationAcquisition);
        catalog.add(template_d);

        // Plantilla E: Crisis Pública
        let template_e = SceneTemplate::new("E", SceneTemplateType::EPublicCrisis)
            .with_description("Evento de crisis que afecta a la comunidad")
            .with_function(SceneFunction::PublicConflictResolution)
            .with_function(SceneFunction::HighStakes);
        catalog.add(template_e);

        // Plantilla F: Consecuencia Personal
        let template_f = SceneTemplate::new("F", SceneTemplateType::FPersonalConsequence)
            .with_description("Impacto personal en el protagonista")
            .with_function(SceneFunction::PersonalConsequenceHandling)
            .with_function(SceneFunction::MoralDilemma);
        catalog.add(template_f);

        catalog
    }
}

/// Factory para crear plantillas
#[derive(Debug, Clone)]
pub struct SceneTemplateFactory {
    catalog: SceneTemplateCatalog,
}

impl SceneTemplateFactory {
    pub fn new() -> Self {
        Self {
            catalog: SceneTemplateCatalog::create_default_templates(),
        }
    }

    pub fn create_template(&self, template_type: SceneTemplateType) -> Option<SceneTemplate> {
        self.catalog.get_by_type(template_type).cloned()
    }

    pub fn has_templates(&self) -> bool {
        !self.catalog.templates.is_empty()
    }

    /// Aplicar efectos de la plantilla a una instancia de evento
    pub fn apply_template_effects(
        &self,
        template_type: SceneTemplateType,
        event: &mut EventInstance,
    ) -> Result<(), String> {
        if let Some(template) = self.catalog.get_by_type(template_type) {
            // Añadir efectos de medidores
            event.expected_meter_effects.extend(template.meter_effects.clone());
            
            // Añadir tags generados
            // (Por ahora, no hacemos nada especial con los tags)
            
            Ok(())
        } else {
            Err(format!("Template type {:?} not found", template_type))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_template_creation() {
        let template = SceneTemplate::new("A", SceneTemplateType::AInstitutionalSession)
            .with_description("Sesión institucional");
        
        assert_eq!(template.id.0, "A");
        assert_eq!(template.template_type, SceneTemplateType::AInstitutionalSession);
        assert_eq!(template.description, "Sesión institucional");
    }

    #[test]
    fn test_scene_template_catalog_default() {
        let catalog = SceneTemplateCatalog::create_default_templates();
        
        assert_eq!(catalog.templates.len(), 6); // A-F
        
        // Verificar que existe la plantilla A
        assert!(catalog.get(&SceneTemplateId("A".to_string())).is_some());
        assert!(catalog.get_by_type(SceneTemplateType::AInstitutionalSession).is_some());
    }

    #[test]
    fn test_scene_template_factory() {
        let factory = SceneTemplateFactory::new();
        
        let template = factory.create_template(SceneTemplateType::AInstitutionalSession);
        assert!(template.is_some());
        assert_eq!(template.unwrap().template_type, SceneTemplateType::AInstitutionalSession);
    }

    #[test]
    fn test_apply_template_effects() {
        let factory = SceneTemplateFactory::new();
        let mut event = EventInstance::new("event_test");
        
        let result = factory.apply_template_effects(SceneTemplateType::AInstitutionalSession, &mut event);
        assert!(result.is_ok());
    }
}
