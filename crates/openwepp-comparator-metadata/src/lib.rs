//! Typed comparator confidence-tier routing metadata.

#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

use std::error::Error;
use std::fmt;

/// Message id for single-OFE daily surfaces routed to higher confidence tier.
pub const COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID: &str =
    "COMPMETA-HC-SINGLE-OFE-DAILY-001";
/// Message id for hourly surfaces routed to investigation tier.
pub const COMPMETA_INVESTIGATION_HOURLY_MESSAGE_ID: &str = "COMPMETA-I-HOURLY-001";
/// Message id for watershed surfaces routed to investigation tier.
pub const COMPMETA_INVESTIGATION_WATERSHED_MESSAGE_ID: &str = "COMPMETA-I-WATERSHED-001";

/// Message id for missing required OFE-count metadata.
pub const COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID: &str = "COMPMETA-E-MISSING-OFE-COUNT";
/// Message id for invalid OFE-count values (must be >= 1).
pub const COMPMETA_ERROR_INVALID_OFE_COUNT_MESSAGE_ID: &str = "COMPMETA-E-INVALID-OFE-COUNT";
/// Message id for single-OFE routing mismatch (count must equal 1).
pub const COMPMETA_ERROR_SINGLE_OFE_COUNT_MISMATCH_MESSAGE_ID: &str =
    "COMPMETA-E-SINGLE-OFE-COUNT-MISMATCH";

/// Comparator confidence tier class from ADR-0011 governance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComparatorConfidenceTier {
    HigherConfidence,
    Investigation,
}

impl ComparatorConfidenceTier {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HigherConfidence => "higher_confidence",
            Self::Investigation => "investigation",
        }
    }
}

/// Comparator surface class used for deterministic tier routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComparatorSurfaceClass {
    SingleOfeDailyWaterBalance,
    HourlyWaterBalance,
    WatershedWaterBalance,
}

impl ComparatorSurfaceClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleOfeDailyWaterBalance => "single_ofe_daily_water_balance",
            Self::HourlyWaterBalance => "hourly_water_balance",
            Self::WatershedWaterBalance => "watershed_water_balance",
        }
    }
}

/// Typed request surface for comparator confidence-tier routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComparatorTierRoutingRequest {
    pub surface_class: ComparatorSurfaceClass,
    pub contributor_ofe_count: Option<u32>,
}

impl ComparatorTierRoutingRequest {
    #[must_use]
    pub const fn new(
        surface_class: ComparatorSurfaceClass,
        contributor_ofe_count: Option<u32>,
    ) -> Self {
        Self {
            surface_class,
            contributor_ofe_count,
        }
    }

    #[must_use]
    pub const fn single_ofe_daily() -> Self {
        Self {
            surface_class: ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
            contributor_ofe_count: Some(1),
        }
    }
}

/// Routed metadata attached to reporting/comparator outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComparatorTierRoutingMetadata {
    pub surface_class: ComparatorSurfaceClass,
    pub confidence_tier: ComparatorConfidenceTier,
    pub message_id: &'static str,
}

/// Typed failure class for invalid routing metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComparatorTierRoutingFailureClass {
    MissingRequiredMetadata,
    InvalidMetadata,
}

/// Routing errors for invalid comparator metadata paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComparatorTierRoutingError {
    MissingRequiredMetadata {
        field: &'static str,
        message_id: &'static str,
    },
    InvalidContributorOfeCount {
        contributor_ofe_count: u32,
        message_id: &'static str,
    },
    SingleOfeCountMismatch {
        contributor_ofe_count: u32,
        message_id: &'static str,
    },
}

impl ComparatorTierRoutingError {
    #[must_use]
    pub const fn failure_class(self) -> ComparatorTierRoutingFailureClass {
        match self {
            Self::MissingRequiredMetadata { .. } => {
                ComparatorTierRoutingFailureClass::MissingRequiredMetadata
            }
            Self::InvalidContributorOfeCount { .. } | Self::SingleOfeCountMismatch { .. } => {
                ComparatorTierRoutingFailureClass::InvalidMetadata
            }
        }
    }

    #[must_use]
    pub const fn message_id(self) -> &'static str {
        match self {
            Self::MissingRequiredMetadata { message_id, .. }
            | Self::InvalidContributorOfeCount { message_id, .. }
            | Self::SingleOfeCountMismatch { message_id, .. } => message_id,
        }
    }
}

impl fmt::Display for ComparatorTierRoutingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredMetadata { field, .. } => {
                write!(f, "missing required comparator metadata field: {field}")
            }
            Self::InvalidContributorOfeCount {
                contributor_ofe_count,
                ..
            } => {
                write!(
                    f,
                    "invalid contributor OFE count: {contributor_ofe_count}; expected >= 1"
                )
            }
            Self::SingleOfeCountMismatch {
                contributor_ofe_count,
                ..
            } => {
                write!(
                    f,
                    "single OFE daily comparator surface requires contributor_ofe_count == 1; received {contributor_ofe_count}"
                )
            }
        }
    }
}

