//! Estado del mundo

use crate::domain::enums::*;
use crate::domain::ids::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Estado de una facción
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FactionState {
    pub force: u8,
    pub need_from_player: u8,
    pub internal_cohesion: u8,
    pub red_line_active: bool,
    pub surveillance_over_player: u8,
}

impl Default for FactionState {
    fn default() -> Self {
        Self {
            force: 50,
            need_from_player: 50,
            internal_cohesion: 70,
            red_line_active: false,
            surveillance_over_player: 10,
        }
    }
}

impl FactionState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Estado de un espacio
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpaceState {
    pub climate: SpaceClimate,
    pub occupied_by: Vec<FactionId>,
    pub accessibility: u8,
}

impl Default for SpaceState {
    fn default() -> Self {
        Self {
            climate: SpaceClimate::Calm,
            occupied_by: vec![],
            accessibility: 100,
        }
    }
}

impl SpaceState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Crisis activa
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveCrisis {
    pub crisis_id: ElementId,
    pub phase: CrisisPhase,
    pub intensity: u8,
    pub involved_factions: Vec<FactionId>,
}

impl ActiveCrisis {
    pub fn new(crisis_id: impl Into<ElementId>) -> Self {
        Self {
            crisis_id: crisis_id.into(),
            phase: CrisisPhase::Signal,
            intensity: 50,
            involved_factions: vec![],
        }
    }
}

/// Estado global del mundo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorldState {
    pub tram_id: String,
    pub narrative_act: u8,
    pub absolute_journey: u32,
    pub distance_to_next_pivot: u32,
    pub global_state: GlobalState,
    pub polarization: u8,
    pub board_visibility: u8,
    pub hot_topic_id: Option<ElementId>,
    pub factions: HashMap<FactionId, FactionState>,
    pub spaces: HashMap<SpaceId, SpaceState>,
    pub current_scenario: Option<ScenarioId>,
    pub crisis: Option<ActiveCrisis>,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            tram_id: "default".to_string(),
            narrative_act: 1,
            absolute_journey: 1,
            distance_to_next_pivot: 100,
            global_state: GlobalState::TenseNormality,
            polarization: 50,
            board_visibility: 70,
            hot_topic_id: None,
            factions: HashMap::new(),
            spaces: HashMap::new(),
            current_scenario: None,
            crisis: None,
        }
    }
}

impl WorldState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tram_id(mut self, tram_id: impl Into<String>) -> Self {
        self.tram_id = tram_id.into();
        self
    }

    pub fn with_narrative_act(mut self, act: u8) -> Self {
        self.narrative_act = act;
        self
    }

    pub fn with_journey(mut self, journey: u32) -> Self {
        self.absolute_journey = journey;
        self
    }

    pub fn with_global_state(mut self, state: GlobalState) -> Self {
        self.global_state = state;
        self
    }

    pub fn add_faction(mut self, faction_id: FactionId, state: FactionState) -> Self {
        self.factions.insert(faction_id, state);
        self
    }

    pub fn add_space(mut self, space_id: SpaceId, state: SpaceState) -> Self {
        self.spaces.insert(space_id, state);
        self
    }

    /// Obtiene el año aproximado basado en la jornada
    pub fn get_approximate_year(&self) -> u32 {
        match self.absolute_journey {
            0..=100 => 1810,
            101..=200 => 1811,
            201..=300 => 1812,
            301..=400 => 1813,
            401..=500 => 1814,
            _ => 1812,
        }
    }

    /// Obtiene la ventana temporal basada en la jornada
    pub fn get_time_slice(&self) -> TimeSlice {
        match self.absolute_journey {
            0..=100 => TimeSlice::Y1810,
            101..=200 => TimeSlice::Y1811,
            201..=300 => TimeSlice::Y1812,
            301..=400 => TimeSlice::Y1813,
            401..=500 => TimeSlice::Y1814,
            _ => TimeSlice::Y1812,
        }
    }
}

/// Contexto de juego para selección de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameContext {
    pub world_state: WorldState,
    pub current_journey: u32,
    pub active_events: Vec<ElementId>,
}

impl GameContext {
    pub fn new(world_state: WorldState, current_journey: u32, active_events: Vec<ElementId>) -> Self {
        Self {
            world_state,
            current_journey,
            active_events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_state_default() {
        let state = WorldState::new();
        assert_eq!(state.narrative_act, 1);
        assert_eq!(state.global_state, GlobalState::TenseNormality);
        assert!(state.factions.is_empty());
    }

    #[test]
    fn test_world_state_with_faction() {
        let mut state = WorldState::new();
        let faction_id = FactionId("liberal".to_string());
        let faction_state = FactionState::new();
        state = state.add_faction(faction_id.clone(), faction_state);
        
        assert!(state.factions.contains_key(&faction_id));
    }

    #[test]
    fn test_world_state_time_slice() {
        let state = WorldState::new().with_journey(150);
        assert_eq!(state.get_time_slice(), TimeSlice::Y1811);
        
        let state = WorldState::new().with_journey(250);
        assert_eq!(state.get_time_slice(), TimeSlice::Y1812);
    }

    #[test]
    fn test_game_context_creation() {
        let world = WorldState::new();
        let context = GameContext::new(world, 100, vec![]);
        assert_eq!(context.current_journey, 100);
    }
}
