//! Generador de prompts mejorados para agentes Mistral

use serde_json::Value;
use crate::domain::structs::NarrativeElements;

/// Generador de prompts mejorados
#[derive(Debug, Clone)]
pub struct PromptBuilder {
    context_builder: ContextBuilder,
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            context_builder: ContextBuilder::new(),
        }
    }

    /// Construir prompt para narrative_elements
    pub fn build_narrative_elements_prompt(
        &self,
        action: &str,
        project: &Value,
        existing_elements: Option<&NarrativeElements>,
    ) -> String {
        let context = self.context_builder.build_context(project, existing_elements);
        
        match action {
            "generate" => self.build_generate_prompt(&context),
            "refine" => self.build_refine_prompt(&context),
            "suggest" => self.build_suggest_prompt(&context),
            "expand" => self.build_expand_prompt(&context),
            _ => self.build_generate_prompt(&context),
        }
    }

    /// Construir prompt para narrativa
    pub fn build_narrative_prompt(
        &self,
        action: &str,
        project: &Value,
    ) -> String {
        let context = self.context_builder.build_narrative_context(project);
        
        match action {
            "generate" => self.build_narrative_generate_prompt(&context),
            "refine" => self.build_narrative_refine_prompt(&context),
            _ => self.build_narrative_generate_prompt(&context),
        }
    }

    /// Construir prompt para eventos
    pub fn build_event_prompt(
        &self,
        action: &str,
        project: &Value,
        story_element_id: Option<&str>,
    ) -> String {
        let context = self.context_builder.build_event_context(project, story_element_id);
        
        match action {
            "generate_from_story_element" => self.build_event_from_story_element_prompt(&context),
            "propose_texts" => self.build_event_texts_prompt(&context),
            "propose_decisions" => self.build_event_decisions_prompt(&context),
            _ => self.build_event_from_story_element_prompt(&context),
        }
    }

    /// Construir prompt para personajes
    pub fn build_characters_prompt(
        &self,
        action: &str,
        project: &Value,
    ) -> String {
        let context = self.context_builder.build_characters_context(project);
        
        match action {
            "propose" => self.build_characters_propose_prompt(&context),
            "refine" => self.build_characters_refine_prompt(&context),
            _ => self.build_characters_propose_prompt(&context),
        }
    }

    // ========== Prompts para narrative_elements ==========

    fn build_generate_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de elementos narrativos unificados.

== CONTEXTO ==
{context}

== ACCIÓN ==
Generar elementos narrativos completos para el proyecto.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. No inventes IDs definitivos. Si no conoces un ID exacto, usa un objeto con campo "unresolved": true
3. Usa solo valores de enum válidos (Act1, Act2, Act3, Act4, Festive, Solemn, etc.)
4. Incluye todos los campos requeridos en cada tipo de elemento
5. Prioriza coherencia local, causalidad jugable y tensión política
6. Genera entre 3 y 8 elementos por tipo

