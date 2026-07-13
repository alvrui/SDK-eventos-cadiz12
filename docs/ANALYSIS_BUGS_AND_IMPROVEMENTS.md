# Análisis de Bugs y Mejoras - SDK Eventos Cádiz 1812

## Fecha: 2025-07-13

## Resumen Ejecutivo

Se han identificado **8 bugs críticos** y **4 oportunidades de mejora** en la gestión de agentes Mistral y la unificación de secciones (tramas/story elements). El principal problema es la **desconexión de datos** entre el frontend (UI), el backend (Rust), y los agentes de IA (secretario.py), lo que causa inconsistencias en la generación de contenido narrativo.

---

## 🐛 Bugs Críticos

### 1. **Desconexión entre `story_elements` y `plots`**
- **Ubicación**: `src/main.rs` (líneas 450-470)
- **Problema**: Existen endpoints separados `/api/ai/story-elements` y `/api/ai/plots` que deberían estar unificados según la documentación.
- **Impacto**: Duplicación de funcionalidad, confusión en la UI sobre qué sección usar.
- **Evidencia**: 
  - `docs/ia-actions-by-section.md` describe acciones para "Story Elements" pero no menciona "Plots" como sección separada.
  - En `project.json` aparecen ambos: `"plots":[]` y `"storyelements":[]`.

### 2. **Inconsistencia en nombres de agentes**
- **Ubicación**: `project_data/project.json` vs `src/main.rs`
- **Problema**: 
  - En `project.json`: `"storyelements": "DiseñadorDeStoryElements"`
  - En código: se usa la clave `"story_elements"` (con guión bajo)
- **Impacto**: El agente configurado en el proyecto no se usa correctamente.
- **Código afectado**: `get_agent_name()` en `main.rs:280`

### 3. **Falta de validación en `parse_agent_json_payload`**
- **Ubicación**: `src/main.rs` (líneas 180-240)
- **Problema**: La función intenta parsear JSON de múltiples formas pero no valida correctamente cuando el JSON válido está dentro del campo `data` del payload del agente.
- **Impacto**: Respuestas de agentes válidas son descartadas como errores.
- **Ejemplo**: Si el agente devuelve `{"status":"success","data":{"themes":[...]}}`, no se extrae correctamente.

### 4. **Problema en migración de `story_elements` a `plots`**
- **Ubicación**: `src/main.rs` (líneas 80-120, función `migrate_project_shape`)
- **Problema**: 
  - La migración crea `"plots":[]` pero no migra los datos de `story_elements` a `plots`.
  - Solo guarda `story_elements` en `"legacyStoryElements"` sin integrarlos.
- **Impacto**: Pérdida de datos al cargar proyectos antiguos.

### 5. **Falta de implementación en `EventRuntime::apply_event_effects`**
- **Ubicación**: `src/runtime/mod.rs` (líneas 30-45)
- **Problema**: 
  - No procesa `relationship_deltas` (solo hay un comentario)
  - No procesa `reputation_deltas`
  - No aplica `unlock_elements` ni `lock_elements`
- **Impacto**: Los efectos de relaciones y reputación no se aplican al estado del juego.

### 6. **Rangos incompletos en `WorldState::get_time_slice`**
- **Ubicación**: `src/state/world_state.rs` (líneas 120-130)
- **Problema**: 
  - Falta manejo de `TimeSlice::Y1805_1808`
  - Falta manejo de `TimeSlice::Y1809`
  - Falta manejo de `TimeSlice::Y1815_1816`
  - El default es `Y1812` para cualquier jornada > 500
- **Impacto**: Temas con ventanas temporales Y1805_1808, Y1809, Y1815_1816 nunca serán seleccionados.

### 7. **Inconsistencia en `HistoricalScope`**
- **Ubicación**: `project_data/project.json` vs `src/domain/enums.rs`
- **Problema**: 
  - En enum: `HistoricalScope::PlausibleDocumented`, `PlausibleInferred`, etc.
  - En JSON: se usa `"historicalscope":"StrictHistorical"` (no existe en el enum)
- **Impacto**: Error al deserializar proyectos existentes.

### 8. **Falta de lógica de fallback en selección de secundarios/procedimientos**
- **Ubicación**: `src/selector/mod.rs` (líneas 220-250)
- **Problema**: 
  - `select_secondaries()` devuelve `vec![]` si no hay bindings
  - `select_procedure()` devuelve un ID hardcodeado si no hay bindings
- **Impacto**: Eventos generados sin secundarios o con procedimientos no óptimos.

---

## 🎯 Mejoras para Gestión de Agentes Mistral

### 1. **Unificar secciones `tramas` y `story elements`**
**Propuesta**:
- Crear una nueva sección llamada `"narrative_elements"` que combine:
  - Themes
  - Protagonists
  - Antagonists
  - Secondaries
  - Scenarios
  - Procedures
  - DramaticResources
  - SocialPressures
- Eliminar la duplicación entre `plots` y `story_elements`.
- Mantener retrocompatibilidad con proyectos existentes.

**Beneficios**:
- Simplificación de la UI
- Menos confusión para el usuario
- Mejor integración con los agentes de IA

