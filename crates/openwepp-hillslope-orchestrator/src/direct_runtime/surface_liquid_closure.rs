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
use openwepp_land_surface_energy::{LitterPhaseCapacitySpillV1, OfeId};

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

const REPRESENTATIONAL_CREDIT_ABSOLUTE_KG_M2: f64 = 1.0e-14;
const REPRESENTATIONAL_CREDIT_EPSILON_MULTIPLIER: f64 = 64.0;

include!("surface_liquid_closure_projection_helpers.rs");

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
    interval_s: f64,
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

    #[cfg(test)]
    pub(crate) fn canonical_private_projection_v1(
        &self,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        fn push_bytes(out: &mut Vec<u8>, value: &[u8]) {
            out.extend_from_slice(&(value.len() as u64).to_be_bytes());
            out.extend_from_slice(value);
        }
        fn push_json<T: serde::Serialize>(
            out: &mut Vec<u8>,
            value: &T,
        ) -> Result<(), DirectSurfaceLiquidError> {
            push_bytes(
                out,
                &serde_json::to_vec(value)
                    .map_err(|_| DirectSurfaceLiquidError::Schema("closure private projection"))?,
            );
            Ok(())
        }
        fn push_f64(out: &mut Vec<u8>, value: f64) {
            out.extend_from_slice(&value.to_bits().to_be_bytes());
        }

        let mut out = b"OPENWEPP_SURFACE_LIQUID_CLOSURE_PRIVATE_PROJECTION_V1\0".to_vec();
        out.extend_from_slice(&self.transaction_id.0.to_be_bytes());
        push_f64(&mut out, self.interval_s);
        out.extend_from_slice(&(self.stores.len() as u64).to_be_bytes());
        for store in &self.stores {
            push_json(&mut out, &store.store_key)?;
            for value in [
                store.tile_fraction,
                store.beginning_liquid_kg_m2_tile,
                store.finalized_withdrawal_kg_m2_ofe_ground,
                store.condensation_credit_kg_m2_ofe_ground,
                store.condensation_overflow_kg_m2_ofe_ground,
                store.retained_excess_kg_m2_ofe_ground,
                store.ending_liquid_kg_m2_tile,
            ] {
                push_f64(&mut out, value);
            }
        }
        out.extend_from_slice(&(self.source_parcels.len() as u64).to_be_bytes());
        for parcel in &self.source_parcels {
            push_bytes(&mut out, parcel.source_parcel_id.as_bytes());
            push_json(&mut out, &parcel.origin_store_key)?;
            push_json(&mut out, &parcel.basis_ofe_id)?;
            push_json(&mut out, &parcel.kind)?;
            for value in [
                parcel.start_s,
                parcel.end_s,
                parcel.temperature_k,
                parcel.specific_liquid_enthalpy_j_kg,
                parcel.mass_kg_m2_basis_ofe_ground,
                parcel.enthalpy_j_m2_basis_ofe_ground,
            ] {
                push_f64(&mut out, value);
            }
        }
        out.extend_from_slice(&(self.partition_inputs.len() as u64).to_be_bytes());
        for partition in &self.partition_inputs {
            push_json(&mut out, &partition.ofe_id)?;
            for value in [
                partition.effective_conductivity_m_s,
                partition.matric_potential_m,
                partition.infiltration_storage_capacity_m,
                partition.beginning_cumulative_supply_m,
                partition.beginning_cumulative_infiltration_m,
            ] {
                push_f64(&mut out, value);
            }
            out.extend_from_slice(&(partition.ending_day_index as u64).to_be_bytes());
            out.push(partition.ending_next_interval_index);
        }
        Ok(out)
    }

    pub(super) fn store_operands_match(&self, expected: &Self) -> bool {
        self.stores == expected.stores
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

    /// Exact no-work custody for a represented-snow surface. The atmospheric
    /// ground boundary is the snowpack, so the snow-free surface and WB14
    /// partition have no source parcels or partition operands to execute.
    pub(super) fn try_new_stage3_covered_native_inactive(
        transaction_id: TransactionId,
        configuration: &DirectSurfaceLiquidConfiguration,
        resource: &DirectSurfaceLiquidResourceCandidate,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        if resource.transaction_id() != transaction_id
            || resource.beginning_state() != resource.working_state()
            || resource
                .finalized_uses()
                .iter()
                .any(|row| row.amount_kg_m2_stand_ground.to_bits() != 0.0_f64.to_bits())
            || !resource.condensation_credits().is_empty()
            || !resource.condensation_overflow().is_empty()
        {
            return Err(DirectSurfaceLiquidError::Closure(
                "Stage3CoveredNative inactive surface resource custody",
            ));
        }
        let stores = configuration
            .records
            .iter()
            .map(|configured| {
                let beginning = resource
                    .beginning_state()
                    .records
                    .iter()
                    .find(|row| row.key == configured.key)
                    .ok_or(DirectSurfaceLiquidError::Identity(
                        "Stage3CoveredNative beginning surface store",
                    ))?;
                Ok(DirectSurfaceLiquidStoreClosureOperands {
                    store_key: configured.key.clone(),
                    tile_fraction: configured.tile_fraction,
                    beginning_liquid_kg_m2_tile: beginning.liquid_kg_m2_tile,
                    finalized_withdrawal_kg_m2_ofe_ground: 0.0,
                    condensation_credit_kg_m2_ofe_ground: 0.0,
                    condensation_overflow_kg_m2_ofe_ground: 0.0,
                    retained_excess_kg_m2_ofe_ground: 0.0,
                    ending_liquid_kg_m2_tile: beginning.liquid_kg_m2_tile,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            transaction_id,
            interval_s: 0.0,
            stores,
            source_parcels: Vec::new(),
            partition_inputs: Vec::new(),
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
    expected_temperature_k: BTreeMap<ParcelJoinKey, f64>,
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

#[allow(clippy::too_many_arguments)]
pub(super) fn capture_and_validate_surface_liquid_closure_with_phase_capacity_spills(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    ending: &DirectSurfaceLiquidOwnedState,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    wb14_beginnings: &BTreeMap<OfeId, (f64, f64)>,
    phase_capacity_spills: &[LitterPhaseCapacitySpillV1],
) -> Result<DirectSurfaceLiquidClosureOperands, DirectSurfaceLiquidError> {
    (|| {
        let operands = capture_operands(
            configuration,
            resource,
            input,
            ending,
            receipts,
            wb14_beginnings,
            phase_capacity_spills,
        )?;
        validate_surface_liquid_closure_operands_with_input_and_phase_capacity_spills(
            configuration,
            resource,
            input,
            &operands,
            receipts,
            ending,
            phase_capacity_spills,
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
    wb14_beginnings: &BTreeMap<OfeId, (f64, f64)>,
    phase_capacity_spills: &[LitterPhaseCapacitySpillV1],
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
    let source_parcels =
        capture_source_parcels(configuration, resource, input, phase_capacity_spills)?;
    let partition_inputs =
        configuration
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
                let (beginning_cumulative_supply_m, beginning_cumulative_infiltration_m) =
                    wb14_beginnings.get(ofe_id).copied().ok_or(
                        DirectSurfaceLiquidError::Identity("closure WB14 beginning missing"),
                    )?;
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
        interval_s: input.interval_s,
        stores,
        source_parcels,
        partition_inputs,
    })
}

fn capture_source_parcels(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    phase_capacity_spills: &[LitterPhaseCapacitySpillV1],
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
        result.push(capture_overflow(
            input.transaction_id,
            input.interval_s,
            overflow,
        )?);
    }
    for spill in phase_capacity_spills {
        result.push(capture_litter_phase_capacity_spill(
            configuration,
            input.transaction_id,
            input.interval_s,
            spill,
        )?);
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
    interval_s: f64,
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
        end_s: interval_s,
        temperature_k: overflow.temperature_k,
        specific_liquid_enthalpy_j_kg: overflow.specific_liquid_enthalpy_j_kg,
        mass_kg_m2_basis_ofe_ground: overflow.amount_kg_m2_ofe_ground,
        enthalpy_j_m2_basis_ofe_ground: enthalpy,
    })
}

fn capture_litter_phase_capacity_spill(
    configuration: &DirectSurfaceLiquidConfiguration,
    transaction_id: TransactionId,
    interval_s: f64,
    spill: &LitterPhaseCapacitySpillV1,
) -> Result<DirectSurfaceLiquidParcelClosureOperands, DirectSurfaceLiquidError> {
    let configured = configuration
        .records
        .iter()
        .find(|record| record.key.ofe_id == spill.ofe_id && record.key.tile_id == spill.tile_id)
        .ok_or(DirectSurfaceLiquidError::Identity(
            "phase-spill closure surface key",
        ))?;
    let mass = checked_surface_liquid_mul(configured.tile_fraction, spill.spill_liquid_kg_m2_tile)
        .ok_or_else(|| {
            contextual_closure_arithmetic_failure(
                transaction_id,
                &configured.key,
                None,
                "phase-spill closure mass area conversion",
            )
        })?;
    let enthalpy = checked_surface_liquid_mul(mass, spill.spill_specific_sensible_enthalpy_j_kg)
        .ok_or_else(|| {
            contextual_closure_arithmetic_failure(
                transaction_id,
                &configured.key,
                None,
                "phase-spill closure enthalpy area conversion",
            )
        })?;
    Ok(DirectSurfaceLiquidParcelClosureOperands {
        source_parcel_id: canonical_surface_liquid_source_id(
            CanonicalSurfaceLiquidSource::LitterPhaseOverflow {
                transaction_id,
                store_key: &configured.key,
                phase_receipt_sha256: spill.phase_receipt_sha256.as_str(),
            },
        ),
        origin_store_key: configured.key.clone(),
        basis_ofe_id: configured.key.ofe_id.clone(),
        kind: DirectSurfaceLiquidParcelKind::LitterPhaseOverflow,
        start_s: 0.0,
        end_s: interval_s,
        temperature_k: spill.raw_ending.temperature_k,
        specific_liquid_enthalpy_j_kg: spill.spill_specific_sensible_enthalpy_j_kg,
        mass_kg_m2_basis_ofe_ground: mass,
        enthalpy_j_m2_basis_ofe_ground: enthalpy,
    })
}

include!("surface_liquid_closure_terminal_validation.rs");
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
            && parcel.end_s <= operands.interval_s;
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
            && receipt.end_s <= operands.interval_s;
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
        let expected_temperature_k = projection
            .expected_temperature_k
            .get(&receipt_join_key(&configuration.owner_id, receipt))
            .copied()
            .ok_or_else(|| {
                contextual_closure_failure(
                    operands.transaction_id,
                    &receipt.origin_store_key,
                    Some(receipt.parcel_id.clone()),
                    "receipt temperature has no independent partition lineage",
                )
            })?;
        validate_receipt_enthalpy(&configuration.owner_id, receipt, expected_temperature_k)?;
    }

    compare_parcel_projection(
        configuration,
        operands,
        &projection,
        ComparisonDisposition::RequireClosure,
    )?;
    validate_projected_ending_state(configuration, operands, ending, &projection)
}

include!("surface_liquid_closure_receipt_routing.rs");

#[cfg(test)]
mod full_capacity_projection_tests {
    use super::*;

    #[test]
    fn independent_projection_reconstructs_exact_full_capacity_endpoint() {
        let capacity_tile = 6.0_f64;
        let tile_fraction = 0.38_f64;
        let available_tile = capacity_tile;
        let available_ofe = tile_fraction * available_tile;
        assert_eq!(
            (available_ofe / tile_fraction).to_bits(),
            capacity_tile.to_bits() + 1,
        );
        assert_eq!(
            independently_project_ending_store(
                0.0,
                capacity_tile,
                tile_fraction,
                available_tile,
                available_ofe,
                available_ofe,
            )
            .expect("independent full-capacity projection")
            .to_bits(),
            capacity_tile.to_bits(),
        );
    }

    #[test]
    fn independent_projection_rejects_substituted_capacity_basis() {
        let capacity_tile = 6.0_f64;
        let tile_fraction = 0.38_f64;
        let available_tile = capacity_tile;
        let available_ofe = tile_fraction * available_tile;
        assert!(
            independently_project_ending_store(
                0.0,
                capacity_tile,
                tile_fraction,
                f64::from_bits(available_tile.to_bits() - 1),
                available_ofe,
                available_ofe,
            )
            .is_none()
        );
        assert!(
            independently_project_ending_store(
                0.0,
                capacity_tile,
                tile_fraction,
                available_tile,
                available_ofe,
                f64::from_bits(available_ofe.to_bits() + 1),
            )
            .is_none()
        );
    }
}
