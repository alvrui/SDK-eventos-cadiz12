# 📋 Contratos de Comunicación con Agentes Mistral

Este documento describe los contratos de comunicación entre la aplicación SDK-eventos-cadiz12 y los agentes Mistral. Estos contratos definen:
- El **contexto** que se envía a los agentes
- El **formato de entrada** esperado
- El **formato de salida** que debe devolver la IA
- **Ejemplos** de peticiones y respuestas

---

## 🎯 Agentes Principales

| Agente | ID | Uso Principal | Secciones Relacionadas |
|--------|-----|----------------|------------------------|
| CoordinadorNarrativo | `ag_019f34c158827241b3bfe6d6a396cd89` | Coordinación de narrativa, generación de contenido | Narrativa, Tramas, Catálogos, Eventos |
| ContextoHistorico | `ag_019ee18dbd25731ea2a0bffcb261e616` | Contexto histórico, validación de coherencia | Narrativa, Tramas-Catálogos, Eventos |
| Verificacion | `ag_019ee195c42974248d948588652bbc55` | Revisión, validación, detección de errores | Revisión, JSON |

---

## 📡 Contrato Base

Todos los mensajes a los agentes siguen este formato base:

### Request (Entrada)
```json
{
  "agente": "NombreDelAgente",
  "mensaje": "Prompt en lenguaje natural con contexto"
}
```

### Response (Salida Esperada)
```json
{
  "status": "success" | "error",
  "text": "Respuesta en texto (opcional, si no es JSON directo)",
  "conversation_id": "ID de la conversación (opcional)"
}
```

**Importante**: La aplicación espera que la IA devuelva **SOLO JSON válido** sin markdown, explicaciones adicionales o texto fuera del JSON.

---

## 📖 Contratos por Acción

### 1. Generar Catálogo Adaptado

**Agente**: CoordinadorNarrativo / ContextoHistorico
**Acción**: `generate_catalog`
**Sección**: Tramas - Catálogos

#### Request
```json
{
  "agente": "CoordinadorNarrativo",
  "mensaje": "Generar un catálogo de story elements adaptado al siguiente proyecto: {PROYECTO_JSON}. Basado en el catálogo genérico: {CATALOGO_GENERICO_SNIPPET}. Devuelve SOLO JSON con el formato: {\"name\": \"nombre del catálogo\", \"elements\": [{\"id\": \"...\", \"name\": \"...\", \"type\": \"...\", \"description\": \"...\", \"recommended\": true/false}]}"
}
```

#### Response
```json
{
  "name": "Catálogo para Proyecto X",
  "elements": [
    {
      "id": "PROTAGONIST_001",
      "name": "Héroe Accidental",
      "type": "Protagonist",
      "description": "Personaje que se ve obligado a ser héroe",
      "recommended": true,
      "reason": "Adecuado para el tono y género del proyecto"
    },
    {
      "id": "THEME_001",
      "name": "Justicia vs Injusticia",
      "type": "Theme",
      "description": "Conflicto moral central",
      "recommended": true,
      "reason": "Alinea con los valores del proyecto"
    }
  ]
}
```

#### Ejemplo Real
```json
{
  "name": "Catálogo para Drama Histórico",
  "elements": [
    {
      "id": "ACCIDENTAL_HERO",
      "name": "Héroe Accidental",
      "spanish_name": "Héroe accidental",
      "type": "Protagonist",
      "subtype": "heroic_archetype",
      "description": "Una persona corriente se ve obligada a convertirse en héroe tras un incidente inesperado.",
      "recommended": true
    }
  ]
}
```

---

### 2. Resaltar Elementos Recomendados

**Agente**: ContextoHistorico
**Acción**: `recommend_elements`
**Sección**: Tramas - Catálogos

