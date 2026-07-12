# Cadiz12 SDK Eventos + Narrative UI

Este proyecto incluye el SDK de eventos y una UI web local mínima para editar narrativa, story elements y eventos.

## Ejecutar la UI

```bash
cargo run
```

Después abre:

```text
http://127.0.0.1:7878
```

## Qué incluye la UI

- Sección de Narrativa.
- Sección de Story Elements.
- Sección de Eventos.
- Sección de Validación.
- Sección de Exportación.
- Botones contextuales de IA por sección.

## Estado actual

La integración con IA está mockeada en endpoints locales para poder iterar sobre la interfaz.

Endpoints actuales:

- `POST /api/ai/narrative`
- `POST /api/ai/story-elements`
- `POST /api/ai/event`
- `POST /api/ai/review`

La persistencia del proyecto se guarda en:

```text
project_data/project.json
```

## Siguiente paso recomendado

Sustituir los mocks de IA por llamadas reales a `secretario.py` y mapear la salida a las estructuras canónicas del SDK.

