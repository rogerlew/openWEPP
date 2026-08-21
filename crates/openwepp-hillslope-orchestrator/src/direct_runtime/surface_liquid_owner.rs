//! Persistent default-off snow-free surface-liquid hydrology owner.

#![allow(clippy::missing_errors_doc)]

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use openwepp_land_surface_energy::{
    CondensationCredit, GroundWaterKey, OfeId, RequestingComponent, SourceId,
    StandGroundWaterAmountBasis, SurfaceClass, SurfaceId, WaterAmount, WaterAuthorization,
    WaterAuthorizationReason, WaterSourceType,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

mod resource_validation;

use resource_validation::preflight_resource_phase_inputs;

const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const TOPOLOGY_MULTIPLIER: f64 = 64.0;
const JOINT_AUTHORIZATION_SCALE_BIT_DECISIONS: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DirectSurfaceLiquidClosureUnit {
    MassKgM2,
    MassM,
    EnthalpyJM2,
}

pub(crate) fn checked_surface_liquid_add(left: f64, right: f64) -> Option<f64> {
    if !left.is_finite() || !right.is_finite() {
        return None;
    }
    let result = left + right;
    result.is_finite().then_some(result)
}

pub(crate) fn checked_surface_liquid_sum(values: impl IntoIterator<Item = f64>) -> Option<f64> {
    values.into_iter().try_fold(0.0, checked_surface_liquid_add)
}

pub(crate) fn checked_surface_liquid_sub(left: f64, right: f64) -> Option<f64> {
    if !left.is_finite() || !right.is_finite() {
        return None;
    }
    let result = left - right;
    result.is_finite().then_some(result)
}

pub(crate) fn checked_surface_liquid_mul(left: f64, right: f64) -> Option<f64> {
    if !left.is_finite() || !right.is_finite() {
        return None;
    }
    let result = left * right;
    if !result.is_finite() || (result == 0.0 && left != 0.0 && right != 0.0) {
        None
    } else {
        Some(result)
    }
}

pub(crate) fn checked_surface_liquid_div(numerator: f64, denominator: f64) -> Option<f64> {
    if !numerator.is_finite() || !denominator.is_finite() || denominator == 0.0 {
        return None;
    }
    let result = numerator / denominator;
    if !result.is_finite() || (result == 0.0 && numerator != 0.0) {
        None
    } else {
        Some(result)
    }
}

fn authorization_sum_at_common_scale(raw_shares: &[f64], scale: f64) -> Option<f64> {
    if !scale.is_finite() || !(0.0..=1.0).contains(&scale) {
        return None;
    }
    raw_shares.iter().try_fold(0.0, |sum, raw_share| {
        if !raw_share.is_finite() || *raw_share < 0.0 {
            return None;
        }
        let scaled = raw_share * scale;
        if !scaled.is_finite() {
            return None;
        }
        checked_surface_liquid_add(sum, scaled)
    })
}

/// Apply the SC-SURFACELIQUID-001 v6 common representability scale.
fn jointly_safe_proportional_authorizations(raw_shares: &[f64], supply: f64) -> Option<Vec<f64>> {
    let raw_sum = checked_surface_liquid_sum(raw_shares.iter().copied())?;
    if raw_sum <= supply {
        return Some(raw_shares.to_vec());
    }
    if !checked_surface_liquid_close(raw_sum, supply, DirectSurfaceLiquidClosureUnit::MassKgM2)? {
        return None;
    }

    let initial_scale = checked_surface_liquid_div(supply, raw_sum)?;
    if !(0.0..=1.0).contains(&initial_scale) || initial_scale == 0.0 {
        return None;
    }
    let initial_sum = authorization_sum_at_common_scale(raw_shares, initial_scale)?;
    let scale = if initial_sum <= supply {
        initial_scale
    } else {
        let mut lower_bits = 0_u64;
        let mut upper_bits = initial_scale.to_bits();
        let mut decisions = 0_usize;
        while lower_bits + 1 < upper_bits {
            if decisions == JOINT_AUTHORIZATION_SCALE_BIT_DECISIONS {
                return None;
            }
            let middle_bits = lower_bits + (upper_bits - lower_bits) / 2;
            let middle = f64::from_bits(middle_bits);
            if authorization_sum_at_common_scale(raw_shares, middle)? <= supply {
                lower_bits = middle_bits;
            } else {
                upper_bits = middle_bits;
            }
            decisions += 1;
        }
        f64::from_bits(lower_bits)
    };
    if scale == 0.0 {
        return None;
    }

    let authorizations = raw_shares
        .iter()
        .map(|raw_share| checked_surface_liquid_mul(*raw_share, scale))
        .collect::<Option<Vec<_>>>()?;
    if raw_shares
        .iter()
        .zip(&authorizations)
        .any(|(raw_share, authorization)| *raw_share > 0.0 && *authorization == 0.0)
        || checked_surface_liquid_sum(authorizations.iter().copied())? > supply
    {
        return None;
    }
    Some(authorizations)
}

pub(crate) fn checked_surface_liquid_close(
    actual: f64,
    expected: f64,
    unit: DirectSurfaceLiquidClosureUnit,
) -> Option<bool> {
    let absolute = match unit {
        DirectSurfaceLiquidClosureUnit::MassKgM2 => 1.0e-14,
        DirectSurfaceLiquidClosureUnit::MassM => 1.0e-17,
        DirectSurfaceLiquidClosureUnit::EnthalpyJM2 => 1.0e-9,
    };
    let difference = checked_surface_liquid_sub(actual, expected)?.abs();
    let scale = checked_surface_liquid_add(actual.abs(), expected.abs())?;
    let scaled = checked_surface_liquid_mul(TOPOLOGY_MULTIPLIER * f64::EPSILON, scale)?;
    let tolerance = checked_surface_liquid_add(absolute, scaled)?;
    Some(difference <= tolerance)
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum DirectSurfaceLiquidError {
    #[error("surface-liquid schema failure: {0}")]
    Schema(&'static str),
    #[error("surface-liquid identity failure: {0}")]
    Identity(&'static str),
    #[error("surface-liquid domain failure: {0}")]
    Domain(&'static str),
    #[error("surface-liquid protocol failure: {0}")]
    Protocol(&'static str),
    #[error("surface-liquid bound failure: {0}")]
    Bound(&'static str),
    #[error("surface-liquid closure failure: {0}")]
    Closure(&'static str),
    #[error("{0}")]
    Failure(Box<DirectSurfaceLiquidFailure>),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DirectSurfaceLiquidErrorCode {
    #[serde(rename = "SURFACELIQUID-E-001")]
    E001,
    #[serde(rename = "SURFACELIQUID-E-002")]
    E002,
    #[serde(rename = "SURFACELIQUID-E-003")]
    E003,
    #[serde(rename = "SURFACELIQUID-E-004")]
    E004,
    #[serde(rename = "SURFACELIQUID-E-005")]
    E005,
    #[serde(rename = "SURFACELIQUID-E-006")]
    E006,
    #[serde(rename = "SURFACELIQUID-E-007")]
    E007,
    #[serde(rename = "SURFACELIQUID-E-008")]
    E008,
    #[serde(rename = "SURFACELIQUID-E-009")]
    E009,
    #[serde(rename = "SURFACELIQUID-E-010")]
    E010,
    #[serde(rename = "SURFACELIQUID-E-011")]
    E011,
}

impl DirectSurfaceLiquidErrorCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::E001 => "SURFACELIQUID-E-001",
            Self::E002 => "SURFACELIQUID-E-002",
            Self::E003 => "SURFACELIQUID-E-003",
            Self::E004 => "SURFACELIQUID-E-004",
            Self::E005 => "SURFACELIQUID-E-005",
            Self::E006 => "SURFACELIQUID-E-006",
            Self::E007 => "SURFACELIQUID-E-007",
            Self::E008 => "SURFACELIQUID-E-008",
            Self::E009 => "SURFACELIQUID-E-009",
            Self::E010 => "SURFACELIQUID-E-010",
            Self::E011 => "SURFACELIQUID-E-011",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectSurfaceLiquidPhase {
    Configuration,
    Restart,
    Authorization,
    ResourceCandidate,
    IngressCandidate,
    IndependentClosure,
    AtomicEnvelope,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidErrorContext {
    pub transaction_id: Option<TransactionId>,
    pub owner_id: Option<ResourceOwnerId>,
    pub ofe_id: Option<OfeId>,
    pub tile_id: Option<TileId>,
    pub surface_id: Option<SurfaceId>,
    pub source_id: Option<SourceId>,
    pub parcel_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidRollbackHashes {
    pub beginning_owner_sha256: Option<String>,
    pub attempted_owner_sha256: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidFailure {
    pub code: DirectSurfaceLiquidErrorCode,
    pub phase: DirectSurfaceLiquidPhase,
    pub context: DirectSurfaceLiquidErrorContext,
    pub rollback: DirectSurfaceLiquidRollbackHashes,
    pub detail: String,
}

impl std::fmt::Display for DirectSurfaceLiquidFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} {:?}: {}",
            self.code.as_str(),
            self.phase,
            self.detail
        )
    }
}

impl DirectSurfaceLiquidError {
    #[must_use]
    pub fn canonical_failure(
        code: DirectSurfaceLiquidErrorCode,
        phase: DirectSurfaceLiquidPhase,
        context: DirectSurfaceLiquidErrorContext,
        rollback: DirectSurfaceLiquidRollbackHashes,
        detail: impl Into<String>,
    ) -> Self {
        Self::Failure(Box::new(DirectSurfaceLiquidFailure {
            code,
            phase,
            context,
            rollback,
            detail: detail.into(),
        }))
    }

    /// Construct the canonical unsupported snow-free-domain failure.
    #[must_use]
    pub fn unsupported_domain_failure(
        phase: DirectSurfaceLiquidPhase,
        context: DirectSurfaceLiquidErrorContext,
        beginning_owner_sha256: Option<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::canonical_failure(
            DirectSurfaceLiquidErrorCode::E004,
            phase,
            context,
            DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256,
                attempted_owner_sha256: None,
            },
            detail,
        )
    }

    /// Construct the canonical duplicate-custody failure.
    #[must_use]
    pub fn exact_one_owner_failure(
        phase: DirectSurfaceLiquidPhase,
        context: DirectSurfaceLiquidErrorContext,
        beginning_owner_sha256: Option<String>,
        attempted_owner_sha256: Option<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::canonical_failure(
            DirectSurfaceLiquidErrorCode::E007,
            phase,
            context,
            DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256,
                attempted_owner_sha256,
            },
            detail,
        )
    }

    /// Construct the canonical complete-owner/rollback-envelope failure.
    #[must_use]
    pub fn atomic_envelope_failure(
        context: DirectSurfaceLiquidErrorContext,
        beginning_owner_sha256: Option<String>,
        attempted_owner_sha256: Option<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::canonical_failure(
            DirectSurfaceLiquidErrorCode::E011,
            DirectSurfaceLiquidPhase::AtomicEnvelope,
            context,
            DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256,
                attempted_owner_sha256,
            },
            detail,
        )
    }

    #[must_use]
    pub fn failure(&self) -> Option<&DirectSurfaceLiquidFailure> {
        match self {
            Self::Failure(failure) => Some(failure),
            _ => None,
        }
    }

    #[must_use]
    pub const fn code(&self) -> DirectSurfaceLiquidErrorCode {
        match self {
            Self::Schema(_) => DirectSurfaceLiquidErrorCode::E001,
            Self::Identity(_) => DirectSurfaceLiquidErrorCode::E002,
            Self::Domain(_) => DirectSurfaceLiquidErrorCode::E003,
            Self::Protocol(_) => DirectSurfaceLiquidErrorCode::E005,
            Self::Bound(_) => DirectSurfaceLiquidErrorCode::E006,
            Self::Closure(_) => DirectSurfaceLiquidErrorCode::E010,
            Self::Failure(failure) => failure.code,
        }
    }

    fn recontextualize(
        self,
        code: DirectSurfaceLiquidErrorCode,
        phase: DirectSurfaceLiquidPhase,
        context: DirectSurfaceLiquidErrorContext,
        beginning_owner_sha256: Option<String>,
        attempted_owner_sha256: Option<String>,
    ) -> Self {
        let detail = match self {
            Self::Failure(failure) => failure.detail,
            other => other.to_string(),
        };
        Self::Failure(Box::new(DirectSurfaceLiquidFailure {
            code,
            phase,
            context,
            rollback: DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256,
                attempted_owner_sha256,
            },
            detail,
        }))
    }

    pub(super) fn complete_context(
        self,
        code: DirectSurfaceLiquidErrorCode,
        phase: DirectSurfaceLiquidPhase,
        fallback_context: DirectSurfaceLiquidErrorContext,
        beginning_owner_sha256: Option<String>,
        attempted_owner_sha256: Option<String>,
    ) -> Self {
        match self {
            Self::Failure(mut failure) => {
                if failure.context.transaction_id.is_none() {
                    failure.context.transaction_id = fallback_context.transaction_id;
                }
                if failure.context.owner_id.is_none() {
                    failure.context.owner_id = fallback_context.owner_id;
                }
                if failure.context.ofe_id.is_none() {
                    failure.context.ofe_id = fallback_context.ofe_id;
                }
                if failure.context.tile_id.is_none() {
                    failure.context.tile_id = fallback_context.tile_id;
                }
                if failure.context.surface_id.is_none() {
                    failure.context.surface_id = fallback_context.surface_id;
                }
                if failure.context.source_id.is_none() {
                    failure.context.source_id = fallback_context.source_id;
                }
                if failure.context.parcel_id.is_none() {
                    failure.context.parcel_id = fallback_context.parcel_id;
                }
                if failure.rollback.beginning_owner_sha256.is_none() {
                    failure.rollback.beginning_owner_sha256 = beginning_owner_sha256;
                }
                if failure.rollback.attempted_owner_sha256.is_none() {
                    failure.rollback.attempted_owner_sha256 = attempted_owner_sha256;
                }
                Self::Failure(failure)
            }
            other => other.recontextualize(
                code,
                phase,
                fallback_context,
                beginning_owner_sha256,
                attempted_owner_sha256,
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectGroundIngressMode {
    OpenRawPrecipitation,
    CoveredCanopyRelease,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidStoreKey {
    pub run_id: u64,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub surface_id: SurfaceId,
    pub surface_class: SurfaceClass,
    pub source_type: WaterSourceType,
    pub source_id: SourceId,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidConfigurationRecord {
    pub key: DirectSurfaceLiquidStoreKey,
    pub tile_fraction: f64,
    pub capacity_kg_m2_tile: f64,
    pub ofe_area_m2: f64,
    pub ground_ingress_mode: DirectGroundIngressMode,
    pub runon_destination_ofe_id: Option<OfeId>,
    pub runon_destination_tile_id: Option<TileId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidOfeBinding {
    pub ofe_id: OfeId,
    pub production_lane_index: usize,
    pub production_lane_id: u32,
    pub ordered_soil_layer_ids: Vec<SoilLayerId>,
    pub infiltration_soil_thermal_layer_id: SoilLayerId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSurfaceLiquidConfiguration {
    pub owner_id: ResourceOwnerId,
    pub run_id: u64,
    pub configuration_sha256: String,
    pub ofe_topology: Vec<OfeId>,
    pub ofe_bindings: Vec<DirectSurfaceLiquidOfeBinding>,
    pub records: Vec<DirectSurfaceLiquidConfigurationRecord>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidConfigurationRecord {
    key: DirectSurfaceLiquidStoreKey,
    tile_fraction: String,
    capacity_kg_m2_tile: String,
    ofe_area_m2: String,
    ground_ingress_mode: DirectGroundIngressMode,
    runon_destination_ofe_id: Option<OfeId>,
    runon_destination_tile_id: Option<TileId>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidConfiguration {
    owner_id: ResourceOwnerId,
    run_id: u64,
    configuration_sha256: String,
    ofe_topology: Vec<OfeId>,
    ofe_bindings: Vec<DirectSurfaceLiquidOfeBinding>,
    records: Vec<CanonicalSurfaceLiquidConfigurationRecord>,
}

impl DirectSurfaceLiquidConfiguration {
    pub fn new(
        owner_id: ResourceOwnerId,
        run_id: u64,
        ofe_topology: Vec<OfeId>,
        ofe_bindings: Vec<DirectSurfaceLiquidOfeBinding>,
        records: Vec<DirectSurfaceLiquidConfigurationRecord>,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let fallback_owner_id = owner_id.clone();
        Self::new_inner(owner_id, run_id, ofe_topology, ofe_bindings, records).map_err(|error| {
            let code = error.code();
            error.complete_context(
                code,
                DirectSurfaceLiquidPhase::Configuration,
                DirectSurfaceLiquidErrorContext {
                    owner_id: Some(fallback_owner_id),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                None,
                None,
            )
        })
    }

    fn new_inner(
        owner_id: ResourceOwnerId,
        run_id: u64,
        ofe_topology: Vec<OfeId>,
        ofe_bindings: Vec<DirectSurfaceLiquidOfeBinding>,
        mut records: Vec<DirectSurfaceLiquidConfigurationRecord>,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let topology_rank = ofe_topology
            .iter()
            .enumerate()
            .map(|(index, ofe_id)| (ofe_id.clone(), index))
            .collect::<BTreeMap<_, _>>();
        records.sort_by(|left, right| {
            topology_rank
                .get(&left.key.ofe_id)
                .cmp(&topology_rank.get(&right.key.ofe_id))
                .then_with(|| left.key.cmp(&right.key))
        });
        let mut configuration = Self {
            owner_id,
            run_id,
            configuration_sha256: ZERO_SHA256.into(),
            ofe_topology,
            ofe_bindings,
            records,
        };
        configuration.preflight_schema_and_identity_structure()?;
        configuration.validate_domains()?;
        configuration.configuration_sha256 = configuration.recomputed_sha256()?;
        Ok(configuration)
    }

    pub fn validate(&self) -> Result<(), DirectSurfaceLiquidError> {
        self.validate_inner().map_err(|error| {
            let code = error.code();
            error.complete_context(
                code,
                DirectSurfaceLiquidPhase::Configuration,
                DirectSurfaceLiquidErrorContext {
                    owner_id: Some(self.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                None,
                None,
            )
        })
    }

    fn validate_inner(&self) -> Result<(), DirectSurfaceLiquidError> {
        self.preflight_schema_and_identities()?;
        self.validate_domains()
    }

    fn validate_domains(&self) -> Result<(), DirectSurfaceLiquidError> {
        let mut fraction_by_ofe = BTreeMap::<OfeId, f64>::new();
        for record in &self.records {
            let context = surface_liquid_store_context(&self.owner_id, None, &record.key);
            require_positive(record.tile_fraction, "tile fraction")
                .map_err(|error| configuration_record_failure(error, context.clone()))?;
            if record.tile_fraction > 1.0 {
                return Err(configuration_record_failure(
                    DirectSurfaceLiquidError::Domain("tile fraction above one"),
                    context,
                ));
            }
            require_positive(record.capacity_kg_m2_tile, "store capacity")
                .map_err(|error| configuration_record_failure(error, context.clone()))?;
            require_positive(record.ofe_area_m2, "OFE area")
                .map_err(|error| configuration_record_failure(error, context.clone()))?;
            let fraction = fraction_by_ofe
                .entry(record.key.ofe_id.clone())
                .or_default();
            *fraction =
                checked_surface_liquid_add(*fraction, record.tile_fraction).ok_or_else(|| {
                    configuration_record_failure(
                        DirectSurfaceLiquidError::Domain("tile fraction accumulation is nonfinite"),
                        context.clone(),
                    )
                })?;
        }
        self.validate_fraction_domains(&fraction_by_ofe)
    }

    fn validate_ofe_bindings(&self) -> Result<(), DirectSurfaceLiquidError> {
        if self.ofe_bindings.len() != self.ofe_topology.len() {
            let ofe_id = self
                .ofe_bindings
                .get(self.ofe_topology.len())
                .map(|binding| binding.ofe_id.clone())
                .or_else(|| self.ofe_topology.get(self.ofe_bindings.len()).cloned());
            return Err(configuration_record_failure(
                DirectSurfaceLiquidError::Identity("OFE binding cardinality mismatch"),
                DirectSurfaceLiquidErrorContext {
                    owner_id: Some(self.owner_id.clone()),
                    ofe_id,
                    ..DirectSurfaceLiquidErrorContext::default()
                },
            ));
        }
        let mut lane_ids = BTreeSet::new();
        for (rank, (ofe_id, binding)) in
            self.ofe_topology.iter().zip(&self.ofe_bindings).enumerate()
        {
            let context = DirectSurfaceLiquidErrorContext {
                owner_id: Some(self.owner_id.clone()),
                ofe_id: Some(binding.ofe_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            };
            if binding.ofe_id != *ofe_id
                || binding.production_lane_index != rank
                || binding.production_lane_id == 0
                || !lane_ids.insert(binding.production_lane_id)
            {
                return Err(configuration_record_failure(
                    DirectSurfaceLiquidError::Identity(
                        "wrong-order, duplicate, or wrong-lane OFE binding",
                    ),
                    context,
                ));
            }
            let unique_layers = binding
                .ordered_soil_layer_ids
                .iter()
                .collect::<BTreeSet<_>>();
            if binding.ordered_soil_layer_ids.is_empty()
                || unique_layers.len() != binding.ordered_soil_layer_ids.len()
                || binding.ordered_soil_layer_ids.first()
                    != Some(&binding.infiltration_soil_thermal_layer_id)
            {
                return Err(configuration_record_failure(
                    DirectSurfaceLiquidError::Identity(
                        "invalid ordered soil-layer or thermal-recipient binding",
                    ),
                    context,
                ));
            }
        }
        Ok(())
    }

    fn validate_canonical_record_order(&self) -> Result<(), DirectSurfaceLiquidError> {
        let topology_rank = self
            .ofe_topology
            .iter()
            .enumerate()
            .map(|(index, ofe_id)| (ofe_id.clone(), index))
            .collect::<BTreeMap<_, _>>();
        let mut canonical_keys = self
            .records
            .iter()
            .map(|record| record.key.clone())
            .collect::<Vec<_>>();
        canonical_keys.sort_by(|left, right| {
            topology_rank
                .get(&left.ofe_id)
                .cmp(&topology_rank.get(&right.ofe_id))
                .then_with(|| left.cmp(right))
        });
        if let Some(record) = self
            .records
            .iter()
            .zip(&canonical_keys)
            .find_map(|(record, expected)| (record.key != *expected).then_some(record))
        {
            return Err(configuration_record_failure(
                DirectSurfaceLiquidError::Identity("noncanonical record order"),
                surface_liquid_store_context(&self.owner_id, None, &record.key),
            ));
        }
        Ok(())
    }

    fn validate_fraction_domains(
        &self,
        fraction_by_ofe: &BTreeMap<OfeId, f64>,
    ) -> Result<(), DirectSurfaceLiquidError> {
        for (ofe_id, sum) in fraction_by_ofe {
            let tolerance = TOPOLOGY_MULTIPLIER * f64::EPSILON * sum.abs().max(1.0);
            if (*sum - 1.0).abs() > tolerance {
                return Err(configuration_record_failure(
                    DirectSurfaceLiquidError::Domain("tile fractions do not close"),
                    DirectSurfaceLiquidErrorContext {
                        owner_id: Some(self.owner_id.clone()),
                        ofe_id: Some(ofe_id.clone()),
                        ..DirectSurfaceLiquidErrorContext::default()
                    },
                ));
            }
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        self.validate()?;
        self.canonical_bytes_with_digest(&self.configuration_sha256)
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, DirectSurfaceLiquidError> {
        let attempted_owner_sha256 =
            super::surface_liquid_attachment::surface_liquid_raw_bytes_sha256(
                "openwepp-surface-liquid-configuration-parse-v1",
                bytes,
            );
        Self::from_canonical_bytes_inner(bytes).map_err(|error| {
            super::surface_liquid_attachment::surface_liquid_attachment_error(
                error,
                DirectSurfaceLiquidPhase::Configuration,
                DirectSurfaceLiquidErrorContext::default(),
                None,
                Some(attempted_owner_sha256),
            )
        })
    }

    fn from_canonical_bytes_inner(bytes: &[u8]) -> Result<Self, DirectSurfaceLiquidError> {
        let canonical: CanonicalSurfaceLiquidConfiguration = serde_json::from_slice(bytes)
            .map_err(|_| DirectSurfaceLiquidError::Schema("canonical configuration parse"))?;
        let parsed_owner_id = canonical.owner_id.clone();
        let records = canonical
            .records
            .into_iter()
            .map(|record| {
                let context = surface_liquid_store_context(&parsed_owner_id, None, &record.key);
                Ok(DirectSurfaceLiquidConfigurationRecord {
                    key: record.key,
                    tile_fraction: parse_f64_bits(&record.tile_fraction)
                        .map_err(|error| configuration_record_failure(error, context.clone()))?,
                    capacity_kg_m2_tile: parse_f64_bits(&record.capacity_kg_m2_tile)
                        .map_err(|error| configuration_record_failure(error, context.clone()))?,
                    ofe_area_m2: parse_f64_bits(&record.ofe_area_m2)
                        .map_err(|error| configuration_record_failure(error, context))?,
                    ground_ingress_mode: record.ground_ingress_mode,
                    runon_destination_ofe_id: record.runon_destination_ofe_id,
                    runon_destination_tile_id: record.runon_destination_tile_id,
                })
            })
            .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
        let configuration = Self {
            owner_id: canonical.owner_id,
            run_id: canonical.run_id,
            configuration_sha256: canonical.configuration_sha256,
            ofe_topology: canonical.ofe_topology,
            ofe_bindings: canonical.ofe_bindings,
            records,
        };
        configuration.validate()?;
        if configuration.canonical_bytes()? != bytes {
            return Err(configuration_record_failure(
                DirectSurfaceLiquidError::Schema("noncanonical configuration bytes"),
                DirectSurfaceLiquidErrorContext {
                    owner_id: Some(configuration.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
            ));
        }
        Ok(configuration)
    }

    pub(super) fn recomputed_sha256(&self) -> Result<String, DirectSurfaceLiquidError> {
        Ok(format!(
            "{:x}",
            Sha256::digest(self.canonical_bytes_with_digest(ZERO_SHA256)?)
        ))
    }

    fn canonical_bytes_with_digest(
        &self,
        digest: &str,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        let records = self
            .records
            .iter()
            .map(|record| CanonicalSurfaceLiquidConfigurationRecord {
                key: record.key.clone(),
                tile_fraction: f64_bits(record.tile_fraction),
                capacity_kg_m2_tile: f64_bits(record.capacity_kg_m2_tile),
                ofe_area_m2: f64_bits(record.ofe_area_m2),
                ground_ingress_mode: record.ground_ingress_mode,
                runon_destination_ofe_id: record.runon_destination_ofe_id.clone(),
                runon_destination_tile_id: record.runon_destination_tile_id.clone(),
            })
            .collect();
        serde_json::to_vec(&CanonicalSurfaceLiquidConfiguration {
            owner_id: self.owner_id.clone(),
            run_id: self.run_id,
            configuration_sha256: digest.into(),
            ofe_topology: self.ofe_topology.clone(),
            ofe_bindings: self.ofe_bindings.clone(),
            records,
        })
        .map_err(|_| DirectSurfaceLiquidError::Schema("canonical configuration serialization"))
    }

    fn record_for_key(
        &self,
        key: &DirectSurfaceLiquidStoreKey,
    ) -> Option<&DirectSurfaceLiquidConfigurationRecord> {
        self.records.iter().find(|record| &record.key == key)
    }

    fn store_key_for_water(
        &self,
        key: &GroundWaterKey,
    ) -> Result<DirectSurfaceLiquidStoreKey, DirectSurfaceLiquidError> {
        if key.requesting_component != RequestingComponent::GroundSurface
            || key.occupancy_id.is_some()
            || key.soil_layer_id.is_some()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "not a ground surface-liquid request",
            ));
        }
        let surface_id = key
            .surface_id
            .clone()
            .ok_or(DirectSurfaceLiquidError::Identity("missing surface id"))?;
        let surface_class = key
            .surface_class
            .ok_or(DirectSurfaceLiquidError::Identity("missing surface class"))?;
        let tile_id = key
            .source_tile_id
            .clone()
            .ok_or(DirectSurfaceLiquidError::Identity("missing source tile"))?;
        if tile_id != key.requesting_tile_id {
            return Err(DirectSurfaceLiquidError::Identity(
                "requesting/source tile mismatch",
            ));
        }
        let store = DirectSurfaceLiquidStoreKey {
            run_id: self.run_id,
            ofe_id: key.ofe_id.clone(),
            tile_id,
            surface_id,
            surface_class,
            source_type: key.source_type,
            source_id: key.source_id.clone(),
        };
        if self.record_for_key(&store).is_none() {
            return Err(DirectSurfaceLiquidError::Identity(
                "water key has no exact configured store",
            ));
        }
        Ok(store)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidStateRecord {
    pub key: DirectSurfaceLiquidStoreKey,
    pub liquid_kg_m2_tile: f64,
    pub last_accepted_transaction_id: Option<TransactionId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidContinuationState {
    pub ofe_id: OfeId,
    pub day_index: usize,
    pub next_interval_index: u8,
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
    pub last_accepted_transaction_id: Option<TransactionId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSurfaceLiquidOwnedState {
    pub owner_id: ResourceOwnerId,
    pub configuration_sha256: String,
    pub state_sha256: String,
    pub records: Vec<DirectSurfaceLiquidStateRecord>,
    pub continuations: Vec<DirectSurfaceLiquidContinuationState>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidStateRecord {
    key: DirectSurfaceLiquidStoreKey,
    liquid_kg_m2_tile: String,
    last_accepted_transaction_id: Option<TransactionId>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidContinuationState {
    ofe_id: OfeId,
    day_index: usize,
    next_interval_index: u8,
    cumulative_supply_m: String,
    cumulative_infiltration_m: String,
    last_accepted_transaction_id: Option<TransactionId>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalSurfaceLiquidOwnedState {
    owner_id: ResourceOwnerId,
    configuration_sha256: String,
    state_sha256: String,
    records: Vec<CanonicalSurfaceLiquidStateRecord>,
    continuations: Vec<CanonicalSurfaceLiquidContinuationState>,
}

impl DirectSurfaceLiquidOwnedState {
    pub fn new_initial(
        configuration: &DirectSurfaceLiquidConfiguration,
        liquid_by_key: &BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
        day_index: usize,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        Self::new_initial_inner(configuration, liquid_by_key, day_index).map_err(|error| {
            let code = error.code();
            error.complete_context(
                code,
                DirectSurfaceLiquidPhase::Restart,
                DirectSurfaceLiquidErrorContext {
                    owner_id: Some(configuration.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                None,
                None,
            )
        })
    }

    fn new_initial_inner(
        configuration: &DirectSurfaceLiquidConfiguration,
        liquid_by_key: &BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
        day_index: usize,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        configuration.validate()?;
        if liquid_by_key.len() != configuration.records.len() {
            let key = liquid_by_key
                .keys()
                .find(|key| configuration.record_for_key(key).is_none())
                .or_else(|| {
                    configuration
                        .records
                        .iter()
                        .map(|record| &record.key)
                        .find(|key| !liquid_by_key.contains_key(*key))
                });
            let context = key.map_or_else(
                || DirectSurfaceLiquidErrorContext {
                    owner_id: Some(configuration.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                |key| surface_liquid_store_context(&configuration.owner_id, None, key),
            );
            return Err(restart_record_failure(
                DirectSurfaceLiquidError::Identity("initial liquid key set mismatch"),
                context,
            ));
        }
        let records = configuration
            .records
            .iter()
            .map(|record| {
                let liquid = *liquid_by_key.get(&record.key).ok_or_else(|| {
                    restart_record_failure(
                        DirectSurfaceLiquidError::Identity("missing initial liquid key"),
                        surface_liquid_store_context(&configuration.owner_id, None, &record.key),
                    )
                })?;
                Ok(DirectSurfaceLiquidStateRecord {
                    key: record.key.clone(),
                    liquid_kg_m2_tile: liquid,
                    last_accepted_transaction_id: None,
                })
            })
            .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
        let continuations = configured_ofes(configuration)
            .into_iter()
            .map(|ofe_id| DirectSurfaceLiquidContinuationState {
                ofe_id,
                day_index,
                next_interval_index: 0,
                cumulative_supply_m: 0.0,
                cumulative_infiltration_m: 0.0,
                last_accepted_transaction_id: None,
            })
            .collect();
        let mut state = Self {
            owner_id: configuration.owner_id.clone(),
            configuration_sha256: configuration.configuration_sha256.clone(),
            state_sha256: ZERO_SHA256.into(),
            records,
            continuations,
        };
        let expected_lineage = state.preflight_schema_and_identity_structure(configuration)?;
        state.validate_domains(configuration, expected_lineage)?;
        state.state_sha256 = state.recomputed_sha256()?;
        Ok(state)
    }

    pub fn validate(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<(), DirectSurfaceLiquidError> {
        let beginning_owner_sha256 =
            Some(super::surface_liquid_attachment::surface_liquid_raw_state_sha256(self));
        self.validate_inner(configuration).map_err(|error| {
            super::surface_liquid_attachment::surface_liquid_attachment_error(
                error,
                DirectSurfaceLiquidPhase::Restart,
                DirectSurfaceLiquidErrorContext {
                    owner_id: Some(self.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                beginning_owner_sha256,
                None,
            )
        })
    }

    fn validate_inner(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<(), DirectSurfaceLiquidError> {
        configuration.preflight_schema_and_identities()?;
        let expected_lineage = self.preflight_schema_and_identities(configuration)?;
        configuration.validate_domains()?;
        self.validate_domains(configuration, expected_lineage)
    }

    fn validate_for_transaction(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
        transaction_id: TransactionId,
        expected_predecessor: Option<TransactionId>,
    ) -> Result<(), DirectSurfaceLiquidError> {
        self.validate(configuration)?;
        if transaction_id.0 == 0 || Some(transaction_id) == expected_predecessor {
            return Err(DirectSurfaceLiquidError::Identity(
                "invalid candidate transaction",
            ));
        }
        if self.accepted_transaction()? != expected_predecessor {
            return Err(DirectSurfaceLiquidError::Identity(
                "predecessor transaction mismatch",
            ));
        }
        Ok(())
    }

    fn accepted_transaction(&self) -> Result<Option<TransactionId>, DirectSurfaceLiquidError> {
        let mut observed = None;
        for record in &self.records {
            let lineage = record.last_accepted_transaction_id;
            match observed {
                None => observed = Some(lineage),
                Some(expected) if expected == lineage => {}
                Some(_) => {
                    return Err(restart_record_failure(
                        DirectSurfaceLiquidError::Identity("mixed accepted transaction lineage"),
                        surface_liquid_store_context(&self.owner_id, lineage, &record.key),
                    ));
                }
            }
        }
        for continuation in &self.continuations {
            let lineage = continuation.last_accepted_transaction_id;
            match observed {
                None => observed = Some(lineage),
                Some(expected) if expected == lineage => {}
                Some(_) => {
                    return Err(restart_record_failure(
                        DirectSurfaceLiquidError::Identity("mixed accepted transaction lineage"),
                        DirectSurfaceLiquidErrorContext {
                            transaction_id: lineage,
                            owner_id: Some(self.owner_id.clone()),
                            ofe_id: Some(continuation.ofe_id.clone()),
                            ..DirectSurfaceLiquidErrorContext::default()
                        },
                    ));
                }
            }
        }
        observed.ok_or(DirectSurfaceLiquidError::Schema("empty state lineage"))
    }

    fn validate_domains(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
        expected_lineage: Option<TransactionId>,
    ) -> Result<(), DirectSurfaceLiquidError> {
        for (state, config) in self.records.iter().zip(&configuration.records) {
            validate_restart_store_domains(&self.owner_id, state, config, expected_lineage)?;
        }
        validate_restart_continuation_domains(&self.owner_id, &self.continuations, expected_lineage)
    }

    pub fn canonical_bytes(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        self.validate(configuration)?;
        self.canonical_bytes_with_digest(&self.state_sha256)
    }

    pub fn from_canonical_bytes(
        configuration: &DirectSurfaceLiquidConfiguration,
        bytes: &[u8],
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let attempted_owner_sha256 =
            super::surface_liquid_attachment::surface_liquid_raw_bytes_sha256(
                "openwepp-surface-liquid-state-parse-v1",
                bytes,
            );
        Self::from_canonical_bytes_inner(configuration, bytes).map_err(|error| {
            super::surface_liquid_attachment::surface_liquid_attachment_error(
                error,
                DirectSurfaceLiquidPhase::Restart,
                DirectSurfaceLiquidErrorContext {
                    owner_id: Some(configuration.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                None,
                Some(attempted_owner_sha256),
            )
        })
    }

    fn from_canonical_bytes_inner(
        configuration: &DirectSurfaceLiquidConfiguration,
        bytes: &[u8],
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let canonical: CanonicalSurfaceLiquidOwnedState = serde_json::from_slice(bytes)
            .map_err(|_| DirectSurfaceLiquidError::Schema("canonical state parse"))?;
        let parsed_owner_id = canonical.owner_id.clone();
        let parsed_state_sha256 = canonical.state_sha256.clone();
        let records = canonical
            .records
            .into_iter()
            .map(|record| {
                let context = surface_liquid_store_context(
                    &parsed_owner_id,
                    record.last_accepted_transaction_id,
                    &record.key,
                );
                Ok(DirectSurfaceLiquidStateRecord {
                    key: record.key,
                    liquid_kg_m2_tile: parse_f64_bits(&record.liquid_kg_m2_tile).map_err(
                        |error| {
                            restart_record_failure_with_hash(
                                error,
                                context,
                                Some(parsed_state_sha256.clone()),
                            )
                        },
                    )?,
                    last_accepted_transaction_id: record.last_accepted_transaction_id,
                })
            })
            .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
        let continuations = canonical
            .continuations
            .into_iter()
            .map(|continuation| {
                let context = DirectSurfaceLiquidErrorContext {
                    transaction_id: continuation.last_accepted_transaction_id,
                    owner_id: Some(parsed_owner_id.clone()),
                    ofe_id: Some(continuation.ofe_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                };
                Ok(DirectSurfaceLiquidContinuationState {
                    ofe_id: continuation.ofe_id,
                    day_index: continuation.day_index,
                    next_interval_index: continuation.next_interval_index,
                    cumulative_supply_m: parse_f64_bits(&continuation.cumulative_supply_m)
                        .map_err(|error| {
                            restart_record_failure_with_hash(
                                error,
                                context.clone(),
                                Some(parsed_state_sha256.clone()),
                            )
                        })?,
                    cumulative_infiltration_m: parse_f64_bits(
                        &continuation.cumulative_infiltration_m,
                    )
                    .map_err(|error| {
                        restart_record_failure_with_hash(
                            error,
                            context,
                            Some(parsed_state_sha256.clone()),
                        )
                    })?,
                    last_accepted_transaction_id: continuation.last_accepted_transaction_id,
                })
            })
            .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
        let state = Self {
            owner_id: canonical.owner_id,
            configuration_sha256: canonical.configuration_sha256,
            state_sha256: canonical.state_sha256,
            records,
            continuations,
        };
        state.validate(configuration)?;
        if state.canonical_bytes(configuration)? != bytes {
            return Err(restart_record_failure_with_hash(
                DirectSurfaceLiquidError::Schema("noncanonical state bytes"),
                DirectSurfaceLiquidErrorContext {
                    transaction_id: state.accepted_transaction()?,
                    owner_id: Some(state.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                Some(state.state_sha256.clone()),
            ));
        }
        Ok(state)
    }

    pub(crate) fn recomputed_sha256(&self) -> Result<String, DirectSurfaceLiquidError> {
        Ok(format!(
            "{:x}",
            Sha256::digest(self.canonical_bytes_with_digest(ZERO_SHA256)?)
        ))
    }

    fn canonical_bytes_with_digest(
        &self,
        digest: &str,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        let records = self
            .records
            .iter()
            .map(|record| CanonicalSurfaceLiquidStateRecord {
                key: record.key.clone(),
                liquid_kg_m2_tile: f64_bits(record.liquid_kg_m2_tile),
                last_accepted_transaction_id: record.last_accepted_transaction_id,
            })
            .collect();
        let continuations = self
            .continuations
            .iter()
            .map(|state| CanonicalSurfaceLiquidContinuationState {
                ofe_id: state.ofe_id.clone(),
                day_index: state.day_index,
                next_interval_index: state.next_interval_index,
                cumulative_supply_m: f64_bits(state.cumulative_supply_m),
                cumulative_infiltration_m: f64_bits(state.cumulative_infiltration_m),
                last_accepted_transaction_id: state.last_accepted_transaction_id,
            })
            .collect();
        serde_json::to_vec(&CanonicalSurfaceLiquidOwnedState {
            owner_id: self.owner_id.clone(),
            configuration_sha256: self.configuration_sha256.clone(),
            state_sha256: digest.into(),
            records,
            continuations,
        })
        .map_err(|_| DirectSurfaceLiquidError::Schema("canonical state serialization"))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSurfaceLiquidArbitration {
    transaction_id: TransactionId,
    expected_predecessor: Option<TransactionId>,
    beginning_state: DirectSurfaceLiquidOwnedState,
    requests: Vec<WaterAmount>,
    authorizations: Vec<WaterAuthorization>,
    request_store_keys: Vec<DirectSurfaceLiquidStoreKey>,
}

impl DirectSurfaceLiquidArbitration {
    #[must_use]
    pub const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    pub(crate) const fn expected_predecessor(&self) -> Option<TransactionId> {
        self.expected_predecessor
    }

    #[must_use]
    pub const fn beginning_state(&self) -> &DirectSurfaceLiquidOwnedState {
        &self.beginning_state
    }

    #[must_use]
    pub fn requests(&self) -> &[WaterAmount] {
        &self.requests
    }

    #[must_use]
    pub fn authorizations(&self) -> &[WaterAuthorization] {
        &self.authorizations
    }

    pub(crate) fn request_store_keys(&self) -> &[DirectSurfaceLiquidStoreKey] {
        &self.request_store_keys
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectCondensationOverflow {
    pub store_key: DirectSurfaceLiquidStoreKey,
    pub amount_kg_m2_ofe_ground: f64,
    pub temperature_k: f64,
    pub specific_liquid_enthalpy_j_kg: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSurfaceLiquidResourceCandidate {
    transaction_id: TransactionId,
    beginning_state: DirectSurfaceLiquidOwnedState,
    working_state: DirectSurfaceLiquidOwnedState,
    finalized_uses: Vec<WaterAmount>,
    condensation_credits: Vec<CondensationCredit>,
    condensation_overflow: Vec<DirectCondensationOverflow>,
    requests: Vec<WaterAmount>,
    authorizations: Vec<WaterAuthorization>,
    request_store_keys: Vec<DirectSurfaceLiquidStoreKey>,
    expected_predecessor: Option<TransactionId>,
}

impl DirectSurfaceLiquidResourceCandidate {
    #[must_use]
    pub const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub const fn expected_predecessor(&self) -> Option<TransactionId> {
        self.expected_predecessor
    }

    #[must_use]
    pub const fn beginning_state(&self) -> &DirectSurfaceLiquidOwnedState {
        &self.beginning_state
    }

    #[must_use]
    pub const fn working_state(&self) -> &DirectSurfaceLiquidOwnedState {
        &self.working_state
    }

    #[must_use]
    pub fn finalized_uses(&self) -> &[WaterAmount] {
        &self.finalized_uses
    }

    #[must_use]
    pub fn condensation_credits(&self) -> &[CondensationCredit] {
        &self.condensation_credits
    }

    #[must_use]
    pub fn condensation_overflow(&self) -> &[DirectCondensationOverflow] {
        &self.condensation_overflow
    }

    #[must_use]
    pub fn requests(&self) -> &[WaterAmount] {
        &self.requests
    }

    #[must_use]
    pub fn authorizations(&self) -> &[WaterAuthorization] {
        &self.authorizations
    }

    pub(crate) fn request_store_keys(&self) -> &[DirectSurfaceLiquidStoreKey] {
        &self.request_store_keys
    }

    pub fn validate(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<(), DirectSurfaceLiquidError> {
        let beginning_owner_sha256 = Some(
            super::surface_liquid_attachment::surface_liquid_raw_state_sha256(
                &self.beginning_state,
            ),
        );
        let attempted_owner_sha256 = Some(
            super::surface_liquid_attachment::surface_liquid_raw_candidate_attempt_sha256(
                configuration,
                self,
            ),
        );
        validate_resource_candidate(configuration, self).map_err(|error| {
            super::surface_liquid_attachment::surface_liquid_attachment_error(
                error,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(self.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                beginning_owner_sha256,
                attempted_owner_sha256,
            )
        })
    }
}

pub fn authorize_surface_liquid_withdrawals(
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning: &DirectSurfaceLiquidOwnedState,
    transaction_id: TransactionId,
    expected_predecessor: Option<TransactionId>,
    requests: &[WaterAmount],
) -> Result<DirectSurfaceLiquidArbitration, DirectSurfaceLiquidError> {
    let beginning_owner_sha256 =
        Some(super::surface_liquid_attachment::surface_liquid_raw_state_sha256(beginning));
    let attempted_owner_sha256 = Some(
        super::surface_liquid_attachment::surface_liquid_raw_authorization_attempt_sha256(
            configuration,
            beginning,
            transaction_id,
            expected_predecessor,
            requests,
        ),
    );
    authorize_surface_liquid_withdrawals_inner(
        configuration,
        beginning,
        transaction_id,
        expected_predecessor,
        requests,
    )
    .map_err(|error| {
        super::surface_liquid_attachment::surface_liquid_attachment_error(
            error,
            DirectSurfaceLiquidPhase::Authorization,
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(transaction_id),
                owner_id: Some(configuration.owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            beginning_owner_sha256,
            attempted_owner_sha256,
        )
    })
}

#[allow(clippy::too_many_lines)]
fn authorize_surface_liquid_withdrawals_inner(
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning: &DirectSurfaceLiquidOwnedState,
    transaction_id: TransactionId,
    expected_predecessor: Option<TransactionId>,
    requests: &[WaterAmount],
) -> Result<DirectSurfaceLiquidArbitration, DirectSurfaceLiquidError> {
    configuration.preflight_schema_and_identities()?;
    let accepted_predecessor = beginning.preflight_schema_and_identities(configuration)?;
    if transaction_id.0 == 0 || Some(transaction_id) == expected_predecessor {
        return Err(DirectSurfaceLiquidError::Identity(
            "invalid candidate transaction",
        ));
    }
    if accepted_predecessor != expected_predecessor {
        return Err(DirectSurfaceLiquidError::Identity(
            "predecessor transaction mismatch",
        ));
    }
    for request in requests {
        request.key.validate(transaction_id).map_err(|_| {
            water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E002,
                DirectSurfaceLiquidPhase::Authorization,
                transaction_id,
                &request.key,
                "invalid LSE request key",
            )
        })?;
        configuration
            .store_key_for_water(&request.key)
            .map_err(|error| {
                water_protocol_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    DirectSurfaceLiquidPhase::Authorization,
                    transaction_id,
                    &request.key,
                    error.to_string(),
                )
            })?;
    }
    beginning.validate_for_transaction(configuration, transaction_id, expected_predecessor)?;
    for request in requests {
        if !request.amount_kg_m2_stand_ground.is_finite() {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E003,
                DirectSurfaceLiquidPhase::Authorization,
                transaction_id,
                &request.key,
                "request amount is nonfinite",
            ));
        }
    }
    let mut seen = BTreeSet::new();
    for request in requests {
        if !seen.insert(request.key.clone()) {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E005,
                DirectSurfaceLiquidPhase::Authorization,
                transaction_id,
                &request.key,
                "duplicate request",
            ));
        }
    }
    let state_by_key = beginning
        .records
        .iter()
        .map(|record| (record.key.clone(), record))
        .collect::<BTreeMap<_, _>>();
    let mut request_store_keys = Vec::with_capacity(requests.len());
    for request in requests {
        if request.amount_kg_m2_stand_ground < 0.0 {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E006,
                DirectSurfaceLiquidPhase::Authorization,
                transaction_id,
                &request.key,
                "negative request amount",
            ));
        }
        let store_key = configuration
            .store_key_for_water(&request.key)
            .map_err(|error| {
                let detail = error.to_string();
                water_protocol_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    DirectSurfaceLiquidPhase::Authorization,
                    transaction_id,
                    &request.key,
                    detail,
                )
            })?;
        request_store_keys.push(store_key);
    }
    let mut authorization_amounts = vec![0.0; requests.len()];
    let mut indexes_by_store = BTreeMap::<DirectSurfaceLiquidStoreKey, Vec<usize>>::new();
    for (index, store_key) in request_store_keys.iter().enumerate() {
        indexes_by_store
            .entry(store_key.clone())
            .or_default()
            .push(index);
    }
    for (store_key, indexes) in indexes_by_store {
        let config = configuration
            .record_for_key(&store_key)
            .ok_or(DirectSurfaceLiquidError::Identity("request store vanished"))?;
        let state = state_by_key
            .get(&store_key)
            .ok_or(DirectSurfaceLiquidError::Identity("request state vanished"))?;
        let first_request = &requests[indexes[0]];
        let supply = checked_surface_liquid_mul(config.tile_fraction, state.liquid_kg_m2_tile)
            .ok_or_else(|| {
                water_protocol_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    DirectSurfaceLiquidPhase::Authorization,
                    transaction_id,
                    &first_request.key,
                    "same-store supply multiplication is nonfinite or underflowed",
                )
            })?;
        let mut canonical_indexes = indexes;
        canonical_indexes.sort_by(|left, right| requests[*left].key.cmp(&requests[*right].key));
        let total_demand = checked_surface_liquid_sum(
            canonical_indexes
                .iter()
                .map(|index| requests[*index].amount_kg_m2_stand_ground),
        )
        .ok_or_else(|| {
            water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E003,
                DirectSurfaceLiquidPhase::Authorization,
                transaction_id,
                &requests[canonical_indexes[0]].key,
                "canonical same-store demand accumulation is nonfinite",
            )
        })?;
        if total_demand <= supply {
            for index in canonical_indexes {
                authorization_amounts[index] = requests[index].amount_kg_m2_stand_ground;
            }
        } else if supply > 0.0 && total_demand > 0.0 {
            let checked_shares = canonical_indexes
                .iter()
                .map(|index| {
                    let numerator = checked_surface_liquid_mul(
                        requests[*index].amount_kg_m2_stand_ground,
                        supply,
                    )
                    .ok_or_else(|| {
                        water_protocol_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            DirectSurfaceLiquidPhase::Authorization,
                            transaction_id,
                            &requests[*index].key,
                            "proportional authorization numerator is nonfinite or underflowed",
                        )
                    })?;
                    let share =
                        checked_surface_liquid_div(numerator, total_demand).ok_or_else(|| {
                            water_protocol_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                DirectSurfaceLiquidPhase::Authorization,
                                transaction_id,
                                &requests[*index].key,
                                "proportional authorization division is nonfinite or underflowed",
                            )
                        })?;
                    Ok(share)
                })
                .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
            let corrected_shares =
                jointly_safe_proportional_authorizations(&checked_shares, supply).ok_or_else(
                    || {
                        water_protocol_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            DirectSurfaceLiquidPhase::Authorization,
                            transaction_id,
                            &requests[canonical_indexes[0]].key,
                            "joint proportional authorization is not safely representable",
                        )
                    },
                )?;
            for (index, amount) in canonical_indexes.into_iter().zip(corrected_shares) {
                if !amount.is_finite()
                    || amount < 0.0
                    || amount > requests[index].amount_kg_m2_stand_ground
                {
                    return Err(water_protocol_failure(
                        DirectSurfaceLiquidErrorCode::E009,
                        DirectSurfaceLiquidPhase::Authorization,
                        transaction_id,
                        &requests[index].key,
                        "proportional authorization formula result",
                    ));
                }
                authorization_amounts[index] = amount;
            }
        }
    }
    let authorizations = requests
        .iter()
        .zip(authorization_amounts)
        .map(|(request, amount)| {
            let reason = if request.amount_kg_m2_stand_ground == 0.0 {
                WaterAuthorizationReason::ZeroSupply
            } else if amount == 0.0 {
                WaterAuthorizationReason::DrySource
            } else if amount.to_bits() == request.amount_kg_m2_stand_ground.to_bits() {
                WaterAuthorizationReason::FullSupply
            } else {
                WaterAuthorizationReason::ProportionalSupply
            };
            WaterAuthorization {
                key: request.key.clone(),
                amount_kg_m2_stand_ground: amount,
                reason,
            }
        })
        .collect();
    Ok(DirectSurfaceLiquidArbitration {
        transaction_id,
        expected_predecessor,
        beginning_state: beginning.clone(),
        requests: requests.to_vec(),
        authorizations,
        request_store_keys,
    })
}

pub fn apply_surface_liquid_resource_phase(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    finalized_uses: &[WaterAmount],
    condensation_credits: &[CondensationCredit],
) -> Result<DirectSurfaceLiquidResourceCandidate, DirectSurfaceLiquidError> {
    let beginning_owner_sha256 = Some(
        super::surface_liquid_attachment::surface_liquid_raw_state_sha256(
            arbitration.beginning_state(),
        ),
    );
    let attempted_owner_sha256 = Some(
        super::surface_liquid_attachment::surface_liquid_raw_resource_attempt_sha256(
            configuration,
            arbitration,
            finalized_uses,
            condensation_credits,
        ),
    );
    apply_surface_liquid_resource_phase_inner(
        configuration,
        arbitration,
        finalized_uses,
        condensation_credits,
    )
    .map_err(|error| {
        super::surface_liquid_attachment::surface_liquid_attachment_error(
            error,
            DirectSurfaceLiquidPhase::ResourceCandidate,
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(arbitration.transaction_id),
                owner_id: Some(configuration.owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            beginning_owner_sha256,
            attempted_owner_sha256,
        )
    })
}

#[allow(clippy::too_many_lines)]
fn apply_surface_liquid_resource_phase_inner(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    finalized_uses: &[WaterAmount],
    condensation_credits: &[CondensationCredit],
) -> Result<DirectSurfaceLiquidResourceCandidate, DirectSurfaceLiquidError> {
    preflight_resource_phase_public_identities(
        configuration,
        arbitration,
        finalized_uses,
        condensation_credits,
    )?;
    arbitration.beginning_state.validate_for_transaction(
        configuration,
        arbitration.transaction_id,
        arbitration.expected_predecessor,
    )?;
    preflight_resource_phase_inputs(
        configuration,
        arbitration,
        finalized_uses,
        condensation_credits,
    )?;
    validate_arbitration(configuration, arbitration)?;
    let debit_by_store = validate_finalized_uses(arbitration, finalized_uses)?;
    let (credit_by_store, credit_details) =
        collect_condensation_credits(configuration, arbitration, condensation_credits)?;
    let mut working_state = arbitration.beginning_state.clone();
    let mut overflows = Vec::new();
    for (state, config) in working_state.records.iter_mut().zip(&configuration.records) {
        let debit = debit_by_store.get(&state.key).copied().unwrap_or(0.0);
        let credit = credit_by_store.get(&state.key).copied().unwrap_or(0.0);
        let debit_tile =
            checked_surface_liquid_div(debit, config.tile_fraction).ok_or_else(|| {
                store_arithmetic_failure(
                    configuration,
                    arbitration.transaction_id,
                    &state.key,
                    "F/f_t resource conversion is nonfinite or underflowed",
                )
            })?;
        let credit_tile =
            checked_surface_liquid_div(credit, config.tile_fraction).ok_or_else(|| {
                store_arithmetic_failure(
                    configuration,
                    arbitration.transaction_id,
                    &state.key,
                    "C/f_t resource conversion is nonfinite or underflowed",
                )
            })?;
        let after_debit = checked_surface_liquid_sub(state.liquid_kg_m2_tile, debit_tile)
            .ok_or_else(|| {
                store_arithmetic_failure(
                    configuration,
                    arbitration.transaction_id,
                    &state.key,
                    "W0-F/f_t resource intermediate is nonfinite",
                )
            })?;
        let raw = checked_surface_liquid_add(after_debit, credit_tile).ok_or_else(|| {
            store_arithmetic_failure(
                configuration,
                arbitration.transaction_id,
                &state.key,
                "W0-F/f_t+C/f_t resource intermediate is nonfinite",
            )
        })?;
        if raw < 0.0 {
            return Err(candidate_closure("negative resource state"));
        }
        if raw > config.capacity_kg_m2_tile {
            let detail = credit_details
                .get(&state.key)
                .ok_or_else(|| candidate_closure("overflow without condensation credit"))?;
            let excess =
                checked_surface_liquid_sub(raw, config.capacity_kg_m2_tile).ok_or_else(|| {
                    store_arithmetic_failure(
                        configuration,
                        arbitration.transaction_id,
                        &state.key,
                        "capacity overflow difference is nonfinite",
                    )
                })?;
            let amount =
                checked_surface_liquid_mul(config.tile_fraction, excess).ok_or_else(|| {
                    store_arithmetic_failure(
                        configuration,
                        arbitration.transaction_id,
                        &state.key,
                        "capacity overflow area conversion is nonfinite or underflowed",
                    )
                })?;
            overflows.push(DirectCondensationOverflow {
                store_key: state.key.clone(),
                amount_kg_m2_ofe_ground: amount,
                temperature_k: detail.temperature_k,
                specific_liquid_enthalpy_j_kg: detail.specific_liquid_enthalpy_j_kg,
            });
        }
        state.liquid_kg_m2_tile = raw.min(config.capacity_kg_m2_tile);
    }
    let candidate = DirectSurfaceLiquidResourceCandidate {
        transaction_id: arbitration.transaction_id,
        beginning_state: arbitration.beginning_state.clone(),
        working_state,
        finalized_uses: finalized_uses.to_vec(),
        condensation_credits: condensation_credits.to_vec(),
        condensation_overflow: overflows,
        requests: arbitration.requests.clone(),
        authorizations: arbitration.authorizations.clone(),
        request_store_keys: arbitration.request_store_keys.clone(),
        expected_predecessor: arbitration.expected_predecessor,
    };
    candidate.validate(configuration)?;
    Ok(candidate)
}

fn preflight_resource_phase_public_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    finalized_uses: &[WaterAmount],
    condensation_credits: &[CondensationCredit],
) -> Result<(), DirectSurfaceLiquidError> {
    configuration.preflight_schema_and_identities()?;
    let accepted_predecessor = arbitration
        .beginning_state
        .preflight_schema_and_identities(configuration)?;
    if arbitration.transaction_id.0 == 0
        || Some(arbitration.transaction_id) == arbitration.expected_predecessor
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "invalid candidate transaction",
        ));
    }
    if accepted_predecessor != arbitration.expected_predecessor {
        return Err(DirectSurfaceLiquidError::Identity(
            "predecessor transaction mismatch",
        ));
    }

    // The shared validator already walks every retained and caller-supplied
    // protocol identity before inspecting any numeric operand. Use that pass
    // only to surface E001/E002 here; the normal call below retains the
    // established E003/E005/E006 ordering after configuration and state
    // validation.
    match preflight_resource_phase_inputs(
        configuration,
        arbitration,
        finalized_uses,
        condensation_credits,
    ) {
        Err(error)
            if matches!(
                error.code(),
                DirectSurfaceLiquidErrorCode::E001 | DirectSurfaceLiquidErrorCode::E002
            ) =>
        {
            Err(error)
        }
        Ok(()) | Err(_) => Ok(()),
    }
}

#[allow(clippy::too_many_lines)]
fn validate_resource_candidate(
    configuration: &DirectSurfaceLiquidConfiguration,
    candidate: &DirectSurfaceLiquidResourceCandidate,
) -> Result<(), DirectSurfaceLiquidError> {
    let retained_arbitration = DirectSurfaceLiquidArbitration {
        transaction_id: candidate.transaction_id,
        expected_predecessor: candidate.expected_predecessor,
        beginning_state: candidate.beginning_state.clone(),
        requests: candidate.requests.clone(),
        authorizations: candidate.authorizations.clone(),
        request_store_keys: candidate.request_store_keys.clone(),
    };
    let predecessor = candidate.beginning_state.accepted_transaction()?;
    candidate.beginning_state.validate_for_transaction(
        configuration,
        candidate.transaction_id,
        predecessor,
    )?;
    if candidate.working_state.owner_id != candidate.beginning_state.owner_id
        || candidate.working_state.configuration_sha256
            != candidate.beginning_state.configuration_sha256
        || candidate.working_state.records.len() != candidate.beginning_state.records.len()
        || candidate
            .working_state
            .records
            .iter()
            .zip(&candidate.beginning_state.records)
            .zip(&configuration.records)
            .any(|((working, beginning), config)| {
                working.key != beginning.key || working.key != config.key
            })
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "resource candidate owner or record identity mismatch",
        ));
    }
    preflight_resource_phase_inputs(
        configuration,
        &retained_arbitration,
        &candidate.finalized_uses,
        &candidate.condensation_credits,
    )?;
    validate_arbitration(configuration, &retained_arbitration)?;
    validate_finalized_uses(&retained_arbitration, &candidate.finalized_uses)?;
    if candidate.working_state.state_sha256 != candidate.beginning_state.state_sha256
        || candidate.working_state.continuations != candidate.beginning_state.continuations
    {
        return Err(DirectSurfaceLiquidError::Closure(
            "resource candidate changed non-resource owner state",
        ));
    }
    let (debit_by_store, credit_by_store, credit_details) =
        reconstruct_resource_candidate_operands(configuration, candidate)?;
    let mut expected_overflow = Vec::new();
    for ((beginning, working), config) in candidate
        .beginning_state
        .records
        .iter()
        .zip(&candidate.working_state.records)
        .zip(&configuration.records)
    {
        if beginning.key != working.key || beginning.key != config.key {
            return Err(DirectSurfaceLiquidError::Identity(
                "resource candidate record key mismatch",
            ));
        }
        let debit = debit_by_store.get(&beginning.key).copied().unwrap_or(0.0);
        let credit = credit_by_store.get(&beginning.key).copied().unwrap_or(0.0);
        let debit_tile =
            checked_surface_liquid_div(debit, config.tile_fraction).ok_or_else(|| {
                store_arithmetic_failure(
                    configuration,
                    candidate.transaction_id,
                    &beginning.key,
                    "independent F/f_t conversion is nonfinite or underflowed",
                )
            })?;
        let credit_tile =
            checked_surface_liquid_div(credit, config.tile_fraction).ok_or_else(|| {
                store_arithmetic_failure(
                    configuration,
                    candidate.transaction_id,
                    &beginning.key,
                    "independent C/f_t conversion is nonfinite or underflowed",
                )
            })?;
        let after_debit = checked_surface_liquid_sub(beginning.liquid_kg_m2_tile, debit_tile)
            .ok_or_else(|| {
                store_arithmetic_failure(
                    configuration,
                    candidate.transaction_id,
                    &beginning.key,
                    "independent W0-F/f_t intermediate is nonfinite",
                )
            })?;
        let raw = checked_surface_liquid_add(after_debit, credit_tile).ok_or_else(|| {
            store_arithmetic_failure(
                configuration,
                candidate.transaction_id,
                &beginning.key,
                "independent W0-F/f_t+C/f_t intermediate is nonfinite",
            )
        })?;
        let expected_liquid = raw.min(config.capacity_kg_m2_tile);
        if !raw.is_finite()
            || raw < 0.0
            || working.liquid_kg_m2_tile.to_bits() != expected_liquid.to_bits()
            || working.last_accepted_transaction_id != beginning.last_accepted_transaction_id
        {
            return Err(DirectSurfaceLiquidError::Closure(
                "independent W0-F+C resource reconstruction mismatch",
            ));
        }
        if raw > config.capacity_kg_m2_tile {
            let detail =
                credit_details
                    .get(&beginning.key)
                    .ok_or(DirectSurfaceLiquidError::Closure(
                        "overflow without condensation credit",
                    ))?;
            let excess =
                checked_surface_liquid_sub(raw, config.capacity_kg_m2_tile).ok_or_else(|| {
                    store_arithmetic_failure(
                        configuration,
                        candidate.transaction_id,
                        &beginning.key,
                        "independent capacity overflow difference is nonfinite",
                    )
                })?;
            let amount =
                checked_surface_liquid_mul(config.tile_fraction, excess).ok_or_else(|| {
                    store_arithmetic_failure(
                        configuration,
                        candidate.transaction_id,
                        &beginning.key,
                        "independent overflow area conversion is nonfinite or underflowed",
                    )
                })?;
            expected_overflow.push(DirectCondensationOverflow {
                store_key: beginning.key.clone(),
                amount_kg_m2_ofe_ground: amount,
                temperature_k: detail.temperature_k,
                specific_liquid_enthalpy_j_kg: detail.specific_liquid_enthalpy_j_kg,
            });
        }
    }
    if !overflow_eq_bits(&expected_overflow, &candidate.condensation_overflow) {
        return Err(DirectSurfaceLiquidError::Closure(
            "independent condensation-overflow reconstruction mismatch",
        ));
    }
    Ok(())
}

fn validate_arbitration(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
) -> Result<(), DirectSurfaceLiquidError> {
    let expected = authorize_surface_liquid_withdrawals_inner(
        configuration,
        &arbitration.beginning_state,
        arbitration.transaction_id,
        arbitration.expected_predecessor,
        &arbitration.requests,
    )?;
    let authorizations_match = expected.authorizations.len() == arbitration.authorizations.len()
        && expected
            .authorizations
            .iter()
            .zip(&arbitration.authorizations)
            .all(|(left, right)| {
                left.key == right.key
                    && left.amount_kg_m2_stand_ground.to_bits()
                        == right.amount_kg_m2_stand_ground.to_bits()
                    && left.reason == right.reason
            });
    if !authorizations_match || expected.request_store_keys != arbitration.request_store_keys {
        return Err(candidate_closure(
            "authorization does not reconstruct from immutable beginning supply and demand",
        ));
    }
    Ok(())
}

type ResourceCandidateOperands<'a> = (
    BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
    BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
    BTreeMap<DirectSurfaceLiquidStoreKey, &'a CondensationCredit>,
);

fn canonical_finalized_debits(
    transaction_id: TransactionId,
    entries: Vec<(DirectSurfaceLiquidStoreKey, GroundWaterKey, f64)>,
) -> Result<BTreeMap<DirectSurfaceLiquidStoreKey, f64>, DirectSurfaceLiquidError> {
    let mut by_store =
        BTreeMap::<DirectSurfaceLiquidStoreKey, BTreeMap<GroundWaterKey, f64>>::new();
    for (store_key, water_key, amount) in entries {
        if by_store
            .entry(store_key)
            .or_default()
            .insert(water_key.clone(), amount)
            .is_some()
        {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E005,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &water_key,
                "duplicate finalized use during canonical debit aggregation",
            ));
        }
    }

    by_store
        .into_iter()
        .map(|(store_key, rows)| {
            let mut sum = 0.0;
            for (water_key, amount) in rows {
                sum = checked_surface_liquid_add(sum, amount).ok_or_else(|| {
                    water_protocol_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        DirectSurfaceLiquidPhase::ResourceCandidate,
                        transaction_id,
                        &water_key,
                        "canonical finalized-use aggregation is nonfinite",
                    )
                })?;
            }
            Ok((store_key, sum))
        })
        .collect()
}

fn reconstruct_resource_candidate_operands<'a>(
    configuration: &DirectSurfaceLiquidConfiguration,
    candidate: &'a DirectSurfaceLiquidResourceCandidate,
) -> Result<ResourceCandidateOperands<'a>, DirectSurfaceLiquidError> {
    let mut debit_entries = Vec::with_capacity(candidate.finalized_uses.len());
    let mut finalized_keys = BTreeSet::new();
    for finalized in &candidate.finalized_uses {
        finalized
            .key
            .validate(candidate.transaction_id)
            .map_err(|_| DirectSurfaceLiquidError::Identity("invalid finalized-use key"))?;
        if !finalized.amount_kg_m2_stand_ground.is_finite() {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E003,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                candidate.transaction_id,
                &finalized.key,
                "nonfinite finalized use",
            ));
        }
        if finalized.amount_kg_m2_stand_ground < 0.0 {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E006,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                candidate.transaction_id,
                &finalized.key,
                "negative finalized use",
            ));
        }
        if !finalized_keys.insert(finalized.key.clone()) {
            return Err(DirectSurfaceLiquidError::Protocol(
                "duplicate finalized use in candidate",
            ));
        }
        let store = configuration.store_key_for_water(&finalized.key)?;
        debit_entries.push((
            store,
            finalized.key.clone(),
            finalized.amount_kg_m2_stand_ground,
        ));
    }
    let debit_by_store = canonical_finalized_debits(candidate.transaction_id, debit_entries)?;
    let mut credit_by_store = BTreeMap::<DirectSurfaceLiquidStoreKey, f64>::new();
    let mut credit_details = BTreeMap::<DirectSurfaceLiquidStoreKey, &CondensationCredit>::new();
    for credit in &candidate.condensation_credits {
        validate_candidate_condensation_credit(configuration, candidate.transaction_id, credit)?;
        let store = configuration
            .records
            .iter()
            .find(|record| {
                record.key.ofe_id == credit.ofe_id
                    && record.key.tile_id == credit.tile_id
                    && record.key.surface_id == credit.surface_id
            })
            .map(|record| record.key.clone())
            .ok_or(DirectSurfaceLiquidError::Identity(
                "condensation store missing",
            ))?;
        if credit_details.insert(store.clone(), credit).is_some() {
            return Err(DirectSurfaceLiquidError::Protocol(
                "duplicate condensation credit in candidate",
            ));
        }
        credit_by_store.insert(store, credit.amount_kg_m2_stand_ground);
    }
    Ok((debit_by_store, credit_by_store, credit_details))
}

fn overflow_eq_bits(
    left: &[DirectCondensationOverflow],
    right: &[DirectCondensationOverflow],
) -> bool {
    left.len() == right.len()
        && left.iter().zip(right).all(|(left, right)| {
            left.store_key == right.store_key
                && left.amount_kg_m2_ofe_ground.to_bits() == right.amount_kg_m2_ofe_ground.to_bits()
                && left.temperature_k.to_bits() == right.temperature_k.to_bits()
                && left.specific_liquid_enthalpy_j_kg.to_bits()
                    == right.specific_liquid_enthalpy_j_kg.to_bits()
        })
}

fn validate_finalized_uses(
    arbitration: &DirectSurfaceLiquidArbitration,
    finalized_uses: &[WaterAmount],
) -> Result<BTreeMap<DirectSurfaceLiquidStoreKey, f64>, DirectSurfaceLiquidError> {
    let request_map = arbitration
        .requests
        .iter()
        .enumerate()
        .map(|(index, request)| (request.key.clone(), index))
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut debit_entries = Vec::with_capacity(finalized_uses.len());
    for finalized in finalized_uses {
        finalized
            .key
            .validate(arbitration.transaction_id)
            .map_err(|_| {
                water_protocol_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    DirectSurfaceLiquidPhase::ResourceCandidate,
                    arbitration.transaction_id,
                    &finalized.key,
                    "invalid finalized-use key",
                )
            })?;
        let index = *request_map.get(&finalized.key).ok_or_else(|| {
            water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E005,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                arbitration.transaction_id,
                &finalized.key,
                "use without request",
            )
        })?;
        if !seen.insert(finalized.key.clone()) {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E005,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                arbitration.transaction_id,
                &finalized.key,
                "duplicate finalized use",
            ));
        }
        if !finalized.amount_kg_m2_stand_ground.is_finite() {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E003,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                arbitration.transaction_id,
                &finalized.key,
                "nonfinite finalized use",
            ));
        }
        if finalized.amount_kg_m2_stand_ground < 0.0 {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E006,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                arbitration.transaction_id,
                &finalized.key,
                "negative finalized use",
            ));
        }
        let authorization = &arbitration.authorizations[index];
        if finalized.amount_kg_m2_stand_ground > authorization.amount_kg_m2_stand_ground
            || authorization.amount_kg_m2_stand_ground
                > arbitration.requests[index].amount_kg_m2_stand_ground
        {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E006,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                arbitration.transaction_id,
                &finalized.key,
                "F <= A <= D",
            ));
        }
        debit_entries.push((
            arbitration.request_store_keys[index].clone(),
            finalized.key.clone(),
            finalized.amount_kg_m2_stand_ground,
        ));
    }
    if seen.len() != arbitration.requests.len() {
        return Err(DirectSurfaceLiquidError::canonical_failure(
            DirectSurfaceLiquidErrorCode::E005,
            DirectSurfaceLiquidPhase::ResourceCandidate,
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(arbitration.transaction_id),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256: None,
                attempted_owner_sha256: None,
            },
            "missing finalized-use identity",
        ));
    }
    canonical_finalized_debits(arbitration.transaction_id, debit_entries)
}

type CondensationByStore<'a> = (
    BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
    BTreeMap<DirectSurfaceLiquidStoreKey, &'a CondensationCredit>,
);

fn collect_condensation_credits<'a>(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    condensation_credits: &'a [CondensationCredit],
) -> Result<CondensationByStore<'a>, DirectSurfaceLiquidError> {
    let mut credit_by_store = BTreeMap::<DirectSurfaceLiquidStoreKey, f64>::new();
    let mut credit_details = BTreeMap::<DirectSurfaceLiquidStoreKey, &CondensationCredit>::new();
    for credit in condensation_credits {
        validate_condensation_credit(configuration, arbitration, credit)?;
        let store = configuration
            .records
            .iter()
            .find(|record| {
                record.key.ofe_id == credit.ofe_id
                    && record.key.tile_id == credit.tile_id
                    && record.key.surface_id == credit.surface_id
            })
            .map(|record| record.key.clone())
            .ok_or(DirectSurfaceLiquidError::Identity(
                "condensation store missing",
            ))?;
        if credit_details.insert(store.clone(), credit).is_some() {
            return Err(DirectSurfaceLiquidError::Protocol(
                "duplicate condensation credit",
            ));
        }
        credit_by_store.insert(store, credit.amount_kg_m2_stand_ground);
    }
    Ok((credit_by_store, credit_details))
}

fn validate_condensation_credit(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    credit: &CondensationCredit,
) -> Result<(), DirectSurfaceLiquidError> {
    validate_candidate_condensation_credit(configuration, arbitration.transaction_id, credit)
}

fn validate_candidate_condensation_credit(
    configuration: &DirectSurfaceLiquidConfiguration,
    transaction_id: TransactionId,
    credit: &CondensationCredit,
) -> Result<(), DirectSurfaceLiquidError> {
    if credit.transaction_id != transaction_id
        || credit.hydrology_owner_id != configuration.owner_id
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "condensation transaction or owner mismatch",
        ));
    }
    if !credit.amount_kg_m2_stand_ground.is_finite()
        || !credit.temperature_k.is_finite()
        || !credit.specific_liquid_enthalpy_j_kg.is_finite()
    {
        return Err(DirectSurfaceLiquidError::Domain(
            "nonfinite condensation amount, temperature, or enthalpy",
        ));
    }
    if !(200.0..=350.0).contains(&credit.temperature_k) {
        return Err(DirectSurfaceLiquidError::Domain("condensation temperature"));
    }
    if credit.amount_kg_m2_stand_ground <= 0.0
        || credit.amount_basis != StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval
    {
        return Err(DirectSurfaceLiquidError::Bound(
            "nonpositive condensation amount or wrong basis",
        ));
    }
    if credit.specific_liquid_enthalpy_j_kg.to_bits()
        != openwepp_land_surface_energy::liquid_enthalpy_j_kg(credit.temperature_k).to_bits()
    {
        return Err(condensation_candidate_closure(
            configuration,
            transaction_id,
            credit,
            "condensation temperature/enthalpy mismatch",
        ));
    }
    Ok(())
}

