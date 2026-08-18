//! Independent surface-liquid store, parcel, enthalpy, and routing closure.

#![allow(clippy::missing_errors_doc)]

#[path = "surface_liquid_closure_preflight.rs"]
mod arithmetic_preflight;
#[path = "surface_liquid_ending_validation.rs"]
mod ending_validation;
#[path = "surface_liquid_enthalpy_closure.rs"]
mod enthalpy_reconstruction;
#[path = "surface_liquid_raw_parent_closure.rs"]
mod raw_parent_reconstruction;
#[path = "surface_liquid_closure_comparison.rs"]
mod terminal_comparison;

use ending_validation::{
    ending_aggregate_failure, first_membership_aware_mismatch, validate_projected_ending_digest,
};
use terminal_comparison::{
    require_close_mass, validate_receipt_enthalpy, water_key_matches_record,
};

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::OfeId;

use super::runoff::{DirectWb14ContinuationIntervalInputs, advance_wb14_continuation_interval};
use super::surface_liquid_ingress::{
    CanonicalParcelOrderKey, CanonicalSurfaceLiquidSource, DirectCanopyLiquidRelease,
    DirectIngressAmount, DirectSurfaceLiquidIngressInput, DirectSurfaceLiquidParcelKind,
    DirectSurfaceLiquidParcelReceipt, DirectSurfaceLiquidReceiptDisposition,
    DirectSurfaceLiquidReceiptRecipient, DirectTileGroundIngress, INTERVAL_S,
    LIQUID_HEAT_CAPACITY_J_KG_K, REFERENCE_TEMPERATURE_K, WATER_DENSITY_KG_M3,
    canonical_parcel_order, canonical_surface_liquid_source_id,
};
use super::surface_liquid_owner::{
    DirectCondensationOverflow, DirectSurfaceLiquidClosureUnit, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidConfigurationRecord, DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode,
    DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidResourceCandidate, DirectSurfaceLiquidRollbackHashes,
    DirectSurfaceLiquidStoreKey, checked_surface_liquid_add, checked_surface_liquid_close,
    checked_surface_liquid_div, checked_surface_liquid_mul, checked_surface_liquid_sub,
    checked_surface_liquid_sum,
};

