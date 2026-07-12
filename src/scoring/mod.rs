//! Scoring module - Cálculo de puntuaciones

use crate::compat::matrices::CompatibilitySet;
use crate::domain::enums::*;
use crate::domain::ids::*;
use crate::domain::structs::*;
use crate::state::world_state::WorldState;
use crate::state::protagonist_state::ProtagonistState;
use crate::trace::{ThemeCandidateTrace, ScoreBreakdown};

/// Calculador de puntuaciones para selección de eventos
#[derive(Debug, Clone)]
pub struct ScoreCalculator {
    compatibility_set: CompatibilitySet,
    weights: SelectionWeights,
}

impl ScoreCalculator {
    pub fn new(compatibility_set: CompatibilitySet, weights: Option<SelectionWeights>) -> Self {
        Self {
            compatibility_set,
            weights: weights.unwrap_or_default(),
        }
    }

    /// Calcular puntuación total para un tema dado el contexto
    pub fn calculate_theme_score(
        &self,
        theme: &Theme,
        protagonist_state: &ProtagonistState,
        world_state: &WorldState,
        scenario_id: &ScenarioId,
    ) -> ThemeCandidateTrace {
        let mut trace = ThemeCandidateTrace::new(ThemeId::from(theme.base.id.clone()), 0.0);
        let mut score_breakdown = Vec::new();

        // Puntuación de protagonista
        let prot_score = self.calculate_protagonist_score(theme, protagonist_state);
        let prot_weighted = prot_score * self.weights.w_protagonist;
        score_breakdown.push(ScoreBreakdown::new("protagonist", prot_score, self.weights.w_protagonist));
        trace.reasons.push(format!("Protagonist score: {:.2}", prot_score));

        // Puntuación de escenario
        let scen_score = self.calculate_scenario_score(theme, scenario_id);
        let scen_weighted = scen_score * self.weights.w_scenario;
        score_breakdown.push(ScoreBreakdown::new("scenario", scen_score, self.weights.w_scenario));
        trace.reasons.push(format!("Scenario score: {:.2}", scen_score));

        // Puntuación de tiempo
        let time_score = self.calculate_time_score(theme, world_state);
        let time_weighted = time_score * self.weights.w_time;
        score_breakdown.push(ScoreBreakdown::new("time", time_score, self.weights.w_time));
        trace.reasons.push(format!("Time fit score: {:.2}", time_score));

        // Puntuación de acto
        let act_score = self.calculate_act_score(theme, world_state);
        let act_weighted = act_score * self.weights.w_act;
        score_breakdown.push(ScoreBreakdown::new("act", act_score, self.weights.w_act));
        trace.reasons.push(format!("Act fit score: {:.2}", act_score));

        // Puntuación de facción
        let faction_score = self.calculate_faction_score(theme, protagonist_state);
        let faction_weighted = faction_score * self.weights.w_faction;
        score_breakdown.push(ScoreBreakdown::new("faction", faction_score, self.weights.w_faction));
        trace.reasons.push(format!("Faction fit score: {:.2}", faction_score));

        // Puntuación de estado
        let state_score = self.calculate_state_score(theme, world_state);
        let state_weighted = state_score * self.weights.w_state;
        score_breakdown.push(ScoreBreakdown::new("state", state_score, self.weights.w_state));
        trace.reasons.push(format!("State fit score: {:.2}", state_score));

        // Puntuación de novedad
        let novelty_score = self.calculate_novelty_score(theme, protagonist_state);
        let novelty_weighted = novelty_score * self.weights.w_novelty;
        score_breakdown.push(ScoreBreakdown::new("novelty", novelty_score, self.weights.w_novelty));
        trace.reasons.push(format!("Novelty score: {:.2}", novelty_score));

        // Calcular puntuación total
        let total_score = prot_weighted + scen_weighted + time_weighted + act_weighted + 
            faction_weighted + state_weighted + novelty_weighted;

        trace.score = total_score;
        trace.reasons.push(format!("Total score: {:.2}", total_score));

        trace
    }