fn validate_store_pair(
    surface_class: SurfaceClass,
    source_type: WaterSourceType,
) -> Result<(), DirectSurfaceLiquidError> {
    if matches!(
        (surface_class, source_type),
        (
            SurfaceClass::BareMineralSoil,
            WaterSourceType::SurfaceLiquid
        ) | (SurfaceClass::ForestLitter, WaterSourceType::LitterLiquid)
    ) {
        Ok(())
    } else {
        Err(DirectSurfaceLiquidError::Identity(
            "invalid surface/source pair",
        ))
    }
}

fn candidate_closure(detail: &'static str) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E009,
        DirectSurfaceLiquidPhase::ResourceCandidate,
        DirectSurfaceLiquidErrorContext::default(),
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

fn condensation_candidate_closure(
    configuration: &DirectSurfaceLiquidConfiguration,
    transaction_id: TransactionId,
    credit: &CondensationCredit,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    let source_id = configuration
        .records
        .iter()
        .find(|record| {
            record.key.ofe_id == credit.ofe_id
                && record.key.tile_id == credit.tile_id
                && record.key.surface_id == credit.surface_id
        })
        .map(|record| record.key.source_id.clone());
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E009,
        DirectSurfaceLiquidPhase::ResourceCandidate,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: Some(configuration.owner_id.clone()),
            ofe_id: Some(credit.ofe_id.clone()),
            tile_id: Some(credit.tile_id.clone()),
            surface_id: Some(credit.surface_id.clone()),
            source_id,
            parcel_id: None,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

fn surface_liquid_store_context(
    owner_id: &ResourceOwnerId,
    transaction_id: Option<TransactionId>,
    key: &DirectSurfaceLiquidStoreKey,
) -> DirectSurfaceLiquidErrorContext {
    DirectSurfaceLiquidErrorContext {
        transaction_id,
        owner_id: Some(owner_id.clone()),
        ofe_id: Some(key.ofe_id.clone()),
        tile_id: Some(key.tile_id.clone()),
        surface_id: Some(key.surface_id.clone()),
        source_id: Some(key.source_id.clone()),
        parcel_id: None,
    }
}

fn configuration_record_failure(
    error: DirectSurfaceLiquidError,
    context: DirectSurfaceLiquidErrorContext,
) -> DirectSurfaceLiquidError {
    let code = error.code();
    error.complete_context(
        code,
        DirectSurfaceLiquidPhase::Configuration,
        context,
        None,
        None,
    )
}

fn restart_record_failure(
    error: DirectSurfaceLiquidError,
    context: DirectSurfaceLiquidErrorContext,
) -> DirectSurfaceLiquidError {
    restart_record_failure_with_hash(error, context, None)
}

fn restart_record_failure_with_hash(
    error: DirectSurfaceLiquidError,
    context: DirectSurfaceLiquidErrorContext,
    beginning_owner_sha256: Option<String>,
) -> DirectSurfaceLiquidError {
    let code = error.code();
    error.complete_context(
        code,
        DirectSurfaceLiquidPhase::Restart,
        context,
        beginning_owner_sha256,
        None,
    )
}

fn validate_restart_store_domains(
    owner_id: &ResourceOwnerId,
    state: &DirectSurfaceLiquidStateRecord,
    configuration: &DirectSurfaceLiquidConfigurationRecord,
    expected_lineage: Option<TransactionId>,
) -> Result<(), DirectSurfaceLiquidError> {
    let transaction_id = state.last_accepted_transaction_id.or(expected_lineage);
    let context = surface_liquid_store_context(owner_id, transaction_id, &state.key);
    require_nonnegative(state.liquid_kg_m2_tile, "state liquid")
        .map_err(|error| restart_record_failure(error, context.clone()))?;
    if state.liquid_kg_m2_tile > configuration.capacity_kg_m2_tile {
        return Err(restart_record_failure(
            DirectSurfaceLiquidError::Domain("state liquid exceeds capacity"),
            context,
        ));
    }
    Ok(())
}

fn validate_restart_continuation_domains(
    owner_id: &ResourceOwnerId,
    continuations: &[DirectSurfaceLiquidContinuationState],
    expected_lineage: Option<TransactionId>,
) -> Result<(), DirectSurfaceLiquidError> {
    for continuation in continuations {
        if continuation.next_interval_index > 48 {
            return Err(restart_binding_failure(
                owner_id,
                expected_lineage,
                Some(continuation.ofe_id.clone()),
                "continuation interval exceeds 48",
            ));
        }
        let context = DirectSurfaceLiquidErrorContext {
            transaction_id: continuation
                .last_accepted_transaction_id
                .or(expected_lineage),
            owner_id: Some(owner_id.clone()),
            ofe_id: Some(continuation.ofe_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        };
        require_nonnegative(continuation.cumulative_supply_m, "cumulative supply")
            .map_err(|error| restart_record_failure(error, context.clone()))?;
        require_nonnegative(
            continuation.cumulative_infiltration_m,
            "cumulative infiltration",
        )
        .map_err(|error| restart_record_failure(error, context))?;
        if continuation.cumulative_infiltration_m > continuation.cumulative_supply_m {
            return Err(restart_binding_failure(
                owner_id,
                expected_lineage,
                Some(continuation.ofe_id.clone()),
                "cumulative infiltration exceeds supply",
            ));
        }
        match expected_lineage {
            None if continuation.next_interval_index == 0
                && continuation.cumulative_supply_m.to_bits() == 0.0_f64.to_bits()
                && continuation.cumulative_infiltration_m.to_bits() == 0.0_f64.to_bits() => {}
            Some(_) if (1..=48).contains(&continuation.next_interval_index) => {}
            None => {
                return Err(restart_binding_failure(
                    owner_id,
                    expected_lineage,
                    Some(continuation.ofe_id.clone()),
                    "initial continuation must be interval zero with zero carry",
                ));
            }
            Some(_) => {
                return Err(restart_binding_failure(
                    owner_id,
                    expected_lineage,
                    Some(continuation.ofe_id.clone()),
                    "accepted continuation must have interval 1..=48",
                ));
            }
        }
    }
    Ok(())
}

fn water_protocol_failure(
    code: DirectSurfaceLiquidErrorCode,
    phase: DirectSurfaceLiquidPhase,
    transaction_id: TransactionId,
    key: &GroundWaterKey,
    detail: impl Into<String>,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        code,
        phase,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: Some(key.requesting_owner_id.clone()),
            ofe_id: Some(key.ofe_id.clone()),
            tile_id: key.source_tile_id.clone(),
            surface_id: key.surface_id.clone(),
            source_id: Some(key.source_id.clone()),
            parcel_id: None,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

fn store_arithmetic_failure(
    configuration: &DirectSurfaceLiquidConfiguration,
    transaction_id: TransactionId,
    key: &DirectSurfaceLiquidStoreKey,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E003,
        DirectSurfaceLiquidPhase::ResourceCandidate,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: Some(configuration.owner_id.clone()),
            ofe_id: Some(key.ofe_id.clone()),
            tile_id: Some(key.tile_id.clone()),
            surface_id: Some(key.surface_id.clone()),
            source_id: Some(key.source_id.clone()),
            parcel_id: None,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

fn restart_binding_failure(
    owner_id: &ResourceOwnerId,
    transaction_id: Option<TransactionId>,
    ofe_id: Option<OfeId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E008,
        DirectSurfaceLiquidPhase::Restart,
        DirectSurfaceLiquidErrorContext {
            transaction_id,
            owner_id: Some(owner_id.clone()),
            ofe_id,
            ..DirectSurfaceLiquidErrorContext::default()
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

fn validate_same_ofe_value<T: Eq>(
    values: &mut BTreeMap<OfeId, T>,
    ofe_id: OfeId,
    value: T,
    error: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    match values.entry(ofe_id) {
        std::collections::btree_map::Entry::Vacant(entry) => {
            entry.insert(value);
        }
        std::collections::btree_map::Entry::Occupied(entry) if entry.get() != &value => {
            return Err(DirectSurfaceLiquidError::Identity(error));
        }
        std::collections::btree_map::Entry::Occupied(_) => {}
    }
    Ok(())
}

fn configured_ofes(configuration: &DirectSurfaceLiquidConfiguration) -> Vec<OfeId> {
    configuration.ofe_topology.clone()
}

fn require_positive(value: f64, field: &'static str) -> Result<(), DirectSurfaceLiquidError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(DirectSurfaceLiquidError::Domain(field))
    }
}

fn require_nonnegative(value: f64, field: &'static str) -> Result<(), DirectSurfaceLiquidError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(DirectSurfaceLiquidError::Domain(field))
    }
}

fn f64_bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}

fn parse_f64_bits(value: &str) -> Result<f64, DirectSurfaceLiquidError> {
    if value.len() != 16
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(DirectSurfaceLiquidError::Schema(
            "canonical f64 must be 16 lowercase hexadecimal digits",
        ));
    }
    let bits = u64::from_str_radix(value, 16)
        .map_err(|_| DirectSurfaceLiquidError::Schema("canonical f64 parse"))?;
    Ok(f64::from_bits(bits))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[cfg(test)]
#[path = "surface_liquid_owner_tests.rs"]
mod tests;

#[path = "surface_liquid_owner/identity_validation.rs"]
mod identity_validation;