fn contextual_closure_failure(
    transaction_id: TransactionId,
    store_key: &DirectSurfaceLiquidStoreKey,
    parcel_id: Option<String>,
    detail: impl Into<String>,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E010,
        DirectSurfaceLiquidPhase::IndependentClosure,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: None,
            ofe_id: Some(store_key.ofe_id.clone()),
            tile_id: Some(store_key.tile_id.clone()),
            surface_id: Some(store_key.surface_id.clone()),
            source_id: Some(store_key.source_id.clone()),
            parcel_id,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

fn contextual_closure_arithmetic_failure(
    transaction_id: TransactionId,
    store_key: &DirectSurfaceLiquidStoreKey,
    parcel_id: Option<String>,
    detail: impl Into<String>,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E003,
        DirectSurfaceLiquidPhase::IndependentClosure,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: None,
            ofe_id: Some(store_key.ofe_id.clone()),
            tile_id: Some(store_key.tile_id.clone()),
            surface_id: Some(store_key.surface_id.clone()),
            source_id: Some(store_key.source_id.clone()),
            parcel_id,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

fn contextual_comparison_failure(
    code: DirectSurfaceLiquidErrorCode,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
    store_key: &DirectSurfaceLiquidStoreKey,
    parcel_id: Option<String>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::IndependentClosure,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: Some(owner_id.clone()),
            ofe_id: Some(store_key.ofe_id.clone()),
            tile_id: Some(store_key.tile_id.clone()),
            surface_id: Some(store_key.surface_id.clone()),
            source_id: Some(store_key.source_id.clone()),
            parcel_id,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

fn contextual_ofe_comparison_failure(
    code: DirectSurfaceLiquidErrorCode,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
    ofe_id: &OfeId,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::IndependentClosure,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: Some(owner_id.clone()),
            ofe_id: Some(ofe_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

/// Frozen, non-residual operands consumed by the independent closure validator.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSurfaceLiquidClosureOperands {
    transaction_id: TransactionId,
    stores: Vec<DirectSurfaceLiquidStoreClosureOperands>,
    source_parcels: Vec<DirectSurfaceLiquidParcelClosureOperands>,
    partition_inputs: Vec<DirectSurfaceLiquidPartitionClosureOperands>,
}

#[derive(Clone, Debug, PartialEq)]
struct DirectSurfaceLiquidPartitionClosureOperands {
    ofe_id: OfeId,
    effective_conductivity_m_s: f64,
    matric_potential_m: f64,
    infiltration_storage_capacity_m: f64,
    beginning_cumulative_supply_m: f64,
    beginning_cumulative_infiltration_m: f64,
    ending_day_index: usize,
    ending_next_interval_index: u8,
}

impl DirectSurfaceLiquidClosureOperands {
    #[must_use]
    pub const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub fn stores(&self) -> &[DirectSurfaceLiquidStoreClosureOperands] {
        &self.stores
    }

    #[must_use]
    pub fn source_parcels(&self) -> &[DirectSurfaceLiquidParcelClosureOperands] {
        &self.source_parcels
    }

    pub(super) fn first_source_identity_mismatch(
        &self,
        expected: &Self,
    ) -> Option<(DirectSurfaceLiquidStoreKey, String)> {
        let actual = self
            .source_parcels
            .iter()
            .map(FrozenSourceIdentity::from)
            .collect::<Vec<_>>();
        let frozen = expected
            .source_parcels
            .iter()
            .map(FrozenSourceIdentity::from)
            .collect::<Vec<_>>();
        if actual.len() < frozen.len() {
            let actual_set = actual.iter().cloned().collect::<BTreeSet<_>>();
            if let Some(missing) = frozen.iter().find(|row| !actual_set.contains(*row)) {
                return Some((
                    missing.origin_store_key.clone(),
                    missing.source_parcel_id.clone(),
                ));
            }
        }
        actual
            .iter()
            .zip(&frozen)
            .find(|(actual_row, expected_row)| actual_row != expected_row)
            .map(|(row, _)| (row.origin_store_key.clone(), row.source_parcel_id.clone()))
            .or_else(|| {
                actual
                    .get(frozen.len())
                    .map(|row| (row.origin_store_key.clone(), row.source_parcel_id.clone()))
            })
    }

    pub(super) fn first_partition_input_mismatch(&self, expected: &Self) -> Option<OfeId> {
        let actual_ids = self
            .partition_inputs
            .iter()
            .map(|row| row.ofe_id.clone())
            .collect::<Vec<_>>();
        let expected_ids = expected
            .partition_inputs
            .iter()
            .map(|row| row.ofe_id.clone())
            .collect::<Vec<_>>();
        let actual_set = actual_ids.iter().cloned().collect::<BTreeSet<_>>();
        let expected_set = expected_ids.iter().cloned().collect::<BTreeSet<_>>();
        expected_ids
            .iter()
            .find(|id| !actual_set.contains(*id))
            .cloned()
            .or_else(|| {
                actual_ids
                    .iter()
                    .find(|id| !expected_set.contains(*id))
                    .cloned()
            })
            .or_else(|| {
                self.partition_inputs
                    .iter()
                    .zip(&expected.partition_inputs)
                    .find(|(actual, frozen)| actual != frozen)
                    .map(|(actual, _)| actual.ofe_id.clone())
            })
            .or_else(|| {
                self.partition_inputs
                    .get(expected.partition_inputs.len())
                    .map(|row| row.ofe_id.clone())
            })
            .or_else(|| {
                expected
                    .partition_inputs
                    .get(self.partition_inputs.len())
                    .map(|row| row.ofe_id.clone())
            })
    }

    #[cfg(test)]
    pub(super) fn remove_partition_input_for_test(&mut self, index: usize) -> OfeId {
        self.partition_inputs.remove(index).ofe_id
    }

    #[cfg(test)]
    pub(super) fn duplicate_partition_input_for_test(&mut self, index: usize) -> OfeId {
        let duplicate = self.partition_inputs[index].clone();
        let id = duplicate.ofe_id.clone();
        self.partition_inputs.push(duplicate);
        id
    }

    #[cfg(test)]
    pub(super) fn reorder_partition_inputs_for_test(&mut self) {
        self.partition_inputs.swap(0, 1);
    }

    #[cfg(test)]
    pub(super) fn rekey_partition_input_for_test(&mut self, index: usize, ofe_id: OfeId) {
        self.partition_inputs[index].ofe_id = ofe_id;
    }

    #[cfg(test)]
    pub(super) fn poison_partition_cumulative_bound_for_test(&mut self, index: usize) {
        self.partition_inputs[index].beginning_cumulative_supply_m = 0.0;
        self.partition_inputs[index].beginning_cumulative_infiltration_m = f64::MIN_POSITIVE;
    }

    #[cfg(test)]
    pub(super) fn poison_partition_capacity_bound_for_test(&mut self, index: usize) {
        self.partition_inputs[index].infiltration_storage_capacity_m = 0.0;
        self.partition_inputs[index].beginning_cumulative_supply_m = 1.0;
        self.partition_inputs[index].beginning_cumulative_infiltration_m = f64::MIN_POSITIVE;
    }

    #[cfg(test)]
    pub(super) fn poison_first_beginning_for_test(&mut self) {
        if let Some(first) = self.stores.first_mut() {
            first.beginning_liquid_kg_m2_tile += 0.25;
        }
    }

    #[cfg(test)]
    pub(super) fn poison_first_store_arithmetic_overflow_for_test(&mut self) {
        if let Some(first) = self.stores.first_mut() {
            first.beginning_liquid_kg_m2_tile = f64::MAX;
            first.retained_excess_kg_m2_ofe_ground = f64::MAX;
        }
    }

    #[cfg(test)]
    pub(super) fn forge_first_store_retained_and_ending_for_test(&mut self, delta_tile: f64) {
        let first = self.stores.first_mut().expect("store operand");
        first.retained_excess_kg_m2_ofe_ground += delta_tile * first.tile_fraction;
        first.ending_liquid_kg_m2_tile += delta_tile;
    }

    #[cfg(test)]
    pub(super) fn poison_first_finite_and_last_arithmetic_for_test(&mut self) {
        if let Some(first) = self.stores.first_mut() {
            first.beginning_liquid_kg_m2_tile += 0.25;
        }
        if let Some(last) = self.stores.last_mut() {
            last.beginning_liquid_kg_m2_tile = f64::MAX;
            last.retained_excess_kg_m2_ofe_ground = f64::MAX;
        }
    }

    #[cfg(test)]
    pub(super) fn poison_finite_store_and_two_parcel_aggregate_for_test(&mut self) -> Vec<String> {
        if let Some(first) = self.stores.first_mut() {
            first.beginning_liquid_kg_m2_tile += 0.25;
        }
        self.source_parcels
            .iter_mut()
            .filter(|parcel| parcel.mass_kg_m2_basis_ofe_ground > 0.0)
            .take(2)
            .map(|parcel| {
                parcel.enthalpy_j_m2_basis_ofe_ground = f64::MAX * 0.3;
                parcel.source_parcel_id.clone()
            })
            .collect()
    }

    #[cfg(test)]
    pub(super) fn swap_first_two_source_enthalpies_for_test(&mut self) -> (String, String) {
        let (first, rest) = self.source_parcels.split_first_mut().expect("first parcel");
        let second = rest.first_mut().expect("second parcel");
        std::mem::swap(
            &mut first.enthalpy_j_m2_basis_ofe_ground,
            &mut second.enthalpy_j_m2_basis_ofe_ground,
        );
        (
            first.source_parcel_id.clone(),
            second.source_parcel_id.clone(),
        )
    }

    #[cfg(test)]
    pub(super) fn poison_first_source_comparison_scale_for_test(&mut self) -> String {
        let first = self.source_parcels.first_mut().expect("source parcel");
        first.enthalpy_j_m2_basis_ofe_ground = f64::MAX * 0.6;
        first.source_parcel_id.clone()
    }

    #[cfg(test)]
    pub(super) fn poison_first_source_mass_comparison_scale_for_test(&mut self) -> String {
        let first = self.source_parcels.first_mut().expect("source parcel");
        first.mass_kg_m2_basis_ofe_ground = f64::MAX * 0.6;
        first.temperature_k = REFERENCE_TEMPERATURE_K;
        first.specific_liquid_enthalpy_j_kg = 0.0;
        first.enthalpy_j_m2_basis_ofe_ground = 0.0;
        first.source_parcel_id.clone()
    }

    #[cfg(test)]
    pub(super) fn remove_source_for_test(&mut self, index: usize) -> String {
        self.source_parcels.remove(index).source_parcel_id
    }

    #[cfg(test)]
    pub(super) fn duplicate_first_source_for_test(&mut self) -> String {
        let duplicate = self.source_parcels.first().expect("source parcel").clone();
        let id = duplicate.source_parcel_id.clone();
        self.source_parcels.push(duplicate);
        id
    }

    #[cfg(test)]
    pub(super) fn rekey_first_source_for_test(&mut self) -> String {
        let first = self.source_parcels.first_mut().expect("source parcel");
        first.source_parcel_id.push_str(":rekeyed");
        first.source_parcel_id.clone()
    }

    #[cfg(test)]
    pub(super) fn swap_first_two_source_kinds_for_test(&mut self) -> String {
        let (first, rest) = self.source_parcels.split_first_mut().expect("first parcel");
        let second = rest.first_mut().expect("second parcel");
        std::mem::swap(&mut first.kind, &mut second.kind);
        first.source_parcel_id.clone()
    }

    #[cfg(test)]
    pub(super) fn poison_first_source_support_for_test(&mut self) -> String {
        let first = self.source_parcels.first_mut().expect("source parcel");
        first.start_s += 1.0;
        first.source_parcel_id.clone()
    }

    #[cfg(test)]
    pub(super) fn poison_first_source_nan_support_for_test(&mut self) -> String {
        let first = self.source_parcels.first_mut().expect("source parcel");
        first.start_s = f64::NAN;
        first.source_parcel_id.clone()
    }

    #[cfg(test)]
    pub(super) fn poison_first_source_raw_enthalpy_for_test(&mut self) -> String {
        let first = self.source_parcels.first_mut().expect("source parcel");
        first.enthalpy_j_m2_basis_ofe_ground += 1.0;
        first.source_parcel_id.clone()
    }
}

/// The exact full-equation terms for one persistent surface store.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSurfaceLiquidStoreClosureOperands {
    store_key: DirectSurfaceLiquidStoreKey,
    tile_fraction: f64,
    beginning_liquid_kg_m2_tile: f64,
    finalized_withdrawal_kg_m2_ofe_ground: f64,
    condensation_credit_kg_m2_ofe_ground: f64,
    condensation_overflow_kg_m2_ofe_ground: f64,
    retained_excess_kg_m2_ofe_ground: f64,
    ending_liquid_kg_m2_tile: f64,
}

impl DirectSurfaceLiquidStoreClosureOperands {
    #[must_use]
    pub const fn store_key(&self) -> &DirectSurfaceLiquidStoreKey {
        &self.store_key
    }

    #[must_use]
    pub const fn tile_fraction(&self) -> f64 {
        self.tile_fraction
    }

    #[must_use]
    pub const fn beginning_liquid_kg_m2_tile(&self) -> f64 {
        self.beginning_liquid_kg_m2_tile
    }

    #[must_use]
    pub const fn finalized_withdrawal_kg_m2_ofe_ground(&self) -> f64 {
        self.finalized_withdrawal_kg_m2_ofe_ground
    }

    #[must_use]
    pub const fn condensation_credit_kg_m2_ofe_ground(&self) -> f64 {
        self.condensation_credit_kg_m2_ofe_ground
    }

    #[must_use]
    pub const fn condensation_overflow_kg_m2_ofe_ground(&self) -> f64 {
        self.condensation_overflow_kg_m2_ofe_ground
    }

    #[must_use]
    pub const fn retained_excess_kg_m2_ofe_ground(&self) -> f64 {
        self.retained_excess_kg_m2_ofe_ground
    }

    #[must_use]
    pub const fn ending_liquid_kg_m2_tile(&self) -> f64 {
        self.ending_liquid_kg_m2_tile
    }
}

/// An original ingress parcel before any producing partition or routing arithmetic.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSurfaceLiquidParcelClosureOperands {
    source_parcel_id: String,
    origin_store_key: DirectSurfaceLiquidStoreKey,
    basis_ofe_id: OfeId,
    kind: DirectSurfaceLiquidParcelKind,
    start_s: f64,
    end_s: f64,
    temperature_k: f64,
    specific_liquid_enthalpy_j_kg: f64,
    mass_kg_m2_basis_ofe_ground: f64,
    enthalpy_j_m2_basis_ofe_ground: f64,
}

impl DirectSurfaceLiquidParcelClosureOperands {
    #[must_use]
    pub fn source_parcel_id(&self) -> &str {
        &self.source_parcel_id
    }

    #[must_use]
    pub const fn origin_store_key(&self) -> &DirectSurfaceLiquidStoreKey {
        &self.origin_store_key
    }

    #[must_use]
    pub const fn basis_ofe_id(&self) -> &OfeId {
        &self.basis_ofe_id
    }

    #[must_use]
    pub const fn kind(&self) -> DirectSurfaceLiquidParcelKind {
        self.kind
    }

    #[must_use]
    pub const fn start_s(&self) -> f64 {
        self.start_s
    }

    #[must_use]
    pub const fn end_s(&self) -> f64 {
        self.end_s
    }

    #[must_use]
    pub const fn mass_kg_m2_basis_ofe_ground(&self) -> f64 {
        self.mass_kg_m2_basis_ofe_ground
    }

    #[must_use]
    pub const fn enthalpy_j_m2_basis_ofe_ground(&self) -> f64 {
        self.enthalpy_j_m2_basis_ofe_ground
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ParcelJoinKey {
    owner_id: ResourceOwnerId,
    source_parcel_id: String,
    origin_store_key: DirectSurfaceLiquidStoreKey,
    recipient_store_key: DirectSurfaceLiquidStoreKey,
    recipient: DirectSurfaceLiquidReceiptRecipient,
    basis_ofe_id: OfeId,
    kind: DirectSurfaceLiquidParcelKind,
    start_s_bits: u64,
    end_s_bits: u64,
    disposition: Option<DirectSurfaceLiquidReceiptDisposition>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FrozenSourceIdentity {
    source_parcel_id: String,
    kind: DirectSurfaceLiquidParcelKind,
    origin_store_key: DirectSurfaceLiquidStoreKey,
    basis_ofe_id: OfeId,
    start_s_bits: u64,
    end_s_bits: u64,
}

impl From<&DirectSurfaceLiquidParcelClosureOperands> for FrozenSourceIdentity {
    fn from(parcel: &DirectSurfaceLiquidParcelClosureOperands) -> Self {
        Self {
            source_parcel_id: parcel.source_parcel_id.clone(),
            kind: parcel.kind,
            origin_store_key: parcel.origin_store_key.clone(),
            basis_ofe_id: parcel.basis_ofe_id.clone(),
            start_s_bits: parcel.start_s.to_bits(),
            end_s_bits: parcel.end_s.to_bits(),
        }
    }
}

fn frozen_source_identity_order(
    left: &FrozenSourceIdentity,
    right: &FrozenSourceIdentity,
) -> std::cmp::Ordering {
    canonical_parcel_order(
        CanonicalParcelOrderKey {
            start_s: f64::from_bits(left.start_s_bits),
            end_s: f64::from_bits(left.end_s_bits),
            origin_store_key: &left.origin_store_key,
            kind: left.kind,
            source_parcel_id: &left.source_parcel_id,
        },
        CanonicalParcelOrderKey {
            start_s: f64::from_bits(right.start_s_bits),
            end_s: f64::from_bits(right.end_s_bits),
            origin_store_key: &right.origin_store_key,
            kind: right.kind,
            source_parcel_id: &right.source_parcel_id,
        },
    )
}

#[derive(Clone, Copy, Debug, Default)]
struct AmountPair {
    mass: f64,
    enthalpy: f64,
}

#[derive(Clone)]
struct RawParcelSegment {
    source_parcel_id: String,
    basis_ofe_id: OfeId,
    origin_store_key: DirectSurfaceLiquidStoreKey,
    recipient_store_key: DirectSurfaceLiquidStoreKey,
    kind: DirectSurfaceLiquidParcelKind,
    start_s: f64,
    end_s: f64,
    mass: f64,
    enthalpy: f64,
}

fn frozen_parcel_order(
    left: &DirectSurfaceLiquidParcelClosureOperands,
    right: &DirectSurfaceLiquidParcelClosureOperands,
) -> std::cmp::Ordering {
    canonical_parcel_order(
        CanonicalParcelOrderKey {
            start_s: left.start_s,
            end_s: left.end_s,
            origin_store_key: &left.origin_store_key,
            kind: left.kind,
            source_parcel_id: &left.source_parcel_id,
        },
        CanonicalParcelOrderKey {
            start_s: right.start_s,
            end_s: right.end_s,
            origin_store_key: &right.origin_store_key,
            kind: right.kind,
            source_parcel_id: &right.source_parcel_id,
        },
    )
}

fn projected_parcel_order(left: &RawParcelSegment, right: &RawParcelSegment) -> std::cmp::Ordering {
    canonical_parcel_order(
        CanonicalParcelOrderKey {
            start_s: left.start_s,
            end_s: left.end_s,
            origin_store_key: &left.origin_store_key,
            kind: left.kind,
            source_parcel_id: &left.source_parcel_id,
        },
        CanonicalParcelOrderKey {
            start_s: right.start_s,
            end_s: right.end_s,
            origin_store_key: &right.origin_store_key,
            kind: right.kind,
            source_parcel_id: &right.source_parcel_id,
        },
    )
}

struct ParcelArithmeticProjection {
    expected: BTreeMap<ParcelJoinKey, AmountPair>,
    actual: BTreeMap<ParcelJoinKey, AmountPair>,
    expected_source_mass: BTreeMap<(OfeId, String), f64>,
    actual_source_mass: BTreeMap<(OfeId, String), f64>,
    raw_source_mass: BTreeMap<(OfeId, String), f64>,
    expected_ofe_mass: BTreeMap<OfeId, f64>,
    actual_ofe_mass: BTreeMap<OfeId, f64>,
    raw_ofe_mass: BTreeMap<OfeId, f64>,
    expected_ofe_enthalpy: BTreeMap<OfeId, f64>,
    actual_ofe_enthalpy: BTreeMap<OfeId, f64>,
    raw_ofe_enthalpy: BTreeMap<OfeId, f64>,
    expected_store_liquid: BTreeMap<DirectSurfaceLiquidStoreKey, f64>,
    expected_continuations: BTreeMap<OfeId, DirectProjectedContinuation>,
}

#[derive(Clone, Copy)]
struct DirectProjectedContinuation {
    day_index: usize,
    next_interval_index: u8,
    cumulative_supply_m: f64,
    cumulative_infiltration_m: f64,
    transaction_id: TransactionId,
}

#[derive(Clone, Copy)]
enum ComparisonDisposition {
    ArithmeticPreflight,
    RequireClosure,
}

#[derive(Clone, Copy)]
struct StoreArithmeticProjection {
    pre_ingress_liquid_kg_m2_tile: f64,
    ending_liquid_kg_m2_tile: f64,
}

impl AmountPair {
    fn checked_add(&mut self, mass: f64, enthalpy: f64) -> Option<()> {
        self.mass = checked_surface_liquid_add(self.mass, mass)?;
        self.enthalpy = checked_surface_liquid_add(self.enthalpy, enthalpy)?;
        Some(())
    }
}

fn project_store_arithmetic(
    row: &DirectSurfaceLiquidStoreClosureOperands,
) -> Option<StoreArithmeticProjection> {
    if !row.tile_fraction.is_finite() || row.tile_fraction <= 0.0 {
        return None;
    }
    let finalized =
        checked_surface_liquid_div(row.finalized_withdrawal_kg_m2_ofe_ground, row.tile_fraction)?;
    let condensation =
        checked_surface_liquid_div(row.condensation_credit_kg_m2_ofe_ground, row.tile_fraction)?;
    let overflow = checked_surface_liquid_div(
        row.condensation_overflow_kg_m2_ofe_ground,
        row.tile_fraction,
    )?;
    let retained =
        checked_surface_liquid_div(row.retained_excess_kg_m2_ofe_ground, row.tile_fraction)?;
    let pre_ingress = checked_surface_liquid_sub(row.beginning_liquid_kg_m2_tile, finalized)
        .and_then(|value| checked_surface_liquid_add(value, condensation))
        .and_then(|value| checked_surface_liquid_sub(value, overflow))?;
    let ending = checked_surface_liquid_add(pre_ingress, retained)?;
    Some(StoreArithmeticProjection {
        pre_ingress_liquid_kg_m2_tile: pre_ingress,
        ending_liquid_kg_m2_tile: ending,
    })
}

fn projection_key_store(key: &ParcelJoinKey) -> &DirectSurfaceLiquidStoreKey {
    &key.recipient_store_key
}

#[allow(clippy::too_many_arguments)]
fn compare_projected_value(
    actual: f64,
    expected: f64,
    unit: DirectSurfaceLiquidClosureUnit,
    disposition: ComparisonDisposition,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
    store_key: &DirectSurfaceLiquidStoreKey,
    parcel_id: Option<String>,
    detail: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    match checked_surface_liquid_close(actual, expected, unit) {
        None => Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E003,
            transaction_id,
            owner_id,
            store_key,
            parcel_id,
            detail,
        )),
        Some(false) if matches!(disposition, ComparisonDisposition::RequireClosure) => {
            Err(contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E010,
                transaction_id,
                owner_id,
                store_key,
                parcel_id,
                detail,
            ))
        }
        Some(_) => Ok(()),
    }
}

#[allow(clippy::too_many_arguments)]
fn compare_ofe_value(
    actual: f64,
    expected: f64,
    unit: DirectSurfaceLiquidClosureUnit,
    disposition: ComparisonDisposition,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
    ofe_id: &OfeId,
    detail: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    match checked_surface_liquid_close(actual, expected, unit) {
        None => Err(contextual_ofe_comparison_failure(
            DirectSurfaceLiquidErrorCode::E003,
            transaction_id,
            owner_id,
            ofe_id,
            detail,
        )),
        Some(false) if matches!(disposition, ComparisonDisposition::RequireClosure) => {
            Err(contextual_ofe_comparison_failure(
                DirectSurfaceLiquidErrorCode::E010,
                transaction_id,
                owner_id,
                ofe_id,
                detail,
            ))
        }
        Some(_) => Ok(()),
    }
}

pub(super) fn capture_and_validate_surface_liquid_closure(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    ending: &DirectSurfaceLiquidOwnedState,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<DirectSurfaceLiquidClosureOperands, DirectSurfaceLiquidError> {
    (|| {
        let operands = capture_operands(configuration, resource, input, ending, receipts)?;
        validate_surface_liquid_closure_operands_with_input(
            configuration,
            resource,
            input,
            &operands,
            receipts,
            ending,
        )?;
        Ok(operands)
    })()
    .map_err(|error: DirectSurfaceLiquidError| {
        let code = error.code();
        error.complete_context(
            code,
            DirectSurfaceLiquidPhase::IndependentClosure,
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(input.transaction_id),
                owner_id: Some(resource.beginning_state().owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(resource.beginning_state().state_sha256.clone()),
            ending.recomputed_sha256().ok(),
        )
    })
}

#[allow(clippy::too_many_lines)]
fn capture_operands(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    ending: &DirectSurfaceLiquidOwnedState,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<DirectSurfaceLiquidClosureOperands, DirectSurfaceLiquidError> {
    let mut stores = Vec::with_capacity(configuration.records.len());
    for configured in &configuration.records {
        let beginning = resource
            .beginning_state()
            .records
            .iter()
            .find(|row| row.key == configured.key)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "closure beginning store missing",
            ))?;
        let ending_record = ending
            .records
            .iter()
            .find(|row| row.key == configured.key)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "closure ending store missing",
            ))?;
        let finalized_values = resource
            .finalized_uses()
            .iter()
            .filter(|row| water_key_matches_record(&row.key, configured))
            .map(|row| row.amount_kg_m2_stand_ground)
            .collect::<Vec<_>>();
        let finalized =
            checked_surface_liquid_sum(finalized_values.iter().copied()).ok_or_else(|| {
                contextual_closure_arithmetic_failure(
                    input.transaction_id,
                    &configured.key,
                    None,
                    "finalized-use closure sum is nonfinite or underflowed",
                )
            })?;
        let condensation_values = resource
            .condensation_credits()
            .iter()
            .filter(|row| {
                row.ofe_id == configured.key.ofe_id
                    && row.tile_id == configured.key.tile_id
                    && row.surface_id == configured.key.surface_id
            })
            .map(|row| row.amount_kg_m2_stand_ground)
            .collect::<Vec<_>>();
        let condensation = checked_surface_liquid_sum(condensation_values.iter().copied())
            .ok_or_else(|| {
                contextual_closure_arithmetic_failure(
                    input.transaction_id,
                    &configured.key,
                    None,
                    "condensation closure sum is nonfinite or underflowed",
                )
            })?;
        let overflow_values = resource
            .condensation_overflow()
            .iter()
            .filter(|row| row.store_key == configured.key)
            .map(|row| row.amount_kg_m2_ofe_ground)
            .collect::<Vec<_>>();
        let overflow =
            checked_surface_liquid_sum(overflow_values.iter().copied()).ok_or_else(|| {
                contextual_closure_arithmetic_failure(
                    input.transaction_id,
                    &configured.key,
                    None,
                    "overflow closure sum is nonfinite or underflowed",
                )
            })?;
        let retained_values = receipts
            .iter()
            .filter(|receipt| {
                receipt.disposition == DirectSurfaceLiquidReceiptDisposition::RetainedSurface
                    && matches!(
                        &receipt.recipient,
                        DirectSurfaceLiquidReceiptRecipient::SurfaceStore { store_key }
                            if store_key == &configured.key
                    )
            })
            .map(|receipt| receipt.mass_kg_m2_basis_ofe_ground)
            .collect::<Vec<_>>();
        let retained =
            checked_surface_liquid_sum(retained_values.iter().copied()).ok_or_else(|| {
                contextual_closure_arithmetic_failure(
                    input.transaction_id,
                    &configured.key,
                    None,
                    "retained closure sum is nonfinite or underflowed",
                )
            })?;
        stores.push(DirectSurfaceLiquidStoreClosureOperands {
            store_key: configured.key.clone(),
            tile_fraction: configured.tile_fraction,
            beginning_liquid_kg_m2_tile: beginning.liquid_kg_m2_tile,
            finalized_withdrawal_kg_m2_ofe_ground: finalized,
            condensation_credit_kg_m2_ofe_ground: condensation,
            condensation_overflow_kg_m2_ofe_ground: overflow,
            retained_excess_kg_m2_ofe_ground: retained,
            ending_liquid_kg_m2_tile: ending_record.liquid_kg_m2_tile,
        });
    }
    let source_parcels = capture_source_parcels(configuration, resource, input)?;
    let partition_inputs = configuration
        .ofe_topology
        .iter()
        .map(|ofe_id| {
            let parameter = input
                .wb14_parameters
                .iter()
                .find(|row| &row.ofe_id == ofe_id)
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "closure WB14 parameter missing",
                ))?;
            let continuation = resource
                .beginning_state()
                .continuations
                .iter()
                .find(|row| &row.ofe_id == ofe_id)
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "closure beginning continuation missing",
                ))?;
            let (beginning_cumulative_supply_m, beginning_cumulative_infiltration_m) =
                if continuation.next_interval_index == 48 {
                    (0.0, 0.0)
                } else {
                    (
                        continuation.cumulative_supply_m,
                        continuation.cumulative_infiltration_m,
                    )
                };
            Ok(DirectSurfaceLiquidPartitionClosureOperands {
                ofe_id: ofe_id.clone(),
                effective_conductivity_m_s: parameter.effective_conductivity_m_s,
                matric_potential_m: parameter.matric_potential_m,
                infiltration_storage_capacity_m: parameter.infiltration_storage_capacity_m,
                beginning_cumulative_supply_m,
                beginning_cumulative_infiltration_m,
                ending_day_index: input.day_index,
                ending_next_interval_index: input.interval_index.checked_add(1).ok_or(
                    DirectSurfaceLiquidError::Closure("closure interval index overflow"),
                )?,
            })
        })
        .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
    Ok(DirectSurfaceLiquidClosureOperands {
        transaction_id: input.transaction_id,
        stores,
        source_parcels,
        partition_inputs,
    })
}