#### Request
```json
{
  "agente": "ContextoHistorico",
  "mensaje": "Analiza el siguiente proyecto: {PROYECTO_JSON}. De los siguientes story elements, selecciona los más adecuados: {ELEMENTOS_SNIPPET}. Devuelve SOLO JSON con un array de IDs recomendados: [\"id1\", \"id2\", ...]"
}
```

#### Response
```json
["ACCIDENTAL_HERO", "THEME_001", "ANTAGONIST_003"]
```

---

### 3. Generar Trama desde Catálogo

**Agente**: CoordinadorNarrativo
**Acción**: `generate_from_catalog`
**Sección**: Tramas - Workflow

#### Request
```json
{
  "agente": "CoordinadorNarrativo",
  "mensaje": "Generar una trama completa para el proyecto: {PROYECTO_JSON}. Usa los siguientes story elements del catálogo: {ELEMENTOS_SNIPPET}. Devuelve SOLO JSON con el formato de trama: {\"id\": \"...\", \"title\": \"...\", \"summary\": \"...\", \"status\": \"draft\", \"protagonist_id\": \"...\", \"antagonist_ids\": [...], \"theme_ids\": [...], \"event_ids\": [...], \"setting_ids\": [...], \"finale_id\": \"...\"}"
}
```

#### Response
```json
{
  "id": "plot_001",
  "title": "El Ascenso del Héroe",
  "summary": "Un héroe accidental debe enfrentar sus miedos para salvar su comunidad.",
  "status": "draft",
  "protagonist_id": "ACCIDENTAL_HERO",
  "antagonist_ids": ["ANTAGONIST_001"],
  "theme_ids": ["THEME_001", "THEME_002"],
  "event_ids": ["EVENT_001", "EVENT_002"],
  "setting_ids": ["SETTING_001"],
  "finale_id": "FINALE_001",
  "extendedNotes": "Generada desde catálogo genérico"
}
```

---

### 4. Sugerir Elementos del Catálogo

**Agente**: ContextoHistorico
**Acción**: `suggest_catalog_elements`
**Sección**: Tramas - Workflow

#### Request
```json
{
  "agente": "ContextoHistorico",
  "mensaje": "Para el proyecto: {PROYECTO_JSON}. Sugiere story elements adecuados del siguiente catálogo: {ELEMENTOS_SNIPPET}. Devuelve SOLO JSON con un array de elementos recomendados: [{\"id\": \"...\", \"name\": \"...\", \"type\": \"...\", \"reason\": \"...\"}]"
}
```

#### Response
```json
[
  {
    "id": "ACCIDENTAL_HERO",
    "name": "Héroe Accidental",
    "type": "Protagonist",
    "reason": "Adecuado para el arco de crecimiento del proyecto"
  },
  {
    "id": "THEME_001",
    "name": "Justicia vs Injusticia",
    "type": "Theme",
    "reason": "Alinea con el conflicto central del proyecto"
  }
]
```

---

### 5. Validar Trama con Catálogo

**Agente**: Verificacion
**Acción**: `validate_with_catalog`
**Sección**: Tramas - Workflow

#### Request
```json
{
  "agente": "Verificacion",
  "mensaje": "Valida la siguiente trama contra el catálogo: Trama: {TRAMA_JSON}. Catálogo: {ELEMENTOS_SNIPPET}. Devuelve SOLO JSON con el formato: {\"valid\": true/false, \"issues\": [...], \"suggestions\": [...]}"
}
```

#### Response
```json
{
  "valid": true,
  "issues": [],
  "suggestions": [
    "Considerar añadir un antagonista más fuerte",
    "El tema X podría reforzarse"
  ]
}
```

O con errores:
```json
{
  "valid": false,
  "issues": [
    "El protagonista no está en el catálogo",
    "El tema Y no es coherente con el género"
  ],
  "suggestions": [
    "Usar el protagonista PROTAGONIST_001",
    "Cambiar el tema a THEME_002"
  ]
}
```

---

### 6. Generar Evento desde Story Element

**Agente**: CoordinadorNarrativo
**Acción**: `generate_from_story_element`
**Sección**: Eventos

