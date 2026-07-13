//! Structs del dominio - Estructuras de datos fundamentales

use crate::domain::enums::*;
use crate::domain::ids::*;
use serde::{Deserialize, Serialize};

/// Base común para todos los elementos de guion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptElementBase {
    pub id: ElementId,
    pub category: ScriptElementCategory,
    pub label: String,
    pub description: String,
    pub historical_scope: HistoricalScope,
    pub time_window: Vec<TimeSlice>,
    pub act_bias: Vec<Act>,
    pub faction_vectors: Vec<FactionId>,
    pub space_vectors: Vec<SpaceId>,
    pub stakes_axis: Vec<StakesAxis>,
    pub tone: Tone,
    pub chain_roles: Vec<ChainRole>,
    pub repeatability: Repeatability,
    pub meter_affinity: Vec<MeterType>,
    pub visibility_profile: VisibilityProfile,
    pub information_profile: InformationProfile,
    pub compatibility_tags: Vec<TagId>,
    pub blocking_tags: Vec<TagId>,
    pub unlock_tags: Vec<TagId>,
    pub generated_tags: Vec<TagId>,
}

impl ScriptElementBase {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            category: ScriptElementCategory::ThemeEvent,
            label: String::new(),
            description: String::new(),
            historical_scope: HistoricalScope::PlausibleInferred,
            time_window: vec![],
            act_bias: vec![],
            faction_vectors: vec![],
            space_vectors: vec![],
            stakes_axis: vec![],
            tone: Tone::Ambiguous,
            chain_roles: vec![],
            repeatability: Repeatability::Rare,
            meter_affinity: vec![],
            visibility_profile: VisibilityProfile::Discreet,
            information_profile: InformationProfile::PlausibleRumor,
            compatibility_tags: vec![],
            blocking_tags: vec![],
            unlock_tags: vec![],
            generated_tags: vec![],
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_historical_scope(mut self, scope: HistoricalScope) -> Self {
        self.historical_scope = scope;
        self
    }

    pub fn with_time_window(mut self, window: Vec<TimeSlice>) -> Self {
        self.time_window = window;
        self
    }

    pub fn with_act_bias(mut self, acts: Vec<Act>) -> Self {
        self.act_bias = acts;
        self
    }
}

/// Tema de evento
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub base: ScriptElementBase,
    pub category: ScriptElementCategory,
}

impl Theme {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: ScriptElementBase::new(id),
            category: ScriptElementCategory::ThemeEvent,
        }
    }
}

/// Arquetipo de protagonista
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtagonistArchetype {
    pub base: ScriptElementBase,
    pub eligible_profiles: Vec<PlayerProfileRef>,
    pub eligible_positions: Vec<FormalPositionId>,
}

impl ProtagonistArchetype {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: ScriptElementBase::new(id),
            eligible_profiles: vec![],
            eligible_positions: vec![],
        }
    }
}

/// Antagonista
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Antagonist {
    pub base: ScriptElementBase,
}

impl Antagonist {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: ScriptElementBase::new(id),
        }
    }
}

/// Secundario
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Secondary {
    pub base: ScriptElementBase,
}

impl Secondary {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: ScriptElementBase::new(id),
        }
    }
}

/// Escenario
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Scenario {
    pub base: ScriptElementBase,
}

impl Scenario {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: ScriptElementBase::new(id),
        }
    }
}

/// Procedimiento
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Procedure {
    pub base: ScriptElementBase,
    pub kind: ProcedureKind,
}

impl Procedure {
    pub fn new(id: impl Into<ElementId>, kind: ProcedureKind) -> Self {
        Self {
            base: ScriptElementBase::new(id),
            kind,
        }
    }
}

/// Recurso dramático
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DramaticResource {
    pub base: ScriptElementBase,
}

impl DramaticResource {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: ScriptElementBase::new(id),
        }
    }
}

