//! Typed frozen-litter V3 candidates and sealed receipt operands.

use openwepp_kernel_contract::{ResourceOwnerId, TileId, TransactionId};
use serde::{Deserialize, Serialize};

use crate::{OfeId, Sha256Digest};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LitterPhaseConfiguration {
    pub litter_depth_m: f64,
    pub dry_heat_capacity_j_m2_k: f64,
    pub liquid_capacity_kg_m2_tile: f64,
    pub ice_capacity_kg_m2_tile: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BeginningLitterPhaseState {
    pub liquid_kg_m2_tile: f64,
    pub ice_kg_m2_tile: f64,
    pub sensible_energy_j_m2_tile: f64,
    pub temperature_k: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LitterVaporEnvironment {
    pub accepted_phase_free_temperature_k: f64,
    pub air_density_kg_m3: f64,
    pub air_pressure_pa: f64,
    pub recipient_specific_humidity_kg_kg: f64,
    pub litter_to_canopy_resistance_s_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawLitterVapor {
    pub environment: LitterVaporEnvironment,
    pub frozen_fraction: f64,
    pub liquid_interception_factor: f64,
    pub ice_interception_factor: f64,
    pub liquid_saturation_specific_humidity_kg_kg: f64,
    pub raw_liquid_signed_rate_kg_m2_s: f64,
    pub raw_ice_signed_rate_kg_m2_s: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizedLitterVapor {
    pub liquid_signed_rate_kg_m2_s: f64,
    pub ice_signed_rate_kg_m2_s: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LitterVaporReceipt {
    pub raw: RawLitterVapor,
    pub finalized: FinalizedLitterVapor,
    pub liquid_signed_mass_kg_m2: f64,
    pub ice_signed_mass_kg_m2: f64,
    pub liquid_specific_enthalpy_j_kg: f64,
    pub ice_specific_enthalpy_j_kg: f64,
    pub liquid_signed_energy_j_m2: f64,
    pub ice_signed_energy_j_m2: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostVaporLitterState {
    pub liquid_kg_m2_tile: f64,
    pub ice_kg_m2_tile: f64,
    pub sensible_energy_j_m2_tile: f64,
    pub temperature_k: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LitterPhaseTransfer {
    pub warm_energy_mass_bound_kg_m2: f64,
    pub cold_energy_mass_bound_kg_m2: f64,
    pub melt_kg_m2: f64,
    pub freeze_kg_m2: f64,
    pub signed_phase_kg_m2: f64,
    pub fusion_energy_j_m2: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EndingLitterPhaseState {
    pub liquid_kg_m2_tile: f64,
    pub ice_kg_m2_tile: f64,
    pub sensible_energy_j_m2_tile: f64,
    pub temperature_k: f64,
    pub heat_capacity_j_m2_k: f64,
}

/// Exact post-phase liquid-capacity spill tied to the immutable raw phase
/// receipt. The raw ending remains receipt authority; only `retained_ending`
/// is eligible for installation in the surface owner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LitterPhaseCapacitySpillV1 {
    pub phase_receipt_sha256: Sha256Digest,
    pub lse_configuration_sha256: Sha256Digest,
    pub transaction_id: TransactionId,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub surface_owner_id: ResourceOwnerId,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub liquid_capacity_kg_m2_tile: f64,
    pub raw_ending: EndingLitterPhaseState,
    pub spill_liquid_kg_m2_tile: f64,
    pub spill_specific_sensible_enthalpy_j_kg: f64,
    pub spill_sensible_energy_j_m2_tile: f64,
    pub retained_ending: EndingLitterPhaseState,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LitterPhaseClosure {
    pub liquid_mass_residual_kg_m2: f64,
    pub ice_mass_residual_kg_m2: f64,
    pub total_phase_mass_residual_kg_m2: f64,
    pub fusion_energy_residual_j_m2: f64,
    pub phase_enthalpy_residual_j_m2: f64,
    pub ending_temperature_residual_k: f64,
    pub liquid_vapor_energy_residual_j_m2: f64,
    pub ice_vapor_energy_residual_j_m2: f64,
    pub phase_free_storage_residual_w_m2: f64,
    pub phase_free_surface_energy_residual_w_m2: f64,
    pub phase_free_producer_residual_delta_w_m2: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LitterPhaseReceiptIdentity {
    pub receipt_version: String,
    pub model_version: String,
    pub model_definition_sha256: Sha256Digest,
    pub lse_configuration_sha256: Sha256Digest,
    pub transaction_id: TransactionId,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub surface_owner_id: ResourceOwnerId,
    pub beginning_surface_owner_sha256: Sha256Digest,
    pub candidate_surface_owner_sha256: Sha256Digest,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub support_duration_seconds_bits: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LitterPhaseReceipt {
    pub identity: LitterPhaseReceiptIdentity,
    pub receipt_sha256: Sha256Digest,
    pub configuration: LitterPhaseConfiguration,
    pub beginning: BeginningLitterPhaseState,
    pub vapor: LitterVaporReceipt,
    pub post_vapor: PostVaporLitterState,
    pub phase_free_surface_energy: crate::V3PhaseFreeSurfaceEnergyLedger,
    pub transfer: LitterPhaseTransfer,
    pub ending: EndingLitterPhaseState,
    pub closure: LitterPhaseClosure,
    pub same_support_resolve_count: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AcceptedLitterPhaseCandidate {
    /// Immutable raw bounded-phase image sealed by `receipt`.
    pub ending: EndingLitterPhaseState,
    /// Within-capacity state eligible for surface-owner installation.
    pub retained_ending: EndingLitterPhaseState,
    pub capacity_spill: Option<LitterPhaseCapacitySpillV1>,
    pub receipt: LitterPhaseReceipt,
}
