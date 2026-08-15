//! Independent surface-liquid store, parcel, enthalpy, and routing closure.

#![allow(clippy::missing_errors_doc)]

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::OfeId;

use super::surface_liquid_ingress::{
    DirectIngressAmount, DirectSurfaceLiquidIngressInput, DirectSurfaceLiquidParcelKind,
    DirectSurfaceLiquidParcelReceipt, DirectSurfaceLiquidReceiptDisposition,
    DirectSurfaceLiquidReceiptRecipient, DirectTileGroundIngress,
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

/// Frozen, non-residual operands consumed by the independent closure validator.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSurfaceLiquidClosureOperands {
    transaction_id: TransactionId,
    stores: Vec<DirectSurfaceLiquidStoreClosureOperands>,
    source_parcels: Vec<DirectSurfaceLiquidParcelClosureOperands>,
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
    pub(super) fn add_downstream_projection_poison_for_test(
        &mut self,
        destination_ofe_id: OfeId,
        enthalpy: f64,
    ) -> String {
        let mut projected = self.source_parcels.first().expect("source parcel").clone();
        projected.basis_ofe_id = destination_ofe_id;
        projected.mass_kg_m2_basis_ofe_ground = 0.0;
        projected.enthalpy_j_m2_basis_ofe_ground = enthalpy;
        let source_parcel_id = projected.source_parcel_id.clone();
        self.source_parcels.push(projected);
        source_parcel_id
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
    source_parcel_id: String,
    basis_ofe_id: OfeId,
}

#[derive(Clone, Copy, Debug, Default)]
struct AmountPair {
    mass: f64,
    enthalpy: f64,
}

struct ParcelArithmeticProjection {
    expected: BTreeMap<ParcelJoinKey, AmountPair>,
    actual: BTreeMap<ParcelJoinKey, AmountPair>,
    expected_ofe_enthalpy: BTreeMap<OfeId, f64>,
    actual_ofe_enthalpy: BTreeMap<OfeId, f64>,
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

fn projection_store_key<'a>(
    configuration: &'a DirectSurfaceLiquidConfiguration,
    basis_ofe_id: &OfeId,
) -> Result<&'a DirectSurfaceLiquidStoreKey, DirectSurfaceLiquidError> {
    configuration
        .records
        .iter()
        .find(|row| &row.key.ofe_id == basis_ofe_id)
        .map(|row| &row.key)
        .ok_or(DirectSurfaceLiquidError::Closure(
            "projection basis OFE has no canonical store",
        ))
}

fn projection_key_store<'a>(
    configuration: &'a DirectSurfaceLiquidConfiguration,
    operands: &'a DirectSurfaceLiquidClosureOperands,
    receipts: &'a [DirectSurfaceLiquidParcelReceipt],
    key: &ParcelJoinKey,
) -> Result<&'a DirectSurfaceLiquidStoreKey, DirectSurfaceLiquidError> {
    if let Some(parcel) = operands
        .source_parcels
        .iter()
        .find(|parcel| parcel.source_parcel_id == key.source_parcel_id)
    {
        if parcel.basis_ofe_id == key.basis_ofe_id {
            return Ok(&parcel.origin_store_key);
        }
        if let Some(source) = configuration
            .records
            .iter()
            .find(|record| record.key == parcel.origin_store_key)
        {
            if let (Some(destination_ofe), Some(destination_tile)) = (
                source.runon_destination_ofe_id.as_ref(),
                source.runon_destination_tile_id.as_ref(),
            ) {
                if destination_ofe == &key.basis_ofe_id {
                    if let Some(destination) = configuration.records.iter().find(|record| {
                        &record.key.ofe_id == destination_ofe
                            && &record.key.tile_id == destination_tile
                    }) {
                        return Ok(&destination.key);
                    }
                }
            }
        }
    }
    if let Some(receipt) = receipts.iter().find(|receipt| {
        receipt.source_parcel_id == key.source_parcel_id && receipt.basis_ofe_id == key.basis_ofe_id
    }) {
        return Ok(&receipt.recipient_store_key);
    }
    projection_store_key(configuration, &key.basis_ofe_id)
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