/// Presión social
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialPressure {
    pub base: ScriptElementBase,
}

impl SocialPressure {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: ScriptElementBase::new(id),
        }
    }
}

/// Delta de medidor (cambio en un medidor)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeterDelta {
    pub meter_type: MeterType,
    pub delta: f32,
}

impl MeterDelta {
    pub fn new(meter_type: MeterType, delta: f32) -> Self {
        Self { meter_type, delta }
    }
}

/// Delta de relación
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelationshipDelta {
    pub entity_id: ElementId,
    pub level_delta: i8,
    pub state_delta: Option<RelationshipState>,
}

impl RelationshipDelta {
    pub fn new(entity_id: impl Into<ElementId>, level_delta: i8) -> Self {
        Self {
            entity_id: entity_id.into(),
            level_delta,
            state_delta: None,
        }
    }
}

/// Delta de reputación
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReputationDelta {
    pub group_id: ElementId,
    pub delta: f32,
}

impl ReputationDelta {
    pub fn new(group_id: impl Into<ElementId>, delta: f32) -> Self {
        Self {
            group_id: group_id.into(),
            delta,
        }
    }
}

/// Prototipo de consecuencias de evento
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventOutcomePrototype {
    pub meter_deltas: Vec<MeterDelta>,
    pub relationship_deltas: Vec<RelationshipDelta>,
    pub reputation_deltas: Vec<ReputationDelta>,
    pub add_tags: Vec<TagId>,
    pub remove_tags: Vec<TagId>,
    pub unlock_elements: Vec<ElementId>,
    pub lock_elements: Vec<ElementId>,
}

impl EventOutcomePrototype {
    pub fn new() -> Self {
        Self {
            meter_deltas: vec![],
            relationship_deltas: vec![],
            reputation_deltas: vec![],
            add_tags: vec![],
            remove_tags: vec![],
            unlock_elements: vec![],
            lock_elements: vec![],
        }
    }

    pub fn with_meter_delta(mut self, delta: MeterDelta) -> Self {
        self.meter_deltas.push(delta);
        self
    }

    pub fn with_relationship_delta(mut self, delta: RelationshipDelta) -> Self {
        self.relationship_deltas.push(delta);
        self
    }

    pub fn with_reputation_delta(mut self, delta: ReputationDelta) -> Self {
        self.reputation_deltas.push(delta);
        self
    }
}

/// Instancia de evento generado
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventInstance {
    pub id: EventInstanceId,
    pub scene_template: SceneTemplateType,
    pub pressure_series: Option<PressureSeriesType>,
    pub main_theme: ThemeId,
    pub satellite_themes: Vec<ThemeId>,
    pub protagonist: ProtagonistId,
    pub antagonist: Option<AntagonistId>,
    pub secondaries: Vec<SecondaryId>,
    pub scenario: ScenarioId,
    pub procedure: ProcedureId,
    pub dramatic_resources: Vec<ResourceId>,
    pub scheduled_time: TimeSlice,
    pub stakes_axis: Vec<StakesAxis>,
    pub tone: Tone,
    pub expected_meter_effects: Vec<MeterDelta>,
    pub generated_tags: Vec<TagId>,
    pub trace: crate::trace::SelectionTrace,
}

impl EventInstance {
    pub fn new(id: impl Into<EventInstanceId>) -> Self {
        Self {
            id: id.into(),
            scene_template: SceneTemplateType::AInstitutionalSession,
            pressure_series: None,
            main_theme: ThemeId("default".to_string()),
            satellite_themes: vec![],
            protagonist: ProtagonistId("default".to_string()),
            antagonist: None,
            secondaries: vec![],
            scenario: ScenarioId("default".to_string()),
            procedure: ProcedureId("default".to_string()),
            dramatic_resources: vec![],
            scheduled_time: TimeSlice::Y1810,
            stakes_axis: vec![],
            tone: Tone::Ambiguous,
            expected_meter_effects: vec![],
            generated_tags: vec![],
            trace: crate::trace::SelectionTrace::new(),
        }
    }
}