impl Error for ComparatorTierRoutingError {}

/// Deterministically route comparator confidence-tier metadata.
///
/// # Errors
///
/// Returns [`ComparatorTierRoutingError`] for invalid or missing metadata.
pub fn route_comparator_tier_metadata(
    request: ComparatorTierRoutingRequest,
) -> Result<ComparatorTierRoutingMetadata, ComparatorTierRoutingError> {
    let count = request.contributor_ofe_count;
    if matches!(count, Some(0)) {
        return Err(ComparatorTierRoutingError::InvalidContributorOfeCount {
            contributor_ofe_count: 0,
            message_id: COMPMETA_ERROR_INVALID_OFE_COUNT_MESSAGE_ID,
        });
    }

    match request.surface_class {
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance => {
            let contributor_ofe_count =
                count.ok_or(ComparatorTierRoutingError::MissingRequiredMetadata {
                    field: "contributor_ofe_count",
                    message_id: COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID,
                })?;

            if contributor_ofe_count != 1 {
                return Err(ComparatorTierRoutingError::SingleOfeCountMismatch {
                    contributor_ofe_count,
                    message_id: COMPMETA_ERROR_SINGLE_OFE_COUNT_MISMATCH_MESSAGE_ID,
                });
            }

            Ok(ComparatorTierRoutingMetadata {
                surface_class: ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
                confidence_tier: ComparatorConfidenceTier::HigherConfidence,
                message_id: COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID,
            })
        }
        ComparatorSurfaceClass::HourlyWaterBalance => Ok(ComparatorTierRoutingMetadata {
            surface_class: ComparatorSurfaceClass::HourlyWaterBalance,
            confidence_tier: ComparatorConfidenceTier::Investigation,
            message_id: COMPMETA_INVESTIGATION_HOURLY_MESSAGE_ID,
        }),
        ComparatorSurfaceClass::WatershedWaterBalance => Ok(ComparatorTierRoutingMetadata {
            surface_class: ComparatorSurfaceClass::WatershedWaterBalance,
            confidence_tier: ComparatorConfidenceTier::Investigation,
            message_id: COMPMETA_INVESTIGATION_WATERSHED_MESSAGE_ID,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_ofe_daily_routes_to_higher_confidence() {
        let metadata = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
            ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
            Some(1),
        ))
        .expect("single OFE daily should route");

        assert_eq!(
            metadata.confidence_tier,
            ComparatorConfidenceTier::HigherConfidence
        );
        assert_eq!(
            metadata.message_id,
            COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID
        );
    }

    #[test]
    fn hourly_and_watershed_route_to_investigation() {
        for request in [
            ComparatorTierRoutingRequest::new(ComparatorSurfaceClass::HourlyWaterBalance, Some(3)),
            ComparatorTierRoutingRequest::new(
                ComparatorSurfaceClass::WatershedWaterBalance,
                Some(2),
            ),
            ComparatorTierRoutingRequest::new(ComparatorSurfaceClass::HourlyWaterBalance, None),
        ] {
            let metadata = route_comparator_tier_metadata(request).expect("route should succeed");
            assert_eq!(
                metadata.confidence_tier,
                ComparatorConfidenceTier::Investigation
            );
        }
    }

    #[test]
    fn missing_single_ofe_count_is_typed_error() {
        let error = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
            ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
            None,
        ))
        .expect_err("missing count should fail");

        assert_eq!(
            error,
            ComparatorTierRoutingError::MissingRequiredMetadata {
                field: "contributor_ofe_count",
                message_id: COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID,
            }
        );
        assert_eq!(
            error.failure_class(),
            ComparatorTierRoutingFailureClass::MissingRequiredMetadata
        );
    }

    #[test]
    fn non_positive_ofe_count_is_typed_error() {
        let error = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
            ComparatorSurfaceClass::HourlyWaterBalance,
            Some(0),
        ))
        .expect_err("non-positive count should fail");

        assert_eq!(
            error,
            ComparatorTierRoutingError::InvalidContributorOfeCount {
                contributor_ofe_count: 0,
                message_id: COMPMETA_ERROR_INVALID_OFE_COUNT_MESSAGE_ID,
            }
        );
        assert_eq!(
            error.failure_class(),
            ComparatorTierRoutingFailureClass::InvalidMetadata
        );
    }

    #[test]
    fn single_ofe_count_mismatch_is_typed_error() {
        let error = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
            ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
            Some(3),
        ))
        .expect_err("count mismatch should fail");

        assert_eq!(
            error,
            ComparatorTierRoutingError::SingleOfeCountMismatch {
                contributor_ofe_count: 3,
                message_id: COMPMETA_ERROR_SINGLE_OFE_COUNT_MISMATCH_MESSAGE_ID,
            }
        );
        assert_eq!(
            error.failure_class(),
            ComparatorTierRoutingFailureClass::InvalidMetadata
        );
    }
}