pub(super) fn capture_and_validate_surface_liquid_closure(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    ending: &DirectSurfaceLiquidOwnedState,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<DirectSurfaceLiquidClosureOperands, DirectSurfaceLiquidError> {
    (|| {
        let operands = capture_operands(configuration, resource, input, ending, receipts)?;
        validate_surface_liquid_closure_operands(configuration, resource, &operands, receipts)?;
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
    Ok(DirectSurfaceLiquidClosureOperands {
        transaction_id: input.transaction_id,
        stores,
        source_parcels,
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
            DirectTileGroundIngress::CoveredCanopyRelease { release, .. } => {
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
                    capture_amount(configured, kind, amount, &mut result, input.transaction_id)?;
                }
            }
        }
    }
    for overflow in resource.condensation_overflow() {
        result.push(capture_overflow(input.transaction_id, overflow)?);
    }
    Ok(result)
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
        source_parcel_id: format!(
            "local:{:?}:{:?}:{kind:?}",
            configured.key.ofe_id, configured.key.tile_id
        ),
        origin_store_key: configured.key.clone(),
        basis_ofe_id: configured.key.ofe_id.clone(),
        kind,
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
        source_parcel_id: format!(
            "condensation:{}:{:?}:{:?}",
            transaction_id.0, overflow.store_key.ofe_id, overflow.store_key.tile_id
        ),
        origin_store_key: overflow.store_key.clone(),
        basis_ofe_id: overflow.store_key.ofe_id.clone(),
        kind: DirectSurfaceLiquidParcelKind::CondensationOverflow,
        mass_kg_m2_basis_ofe_ground: overflow.amount_kg_m2_ofe_ground,
        enthalpy_j_m2_basis_ofe_ground: enthalpy,
    })
}