/// Pesos de puntuación para el algoritmo de selección
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionWeights {
    pub w_protagonist: f32,
    pub w_scenario: f32,
    pub w_antagonist: f32,
    pub w_time: f32,
    pub w_act: f32,
    pub w_faction: f32,
    pub w_state: f32,
    pub w_novelty: f32,
}

impl Default for SelectionWeights {
    fn default() -> Self {
        Self {
            w_protagonist: 3.0,
            w_scenario: 2.0,
            w_antagonist: 1.5,
            w_time: 2.0,
            w_act: 1.5,
            w_faction: 1.5,
            w_state: 2.0,
            w_novelty: 1.0,
        }
    }
}

impl SelectionWeights {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Elementos narrativos unificados
/// Combina themes, protagonists, antagonists, secondaries, scenarios, procedures, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NarrativeElements {
    pub themes: Vec<Theme>,
    pub protagonists: Vec<ProtagonistArchetype>,
    pub antagonists: Vec<Antagonist>,
    pub secondaries: Vec<Secondary>,
    pub scenarios: Vec<Scenario>,
    pub procedures: Vec<Procedure>,
    pub dramatic_resources: Vec<DramaticResource>,
    pub social_pressures: Vec<SocialPressure>,
}

impl NarrativeElements {
    pub fn new() -> Self {
        Self {
            themes: vec![],
            protagonists: vec![],
            antagonists: vec![],
            secondaries: vec![],
            scenarios: vec![],
            procedures: vec![],
            dramatic_resources: vec![],
            social_pressures: vec![],
        }
    }

