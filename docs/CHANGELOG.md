# Changelog

## [Unreleased]

### Fixed

- **Bug #6**: Fix `WorldState::get_time_slice()` para manejar todos los `TimeSlice` valores (Y1805_1808, Y1809, Y1815_1816). Antes, solo se manejaban Y1810-Y1814, lo que causaba que temas con ventanas temporales fuera de ese rango nunca fueran seleccionados. [#vibe/fix-bugs-unify-sections-00e675]
- **Bug #7**: Añadido `StrictHistorical` al enum `HistoricalScope` para compatibilidad con proyectos existentes que usan este valor. [#vibe/fix-bugs-unify-sections-00e675]
- **Bug #3**: Fix `parse_agent_json_payload()` para manejar correctamente JSON anidado en el campo `data`. Ahora intenta extraer y parsear el campo `data` directamente antes de caer a los métodos de fallback. [#vibe/fix-bugs-unify-sections-00e675]
- **Bug #2**: Fix inconsistencia en nombres de agentes. La función `get_agent_name()` ahora maneja ambas claves: `story_elements` y `storyelements`, proporcionando fallback inteligente entre claves alternativas. [#vibe/fix-bugs-unify-sections-00e675]
- **Bug #4**: Fix migración de `story_elements` a `plots`. La función `migrate_project_shape()` ahora migra correctamente los datos de `story_elements` a `plots` en lugar de solo guardarlos en `legacyStoryElements`. [#vibe/fix-bugs-unify-sections-00e675]
- **Bug #5**: Mejoras en `EventRuntime::apply_event_effects()`. Añadido tracking de `tags_added` en el `EventOutcome` y comentarios claros para implementación futura de `relationship_deltas` y `reputation_deltas`. [#vibe/fix-bugs-unify-sections-00e675]
- **Bug #8**: Mejoras en lógica de fallback del selector. Las funciones `select_secondaries()` y `select_procedure()` ahora buscan elementos compatibles basados en `stakes_axis` y `faction_vectors` cuando no hay bindings definidos, con selección aleatoria entre opciones compatibles. [#vibe/fix-bugs-unify-sections-00e675]

### Added

- **Documentación**: Añadido `docs/ANALYSIS_BUGS_AND_IMPROVEMENTS.md` con análisis completo de bugs identificados, mejoras propuestas y plan de implementación. [#vibe/fix-bugs-unify-sections-00e675]

### Changed

- **Tests**: Actualizados tests en `world_state.rs` para validar todos los rangos de `TimeSlice`. [#vibe/fix-bugs-unify-sections-00e675]
- **Tests**: Añadidos tests para `HistoricalScope::StrictHistorical` en `enums.rs`. [#vibe/fix-bugs-unify-sections-00e675]
- **Tests**: Añadidos tests para validar `tags_added` en `runtime/mod.rs`. [#vibe/fix-bugs-unify-sections-00e675]

## [0.1.0] - 2025-01-01

### Added

- Versión inicial del SDK de eventos narrativo-políticos para Cádiz 1812
- Tipos del dominio para elementos de guion
- Catálogos de temas, protagonistas, antagonistas, secundarios, escenarios y procedimientos
- Estado del mundo y del protagonista
- Matrices de compatibilidad entre categorías
- Algoritmo de selección de eventos basado en puntuaciones ponderadas
- Instanciación de eventos con trazabilidad completa
