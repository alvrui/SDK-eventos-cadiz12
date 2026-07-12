# Arquitectura de información de la UI

## Layout general

### Sidebar izquierda

Navegación principal:

- Proyecto
- Narrativa
- Story Elements
- Eventos
- Validación
- Exportación
- Historial IA

### Barra superior

Elementos:

- nombre del proyecto;
- estado de validación;
- botón guardar;
- botón exportar;
- acceso a configuración;
- indicador de conexión con IA.

### Workspace central

Contiene la pantalla principal de edición según la fase activa.

### Panel derecho

Asistente IA contextual.

Debe mostrar:

- acciones disponibles en la sección actual;
- prompt o contexto resumido;
- respuesta más reciente;
- variantes sugeridas;
- botón para aplicar o descartar.

## Regla de visibilidad

Cuando el usuario cambie de sección, el panel derecho debe cambiar también sus acciones de IA para reflejar el contexto activo.

## Historial IA

Debe existir una sección o drawer con:

- solicitud realizada;
- sección desde la que se lanzó;
- agente usado;
- timestamp;
- resultado aplicado o descartado.

## Estados visuales

Cada bloque editable debería poder mostrarse como:

- vacío;
- borrador;
- generado por IA;
- editado manualmente;
- validado;
- con errores.

