# Integración IA con secretario.py para generación narrativa

## Objetivo

Definir una capa de integración entre el SDK de eventos de Cadiz12 y `secretario.py` para que los agentes de IA puedan proponer configuraciones narrativas y cadenas de eventos coherentes, siempre bajo validación del dominio del SDK.

## Enfoque recomendado

La IA no debe devolver narrativa libre ni texto ornamental como salida principal. Debe devolver artefactos estructurados que el SDK pueda validar, corregir o rechazar.

Flujo propuesto:

1. El editor narrativo recoge un brief corto y restricciones explícitas.
2. La aplicación compone un payload con contexto jugable y contexto de lore.
3. `secretario.py` reenvía la solicitud a uno o varios agentes especializados.
4. El agente devuelve una propuesta en JSON o YAML estricto.
5. El backend valida la propuesta contra enums, ids, tags, ventanas temporales y compatibilidades del SDK.
6. La UI presenta la propuesta como borrador editable, no como verdad final.

## Tipos de salida de IA

Se recomienda soportar, como mínimo, estas operaciones:

- `generate_chain`: propone una cadena narrativa completa.
- `expand_event`: amplía un nodo ya existente con consecuencias o variantes.
- `repair_chain`: repara incoherencias detectadas por el validador.
- `suggest_elements`: propone story elements compatibles con el contexto actual.
- `explain_fit`: resume por qué una propuesta encaja con el lore y el estado actual.

## Contrato de salida sugerido

```json
{
  "operation": "generate_chain",
  "title": "Rastro de pólvora en el muelle",
  "summary": "Cadena corta de tensión entre contrabando, rumor político y reacción faccional.",
  "constraints_used": {
    "act": "Act2",
    "tone": "Ambiguous",
    "space": ["muelle_sur"],
    "factions": ["aduana_local", "circulo_mercantil"]
  },
  "events": [
    {
      "id": "evt_chain_001",
      "label": "Inspección fuera de horario",
      "type": "theme_event",
      "chain_role": "trigger",
      "requires": ["tag_contrabando_latente"],
      "generates": ["tag_sospecha_aduana"],
      "tone": "Ambiguous",
      "stakes": ["control", "reputacion"],
      "notes": "Debe sentirse plausible y local, no épico."
    }
  ],
  "warnings": [],
  "open_questions": [
    "Falta precisar si la facción portuaria ya está activa en este arco."
  ]
}
```

## Principios de integración

- La salida de IA debe usar ids y enums canónicos cuando existan.
- Si la IA no conoce un id exacto, debe marcarlo como candidato, nunca inventarlo como definitivo.
- Toda propuesta debe ser validable sin interpretación humana adicional.
- Los errores de validación deben volver a la IA como feedback estructurado para iterar.
- El sistema debe distinguir entre borrador IA, propuesta validada y cadena aprobada.

## Papel de secretario.py

`secretario.py` puede seguir siendo el punto de entrada operativo, pero conviene usarlo como orquestador, no como contenedor de toda la lógica.

Responsabilidades recomendadas:

- Resolver qué agente o combinación de agentes usar.
- Inyectar prompt base, contexto de lore y restricciones activas.
- Registrar trazabilidad de entradas y salidas.
- Devolver una respuesta estructurada al editor narrativo.

Responsabilidades que no debería asumir:

- Validación semántica profunda del dominio.
- Persistencia editorial final.
- Renderizado rico de la herramienta de edición.

## Prompting recomendado

La solicitud a IA debería incluir siempre:

- objetivo narrativo,
- acto,
- tono,
- facciones implicadas,
- espacios implicados,
- tags activos,
- tags bloqueantes,
- formato de salida exacto,
- regla explícita de no inventar ids definitivos.

Ejemplo de instrucción base:

```text
Genera una propuesta de cadena narrativa para Cadiz12.
Devuelve solo JSON válido.
No escribas introducciones.
No inventes enums fuera del vocabulario dado.
Si falta un id exacto, usa un objeto candidato con campo `unresolved`.
Prioriza coherencia local, tensión política y causalidad jugable.
```

## Validación

La validación debería ejecutarse justo después de la respuesta del agente.

Chequeos mínimos:

- enums válidos;
- ids existentes o marcados como no resueltos;
- tags bloqueantes ausentes;
- compatibilidad de tono y acto;
- consistencia de chain roles;
- dependencias `requires` satisfechas o justificadas.

## Evolución sugerida

Fase 1: generación de borradores de cadena.

Fase 2: reparación automática de incoherencias detectadas.

Fase 3: coautoría asistida, con regeneración parcial de un solo nodo o tramo.

Fase 4: comparación de variantes y puntuación editorial.