fn capture_source_parcels(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<Vec<DirectSurfaceLiquidParcelClosureOperands>, DirectSurfaceLiquidError> {
    let mut result = Vec::new();
    for ingress in &input.tile_ingress {
        let (ofe_id, tile_id, _) = ingress.identity();
        let configured = configuration
            .records
            .iter()
            .find(|row| row.key.ofe_id == *ofe_id && row.key.tile_id == *tile_id)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "closure ingress store missing",
            ))?;
        match ingress {
            DirectTileGroundIngress::OpenRawPrecipitation {
                raw_precipitation, ..
            } => capture_amount(
                configured,
                DirectSurfaceLiquidParcelKind::RawPrecipitation,
                raw_precipitation,
                &mut result,
                input.transaction_id,
            )?,
            DirectTileGroundIngress::OpenLiquidParcels { parcels, .. } => {
                for parcel in parcels {
                    capture_amount(
                        configured,
                        parcel.kind,
                        &parcel.amount,
                        &mut result,
                        input.transaction_id,
                    )?;
                    let captured = result.last_mut().ok_or(DirectSurfaceLiquidError::Closure(
                        "missing captured open liquid parcel",
                    ))?;
                    captured.source_parcel_id = parcel.parcel_id.to_string();
                }
            }
            DirectTileGroundIngress::CoveredCanopyRelease { release, .. } => {
                capture_canopy_release(configured, release, &mut result, input.transaction_id)?;
            }
            DirectTileGroundIngress::CoveredCanopyReleaseAndRunon {
                release,
                runon_parcels,
                ..
            } => {
                capture_canopy_release(configured, release, &mut result, input.transaction_id)?;
                for parcel in runon_parcels {
                    capture_amount(
                        configured,
                        parcel.kind,
                        &parcel.amount,
                        &mut result,
                        input.transaction_id,
                    )?;
                    result
                        .last_mut()
                        .ok_or(DirectSurfaceLiquidError::Closure(
                            "missing captured covered runon parcel",
                        ))?
                        .source_parcel_id = parcel.parcel_id.to_string();
                }
            }
        }
    }
    for overflow in resource.condensation_overflow() {
        result.push(capture_overflow(input.transaction_id, overflow)?);
    }
    result.sort_by(frozen_parcel_order);
    Ok(result)
}