    /// Añadir un tema
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.themes.push(theme);
        self
    }

    /// Añadir un protagonista
    pub fn with_protagonist(mut self, protagonist: ProtagonistArchetype) -> Self {
        self.protagonists.push(protagonist);
        self
    }

    /// Añadir un antagonista
    pub fn with_antagonist(mut self, antagonist: Antagonist) -> Self {
        self.antagonists.push(antagonist);
        self
    }

    /// Añadir un secundario
    pub fn with_secondary(mut self, secondary: Secondary) -> Self {
        self.secondaries.push(secondary);
        self
    }

    /// Añadir un escenario
    pub fn with_scenario(mut self, scenario: Scenario) -> Self {
        self.scenarios.push(scenario);
        self
    }

    /// Añadir un procedimiento
    pub fn with_procedure(mut self, procedure: Procedure) -> Self {
        self.procedures.push(procedure);
        self
    }

    /// Añadir un recurso dramático
    pub fn with_dramatic_resource(mut self, resource: DramaticResource) -> Self {
        self.dramatic_resources.push(resource);
        self
    }

    /// Añadir una presión social
    pub fn with_social_pressure(mut self, pressure: SocialPressure) -> Self {
        self.social_pressures.push(pressure);
        self
    }

    /// Obtener todos los elementos como un vector plano
    pub fn all_elements(&self) -> Vec<&dyn ScriptElementBase> {
        let mut all: Vec<&dyn ScriptElementBase> = Vec::new();
        
        for theme in &self.themes {
            all.push(&theme.base);
        }
        for protagonist in &self.protagonists {
            all.push(&protagonist.base);
        }
        for antagonist in &self.antagonists {
            all.push(&antagonist.base);
        }
        for secondary in &self.secondaries {
            all.push(&secondary.base);
        }
        for scenario in &self.scenarios {
            all.push(&scenario.base);
        }
        for procedure in &self.procedures {
            all.push(&procedure.base);
        }
        for resource in &self.dramatic_resources {
            all.push(&resource.base);
        }
        for pressure in &self.social_pressures {
            all.push(&pressure.base);
        }
        
        all
    }

    /// Validar que todos los elementos tienen IDs únicas
    pub fn validate_unique_ids(&self) -> Result<(), Vec<String>> {
        use std::collections::HashSet;
        
        let mut errors = Vec::new();
        let mut ids = HashSet::new();
        
        for theme in &self.themes {
            if !ids.insert(&theme.base.id.0) {
                errors.push(format!("Duplicate ID: {}", theme.base.id.0));
            }
        }
        
        for protagonist in &self.protagonists {
            if !ids.insert(&protagonist.base.id.0) {
                errors.push(format!("Duplicate ID: {}", protagonist.base.id.0));
            }
        }
        
        for antagonist in &self.antagonists {
            if !ids.insert(&antagonist.base.id.0) {
                errors.push(format!("Duplicate ID: {}", antagonist.base.id.0));
            }
        }
        
        for secondary in &self.secondaries {
            if !ids.insert(&secondary.base.id.0) {
                errors.push(format!("Duplicate ID: {}", secondary.base.id.0));
            }
        }
        
        for scenario in &self.scenarios {
            if !ids.insert(&scenario.base.id.0) {
                errors.push(format!("Duplicate ID: {}", scenario.base.id.0));
            }
        }
        
        for procedure in &self.procedures {
            if !ids.insert(&procedure.base.id.0) {
                errors.push(format!("Duplicate ID: {}", procedure.base.id.0));
            }
        }
        
        for resource in &self.dramatic_resources {
            if !ids.insert(&resource.base.id.0) {
                errors.push(format!("Duplicate ID: {}", resource.base.id.0));
            }
        }
        
        for pressure in &self.social_pressures {
            if !ids.insert(&pressure.base.id.0) {
                errors.push(format!("Duplicate ID: {}", pressure.base.id.0));
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_element_base_creation() {
        let base = ScriptElementBase::new("test_id")
            .with_label("Test Label")
            .with_description("Test Description");
        
        assert_eq!(base.id.0, "test_id");
        assert_eq!(base.label, "Test Label");
        assert_eq!(base.description, "Test Description");
    }

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new("tema_test");
        assert_eq!(theme.base.id.0, "tema_test");
        assert_eq!(theme.category, ScriptElementCategory::ThemeEvent);
    }

    #[test]
    fn test_selection_weights_default() {
        let weights = SelectionWeights::default();
        assert_eq!(weights.w_protagonist, 3.0);
        assert_eq!(weights.w_scenario, 2.0);
        assert_eq!(weights.w_antagonist, 1.5);
    }

    #[test]
    fn test_narrative_elements_creation() {
        let elements = NarrativeElements::new();
        assert!(elements.themes.is_empty());
        assert!(elements.protagonists.is_empty());
        assert!(elements.antagonists.is_empty());
    }

    #[test]
    fn test_narrative_elements_with_elements() {
        let mut elements = NarrativeElements::new();
        elements = elements.with_theme(Theme::new("theme_1"));
        elements = elements.with_protagonist(ProtagonistArchetype::new("prot_1"));
        elements = elements.with_antagonist(Antagonist::new("ant_1"));
        
        assert_eq!(elements.themes.len(), 1);
        assert_eq!(elements.protagonists.len(), 1);
        assert_eq!(elements.antagonists.len(), 1);
    }

    #[test]
    fn test_narrative_elements_validate_unique_ids() {
        let mut elements = NarrativeElements::new();
        elements = elements.with_theme(Theme::new("id_1"));
        elements = elements.with_theme(Theme::new("id_2"));
        
        let result = elements.validate_unique_ids();
        assert!(result.is_ok());
        
        let mut elements_with_dup = NarrativeElements::new();
        elements_with_dup = elements_with_dup.with_theme(Theme::new("id_1"));
        elements_with_dup = elements_with_dup.with_theme(Theme::new("id_1"));
        
        let result = elements_with_dup.validate_unique_ids();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("Duplicate ID")));
    }
}
