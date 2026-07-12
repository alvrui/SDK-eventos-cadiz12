# Acciones de IA por sección

## Objetivo

Definir de manera explícita qué botones de IA deben existir en la interfaz y qué payload deben enviar según la sección activa.

## Narrativa

### Botón: Generar propuesta narrativa

Entrada contextual:

- título provisional;
- tono;
- acto;
- facciones seleccionadas;
- espacios seleccionados;
- stakes;
- brief editorial.

Salida esperada:

- resumen narrativo;
- conflicto central;
- posibles subconflictos;
- tags iniciales;
- warnings.

### Botón: Refinar tono

Entrada contextual:

- narrativa actual;
- tono deseado;
- restricciones de lore.

Salida esperada:

- versión reescrita;
- observaciones de ajuste;
- términos a evitar.

### Botón: Sugerir conflictos

Entrada contextual:

- narrativa base;
- facciones y stakes.

Salida esperada:

- lista de conflictos plausibles;
- nivel de intensidad;
- justificación diegética.

## Story Elements

### Botón: Proponer story elements desde narrativa

Entrada contextual:

- narrativa aprobada;
- acto;
- tono;
- facciones;
- espacios;
- tags.

Salida esperada:

- themes sugeridos;
- protagonistas;
- antagonistas;
- secundarios;
- escenarios;
- procedimientos;
- recursos dramáticos.

### Botón: Añadir variantes

Entrada contextual:

- conjunto de elementos actual;
- huecos detectados;
- objetivo de variedad.

Salida esperada:

- alternativas compatibles;
- explicación de uso;
- grado de riesgo tonal.

### Botón: Reequilibrar conjunto

Entrada contextual:

- story elements ya aceptados.

Salida esperada:

- elementos redundantes;
- elementos faltantes;
- propuesta de sustitución.

## Eventos

### Botón: Generar evento desde story element

Entrada contextual:

- story element origen;
- narrativa activa;
- contexto del proyecto;
- elementos ya usados.

Salida esperada:

- estructura de evento completa;
- textos base;
- decisiones;
- consecuencias;
- tags;
- assets sugeridos.

### Botón: Proponer textos

Entrada contextual:

- tipo de evento;
- tono;
- situación;
- facciones;
- decisiones previstas.

Salida esperada:

- título;
- subtítulo;
- cuerpo;
- microtextos opcionales.

### Botón: Proponer decisiones

Entrada contextual:

- evento en edición;
- stakes;
- rol del jugador.

Salida esperada:

- opciones de decisión;
- coste percibido;
- consecuencias resumidas;
- tono de cada opción.

### Botón: Proponer assets visuales

Entrada contextual:

- evento;
- localización;
- atmósfera;
- referencias visuales disponibles.

Salida esperada:

- prompts visuales;
- lista de iconos o motifs;
- sugerencias de composición.

### Botón: Detectar incoherencias

Entrada contextual:

- evento completo;
- narrativa global;
- tags activos.

Salida esperada:

- errores;
- warnings;
- propuestas de reparación.

## Validación global

### Botón: Revisar coherencia global

Entrada contextual:

- narrativa;
- story elements aceptados;
- eventos creados.

Salida esperada:

- problemas de continuidad;
- redundancias;
- saltos de tono;
- elementos huérfanos.

### Botón: Sugerir mejoras editoriales

Entrada contextual:

- proyecto completo.

Salida esperada:

- mejoras priorizadas;
- quick wins;
- zonas que conviene rehacer.

## Exportación

### Botón: Preparar textos para exportación

Entrada contextual:

- todos los eventos listos para exportar.

Salida esperada:

- normalización de texto;
- ajustes de consistencia;
- observaciones finales.

### Botón: Generar checklist final

Entrada contextual:

- proyecto completo y exportable.

Salida esperada:

- checklist de revisión;
- riesgos pendientes;
- confirmación de completitud.

