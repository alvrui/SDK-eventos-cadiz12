//! # Cadiz12 SDK Eventos
//!
//! SDK de eventos narrativo-polticos para el juego Cdiz 1812.
//! 
//! Este crate proporciona:
//! - Tipos del dominio para elementos de guion
//! - Catlogo de temas, protagonistas, antagonistas, secundarios, escenarios y procedimientos
//! - Estado del mundo y del protagonista
//! - Matrices de compatibilidad entre categors
//! - Algoritmo de seleccin de eventos basado en puntuaciones ponderadas
//! - Instanciacin de eventos con trazabilidad completa
//! - Gestión de agentes Mistral para generacin de contenido

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod ai;
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

// Re-exportar mdulos principales de forma especfica para evitar ambigedad
pub use ai::schemas::*;
pub use ai::validator::*;
pub use ai::logger::*;
pub use ai::feedback::*;
pub use ai::prompts::*;

pub use catalog::catalog::*;
pub use catalog::loader::*;
pub use catalog::theme_bindings::*;

pub use compat::matrices::*;
pub use compat::loader::*;

pub use domain::ids::*;
pub use domain::enums::*;
pub use domain::structs::*;

pub use state::world_state::*;
pub use state::protagonist_state::*;
pub use state::relationship_state::*;

pub use scoring::*;

pub use selector::*;

pub use templates::*;

pub use runtime::*;

pub use validation::*;

pub use trace::*;

/// Versin del SDK
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
