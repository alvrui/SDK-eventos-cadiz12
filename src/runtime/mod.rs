//! Runtime module - Ejecución y aplicación de eventos

use crate::domain::enums::*;
use crate::domain::ids::*;
use crate::domain::structs::*;
use crate::state::protagonist_state::ProtagonistState;
use crate::state::world_state::WorldState;
use crate::templates::SceneTemplateFactory;
use serde::{Deserialize, Serialize};

/// Ejecutor de eventos
#[derive(Debug, Clone)]
pub struct EventRuntime {
    template_factory: SceneTemplateFactory,
}

impl EventRuntime {
    pub fn new() -> Self {
        Self {
            template_factory: SceneTemplateFactory::new(),
        }
    }

    /// Aplicar efectos de un evento al estado del mundo y protagonista
    pub fn apply_event_effects(
        &self,
        event: &EventInstance,
        protagonist_state: &mut ProtagonistState,
        world_state: &mut WorldState,
    ) -> EventOutcome {
        let mut outcome = EventOutcome::new(event.id.clone());

        // Aplicar efectos a medidores del protagonista
        for delta in &event.expected_meter_effects {
            let old_value = protagonist_state.get_meter(delta.meter_type);
            protagonist_state.meters.apply_delta(delta);
            let new_value = protagonist_state.get_meter(delta.meter_type);
            
            outcome.meter_changes.push(MeterChange {
                meter_type: delta.meter_type,
                old_value,
                new_value,
                delta: delta.delta,
            });
        }

        // Aplicar efectos de relación
        for _delta in &event.trace.final_score_breakdown {
            // Esto es un placeholder - debería procesar relationship_deltas
        }

        // Registrar tema en memoria del protagonista
        protagonist_state.record_theme(event.main_theme.clone());

        // Añadir tags generados al protagonista
        for tag in &event.generated_tags {
            protagonist_state.add_tag(tag.clone());
        }

        // Incrementar jornada
        world_state.absolute_journey += 1;

        outcome
    }

    /// Procesar una instancia de evento por completo
    pub fn process_event(
        &self,
        event: &EventInstance,
        protagonist_state: &mut ProtagonistState,
        world_state: &mut WorldState,
    ) -> Result<EventOutcome, String> {
        // Validar evento
        self.validate_event(event)?;

        // Aplicar efectos
        let outcome = self.apply_event_effects(event, protagonist_state, world_state);

        Ok(outcome)
    }

    /// Validar una instancia de evento
    pub fn validate_event(&self, event: &EventInstance) -> Result<(), String> {
        // Verificar que el tema existe
        // (Esto debería verificarse contra el catálogo)
        
        // Verificar que el protagonista existe
        if event.protagonist.0.is_empty() {
            return Err("Event has no protagonist".to_string());
        }

        // Verificar que el escenario existe
        // Verificar que el procedimiento existe
        
        Ok(())
    }

    /// Crear un resultado de evento
    pub fn create_outcome(
        &self,
        event: &EventInstance,
        protagonist_state: &ProtagonistState,
        world_state: &WorldState,
    ) -> EventOutcome {
        let mut outcome = EventOutcome::new(event.id.clone());
        
        // Llenar con información del evento
        outcome.event_id = event.id.clone();
        outcome.theme_id = Some(event.main_theme.clone());
        outcome.scene_template = Some(event.scene_template);
        outcome.journey = world_state.absolute_journey;
        
        // Calcular cambios esperados
        for delta in &event.expected_meter_effects {
            let current_value = protagonist_state.get_meter(delta.meter_type);
            outcome.meter_changes.push(MeterChange {
                meter_type: delta.meter_type,
                old_value: current_value,
                new_value: (current_value + delta.delta).clamp(0.0, 100.0),
                delta: delta.delta,
            });
        }
        
        outcome
    }
}

