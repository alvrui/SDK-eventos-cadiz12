# Plan de Implementación - Fase 2
# Unificación de Secciones: Tramas y Story Elements

## Objetivo
Unificar las secciones `plots` y `story_elements` en una sola sección llamada `narrative_elements` que combine toda la funcionalidad de generación de contenido narrativo.

## Contexto
Actualmente:
- Existen dos secciones separadas: `plots` y `story_elements`
- Cada una tiene su propio endpoint de API: `/api/ai/plots` y `/api/ai/story-elements`
- Esto causa confusión en la UI y duplicación de funcionalidad
- Los agentes de IA generan contenido para ambas secciones de forma independiente

## Alcance

### 1. Nueva Estructura de Datos
Crear una estructura unificada `NarrativeElements` que contenga:
- Themes
- Protagonists  
- Antagonists
- Secondaries
- Scenarios
- Procedures
- DramaticResources
- SocialPressures

### 2. Migración de Datos
- Migrar `plots` + `story_elements` a `narrative_elements`
- Mantener retrocompatibilidad con proyectos existentes
- Validar que no se pierden datos en la migración

### 3. Unificación de Endpoints
- Crear nuevo endpoint `/api/ai/narrative-elements`
- Mantener endpoints antiguos por retrocompatibilidad
- Consolidar lógica de generación de contenido

### 4. Mejoras en Prompts
- Crear prompts unificados para agentes Mistral
- Validar que los IDs devueltos existen en catálogos
- Marcar IDs no resueltos claramente

---

## Tareas Detalladas

### Tarea 1: Definir Estructura `NarrativeElements`
**Archivo**: `src/domain/structs.rs`

```rust
/// Elementos narrativos unificados
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
```

**Prioridad**: Alta
**Esfuerzo**: Medio

---

### Tarea 2: Crear Funciones de Migración
**Archivo**: `src/main.rs` (función `migrate_project_shape`)

```rust
fn migrate_to_narrative_elements(v: &mut Value) {
    // Si ya existe narrative_elements, no hacer nada
    if v.get("narrative_elements").is_some() {
        return;
    }
    
    // Crear narrative_elements combinando plots y story_elements
    let mut narrative_elements = json!({});
    
    // Migrar themes desde story_elements
    if let Some(story_elements) = v.get("story_elements").and_then(|v| v.as_array()) {
        let themes: Vec<Value> = story_elements
            .iter()
            .filter(|e| e.get("type").and_then(|t| t.as_str()) == Some("Theme"))
            .cloned()
            .collect();
        narrative_elements["themes"] = json!(themes);
    }
    
    // Migrar desde plots
    if let Some(plots) = v.get("plots").and_then(|v| v.as_array()) {
        // Plots pueden contener themes, protagonists, etc.
        // Necesitamos mapear cada plot a su tipo correspondiente
        for plot in plots {
            // Lógica de mapeo...
        }
    }
    
    v["narrative_elements"] = narrative_elements;
}
```

**Prioridad**: Alta
**Esfuerzo**: Alto

---

### Tarea 3: Crear Nuevo Endpoint de API
**Archivo**: `src/main.rs`

```rust
(Method::Post, "/api/ai/narrative-elements") => {
    let incoming = load_json_body(&mut request);
    let action = incoming.get("action").and_then(|v| v.as_str()).unwrap_or("generate");
    let project = incoming.get("project").cloned().unwrap_or_else(|| json!({}));

    let response_body = handle_ai_request(
        "narrative_elements",
        "CoordinadorNarrativo",
        action,
        &project,
        prompt_for_narrative_elements,
    );

    let _ = request.respond(json_response(response_body));
}
```

**Prioridad**: Alta
**Esfuerzo**: Medio

---

### Tarea 4: Crear Prompt Unificado
**Archivo**: `src/main.rs`