== FORMATO DE SALIDA ==
{{
  "status": "success",
  "section": "narrative_elements",
  "action": "generate",
  "data": {{
    "themes": [
      {{
        "id": "string",
        "type": "Theme",
        "label": "string",
        "description": "string",
        "tone": "string (enum: Festive, Satirical, Anxious, Solemn, Intimate, Conspiratorial, Patriotic, Sordid, Tragic, Ambiguous, Tense, Polemical, Funereal, Resilient, Combative, Compassionate)",
        "historical_scope": "string (enum: StrictHistorical, PlausibleDocumented, PlausibleInferred, ExceptionalButVerisimilar, Discarded)",
        "time_window": ["string (enum: Y1805_1808, Y1809, Y1810, Y1811, Y1812, Y1813, Y1814, Y1815_1816)"],
        "act_bias": ["string (enum: Act1, Act2, Act3, Act4)"],
        "stakes_axis": ["string (enum: Personal, Political, Urban, Institutional, Imperial, Moral, Economic, Religious, Military, Media)"],
        "faction_vectors": ["string"],
        "space_vectors": ["string"],
        "compatibility_tags": ["string"],
        "blocking_tags": ["string"],
        "unlock_tags": ["string"],
        "generated_tags": ["string"]
      }}
    ],
    "protagonists": [
      {{
        "id": "string",
        "type": "Protagonist",
        "label": "string",
        "description": "string",
        "eligible_profiles": ["string"],
        "eligible_positions": ["string"]
      }}
    ],
    "antagonists": [
      {{
        "id": "string",
        "type": "Antagonist",
        "label": "string",
        "description": "string"
      }}
    ],
    "secondaries": [
      {{
        "id": "string",
        "type": "Secondary",
        "label": "string",
        "description": "string"
      }}
    ],
    "scenarios": [
      {{
        "id": "string",
        "type": "Scenario",
        "label": "string",
        "description": "string"
      }}
    ],
    "procedures": [
      {{
        "id": "string",
        "type": "Procedure",
        "label": "string",
        "description": "string",
        "kind": "string (enum: DebatePlenary, TechnicalCommission, EmergencySession, HonorSession, DecreeVote, PriorPositioningCall, PrivateNegotiation, PetitionSubmission, DocumentReading, PressPublication, PressDenunciation, StrategicLeak, StrategicDelay, AmendmentProposal, SecretVoteRequest, SafeConductProcessing, HousingAssignment, HealthDeclaration, NeighborhoodRelief, SignatureCirculation)"
      }}
    ],
    "dramatic_resources": [
      {{
        "id": "string",
        "type": "DramaticResource",
        "label": "string",
        "description": "string"
      }}
    ],
    "social_pressures": [
      {{
        "id": "string",
        "type": "SocialPressure",
        "label": "string",
        "description": "string"
      }}
    ]
  }},
  "warnings": []
}}"#
        )
    }

    fn build_refine_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de elementos narrativos unificados.

== CONTEXTO ==
{context}

== ACCIÓN ==
Refinar/improve los elementos narrativos existentes.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. Mantén los IDs existentes
3. Mejora los campos: label, description, tone, etc.
4. Añade campos faltantes si es necesario
5. No elimines elementos existentes

== FORMATO DE SALIDA ==
Mismo formato que para "generate", pero con los elementos mejorados.

== NOTAS ==
- Enfócate en mejorar la coherencia y calidad de los elementos
- Mantén el tono y estilo del proyecto
- Añade detalles que falten"#
        )
    }

    fn build_suggest_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de elementos narrativos unificados.

== CONTEXTO ==
{context}

== ACCIÓN ==
Sugerir elementos narrativos que complementen los existentes.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. Sugiere elementos que llenen vacíos en el proyecto
3. Asegúrate de que los nuevos elementos sean compatibles con los existentes
4. Genera entre 2 y 5 elementos por tipo

== FORMATO DE SALIDA ==
Mismo formato que para "generate", pero solo con los nuevos elementos sugeridos.

== NOTAS ==
- Analiza qué tipos de elementos faltan
- Sugiere elementos que complementen los existentes
- Asegúrate de que haya variedad"#
        )
    }

    fn build_expand_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de elementos narrativos unificados.

== CONTEXTO ==
{context}

== ACCIÓN ==
Expandir un elemento narrativo específico.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. Expande el elemento con más detalles
3. Añade campos opcionales que falten
4. Mantén el ID y tipo del elemento

== FORMATO DE SALIDA ==
Mismo formato, pero con el elemento expandido.

== NOTAS ==
- Añade detalles históricos
- Expande descripciones
- Añade tags de compatibilidad"#
        )
    }

    // ========== Prompts para narrativa ==========

    fn build_narrative_generate_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de narrativa.

== CONTEXTO ==
{context}

== ACCIÓN ==
Generar una propuesta narrativa completa.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. Incluye todos los campos requeridos
3. Usa solo valores de enum válidos
4. Prioriza coherencia histórica y tensión política

== FORMATO DE SALIDA ==
{{
  "status": "success",
  "section": "narrative",
  "action": "generate",
  "data": {{
    "title": "string",
    "summary": "string",
    "description": "string",
    "act": "string (enum: Act1, Act2, Act3, Act4)",
    "tone": "string (enum: Festive, Satirical, Anxious, Solemn, Intimate, Conspiratorial, Patriotic, Sordid, Tragic, Ambiguous, Tense, Polemical, Funereal, Resilient, Combative, Compassionate)",
    "historical_scope": "string (enum: StrictHistorical, PlausibleDocumented, PlausibleInferred, ExceptionalButVerisimilar, Discarded)",
    "spaces": ["string"],
    "factions": ["string"],
    "stakes": ["string"],
    "tags": ["string"],
    "extendedNotes": "string"
  }},
  "warnings": []
}}"#
        )
    }

    fn build_narrative_refine_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de narrativa.