fn capture_canopy_release(
    configured: &DirectSurfaceLiquidConfigurationRecord,
    release: &DirectCanopyLiquidRelease,
    result: &mut Vec<DirectSurfaceLiquidParcelClosureOperands>,
    transaction_id: TransactionId,
) -> Result<(), DirectSurfaceLiquidError> {
    for (kind, amount) in [
        (
            DirectSurfaceLiquidParcelKind::CanopyThroughfall,
            &release.throughfall,
        ),
        (
            DirectSurfaceLiquidParcelKind::CanopyInitialDrainage,
            &release.initial_drainage,
        ),
        (
            DirectSurfaceLiquidParcelKind::CanopySecondDrainage,
            &release.second_drainage,
        ),
        (
            DirectSurfaceLiquidParcelKind::CanopyStemflow,
            &release.stemflow,
        ),
    ] {
        capture_amount(configured, kind, amount, result, transaction_id)?;
    }
    Ok(())
}

fn capture_amount(
    configured: &DirectSurfaceLiquidConfigurationRecord,
    kind: DirectSurfaceLiquidParcelKind,
    amount: &DirectIngressAmount,
    result: &mut Vec<DirectSurfaceLiquidParcelClosureOperands>,
    transaction_id: TransactionId,
) -> Result<(), DirectSurfaceLiquidError> {
    let mass = checked_surface_liquid_mul(configured.tile_fraction, amount.mass_kg_m2_tile_ground)
        .ok_or_else(|| {
            contextual_closure_arithmetic_failure(
                transaction_id,
                &configured.key,
                None,
                "ingress closure area conversion is nonfinite or underflowed",
            )
        })?;
    let enthalpy = checked_surface_liquid_mul(mass, amount.specific_liquid_enthalpy_j_kg)
        .ok_or_else(|| {
            contextual_closure_arithmetic_failure(
                transaction_id,
                &configured.key,
                None,
                "ingress closure enthalpy is nonfinite or underflowed",
            )
        })?;
    result.push(DirectSurfaceLiquidParcelClosureOperands {
        source_parcel_id: canonical_surface_liquid_source_id(CanonicalSurfaceLiquidSource::Local {
            store_key: &configured.key,
            kind,
        }),
        origin_store_key: configured.key.clone(),
        basis_ofe_id: configured.key.ofe_id.clone(),
        kind,
        start_s: amount.start_s,
        end_s: amount.end_s,
        temperature_k: amount.temperature_k,
        specific_liquid_enthalpy_j_kg: amount.specific_liquid_enthalpy_j_kg,
        mass_kg_m2_basis_ofe_ground: mass,
        enthalpy_j_m2_basis_ofe_ground: enthalpy,
    });
    Ok(())
}

fn capture_overflow(
    transaction_id: TransactionId,
    overflow: &DirectCondensationOverflow,
) -> Result<DirectSurfaceLiquidParcelClosureOperands, DirectSurfaceLiquidError> {
    let enthalpy = checked_surface_liquid_mul(
        overflow.amount_kg_m2_ofe_ground,
        overflow.specific_liquid_enthalpy_j_kg,
    )
    .ok_or_else(|| {
        contextual_closure_arithmetic_failure(
            transaction_id,
            &overflow.store_key,
            None,
            "overflow closure enthalpy is nonfinite or underflowed",
        )
    })?;
    Ok(DirectSurfaceLiquidParcelClosureOperands {
        source_parcel_id: canonical_surface_liquid_source_id(
            CanonicalSurfaceLiquidSource::Condensation {
                transaction_id,
                store_key: &overflow.store_key,
            },
        ),
        origin_store_key: overflow.store_key.clone(),
        basis_ofe_id: overflow.store_key.ofe_id.clone(),
        kind: DirectSurfaceLiquidParcelKind::CondensationOverflow,
        start_s: 0.0,
        end_s: INTERVAL_S,
        temperature_k: overflow.temperature_k,
        specific_liquid_enthalpy_j_kg: overflow.specific_liquid_enthalpy_j_kg,
        mass_kg_m2_basis_ofe_ground: overflow.amount_kg_m2_ofe_ground,
        enthalpy_j_m2_basis_ofe_ground: enthalpy,
    })
}

#[cfg_attr(not(test), allow(dead_code))]
pub(super) fn validate_surface_liquid_closure_operands(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    ending: &DirectSurfaceLiquidOwnedState,
) -> Result<(), DirectSurfaceLiquidError> {
    validate_surface_liquid_closure_operands_inner(
        configuration,
        resource,
        None,
        operands,
        receipts,
        ending,
    )
}

pub(super) fn validate_surface_liquid_closure_operands_with_input(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    ending: &DirectSurfaceLiquidOwnedState,
) -> Result<(), DirectSurfaceLiquidError> {
    validate_surface_liquid_closure_operands_inner(
        configuration,
        resource,
        Some(input),
        operands,
        receipts,
        ending,
    )
}

fn validate_surface_liquid_closure_operands_inner(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: Option<&DirectSurfaceLiquidIngressInput>,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    ending: &DirectSurfaceLiquidOwnedState,
) -> Result<(), DirectSurfaceLiquidError> {
    let result = (|| {
        if operands.transaction_id != resource.transaction_id() {
            return Err(DirectSurfaceLiquidError::Closure(
                "independent closure transaction mismatch",
            ));
        }
        preflight_surface_liquid_closure_arithmetic(configuration, resource, operands, receipts)?;
        arithmetic_preflight::validate_partition_input_identities(configuration, operands)?;
        validate_frozen_source_identities(configuration, resource, input, operands)?;
        validate_store_equations(configuration, resource, operands)?;
        validate_parcel_joins(configuration, operands, receipts, ending)
    })();
    result.map_err(|error| {
        let code = error.code();
        error.complete_context(
            code,
            DirectSurfaceLiquidPhase::IndependentClosure,
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(operands.transaction_id),
                owner_id: Some(configuration.owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(resource.beginning_state().state_sha256.clone()),
            ending.recomputed_sha256().ok(),
        )
    })
}

fn validate_frozen_source_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: Option<&DirectSurfaceLiquidIngressInput>,
    operands: &DirectSurfaceLiquidClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    let mut expected = Vec::new();
    for record in &configuration.records {
        expected.extend(frozen_identities_for_record(record, input)?);
    }
    for overflow in resource.condensation_overflow() {
        expected.push(FrozenSourceIdentity {
            source_parcel_id: canonical_surface_liquid_source_id(
                CanonicalSurfaceLiquidSource::Condensation {
                    transaction_id: operands.transaction_id,
                    store_key: &overflow.store_key,
                },
            ),
            kind: DirectSurfaceLiquidParcelKind::CondensationOverflow,
            origin_store_key: overflow.store_key.clone(),
            basis_ofe_id: overflow.store_key.ofe_id.clone(),
            start_s_bits: 0.0_f64.to_bits(),
            end_s_bits: INTERVAL_S.to_bits(),
        });
    }
    if input.is_none() {
        align_unfrozen_support(&mut expected, &operands.source_parcels);
    }
    expected.sort_by(frozen_source_identity_order);
    let actual = operands
        .source_parcels
        .iter()
        .map(FrozenSourceIdentity::from)
        .collect::<Vec<_>>();
    if actual == expected {
        return Ok(());
    }
    let offending = frozen_identity_mismatch(&actual, &expected)?;
    Err(contextual_closure_failure(
        operands.transaction_id,
        &offending.origin_store_key,
        Some(offending.source_parcel_id.clone()),
        "frozen source parcel identity mismatch",
    ))
}

fn frozen_identities_for_record(
    record: &DirectSurfaceLiquidConfigurationRecord,
    input: Option<&DirectSurfaceLiquidIngressInput>,
) -> Result<Vec<FrozenSourceIdentity>, DirectSurfaceLiquidError> {
    if let Some(input) = input {
        let ingress = input
            .tile_ingress
            .iter()
            .find(|ingress| {
                ingress.identity()
                    == (
                        &record.key.ofe_id,
                        &record.key.tile_id,
                        &record.key.surface_id,
                    )
            })
            .ok_or(DirectSurfaceLiquidError::Closure(
                "missing ingress for frozen source identity",
            ))?;
        match ingress {
            DirectTileGroundIngress::OpenRawPrecipitation {
                raw_precipitation, ..
            } => {
                return Ok(vec![local_frozen_identity(
                    record,
                    DirectSurfaceLiquidParcelKind::RawPrecipitation,
                    raw_precipitation.start_s,
                    raw_precipitation.end_s,
                )]);
            }
            DirectTileGroundIngress::OpenLiquidParcels { parcels, .. } => {
                return Ok(parcels
                    .iter()
                    .map(|parcel| FrozenSourceIdentity {
                        source_parcel_id: parcel.parcel_id.to_string(),
                        kind: parcel.kind,
                        origin_store_key: record.key.clone(),
                        basis_ofe_id: record.key.ofe_id.clone(),
                        start_s_bits: parcel.amount.start_s.to_bits(),
                        end_s_bits: parcel.amount.end_s.to_bits(),
                    })
                    .collect());
            }
            DirectTileGroundIngress::CoveredCanopyRelease { .. } => {}
            DirectTileGroundIngress::CoveredCanopyReleaseAndRunon { runon_parcels, .. } => {
                let mut identities = covered_canonical_frozen_identities(record);
                identities.extend(runon_parcels.iter().map(|parcel| FrozenSourceIdentity {
                    source_parcel_id: parcel.parcel_id.to_string(),
                    kind: parcel.kind,
                    origin_store_key: record.key.clone(),
                    basis_ofe_id: record.key.ofe_id.clone(),
                    start_s_bits: parcel.amount.start_s.to_bits(),
                    end_s_bits: parcel.amount.end_s.to_bits(),
                }));
                return Ok(identities);
            }
        }
    }
    let kinds: &[DirectSurfaceLiquidParcelKind] = match record.ground_ingress_mode {
        super::surface_liquid_owner::DirectGroundIngressMode::OpenRawPrecipitation => {
            &[DirectSurfaceLiquidParcelKind::RawPrecipitation]
        }
        super::surface_liquid_owner::DirectGroundIngressMode::CoveredCanopyRelease => &[
            DirectSurfaceLiquidParcelKind::CanopyThroughfall,
            DirectSurfaceLiquidParcelKind::CanopyInitialDrainage,
            DirectSurfaceLiquidParcelKind::CanopySecondDrainage,
            DirectSurfaceLiquidParcelKind::CanopyStemflow,
        ],
    };
    Ok(kinds
        .iter()
        .map(|kind| local_frozen_identity(record, *kind, 0.0, INTERVAL_S))
        .collect())
}

fn covered_canonical_frozen_identities(
    record: &DirectSurfaceLiquidConfigurationRecord,
) -> Vec<FrozenSourceIdentity> {
    [
        DirectSurfaceLiquidParcelKind::CanopyThroughfall,
        DirectSurfaceLiquidParcelKind::CanopyInitialDrainage,
        DirectSurfaceLiquidParcelKind::CanopySecondDrainage,
        DirectSurfaceLiquidParcelKind::CanopyStemflow,
    ]
    .into_iter()
    .map(|kind| local_frozen_identity(record, kind, 0.0, INTERVAL_S))
    .collect()
}

fn local_frozen_identity(
    record: &DirectSurfaceLiquidConfigurationRecord,
    kind: DirectSurfaceLiquidParcelKind,
    start_s: f64,
    end_s: f64,
) -> FrozenSourceIdentity {
    FrozenSourceIdentity {
        source_parcel_id: canonical_surface_liquid_source_id(CanonicalSurfaceLiquidSource::Local {
            store_key: &record.key,
            kind,
        }),
        kind,
        origin_store_key: record.key.clone(),
        basis_ofe_id: record.key.ofe_id.clone(),
        start_s_bits: start_s.to_bits(),
        end_s_bits: end_s.to_bits(),
    }
}

fn align_unfrozen_support(
    expected: &mut [FrozenSourceIdentity],
    actual: &[DirectSurfaceLiquidParcelClosureOperands],
) {
    for expected_row in expected {
        if let Some(actual_row) = actual
            .iter()
            .find(|row| row.source_parcel_id == expected_row.source_parcel_id)
        {
            expected_row.start_s_bits = actual_row.start_s.to_bits();
            expected_row.end_s_bits = actual_row.end_s.to_bits();
        }
    }
}

