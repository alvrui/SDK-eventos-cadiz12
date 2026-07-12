//! Newtypes para identificadores fuertes

use serde::{Deserialize, Serialize};
use std::fmt;

/// Identificador genérico para elementos
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElementId(pub String);

/// Identificador para temas
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThemeId(pub String);

/// Identificador para protagonistas
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProtagonistId(pub String);

/// Identificador para antagonistas
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AntagonistId(pub String);

/// Identificador para secundarios
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SecondaryId(pub String);

/// Identificador para escenarios
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScenarioId(pub String);

/// Identificador para procedimientos
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcedureId(pub String);

/// Identificador para facciones
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FactionId(pub String);

/// Identificador para espacios
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpaceId(pub String);

/// Identificador para recursos dramáticos
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceId(pub String);

/// Identificador para tags
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId(pub String);

/// Identificador para instancias de evento
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventInstanceId(pub String);

/// Identificador para plantillas de escena
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SceneTemplateId(pub String);

/// Identificador para posiciones formales
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FormalPositionId(pub String);

/// Identificador para perfiles de jugador
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerProfileRef(pub String);

// Implementaciones comunes para todos los IDs
macro_rules! impl_id_traits {
    ($type:ty) => {
        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<String> for $type {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $type {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }
        
        impl From<&String> for $type {
            fn from(s: &String) -> Self {
                Self(s.clone())
            }
        }
        
        impl From<&$type> for $type {
            fn from(id: &$type) -> Self {
                Self(id.0.clone())
            }
        }

        impl AsRef<str> for $type {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

impl_id_traits!(ElementId);
impl_id_traits!(ThemeId);
impl_id_traits!(ProtagonistId);
impl_id_traits!(AntagonistId);
impl_id_traits!(SecondaryId);
impl_id_traits!(ScenarioId);
impl_id_traits!(ProcedureId);
impl_id_traits!(FactionId);
impl_id_traits!(SpaceId);
impl_id_traits!(ResourceId);
impl_id_traits!(TagId);
impl_id_traits!(EventInstanceId);
impl_id_traits!(SceneTemplateId);
impl_id_traits!(FormalPositionId);
impl_id_traits!(PlayerProfileRef);

// Conversions from ElementId to specific ID types
impl From<ElementId> for ThemeId {
    fn from(id: ElementId) -> Self {
        Self(id.0)
    }
}

impl From<ElementId> for ProtagonistId {
    fn from(id: ElementId) -> Self {
        Self(id.0)
    }
}

impl From<ElementId> for AntagonistId {
    fn from(id: ElementId) -> Self {
        Self(id.0)
    }
}

impl From<ElementId> for SecondaryId {
    fn from(id: ElementId) -> Self {
        Self(id.0)
    }
}

impl From<ElementId> for ScenarioId {
    fn from(id: ElementId) -> Self {
        Self(id.0)
    }
}

impl From<ElementId> for ProcedureId {
    fn from(id: ElementId) -> Self {
        Self(id.0)
    }
}

impl From<ElementId> for FactionId {
    fn from(id: ElementId) -> Self {
        Self(id.0)
    }
}

impl From<ElementId> for SpaceId {
    fn from(id: ElementId) -> Self {
        Self(id.0)
    }
}

impl From<ElementId> for ResourceId {
    fn from(id: ElementId) -> Self {
        Self(id.0)
    }
}

impl From<ElementId> for TagId {
    fn from(id: ElementId) -> Self {
        Self(id.0)
    }
}
