//! Estado de relaciones

use crate::domain::enums::*;
use crate::domain::ids::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Estado de relación entre el protagonista y una entidad
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relationship {
    pub entity_id: ElementId,
    pub level: RelationshipLevel,
    pub state: RelationshipState,
    pub strength: i16,
    pub history: Vec<RelationshipEvent>,
}

impl Relationship {
    pub fn new(entity_id: impl Into<ElementId>) -> Self {
        Self {
            entity_id: entity_id.into(),
            level: RelationshipLevel::Unknown,
            state: RelationshipState::Stable,
            strength: 0,
            history: vec![],
        }
    }

    pub fn with_level(mut self, level: RelationshipLevel) -> Self {
        self.level = level;
        self
    }

    pub fn with_state(mut self, state: RelationshipState) -> Self {
        self.state = state;
        self
    }

    pub fn with_strength(mut self, strength: i16) -> Self {
        self.strength = strength;
        self
    }

    pub fn record_event(&mut self, event: RelationshipEvent) {
        self.history.push(event);
    }
}

/// Evento en la historia de una relación
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelationshipEvent {
    pub event_type: RelationshipEventType,
    pub description: String,
    pub journey: u32,
    pub strength_delta: i16,
}

impl RelationshipEvent {
    pub fn new(event_type: RelationshipEventType, description: impl Into<String>) -> Self {
        Self {
            event_type,
            description: description.into(),
            journey: 0,
            strength_delta: 0,
        }
    }

    pub fn with_journey(mut self, journey: u32) -> Self {
        self.journey = journey;
        self
    }

    pub fn with_strength_delta(mut self, delta: i16) -> Self {
        self.strength_delta = delta;
        self
    }
}

/// Tipo de evento en una relación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipEventType {
    FirstContact,
    SuccessfulCollaboration,
    Betrayal,
    FavorGranted,
    FavorReceived,
    PublicSupport,
    PublicOpposition,
    PrivateMeeting,
    GiftGiven,
    GiftReceived,
}

/// Gestor de relaciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipManager {
    pub relationships: HashMap<ElementId, Relationship>,
}

impl Default for RelationshipManager {
    fn default() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }
}

impl RelationshipManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_relationship(&self, entity_id: &ElementId) -> Option<&Relationship> {
        self.relationships.get(entity_id)
    }

    pub fn get_relationship_mut(&mut self, entity_id: &ElementId) -> Option<&mut Relationship> {
        self.relationships.get_mut(entity_id)
    }

    pub fn add_relationship(&mut self, entity_id: impl Into<ElementId> + Clone) -> &mut Relationship {
        let id = entity_id.into();
        self.relationships.entry(id.clone()).or_insert_with(|| {
            Relationship::new(id)
        })
    }

    pub fn remove_relationship(&mut self, entity_id: &ElementId) -> Option<Relationship> {
        self.relationships.remove(entity_id)
    }

    pub fn get_strength(&self, entity_id: &ElementId) -> i16 {
        self.relationships.get(entity_id).map(|r| r.strength).unwrap_or(0)
    }

    pub fn get_level(&self, entity_id: &ElementId) -> RelationshipLevel {
        self.relationships.get(entity_id).map(|r| r.level).unwrap_or(RelationshipLevel::Unknown)
    }

    pub fn apply_delta(&mut self, entity_id: &ElementId, level_delta: Option<RelationshipLevel>, state_delta: Option<RelationshipState>, strength_delta: i16) {
        if let Some(rel) = self.relationships.get_mut(entity_id) {
            if let Some(level) = level_delta {
                rel.level = level;
            }
            if let Some(state) = state_delta {
                rel.state = state;
            }
            rel.strength = (rel.strength + strength_delta).clamp(-100, 100);
        }
    }

    pub fn record_interaction(&mut self, entity_id: &ElementId, event_type: RelationshipEventType, description: impl Into<String>, strength_delta: i16) {
        let entity_id_clone = entity_id.clone();
        let relationship = self.relationships.entry(entity_id_clone.clone()).or_insert_with(|| {
            Relationship::new(&entity_id_clone)
        });
        
        let event = RelationshipEvent::new(event_type, description)
            .with_strength_delta(strength_delta);
        
        relationship.record_event(event);
        relationship.strength = (relationship.strength + strength_delta).clamp(-100, 100);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_creation() {
        let rel = Relationship::new("entity_1")
            .with_level(RelationshipLevel::Ally)
            .with_strength(75);
        
        assert_eq!(rel.entity_id.0, "entity_1");
        assert_eq!(rel.level, RelationshipLevel::Ally);
        assert_eq!(rel.strength, 75);
    }

    #[test]
    fn test_relationship_event() {
        let event = RelationshipEvent::new(RelationshipEventType::FirstContact, "First meeting")
            .with_journey(10)
            .with_strength_delta(5);
        
        assert_eq!(event.strength_delta, 5);
        assert_eq!(event.journey, 10);
    }

    #[test]
    fn test_relationship_manager() {
        let mut manager = RelationshipManager::new();
        
        {
            let rel = manager.add_relationship("faction_1");
            rel.level = RelationshipLevel::Ally;
        }
        
        assert!(manager.get_relationship(&ElementId("faction_1".to_string())).is_some());
        assert_eq!(manager.get_level(&ElementId("faction_1".to_string())), RelationshipLevel::Ally);
        
        manager.apply_delta(
            &ElementId("faction_1".to_string()),
            None,
            Some(RelationshipState::Tense),
            -10
        );
        
        let rel = manager.get_relationship(&ElementId("faction_1".to_string())).unwrap();
        assert_eq!(rel.state, RelationshipState::Tense);
        assert_eq!(rel.strength, -10);
    }

    #[test]
    fn test_record_interaction() {
        let mut manager = RelationshipManager::new();
        
        manager.record_interaction(
            &ElementId("faction_1".to_string()),
            RelationshipEventType::SuccessfulCollaboration,
            "Worked together on a project",
            15
        );
        
        let rel = manager.get_relationship(&ElementId("faction_1".to_string())).unwrap();
        assert_eq!(rel.strength, 15);
        assert_eq!(rel.history.len(), 1);
    }
}
