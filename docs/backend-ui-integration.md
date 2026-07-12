# Integración backend UI + SDK + secretario.py

## Objetivo

Definir cómo se conectan la interfaz, el SDK de eventos y el servicio de IA.

## Capas

### 1. UI

Responsable de:

- recoger edición del usuario;
- disparar acciones IA por sección;
- mostrar validación y preview;
- lanzar exportaciones.

### 2. Backend de aplicación

Responsable de:

- persistir proyectos;
- mapear formularios a modelos internos;
- invocar funciones del SDK;
- llamar a `secretario.py`;
- transformar respuestas IA en propuestas consumibles por la UI.

### 3. SDK Rust

Responsable de:

- catálogos;
- validación;
- estructuras canónicas;
- serialización de salida.

### 4. secretario.py

Responsable de:

- orquestación de agentes;
- generación contextual;
- reescritura;
- propuesta de variantes;
- reparación editorial asistida.

## Endpoint model sugerido

### Narrativa

- `POST /api/ai/narrative/propose`
- `POST /api/ai/narrative/refine-tone`
- `POST /api/ai/narrative/suggest-conflicts`

### Story elements

- `POST /api/ai/story-elements/propose`
- `POST /api/ai/story-elements/variants`
- `POST /api/ai/story-elements/rebalance`

### Eventos

- `POST /api/ai/events/from-story-element`
- `POST /api/ai/events/propose-texts`
- `POST /api/ai/events/propose-decisions`
- `POST /api/ai/events/propose-assets`
- `POST /api/ai/events/check-consistency`

### Validación global

- `POST /api/ai/project/review`
- `POST /api/ai/project/editorial-suggestions`

### Exportación

- `POST /api/ai/export/prepare-texts`
- `POST /api/ai/export/final-checklist`

## Respuesta backend estándar

```json
{
  "status": "success",
  "section": "events",
  "action": "propose_decisions",
  "data": {},
  "warnings": [],
  "meta": {
    "agent": "CoordinadorNarrativo",
    "timestamp": "2026-07-05T23:54:00",
    "applied": false
  }
}
```

## Regla funcional importante

Toda acción IA debe poder:

- verse antes de aplicarse;
- aplicarse parcialmente;
- descartarse;
- registrarse en historial.

