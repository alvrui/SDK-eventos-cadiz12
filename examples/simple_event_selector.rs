//! Simple CLI example demonstrating the Cadiz12 SDK Eventos
//!
//! This example shows how to:
//! - Create catalogs
//! - Set up compatibility matrices
//! - Create game state
//! - Select an event
//! - Display the selection trace

use cadiz12_sdk_eventos::{
    catalog::{Catalogs, ThemeBindings, ThemeBindingsCatalog},
    compat::CompatibilitySet,
    domain::{
        enums::{Act, HistoricalScope, TimeSlice, Tone},
        ids::{ProtagonistId, ScenarioId, ThemeId},
        structs::{ScriptElementBase, Theme},
    },
    state::{FullGameContext, ProtagonistState, WorldState},
    selector::EventSelector,
    trace::TraceFormatter,
};

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    println!("=== Cadiz12 SDK Eventos - Simple Event Selector ===\n");

    // Step 1: Create catalogs with sample data
    println!("Creating catalogs...");
    let mut catalogs = Catalogs::new();

    // Add a theme
    let mut theme = Theme::new("tema_libertad_imprenta");
    theme.base = ScriptElementBase::new("tema_libertad_imprenta")
        .with_label("Libertad de Imprenta".to_string())
        .with_description("Debate sobre la libertad de imprenta en las Cortes".to_string())
        .with_historical_scope(HistoricalScope::PlausibleDocumented)
        .with_time_window(vec![TimeSlice::Y1810, TimeSlice::Y1811]);
    theme.base.act_bias = vec![Act::Act1, Act::Act2];
    theme.base.tone = Tone::Solemn;
    catalogs.themes.add(theme);

    // Add another theme
    let mut theme2 = Theme::new("tema_decreto_sanitario");
    theme2.base = ScriptElementBase::new("tema_decreto_sanitario")
        .with_label("Decreto Sanitario".to_string())
        .with_description("Medidas sanitarias durante la regencia".to_string())
        .with_historical_scope(HistoricalScope::PlausibleDocumented)
        .with_time_window(vec![TimeSlice::Y1810, TimeSlice::Y1811, TimeSlice::Y1812]);
    theme2.base.act_bias = vec![Act::Act1];
    theme2.base.tone = Tone::Solemn;
    catalogs.themes.add(theme2);

    // Add theme bindings (theme -> secondaries/procedures)
    let mut bindings = ThemeBindingsCatalog::new();
    let theme_bindings = ThemeBindings::new()
        .with_secondary("sec_arguelles")
        .with_secondary("sec_muoz_torrero")
        .with_procedure("proc_debate_pleno_cortes");
    bindings.add(ThemeId("tema_libertad_imprenta".to_string()), theme_bindings);
    catalogs.theme_bindings = bindings;

    println!("✓ Created {} themes", catalogs.themes.themes.len());

    // Step 2: Set up compatibility matrices
    println!("\nSetting up compatibility matrices...");
    let mut compatibility_set = CompatibilitySet::new();

    // Protagonist compatibility with themes
    use std::collections::HashMap;
    let mut prot_map = HashMap::new();
    prot_map.insert(ThemeId("tema_libertad_imprenta".to_string()), 5);
    prot_map.insert(ThemeId("tema_decreto_sanitario".to_string()), 4);
    compatibility_set.protagonist_vs_theme.insert(ProtagonistId("prot_1".to_string()), prot_map);

    // Scenario compatibility with themes
    let mut scen_map = HashMap::new();
    scen_map.insert(ThemeId("tema_libertad_imprenta".to_string()), 5);
    scen_map.insert(ThemeId("tema_decreto_sanitario".to_string()), 4);
    compatibility_set.scenario_vs_theme.insert(ScenarioId("esc_cortes".to_string()), scen_map);

    println!("✓ Set up compatibility matrices");

    // Step 3: Create game context
    println!("\nCreating game context...");
    let mut world_state = WorldState::new()
        .with_journey(50) // Journey 50 = Year 1810
        .with_narrative_act(1);
    world_state.current_scenario = Some(ScenarioId("esc_cortes".to_string()));

    let protagonist_state = ProtagonistState::new("prot_1");

    let context = FullGameContext::new(
        world_state,
        protagonist_state,
        50,
        vec![],
    );

    println!("✓ Created game context (Act {}, Journey {})", context.world_state.narrative_act, context.world_state.absolute_journey);

    // Step 4: Create event selector
    println!("\nCreating event selector...");
    let selector = EventSelector::new(
        catalogs.clone(),
        compatibility_set,
        None, // Use default weights
    );

    // Step 5: Select an event
    println!("\nSelecting event...");
    match selector.select_event(&context, Some(42)) {
        Some(event) => {
            println!("✓ Selected event: {}", event.id.0);
            println!("  - Main theme: {}", event.main_theme.0);
            println!("  - Scene template: {:?}", event.scene_template);
            println!("  - Procedure: {}", event.procedure.0);
            println!("  - Secondaries: {:?}", event.secondaries);
            println!("  - Scheduled time: {:?}", event.scheduled_time);
            println!("  - Tone: {:?}", event.tone);
            
            // Display the selection trace
            println!("\n=== Selection Trace ===");
            let formatter = TraceFormatter::new();
            println!("{}", formatter.format_as_text(&event.trace));
        }
        None => {
            println!("✗ No event selected (no valid themes available)");
        }
    }

    println!("\n=== Example Complete ===");
}
