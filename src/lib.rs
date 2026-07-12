//! # Cadiz12 SDK Eventos
//!
//! SDK de eventos narrativo-políticos para el juego Cádiz 1812.
//! 
//! Este crate proporciona:
//! - Tipos del dominio para elementos de guion
//! - Catálogos de temas, protagonistas, antagonistas, secundarios, escenarios y procedimientos
//! - Estado del mundo y del protagonista
//! - Matrices de compatibilidad entre categorías
//! - Algoritmo de selección de eventos basado en puntuaciones ponderadas
//! - Instanciación de eventos con trazabilidad completa

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod catalog;
pub mod compat;
pub mod domain;
pub mod state;
pub mod scoring;
pub mod selector;
pub mod templates;
pub mod runtime;
pub mod validation;
pub mod trace;

// Re-exportar módulos principales
pub use catalog::*;
pub use compat::*;
pub use domain::*;
pub use state::*;
pub use scoring::*;
pub use selector::*;
pub use templates::*;
pub use runtime::*;
pub use validation::*;
pub use trace::*;

/// Versión del SDK
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Nombre del SDK
pub const SDK_NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdk_constants() {
        assert_eq!(SDK_NAME, "cadiz12-sdk-eventos");
        assert!(!SDK_VERSION.is_empty());
    }
}