== CONTEXTO ==
{context}

== ACCIÓN ==
Refinar la narrativa existente.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. Mejora los campos: title, summary, description
3. Mantén el tono y acto
4. Añade detalles que falten

== FORMATO DE SALIDA ==
Mismo formato que para "generate", pero con los campos mejorados."#
        )
    }

    // ========== Prompts para eventos ==========

    fn build_event_from_story_element_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de eventos.

== CONTEXTO ==
{context}

== ACCIÓN ==
Generar evento desde un story element.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. Genera un evento completo y jugable
3. Incluye: id, label, title, story_element_id, body_text, choices, consequences
4. Asegúrate de que el evento sea coherente con el story element
5. Genera entre 2 y 4 opciones de decisión

== FORMATO DE SALIDA ==
{{
  "status": "success",
  "section": "events",
  "action": "generate_from_story_element",
  "data": {{
    "id": "string",
    "label": "string",
    "title": "string",
    "story_element_id": "string",
    "body_text": "string",
    "flavor_text": "string",
    "choices": [
      {{
        "id": "string",
        "label": "string",
        "outcome": "string"
      }}
    ],
    "consequences": ["string"],
    "assets": ["string"]
  }},
  "warnings": []
}}

== NOTAS ==
- El body_text debe ser claro y descriptivo
- Las choices deben tener consecuencias diferentes
- El evento debe ser jugable y tener impacto en la narrativa"#
        )
    }

    fn build_event_texts_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de eventos.

== CONTEXTO ==
{context}

== ACCIÓN ==
Proponer textos para un evento.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. Genera textos claros y descriptivos
3. Incluye: title, subtitle (opcional), body_text, flavor_text (opcional)
4. Mantén el tono del proyecto

== FORMATO DE SALIDA ==
{{
  "status": "success",
  "section": "events",
  "action": "propose_texts",
  "data": {{
    "id": "string",
    "label": "string",
    "title": "string",
    "subtitle": "string",
    "body_text": "string",
    "flavor_text": "string"
  }},
  "warnings": []
}}"#
        )
    }

    fn build_event_decisions_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de eventos.

== CONTEXTO ==
{context}

== ACCIÓN ==
Proponer decisiones para un evento.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. Genera entre 2 y 4 opciones de decisión
3. Cada opción debe tener: id, label, outcome
4. Las decisiones deben ser significativas y tener consecuencias diferentes

== FORMATO DE SALIDA ==
{{
  "status": "success",
  "section": "events",
  "action": "propose_decisions",
  "data": {{
    "id": "string",
    "choices": [
      {{
        "id": "string",
        "label": "string",
        "outcome": "string"
      }}
    ]
  }},
  "warnings": []
}}

== NOTAS ==
- Las decisiones deben ser claras y distintas
- Cada decisión debe tener un outcome diferente
- Las decisiones deben afectar la narrativa"#
        )
    }

    // ========== Prompts para personajes ==========

    fn build_characters_propose_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de personajes.

== CONTEXTO ==
{context}

== ACCIÓN ==
Proponer personajes para el proyecto.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. Genera personajes coherentes con el contexto histórico
3. Incluye: id, name, projectRoleType, summary, description
4. Genera entre 3 y 6 personajes

== FORMATO DE SALIDA ==
{{
  "status": "success",
  "section": "characters",
  "action": "propose",
  "data": [
    {{
      "id": "string",
      "name": "string",
      "projectRoleType": "string (enum: protagonist, antagonist, supporting, flex)",
      "summary": "string",
      "description": "string",
      "extendedNotes": "string",
      "futureCompat": {{}},
      "narrativeProfile": {{
        "dramaticFunctions": ["string"],
        "coreDrives": ["string"],
        "traits": ["string"],
        "toneFit": ["string"]
      }},
      "worldFit": {{
        "allowedPeriods": ["string"],
        "preferredSettings": ["string"],
        "genreAffinity": ["string"]
      }},
      "relationships": []
    }}
  ],
  "warnings": []
}}

