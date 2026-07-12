//! Estado del protagonista

use crate::domain::enums::*;
use crate::domain::ids::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::world_state::WorldState;

/// Medidores del protagonista
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtagonistMeters {
    pub influence: f32,
    pub relational_capital: f32,
    pub reputation: f32,
    pub coherence: f32,
    pub resources: f32,
    pub stamina: f32,
}

impl Default for ProtagonistMeters {
    fn default() -> Self {
        Self {
            influence: 50.0,
            relational_capital: 50.0,
            reputation: 50.0,
            coherence: 70.0,
            resources: 50.0,
            stamina: 80.0,
        }
    }
}

impl ProtagonistMeters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, meter_type: MeterType) -> f32 {
        match meter_type {
            MeterType::Influence => self.influence,
            MeterType::RelationalCapital => self.relational_capital,
            MeterType::Reputation => self.reputation,
            MeterType::Coherence => self.coherence,
            MeterType::Resources => self.resources,
            MeterType::Stamina => self.stamina,
        }
    }

    pub fn set(&mut self, meter_type: MeterType, value: f32) {
        match meter_type {
            MeterType::Influence => self.influence = value,
            MeterType::RelationalCapital => self.relational_capital = value,
            MeterType::Reputation => self.reputation = value,
            MeterType::Coherence => self.coherence = value,
            MeterType::Resources => self.resources = value,
            MeterType::Stamina => self.stamina = value,
        }
    }

    pub fn apply_delta(&mut self, delta: &crate::domain::structs::MeterDelta) {
        let new_value = self.get(delta.meter_type) + delta.delta;
        self.set(delta.meter_type, new_value.clamp(0.0, 100.0));
    }
}

/// Estado de relación con una entidad
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityRelationship {
    pub level: RelationshipLevel,
    pub state: RelationshipState,
    pub strength: i16,
}

impl Default for EntityRelationship {
    fn default() -> Self {
        Self {
            level: RelationshipLevel::Contact,
            state: RelationshipState::Stable,
            strength: 0,
        }
    }
}

impl EntityRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Estado del protagonista
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtagonistState {
    pub protagonist_id: ProtagonistId,
    pub profile: PlayerProfileRef,
    pub formal_position: Option<FormalPositionId>,
    pub public_visibility: PublicVisibilityLevel,
    pub moral_trajectory: MoralTrajectory,
    pub meters: ProtagonistMeters,
    pub relationships: HashMap<ElementId, EntityRelationship>,
    pub active_tags: Vec<TagId>,
    pub memory_recent_themes: Vec<ThemeId>,
}

impl Default for ProtagonistState {
    fn default() -> Self {
        Self {
            protagonist_id: ProtagonistId("default".to_string()),
            profile: PlayerProfileRef("default".to_string()),
            formal_position: None,
            public_visibility: PublicVisibilityLevel::Emerging,
            moral_trajectory: MoralTrajectory::Coherent,
            meters: ProtagonistMeters::new(),
            relationships: HashMap::new(),
            active_tags: vec![],
            memory_recent_themes: vec![],
        }
    }
}

impl ProtagonistState {
    pub fn new(protagonist_id: impl Into<ProtagonistId>) -> Self {
        Self {
            protagonist_id: protagonist_id.into(),
            ..Default::default()
        }
    }

    pub fn with_profile(mut self, profile: impl Into<PlayerProfileRef>) -> Self {
        self.profile = profile.into();
        self
    }

    pub fn with_visibility(mut self, visibility: PublicVisibilityLevel) -> Self {
        self.public_visibility = visibility;
        self
    }

    pub fn with_moral_trajectory(mut self, trajectory: MoralTrajectory) -> Self {
        self.moral_trajectory = trajectory;
        self
    }

    pub fn get_meter(&self, meter_type: MeterType) -> f32 {
        self.meters.get(meter_type)
    }

    pub fn get_relationship(&self, entity_id: &ElementId) -> Option<&EntityRelationship> {
        self.relationships.get(entity_id)
    }

    pub fn add_tag(&mut self, tag: impl Into<TagId> + Clone) {
        let tag_id: TagId = tag.into();
        if !self.active_tags.contains(&tag_id) {
            self.active_tags.push(tag_id);
        }
    }

    pub fn has_tag(&self, tag: &TagId) -> bool {
        self.active_tags.contains(tag)
    }

    pub fn record_theme(&mut self, theme_id: impl Into<ThemeId>) {
        self.memory_recent_themes.push(theme_id.into());
        // Mantener solo los últimos 10 temas
        if self.memory_recent_themes.len() > 10 {
            self.memory_recent_themes.remove(0);
        }
    }

    pub fn has_recent_theme(&self, theme_id: &ThemeId) -> bool {
        self.memory_recent_themes.contains(theme_id)
    }
}

/// Contexto completo para selección de eventos (mundo + protagonista)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullGameContext {
    pub world_state: WorldState,
    pub protagonist_state: ProtagonistState,
    pub current_journey: u32,
    pub active_events: Vec<ElementId>,
}

impl FullGameContext {
    pub fn new(
        world_state: WorldState,
        protagonist_state: ProtagonistState,
        current_journey: u32,
        active_events: Vec<ElementId>,
    ) -> Self {
        Self {
            world_state,
            protagonist_state,
            current_journey,
            active_events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protagonist_meters_default() {
        let meters = ProtagonistMeters::new();
        assert_eq!(meters.influence, 50.0);
        assert_eq!(meters.stamina, 80.0);
    }

    #[test]
    fn test_protagonist_meters_get_set() {
        let mut meters = ProtagonistMeters::new();
        assert_eq!(meters.get(MeterType::Influence), 50.0);
        
        meters.set(MeterType::Influence, 75.0);
        assert_eq!(meters.get(MeterType::Influence), 75.0);
    }

    #[test]
    fn test_protagonist_meters_apply_delta() {
        use crate::domain::structs::MeterDelta;
        
        let mut meters = ProtagonistMeters::new();
        let delta = MeterDelta::new(MeterType::Influence, 10.0);
        meters.apply_delta(&delta);
        assert_eq!(meters.get(MeterType::Influence), 60.0);
        
        // Test clamping
        let delta = MeterDelta::new(MeterType::Influence, 200.0);
        meters.apply_delta(&delta);
        assert_eq!(meters.get(MeterType::Influence), 100.0);
    }

    #[test]
    fn test_protagonist_state_creation() {
        let state = ProtagonistState::new("prot_1")
            .with_profile("joven_tribuno")
            .with_visibility(PublicVisibilityLevel::RecognizableFigure);
        
        assert_eq!(state.protagonist_id.0, "prot_1");
        assert_eq!(state.profile.0, "joven_tribuno");
        assert_eq!(state.public_visibility, PublicVisibilityLevel::RecognizableFigure);
    }

    #[test]
    fn test_protagonist_tags() {
        let mut state = ProtagonistState::new("prot_1");
        state.add_tag("liberal");
        state.add_tag("progresista");
        
        assert!(state.has_tag(&TagId("liberal".to_string())));
        assert!(!state.has_tag(&TagId("conservador".to_string())));
    }

    #[test]
    fn test_protagonist_recent_themes() {
        let mut state = ProtagonistState::new("prot_1");
        state.record_theme("tema_1");
        state.record_theme("tema_2");
        
        assert!(state.has_recent_theme(&ThemeId("tema_1".to_string())));
        assert!(state.has_recent_theme(&ThemeId("tema_2".to_string())));
    }
}
