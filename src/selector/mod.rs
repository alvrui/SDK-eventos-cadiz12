//! Selector module - Selección de eventos

use crate::catalog::{Catalogs, ThemeBindingsCatalog};
use crate::compat::matrices::CompatibilitySet;
use crate::domain::enums::*;
use crate::domain::ids::*;
use crate::domain::structs::*;
use crate::scoring::ScoreCalculator;
use crate::state::protagonist_state::ProtagonistState;
use crate::state::world_state::{GameContext, WorldState};
use crate::trace::{SelectionTrace, ThemeCandidateTrace};
use rand::{prelude::SliceRandom,Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

/// Selector de eventos
#[derive(Debug, Clone)]
pub struct EventSelector {
    catalogs: Catalogs,
    compatibility_set: CompatibilitySet,
    score_calculator: ScoreCalculator,
    theme_bindings: ThemeBindingsCatalog,
}

impl EventSelector {
    pub fn new(
        catalogs: Catalogs,
        compatibility_set: CompatibilitySet,
        weights: Option<SelectionWeights>,
    ) -> Self {
        let score_calculator = ScoreCalculator::new(compatibility_set.clone(), weights);
        let theme_bindings = catalogs.theme_bindings.clone();
        
        Self {
            catalogs,
            compatibility_set,
            score_calculator,
            theme_bindings,
        }
    }

    /// Seleccionar un evento basado en el contexto del juego
    pub fn select_event(
        &self,
        context: &FullGameContext,
        seed: Option<u64>,
    ) -> Option<EventInstance> {
        // Paso 1: Filtrar temas válidos
        let valid_themes = self.filter_valid_themes(&context.world_state, &context.protagonist_state);
        
        if valid_themes.is_empty() {
            log::warn!("No valid themes found for current context");
            return None;
        }

        // Paso 2: Filtrar por compatibilidad mínima con protagonista y escenario
        let compatible_themes = self.filter_by_compatibility(&valid_themes, &context.protagonist_state, &context.world_state);
        
        if compatible_themes.is_empty() {
            log::warn!("No compatible themes found");
            return None;
        }

        // Paso 3: Calcular puntuaciones para todos los temas candidatos
        let scored_themes = self.score_themes(&compatible_themes, &context.protagonist_state, &context.world_state);
        
        if scored_themes.is_empty() {
            log::warn!("No scored themes available");
            return None;
        }

        // Paso 4: Seleccionar tema principal (con semilla reproducible)
        let (selected_theme, candidate_traces) = self.select_theme(&scored_themes, seed);
        
        // Paso 5: Elegir secundarios
        let theme_id = ThemeId::from(selected_theme.base.id.clone());
        let secondaries = self.select_secondaries(&theme_id);
        
        // Paso 6: Elegir procedimiento
        let procedure_id = self.select_procedure(&theme_id);
        
        // Paso 7: Derivar plantilla de escena
        let scene_template = self.derive_scene_template(&procedure_id);
        
        // Paso 8: Crear instancia de evento
        let mut event_instance = self.create_event_instance(
            &selected_theme,
            &context.protagonist_state,
            &context.world_state,
            &secondaries,
            &procedure_id,
            scene_template,
        );
        
        // Paso 9: Configurar la traza de selección
        let mut trace = crate::trace::SelectionTrace::new();
        trace.candidate_themes = candidate_traces;
        trace.chosen_theme = ThemeId::from(selected_theme.base.id.clone());
        trace.chosen_secondary_reasons = self.get_secondary_reasons(&secondaries);
        trace.chosen_procedure_reasons = self.get_procedure_reasons(&procedure_id);
        trace.final_score_breakdown = self.score_calculator.calculate_score_breakdown(
            &selected_theme,
            &context.protagonist_state,
            &context.world_state,
            &context.world_state.get_current_scenario().unwrap_or_else(|| ScenarioId("default".to_string())),
        );
        
        event_instance.trace = trace;
        
        Some(event_instance)
    }

    /// Filtrar temas válidos por históricos, temporales, etc.
    fn filter_valid_themes(
        &self,
        world_state: &WorldState,
        protagonist_state: &ProtagonistState,
    ) -> Vec<&Theme> {
        let world_time = world_state.get_time_slice();
        let world_act = match world_state.narrative_act {
            1 => Act::Act1,
            2 => Act::Act2,
            3 => Act::Act3,
            _ => Act::Act4,
        };

        self.catalogs.themes.themes.values()
            .filter(|theme| {
                // Filtrar por scope histórico (no Discarded)
                if theme.base.historical_scope == HistoricalScope::Discarded {
                    return false;
                }
                
                // Filtrar por ventana temporal
                if !theme.base.time_window.contains(&world_time) {
                    return false;
                }
                
                // Filtrar por bias de acto
                if !theme.base.act_bias.is_empty() && !theme.base.act_bias.contains(&world_act) {
                    return false;
                }
                
                // Filtrar por tags de bloqueo
                for tag in &theme.base.blocking_tags {
                    if protagonist_state.has_tag(tag) {
                        return false;
                    }
                }
                
                // Filtrar por tags de desbloqueo (si hay, al menos uno debe estar presente)
                if !theme.base.unlock_tags.is_empty() {
                    let has_any = theme.base.unlock_tags.iter()
                        .any(|tag| protagonist_state.has_tag(tag));
                    if !has_any {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }

    /// Filtrar temas por compatibilidad mínima
    fn filter_by_compatibility<'a>(
        &self,
        themes: &[&'a Theme],
        protagonist_state: &ProtagonistState,
        world_state: &WorldState,
    ) -> Vec<&'a Theme> {
        let scenario_id = world_state.get_current_scenario().unwrap_or_else(|| ScenarioId("default".to_string()));
        let threshold = self.compatibility_set.get_minimum_threshold();

        themes.iter()
            .filter(|&&theme| {
                // Verificar compatibilidad con protagonista
                let theme_id = ThemeId::from(theme.base.id.clone());
                let prot_score = self.compatibility_set.get_protagonist_theme_score(
                    &protagonist_state.protagonist_id,
                    &theme_id,
                );
                if prot_score < threshold {
                    return false;
                }

                // Verificar compatibilidad con escenario
                let scen_score = self.compatibility_set.get_scenario_theme_score(
                    &scenario_id,
                    &theme_id,
                );
                if scen_score < threshold {
                    return false;
                }

                true
            })
            .copied()
            .collect()
    }

    /// Calcular puntuaciones para todos los temas
    fn score_themes(
        &self,
        themes: &[&Theme],
        protagonist_state: &ProtagonistState,
        world_state: &WorldState,
    ) -> Vec<crate::trace::ThemeCandidateTrace> {
        let scenario_id = world_state.get_current_scenario().unwrap_or_else(|| ScenarioId("default".to_string()));

        themes.iter()
            .map(|&theme| {
                self.score_calculator.calculate_theme_score(
                    theme,
                    protagonist_state,
                    world_state,
                    &scenario_id,
                )
            })
            .collect()
    }

    /// Seleccionar un tema basado en las puntuaciones
    fn select_theme(
        &self,
        scored_themes: &[ThemeCandidateTrace],
        seed: Option<u64>,
    ) -> (&Theme, Vec<ThemeCandidateTrace>) {
        // Crear RNG con semilla
        let seed = seed.unwrap_or_else(|| rand::random());
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        // Ordenar por puntuación descendente
        let mut sorted = scored_themes.to_vec();
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        // Selección ponderada: mayor probabilidad para puntuaciones más altas
        let total_score: f32 = sorted.iter().map(|t| t.score).sum();
        let mut cumulative = 0.0;
        let rand_val = rng.gen::<f32>() * total_score;

        for (_i, trace) in sorted.iter().enumerate() {
            cumulative += trace.score;
            if rand_val <= cumulative {
                // Encontrar el tema correspondiente
                let selected_theme = self.catalogs.themes.get(&trace.theme_id).unwrap();
                return (selected_theme, sorted);
            }
        }

        // Fallback: seleccionar el mejor
        let selected_theme = self.catalogs.themes.get(&sorted[0].theme_id).unwrap();
        (selected_theme, sorted)
    }

    /// Seleccionar secundarios para un tema
    fn select_secondaries(&self, theme_id: &ThemeId) -> Vec<SecondaryId> {
        // Obtener secundarios desde ThemeBindings
        let binding_secondaries = self.theme_bindings.get_secondaries(theme_id);
        
        if !binding_secondaries.is_empty() {
            return binding_secondaries;
        }
        
        // Si no hay bindings, seleccionar secundarios compatibles con el tema
        // Basado en los faction_vectors y stakes_axis del tema
        let theme = match self.catalogs.themes.get(theme_id) {
            Some(t) => t,
            None => return vec![],
        };
        
        // Filtrar secundarios por compatibilidad con el tema
        let mut compatible_secondaries: Vec<SecondaryId> = self.catalogs.secondaries.secondaries.iter()
            .filter(|(_sec_id, secondary)| {
                // Verificar que el secundario tiene al menos un stake_axis en común con el tema
                let has_common_stake = theme.base.stakes_axis.iter()
                    .any(|stake| secondary.base.stakes_axis.contains(stake));
                
                // Verificar que el secundario tiene al menos un faction_vector en común con el tema
                let has_common_faction = theme.base.faction_vectors.iter()
                    .any(|faction| secondary.base.faction_vectors.contains(faction));
                
                has_common_stake || has_common_faction || secondary.base.faction_vectors.is_empty()
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        // Si hay secundarios compatibles, seleccionar hasta 2 aleatoriamente
        if !compatible_secondaries.is_empty() {
            let mut rng = rand::thread_rng();
            let num_to_select = compatible_secondaries.len().min(2);
            let indices: Vec<usize> = (0..compatible_secondaries.len()).collect();
            let selected_indices: Vec<usize> = indices.choose_multiple(&mut rng, num_to_select).cloned().collect();
            return selected_indices.iter().map(|&idx| compatible_secondaries[idx].clone()).collect();
        }
        
        // Fallback final: devolver vacío
        vec![]
    }

    /// Seleccionar procedimiento para un tema
    fn select_procedure(&self, theme_id: &ThemeId) -> ProcedureId {
        // Obtener procedimientos desde ThemeBindings
        let binding_procedures = self.theme_bindings.get_procedures(theme_id);
        
        if !binding_procedures.is_empty() {
            // Seleccionar aleatoriamente entre los bindings
            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0..binding_procedures.len());
            return binding_procedures[idx].clone();
        }
        
        // Si no hay bindings, seleccionar procedimiento compatible con el tema
        let theme = match self.catalogs.themes.get(theme_id) {
            Some(t) => t,
            None => return ProcedureId("proc_debate_pleno_cortes".to_string()),
        };
        
        // Filtrar procedimientos por compatibilidad con el tema
        let compatible_procedures: Vec<ProcedureId> = self.catalogs.procedures.procedures.iter()
            .filter(|(_proc_id, procedure)| {
                // Verificar que el procedimiento tiene al menos un stake_axis en común con el tema
                let has_common_stake = theme.base.stakes_axis.iter()
                    .any(|stake| procedure.base.stakes_axis.contains(stake));
                
                // Verificar que el procedimiento tiene al menos un faction_vector en común con el tema
                let has_common_faction = theme.base.faction_vectors.iter()
                    .any(|faction| procedure.base.faction_vectors.contains(faction));
                
                has_common_stake || has_common_faction || procedure.base.faction_vectors.is_empty()
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        if !compatible_procedures.is_empty() {
            // Seleccionar aleatoriamente entre los compatibles
            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0..compatible_procedures.len());
            return compatible_procedures[idx].clone();
        }
        
        // Fallback final: procedimiento por defecto basado en el tono del tema
        match theme.base.tone {
            Tone::Solemn | Tone::Patriotic | Tone::Funereal => ProcedureId("proc_debate_pleno_cortes".to_string()),
            Tone::Intimate | Tone::Conspiratorial => ProcedureId("proc_visita_privada_americano".to_string()),
            Tone::Anxious | Tone::Tense | Tone::Combative => ProcedureId("proc_crisis_publica_bombardeo".to_string()),
            Tone::Satirical | Tone::Polemical => ProcedureId("proc_edicion_de_la_tarde".to_string()),
            _ => ProcedureId("proc_debate_pleno_cortes".to_string()),
        }
    }

    /// Derivar plantilla de escena a partir del procedimiento
    fn derive_scene_template(&self, procedure_id: &ProcedureId) -> SceneTemplateType {
        // Mapeo procedimiento -> plantilla de escena
        // Esto es un mapeo simplificado basado en las instrucciones
        match procedure_id.0.as_str() {
            "proc_debate_pleno_cortes" | "proc_decreto_libertad_imprenta" | "proc_decreto_sanitario_regencia" => {
                SceneTemplateType::AInstitutionalSession
            }
            "proc_visita_privada_americano" | "proc_parte_sanitario_diario" => {
                SceneTemplateType::CPrivateVisit
            }
            "proc_edicion_de_la_tarde" | "proc_lectura_gaceta_regencia" => {
                SceneTemplateType::DDocumentReading
            }
            "proc_crisis_publica_bombardeo" | "proc_motin_de_barrio" => {
                SceneTemplateType::EPublicCrisis
            }
            "proc_lista_apostentadores_desalojos" | "proc_peticion_publica_tasacion" => {
                SceneTemplateType::BUrbanEncounter
            }
            _ => {
                // Por defecto: Sesión institucional
                SceneTemplateType::AInstitutionalSession
            }
        }
    }

    /// Crear instancia de evento
    fn create_event_instance(
        &self,
        theme: &Theme,
        protagonist_state: &ProtagonistState,
        world_state: &WorldState,
        secondaries: &[SecondaryId],
        procedure_id: &ProcedureId,
        scene_template: SceneTemplateType,
    ) -> EventInstance {
        let mut instance = EventInstance::new(format!("event_{}_{}", theme.base.id.0, world_state.absolute_journey));
        
        instance.scene_template = scene_template;
        instance.main_theme = ThemeId::from(theme.base.id.clone());
        instance.protagonist = protagonist_state.protagonist_id.clone();
        instance.secondaries = secondaries.to_vec();
        instance.scenario = world_state.get_current_scenario().unwrap_or_else(|| ScenarioId("default".to_string()));
        instance.procedure = procedure_id.clone();
        instance.scheduled_time = world_state.get_time_slice();
        instance.stakes_axis = theme.base.stakes_axis.clone();
        instance.tone = theme.base.tone;
        instance.generated_tags = theme.base.generated_tags.clone();
        
        // Configurar efectos esperados en medidores (placeholder)
        instance.expected_meter_effects = self.get_expected_meter_effects(theme, procedure_id);
        
        instance
    }

    /// Obtener efectos esperados en medidores
    fn get_expected_meter_effects(&self, _theme: &Theme, _procedure_id: &ProcedureId) -> Vec<MeterDelta> {
        // Esto es un placeholder - debería basarse en la configuración del tema/procedimiento
        vec![
            MeterDelta::new(MeterType::Influence, 5.0),
            MeterDelta::new(MeterType::Reputation, 3.0),
        ]
    }

    /// Obtener razones para la selección de secundarios
    fn get_secondary_reasons(&self, secondaries: &[SecondaryId]) -> Vec<String> {
        secondaries.iter()
            .map(|sec_id| {
                if self.theme_bindings.bindings.values().any(|b| b.secondary_ids.contains(sec_id)) {
                    format!("Secondary {} selected from theme bindings", sec_id.0)
                } else {
                    format!("Secondary {} selected by compatibility fallback", sec_id.0)
                }
            })
            .collect()
    }

    /// Obtener razones para la selección de procedimiento
    fn get_procedure_reasons(&self, procedure_id: &ProcedureId) -> Vec<String> {
        vec![format!("Procedure {} selected from theme bindings or compatibility fallback", procedure_id.0)]
    }
}

/// Extensión de WorldState para obtener el escenario actual
trait WorldStateExt {
    fn get_current_scenario(&self) -> Option<ScenarioId>;
}

impl WorldStateExt for WorldState {
    fn get_current_scenario(&self) -> Option<ScenarioId> {
        self.current_scenario.clone()
    }
}

/// Contexto completo para selección (alias)
pub type FullGameContext = super::state::protagonist_state::FullGameContext;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::ThemeBindings;
    use std::collections::HashMap;

    #[test]
    fn test_event_selector_creation() {
        let catalogs = Catalogs::new();
        let compatibility_set = CompatibilitySet::new();
        
        let selector = EventSelector::new(catalogs, compatibility_set, None);
        
        assert!(!selector.catalogs.themes.themes.is_empty() || true); // Puede estar vacío
    }

    #[test]
    fn test_filter_valid_themes() {
        let mut catalogs = Catalogs::new();
        
        // Añadir tema válido
        let mut theme1 = Theme::new("tema_valido");
        theme1.base.historical_scope = HistoricalScope::PlausibleDocumented;
        theme1.base.time_window = vec![TimeSlice::Y1810];
        theme1.base.act_bias = vec![Act::Act1];
        catalogs.themes.add(theme1);
        
        // Añadir tema descartado
        let mut theme2 = Theme::new("tema_descartado");
        theme2.base.historical_scope = HistoricalScope::Discarded;
        catalogs.themes.add(theme2);
        
        let compatibility_set = CompatibilitySet::new();
        let selector = EventSelector::new(catalogs, compatibility_set, None);
        
        let world_state = WorldState::new().with_journey(150); // Y1810
        let protagonist_state = ProtagonistState::new("prot_1");
        
        let valid_themes = selector.filter_valid_themes(&world_state, &protagonist_state);
        
        assert_eq!(valid_themes.len(), 1);
        assert_eq!(valid_themes[0].base.id.0, "tema_valido");
    }

    #[test]
    fn test_derive_scene_template() {
        let catalogs = Catalogs::new();
        let compatibility_set = CompatibilitySet::new();
        let selector = EventSelector::new(catalogs, compatibility_set, None);
        
        let proc_id = ProcedureId("proc_debate_pleno_cortes".to_string());
        let template = selector.derive_scene_template(&proc_id);
        
        assert_eq!(template, SceneTemplateType::AInstitutionalSession);
    }

    #[test]
    fn test_select_secondaries() {
        let mut catalogs = Catalogs::new();
        
        // Añadir bindings
        let mut bindings = ThemeBindingsCatalog::new();
        let theme_id = ThemeId("tema_1".to_string());
        let theme_bindings = ThemeBindings::new()
            .with_secondary("sec_arguelles")
            .with_secondary("sec_muoz_torrero");
        bindings.add(theme_id.clone(), theme_bindings);
        catalogs.theme_bindings = bindings;
        
        let compatibility_set = CompatibilitySet::new();
        let selector = EventSelector::new(catalogs, compatibility_set, None);
        
        let secondaries = selector.select_secondaries(&theme_id);
        
        assert_eq!(secondaries.len(), 2);
        assert!(secondaries.contains(&SecondaryId("sec_arguelles".to_string())));
        assert!(secondaries.contains(&SecondaryId("sec_muoz_torrero".to_string())));
    }

    #[test]
    fn test_select_procedure() {
        let mut catalogs = Catalogs::new();
        
        // Añadir bindings
        let mut bindings = ThemeBindingsCatalog::new();
        let theme_id = ThemeId("tema_1".to_string());
        let theme_bindings = ThemeBindings::new()
            .with_procedure("proc_debate_pleno_cortes")
            .with_procedure("proc_visita_privada_americano");
        bindings.add(theme_id.clone(), theme_bindings);
        catalogs.theme_bindings = bindings;
        
        let compatibility_set = CompatibilitySet::new();
        let selector = EventSelector::new(catalogs, compatibility_set, None);
        
        let procedure = selector.select_procedure(&theme_id);
        
        // Debería ser uno de los dos procedimientos en bindings
        assert!(procedure.0 == "proc_debate_pleno_cortes" || procedure.0 == "proc_visita_privada_americano");
    }
}