#### Request
```json
{
  "agente": "CoordinadorNarrativo",
  "mensaje": "Generar un evento completo basado en el siguiente story element: {STORY_ELEMENT_JSON}. Contexto del proyecto: {PROYECTO_JSON}. Devuelve SOLO JSON con el formato: {\"id\": \"...\", \"label\": \"...\", \"title\": \"...\", \"story_element_id\": \"...\", \"body_text\": \"...\", \"flavor_text\": \"...\", \"choices\": [...], \"consequences\": [...], \"assets\": [...]}"
}
```

#### Response
```json
{
  "id": "event_001",
  "label": "encuentro_con_mentor",
  "title": "El Encuentro con el Mentor",
  "story_element_id": "STORY_ELEMENT_001",
  "body_text": "El protagonista conoce a un personaje sabio que le dará las claves para su viaje.",
  "flavor_text": "Una figura misteriosa aparece en el camino, envuelta en una capa oscura...",
  "choices": [
    {
      "id": "choice_001",
      "label": "Aceptar la ayuda",
      "outcome": "El mentor se convierte en tu guía"
    },
    {
      "id": "choice_002",
      "label": "Rechazar la ayuda",
      "outcome": "El protagonista debe enfrentar el desafío solo"
    }
  ],
  "consequences": [
    "Ganas acceso a conocimiento oculto",
    "Pierdes la oportunidad de aprender"
  ],
  "assets": ["mentor_portrait", "forest_background"]
}
```

---

## 📝 Estructura del Proyecto (Contexto)

El objeto `project` que se envía a los agentes tiene la siguiente estructura:

```json
{
  "projectMeta": {
    "id": "string",
    "title": "string",
    "summary": "string",
    "format": "theatre_play" | "novel" | "videogame_campaign" | "interactive_fiction" | "series" | "film",
    "worldContext": "string",
    "allowedGenres": ["string"],
    "toneProfile": {
      "seriousnessMin": 1-5,
      "seriousnessMax": 1-5,
      "darknessMin": 1-5,
      "darknessMax": 1-5
    },
    "contentLimits": {
      "maxRating": "G" | "PG" | "PG-13" | "R" | "NC-17",
      "blockedSensitivityTags": ["string"]
    },
    "productionConstraints": {
      "maxCastSize": number | null,
      "maxLocations": number | null,
      "budgetBand": "low" | "medium" | "high"
    },
    "notes": "string"
  },
  "narratives": [
    {
      "id": "string",
      "title": "string",
      "summary": "string",
      "description": "string",
      "act": "Act1" | "Act2" | "Act3",
      "tone": "string",
      "historicalscope": "string",
      "spaces": ["string"],
      "factions": ["string"],
      "stakes": ["string"],
      "tags": ["string"],
      "extendedNotes": "string"
    }
  ],
  "plots": [
    {
      "id": "string",
      "title": "string",
      "summary": "string",
      "status": "draft" | "review" | "approved",
      "protagonist_id": "string",
      "antagonist_ids": ["string"],
      "supporting_ids": ["string"],
      "theme_ids": ["string"],
      "event_ids": ["string"],
      "setting_ids": ["string"],
      "finale_id": "string",
      "extendedNotes": "string"
    }
  ],
  "storyelements": [
    {
      "id": "string",
      "label": "string",
      "type": "Theme" | "Scenario" | "Procedure" | "Antagonist" | "Secondary" | "Protagonist" | "DramaticResource",
      "tone": "string",
      "description": "string"
    }
  ],
  "events": [
    {
      "id": "string",
      "label": "string",
      "title": "string",
      "story_element_id": "string",
      "body_text": "string",
      "flavor_text": "string",
      "choices": [
        {
          "id": "string",
          "label": "string",
          "outcome": "string"
        }
      ],
      "consequences": ["string"],
      "assets": ["string"]
    }
  ]
}
```

---

