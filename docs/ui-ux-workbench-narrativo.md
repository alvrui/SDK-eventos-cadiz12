# Propuesta de UI/UX para el workbench narrativo de Cadiz12

## Objetivo

Transformar la herramienta en un editor de trabajo narrativo asistido, no en un formulario técnico ni en una consola de logs.

## Modelo de interfaz

Se propone una interfaz de tres paneles:

### 1. Panel izquierdo: catálogo y contexto

Debe incluir:

- búsqueda de story elements;
- filtros por categoría;
- filtros por tono, acto, facción, espacio y stakes;
- resumen del contexto activo del proyecto;
- accesos rápidos a presets de generación.

Este panel responde a una necesidad central del SDK: el dominio ya está organizado por ejes claros, por lo que la UI debe exponer esos ejes de forma visible y operativa.

### 2. Panel central: cadena narrativa

Debe mostrar:

- timeline o grafo lineal de eventos;
- nodos con título, chain role, tags requeridos y tags generados;
- huecos narrativos pendientes;
- conexiones causales entre nodos;
- estado de validación por nodo.

Este panel es el corazón del flujo. La persona diseñadora debe sentir que compone una secuencia dramática y no que rellena campos inconexos.

### 3. Panel derecho: inspector y asistente IA

Debe contener:

- detalle del nodo seleccionado;
- editor JSON o YAML con autocompletado;
- explicación diegética resumida;
- acciones de IA contextual: regenerar nodo, expandir consecuencias, reparar incoherencia, sugerir variantes;
- warnings del validador.

## Flujo principal de usuario

1. Crear o abrir un proyecto narrativo.
2. Definir un brief breve: tono, conflicto, ámbito y facciones.
3. Pulsar “Generar semilla” para obtener una primera cadena.
4. Revisar cada nodo con ayuda del inspector.
5. Validar la cadena completa.
6. Aceptar, editar o regenerar por secciones.

## Mejoras UX prioritarias

### Diferenciar estados

Cada elemento generado debe tener estado visual claro:

- borrador IA,
- validado,
- con warnings,
- aprobado por diseño,
- descartado.

### Mantener trazabilidad

La UI debe permitir ver:

- agente usado,
- prompt base,
- restricciones activas,
- fecha de generación,
- versión del lore pack o del proyecto.

Esto evita perder confianza cuando se comparan variantes o se revisa una generación antigua.

### Reducir fricción al iterar

Acciones rápidas necesarias:

- regenerar solo un nodo;
- regenerar desde aquí hasta final;
- pedir una variante más sobria o más conflictiva;
- bloquear un elemento para que no cambie;
- convertir una propuesta en base aprobada.

### Hacer visibles los errores

Los errores no deben vivir solo en logs. Deben aparecer sobre la propia cadena:

- incompatibilidad de tags;
- conflicto de ventana temporal;
- tono fuera de rango;
- dependencia no satisfecha;
- ids no resueltos.

## Componentes recomendados

- barra superior con proyecto, estado y acciones de guardar/validar/generar;
- chips de contexto activo;
- cards de evento con badges semánticos;
- drawer lateral para variantes IA;
- diff visual entre versiones de una cadena;
- consola técnica plegable para depuración, nunca como vista principal.

## Persistencia recomendada

La UI debería guardar:

- proyecto activo,
- filtros activos,
- nodo seleccionado,
- layout de paneles,
- historial de generaciones,
- variante aprobada actual.

## Criterios de calidad UX

La herramienta será mejor si permite responder con rapidez a estas preguntas:

- qué intenta contar esta cadena;
- por qué este nodo está aquí;
- qué bloquea su validez;
- qué parte propuso la IA;
- qué parte ya fue aprobada por diseño.