fn frozen_identity_mismatch<'a>(
    actual: &'a [FrozenSourceIdentity],
    expected: &'a [FrozenSourceIdentity],
) -> Result<&'a FrozenSourceIdentity, DirectSurfaceLiquidError> {
    let actual_set = actual.iter().cloned().collect::<BTreeSet<_>>();
    let mismatch = if actual.len() < expected.len() {
        expected
            .iter()
            .find(|row| !actual_set.contains(*row))
            .or_else(|| actual.first())
    } else {
        actual
            .iter()
            .zip(expected)
            .find(|(actual_row, expected_row)| actual_row != expected_row)
            .map(|(actual_row, _)| actual_row)
            .or_else(|| actual.get(expected.len()))
            .or_else(|| expected.first())
    };
    mismatch.ok_or(DirectSurfaceLiquidError::Closure(
        "empty frozen source identity mismatch",
    ))
}

#[allow(clippy::too_many_lines)]
fn parcel_join_key(
    owner_id: &ResourceOwnerId,
    segment: &RawParcelSegment,
    recipient: DirectSurfaceLiquidReceiptRecipient,
    disposition: DirectSurfaceLiquidReceiptDisposition,
    start_s: f64,
    end_s: f64,
) -> ParcelJoinKey {
    ParcelJoinKey {
        owner_id: owner_id.clone(),
        source_parcel_id: segment.source_parcel_id.clone(),
        origin_store_key: segment.origin_store_key.clone(),
        recipient_store_key: segment.recipient_store_key.clone(),
        recipient,
        basis_ofe_id: segment.basis_ofe_id.clone(),
        kind: segment.kind,
        start_s_bits: start_s.to_bits(),
        end_s_bits: end_s.to_bits(),
        disposition: Some(disposition),
    }
}

fn receipt_join_key(
    owner_id: &ResourceOwnerId,
    receipt: &DirectSurfaceLiquidParcelReceipt,
) -> ParcelJoinKey {
    ParcelJoinKey {
        owner_id: owner_id.clone(),
        source_parcel_id: receipt.source_parcel_id.clone(),
        origin_store_key: receipt.origin_store_key.clone(),
        recipient_store_key: receipt.recipient_store_key.clone(),
        recipient: receipt.recipient.clone(),
        basis_ofe_id: receipt.basis_ofe_id.clone(),
        kind: receipt.kind,
        start_s_bits: receipt.start_s.to_bits(),
        end_s_bits: receipt.end_s.to_bits(),
        disposition: Some(receipt.disposition),
    }
}

fn add_expected_partition(
    expected: &mut BTreeMap<ParcelJoinKey, AmountPair>,
    key: ParcelJoinKey,
    mass: f64,
    enthalpy: f64,
) -> Option<()> {
    if !mass.is_finite() || mass < 0.0 || !enthalpy.is_finite() {
        return None;
    }
    expected.entry(key).or_default().checked_add(mass, enthalpy)
}

fn project_actual_receipt_arithmetic(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<BTreeMap<ParcelJoinKey, AmountPair>, DirectSurfaceLiquidError> {
    let mut actual = BTreeMap::<ParcelJoinKey, AmountPair>::new();
    for receipt in receipts {
        actual
            .entry(receipt_join_key(&configuration.owner_id, receipt))
            .or_default()
            .checked_add(
                receipt.mass_kg_m2_basis_ofe_ground,
                receipt.enthalpy_j_m2_basis_ofe_ground,
            )
            .ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &receipt.recipient_store_key,
                    Some(receipt.parcel_id.clone()),
                    "receipt aggregate arithmetic",
                )
            })?;
    }
    Ok(actual)
}