== NOTAS ==
- Los personajes deben ser históricos o plausibles
- Cada personaje debe tener un rol claro
- Los personajes deben ser compatibles entre sí"#
        )
    }

    fn build_characters_refine_prompt(&self, context: &str) -> String {
        format!(
            r#"Devuelve SOLO JSON válido. No uses markdown. No expliques nada fuera del JSON.

Estás trabajando para Cadiz12 en la sección de personajes.

== CONTEXTO ==
{context}

== ACCIÓN ==
Refinar personajes existentes.

== REQUISITOS ==
1. Devuelve UNICAMENTE JSON válido
2. Mejora los campos: name, summary, description
3. Mantén los IDs existentes
4. Añade detalles que falten

== FORMATO DE SALIDA ==
Mismo formato que para "propose", pero con los personajes mejorados."#
        )
    }
}

/// Constructor de contexto
#[derive(Debug, Clone)]
pub struct ContextBuilder;

impl ContextBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Construir contexto para narrative_elements
    pub fn build_context(
        &self,
        project: &Value,
        existing_elements: Option<&NarrativeElements>,
    ) -> String {
        let mut context_parts = Vec::new();
        
        // Información del proyecto
        self.add_project_meta(&mut context_parts, project);
        
        // Narrativas
        self.add_narratives(&mut context_parts, project);
        
        // Elementos existentes
        if let Some(elements) = existing_elements {
            self.add_existing_elements(&mut context_parts, elements);
        }
        
        // Restricciones
        self.add_constraints(&mut context_parts, project);
        
        context_parts.join("\n")
    }

    /// Construir contexto para narrativa
    pub fn build_narrative_context(&self, project: &Value) -> String {
        let mut context_parts = Vec::new();
        
        self.add_project_meta(&mut context_parts, project);
        self.add_constraints(&mut context_parts, project);
        
        context_parts.join("\n")
    }

    /// Construir contexto para eventos
    pub fn build_event_context(
        &self,
        project: &Value,
        story_element_id: Option<&str>,
    ) -> String {
        let mut context_parts = Vec::new();
        
        self.add_project_meta(&mut context_parts, project);
        self.add_narratives(&mut context_parts, project);
        
        if let Some(id) = story_element_id {
            context_parts.push(format!("Story Element ID: {}", id));
        }
        
        self.add_constraints(&mut context_parts, project);
        
        context_parts.join("\n")
    }

    /// Construir contexto para personajes
    pub fn build_characters_context(&self, project: &Value) -> String {
        let mut context_parts = Vec::new();
        
        self.add_project_meta(&mut context_parts, project);
        self.add_narratives(&mut context_parts, project);
        self.add_constraints(&mut context_parts, project);
        
        context_parts.join("\n")
    }

    /// Añadir información del proyecto
    fn add_project_meta(&self, context: &mut Vec<String>, project: &Value) {
        context.push("== PROYECTO ==".to_string());
        
        if let Some(meta) = project.get("projectMeta") {
            if let Some(title) = meta.get("title").and_then(|v| v.as_str()) {
                context.push(format!("Título: {}", title));
            }
            if let Some(summary) = meta.get("summary").and_then(|v| v.as_str()) {
                context.push(format!("Resumen: {}", summary));
            }
            if let Some(format) = meta.get("format").and_then(|v| v.as_str()) {
                context.push(format!("Formato: {}", format));
            }
            if let Some(world_context) = meta.get("worldContext").and_then(|v| v.as_str()) {
                context.push(format!("Contexto del mundo: {}", world_context));
            }
        }
    }

    /// Añadir narrativas
    fn add_narratives(&self, context: &mut Vec<String>, project: &Value) {
        if let Some(narratives) = project.get("narratives").and_then(|v| v.as_array()) {
            if !narratives.is_empty() {
                context.push("\n== NARRATIVAS ==".to_string());
                for nar in narratives {
                    let id = nar.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let title = nar.get("title").and_then(|v| v.as_str()).unwrap_or("");
                    let act = nar.get("act").and_then(|v| v.as_str()).unwrap_or("");
                    let tone = nar.get("tone").and_then(|v| v.as_str()).unwrap_or("");
                    context.push(format!("- ID: {}, Título: {}, Acto: {}, Tono: {}", id, title, act, tone));
                }
            }
        }
    }

    /// Añadir elementos existentes
    fn add_existing_elements(&self, context: &mut Vec<String>, elements: &NarrativeElements) {
        context.push("\n== ELEMENTOS EXISTENTES ==".to_string());
        context.push(format!("Themes: {}", elements.themes.len()));
        context.push(format!("Protagonists: {}", elements.protagonists.len()));
        context.push(format!("Antagonists: {}", elements.antagonists.len()));
        context.push(format!("Secondaries: {}", elements.secondaries.len()));
        context.push(format!("Scenarios: {}", elements.scenarios.len()));
        context.push(format!("Procedures: {}", elements.procedures.len()));
        context.push(format!("Dramatic Resources: {}", elements.dramatic_resources.len()));
        context.push(format!("Social Pressures: {}", elements.social_pressures.len()));
    }

    /// Añadir restricciones
    fn add_constraints(&self, context: &mut Vec<String>, project: &Value) {
        context.push("\n== RESTRICCIONES ==".to_string());
        
        if let Some(meta) = project.get("projectMeta") {
            if let Some(tone_profile) = meta.get("toneProfile") {
                if let Some(min) = tone_profile.get("seriousnessMin").and_then(|v| v.as_u64()) {
                    if let Some(max) = tone_profile.get("seriousnessMax").and_then(|v| v.as_u64()) {
                        context.push(format!("Seriedad: {}-{}", min, max));
                    }
                }
                if let Some(min) = tone_profile.get("darknessMin").and_then(|v| v.as_u64()) {
                    if let Some(max) = tone_profile.get("darknessMax").and_then(|v| v.as_u64()) {
                        context.push(format!("Oscuridad: {}-{}", min, max));
                    }
                }
            }
            
            if let Some(content_limits) = meta.get("contentLimits") {
                if let Some(max_rating) = content_limits.get("maxRating").and_then(|v| v.as_str()) {
                    context.push(format!("Clasificación máxima: {}", max_rating));
                }
                if let Some(blocked_tags) = content_limits.get("blockedSensitivityTags").and_then(|v| v.as_array()) {
                    if !blocked_tags.is_empty() {
                        let tags: Vec<String> = blocked_tags.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect();
                        context.push(format!("Tags bloqueados: {}", tags.join(", ")));
                    }
                }
            }
        }
    }
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_prompt_builder_creation() {
        let builder = PromptBuilder::new();
        assert!(!builder.context_builder.build_context(&json!({}), None).is_empty());
    }

    #[test]
    fn test_build_generate_prompt() {
        let builder = PromptBuilder::new();
        let prompt = builder.build_narrative_elements_prompt("generate", &json!({}), None);
        
        assert!(prompt.contains("Devuelve SOLO JSON válido"));
        assert!(prompt.contains("narrative_elements"));
        assert!(prompt.contains("generate"));
        assert!(prompt.contains("themes"));
        assert!(prompt.contains("protagonists"));
    }

    #[test]
    fn test_build_refine_prompt() {
        let builder = PromptBuilder::new();
        let prompt = builder.build_narrative_elements_prompt("refine", &json!({}), None);
        
        assert!(prompt.contains("Refinar/improve"));
        assert!(prompt.contains("refine"));
    }

    #[test]
    fn test_build_narrative_prompt() {
        let builder = PromptBuilder::new();
        let prompt = builder.build_narrative_prompt("generate", &json!({}));
        
        assert!(prompt.contains("narrative"));
        assert!(prompt.contains("title"));
        assert!(prompt.contains("summary"));
    }

    #[test]
    fn test_context_builder() {
        let builder = ContextBuilder::new();
        let project = json!({
            "projectMeta": {
                "title": "Test Project",
                "summary": "Test Summary",
                "format": "theatre_play",
                "worldContext": "Test World"
            },
            "narratives": [{
                "id": "nar_1",
                "title": "Narrative 1",
                "act": "Act1",
                "tone": "Ambiguous"
            }]
        });
        
        let context = builder.build_context(&project, None);
        
        assert!(context.contains("Test Project"));
        assert!(context.contains("Test Summary"));
        assert!(context.contains("Narrative 1"));
    }
}