### 2. **Mejorar los prompts para agentes**
**Acciones**:
- Establecer un contrato claro de entrada/salida para cada acción
- Validar que los IDs devueltos existen en los catálogos
- Marcar claramente los IDs no resueltos (unresolved)
- Incluir siempre el contexto completo (acto, tono, facciones, espacios, tags)

**Ejemplo de mejora**:
```rust
// Antes: Prompt genérico
"Genera story elements para esta narrativa"

// Después: Prompt estructurado
{
  "action": "propose_narrative_elements",
  "context": {
    "narrative_id": "nar_001",
    "act": "Act2",
    "tone": "Ambiguous",
    "factions": ["liberales", "absolutistas"],
    "spaces": ["cortes", "calle"],
    "existing_elements": ["tema_001", "tema_002"],
    "blocked_tags": ["violencia_explicita"]
  },
  "required_format": {
    "themes": [{"id": "string", "label": "string", ...}],
    "protagonists": [...],
    "antagonists": [...],
    // etc.
  }
}
```

### 3. **Añadir validación estricta de respuestas de agentes**
**Implementación**:
- Crear un validador de schemas JSON para cada tipo de respuesta
- Validar antes de procesar la respuesta
- Devolver errores claros al agente para iteración

### 4. **Mejorar la gestión de errores**
**Acciones**:
- Crear tipos de error específicos para problemas de IA
- Registrar todas las interacciones con agentes para debugging
- Proporcionar feedback estructurado a los agentes

---

## 📋 Plan de Implementación

### Fase 1: Fixes Críticos (Prioridad Alta)
1. ✅ Fix `WorldState::get_time_slice` para manejar todos los TimeSlice
2. ✅ Fix `HistoricalScope` enum para incluir `StrictHistorical`
3. ✅ Fix migración de `story_elements` a `plots`
4. ✅ Fix `parse_agent_json_payload` para manejar JSON anidado en `data`
5. ✅ Fix inconsistencia en nombres de agentes (`storyelements` vs `story_elements`)

### Fase 2: Unificación de Secciones (Prioridad Media)
1. Crear nueva sección `narrative_elements`
2. Migrar funcionalidad de `plots` y `story_elements` a `narrative_elements`
3. Actualizar endpoints de API
4. Actualizar UI (si aplica)
5. Mantener retrocompatibilidad

### Fase 3: Mejoras en Agentes (Prioridad Media)
1. Implementar validación de schemas para respuestas de agentes
2. Mejorar prompts con contexto estructurado
3. Añadir logging de interacciones con agentes
4. Implementar feedback estructurado para agentes

### Fase 4: Completar Implementación (Prioridad Baja)
1. Implementar procesamiento de `relationship_deltas` en EventRuntime
2. Implementar procesamiento de `reputation_deltas` en EventRuntime
3. Implementar lógica de fallback inteligente para secundarios/procedimientos
4. Añadir tests para todos los casos de borde

---

## 🔍 Pruebas para Validar Fixes

### Test para Bug #6 (TimeSlice):
```rust
#[test]
fn test_all_time_slices_covered() {
    let state = WorldState::new().with_journey(10);
    assert_eq!(state.get_time_slice(), TimeSlice::Y1810);
    
    let state = WorldState::new().with_journey(5);
    assert_eq!(state.get_time_slice(), TimeSlice::Y1805_1808);
    
    let state = WorldState::new().with_journey(550);
    assert_eq!(state.get_time_slice(), TimeSlice::Y1815_1816);
}
```

### Test para Bug #3 (parse_agent_json_payload):
```rust
#[test]
fn test_parse_nested_json_in_data() {
    let payload = json!({
        "status": "success",
        "data": {
            "status": "success",
            "section": "plots",
            "action": "propose",
            "data": [{"id": "plot_1", "title": "Test"}]
        }
    });
    
    let result = parse_agent_json_payload(payload, "plots", "propose");
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed["section"], "plots");
}
```

---

## 📊 Métricas de Éxito

- [ ] Todos los tests existentes pasan
- [ ] No hay errores de compilación
- [ ] Las migraciones de proyectos antiguos funcionan correctamente
- [ ] Los agentes de IA devuelven respuestas válidas en >90% de los casos
- [ ] El tiempo de respuesta de la API no aumenta
- [ ] La cobertura de código se mantiene o mejora

---

## 🎓 Lecciones Aprendidas

1. **Validación temprana**: Validar datos de entrada de agentes lo antes posible
2. **Retrocompatibilidad**: Siempre considerar migraciones de datos al cambiar schemas
3. **Consistencia de nombres**: Usar los mismos nombres en código, JSON y documentación
4. **Logging**: Registrar interacciones con sistemas externos para debugging
5. **Testing**: Probar todos los casos de borde, especialmente en parsing de JSON

---

## 📝 Próximos Pasos

1. Implementar fixes de Fase 1
2. Ejecutar tests existentes
3. Validar que no se introducen nuevos bugs
4. Proceder con Fase 2
5. Documentar cambios en CHANGELOG.md
