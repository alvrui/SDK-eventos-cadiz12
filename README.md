# SDK Eventos Cádiz 1812

![Cadiz 1812](https://img.shields.io/badge/Cadiz-1812-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

SDK de eventos narrativo-políticos para el juego **Cádiz 1812**. Este proyecto proporciona herramientas para generar, validar y gestionar contenido narrativo usando agentes de IA (Mistral).

## 📋 Tabla de Contenidos

- [Instalación](#-instalación)
- [Estructura del Proyecto](#-estructura-del-proyecto)
- [Configuración](#-configuración)
- [Ejecutar la Aplicación](#-ejecutar-la-aplicación)
- [Endpoints de API](#-endpoints-de-api)
- [Testing](#-testing)
- [Desarrollo](#-desarrollo)
- [Documentación](#-documentación)
- [Contribuir](#-contribuir)
- [Licencia](#-licencia)

---

## 📥 Instalación

### Requisitos Previos

1. **Rust** (versión 1.70 o superior)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Dependencias del Sistema** (para compilación completa)
   ```bash
   # En Ubuntu/Debian
   sudo apt-get install pkg-config libssl-dev
   
   # En Fedora
   sudo dnf install pkg-config openssl-devel
   
   # En macOS (con Homebrew)
   brew install pkg-config openssl
   ```

3. **Clonar el Repositorio**
   ```bash
   git clone https://github.com/alvrui/SDK-eventos-cadiz12.git
   cd SDK-eventos-cadiz12
   ```

---

## 🏗 Estructura del Proyecto

```
SDK-eventos-cadiz12/
├── Cargo.toml                 # Dependencias y configuración
├── src/
│   ├── lib.rs                 # Biblioteca principal
│   ├── main.rs                # Servidor web y endpoints de API
│   ├── ai/                    # Módulo de gestión de agentes IA
│   │   ├── mod.rs             # Exports del módulo AI
│   │   ├── schemas.rs         # Schemas JSON para validación
│   │   ├── validator.rs       # Validador de respuestas
│   │   ├── logger.rs          # Logger de interacciones
│   │   ├── feedback.rs        # Sistema de feedback
│   │   └── prompts.rs         # Generador de prompts
│   ├── domain/                # Tipos del dominio
│   │   ├── enums.rs           # Enums (TimeSlice, HistoricalScope, etc.)
│   │   ├── ids.rs             # IDs fuertes
│   │   └── structs.rs         # Estructuras (Theme, NarrativeElements, etc.)
│   ├── catalog/               # Catálogos
│   ├── state/                 # Estado del juego
│   ├── validation/            # Validación
│   ├── selector/              # Selección de eventos
│   ├── runtime/               # Ejecución de eventos
│   └── trace/                 # Trazabilidad
├── docs/                      # Documentación
│   ├── ANALYSIS_BUGS_AND_IMPROVEMENTS.md  # Análisis de bugs
│   ├── CHANGELOG.md          # Historial de cambios
│   ├── IMPLEMENTATION_STATUS.md  # Estado de implementación
│   ├── PHASE2_PLAN.md         # Plan de Fase 2
│   ├── PHASE3_PLAN.md         # Plan de Fase 3
│   └── FINAL_SUMMARY.md       # Resumen final
├── ui/                        # Interfaz web (estática)
├── project_data/              # Datos de proyectos
└── logs/                      # Logs de interacciones con IA
```

---

## ⚙ Configuración

### Configurar Agentes de IA

El proyecto usa `secretario.py` como orquestador de agentes Mistral. Configura los agentes en el archivo de proyecto:

```json
{
  "settings": {
    "agents": {
      "project": "CoordinadorNarrativo",
      "narrative": "CoordinadorNarrativo",
      "characters": "CoordinadorNarrativo",
      "plots": "DiseñadorDeStoryElements",
      "story_elements": "DiseñadorDeStoryElements",
      "narrative_elements": "CoordinadorNarrativo",
      "events": "DiseñadorDeEventos",
      "validation": "RevisorNarrativo",
      "export": "EditorDeExportacion"
    }
  }
}
```

### Configurar secretario.py

Asegúrate de que `secretario.py` esté ejecutándose y accesible en `http://127.0.0.1:8000`:

```bash
# Ejemplo de ejecución (ajustar según tu configuración)
python3 secretario.py --port 8000
```

### Variables de Entorno

```bash
# Puerto del servidor (opcional, default: 7879)
export UI_HOST="0.0.0.0:7879"

# URL de secretario.py (opcional, default: http://127.0.0.1:8000)
export SECRETARIO_BASE_URL="http://127.0.0.1:8000"

# Directorio de logs (opcional, default: logs/agents)
export AI_LOG_DIR="logs/agents"
```

---

## ▶ Ejecutar la Aplicación

### Compilar y Ejecutar

```bash
# Compilar en modo debug (para desarrollo)
cargo build

# Ejecutar el servidor
cargo run
```

El servidor se iniciará en `http://0.0.0.0:7879` y mostrará:
```
Cadiz12 Narrative UI en http://0.0.0.0:7879
```

### Compilar en Modo Release (Producción)

```bash
# Compilar con optimizaciones
cargo build --release

# Ejecutar la versión optimizada
./target/release/cadiz12-sdk-eventos
```

---

## 🌐 Endpoints de API

### Endpoints Principales

| Método | Endpoint | Descripción | Agente por Defecto |
|--------|----------|-------------|-------------------|
| GET | `/` | Interfaz web | - |
| GET | `/api/project` | Obtener proyecto actual | - |
| POST | `/api/project` | Guardar proyecto | - |
| GET | `/api/agents` | Listar agentes disponibles | - |

### Endpoints de IA (Generación de Contenido)

| Método | Endpoint | Descripción | Agente por Defecto |
|--------|----------|-------------|-------------------|
| POST | `/api/ai/project` | Generar/refinar proyecto | CoordinadorNarrativo |
| POST | `/api/ai/narrative` | Generar/refinar narrativa | CoordinadorNarrativo |
| POST | `/api/ai/narrative-elements` | Generar elementos narrativos unificados | CoordinadorNarrativo |
| POST | `/api/ai/plots` | Generar/refinar tramas | DiseñadorDeStoryElements |
| POST | `/api/ai/story-elements` | Generar elementos de story | DiseñadorDeStoryElements |
| POST | `/api/ai/event` | Generar/refinar evento | DiseñadorDeEventos |
| POST | `/api/ai/characters` | Generar/refinar personajes | CoordinadorNarrativo |
| POST | `/api/ai/review` | Revisión editorial | RevisorNarrativo |
| POST | `/api/ai/export` | Preparar exportación | EditorDeExportacion |

### Ejemplo de Uso de Endpoints

#### 1. Obtener Proyecto Actual

```bash
curl -X GET http://localhost:7879/api/project
```

#### 2. Generar Narrativa

```bash
curl -X POST http://localhost:7879/api/ai/narrative \
  -H "Content-Type: application/json" \
  -d '{
    "action": "generate",
    "project": {
      "projectMeta": {
        "title": "Mi Proyecto",
        "summary": "Un proyecto sobre la Constitución de Cádiz"
      }
    }
  }'
```

#### 3. Generar Elementos Narrativos Unificados

```bash
curl -X POST http://localhost:7879/api/ai/narrative-elements \
  -H "Content-Type: application/json" \
  -d '{
    "action": "generate",
    "project": {
      "projectMeta": {
        "title": "Mi Proyecto",
        "summary": "Un proyecto sobre la Constitución de Cádiz"
      },
      "narratives": [{
        "id": "nar_1",
        "title": "Primera narrativa",
        "act": "Act1",
        "tone": "Ambiguous"
      }]
    }
  }'
```

#### 4. Generar Evento

```bash
curl -X POST http://localhost:7879/api/ai/event \
  -H "Content-Type: application/json" \
  -d '{
    "action": "generate_from_story_element",
    "project": {
      "projectMeta": {
        "title": "Mi Proyecto"
      }
    },
    "story_element_id": "theme_1"
  }'
```

#### 5. Revisión Editorial

```bash
curl -X POST http://localhost:7879/api/ai/review \
  -H "Content-Type: application/json" \
  -d '{
    "action": "review",
    "project": {
      "projectMeta": {
        "title": "Mi Proyecto"
      },
      "narratives": [...],
      "plots": [...],
      "events": [...]
    }
  }'
```

---

## 🧪 Testing

### Ejecutar Todos los Tests

```bash
# Ejecutar todos los tests
cargo test

# Ejecutar tests con logging
RUST_LOG=debug cargo test
```

### Tests Específicos

#### Tests del Módulo AI

```bash
# Tests de schemas
cargo test ai::schemas -- --nocapture

# Tests de validador
cargo test ai::validator -- --nocapture

# Tests de logger
cargo test ai::logger -- --nocapture

# Tests de feedback
cargo test ai::feedback -- --nocapture

# Tests de prompts
cargo test ai::prompts -- --nocapture
```

#### Tests de Dominio

```bash
# Tests de enums
cargo test domain::enums -- --nocapture

# Tests de structs
cargo test domain::structs -- --nocapture

# Tests de world_state
cargo test state::world_state -- --nocapture

# Tests de protagonist_state
cargo test state::protagonist_state -- --nocapture
```

#### Tests de Validación

```bash
# Tests de validación
cargo test validation -- --nocapture
```

#### Tests de Selección

```bash
# Tests de selector
cargo test selector -- --nocapture
```

### Verificar Compilación

```bash
# Verificar que el código compila (sin ejecutar)
cargo check

# Verificar solo la biblioteca
cargo check --lib
```

---

## 🛠 Desarrollo

### Estructura de un Proyecto

Un proyecto típico tiene la siguiente estructura:

```json
{
  "projectMeta": {
    "id": "proyecto_1",
    "title": "La Constitución de Cádiz",
    "summary": "Proyecto sobre las Cortes de Cádiz",
    "format": "theatre_play",
    "worldContext": "España, 1810-1814",
    "allowedGenres": ["Drama", "Historia"],
    "toneProfile": {
      "seriousnessMin": 3,
      "seriousnessMax": 5,
      "darknessMin": 2,
      "darknessMax": 4
    },
    "contentLimits": {
      "maxRating": "PG-13",
      "blockedSensitivityTags": []
    },
    "productionConstraints": {
      "maxCastSize": 10,
      "maxLocations": 5,
      "budgetBand": "medium"
    },
    "notes": ""
  },
  "narratives": [
    {
      "id": "nar_1",
      "title": "El Debate Constitucional",
      "summary": "Debate sobre la soberanía nacional",
      "description": "...",
      "act": "Act2",
      "tone": "Solemn",
      "historicalscope": "PlausibleDocumented",
      "spaces": ["cortes", "sala_de_debates"],
      "factions": ["liberales", "absolutistas"],
      "stakes": ["soberania", "libertad"],
      "tags": ["constitucion", "debate"],
      "extendedNotes": "...",
      "cast": {
        "rules": {
          "protagonistMode": "single",
          "maxAntagonists": 2,
          "maxSupporting": 4
        },
        "entries": []
      }
    }
  ],
  "narrative_elements": {
    "themes": [...],
    "protagonists": [...],
    "antagonists": [...],
    "secondaries": [...],
    "scenarios": [...],
    "procedures": [...],
    "dramatic_resources": [...],
    "social_pressures": [...]
  },
  "plots": [...],
  "events": [...],
  "review": {
    "summary": "",
    "issues": []
  },
  "settings": {
    "agents": {
      "project": "CoordinadorNarrativo",
      "narrative": "CoordinadorNarrativo",
      "narrative_elements": "CoordinadorNarrativo",
      "plots": "DiseñadorDeStoryElements",
      "events": "DiseñadorDeEventos",
      "validation": "RevisorNarrativo",
      "export": "EditorDeExportacion"
    }
  }
}
```

### Crear un Nuevo Elemento Narrativo

```rust
use cadiz12_sdk_eventos::domain::structs::{NarrativeElements, Theme};
use cadiz12_sdk_eventos::domain::enums::{ScriptElementCategory, HistoricalScope, TimeSlice, Act, Tone, StakesAxis};

let mut elements = NarrativeElements::new();

// Añadir un tema
let mut theme = Theme::new("theme_1");
theme.base.label = "La Soberanía Nacional".to_string();
theme.base.description = "Debate sobre la soberanía de la nación".to_string();
theme.base.tone = Tone::Solemn;
theme.base.historical_scope = HistoricalScope::PlausibleDocumented;
theme.base.time_window = vec![TimeSlice::Y1810, TimeSlice::Y1811];
theme.base.act_bias = vec![Act::Act2];
theme.base.stakes_axis = vec![StakesAxis::Political, StakesAxis::Institutional];

elements = elements.with_theme(theme);
```

### Validar Elementos Narrativos

```rust
use cadiz12_sdk_eventos::validation::NarrativeElementsValidator;

let validator = NarrativeElementsValidator::new();
let result = validator.validate_narrative_elements(&elements);

match result {
    Ok(_) => println!("Elementos válidos"),
    Err(errors) => {
        for error in errors {
            println!("Error: {}", error);
        }
    }
}
```

### Usar el Validador de Respuestas de Agentes

```rust
use cadiz12_sdk_eventos::ai::{AgentResponseValidator, SchemaManager};
use serde_json::json;

let validator = AgentResponseValidator::new();
let response = json!({
    "status": "success",
    "section": "narrative_elements",
    "action": "generate",
    "data": {
        "themes": [...],
        "protagonists": [...],
        // ...
    },
    "warnings": []
});

let result = validator.validate_response("narrative_elements", &response);
match result {
    Ok(_) => println!("Respuesta válida"),
    Err(errors) => {
        for error in errors {
            println!("Error de validación: {}", error);
        }
    }
}
```

---

## 📚 Documentación

- **[Análisis de Bugs y Mejoras](docs/ANALYSIS_BUGS_AND_IMPROVEMENTS.md)** - Análisis detallado de bugs identificados y soluciones
- **[Changelog](docs/CHANGELOG.md)** - Historial de cambios
- **[Estado de Implementación](docs/IMPLEMENTATION_STATUS.md)** - Estado actual del proyecto
- **[Plan Fase 2](docs/PHASE2_PLAN.md)** - Plan de unificación de secciones
- **[Plan Fase 3](docs/PHASE3_PLAN.md)** - Plan de mejoras en agentes Mistral
- **[Resumen Final](docs/FINAL_SUMMARY.md)** - Resumen completo de todos los cambios

---

## 🤝 Contribuir

1. **Fork** el repositorio
2. **Crea una branch** para tu feature (`git checkout -b feature/nueva-funcionalidad`)
3. **Commit** tus cambios (`git commit -m 'Añadida nueva funcionalidad'`)
4. **Push** a la branch (`git push origin feature/nueva-funcionalidad`)
5. **Abre un Pull Request**

### Convenciones de Commit

- `fix: ` - Para fixes de bugs
- `feat: ` - Para nuevas funcionalidades
- `refactor: ` - Para refactorización de código
- `docs: ` - Para documentación
- `test: ` - Para tests
- `chore: ` - Para tareas de mantenimiento

### Estándares de Código

- Usa `rustfmt` para formatear el código
- Añade tests para nuevas funcionalidades
- Documenta funciones públicas
- Usa nombres descriptivos para variables y funciones

---

## 📜 Licencia

Este proyecto está bajo la Licencia **MIT**. Ver el archivo [LICENSE](LICENSE) para más detalles.

---

## 🙏 Agradecimientos

- A todos los contribuyentes del proyecto
- A la comunidad de Rust por su excelente documentación
- A los desarrolladores de los agentes Mistral por su trabajo en IA

---

## 📞 Soporte

Para preguntas o problemas:
1. Revisa la documentación en `docs/`
2. Revisa los issues abiertos en GitHub
3. Abre un nuevo issue con una descripción detallada

---

**¡Gracias por usar el SDK Eventos Cádiz 1812!** 🎉
