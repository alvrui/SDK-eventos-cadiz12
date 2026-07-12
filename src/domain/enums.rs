//! Enums del dominio - Valores cerrados obligatorios

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Categoría de elemento de guion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ScriptElementCategory {
    Protagonist,
    Antagonist,
    Secondary,
    ThemeEvent,
    Finale,
    Scenario,
    Procedure,
    SocialPressure,
    DramaticResource,
}

/// Alcance histórico
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum HistoricalScope {
    PlausibleDocumented,
    PlausibleInferred,
    ExceptionalButVerisimilar,
    Discarded,
}

/// Ventanas temporales
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TimeSlice {
    Y1805_1808,
    Y1809,
    Y1810,
    Y1811,
    Y1812,
    Y1813,
    Y1814,
    Y1815_1816,
}

/// Actos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Act {
    Act1,
    Act2,
    Act3,
    Act4,
}

/// Ejes dramáticos (Stakes Axis)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum StakesAxis {
    Personal,
    Political,
    Urban,
    Institutional,
    Imperial,
    Moral,
    Economic,
    Religious,
    Military,
    Media,
}

/// Tono
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Tone {
    Festive,
    Satirical,
    Anxious,
    Solemn,
    Intimate,
    Conspiratorial,
    Patriotic,
    Sordid,
    Tragic,
    Ambiguous,
    Tense,
    Polemical,
    Funereal,
    Resilient,
    Combative,
    Compassionate,
}

/// Rol en cadena
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ChainRole {
    Seed,
    Complication,
    Escalation,
    Crisis,
    Revelation,
    Resolution,
    Aftermath,
}

/// Repetibilidad
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Repeatability {
    Unique,
    Rare,
    ControlledRecurring,
    Serial,
}

/// Perfil de visibilidad
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum VisibilityProfile {
    None,
    Discreet,
    Public,
}

/// Perfil de información
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum InformationProfile {
    PublicFact,
    PlausibleRumor,
    ConfidentialDocument,
    NetworkSecret,
    DeliberateAmbiguity,
}

/// Tipo de medidor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum MeterType {
    Influence,
    RelationalCapital,
    Reputation,
    Coherence,
    Resources,
    Stamina,
}

/// Estado global del mundo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum GlobalState {
    TenseNormality,
    PreCrisis,
    OpenCrisis,
    PostCrisisAftermath,
    PublicCelebration,
    LatentRepression,
}

/// Nivel de visibilidad pública del protagonista
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PublicVisibilityLevel {
    Unknown,
    Emerging,
    RecognizableFigure,
    HighlyExposed,
}

/// Trayectoria moral percibida
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum MoralTrajectory {
    Opportunist,
    Coherent,
    Ambiguous,
    Reliable,
    Feared,
    Indispensable,
}

/// Clima de espacio
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SpaceClimate {
    Calm,
    Saturated,
    Nervous,
    Watched,
    Empty,
    Effervescent,
}

/// Fase de crisis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum CrisisPhase {
    Signal,
    Outbreak,
    ReactionPeriod,
    Resolution,
    Aftermath,
}

/// Nivel de relación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RelationshipLevel {
    Unknown,
    Contact,
    Ally,
    IntimatePolitical,
    Rival,
    Enemy,
}

/// Estado de relación
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RelationshipState {
    Stable,
    Resentful,
    Tense,
    Grateful,
    Broken,
    UnderReview,
}

/// Tipo de plantilla de escena (A-F)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SceneTemplateType {
    AInstitutionalSession,
    BUrbanEncounter,
    CPrivateVisit,
    DDocumentReading,
    EPublicCrisis,
    FPersonalConsequence,
}

/// Tipo de serie de presión cotidiana
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PressureSeriesType {
    MorningMail,
    AfternoonEdition,
    CallToPosition,
    PendingCommitmentPressure,
    RumorWithConsequence,
}

/// Tipo de procedimiento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ProcedureKind {
    DebatePlenary,
    TechnicalCommission,
    EmergencySession,
    HonorSession,
    DecreeVote,
    PriorPositioningCall,
    PrivateNegotiation,
    PetitionSubmission,
    DocumentReading,
    PressPublication,
    PressDenunciation,
    StrategicLeak,
    StrategicDelay,
    AmendmentProposal,
    SecretVoteRequest,
    SafeConductProcessing,
    HousingAssignment,
    HealthDeclaration,
    NeighborhoodRelief,
    SignatureCirculation,
}

/// Puntuación de compatibilidad (0-5)
pub type CompatibilityScore = u8;

/// Validación de puntuación de compatibilidad
pub fn validate_compatibility_score(score: CompatibilityScore) -> Result<(), String> {
    if score <= 5 {
        Ok(())
    } else {
        Err(format!("Compatibility score {} is out of range (0-5)", score))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compatibility_score_validation() {
        assert!(validate_compatibility_score(0).is_ok());
        assert!(validate_compatibility_score(5).is_ok());
        assert!(validate_compatibility_score(6).is_err());
    }

    #[test]
    fn test_enum_display() {
        assert_eq!(format!("{}", HistoricalScope::PlausibleDocumented), "plausible_documented");
        assert_eq!(format!("{}", SceneTemplateType::AInstitutionalSession), "a_institutional_session");
        assert_eq!(format!("{}", ProcedureKind::DebatePlenary), "debate_plenary");
    }

    #[test]
    fn test_enum_from_str() {
        assert_eq!("plausible_documented".parse::<HistoricalScope>().ok(), Some(HistoricalScope::PlausibleDocumented));
        assert_eq!("a_institutional_session".parse::<SceneTemplateType>().ok(), Some(SceneTemplateType::AInstitutionalSession));
    }
}