    /// Calcular puntuación de protagonista (0-5)
    fn calculate_protagonist_score(&self, theme: &Theme, protagonist_state: &ProtagonistState) -> f32 {
        // Basado en compatibilidad protagonista-tema
        let theme_id = ThemeId::from(theme.base.id.clone());
        let compat_score = self.compatibility_set.get_protagonist_theme_score(
            &protagonist_state.protagonist_id,
            &theme_id
        );
        
        // Normalizar a 0-5
        compat_score as f32 / 5.0 * 5.0
    }

    /// Calcular puntuación de escenario (0-5)
    fn calculate_scenario_score(&self, theme: &Theme, scenario_id: &ScenarioId) -> f32 {
        let theme_id = ThemeId::from(theme.base.id.clone());
        let compat_score = self.compatibility_set.get_scenario_theme_score(scenario_id, &theme_id);
        compat_score as f32 / 5.0 * 5.0
    }

    /// Calcular puntuación de tiempo (0-5)
    fn calculate_time_score(&self, theme: &Theme, world_state: &WorldState) -> f32 {
        let world_time = world_state.get_time_slice();
        
        // Verificar si el tiempo del mundo está en las ventanas temporales del tema
        if theme.base.time_window.contains(&world_time) {
            5.0
        } else {
            // Penalización si no coincide
            0.0
        }
    }

    /// Calcular puntuación de acto (0-5)
    fn calculate_act_score(&self, theme: &Theme, world_state: &WorldState) -> f32 {
        let world_act = match world_state.narrative_act {
            1 => Act::Act1,
            2 => Act::Act2,
            3 => Act::Act3,
            _ => Act::Act4,
        };
        
        // Verificar si el acto del mundo está en los bias del tema
        if theme.base.act_bias.contains(&world_act) {
            5.0
        } else {
            // Penalización parcial si hay algún bias
            if !theme.base.act_bias.is_empty() {
                2.5
            } else {
                5.0
            }
        }
    }

    /// Calcular puntuación de facción (0-5)
    fn calculate_faction_score(&self, theme: &Theme, _protagonist_state: &ProtagonistState) -> f32 {
        // Verificar si las facciones del tema coinciden con las del protagonista
        // Esto es un placeholder - debería usar información de relaciones
        if !theme.base.faction_vectors.is_empty() {
            // Check if any faction vector matches protagonist's profile
            // For now, just return high score if theme has faction vectors
            5.0
        } else {
            3.0
        }
    }

    /// Calcular puntuación de estado (0-5)
    fn calculate_state_score(&self, _theme: &Theme, world_state: &WorldState) -> f32 {
        // Basado en el estado global del mundo
        match world_state.global_state {
            GlobalState::TenseNormality => 4.0,
            GlobalState::PreCrisis => 4.5,
            GlobalState::OpenCrisis => 5.0,
            GlobalState::PostCrisisAftermath => 3.5,
            GlobalState::PublicCelebration => 3.0,
            GlobalState::LatentRepression => 2.5,
        }
    }

    /// Calcular puntuación de novedad (0-5)
    fn calculate_novelty_score(&self, theme: &Theme, protagonist_state: &ProtagonistState) -> f32 {
        // Penalizar si el tema ha sido reciente
        let theme_id = ThemeId::from(theme.base.id.clone());
        if protagonist_state.has_recent_theme(&theme_id) {
            1.0 // Baja puntuación si ya se ha usado
        } else {
            5.0 // Máxima puntuación si es nuevo
        }
    }