#[allow(clippy::too_many_lines)]
fn project_parcel_arithmetic(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<ParcelArithmeticProjection, DirectSurfaceLiquidError> {
    let actual = project_actual_receipt_arithmetic(configuration, operands, receipts)?;

    let mut raw_segments = operands
        .source_parcels
        .iter()
        .map(|parcel| RawParcelSegment {
            source_parcel_id: parcel.source_parcel_id.clone(),
            basis_ofe_id: parcel.basis_ofe_id.clone(),
            origin_store_key: parcel.origin_store_key.clone(),
            recipient_store_key: parcel.origin_store_key.clone(),
            kind: parcel.kind,
            start_s: parcel.start_s,
            end_s: parcel.end_s,
            mass: parcel.mass_kg_m2_basis_ofe_ground,
            enthalpy: parcel.enthalpy_j_m2_basis_ofe_ground,
        })
        .collect::<Vec<_>>();
    let mut store_liquid = operands
        .stores
        .iter()
        .map(|store| {
            project_store_arithmetic(store)
                .map(|projected| {
                    (
                        store.store_key.clone(),
                        projected.pre_ingress_liquid_kg_m2_tile,
                    )
                })
                .ok_or_else(|| {
                    contextual_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        &store.store_key,
                        None,
                        "partition beginning-store arithmetic",
                    )
                })
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;

    let mut expected = BTreeMap::<ParcelJoinKey, AmountPair>::new();
    let (raw_ofe_mass, raw_source_mass) =
        raw_parent_reconstruction::reconstruct_raw_parent_mass(configuration, operands)?;
    let mut replayed_ofe_enthalpy = BTreeMap::<OfeId, f64>::new();
    let mut expected_continuations = BTreeMap::<OfeId, DirectProjectedContinuation>::new();
    for ofe_id in &configuration.ofe_topology {
        let partition = operands
            .partition_inputs
            .iter()
            .find(|row| &row.ofe_id == ofe_id)
            .ok_or_else(|| {
                contextual_ofe_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    ofe_id,
                    "frozen partition inputs missing",
                )
            })?;
        let binding = configuration
            .ofe_bindings
            .iter()
            .find(|row| &row.ofe_id == ofe_id)
            .ok_or(DirectSurfaceLiquidError::Closure(
                "partition infiltration binding missing",
            ))?;
        let route = configuration
            .records
            .iter()
            .find(|row| &row.key.ofe_id == ofe_id)
            .ok_or(DirectSurfaceLiquidError::Closure(
                "partition route source missing",
            ))?;
        let segments = raw_segments
            .iter()
            .filter(|segment| &segment.basis_ofe_id == ofe_id)
            .cloned()
            .collect::<Vec<_>>();
        let mut boundaries = segments
            .iter()
            .flat_map(|segment| [segment.start_s, segment.end_s])
            .collect::<Vec<_>>();
        boundaries.extend([0.0, INTERVAL_S]);
        boundaries.sort_by(f64::total_cmp);
        boundaries.dedup_by(|left, right| left.to_bits() == right.to_bits());

        let mut cumulative_supply_m = partition.beginning_cumulative_supply_m;
        let mut cumulative_infiltration_m = partition.beginning_cumulative_infiltration_m;
        let mut routed_segments = Vec::new();
        let mut allocated_temporal_mass = vec![0.0; segments.len()];
        let mut allocated_temporal_enthalpy = vec![0.0; segments.len()];
        for window in boundaries.windows(2) {
            let start_s = window[0];
            let end_s = window[1];
            if end_s <= start_s {
                continue;
            }
            let mut contributions = segments
                .iter()
                .enumerate()
                .filter(|(_, segment)| segment.start_s <= start_s && segment.end_s >= end_s)
                .map(|(segment_index, segment)| {
                    let fraction = checked_surface_liquid_div(
                        end_s - start_s,
                        segment.end_s - segment.start_s,
                    )?;
                    // Replay the producer's temporal child identity separately
                    // so receipt bits remain independently checked. Raw mass
                    // custody is reconstructed above only from frozen parents,
                    // never from these replayed children.
                    let is_last = end_s.to_bits() == segment.end_s.to_bits();
                    let (mass, allocated) = enthalpy_reconstruction::allocate_ordered_child(
                        segment.mass,
                        allocated_temporal_mass[segment_index],
                        checked_surface_liquid_mul(segment.mass, fraction),
                        is_last,
                    )?;
                    if mass < 0.0 {
                        return None;
                    }
                    allocated_temporal_mass[segment_index] = allocated;
                    let (enthalpy, allocated) = enthalpy_reconstruction::allocate_ordered_child(
                        segment.enthalpy,
                        allocated_temporal_enthalpy[segment_index],
                        enthalpy_reconstruction::proportional_q(
                            segment.enthalpy,
                            end_s - start_s,
                            segment.end_s - segment.start_s,
                        ),
                        is_last,
                    )?;
                    allocated_temporal_enthalpy[segment_index] = allocated;
                    Some((segment.clone(), mass, enthalpy))
                })
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition window projection arithmetic",
                    )
                })?;
            contributions.retain(|row| row.1 > 0.0);
            contributions.sort_by(|left, right| projected_parcel_order(&left.0, &right.0));
            let supply_mass = checked_surface_liquid_sum(contributions.iter().map(|row| row.1))
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition supply mass arithmetic",
                    )
                })?;
            let supply_enthalpy = checked_surface_liquid_sum(contributions.iter().map(|row| row.2))
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition supply enthalpy arithmetic",
                    )
                })?;
            if supply_mass == 0.0 {
                continue;
            }
            let raw_enthalpy = replayed_ofe_enthalpy.entry(ofe_id.clone()).or_default();
            *raw_enthalpy =
                checked_surface_liquid_add(*raw_enthalpy, supply_enthalpy).ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition raw enthalpy aggregate arithmetic",
                    )
                })?;
            let duration_s = end_s - start_s;
            let interval_supply_m = checked_surface_liquid_div(supply_mass, WATER_DENSITY_KG_M3)
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition supply depth arithmetic",
                    )
                })?;
            let outcome =
                advance_wb14_continuation_interval(DirectWb14ContinuationIntervalInputs {
                    cumulative_supply_m,
                    cumulative_infiltration_m,
                    interval_supply_m,
                    interval_duration_s: duration_s,
                    effective_conductivity_m_s: partition.effective_conductivity_m_s,
                    matric_potential_m: partition.matric_potential_m,
                    storage_capacity_m: partition.infiltration_storage_capacity_m,
                })
                .map_err(|_| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "independent WB14 partition rejected",
                    )
                })?;
            cumulative_supply_m = outcome.cumulative_supply_m;
            cumulative_infiltration_m = outcome.cumulative_infiltration_m;
            let full_infiltration =
                outcome.interval_infiltration_m.to_bits() == interval_supply_m.to_bits();
            let total_infiltration = if full_infiltration {
                supply_mass
            } else {
                checked_surface_liquid_mul(outcome.interval_infiltration_m, WATER_DENSITY_KG_M3)
                    .ok_or_else(|| {
                        contextual_ofe_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            ofe_id,
                            "partition infiltration mass arithmetic",
                        )
                    })?
            };
            let _h_mix =
                checked_surface_liquid_div(supply_enthalpy, supply_mass).ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition mixture enthalpy arithmetic",
                    )
                })?;
            let mut allocated_infiltration = 0.0;
            let mut allocated_mixed_enthalpy = 0.0;
            let count = contributions.len();
            let mut excess_by_store =
                BTreeMap::<DirectSurfaceLiquidStoreKey, Vec<(RawParcelSegment, f64, f64)>>::new();
            for (index, (segment, mass, _)) in contributions.into_iter().enumerate() {
                let (mixed_part_q, allocated) = enthalpy_reconstruction::allocate_ordered_child(
                    supply_enthalpy,
                    allocated_mixed_enthalpy,
                    enthalpy_reconstruction::proportional_q(supply_enthalpy, mass, supply_mass),
                    index + 1 == count,
                )
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition mixed enthalpy allocation arithmetic",
                    )
                })?;
                allocated_mixed_enthalpy = allocated;
                let infiltrated = if full_infiltration {
                    Some(mass)
                } else if index + 1 == count {
                    checked_surface_liquid_sub(total_infiltration, allocated_infiltration)
                } else {
                    checked_surface_liquid_mul(total_infiltration, mass)
                        .and_then(|value| checked_surface_liquid_div(value, supply_mass))
                }
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition infiltration allocation arithmetic",
                    )
                })?;
                allocated_infiltration =
                    checked_surface_liquid_add(allocated_infiltration, infiltrated).ok_or_else(
                        || {
                            contextual_ofe_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                ofe_id,
                                "partition infiltration accumulation arithmetic",
                            )
                        },
                    )?;
                let infiltration_recipient =
                    DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
                        ofe_id: binding.ofe_id.clone(),
                        production_lane_index: binding.production_lane_index,
                        production_lane_id: binding.production_lane_id,
                        ordered_soil_layer_ids: binding.ordered_soil_layer_ids.clone(),
                        soil_thermal_layer_id: binding.infiltration_soil_thermal_layer_id.clone(),
                    };
                let key = parcel_join_key(
                    &configuration.owner_id,
                    &segment,
                    infiltration_recipient,
                    DirectSurfaceLiquidReceiptDisposition::Infiltration,
                    start_s,
                    end_s,
                );
                let excess = checked_surface_liquid_sub(mass, infiltrated).ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition excess arithmetic",
                    )
                })?;
                let (infiltration_q, excess_q) =
                    enthalpy_reconstruction::split_first_then_remainder(
                        mixed_part_q,
                        mass,
                        infiltrated,
                        excess,
                    )
                    .ok_or_else(|| {
                        contextual_ofe_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            ofe_id,
                            "partition infiltration enthalpy arithmetic",
                        )
                    })?;
                add_expected_partition(&mut expected, key, infiltrated, infiltration_q)
                    .ok_or_else(|| {
                        contextual_ofe_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            ofe_id,
                            "partition infiltration enthalpy arithmetic",
                        )
                    })?;
                excess_by_store
                    .entry(segment.recipient_store_key.clone())
                    .or_default()
                    .push((segment, excess, excess_q));
            }

            for (store_key, mut parts) in excess_by_store {
                parts.sort_by(|left, right| projected_parcel_order(&left.0, &right.0));
                let configured = configuration
                    .records
                    .iter()
                    .find(|row| row.key == store_key)
                    .ok_or(DirectSurfaceLiquidError::Closure(
                        "partition recipient store missing",
                    ))?;
                let current_liquid = store_liquid.get(&store_key).copied().ok_or(
                    DirectSurfaceLiquidError::Closure("partition store state missing"),
                )?;
                let available_tile =
                    checked_surface_liquid_sub(configured.capacity_kg_m2_tile, current_liquid)
                        .ok_or_else(|| {
                            contextual_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                &store_key,
                                None,
                                "partition retention capacity arithmetic",
                            )
                        })?;
                let available =
                    checked_surface_liquid_mul(configured.tile_fraction, available_tile)
                        .ok_or_else(|| {
                            contextual_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                &store_key,
                                None,
                                "partition retention area arithmetic",
                            )
                        })?;
                let total_excess = checked_surface_liquid_sum(parts.iter().map(|row| row.1))
                    .ok_or_else(|| {
                        contextual_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            &store_key,
                            None,
                            "partition excess aggregate arithmetic",
                        )
                    })?;
                let total_retained = total_excess.min(available);
                let mut allocated_retained = 0.0;
                let part_count = parts.len();
                for (index, (segment, excess, excess_q)) in parts.into_iter().enumerate() {
                    let retained = if index + 1 == part_count {
                        checked_surface_liquid_sub(total_retained, allocated_retained)
                    } else if total_excess == 0.0 {
                        Some(0.0)
                    } else {
                        checked_surface_liquid_mul(total_retained, excess)
                            .and_then(|value| checked_surface_liquid_div(value, total_excess))
                    }
                    .ok_or_else(|| {
                        contextual_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            &store_key,
                            Some(segment.source_parcel_id.clone()),
                            "partition retained allocation arithmetic",
                        )
                    })?;
                    allocated_retained = checked_surface_liquid_add(allocated_retained, retained)
                        .ok_or_else(|| {
                        contextual_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            &store_key,
                            Some(segment.source_parcel_id.clone()),
                            "partition retained accumulation arithmetic",
                        )
                    })?;
                    let runoff = checked_surface_liquid_sub(excess, retained).ok_or_else(|| {
                        contextual_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            &store_key,
                            Some(segment.source_parcel_id.clone()),
                            "partition runoff arithmetic",
                        )
                    })?;
                    let (retained_q, runoff_q) =
                        enthalpy_reconstruction::split_first_then_remainder(
                            excess_q, excess, retained, runoff,
                        )
                        .ok_or_else(|| {
                            contextual_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                &store_key,
                                Some(segment.source_parcel_id.clone()),
                                "partition retained enthalpy arithmetic",
                            )
                        })?;
                    if retained > 0.0 {
                        let key = parcel_join_key(
                            &configuration.owner_id,
                            &segment,
                            DirectSurfaceLiquidReceiptRecipient::SurfaceStore {
                                store_key: store_key.clone(),
                            },
                            DirectSurfaceLiquidReceiptDisposition::RetainedSurface,
                            start_s,
                            end_s,
                        );
                        add_expected_partition(&mut expected, key, retained, retained_q)
                            .ok_or_else(|| {
                                contextual_comparison_failure(
                                    DirectSurfaceLiquidErrorCode::E003,
                                    operands.transaction_id,
                                    &configuration.owner_id,
                                    &store_key,
                                    Some(segment.source_parcel_id.clone()),
                                    "partition retained enthalpy arithmetic",
                                )
                            })?;
                    }
                    if runoff == 0.0 {
                        continue;
                    }
                    let (disposition, recipient) = if let (
                        Some(destination_ofe),
                        Some(destination_tile),
                    ) = (
                        route.runon_destination_ofe_id.as_ref(),
                        route.runon_destination_tile_id.as_ref(),
                    ) {
                        let destination = configuration
                            .records
                            .iter()
                            .find(|row| {
                                &row.key.ofe_id == destination_ofe
                                    && &row.key.tile_id == destination_tile
                            })
                            .ok_or(DirectSurfaceLiquidError::Closure(
                                "partition route destination missing",
                            ))?;
                        let area_ratio =
                            checked_surface_liquid_div(route.ofe_area_m2, destination.ofe_area_m2)
                                .ok_or_else(|| {
                                    contextual_comparison_failure(
                                        DirectSurfaceLiquidErrorCode::E003,
                                        operands.transaction_id,
                                        &configuration.owner_id,
                                        &store_key,
                                        Some(segment.source_parcel_id.clone()),
                                        "partition route area arithmetic",
                                    )
                                })?;
                        let routed_mass = checked_surface_liquid_mul(runoff, area_ratio)
                            .ok_or_else(|| {
                                contextual_comparison_failure(
                                    DirectSurfaceLiquidErrorCode::E003,
                                    operands.transaction_id,
                                    &configuration.owner_id,
                                    &store_key,
                                    Some(segment.source_parcel_id.clone()),
                                    "partition routed mass arithmetic",
                                )
                            })?;
                        let routed_enthalpy = checked_surface_liquid_mul(runoff_q, area_ratio)
                            .ok_or_else(|| {
                                contextual_comparison_failure(
                                    DirectSurfaceLiquidErrorCode::E003,
                                    operands.transaction_id,
                                    &configuration.owner_id,
                                    &store_key,
                                    Some(segment.source_parcel_id.clone()),
                                    "partition routed enthalpy arithmetic",
                                )
                            })?;
                        routed_segments.push(RawParcelSegment {
                            source_parcel_id: segment.source_parcel_id.clone(),
                            basis_ofe_id: destination_ofe.clone(),
                            origin_store_key: segment.origin_store_key.clone(),
                            recipient_store_key: destination.key.clone(),
                            kind: DirectSurfaceLiquidParcelKind::UpstreamRunon,
                            start_s,
                            end_s,
                            mass: routed_mass,
                            enthalpy: routed_enthalpy,
                        });
                        (
                            DirectSurfaceLiquidReceiptDisposition::RoutedRunoff,
                            DirectSurfaceLiquidReceiptRecipient::RoutedOfe {
                                source_ofe_id: segment.basis_ofe_id.clone(),
                                destination_ofe_id: destination_ofe.clone(),
                                destination_store_key: destination.key.clone(),
                            },
                        )
                    } else {
                        (
                            DirectSurfaceLiquidReceiptDisposition::OutletRunoff,
                            DirectSurfaceLiquidReceiptRecipient::Outlet {
                                ofe_id: segment.basis_ofe_id.clone(),
                            },
                        )
                    };
                    let key = parcel_join_key(
                        &configuration.owner_id,
                        &segment,
                        recipient,
                        disposition,
                        start_s,
                        end_s,
                    );
                    add_expected_partition(&mut expected, key, runoff, runoff_q).ok_or_else(
                        || {
                            contextual_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                &store_key,
                                Some(segment.source_parcel_id.clone()),
                                "partition runoff enthalpy arithmetic",
                            )
                        },
                    )?;
                }
                let retained_tile =
                    checked_surface_liquid_div(total_retained, configured.tile_fraction)
                        .and_then(|value| checked_surface_liquid_add(current_liquid, value))
                        .ok_or_else(|| {
                            contextual_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                &store_key,
                                None,
                                "partition retained store update arithmetic",
                            )
                        })?;
                store_liquid.insert(store_key, retained_tile);
            }
        }
        expected_continuations.insert(
            ofe_id.clone(),
            DirectProjectedContinuation {
                day_index: partition.ending_day_index,
                next_interval_index: partition.ending_next_interval_index,
                cumulative_supply_m,
                cumulative_infiltration_m,
                transaction_id: operands.transaction_id,
            },
        );
        raw_segments.extend(routed_segments);
    }

    let mut expected_source_mass = BTreeMap::<(OfeId, String), f64>::new();
    let mut expected_ofe_mass = BTreeMap::<OfeId, f64>::new();
    let mut expected_ofe_enthalpy = BTreeMap::<OfeId, f64>::new();
    for (key, amount) in &expected {
        let source_mass = expected_source_mass
            .entry((key.basis_ofe_id.clone(), key.source_parcel_id.clone()))
            .or_default();
        *source_mass = checked_surface_liquid_add(*source_mass, amount.mass).ok_or_else(|| {
            contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &key.recipient_store_key,
                Some(key.source_parcel_id.clone()),
                "expected source mass aggregate arithmetic",
            )
        })?;
        let ofe_mass = expected_ofe_mass
            .entry(key.basis_ofe_id.clone())
            .or_default();
        *ofe_mass = checked_surface_liquid_add(*ofe_mass, amount.mass).ok_or_else(|| {
            contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &key.recipient_store_key,
                Some(key.source_parcel_id.clone()),
                "expected OFE mass aggregate arithmetic",
            )
        })?;
        let accumulated = expected_ofe_enthalpy
            .entry(key.basis_ofe_id.clone())
            .or_default();
        *accumulated =
            checked_surface_liquid_add(*accumulated, amount.enthalpy).ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &key.recipient_store_key,
                    Some(key.source_parcel_id.clone()),
                    "expected OFE enthalpy aggregate arithmetic",
                )
            })?;
    }
    let mut actual_source_mass = BTreeMap::<(OfeId, String), f64>::new();
    let mut actual_ofe_mass = BTreeMap::<OfeId, f64>::new();
    let mut actual_ofe_enthalpy = BTreeMap::<OfeId, f64>::new();
    for (key, amount) in &actual {
        let source_mass = actual_source_mass
            .entry((key.basis_ofe_id.clone(), key.source_parcel_id.clone()))
            .or_default();
        *source_mass = checked_surface_liquid_add(*source_mass, amount.mass).ok_or_else(|| {
            contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &key.recipient_store_key,
                Some(key.source_parcel_id.clone()),
                "actual source mass aggregate arithmetic",
            )
        })?;
        let ofe_mass = actual_ofe_mass.entry(key.basis_ofe_id.clone()).or_default();
        *ofe_mass = checked_surface_liquid_add(*ofe_mass, amount.mass).ok_or_else(|| {
            contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &key.recipient_store_key,
                Some(key.source_parcel_id.clone()),
                "actual OFE mass aggregate arithmetic",
            )
        })?;
        let accumulated = actual_ofe_enthalpy
            .entry(key.basis_ofe_id.clone())
            .or_default();
        *accumulated =
            checked_surface_liquid_add(*accumulated, amount.enthalpy).ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &key.recipient_store_key,
                    Some(key.source_parcel_id.clone()),
                    "actual OFE enthalpy aggregate arithmetic",
                )
            })?;
    }
    Ok(ParcelArithmeticProjection {
        expected,
        actual,
        expected_source_mass,
        actual_source_mass,
        raw_source_mass,
        expected_ofe_mass,
        actual_ofe_mass,
        raw_ofe_mass,
        expected_ofe_enthalpy,
        actual_ofe_enthalpy,
        raw_ofe_enthalpy: replayed_ofe_enthalpy,
        expected_store_liquid: store_liquid,
        expected_continuations,
    })
}