```rust
fn prompt_for_narrative_elements(action: &str, project: &Value) -> String {
    format!(
        r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de elementos narrativos unificados.
Acción solicitada: {action}

Contexto del proyecto:
{project_context}

Devuelve este formato exacto:
{{
  "status": "success",
  "section": "narrative_elements",
  "action": "{action}",
  "data": {{
    "themes": [
      {{
        "id": "string",
        "type": "Theme",
        "label": "string",
        "description": "string",
        "tone": "string",
        "historical_scope": "string",
        "time_window": ["string"],
        "act_bias": ["string"],
        "stakes_axis": ["string"]
      }}
    ],
    "protagonists": [
      {{
        "id": "string",
        "type": "Protagonist",
        "label": "string",
        "description": "string"
      }}
    ],
    "antagonists": [...],
    "secondaries": [...],
    "scenarios": [...],
    "procedures": [...],
    "dramatic_resources": [...],
    "social_pressures": [...]
  }},
  "warnings": []
}}

Reglas:
- Usa solo IDs que existan en los catálogos o marca como unresolved
- No inventes enums fuera del vocabulario dado
- Prioriza coherencia local y causalidad jugable"#,
        action = action,
        project_context = serde_json::to_string_pretty(project).unwrap_or_default()
    )
}
```

**Prioridad**: Alta
**Esfuerzo**: Medio

---

### Tarea 5: Actualizar Validación
**Archivo**: `src/validation/mod.rs`

Añadir validación para `narrative_elements`:
```rust
fn validate_narrative_elements(&self, elements: &NarrativeElements, errors: &mut Vec<String>) {
    // Validar que no hay IDs duplicados
    let mut all_ids = HashSet::new();
    
    for theme in &elements.themes {
        if !all_ids.insert(&theme.base.id.0) {
            errors.push(format!("Duplicate ID in themes: {}", theme.base.id.0));
        }
    }
    
    // Validar que todos los elementos tienen IDs
    for protagonist in &elements.protagonists {
        if protagonist.base.id.0.is_empty() {
            errors.push("Protagonist with empty ID".to_string());
        }
    }
    
    // Validar que time_window y act_bias no están vacíos para themes
    for theme in &elements.themes {
        if theme.base.time_window.is_empty() {
            errors.push(format!("Theme {} has no time_window", theme.base.id.0));
        }
        if theme.base.act_bias.is_empty() {
            errors.push(format!("Theme {} has no act_bias", theme.base.id.0));
        }
    }
}
```

**Prioridad**: Media
**Esfuerzo**: Medio

---

### Tarea 6: Actualizar UI (Opcional)
**Archivos**: `ui/index.html`, `ui/app.js`

- Unificar las secciones en la interfaz
- Mostrar `narrative_elements` como sección principal
- Mantener `plots` y `story_elements` como secciones legacy (solo lectura)

**Prioridad**: Baja
**Esfuerzo**: Alto

---

## Cronograma Estimado

| Tarea | Prioridad | Esfuerzo | Tiempo Estimado |
|-------|-----------|----------|-----------------|
| 1. Estructura NarrativeElements | Alta | Medio | 2 horas |
| 2. Funciones de Migración | Alta | Alto | 4 horas |
| 3. Nuevo Endpoint API | Alta | Medio | 2 horas |
| 4. Prompt Unificado | Alta | Medio | 2 horas |
| 5. Validación | Media | Medio | 2 horas |
| 6. UI (Opcional) | Baja | Alto | 4 horas |

**Total Estimado**: 12-16 horas

---

## Dependencias

- Los fixes de la Fase 1 deben estar aplicados
- Todos los tests de la Fase 1 deben pasar
- Revisión y aprobación de los cambios de la Fase 1

---

## Criterios de Aceptación

1. ✅ Todos los tests pasan
2. ✅ No hay errores de compilación
3. ✅ Migraciones de proyectos antiguos funcionan correctamente
4. ✅ El nuevo endpoint `/api/ai/narrative-elements` funciona
5. ✅ Los endpoints antiguos siguen funcionando (retrocompatibilidad)
6. ✅ La validación de `narrative_elements` funciona
7. ✅ No se pierden datos en la migración

---

## Riesgos

1. **Pérdida de datos**: Si la migración no es correcta, se podrían perder datos de proyectos existentes
   - **Mitigación**: Hacer backup de proyectos antes de migrar, validar migración con tests

2. **Incompatibilidad con UI**: Si la UI no se actualiza, podría haber inconsistencias
   - **Mitigación**: Mantener endpoints antiguos, hacer la UI opcional en esta fase

3. **Problemas de rendimiento**: La generación unificada podría ser más lenta
   - **Mitigación**: Optimizar prompts, validar rendimiento con tests

---

## Métricas de Éxito

- [ ] Todos los tests pasan
- [ ] No hay errores de compilación
- [ ] Migraciones funcionan correctamente
- [ ] Nuevo endpoint funciona
- [ ] Endpoints antiguos siguen funcionando
- [ ] Validación funciona
- [ ] No se pierden datos