pub(super) fn validate_surface_liquid_closure_operands(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<(), DirectSurfaceLiquidError> {
    if operands.transaction_id != resource.transaction_id() {
        return Err(DirectSurfaceLiquidError::Closure(
            "independent closure transaction mismatch",
        ));
    }
    validate_store_equations(configuration, resource, operands)?;
    validate_parcel_joins(configuration, operands, receipts)
}

#[allow(clippy::too_many_lines)]
fn project_parcel_arithmetic(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<ParcelArithmeticProjection, DirectSurfaceLiquidError> {
    let mut expected = BTreeMap::<ParcelJoinKey, AmountPair>::new();
    for parcel in &operands.source_parcels {
        if !parcel.mass_kg_m2_basis_ofe_ground.is_finite()
            || parcel.mass_kg_m2_basis_ofe_ground < 0.0
            || !parcel.enthalpy_j_m2_basis_ofe_ground.is_finite()
        {
            return Err(contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &parcel.origin_store_key,
                Some(parcel.source_parcel_id.clone()),
                "source parcel arithmetic domain",
            ));
        }
        expected
            .entry(ParcelJoinKey {
                source_parcel_id: parcel.source_parcel_id.clone(),
                basis_ofe_id: parcel.basis_ofe_id.clone(),
            })
            .or_default()
            .checked_add(
                parcel.mass_kg_m2_basis_ofe_ground,
                parcel.enthalpy_j_m2_basis_ofe_ground,
            )
            .ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &parcel.origin_store_key,
                    Some(parcel.source_parcel_id.clone()),
                    "source parcel aggregate arithmetic",
                )
            })?;
    }
    let mut actual = BTreeMap::<ParcelJoinKey, AmountPair>::new();
    for receipt in receipts {
        if let Err(error) = validate_receipt_enthalpy(&configuration.owner_id, receipt) {
            if error.code() == DirectSurfaceLiquidErrorCode::E003 {
                return Err(error);
            }
        }
        actual
            .entry(ParcelJoinKey {
                source_parcel_id: receipt.source_parcel_id.clone(),
                basis_ofe_id: receipt.basis_ofe_id.clone(),
            })
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
                    &receipt.origin_store_key,
                    Some(receipt.parcel_id.clone()),
                    "receipt aggregate arithmetic",
                )
            })?;
        if receipt.disposition != DirectSurfaceLiquidReceiptDisposition::RoutedRunoff {
            continue;
        }
        let Some(source) = configuration
            .records
            .iter()
            .find(|row| row.key.ofe_id == receipt.basis_ofe_id)
        else {
            continue;
        };
        let Some(destination_ofe) = source.runon_destination_ofe_id.as_ref() else {
            continue;
        };
        let Some(destination_tile) = source.runon_destination_tile_id.as_ref() else {
            continue;
        };
        let Some(destination) = configuration
            .records
            .iter()
            .find(|row| &row.key.ofe_id == destination_ofe && &row.key.tile_id == destination_tile)
        else {
            continue;
        };
        let ratio = checked_surface_liquid_div(source.ofe_area_m2, destination.ofe_area_m2);
        let routed = ratio
            .and_then(|value| {
                checked_surface_liquid_mul(receipt.mass_kg_m2_basis_ofe_ground, value).zip(
                    checked_surface_liquid_mul(receipt.enthalpy_j_m2_basis_ofe_ground, value),
                )
            })
            .ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &destination.key,
                    Some(receipt.parcel_id.clone()),
                    "route area/amount arithmetic",
                )
            })?;
        expected
            .entry(ParcelJoinKey {
                source_parcel_id: receipt.source_parcel_id.clone(),
                basis_ofe_id: destination_ofe.clone(),
            })
            .or_default()
            .checked_add(routed.0, routed.1)
            .ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &destination.key,
                    Some(receipt.parcel_id.clone()),
                    "routed parcel aggregate arithmetic",
                )
            })?;
    }

    let mut expected_ofe_enthalpy = BTreeMap::<OfeId, f64>::new();
    for (key, amount) in &expected {
        let store_key = projection_key_store(configuration, operands, receipts, key)?;
        let accumulated = expected_ofe_enthalpy
            .entry(key.basis_ofe_id.clone())
            .or_default();
        *accumulated =
            checked_surface_liquid_add(*accumulated, amount.enthalpy).ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    store_key,
                    Some(key.source_parcel_id.clone()),
                    "expected OFE enthalpy aggregate arithmetic",
                )
            })?;
    }
    let mut actual_ofe_enthalpy = BTreeMap::<OfeId, f64>::new();
    for (key, amount) in &actual {
        let accumulated = actual_ofe_enthalpy
            .entry(key.basis_ofe_id.clone())
            .or_default();
        let store_key = projection_key_store(configuration, operands, receipts, key)?;
        *accumulated =
            checked_surface_liquid_add(*accumulated, amount.enthalpy).ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    store_key,
                    Some(key.source_parcel_id.clone()),
                    "actual OFE enthalpy aggregate arithmetic",
                )
            })?;
    }
    Ok(ParcelArithmeticProjection {
        expected,
        actual,
        expected_ofe_enthalpy,
        actual_ofe_enthalpy,
    })
}