fn compare_source_mass_projection(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    projection: &ParcelArithmeticProjection,
    disposition: ComparisonDisposition,
) -> Result<(), DirectSurfaceLiquidError> {
    let source_keys = projection
        .expected_source_mass
        .keys()
        .chain(projection.actual_source_mass.keys())
        .chain(projection.raw_source_mass.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for (ofe_id, source_parcel_id) in source_keys {
        let source = operands
            .source_parcels
            .iter()
            .find(|row| row.source_parcel_id == source_parcel_id)
            .ok_or(DirectSurfaceLiquidError::Closure(
                "source mass aggregate has no frozen source",
            ))?;
        let expected = projection
            .expected_source_mass
            .get(&(ofe_id.clone(), source_parcel_id.clone()))
            .copied()
            .unwrap_or_default();
        let actual = projection
            .actual_source_mass
            .get(&(ofe_id.clone(), source_parcel_id.clone()))
            .copied()
            .unwrap_or_default();
        compare_projected_value(
            actual,
            expected,
            DirectSurfaceLiquidClosureUnit::MassKgM2,
            disposition,
            operands.transaction_id,
            &configuration.owner_id,
            &source.origin_store_key,
            Some(source_parcel_id.clone()),
            "source parcel attributed mass join",
        )?;
        if let Some(raw) = projection
            .raw_source_mass
            .get(&(ofe_id, source_parcel_id.clone()))
            .copied()
        {
            compare_projected_value(
                expected,
                raw,
                DirectSurfaceLiquidClosureUnit::MassKgM2,
                disposition,
                operands.transaction_id,
                &configuration.owner_id,
                &source.origin_store_key,
                Some(source_parcel_id),
                "raw parent source mass equals attributed mass",
            )?;
        }
    }
    Ok(())
}

fn compare_ofe_mass_projection(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    projection: &ParcelArithmeticProjection,
    disposition: ComparisonDisposition,
) -> Result<(), DirectSurfaceLiquidError> {
    let ofe_ids = projection
        .expected_ofe_mass
        .keys()
        .chain(projection.actual_ofe_mass.keys())
        .chain(projection.raw_ofe_mass.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for ofe_id in ofe_ids {
        let expected = projection
            .expected_ofe_mass
            .get(&ofe_id)
            .copied()
            .unwrap_or_default();
        let actual = projection
            .actual_ofe_mass
            .get(&ofe_id)
            .copied()
            .unwrap_or_default();
        let raw = projection
            .raw_ofe_mass
            .get(&ofe_id)
            .copied()
            .unwrap_or_default();
        compare_ofe_value(
            actual,
            expected,
            DirectSurfaceLiquidClosureUnit::MassKgM2,
            disposition,
            operands.transaction_id,
            &configuration.owner_id,
            &ofe_id,
            "OFE attributed mass join",
        )?;
        let expected_raw = checked_surface_liquid_sum(
            operands
                .source_parcels
                .iter()
                .filter(|source| source.basis_ofe_id == ofe_id)
                .map(|source| {
                    projection
                        .expected_source_mass
                        .get(&(ofe_id.clone(), source.source_parcel_id.clone()))
                        .copied()
                        .unwrap_or_default()
                }),
        )
        .ok_or_else(|| {
            contextual_ofe_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &ofe_id,
                "raw parent OFE attribution arithmetic",
            )
        })?;
        compare_ofe_value(
            expected_raw,
            raw,
            DirectSurfaceLiquidClosureUnit::MassKgM2,
            disposition,
            operands.transaction_id,
            &configuration.owner_id,
            &ofe_id,
            "raw parent OFE mass equals attributed mass",
        )?;
    }
    Ok(())
}

// The comparison order is contract-significant: identity, dimensional value,
// exact parcel mass/enthalpy, source mass, OFE mass, then OFE enthalpy.
#[allow(clippy::too_many_lines)]
fn compare_parcel_projection(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    projection: &ParcelArithmeticProjection,
    disposition: ComparisonDisposition,
) -> Result<(), DirectSurfaceLiquidError> {
    let keys = projection
        .expected
        .keys()
        .chain(projection.actual.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for key in keys {
        if projection.expected.contains_key(&key) != projection.actual.contains_key(&key)
            && matches!(disposition, ComparisonDisposition::RequireClosure)
        {
            return Err(contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E010,
                operands.transaction_id,
                &configuration.owner_id,
                &key.recipient_store_key,
                Some(key.source_parcel_id.clone()),
                "parcel identity join",
            ));
        }
        let expected = projection.expected.get(&key).copied().unwrap_or_default();
        let actual = projection.actual.get(&key).copied().unwrap_or_default();
        let store_key = projection_key_store(&key);
        compare_projected_value(
            actual.mass,
            expected.mass,
            DirectSurfaceLiquidClosureUnit::MassKgM2,
            disposition,
            operands.transaction_id,
            &configuration.owner_id,
            store_key,
            Some(key.source_parcel_id.clone()),
            "parcel mass join",
        )?;
        if matches!(disposition, ComparisonDisposition::RequireClosure)
            && actual.mass.to_bits() != expected.mass.to_bits()
        {
            return Err(contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E010,
                operands.transaction_id,
                &configuration.owner_id,
                store_key,
                Some(key.source_parcel_id.clone()),
                "parcel exact mass authority join",
            ));
        }
        if matches!(disposition, ComparisonDisposition::RequireClosure)
            && !enthalpy_reconstruction::exact_q_match(actual.enthalpy, expected.enthalpy)
        {
            return Err(contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E010,
                operands.transaction_id,
                &configuration.owner_id,
                store_key,
                Some(key.source_parcel_id.clone()),
                "parcel exact enthalpy authority join",
            ));
        }
    }

    compare_source_mass_projection(configuration, operands, projection, disposition)?;
    compare_ofe_mass_projection(configuration, operands, projection, disposition)?;

    let ofe_ids = projection
        .expected_ofe_enthalpy
        .keys()
        .chain(projection.actual_ofe_enthalpy.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for ofe_id in ofe_ids {
        let expected = projection
            .expected_ofe_enthalpy
            .get(&ofe_id)
            .copied()
            .unwrap_or_default();
        let actual = projection
            .actual_ofe_enthalpy
            .get(&ofe_id)
            .copied()
            .unwrap_or_default();
        compare_ofe_value(
            actual,
            expected,
            DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
            disposition,
            operands.transaction_id,
            &configuration.owner_id,
            &ofe_id,
            "OFE parcel enthalpy join",
        )?;
        let raw = projection
            .raw_ofe_enthalpy
            .get(&ofe_id)
            .copied()
            .unwrap_or_default();
        compare_ofe_value(
            expected,
            raw,
            DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
            disposition,
            operands.transaction_id,
            &configuration.owner_id,
            &ofe_id,
            "raw OFE enthalpy equals post-mix attributed enthalpy",
        )?;
    }
    Ok(())
}

/// Scan every frozen arithmetic surface without evaluating finite closure equality.
///
/// This pass deliberately ignores identity and finite `Some(false)` comparisons so a
/// later arithmetic/domain failure cannot be hidden by an earlier closure mismatch.
#[allow(clippy::too_many_lines)]
pub(super) fn preflight_surface_liquid_closure_arithmetic(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<(), DirectSurfaceLiquidError> {
    for partition in &operands.partition_inputs {
        if !partition.effective_conductivity_m_s.is_finite()
            || partition.effective_conductivity_m_s <= 0.0
            || !partition.matric_potential_m.is_finite()
            || partition.matric_potential_m < 0.0
            || !partition.infiltration_storage_capacity_m.is_finite()
            || partition.infiltration_storage_capacity_m < 0.0
            || !partition.beginning_cumulative_supply_m.is_finite()
            || partition.beginning_cumulative_supply_m < 0.0
            || !partition.beginning_cumulative_infiltration_m.is_finite()
            || partition.beginning_cumulative_infiltration_m < 0.0
            || partition.beginning_cumulative_infiltration_m
                > partition.beginning_cumulative_supply_m
            || partition.beginning_cumulative_infiltration_m
                > partition.infiltration_storage_capacity_m
            || !(1..=48).contains(&partition.ending_next_interval_index)
        {
            return Err(contextual_ofe_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &partition.ofe_id,
                "frozen partition-input domain",
            ));
        }
    }
    for parcel in &operands.source_parcels {
        let support_valid = parcel.start_s.is_finite()
            && parcel.end_s.is_finite()
            && parcel.start_s >= 0.0
            && parcel.start_s < parcel.end_s
            && parcel.end_s <= INTERVAL_S;
        let amount_valid = parcel.mass_kg_m2_basis_ofe_ground.is_finite()
            && parcel.mass_kg_m2_basis_ofe_ground >= 0.0
            && parcel.enthalpy_j_m2_basis_ofe_ground.is_finite();
        let temperature_valid = parcel.temperature_k.is_finite()
            && (200.0..=350.0).contains(&parcel.temperature_k)
            && parcel.specific_liquid_enthalpy_j_kg.is_finite();
        let expected_specific =
            checked_surface_liquid_sub(parcel.temperature_k, REFERENCE_TEMPERATURE_K)
                .and_then(|delta| checked_surface_liquid_mul(LIQUID_HEAT_CAPACITY_J_KG_K, delta));
        let expected_enthalpy = checked_surface_liquid_mul(
            parcel.mass_kg_m2_basis_ofe_ground,
            parcel.specific_liquid_enthalpy_j_kg,
        );
        if !support_valid
            || !amount_valid
            || !temperature_valid
            || expected_specific.is_none()
            || expected_specific.map(f64::to_bits)
                != Some(parcel.specific_liquid_enthalpy_j_kg.to_bits())
            || expected_enthalpy.map(f64::to_bits)
                != Some(parcel.enthalpy_j_m2_basis_ofe_ground.to_bits())
        {
            return Err(contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &parcel.origin_store_key,
                Some(parcel.source_parcel_id.clone()),
                "frozen source parcel domain",
            ));
        }
    }
    for receipt in receipts {
        let support_valid = receipt.start_s.is_finite()
            && receipt.end_s.is_finite()
            && receipt.start_s >= 0.0
            && receipt.start_s < receipt.end_s
            && receipt.end_s <= INTERVAL_S;
        let amount_valid = receipt.mass_kg_m2_basis_ofe_ground.is_finite()
            && receipt.mass_kg_m2_basis_ofe_ground >= 0.0
            && receipt.enthalpy_j_m2_basis_ofe_ground.is_finite();
        let temperature_valid =
            receipt.temperature_k.is_finite() && (200.0..=350.0).contains(&receipt.temperature_k);
        if !support_valid || !amount_valid || !temperature_valid {
            return Err(contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &receipt.recipient_store_key,
                Some(receipt.parcel_id.clone()),
                "parcel receipt domain",
            ));
        }
    }
    for row in &operands.stores {
        let key = &row.store_key;
        let Some(projected) = project_store_arithmetic(row) else {
            return Err(contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                key,
                None,
                "store closure arithmetic is nonfinite or underflowed",
            ));
        };
        if let Some(working) = resource
            .working_state()
            .records
            .iter()
            .find(|candidate| candidate.key == *key)
        {
            for (actual, expected, detail) in [
                (
                    working.liquid_kg_m2_tile,
                    projected.pre_ingress_liquid_kg_m2_tile,
                    "resource store comparison arithmetic",
                ),
                (
                    row.ending_liquid_kg_m2_tile,
                    projected.ending_liquid_kg_m2_tile,
                    "ending store comparison arithmetic",
                ),
            ] {
                compare_projected_value(
                    actual,
                    expected,
                    DirectSurfaceLiquidClosureUnit::MassKgM2,
                    ComparisonDisposition::ArithmeticPreflight,
                    operands.transaction_id,
                    &configuration.owner_id,
                    key,
                    None,
                    detail,
                )?;
            }
        }
    }

    if !arithmetic_preflight::partition_inputs_are_projectable(configuration, operands) {
        project_actual_receipt_arithmetic(configuration, operands, receipts)?;
        raw_parent_reconstruction::reconstruct_raw_parent_mass(configuration, operands)?;
        return Ok(());
    }

    let projection = project_parcel_arithmetic(configuration, operands, receipts)?;
    compare_parcel_projection(
        configuration,
        operands,
        &projection,
        ComparisonDisposition::ArithmeticPreflight,
    )
}

