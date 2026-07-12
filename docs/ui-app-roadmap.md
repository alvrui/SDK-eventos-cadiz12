# UI app roadmap para el editor narrativo de Cadiz12

## Objetivo

Construir una aplicación con interfaz gráfica para definir narrativas, pedir propuestas a IA por sección, transformar propuestas en story elements, derivar eventos jugables completos y exportar configuración final para el juego.

## Idea de producto

La aplicación no debe pensarse como un CLI ni como un visor técnico del SDK. Debe funcionar como un editor de autoría narrativa asistida.

El flujo principal será:

1. Definir narrativa base.
2. Pulsar botones de IA para obtener propuestas contextuales.
3. Seleccionar y editar story elements.
4. Generar un evento por story element.
5. Completar textos, decisiones, recursos gráficos y metadatos.
6. Validar el conjunto.
7. Exportar a configuración del juego.

## Regla UX principal

Cada sección importante debe tener uno o varios botones explícitos para llamar a la IA usando el contexto activo de esa sección.

La IA no debe vivir en una sola pantalla global. Debe estar embebida en el flujo de trabajo.

## Pantallas

### 1. Narrativa

Campos:

- título de la narrativa;
- resumen editorial;
- acto;
- tono;
- scope histórico;
- espacios clave;
- facciones clave;
- stakes principales;
- tags iniciales;
- tags bloqueantes.

Botones IA:

- `Generar propuesta narrativa`;
- `Refinar tono`;
- `Sugerir conflictos`;
- `Sugerir facciones y espacios compatibles`.

### 2. Story Elements

Sección para listar y editar propuestas de themes, protagonists, antagonists, secondaries, scenarios, procedures y recursos dramáticos.

Botones IA:

- `Proponer story elements desde narrativa`;
- `Añadir variantes`;
- `Completar elementos faltantes`;
- `Reequilibrar conjunto`;
- `Explicar por qué encajan`.

### 3. Eventos

Cada story element aceptado puede expandirse a un evento jugable completo.

Campos por evento:

- id;
- label;
- título visible;
- texto descriptivo;
- texto de contexto;
- decisiones del jugador;
- consecuencias;
- requisitos;
- tags generados;
- tono;
- assets o referencias gráficas.

Botones IA por evento:

- `Generar evento desde story element`;
- `Proponer textos`;
- `Proponer decisiones`;
- `Proponer consecuencias`;
- `Proponer assets visuales`;
- `Reescribir en tono Cadiz12`;
- `Detectar incoherencias`.

### 4. Validación y preview

Vista para revisar:

- continuidad narrativa;
- coherencia tonal;
- consistencia de tags;
- huecos de contenido;
- calidad de decisiones;
- cobertura de recursos gráficos y de texto.

Botones IA:

- `Revisar coherencia global`;
- `Sugerir mejoras editoriales`;
- `Resumir arco narrativo`;
- `Encontrar repeticiones o puntos débiles`.

### 5. Exportación

Opciones para exportar a JSON o YAML del juego, o a un bundle editorial intermedio.

Botones IA:

- `Preparar textos para exportación`;
- `Normalizar nomenclatura`;
- `Generar checklist de validación final`.

## Arquitectura propuesta

### Frontend

UI web o Tauri con layout de editor:

- sidebar de navegación por fases;
- workspace central;
- panel derecho con asistente IA contextual;
- barra superior con proyecto, validación y exportación.

### Backend

Backend local apoyado en el SDK Rust para:

- cargar catálogos;
- validar estructuras;
- serializar exportes;
- recibir propuestas IA y mapearlas a objetos internos.

### Servicio IA

Conexión a `secretario.py` mediante endpoints claros por operación.

## Principio clave

La IA debe poder invocarse desde cualquier sección relevante con un botón visible y una acción contextual. Ese punto no es accesorio: es parte del diseño de producto.