fn compare_parcel_projection(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
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
        let expected = projection.expected.get(&key).copied().unwrap_or_default();
        let actual = projection.actual.get(&key).copied().unwrap_or_default();
        let store_key = projection_key_store(configuration, operands, receipts, &key)?;
        for (actual_value, expected_value, unit, detail) in [
            (
                actual.mass,
                expected.mass,
                DirectSurfaceLiquidClosureUnit::MassKgM2,
                "parcel mass join",
            ),
            (
                actual.enthalpy,
                expected.enthalpy,
                DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
                "parcel enthalpy join",
            ),
        ] {
            compare_projected_value(
                actual_value,
                expected_value,
                unit,
                disposition,
                operands.transaction_id,
                &configuration.owner_id,
                store_key,
                Some(key.source_parcel_id.clone()),
                detail,
            )?;
        }
    }

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
        compare_projected_value(
            actual,
            expected,
            DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
            disposition,
            operands.transaction_id,
            &configuration.owner_id,
            projection_store_key(configuration, &ofe_id)?,
            None,
            "OFE parcel enthalpy join",
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

    let projection = project_parcel_arithmetic(configuration, operands, receipts)?;
    compare_parcel_projection(
        configuration,
        operands,
        receipts,
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
        receipts,
        &projection,
        ComparisonDisposition::RequireClosure,
    )
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

fn validate_receipt_enthalpy(
    owner_id: &ResourceOwnerId,
    receipt: &DirectSurfaceLiquidParcelReceipt,
) -> Result<(), DirectSurfaceLiquidError> {
    if !receipt.mass_kg_m2_basis_ofe_ground.is_finite()
        || receipt.mass_kg_m2_basis_ofe_ground < 0.0
        || !receipt.enthalpy_j_m2_basis_ofe_ground.is_finite()
        || !receipt.temperature_k.is_finite()
    {
        return Err(DirectSurfaceLiquidError::Closure(
            "nonfinite or negative parcel receipt",
        ));
    }
    if receipt.mass_kg_m2_basis_ofe_ground == 0.0 {
        if receipt.enthalpy_j_m2_basis_ofe_ground.to_bits() != 0.0_f64.to_bits() {
            return Err(DirectSurfaceLiquidError::Closure(
                "zero-mass parcel carries enthalpy",
            ));
        }
        return Ok(());
    }
    let expected = checked_surface_liquid_sub(receipt.temperature_k, 273.15)
        .and_then(|delta| checked_surface_liquid_mul(4_218.0, delta))
        .and_then(|specific| {
            checked_surface_liquid_mul(receipt.mass_kg_m2_basis_ofe_ground, specific)
        })
        .ok_or_else(|| {
            contextual_closure_arithmetic_failure(
                receipt.transaction_id,
                &receipt.origin_store_key,
                Some(receipt.parcel_id.clone()),
                "parcel temperature/enthalpy arithmetic is nonfinite or underflowed",
            )
        })?;
    require_close_enthalpy(
        receipt.enthalpy_j_m2_basis_ofe_ground,
        expected,
        receipt.transaction_id,
        owner_id,
        &receipt.origin_store_key,
        Some(receipt.parcel_id.clone()),
        "parcel temperature/enthalpy join",
    )
}

fn water_key_matches_record(
    key: &openwepp_land_surface_energy::GroundWaterKey,
    record: &DirectSurfaceLiquidConfigurationRecord,
) -> bool {
    key.ofe_id == record.key.ofe_id
        && key.source_tile_id.as_ref() == Some(&record.key.tile_id)
        && key.surface_id.as_ref() == Some(&record.key.surface_id)
        && key.surface_class == Some(record.key.surface_class)
        && key.source_type == record.key.source_type
        && key.source_id == record.key.source_id
}

fn require_close_mass(
    actual: f64,
    expected: f64,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
    store_key: &DirectSurfaceLiquidStoreKey,
    parcel_id: Option<String>,
    detail: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    match checked_surface_liquid_close(actual, expected, DirectSurfaceLiquidClosureUnit::MassKgM2) {
        Some(true) => Ok(()),
        Some(false) => Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E010,
            transaction_id,
            owner_id,
            store_key,
            parcel_id,
            detail,
        )),
        None => Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E003,
            transaction_id,
            owner_id,
            store_key,
            parcel_id,
            detail,
        )),
    }
}

fn require_close_enthalpy(
    actual: f64,
    expected: f64,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
    store_key: &DirectSurfaceLiquidStoreKey,
    parcel_id: Option<String>,
    detail: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    match checked_surface_liquid_close(
        actual,
        expected,
        DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
    ) {
        Some(true) => Ok(()),
        Some(false) => Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E010,
            transaction_id,
            owner_id,
            store_key,
            parcel_id,
            detail,
        )),
        None => Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E003,
            transaction_id,
            owner_id,
            store_key,
            parcel_id,
            detail,
        )),
    }
}