/// Resultado de la ejecución de un evento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOutcome {
    pub id: EventInstanceId,
    pub event_id: EventInstanceId,
    pub theme_id: Option<ThemeId>,
    pub scene_template: Option<SceneTemplateType>,
    pub journey: u32,
    pub meter_changes: Vec<MeterChange>,
    pub relationship_changes: Vec<RelationshipChange>,
    pub tags_added: Vec<TagId>,
    pub tags_removed: Vec<TagId>,
    pub elements_unlocked: Vec<ElementId>,
    pub elements_locked: Vec<ElementId>,
    pub success: bool,
    pub message: Option<String>,
}

impl EventOutcome {
    pub fn new(event_id: impl Into<EventInstanceId> + Clone) -> Self {
        let event_id_converted = event_id.into();
        Self {
            id: EventInstanceId(format!("outcome_{}", event_id_converted.0)),
            event_id: event_id_converted,
            theme_id: None,
            scene_template: None,
            journey: 0,
            meter_changes: vec![],
            relationship_changes: vec![],
            tags_added: vec![],
            tags_removed: vec![],
            elements_unlocked: vec![],
            elements_locked: vec![],
            success: true,
            message: None,
        }
    }
}

/// Cambio en un medidor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeterChange {
    pub meter_type: MeterType,
    pub old_value: f32,
    pub new_value: f32,
    pub delta: f32,
}

/// Cambio en una relación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipChange {
    pub entity_id: ElementId,
    pub old_level: RelationshipLevel,
    pub new_level: RelationshipLevel,
    pub old_state: RelationshipState,
    pub new_state: RelationshipState,
    pub strength_delta: i16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_runtime_creation() {
        let runtime = EventRuntime::new();
        assert!(runtime.template_factory.has_templates());
    }

    #[test]
    fn test_apply_event_effects() {
        let runtime = EventRuntime::new();
        
        let mut event = EventInstance::new("event_test");
        event.expected_meter_effects = vec![
            MeterDelta::new(MeterType::Influence, 10.0),
            MeterDelta::new(MeterType::Reputation, 5.0),
        ];
        event.generated_tags = vec![TagId("test_tag".to_string())];
        event.main_theme = ThemeId("test_theme".to_string());
        
        let mut protagonist_state = ProtagonistState::new("prot_1");
        let mut world_state = WorldState::new();
        
        let outcome = runtime.apply_event_effects(&event, &mut protagonist_state, &mut world_state);
        
        assert_eq!(outcome.meter_changes.len(), 2);
        assert!(protagonist_state.has_tag(&TagId("test_tag".to_string())));
        assert!(protagonist_state.has_recent_theme(&ThemeId("test_theme".to_string())));
        assert_eq!(world_state.absolute_journey, 2); // Se incrementó
    }

    #[test]
    fn test_validate_event() {
        let runtime = EventRuntime::new();
        
        let mut event = EventInstance::new("event_test");
        event.protagonist = ProtagonistId("prot_1".to_string());
        
        let result = runtime.validate_event(&event);
        assert!(result.is_ok());
        
        let mut invalid_event = EventInstance::new("event_invalid");
        // Set empty protagonist to trigger validation error
        invalid_event.protagonist = ProtagonistId("".to_string());
        let result = runtime.validate_event(&invalid_event);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_outcome() {
        let runtime = EventRuntime::new();
        
        let mut event = EventInstance::new("event_test");
        event.main_theme = ThemeId("test_theme".to_string());
        event.scene_template = SceneTemplateType::AInstitutionalSession;
        event.expected_meter_effects = vec![
            MeterDelta::new(MeterType::Influence, 10.0),
        ];
        
        let protagonist_state = ProtagonistState::new("prot_1");
        let world_state = WorldState::new();
        
        let outcome = runtime.create_outcome(&event, &protagonist_state, &world_state);
        
        assert_eq!(outcome.event_id.0, "event_test");
        assert_eq!(outcome.theme_id, Some(ThemeId("test_theme".to_string())));
        assert_eq!(outcome.scene_template, Some(SceneTemplateType::AInstitutionalSession));
        assert_eq!(outcome.meter_changes.len(), 1);
    }
}