## 🎨 Estructura de Story Element (desde CSV)

Los story elements del catálogo genérico tienen la siguiente estructura (basada en los archivos CSV):

```json
{
  "id": "ACCIDENTAL_HERO",
  "english_name": "Accidental Hero",
  "spanish_name": "Héroe accidental",
  "category": "PROTAGONIST",
  "subtype": "heroic_archetype",
  "role_in_story": "protagonist",
  "logline_usage": "Una persona corriente se ve obligada a convertirse en héroe tras un incidente inesperado.",
  "dramatic_function": "Crear identificación y arco de crecimiento; convertir un personaje cotidiano en figura heroica.",
  "arc_phase_affinity": "setup;inciting_incident;first_plot_point;midpoint;climax",
  "scene_type_affinity": "confrontation;action_setpiece;intimate_dialogue",
  "primary_genres": "DRAMA;ACTION",
  "secondary_genres": "DRAMEDY;ACTION_COMEDY",
  "tone_seriousness": 4,
  "tone_darkness": 3,
  "tone_stylization": "naturalistic",
  "humor_presence": "light",
  "emotional_core": "hope;fear",
  "moral_axis": "justice_vs_injustice;self_vs_community",
  "character_change_potential": "major",
  "narrative_scale": "personal;community",
  "conflict_type": "internal;relational;social",
  "stakes_level": 4,
  "default_periods": "20th_century;contemporary",
  "default_settings": "MODERN_AMERICAN_CITY;MODERN_EUROPEAN_CITY",
  "period_flexibility": "high",
  "worldbuilding_requirement": "low",
  "sensitivity_tags": "",
  "rating_floor": "PG-13",
  "hays_code_relevant": false,
  "professional_notes": "Arquetipo central de cine comercial...",
  "examples_reference": "",
  "script_usage_notes": "Presentar pronto la normalidad del personaje...",
  "production_implications": "Puede requerir escenas de acción...",
  "source": "game_data;script_theory_extended",
  "curation_status": "validated",
  "tags_engine": "hero;ordinary_person;growth_arc",
  "priority_for_generation": 0.8,
  "version": 1
}
```

---

## 🔧 Recomendaciones para los Agentes

### Para CoordinadorNarrativo
- **Enfoque**: Generación de contenido narrativo coherente
- **Contexto necesario**: projectMeta, narratives, catalogElements
- **Formato de salida**: Siempre JSON válido, sin markdown
- **Estilo**: Respuestas directas y estructuradas

### Para ContextoHistorico
- **Enfoque**: Validación de coherencia histórica y temática
- **Contexto necesario**: projectMeta (especialmente worldContext, allowedGenres, toneProfile)
- **Formato de salida**: JSON con recomendaciones o validaciones
- **Estilo**: Análisis crítico y sugerencias concretas

### Para Verificacion
- **Enfoque**: Detección de errores y validación
- **Contexto necesario**: Todo el proyecto o la sección específica a validar
- **Formato de salida**: JSON con status (valid/invalid), issues, suggestions
- **Estilo**: Preciso, con lista de problemas y soluciones

---

## 📌 Notas Importantes

1. **Formato estricto**: Los agentes **DEBEN** devolver SOLO JSON válido, sin texto adicional, markdown o explicaciones fuera del JSON.

2. **Campos obligatorios**: Cada respuesta debe incluir todos los campos del formato esperado, incluso si son vacíos.

3. **IDs consistentes**: Los IDs de story elements, tramas, eventos, etc. deben ser consistentes y únicos.

4. **Lenguaje**: Las respuestas pueden estar en español o inglés según el contexto del proyecto.

5. **Error handling**: Si hay un error, devolver `{ "status": "error", "message": "descripción del error" }`

---

## 🔄 Actualizaciones

Este documento se actualizará conforme se añadan nuevas acciones o se modifiquen los contratos existentes.

**Última actualización**: 2026-07-12
**Versión**: 1.0