    /// Calcular el desglose de puntuación completo
    pub fn calculate_score_breakdown(
        &self,
        theme: &Theme,
        protagonist_state: &ProtagonistState,
        world_state: &WorldState,
        scenario_id: &ScenarioId,
    ) -> Vec<crate::trace::ScoreBreakdown> {
        let mut breakdown = Vec::new();

        let prot_score = self.calculate_protagonist_score(theme, protagonist_state);
        breakdown.push(crate::trace::ScoreBreakdown::new("protagonist", prot_score, self.weights.w_protagonist));

        let scen_score = self.calculate_scenario_score(theme, scenario_id);
        breakdown.push(crate::trace::ScoreBreakdown::new("scenario", scen_score, self.weights.w_scenario));

        let time_score = self.calculate_time_score(theme, world_state);
        breakdown.push(crate::trace::ScoreBreakdown::new("time", time_score, self.weights.w_time));

        let act_score = self.calculate_act_score(theme, world_state);
        breakdown.push(crate::trace::ScoreBreakdown::new("act", act_score, self.weights.w_act));

        let faction_score = self.calculate_faction_score(theme, protagonist_state);
        breakdown.push(crate::trace::ScoreBreakdown::new("faction", faction_score, self.weights.w_faction));

        let state_score = self.calculate_state_score(theme, world_state);
        breakdown.push(crate::trace::ScoreBreakdown::new("state", state_score, self.weights.w_state));

        let novelty_score = self.calculate_novelty_score(theme, protagonist_state);
        breakdown.push(crate::trace::ScoreBreakdown::new("novelty", novelty_score, self.weights.w_novelty));

        breakdown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_score_calculator_creation() {
        let set = CompatibilitySet::new();
        let calculator = ScoreCalculator::new(set, None);
        
        assert_eq!(calculator.weights.w_protagonist, 3.0);
        assert_eq!(calculator.weights.w_scenario, 2.0);
    }

    #[test]
    fn test_calculate_theme_score() {
        let mut set = CompatibilitySet::new();
        
        // Añadir compatibilidad protagonista-tema
        let mut prot_map = HashMap::new();
        prot_map.insert(ThemeId("tema_1".to_string()), 5);
        set.protagonist_vs_theme.insert(ProtagonistId("prot_1".to_string()), prot_map);
        
        // Añadir compatibilidad escenario-tema
        let mut scen_map = HashMap::new();
        scen_map.insert(ThemeId("tema_1".to_string()), 4);
        set.scenario_vs_theme.insert(ScenarioId("esc_1".to_string()), scen_map);
        
        let calculator = ScoreCalculator::new(set, None);
        
        let mut theme = Theme::new("tema_1");
        theme.base.time_window = vec![TimeSlice::Y1810, TimeSlice::Y1811];
        theme.base.act_bias = vec![Act::Act1, Act::Act2];
        
        let protagonist_state = ProtagonistState::new("prot_1");
        let world_state = WorldState::new().with_journey(50); // Y1810
        let scenario_id = ScenarioId("esc_1".to_string());
        
        let trace = calculator.calculate_theme_score(&theme, &protagonist_state, &world_state, &scenario_id);
        
        assert!(trace.score > 0.0);
        assert!(!trace.reasons.is_empty());
    }

    #[test]
    fn test_time_score() {
        let set = CompatibilitySet::new();
        let calculator = ScoreCalculator::new(set, None);
        
        let mut theme = Theme::new("tema_1");
        theme.base.time_window = vec![TimeSlice::Y1810];
        
        let protagonist_state = ProtagonistState::new("prot_1");
        let world_state = WorldState::new().with_journey(50); // Y1810
        let scenario_id = ScenarioId("esc_1".to_string());
        
        let trace = calculator.calculate_theme_score(&theme, &protagonist_state, &world_state, &scenario_id);
        
        // Debería tener buena puntuación de tiempo
        assert!(trace.reasons.iter().any(|r| r.contains("Time fit score")));
    }

    #[test]
    fn test_novelty_score() {
        let set = CompatibilitySet::new();
        let calculator = ScoreCalculator::new(set, None);
        
        let theme = Theme::new("tema_1");
        
        let mut protagonist_state = ProtagonistState::new("prot_1");
        protagonist_state.record_theme("tema_1"); // Marcar como reciente
        
        let world_state = WorldState::new();
        let scenario_id = ScenarioId("esc_1".to_string());
        
        let trace = calculator.calculate_theme_score(&theme, &protagonist_state, &world_state, &scenario_id);
        
        // Debería tener baja puntuación de novedad
        assert!(trace.reasons.iter().any(|r| r.contains("Novelty score: 1.00")));
    }
}