fn validate_store_equations(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    operands: &DirectSurfaceLiquidClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    if operands.stores.len() != configuration.records.len() {
        return Err(DirectSurfaceLiquidError::Closure(
            "independent store operand cardinality",
        ));
    }
    for ((row, configured), working) in operands
        .stores
        .iter()
        .zip(&configuration.records)
        .zip(&resource.working_state().records)
    {
        if row.store_key != configured.key
            || row.tile_fraction.to_bits() != configured.tile_fraction.to_bits()
        {
            return Err(contextual_closure_failure(
                operands.transaction_id,
                &configured.key,
                None,
                "independent store operand identity",
            ));
        }
        let projected = project_store_arithmetic(row).ok_or_else(|| {
            contextual_closure_arithmetic_failure(
                operands.transaction_id,
                &configured.key,
                None,
                "store closure arithmetic is nonfinite or underflowed",
            )
        })?;
        require_close_mass(
            working.liquid_kg_m2_tile,
            projected.pre_ingress_liquid_kg_m2_tile,
            operands.transaction_id,
            &configuration.owner_id,
            &configured.key,
            None,
            "resource state does not reconstruct from W0, F, C, and overflow",
        )?;
        require_close_mass(
            row.ending_liquid_kg_m2_tile,
            projected.ending_liquid_kg_m2_tile,
            operands.transaction_id,
            &configuration.owner_id,
            &configured.key,
            None,
            "W1 does not reconstruct from W0, F, C, overflow, and retained excess",
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn validate_parcel_joins(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    ending: &DirectSurfaceLiquidOwnedState,
) -> Result<(), DirectSurfaceLiquidError> {
    let projection = project_parcel_arithmetic(configuration, operands, receipts)?;
    let mut seen_receipt_ids = BTreeSet::new();
    for receipt in receipts {
        let source = operands
            .source_parcels
            .iter()
            .find(|parcel| parcel.source_parcel_id == receipt.source_parcel_id)
            .ok_or_else(|| {
                contextual_closure_failure(
                    operands.transaction_id,
                    &receipt.origin_store_key,
                    Some(receipt.parcel_id.clone()),
                    "receipt source parcel has no frozen operand",
                )
            })?;
        if receipt.transaction_id != operands.transaction_id
            || receipt.origin_store_key != source.origin_store_key
            || !seen_receipt_ids.insert(receipt.parcel_id.clone())
        {
            return Err(contextual_closure_failure(
                operands.transaction_id,
                &receipt.origin_store_key,
                Some(receipt.parcel_id.clone()),
                "duplicate or wrong-identity parcel receipt",
            ));
        }
        let binding = configuration
            .ofe_bindings
            .iter()
            .find(|binding| binding.ofe_id == receipt.basis_ofe_id)
            .ok_or_else(|| {
                contextual_closure_failure(
                    operands.transaction_id,
                    &receipt.origin_store_key,
                    Some(receipt.parcel_id.clone()),
                    "receipt OFE binding missing",
                )
            })?;
        let route_record = configuration
            .records
            .iter()
            .find(|record| record.key.ofe_id == receipt.basis_ofe_id)
            .ok_or_else(|| {
                contextual_closure_failure(
                    operands.transaction_id,
                    &receipt.origin_store_key,
                    Some(receipt.parcel_id.clone()),
                    "receipt route record missing",
                )
            })?;
        validate_receipt_recipient(configuration, binding, route_record, receipt).map_err(
            |_| {
                contextual_closure_failure(
                    operands.transaction_id,
                    &receipt.origin_store_key,
                    Some(receipt.parcel_id.clone()),
                    "wrong typed parcel recipient",
                )
            },
        )?;
        validate_receipt_enthalpy(&configuration.owner_id, receipt)?;
    }

    compare_parcel_projection(
        configuration,
        operands,
        &projection,
        ComparisonDisposition::RequireClosure,
    )?;
    validate_projected_ending_state(configuration, operands, ending, &projection)
}

fn validate_projected_ending_state(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    ending: &DirectSurfaceLiquidOwnedState,
    projection: &ParcelArithmeticProjection,
) -> Result<(), DirectSurfaceLiquidError> {
    if ending.owner_id != configuration.owner_id
        || ending.configuration_sha256 != configuration.configuration_sha256
    {
        return Err(ending_aggregate_failure(
            operands.transaction_id,
            &configuration.owner_id,
            "projected ending-state owner/configuration",
        ));
    }

    let actual_store_keys = ending
        .records
        .iter()
        .map(|row| row.key.clone())
        .collect::<Vec<_>>();
    let expected_store_keys = configuration
        .records
        .iter()
        .map(|row| row.key.clone())
        .collect::<Vec<_>>();
    if let Some(offender) =
        first_membership_aware_mismatch(&actual_store_keys, &expected_store_keys)
    {
        return Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E010,
            operands.transaction_id,
            &configuration.owner_id,
            &offender,
            None,
            "projected ending store membership/order",
        ));
    }
    for (actual, configured) in ending.records.iter().zip(&configuration.records) {
        let expected_liquid = projection
            .expected_store_liquid
            .get(&configured.key)
            .copied()
            .ok_or(DirectSurfaceLiquidError::Closure(
                "projected ending store absent from independent projection",
            ))?;
        if actual.liquid_kg_m2_tile.to_bits() != expected_liquid.to_bits()
            || actual.last_accepted_transaction_id != Some(operands.transaction_id)
        {
            return Err(contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E010,
                operands.transaction_id,
                &configuration.owner_id,
                &configured.key,
                None,
                "projected ending store join",
            ));
        }
    }

    let actual_continuation_ids = ending
        .continuations
        .iter()
        .map(|row| row.ofe_id.clone())
        .collect::<Vec<_>>();
    if let Some(offender) =
        first_membership_aware_mismatch(&actual_continuation_ids, &configuration.ofe_topology)
    {
        return Err(contextual_ofe_comparison_failure(
            DirectSurfaceLiquidErrorCode::E010,
            operands.transaction_id,
            &configuration.owner_id,
            &offender,
            "projected ending continuation membership/order",
        ));
    }
    for (actual, expected_ofe) in ending.continuations.iter().zip(&configuration.ofe_topology) {
        let expected = projection.expected_continuations.get(expected_ofe).ok_or(
            DirectSurfaceLiquidError::Closure(
                "projected ending continuation absent from independent projection",
            ),
        )?;
        if actual.day_index != expected.day_index
            || actual.next_interval_index != expected.next_interval_index
            || actual.cumulative_supply_m.to_bits() != expected.cumulative_supply_m.to_bits()
            || actual.cumulative_infiltration_m.to_bits()
                != expected.cumulative_infiltration_m.to_bits()
            || actual.last_accepted_transaction_id != Some(expected.transaction_id)
        {
            return Err(contextual_ofe_comparison_failure(
                DirectSurfaceLiquidErrorCode::E010,
                operands.transaction_id,
                &configuration.owner_id,
                expected_ofe,
                "projected ending continuation join",
            ));
        }
    }

    validate_projected_ending_digest(configuration, operands, ending)
}

fn validate_receipt_recipient(
    configuration: &DirectSurfaceLiquidConfiguration,
    binding: &super::surface_liquid_owner::DirectSurfaceLiquidOfeBinding,
    route_record: &DirectSurfaceLiquidConfigurationRecord,
    receipt: &DirectSurfaceLiquidParcelReceipt,
) -> Result<(), DirectSurfaceLiquidError> {
    let valid = match (&receipt.disposition, &receipt.recipient) {
        (
            DirectSurfaceLiquidReceiptDisposition::Infiltration,
            DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
                ofe_id,
                production_lane_index,
                production_lane_id,
                ordered_soil_layer_ids,
                soil_thermal_layer_id,
            },
        ) => {
            ofe_id == &binding.ofe_id
                && production_lane_index == &binding.production_lane_index
                && production_lane_id == &binding.production_lane_id
                && ordered_soil_layer_ids == &binding.ordered_soil_layer_ids
                && soil_thermal_layer_id == &binding.infiltration_soil_thermal_layer_id
        }
        (
            DirectSurfaceLiquidReceiptDisposition::RetainedSurface,
            DirectSurfaceLiquidReceiptRecipient::SurfaceStore { store_key },
        ) => store_key == &receipt.recipient_store_key,
        (
            DirectSurfaceLiquidReceiptDisposition::RoutedRunoff,
            DirectSurfaceLiquidReceiptRecipient::RoutedOfe {
                source_ofe_id,
                destination_ofe_id,
                destination_store_key,
            },
        ) => match route_destination(configuration, route_record) {
            Ok((expected_ofe, expected_record)) => {
                source_ofe_id == &receipt.basis_ofe_id
                    && destination_ofe_id == &expected_ofe
                    && destination_store_key == &expected_record.key
            }
            Err(_) => false,
        },
        (
            DirectSurfaceLiquidReceiptDisposition::OutletRunoff,
            DirectSurfaceLiquidReceiptRecipient::Outlet { ofe_id },
        ) => {
            ofe_id == &receipt.basis_ofe_id
                && route_record.runon_destination_ofe_id.is_none()
                && route_record.runon_destination_tile_id.is_none()
        }
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(DirectSurfaceLiquidError::Closure(
            "wrong typed parcel recipient",
        ))
    }
}

fn route_destination<'a>(
    configuration: &'a DirectSurfaceLiquidConfiguration,
    route_record: &DirectSurfaceLiquidConfigurationRecord,
) -> Result<(OfeId, &'a DirectSurfaceLiquidConfigurationRecord), DirectSurfaceLiquidError> {
    let destination_ofe =
        route_record
            .runon_destination_ofe_id
            .clone()
            .ok_or(DirectSurfaceLiquidError::Closure(
                "routed receipt on terminal OFE",
            ))?;
    let destination_tile = route_record.runon_destination_tile_id.as_ref().ok_or(
        DirectSurfaceLiquidError::Closure("routed receipt missing destination tile"),
    )?;
    let destination = configuration
        .records
        .iter()
        .find(|record| {
            record.key.ofe_id == destination_ofe && record.key.tile_id == *destination_tile
        })
        .ok_or(DirectSurfaceLiquidError::Closure(
            "routed receipt destination missing",
        ))?;
    Ok((destination_ofe, destination))
}
