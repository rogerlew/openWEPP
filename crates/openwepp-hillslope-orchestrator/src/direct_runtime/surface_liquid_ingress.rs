//! Timed snow-free ingress, WB14 continuation, retention, and routing.

#![allow(clippy::missing_errors_doc)]

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use openwepp_land_surface_energy::{OfeId, ParcelId, Sha256Digest, SurfaceId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::runoff::{DirectWb14ContinuationIntervalInputs, advance_wb14_continuation_interval};
use super::surface_liquid_closure::{
    DirectSurfaceLiquidClosureOperands, capture_and_validate_surface_liquid_closure,
};
use super::surface_liquid_owner::{
    DirectGroundIngressMode, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidConfigurationRecord, DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode,
    DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidResourceCandidate, DirectSurfaceLiquidStoreKey, checked_surface_liquid_add,
    checked_surface_liquid_div, checked_surface_liquid_mul, checked_surface_liquid_sub,
    checked_surface_liquid_sum,
};
use super::surface_liquid_wb14::{
    DirectWb14ImmutableIdentityV1, DirectWb14ParentFinalizationV1, DirectWb14ParentIntervalV1,
    DirectWb14PersistentCursorV1, wb14_parent_authority_v1,
};

pub(super) const INTERVAL_S: f64 = 1_800.0;
pub(super) const WATER_DENSITY_KG_M3: f64 = 1_000.0;
pub(super) const LIQUID_HEAT_CAPACITY_J_KG_K: f64 = 4_218.0;
pub(super) const REFERENCE_TEMPERATURE_K: f64 = 273.15;
#[cfg(test)]
const MASS_ABSOLUTE_TOLERANCE_KG_M2: f64 = 1.0e-14;
#[cfg(test)]
const SCALE_MULTIPLIER: f64 = 64.0;

fn production_binding_failure(
    transaction_id: TransactionId,
    ofe_id: Option<OfeId>,
    detail: impl Into<String>,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E008,
        DirectSurfaceLiquidPhase::IngressCandidate,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            ofe_id,
            ..DirectSurfaceLiquidErrorContext::default()
        },
        super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

fn candidate_failure(
    transaction_id: TransactionId,
    mut context: DirectSurfaceLiquidErrorContext,
    detail: impl Into<String>,
) -> DirectSurfaceLiquidError {
    context.transaction_id = Some(transaction_id);
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E009,
        DirectSurfaceLiquidPhase::IngressCandidate,
        context,
        super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

fn ingress_arithmetic_failure(
    transaction_id: TransactionId,
    key: &DirectSurfaceLiquidStoreKey,
    parcel_id: Option<String>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E003,
        DirectSurfaceLiquidPhase::IngressCandidate,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: None,
            ofe_id: Some(key.ofe_id.clone()),
            tile_id: Some(key.tile_id.clone()),
            surface_id: Some(key.surface_id.clone()),
            source_id: Some(key.source_id.clone()),
            parcel_id,
        },
        super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectSurfaceLiquidParcelKind {
    RawPrecipitation,
    CanopyThroughfall,
    CanopyInitialDrainage,
    CanopySecondDrainage,
    CanopyStemflow,
    CondensationOverflow,
    UpstreamRunon,
    TerminalReceiver,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectIngressAmount {
    pub mass_kg_m2_tile_ground: f64,
    pub temperature_k: f64,
    pub specific_liquid_enthalpy_j_kg: f64,
    pub start_s: f64,
    pub end_s: f64,
}

impl DirectIngressAmount {
    fn validate(
        &self,
        require_full_interval: bool,
        interval_s: f64,
    ) -> Result<(), DirectSurfaceLiquidError> {
        require_nonnegative(self.mass_kg_m2_tile_ground, "ingress mass")?;
        require_temperature(self.temperature_k)?;
        let expected = liquid_specific_enthalpy(self.temperature_k);
        if self.specific_liquid_enthalpy_j_kg.to_bits() != expected.to_bits() {
            return Err(DirectSurfaceLiquidError::Closure(
                "ingress temperature/enthalpy mismatch",
            ));
        }
        if !self.start_s.is_finite()
            || !self.end_s.is_finite()
            || self.start_s < 0.0
            || self.end_s <= self.start_s
            || self.end_s > interval_s
            || (require_full_interval
                && (self.start_s.to_bits() != 0.0_f64.to_bits()
                    || self.end_s.to_bits() != interval_s.to_bits()))
        {
            return Err(DirectSurfaceLiquidError::Domain(
                "invalid ingress interval support",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectCanopyLiquidRelease {
    pub throughfall: DirectIngressAmount,
    pub initial_drainage: DirectIngressAmount,
    pub second_drainage: DirectIngressAmount,
    pub stemflow: DirectIngressAmount,
}

/// One already-validated LSE forcing parcel admitted at an open ground
/// boundary. Unlike `DirectIngressAmount`, this form deliberately retains the
/// upstream identity needed to distinguish routed runon from precipitation.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectOpenLiquidIngressParcel {
    pub kind: DirectSurfaceLiquidParcelKind,
    pub parcel_id: ParcelId,
    pub source_owner_id: ResourceOwnerId,
    pub source_ofe_id: OfeId,
    pub source_tile_id: TileId,
    pub destination_ofe_id: OfeId,
    pub destination_tile_id: TileId,
    pub accepted_source_state_sha256: Sha256Digest,
    pub amount: DirectIngressAmount,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "ingress_mode", deny_unknown_fields)]
pub enum DirectTileGroundIngress {
    OpenRawPrecipitation {
        ofe_id: OfeId,
        tile_id: TileId,
        surface_id: SurfaceId,
        raw_precipitation: DirectIngressAmount,
    },
    OpenLiquidParcels {
        ofe_id: OfeId,
        tile_id: TileId,
        surface_id: SurfaceId,
        parcels: Vec<DirectOpenLiquidIngressParcel>,
    },
    CoveredCanopyRelease {
        ofe_id: OfeId,
        tile_id: TileId,
        surface_id: SurfaceId,
        release: DirectCanopyLiquidRelease,
    },
    CoveredCanopyReleaseAndRunon {
        ofe_id: OfeId,
        tile_id: TileId,
        surface_id: SurfaceId,
        release: DirectCanopyLiquidRelease,
        runon_parcels: Vec<DirectOpenLiquidIngressParcel>,
    },
}

impl DirectTileGroundIngress {
    pub(super) fn identity(&self) -> (&OfeId, &TileId, &SurfaceId) {
        match self {
            Self::OpenRawPrecipitation {
                ofe_id,
                tile_id,
                surface_id,
                ..
            }
            | Self::OpenLiquidParcels {
                ofe_id,
                tile_id,
                surface_id,
                ..
            }
            | Self::CoveredCanopyRelease {
                ofe_id,
                tile_id,
                surface_id,
                ..
            }
            | Self::CoveredCanopyReleaseAndRunon {
                ofe_id,
                tile_id,
                surface_id,
                ..
            } => (ofe_id, tile_id, surface_id),
        }
    }

    fn mode(&self) -> DirectGroundIngressMode {
        match self {
            Self::OpenRawPrecipitation { .. } | Self::OpenLiquidParcels { .. } => {
                DirectGroundIngressMode::OpenRawPrecipitation
            }
            Self::CoveredCanopyRelease { .. } | Self::CoveredCanopyReleaseAndRunon { .. } => {
                DirectGroundIngressMode::CoveredCanopyRelease
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectOfeWb14Parameters {
    pub ofe_id: OfeId,
    pub effective_conductivity_m_s: f64,
    pub matric_potential_m: f64,
    pub infiltration_storage_capacity_m: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidIngressInput {
    pub transaction_id: TransactionId,
    pub day_index: usize,
    pub interval_index: u8,
    pub interval_s: f64,
    pub tile_ingress: Vec<DirectTileGroundIngress>,
    pub wb14_parameters: Vec<DirectOfeWb14Parameters>,
}

include!("surface_liquid_ingress_coordinator.rs");
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectSurfaceLiquidReceiptDisposition {
    Infiltration,
    RetainedSurface,
    RoutedRunoff,
    OutletRunoff,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "recipient_kind", deny_unknown_fields)]
pub enum DirectSurfaceLiquidReceiptRecipient {
    SoilInfiltration {
        ofe_id: OfeId,
        production_lane_index: usize,
        production_lane_id: u32,
        ordered_soil_layer_ids: Vec<SoilLayerId>,
        soil_thermal_layer_id: SoilLayerId,
    },
    SurfaceStore {
        store_key: DirectSurfaceLiquidStoreKey,
    },
    RoutedOfe {
        source_ofe_id: OfeId,
        destination_ofe_id: OfeId,
        destination_store_key: DirectSurfaceLiquidStoreKey,
    },
    Outlet {
        ofe_id: OfeId,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidParcelReceipt {
    pub parcel_id: String,
    pub source_parcel_id: String,
    pub transaction_id: TransactionId,
    pub origin_store_key: DirectSurfaceLiquidStoreKey,
    pub recipient_store_key: DirectSurfaceLiquidStoreKey,
    pub recipient: DirectSurfaceLiquidReceiptRecipient,
    pub basis_ofe_id: OfeId,
    pub kind: DirectSurfaceLiquidParcelKind,
    pub disposition: DirectSurfaceLiquidReceiptDisposition,
    pub start_s: f64,
    pub end_s: f64,
    pub mass_kg_m2_basis_ofe_ground: f64,
    pub temperature_k: f64,
    pub enthalpy_j_m2_basis_ofe_ground: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidIngressLedger {
    pub ofe_id: OfeId,
    pub ingress_mass_kg_m2_ofe_ground: f64,
    pub ingress_enthalpy_j_m2_ofe_ground: f64,
    pub infiltration_mass_kg_m2_ofe_ground: f64,
    pub infiltration_enthalpy_j_m2_ofe_ground: f64,
    pub retained_mass_kg_m2_ofe_ground: f64,
    pub retained_enthalpy_j_m2_ofe_ground: f64,
    pub runoff_mass_kg_m2_ofe_ground: f64,
    pub runoff_enthalpy_j_m2_ofe_ground: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSurfaceLiquidIngressCandidate {
    transaction_id: TransactionId,
    beginning_state: DirectSurfaceLiquidOwnedState,
    ending_state: DirectSurfaceLiquidOwnedState,
    receipts: Vec<DirectSurfaceLiquidParcelReceipt>,
    ledgers: Vec<DirectSurfaceLiquidIngressLedger>,
    wb14_calls_by_ofe: BTreeMap<OfeId, u8>,
    closure_operands: DirectSurfaceLiquidClosureOperands,
    open_ingress_parcels: Vec<DirectOpenLiquidIngressParcel>,
    parent_child_mode: bool,
    finalize_parent_interval: bool,
    input_parent_working_state: Option<DirectWb14ParentWorkingState>,
    parent_working_state: Option<DirectWb14ParentWorkingState>,
    wb14_child_receipt_set_sha256: Sha256Digest,
    wb14_parent_receipt_set_sha256: Option<Sha256Digest>,
    wb14_child_replay_bytes: Vec<u8>,
    wb14_parent_replay_bytes: Option<Vec<u8>>,
}

impl DirectSurfaceLiquidIngressCandidate {
    fn closure_ending_state(
        &self,
    ) -> Result<DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidError> {
        let mut state = self.ending_state.clone();
        if let Some(parent) = &self.parent_working_state {
            state = parent.candidate_state.clone();
        }
        Ok(state)
    }

    #[must_use]
    pub const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub const fn beginning_state(&self) -> &DirectSurfaceLiquidOwnedState {
        &self.beginning_state
    }

    #[must_use]
    pub const fn ending_state(&self) -> &DirectSurfaceLiquidOwnedState {
        &self.ending_state
    }

    #[must_use]
    pub fn wb14_child_replay_bytes(&self) -> &[u8] {
        &self.wb14_child_replay_bytes
    }

    #[must_use]
    pub fn wb14_parent_replay_bytes(&self) -> Option<&[u8]> {
        self.wb14_parent_replay_bytes.as_deref()
    }

    #[must_use]
    pub fn receipts(&self) -> &[DirectSurfaceLiquidParcelReceipt] {
        &self.receipts
    }

    #[must_use]
    pub fn ledgers(&self) -> &[DirectSurfaceLiquidIngressLedger] {
        &self.ledgers
    }

    #[must_use]
    pub const fn wb14_calls_by_ofe(&self) -> &BTreeMap<OfeId, u8> {
        &self.wb14_calls_by_ofe
    }

    #[must_use]
    pub const fn closure_operands(&self) -> &DirectSurfaceLiquidClosureOperands {
        &self.closure_operands
    }

    #[must_use]
    pub(crate) const fn parent_working_state(&self) -> Option<&DirectWb14ParentWorkingState> {
        self.parent_working_state.as_ref()
    }

    #[must_use]
    pub(crate) const fn advances_persistent_parent_interval(&self) -> bool {
        !self.parent_child_mode || self.finalize_parent_interval
    }

    #[must_use]
    pub(crate) const fn wb14_child_receipt_set_sha256(&self) -> &Sha256Digest {
        &self.wb14_child_receipt_set_sha256
    }

    #[must_use]
    pub(crate) const fn wb14_parent_receipt_set_sha256(&self) -> Option<&Sha256Digest> {
        self.wb14_parent_receipt_set_sha256.as_ref()
    }

    /// Exact accepted LSE forcing lineage retained alongside the partition
    /// receipts; this prevents routed runon from collapsing into a local-rain
    /// scalar after admission.
    #[must_use]
    pub fn open_ingress_parcels(&self) -> &[DirectOpenLiquidIngressParcel] {
        &self.open_ingress_parcels
    }

    pub fn validate(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
        resource: &DirectSurfaceLiquidResourceCandidate,
        input: &DirectSurfaceLiquidIngressInput,
    ) -> Result<(), DirectSurfaceLiquidError> {
        preflight_surface_liquid_ingress_public_identities(configuration, resource, input)
            .map_err(|error| {
                let code = error.code();
                error.complete_context(
                    code,
                    DirectSurfaceLiquidPhase::IngressCandidate,
                    DirectSurfaceLiquidErrorContext {
                        transaction_id: Some(input.transaction_id),
                        owner_id: Some(configuration.owner_id.clone()),
                        ..DirectSurfaceLiquidErrorContext::default()
                    },
                    Some(resource.beginning_state().state_sha256.clone()),
                    self.ending_state.recomputed_sha256().ok(),
                )
            })?;
        if let Err(error) =
            super::surface_liquid_closure::preflight_surface_liquid_closure_arithmetic(
                configuration,
                resource,
                &self.closure_operands,
                &self.receipts,
            )
        {
            if error.code() == DirectSurfaceLiquidErrorCode::E003 {
                return Err(error.complete_context(
                    DirectSurfaceLiquidErrorCode::E003,
                    DirectSurfaceLiquidPhase::IndependentClosure,
                    DirectSurfaceLiquidErrorContext {
                        transaction_id: Some(input.transaction_id),
                        owner_id: Some(configuration.owner_id.clone()),
                        ..DirectSurfaceLiquidErrorContext::default()
                    },
                    Some(resource.beginning_state().state_sha256.clone()),
                    self.ending_state.recomputed_sha256().ok(),
                ));
            }
        }
        let expected = execute_surface_liquid_ingress_inner(
            configuration,
            resource,
            input,
            self.parent_child_mode,
            self.finalize_parent_interval,
            self.input_parent_working_state.as_ref(),
            None,
        )
        .map_err(|error| {
            let code = error.code();
            error.complete_context(
                code,
                DirectSurfaceLiquidPhase::IngressCandidate,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                Some(resource.beginning_state().state_sha256.clone()),
                self.ending_state.recomputed_sha256().ok(),
            )
        })?;
        if let Some(context) = self.producer_mismatch_context(&expected, configuration, input) {
            return Err(DirectSurfaceLiquidError::canonical_failure(
                DirectSurfaceLiquidErrorCode::E009,
                DirectSurfaceLiquidPhase::IngressCandidate,
                context,
                super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                    beginning_owner_sha256: Some(resource.beginning_state().state_sha256.clone()),
                    attempted_owner_sha256: self.ending_state.recomputed_sha256().ok(),
                },
                "ingress candidate does not reconstruct from immutable inputs",
            ));
        }
        let closure_ending_state = self.closure_ending_state()?;
        super::surface_liquid_closure::validate_surface_liquid_closure_operands_with_input(
            configuration,
            resource,
            input,
            &self.closure_operands,
            &self.receipts,
            &closure_ending_state,
        )
        .map_err(|error| {
            let code = error.code();
            error.complete_context(
                code,
                DirectSurfaceLiquidPhase::IndependentClosure,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                Some(resource.beginning_state().state_sha256.clone()),
                self.ending_state.recomputed_sha256().ok(),
            )
        })?;
        Ok(())
    }

    fn producer_mismatch_context(
        &self,
        expected: &Self,
        configuration: &DirectSurfaceLiquidConfiguration,
        input: &DirectSurfaceLiquidIngressInput,
    ) -> Option<DirectSurfaceLiquidErrorContext> {
        let base = || DirectSurfaceLiquidErrorContext {
            transaction_id: Some(input.transaction_id),
            owner_id: Some(configuration.owner_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        };
        if self.transaction_id != expected.transaction_id {
            return Some(base());
        }
        if self.beginning_state != expected.beginning_state {
            return Some(state_mismatch_context(
                &self.beginning_state,
                &expected.beginning_state,
                base(),
            ));
        }
        if self.ending_state != expected.ending_state {
            return Some(state_mismatch_context(
                &self.ending_state,
                &expected.ending_state,
                base(),
            ));
        }
        if let Some((key, parcel_id)) = self
            .closure_operands
            .first_source_identity_mismatch(&expected.closure_operands)
        {
            return Some(DirectSurfaceLiquidErrorContext {
                transaction_id: Some(input.transaction_id),
                owner_id: Some(configuration.owner_id.clone()),
                ofe_id: Some(key.ofe_id),
                tile_id: Some(key.tile_id),
                surface_id: Some(key.surface_id),
                source_id: Some(key.source_id),
                parcel_id: Some(parcel_id),
            });
        }
        if let Some(ofe_id) = self
            .closure_operands
            .first_partition_input_mismatch(&expected.closure_operands)
        {
            return Some(DirectSurfaceLiquidErrorContext {
                transaction_id: Some(input.transaction_id),
                owner_id: Some(configuration.owner_id.clone()),
                ofe_id: Some(ofe_id),
                ..DirectSurfaceLiquidErrorContext::default()
            });
        }
        if self.receipts != expected.receipts {
            let receipt =
                first_identity_aware_mismatch(&self.receipts, &expected.receipts, |row| {
                    row.parcel_id.clone()
                });
            return Some(
                receipt.map_or_else(base, |row| DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ofe_id: Some(row.recipient_store_key.ofe_id.clone()),
                    tile_id: Some(row.recipient_store_key.tile_id.clone()),
                    surface_id: Some(row.recipient_store_key.surface_id.clone()),
                    source_id: Some(row.recipient_store_key.source_id.clone()),
                    parcel_id: Some(row.parcel_id.clone()),
                }),
            );
        }
        if self.open_ingress_parcels != expected.open_ingress_parcels {
            return Some(base());
        }
        if self.ledgers != expected.ledgers {
            let ledger = first_identity_aware_mismatch(&self.ledgers, &expected.ledgers, |row| {
                row.ofe_id.clone()
            });
            return Some(
                ledger.map_or_else(base, |row| DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ofe_id: Some(row.ofe_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                }),
            );
        }
        if self.wb14_calls_by_ofe != expected.wb14_calls_by_ofe {
            let ofe_id =
                first_map_identity_mismatch(&self.wb14_calls_by_ofe, &expected.wb14_calls_by_ofe);
            return Some(
                ofe_id.map_or_else(base, |ofe_id| DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ofe_id: Some(ofe_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                }),
            );
        }
        None
    }
}

fn first_identity_aware_mismatch<'a, T: PartialEq, K: Ord, F: Fn(&T) -> K>(
    actual: &'a [T],
    expected: &'a [T],
    identity: F,
) -> Option<&'a T> {
    if actual.len() < expected.len() {
        let actual_identities = actual.iter().map(&identity).collect::<BTreeSet<_>>();
        if let Some(missing) = expected
            .iter()
            .find(|row| !actual_identities.contains(&identity(row)))
        {
            return Some(missing);
        }
    }
    (0..actual.len().max(expected.len()))
        .find(|&index| actual.get(index) != expected.get(index))
        .and_then(|index| actual.get(index).or_else(|| expected.get(index)))
}

fn first_map_identity_mismatch<'a>(
    actual: &'a BTreeMap<OfeId, u8>,
    expected: &'a BTreeMap<OfeId, u8>,
) -> Option<&'a OfeId> {
    if actual.len() < expected.len() {
        if let Some(missing) = expected.keys().find(|key| !actual.contains_key(*key)) {
            return Some(missing);
        }
    }
    actual
        .iter()
        .find(|(key, value)| expected.get(*key) != Some(*value))
        .map(|(key, _)| key)
        .or_else(|| expected.keys().find(|key| !actual.contains_key(*key)))
}

fn state_mismatch_context(
    actual: &DirectSurfaceLiquidOwnedState,
    expected: &DirectSurfaceLiquidOwnedState,
    mut context: DirectSurfaceLiquidErrorContext,
) -> DirectSurfaceLiquidErrorContext {
    if let Some(row) =
        first_identity_aware_mismatch(&actual.records, &expected.records, |row| row.key.clone())
    {
        context.ofe_id = Some(row.key.ofe_id.clone());
        context.tile_id = Some(row.key.tile_id.clone());
        context.surface_id = Some(row.key.surface_id.clone());
        context.source_id = Some(row.key.source_id.clone());
        return context;
    }
    if let Some(row) =
        first_identity_aware_mismatch(&actual.continuations, &expected.continuations, |row| {
            row.ofe_id.clone()
        })
    {
        context.ofe_id = Some(row.ofe_id.clone());
    }
    context
}

#[derive(Clone, Debug)]
struct TimedParcel {
    parcel_id: String,
    origin_store_key: DirectSurfaceLiquidStoreKey,
    recipient_store_key: DirectSurfaceLiquidStoreKey,
    basis_ofe_id: OfeId,
    kind: DirectSurfaceLiquidParcelKind,
    start_s: f64,
    end_s: f64,
    mass_kg_m2_basis_ofe_ground: f64,
    enthalpy_j_m2_basis_ofe_ground: f64,
}

#[derive(Clone, Copy)]
pub(super) enum CanonicalSurfaceLiquidSource<'a> {
    Local {
        store_key: &'a DirectSurfaceLiquidStoreKey,
        kind: DirectSurfaceLiquidParcelKind,
    },
    Condensation {
        transaction_id: TransactionId,
        store_key: &'a DirectSurfaceLiquidStoreKey,
    },
}

pub(super) fn canonical_surface_liquid_source_id(
    source: CanonicalSurfaceLiquidSource<'_>,
) -> String {
    match source {
        CanonicalSurfaceLiquidSource::Local { store_key, kind } => format!(
            "local:{:?}:{:?}:{kind:?}",
            store_key.ofe_id, store_key.tile_id
        ),
        CanonicalSurfaceLiquidSource::Condensation {
            transaction_id,
            store_key,
        } => format!(
            "condensation:{}:{:?}:{:?}",
            transaction_id.0, store_key.ofe_id, store_key.tile_id
        ),
    }
}

#[derive(Clone, Copy)]
pub(super) struct CanonicalParcelOrderKey<'a> {
    pub start_s: f64,
    pub end_s: f64,
    pub origin_store_key: &'a DirectSurfaceLiquidStoreKey,
    pub kind: DirectSurfaceLiquidParcelKind,
    pub source_parcel_id: &'a str,
}

pub(super) fn canonical_parcel_order(
    left: CanonicalParcelOrderKey<'_>,
    right: CanonicalParcelOrderKey<'_>,
) -> std::cmp::Ordering {
    left.start_s
        .total_cmp(&right.start_s)
        .then_with(|| left.end_s.total_cmp(&right.end_s))
        .then_with(|| left.origin_store_key.cmp(right.origin_store_key))
        .then_with(|| left.kind.cmp(&right.kind))
        .then_with(|| left.source_parcel_id.cmp(right.source_parcel_id))
}

fn parcel_temperature_k(
    parcel: &TimedParcel,
    transaction_id: TransactionId,
    key: &DirectSurfaceLiquidStoreKey,
) -> Result<f64, DirectSurfaceLiquidError> {
    let specific_enthalpy = checked_surface_liquid_div(
        parcel.enthalpy_j_m2_basis_ofe_ground,
        parcel.mass_kg_m2_basis_ofe_ground,
    )
    .and_then(|value| checked_surface_liquid_div(value, LIQUID_HEAT_CAPACITY_J_KG_K))
    .and_then(|value| checked_surface_liquid_add(REFERENCE_TEMPERATURE_K, value))
    .ok_or_else(|| {
        ingress_arithmetic_failure(
            transaction_id,
            key,
            Some(parcel.parcel_id.clone()),
            "parcel temperature reconstruction is nonfinite or underflowed",
        )
    })?;
    Ok(specific_enthalpy)
}

#[derive(Default)]
struct OfeAdvance {
    receipts: Vec<DirectSurfaceLiquidParcelReceipt>,
    runoff: Vec<TimedParcel>,
    ledger: Option<DirectSurfaceLiquidIngressLedger>,
    cumulative_supply_m: f64,
    cumulative_infiltration_m: f64,
    wb14_transitions: Vec<DirectWb14ContinuationIntervalInputs>,
}

fn pending_routed_queue_sha256(pending: &BTreeMap<OfeId, Vec<TimedParcel>>) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"openwepp-surface-liquid-routed-queue-v1\0");
    for (ofe_id, parcels) in pending {
        digest.update((ofe_id.as_str().len() as u64).to_be_bytes());
        digest.update(ofe_id.as_str().as_bytes());
        digest.update((parcels.len() as u64).to_be_bytes());
        for parcel in parcels {
            for identity in [
                parcel.parcel_id.as_str(),
                parcel.origin_store_key.ofe_id.as_str(),
                parcel.origin_store_key.tile_id.as_str(),
                parcel.recipient_store_key.ofe_id.as_str(),
                parcel.recipient_store_key.tile_id.as_str(),
                parcel.basis_ofe_id.as_str(),
            ] {
                digest.update((identity.len() as u64).to_be_bytes());
                digest.update(identity.as_bytes());
            }
            digest.update((parcel.kind as u8).to_be_bytes());
            digest.update(parcel.start_s.to_bits().to_be_bytes());
            digest.update(parcel.end_s.to_bits().to_be_bytes());
            digest.update(parcel.mass_kg_m2_basis_ofe_ground.to_bits().to_be_bytes());
            digest.update(
                parcel
                    .enthalpy_j_m2_basis_ofe_ground
                    .to_bits()
                    .to_be_bytes(),
            );
        }
    }
    digest.finalize().into()
}

/// Execute the admitted post-resource ingress transaction against a cloned owner state.
pub fn execute_surface_liquid_ingress(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidError> {
    execute_surface_liquid_ingress_inner(configuration, resource, input, false, true, None, None)
        .map_err(|error| {
            let code = error.code();
            let phase = match code {
                DirectSurfaceLiquidErrorCode::E010 => DirectSurfaceLiquidPhase::IndependentClosure,
                _ => DirectSurfaceLiquidPhase::IngressCandidate,
            };
            error.complete_context(
                code,
                phase,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                Some(resource.beginning_state().state_sha256.clone()),
                resource.working_state().recomputed_sha256().ok(),
            )
        })
}

/// Execute one complete-owner child of an enclosing WB14 parent interval.
/// The persistent day/interval cursor advances only for the child that closes
/// the coupled parent; cumulative Green-Ampt state remains available to every
/// accepted child candidate.
pub(crate) fn execute_surface_liquid_ingress_with_parent_finalization(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    finalize_parent_interval: bool,
) -> Result<DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidError> {
    execute_surface_liquid_ingress_with_parent_state(
        configuration,
        resource,
        input,
        None,
        finalize_parent_interval,
    )
}

pub(crate) fn execute_surface_liquid_ingress_with_parent_state(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    parent_working_state: Option<&DirectWb14ParentWorkingState>,
    finalize_parent_interval: bool,
) -> Result<DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidError> {
    execute_surface_liquid_ingress_with_parent_state_and_coupled_binding(
        configuration,
        resource,
        input,
        parent_working_state,
        finalize_parent_interval,
        None,
    )
}

pub(crate) fn execute_surface_liquid_ingress_with_parent_state_and_coupled_binding(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    parent_working_state: Option<&DirectWb14ParentWorkingState>,
    finalize_parent_interval: bool,
    coupled_binding: Option<DirectWb14CoupledChildBindingV1>,
) -> Result<DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidError> {
    // Existing callers may use the V11 endpoint for an independently accepted
    // short slab. With no predecessor parent state and an immediate finalize,
    // that remains an ordinary ingress transaction rather than inventing a
    // partial 1,800-second WB14 parent.
    let parent_child_mode = parent_working_state.is_some()
        || !finalize_parent_interval
        || input.interval_s.to_bits() == INTERVAL_S.to_bits();
    execute_surface_liquid_ingress_inner(
        configuration,
        resource,
        input,
        parent_child_mode,
        finalize_parent_interval,
        parent_working_state,
        coupled_binding,
    )
    .map_err(|error| {
        let code = error.code();
        let phase = match code {
            DirectSurfaceLiquidErrorCode::E010 => DirectSurfaceLiquidPhase::IndependentClosure,
            _ => DirectSurfaceLiquidPhase::IngressCandidate,
        };
        error.complete_context(
            code,
            phase,
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(input.transaction_id),
                owner_id: Some(configuration.owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(resource.beginning_state().state_sha256.clone()),
            resource.working_state().recomputed_sha256().ok(),
        )
    })
}

#[allow(clippy::too_many_lines)]
fn execute_surface_liquid_ingress_inner(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    parent_child_mode: bool,
    finalize_parent_interval: bool,
    parent_working_state: Option<&DirectWb14ParentWorkingState>,
    coupled_binding: Option<DirectWb14CoupledChildBindingV1>,
) -> Result<DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidError> {
    preflight_surface_liquid_ingress_public_identities(configuration, resource, input)?;
    configuration.validate()?;
    resource.beginning_state().validate(configuration)?;
    validate_resource_working_state_domains(configuration, resource)?;
    if !input.interval_s.is_finite() || input.interval_s <= 0.0 {
        return Err(DirectSurfaceLiquidError::Domain(
            "nonfinite ingress interval",
        ));
    }
    preflight_tile_ingress_domains(configuration, input)?;
    preflight_parameter_domains(&input.wb14_parameters)?;
    validate_tile_ingress_cardinality(configuration, input)?;
    let parameters = validate_parameter_cardinality_and_order(
        configuration,
        input.transaction_id,
        &input.wb14_parameters,
    )?;
    validate_cadence(
        resource.beginning_state(),
        input,
        parent_child_mode,
        finalize_parent_interval,
    )?;
    let mut pending = validate_and_build_local_ingress(configuration, resource, input)?;
    let mut ending = resource.working_state().clone();
    let (
        derived_parent_support_start_ns,
        derived_parent_support_end_ns,
        wb14_configuration_sha256,
        production_lane_ids,
    ) = wb14_parent_binding(configuration, input)?;
    let (parent_support_start_ns, parent_support_end_ns) = match coupled_binding {
        Some(binding) => (
            i128::try_from(binding.parent_support_start_ns).map_err(|_| {
                DirectSurfaceLiquidError::Identity("coupled parent support overflow")
            })?,
            i128::try_from(binding.parent_support_end_ns).map_err(|_| {
                DirectSurfaceLiquidError::Identity("coupled parent support overflow")
            })?,
        ),
        None => (
            derived_parent_support_start_ns,
            derived_parent_support_end_ns,
        ),
    };
    let wb14_model_definition_sha256 = format!("{:x}", Sha256::digest(WB14_MODEL_DEFINITION));
    if let Some(parent) = parent_working_state {
        if parent.schema != WB14_PARENT_WORKING_SCHEMA
            || parent.parent_day_index != input.day_index
            || parent.parent_interval_index != input.interval_index
            || parent.parent_support_start_ns != parent_support_start_ns
            || parent.parent_support_end_ns != parent_support_end_ns
            || parent.surface_liquid_configuration_sha256 != configuration.configuration_sha256
            || parent.wb14_configuration_sha256 != wb14_configuration_sha256
            || parent.wb14_model_definition_sha256 != wb14_model_definition_sha256
            || parent.production_lane_ids != production_lane_ids
            || parent.parameters != input.wb14_parameters
            || !parent.accepted_duration_s.is_finite()
            || parent.accepted_duration_s <= 0.0
            || parent.accepted_duration_s + input.interval_s > INTERVAL_S
            || parent.candidate_state.continuations.len() != ending.continuations.len()
            || resource.beginning_state() != &parent.candidate_state
        {
            return Err(production_binding_failure(
                input.transaction_id,
                None,
                "WB14 parent-local identity, chronology, or immutable parameters",
            ));
        }
    }
    let persistent_beginning_state = parent_working_state.map_or_else(
        || resource.beginning_state().clone(),
        |parent| parent.persistent_beginning_state.clone(),
    );
    let mut per_ofe_authorities = match parent_working_state {
        Some(parent) => {
            for authority in parent.per_ofe_authorities.values() {
                authority.canonical_sha256().map_err(|_| {
                    DirectSurfaceLiquidError::Identity("invalid WB14 scalar receipt chain")
                })?;
            }
            parent.per_ofe_authorities.clone()
        }
        None => begin_scalar_wb14_authorities(
            configuration,
            input,
            &persistent_beginning_state,
            parent_support_start_ns,
            parent_support_end_ns,
            &wb14_configuration_sha256,
            &wb14_model_definition_sha256,
            coupled_binding,
        )?,
    };
    let selected_upper_bound_s = coupled_binding.map_or_else(
        || proposed_upper_bound_s(input.interval_s),
        |binding| Ok(f64::from_bits(binding.proposed_upper_bound_s_bits)),
    )?;
    let closure_wb14_beginnings = per_ofe_authorities
        .iter()
        .map(|(ofe_id, authority)| {
            let working = authority.working();
            (
                ofe_id.clone(),
                (
                    working.cumulative_supply_m,
                    working.cumulative_infiltration_m,
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    if !selected_upper_bound_s.is_finite()
        || !matches!(
            selected_upper_bound_s.to_bits(),
            0x409c_2000_0000_0000 | 0x408c_2000_0000_0000 | 0x404e_0000_0000_0000
        )
        || input.interval_s > selected_upper_bound_s
    {
        return Err(DirectSurfaceLiquidError::Domain(
            "accepted WB14 child support exceeds selected coupled upper bound",
        ));
    }
    let mut receipts = Vec::new();
    let mut ledgers = Vec::new();
    let mut call_count = BTreeMap::new();

    for ofe_id in &configuration.ofe_topology {
        let pending_routed_parcels_before_sha256 = pending_routed_queue_sha256(&pending);
        let continuation_index = ending
            .continuations
            .iter()
            .position(|row| &row.ofe_id == ofe_id)
            .ok_or_else(|| {
                production_binding_failure(
                    input.transaction_id,
                    Some(ofe_id.clone()),
                    "missing WB14 continuation",
                )
            })?;
        let scalar_working = per_ofe_authorities
            .get(ofe_id)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "missing WB14 scalar authority",
            ))?
            .working();
        let cumulative_supply_m = scalar_working.cumulative_supply_m;
        let cumulative_infiltration_m = scalar_working.cumulative_infiltration_m;
        let ofe_parcels = pending.remove(ofe_id).unwrap_or_default();
        let parameter = parameters
            .get(ofe_id)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "missing WB14 parameters",
            ))?;
        let mut advanced = advance_one_ofe(
            configuration,
            &mut ending,
            ofe_id,
            ofe_parcels,
            parameter,
            cumulative_supply_m,
            cumulative_infiltration_m,
            input.transaction_id,
            input.interval_s,
        )?;
        route_runoff(
            configuration,
            ofe_id,
            std::mem::take(&mut advanced.runoff),
            &mut pending,
            &mut receipts,
            input.transaction_id,
        )?;
        let pending_routed_parcels_after_sha256 = pending_routed_queue_sha256(&pending);
        let scalar_beginning =
            per_ofe_authorities
                .get(ofe_id)
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "missing WB14 scalar authority",
                ))?;
        let scalar_working = scalar_beginning.working();
        let child_start_ns = coupled_binding.map_or(scalar_working.accepted_until_ns, |binding| {
            binding.child_support_start_ns
        });
        let child_end_ns = coupled_binding
            .map_or_else(
                || {
                    scalar_working
                        .accepted_until_ns
                        .checked_add((input.interval_s * 1_000_000_000.0) as u128)
                },
                |binding| Some(binding.child_support_end_ns),
            )
            .ok_or(DirectSurfaceLiquidError::Identity(
                "WB14 child support overflow",
            ))?;
        if child_start_ns != scalar_working.accepted_until_ns
            || child_end_ns <= child_start_ns
            || (child_end_ns - child_start_ns) as f64 / 1_000_000_000.0 != input.interval_s
        {
            return Err(production_binding_failure(
                input.transaction_id,
                Some(ofe_id.clone()),
                "coupled/WB14 child support join",
            ));
        }
        let (scalar_ending, scalar_outcome) = scalar_beginning
            .accept_child_transitions_with_slab(
                scalar_working.next_child_ordinal,
                child_start_ns,
                child_end_ns,
                scalar_working.receipt_chain_sha256,
                coupled_binding.map_or([0; 32], |binding| binding.accepted_slab_sha256),
                coupled_binding.map_or([0; 32], |binding| {
                    binding.parent_beginning_complete_owner_set_sha256
                }),
                pending_routed_parcels_before_sha256,
                pending_routed_parcels_after_sha256,
                selected_upper_bound_s,
                &advanced.wb14_transitions,
            )
            .map_err(|_| {
                production_binding_failure(
                    input.transaction_id,
                    Some(ofe_id.clone()),
                    "WB14 scalar receipt authority rejected live child",
                )
            })?;
        if scalar_outcome.cumulative_supply_m.to_bits() != advanced.cumulative_supply_m.to_bits()
            || scalar_outcome.cumulative_infiltration_m.to_bits()
                != advanced.cumulative_infiltration_m.to_bits()
        {
            return Err(production_binding_failure(
                input.transaction_id,
                Some(ofe_id.clone()),
                "WB14 scalar/live physical transition mismatch",
            ));
        }
        per_ofe_authorities.insert(ofe_id.clone(), scalar_ending);
        if call_count.insert(ofe_id.clone(), 1).is_some() {
            return Err(production_binding_failure(
                input.transaction_id,
                Some(ofe_id.clone()),
                "duplicate high-level WB14 call",
            ));
        }
        ending.continuations[continuation_index].day_index = input.day_index;
        ending.continuations[continuation_index].next_interval_index =
            input.interval_index.checked_add(1).ok_or_else(|| {
                production_binding_failure(
                    input.transaction_id,
                    Some(ofe_id.clone()),
                    "WB14 interval index overflow",
                )
            })?;
        ending.continuations[continuation_index].cumulative_supply_m = advanced.cumulative_supply_m;
        ending.continuations[continuation_index].cumulative_infiltration_m =
            advanced.cumulative_infiltration_m;
        ending.continuations[continuation_index].last_accepted_transaction_id =
            Some(input.transaction_id);
        receipts.extend(advanced.receipts);
        if let Some(ledger) = advanced.ledger {
            ledgers.push(ledger);
        }
    }
    if pending.values().any(|rows| !rows.is_empty()) {
        return Err(candidate_failure(
            input.transaction_id,
            DirectSurfaceLiquidErrorContext::default(),
            "unresolved routed surface-liquid parcel",
        ));
    }
    for record in &mut ending.records {
        record.last_accepted_transaction_id = Some(input.transaction_id);
    }
    let accepted_duration_s = parent_working_state.map_or(input.interval_s, |parent| {
        parent.accepted_duration_s + input.interval_s
    });
    ending.state_sha256 = ending.recomputed_sha256()?;
    let closure_physical_ending = ending.clone();
    let mut physical_ending = ending.clone();
    let parent_finalizations = if finalize_parent_interval && parent_child_mode {
        Some(
            per_ofe_authorities
                .iter()
                .map(|(ofe_id, parent)| {
                    parent
                        .finalize()
                        .map(|receipt| (ofe_id.clone(), receipt))
                        .map_err(|_| {
                            production_binding_failure(
                                input.transaction_id,
                                Some(ofe_id.clone()),
                                "WB14 scalar parent finalization",
                            )
                        })
                })
                .collect::<Result<BTreeMap<_, _>, _>>()?,
        )
    } else {
        None
    };
    let wb14_child_replay_bytes =
        serde_json::to_vec(&per_ofe_authorities.iter().collect::<Vec<_>>())
            .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 child replay serialization"))?;
    let wb14_child_receipt_set_sha256 =
        Sha256Digest::try_new(format!("{:x}", Sha256::digest(&wb14_child_replay_bytes)))
            .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 child replay digest"))?;
    let wb14_parent_replay_bytes = parent_finalizations
        .as_ref()
        .map(|rows| {
            serde_json::to_vec(&rows.iter().collect::<Vec<_>>())
                .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 parent replay serialization"))
        })
        .transpose()?;
    let wb14_parent_receipt_set_sha256 = wb14_parent_replay_bytes
        .as_ref()
        .map(|bytes| {
            Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes)))
                .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 parent replay digest"))
        })
        .transpose()?;
    let next_parent_working_state = if !parent_child_mode {
        None
    } else if finalize_parent_interval {
        if accepted_duration_s.to_bits() != INTERVAL_S.to_bits() {
            return Err(production_binding_failure(
                input.transaction_id,
                None,
                "final WB14 child does not close exact parent support",
            ));
        }
        None
    } else {
        // Parent-local physical custody has distinct slab lineage. Preserve
        // persistent predecessor/cursor fields until the one parent commit;
        // only mass and WB14 cumulative physics advance between children.
        for (candidate, persistent) in physical_ending
            .records
            .iter_mut()
            .zip(&persistent_beginning_state.records)
        {
            candidate.last_accepted_transaction_id = persistent.last_accepted_transaction_id;
        }
        for (candidate, persistent) in physical_ending
            .continuations
            .iter_mut()
            .zip(&persistent_beginning_state.continuations)
        {
            candidate.day_index = persistent.day_index;
            candidate.next_interval_index = persistent.next_interval_index;
            candidate.cumulative_supply_m = persistent.cumulative_supply_m;
            candidate.cumulative_infiltration_m = persistent.cumulative_infiltration_m;
            candidate.last_accepted_transaction_id = persistent.last_accepted_transaction_id;
        }
        physical_ending.state_sha256 = physical_ending.recomputed_sha256()?;
        physical_ending.validate(configuration)?;
        let working = DirectWb14ParentWorkingState {
            schema: WB14_PARENT_WORKING_SCHEMA.to_owned(),
            parent_day_index: input.day_index,
            parent_interval_index: input.interval_index,
            parent_support_start_ns,
            parent_support_end_ns,
            surface_liquid_configuration_sha256: configuration.configuration_sha256.clone(),
            wb14_configuration_sha256,
            wb14_model_definition_sha256,
            production_lane_ids,
            accepted_duration_s,
            parameters: input.wb14_parameters.clone(),
            persistent_beginning_state: persistent_beginning_state.clone(),
            candidate_state: physical_ending.clone(),
            per_ofe_authorities,
            parent_finalizations,
        };
        ending = persistent_beginning_state.clone();
        Some(working)
    };
    ending.state_sha256 = ending.recomputed_sha256()?;
    if !parent_child_mode || finalize_parent_interval {
        ending.validate(configuration)?;
    }
    let closure_operands = capture_and_validate_surface_liquid_closure(
        configuration,
        resource,
        input,
        &closure_physical_ending,
        &receipts,
        &closure_wb14_beginnings,
    )?;
    Ok(DirectSurfaceLiquidIngressCandidate {
        transaction_id: input.transaction_id,
        beginning_state: resource.beginning_state().clone(),
        ending_state: ending,
        receipts,
        ledgers,
        wb14_calls_by_ofe: call_count,
        closure_operands,
        open_ingress_parcels: input
            .tile_ingress
            .iter()
            .filter_map(|ingress| match ingress {
                DirectTileGroundIngress::OpenLiquidParcels { parcels, .. } => {
                    Some(parcels.as_slice())
                }
                DirectTileGroundIngress::CoveredCanopyReleaseAndRunon { runon_parcels, .. } => {
                    Some(runon_parcels.as_slice())
                }
                _ => None,
            })
            .flatten()
            .cloned()
            .collect(),
        parent_child_mode,
        finalize_parent_interval,
        input_parent_working_state: parent_working_state.cloned(),
        parent_working_state: next_parent_working_state,
        wb14_child_receipt_set_sha256,
        wb14_parent_receipt_set_sha256,
        wb14_child_replay_bytes,
        wb14_parent_replay_bytes,
    })
}

include!("surface_liquid_ingress_preflight.rs");
#[allow(clippy::too_many_lines)]
fn validate_and_build_local_ingress(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<BTreeMap<OfeId, Vec<TimedParcel>>, DirectSurfaceLiquidError> {
    if input.tile_ingress.len() != configuration.records.len() {
        return Err(DirectSurfaceLiquidError::Protocol(
            "ground-ingress tile cardinality mismatch",
        ));
    }
    let mut pending = BTreeMap::<OfeId, Vec<TimedParcel>>::new();
    let mut seen = BTreeSet::new();
    for ingress in &input.tile_ingress {
        let (ofe_id, tile_id, surface_id) = ingress.identity();
        let identity = (ofe_id.clone(), tile_id.clone(), surface_id.clone());
        if !seen.insert(identity) {
            return Err(DirectSurfaceLiquidError::canonical_failure(
                DirectSurfaceLiquidErrorCode::E005,
                DirectSurfaceLiquidPhase::IngressCandidate,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ofe_id: Some(ofe_id.clone()),
                    tile_id: Some(tile_id.clone()),
                    surface_id: Some(surface_id.clone()),
                    source_id: None,
                    parcel_id: None,
                },
                super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                    beginning_owner_sha256: None,
                    attempted_owner_sha256: None,
                },
                "duplicate tile ground ingress",
            ));
        }
        let configured = configuration
            .records
            .iter()
            .find(|row| {
                &row.key.ofe_id == ofe_id
                    && &row.key.tile_id == tile_id
                    && &row.key.surface_id == surface_id
            })
            .ok_or_else(|| {
                DirectSurfaceLiquidError::canonical_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    DirectSurfaceLiquidPhase::IngressCandidate,
                    DirectSurfaceLiquidErrorContext {
                        transaction_id: Some(input.transaction_id),
                        owner_id: Some(configuration.owner_id.clone()),
                        ofe_id: Some(ofe_id.clone()),
                        tile_id: Some(tile_id.clone()),
                        surface_id: Some(surface_id.clone()),
                        source_id: None,
                        parcel_id: None,
                    },
                    super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                        beginning_owner_sha256: None,
                        attempted_owner_sha256: None,
                    },
                    "unknown tile ground ingress",
                )
            })?;
        if ingress.mode() != configured.ground_ingress_mode {
            return Err(DirectSurfaceLiquidError::canonical_failure(
                DirectSurfaceLiquidErrorCode::E002,
                DirectSurfaceLiquidPhase::IngressCandidate,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ofe_id: Some(configured.key.ofe_id.clone()),
                    tile_id: Some(configured.key.tile_id.clone()),
                    surface_id: Some(configured.key.surface_id.clone()),
                    source_id: Some(configured.key.source_id.clone()),
                    parcel_id: None,
                },
                super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                    beginning_owner_sha256: None,
                    attempted_owner_sha256: None,
                },
                "open/covered ingress mode mismatch",
            ));
        }
        append_tile_ingress(
            configured,
            ingress,
            input.transaction_id,
            input.interval_s,
            pending.entry(ofe_id.clone()).or_default(),
        )
        .map_err(|error| {
            let original_code = error.code();
            let code = if original_code == DirectSurfaceLiquidErrorCode::E010 {
                DirectSurfaceLiquidErrorCode::E009
            } else {
                original_code
            };
            error.complete_context(
                code,
                DirectSurfaceLiquidPhase::IngressCandidate,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ofe_id: Some(configured.key.ofe_id.clone()),
                    tile_id: Some(configured.key.tile_id.clone()),
                    surface_id: Some(configured.key.surface_id.clone()),
                    source_id: Some(configured.key.source_id.clone()),
                    parcel_id: None,
                },
                None,
                None,
            )
        })?;
    }
    for overflow in resource.condensation_overflow() {
        require_nonnegative(
            overflow.amount_kg_m2_ofe_ground,
            "condensation overflow mass",
        )?;
        require_temperature(overflow.temperature_k)?;
        let expected = liquid_specific_enthalpy(overflow.temperature_k);
        if overflow.specific_liquid_enthalpy_j_kg.to_bits() != expected.to_bits() {
            return Err(candidate_failure(
                input.transaction_id,
                DirectSurfaceLiquidErrorContext {
                    ofe_id: Some(overflow.store_key.ofe_id.clone()),
                    owner_id: Some(configuration.owner_id.clone()),
                    tile_id: Some(overflow.store_key.tile_id.clone()),
                    surface_id: Some(overflow.store_key.surface_id.clone()),
                    source_id: Some(overflow.store_key.source_id.clone()),
                    parcel_id: None,
                    transaction_id: None,
                },
                "condensation overflow temperature/enthalpy mismatch",
            ));
        }
        let id = canonical_surface_liquid_source_id(CanonicalSurfaceLiquidSource::Condensation {
            transaction_id: input.transaction_id,
            store_key: &overflow.store_key,
        });
        let enthalpy = checked_surface_liquid_mul(
            overflow.amount_kg_m2_ofe_ground,
            overflow.specific_liquid_enthalpy_j_kg,
        )
        .ok_or_else(|| {
            ingress_arithmetic_failure(
                input.transaction_id,
                &overflow.store_key,
                Some(id.clone()),
                "condensation-overflow enthalpy is nonfinite or underflowed",
            )
        })?;
        pending
            .entry(overflow.store_key.ofe_id.clone())
            .or_default()
            .push(TimedParcel {
                parcel_id: id,
                origin_store_key: overflow.store_key.clone(),
                recipient_store_key: overflow.store_key.clone(),
                basis_ofe_id: overflow.store_key.ofe_id.clone(),
                kind: DirectSurfaceLiquidParcelKind::CondensationOverflow,
                start_s: 0.0,
                end_s: input.interval_s,
                mass_kg_m2_basis_ofe_ground: overflow.amount_kg_m2_ofe_ground,
                enthalpy_j_m2_basis_ofe_ground: enthalpy,
            });
    }
    Ok(pending)
}

fn append_tile_ingress(
    configured: &DirectSurfaceLiquidConfigurationRecord,
    ingress: &DirectTileGroundIngress,
    transaction_id: TransactionId,
    interval_s: f64,
    parcels: &mut Vec<TimedParcel>,
) -> Result<(), DirectSurfaceLiquidError> {
    match ingress {
        DirectTileGroundIngress::OpenRawPrecipitation {
            raw_precipitation, ..
        } => append_amount(
            configured,
            DirectSurfaceLiquidParcelKind::RawPrecipitation,
            raw_precipitation,
            false,
            transaction_id,
            interval_s,
            parcels,
        ),
        DirectTileGroundIngress::OpenLiquidParcels {
            ofe_id,
            tile_id,
            parcels: ingress_parcels,
            ..
        } => {
            let mut identities = BTreeSet::new();
            for parcel in ingress_parcels {
                if !matches!(
                    parcel.kind,
                    DirectSurfaceLiquidParcelKind::RawPrecipitation
                        | DirectSurfaceLiquidParcelKind::UpstreamRunon
                        | DirectSurfaceLiquidParcelKind::TerminalReceiver
                ) || parcel.destination_ofe_id != *ofe_id
                    || parcel.destination_tile_id != *tile_id
                    || !identities.insert(parcel.parcel_id.clone())
                {
                    return Err(DirectSurfaceLiquidError::Identity(
                        "invalid open liquid parcel kind, destination, or identity",
                    ));
                }
                parcel.amount.validate(false, interval_s)?;
                let mass = checked_surface_liquid_mul(
                    configured.tile_fraction,
                    parcel.amount.mass_kg_m2_tile_ground,
                )
                .ok_or(DirectSurfaceLiquidError::Domain(
                    "open liquid parcel mass conversion",
                ))?;
                let enthalpy =
                    checked_surface_liquid_mul(mass, parcel.amount.specific_liquid_enthalpy_j_kg)
                        .ok_or(DirectSurfaceLiquidError::Domain(
                        "open liquid parcel enthalpy conversion",
                    ))?;
                parcels.push(TimedParcel {
                    parcel_id: parcel.parcel_id.to_string(),
                    origin_store_key: configured.key.clone(),
                    recipient_store_key: configured.key.clone(),
                    basis_ofe_id: configured.key.ofe_id.clone(),
                    kind: parcel.kind,
                    start_s: parcel.amount.start_s,
                    end_s: parcel.amount.end_s,
                    mass_kg_m2_basis_ofe_ground: mass,
                    enthalpy_j_m2_basis_ofe_ground: enthalpy,
                });
            }
            Ok(())
        }
        DirectTileGroundIngress::CoveredCanopyRelease { release, .. } => {
            append_canopy_release(configured, release, transaction_id, interval_s, parcels)
        }
        DirectTileGroundIngress::CoveredCanopyReleaseAndRunon {
            ofe_id,
            tile_id,
            release,
            runon_parcels,
            ..
        } => {
            append_canopy_release(configured, release, transaction_id, interval_s, parcels)?;
            append_external_runon(
                configured,
                ofe_id,
                tile_id,
                runon_parcels,
                interval_s,
                parcels,
            )
        }
    }
}

fn append_canopy_release(
    configured: &DirectSurfaceLiquidConfigurationRecord,
    release: &DirectCanopyLiquidRelease,
    transaction_id: TransactionId,
    interval_s: f64,
    parcels: &mut Vec<TimedParcel>,
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
        append_amount(
            configured,
            kind,
            amount,
            true,
            transaction_id,
            interval_s,
            parcels,
        )?;
    }
    Ok(())
}

fn append_external_runon(
    configured: &DirectSurfaceLiquidConfigurationRecord,
    ofe_id: &OfeId,
    tile_id: &TileId,
    ingress_parcels: &[DirectOpenLiquidIngressParcel],
    interval_s: f64,
    parcels: &mut Vec<TimedParcel>,
) -> Result<(), DirectSurfaceLiquidError> {
    let mut identities = BTreeSet::new();
    for parcel in ingress_parcels {
        if parcel.kind != DirectSurfaceLiquidParcelKind::UpstreamRunon
            || parcel.destination_ofe_id != *ofe_id
            || parcel.destination_tile_id != *tile_id
            || !identities.insert(parcel.parcel_id.clone())
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "invalid covered runon kind, destination, or identity",
            ));
        }
        parcel.amount.validate(false, interval_s)?;
        let mass = checked_surface_liquid_mul(
            configured.tile_fraction,
            parcel.amount.mass_kg_m2_tile_ground,
        )
        .ok_or(DirectSurfaceLiquidError::Domain(
            "covered runon mass conversion",
        ))?;
        let enthalpy =
            checked_surface_liquid_mul(mass, parcel.amount.specific_liquid_enthalpy_j_kg).ok_or(
                DirectSurfaceLiquidError::Domain("covered runon enthalpy conversion"),
            )?;
        parcels.push(TimedParcel {
            parcel_id: parcel.parcel_id.to_string(),
            origin_store_key: configured.key.clone(),
            recipient_store_key: configured.key.clone(),
            basis_ofe_id: configured.key.ofe_id.clone(),
            kind: parcel.kind,
            start_s: parcel.amount.start_s,
            end_s: parcel.amount.end_s,
            mass_kg_m2_basis_ofe_ground: mass,
            enthalpy_j_m2_basis_ofe_ground: enthalpy,
        });
    }
    Ok(())
}

fn append_amount(
    configured: &DirectSurfaceLiquidConfigurationRecord,
    kind: DirectSurfaceLiquidParcelKind,
    amount: &DirectIngressAmount,
    require_full_interval: bool,
    transaction_id: TransactionId,
    interval_s: f64,
    parcels: &mut Vec<TimedParcel>,
) -> Result<(), DirectSurfaceLiquidError> {
    amount.validate(require_full_interval, interval_s)?;
    let mass = checked_surface_liquid_mul(configured.tile_fraction, amount.mass_kg_m2_tile_ground)
        .ok_or_else(|| {
            ingress_arithmetic_failure(
                transaction_id,
                &configured.key,
                None,
                "tile-to-OFE ingress mass conversion is nonfinite or underflowed",
            )
        })?;
    let enthalpy = checked_surface_liquid_mul(mass, amount.specific_liquid_enthalpy_j_kg)
        .ok_or_else(|| {
            ingress_arithmetic_failure(
                transaction_id,
                &configured.key,
                None,
                "parcel enthalpy construction is nonfinite or underflowed",
            )
        })?;
    let id = canonical_surface_liquid_source_id(CanonicalSurfaceLiquidSource::Local {
        store_key: &configured.key,
        kind,
    });
    parcels.push(TimedParcel {
        parcel_id: id,
        origin_store_key: configured.key.clone(),
        recipient_store_key: configured.key.clone(),
        basis_ofe_id: configured.key.ofe_id.clone(),
        kind,
        start_s: amount.start_s,
        end_s: amount.end_s,
        mass_kg_m2_basis_ofe_ground: mass,
        enthalpy_j_m2_basis_ofe_ground: enthalpy,
    });
    Ok(())
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn advance_one_ofe(
    configuration: &DirectSurfaceLiquidConfiguration,
    ending: &mut DirectSurfaceLiquidOwnedState,
    ofe_id: &OfeId,
    mut parcels: Vec<TimedParcel>,
    parameter: &DirectOfeWb14Parameters,
    mut cumulative_supply_m: f64,
    mut cumulative_infiltration_m: f64,
    transaction_id: TransactionId,
    interval_s: f64,
) -> Result<OfeAdvance, DirectSurfaceLiquidError> {
    let binding = configuration
        .ofe_bindings
        .iter()
        .find(|binding| &binding.ofe_id == ofe_id)
        .ok_or(DirectSurfaceLiquidError::Identity(
            "missing infiltration recipient binding",
        ))?;
    let arithmetic_key = configuration
        .records
        .iter()
        .find(|record| &record.key.ofe_id == ofe_id)
        .map(|record| &record.key)
        .ok_or(DirectSurfaceLiquidError::Identity(
            "missing ingress arithmetic store",
        ))?;
    parcels.sort_by(parcel_order);
    let mut boundaries = parcels
        .iter()
        .flat_map(|parcel| [parcel.start_s, parcel.end_s])
        .collect::<Vec<_>>();
    boundaries.extend([0.0, interval_s]);
    boundaries.sort_by(f64::total_cmp);
    boundaries.dedup_by(|left, right| left.to_bits() == right.to_bits());
    let mut receipts = Vec::new();
    let mut runoff = Vec::new();
    let ingress_mass = checked_surface_liquid_sum(
        parcels
            .iter()
            .map(|parcel| parcel.mass_kg_m2_basis_ofe_ground),
    )
    .ok_or_else(|| {
        ingress_arithmetic_failure(
            transaction_id,
            arithmetic_key,
            None,
            "OFE ingress mass accumulation is nonfinite",
        )
    })?;
    let ingress_enthalpy = checked_surface_liquid_sum(
        parcels
            .iter()
            .map(|parcel| parcel.enthalpy_j_m2_basis_ofe_ground),
    )
    .ok_or_else(|| {
        ingress_arithmetic_failure(
            transaction_id,
            arithmetic_key,
            None,
            "OFE ingress enthalpy accumulation is nonfinite",
        )
    })?;
    let mut infiltration_mass = 0.0;
    let mut infiltration_enthalpy = 0.0;
    let mut retained_mass = 0.0;
    let mut retained_enthalpy = 0.0;
    let mut runoff_mass = 0.0;
    let mut runoff_enthalpy = 0.0;
    let mut allocated_temporal_mass = vec![0.0; parcels.len()];
    let mut allocated_temporal_enthalpy = vec![0.0; parcels.len()];
    let mut wb14_transitions = Vec::new();

    for boundary in boundaries.windows(2) {
        let start_s = boundary[0];
        let end_s = boundary[1];
        if end_s <= start_s {
            continue;
        }
        let mut contributions = Vec::new();
        for (parcel_index, parcel) in parcels
            .iter()
            .enumerate()
            .filter(|(_, parcel)| parcel.start_s <= start_s && parcel.end_s >= end_s)
        {
            let window = checked_surface_liquid_sub(end_s, start_s).ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    Some(parcel.parcel_id.clone()),
                    "parcel window duration is nonfinite",
                )
            })?;
            let parcel_duration = checked_surface_liquid_sub(parcel.end_s, parcel.start_s)
                .ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        arithmetic_key,
                        Some(parcel.parcel_id.clone()),
                        "parcel support duration is nonfinite",
                    )
                })?;
            let fraction =
                checked_surface_liquid_div(window, parcel_duration).ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        arithmetic_key,
                        Some(parcel.parcel_id.clone()),
                        "parcel support fraction is nonfinite or underflowed",
                    )
                })?;
            let mass = if end_s.to_bits() == parcel.end_s.to_bits() {
                checked_surface_liquid_sub(
                    parcel.mass_kg_m2_basis_ofe_ground,
                    allocated_temporal_mass[parcel_index],
                )
            } else {
                checked_surface_liquid_mul(parcel.mass_kg_m2_basis_ofe_ground, fraction)
            }
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    Some(parcel.parcel_id.clone()),
                    "parcel interval mass is nonfinite or underflowed",
                )
            })?;
            if mass < 0.0 {
                return Err(ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    Some(parcel.parcel_id.clone()),
                    "parcel interval mass remainder is negative",
                ));
            }
            allocated_temporal_mass[parcel_index] =
                checked_surface_liquid_add(allocated_temporal_mass[parcel_index], mass)
                    .ok_or_else(|| {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(parcel.parcel_id.clone()),
                            "parcel interval mass accumulation is nonfinite",
                        )
                    })?;
            let enthalpy = if end_s.to_bits() == parcel.end_s.to_bits() {
                checked_surface_liquid_sub(
                    parcel.enthalpy_j_m2_basis_ofe_ground,
                    allocated_temporal_enthalpy[parcel_index],
                )
            } else {
                checked_surface_liquid_mul(parcel.enthalpy_j_m2_basis_ofe_ground, window)
                    .and_then(|numerator| checked_surface_liquid_div(numerator, parcel_duration))
            }
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    Some(parcel.parcel_id.clone()),
                    "parcel interval enthalpy is nonfinite or underflowed",
                )
            })?;
            allocated_temporal_enthalpy[parcel_index] =
                checked_surface_liquid_add(allocated_temporal_enthalpy[parcel_index], enthalpy)
                    .ok_or_else(|| {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(parcel.parcel_id.clone()),
                            "parcel interval enthalpy accumulation is nonfinite",
                        )
                    })?;
            if mass > 0.0 {
                contributions.push((parcel, mass, enthalpy));
            }
        }
        contributions.sort_by(|left, right| parcel_order(left.0, right.0));
        let supply_mass = checked_surface_liquid_sum(contributions.iter().map(|row| row.1))
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    None,
                    "interval supply mass accumulation is nonfinite",
                )
            })?;
        let supply_enthalpy = checked_surface_liquid_sum(contributions.iter().map(|row| row.2))
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    None,
                    "interval supply enthalpy accumulation is nonfinite",
                )
            })?;
        let duration_s = checked_surface_liquid_sub(end_s, start_s).ok_or_else(|| {
            ingress_arithmetic_failure(
                transaction_id,
                arithmetic_key,
                None,
                "WB14 interval duration is nonfinite",
            )
        })?;
        let interval_supply_m = checked_surface_liquid_div(supply_mass, WATER_DENSITY_KG_M3)
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    None,
                    "WB14 supply depth conversion is nonfinite or underflowed",
                )
            })?;
        let transition = DirectWb14ContinuationIntervalInputs {
            cumulative_supply_m,
            cumulative_infiltration_m,
            interval_supply_m,
            interval_duration_s: duration_s,
            effective_conductivity_m_s: parameter.effective_conductivity_m_s,
            matric_potential_m: parameter.matric_potential_m,
            storage_capacity_m: parameter.infiltration_storage_capacity_m,
        };
        let outcome = advance_wb14_continuation_interval(transition).map_err(|_| {
            production_binding_failure(
                transaction_id,
                Some(ofe_id.clone()),
                "WB14 continuation transition rejected",
            )
        })?;
        wb14_transitions.push(transition);
        cumulative_supply_m = outcome.cumulative_supply_m;
        cumulative_infiltration_m = outcome.cumulative_infiltration_m;
        if supply_mass == 0.0 {
            continue;
        }
        // Preserve the source mass identity when WB14 accepts the complete
        // interval supply. A kg->m->kg round trip can move upward by one ULP;
        // the exact WB14 equality is the authoritative full-infiltration
        // branch and therefore carries the original mass operand unchanged.
        let full_infiltration =
            outcome.interval_infiltration_m.to_bits() == interval_supply_m.to_bits();
        let total_infiltration = if full_infiltration {
            supply_mass
        } else {
            checked_surface_liquid_mul(outcome.interval_infiltration_m, WATER_DENSITY_KG_M3)
                .ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        arithmetic_key,
                        None,
                        "infiltration depth-to-mass conversion is nonfinite or underflowed",
                    )
                })?
        };
        let h_mix = checked_surface_liquid_div(supply_enthalpy, supply_mass).ok_or_else(|| {
            ingress_arithmetic_failure(
                transaction_id,
                arithmetic_key,
                None,
                "mixed parcel enthalpy is nonfinite or underflowed",
            )
        })?;
        let temperature_offset = checked_surface_liquid_div(h_mix, LIQUID_HEAT_CAPACITY_J_KG_K)
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    None,
                    "mixed parcel temperature offset is nonfinite or underflowed",
                )
            })?;
        let temperature_k = checked_surface_liquid_add(REFERENCE_TEMPERATURE_K, temperature_offset)
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    None,
                    "mixed parcel temperature is nonfinite",
                )
            })?;
        let mut allocated_infiltration = 0.0;
        let mut allocated_mixed_enthalpy = 0.0;
        let contribution_count = contributions.len();
        let mut excess_parts = Vec::with_capacity(contribution_count);
        for (index, (parcel, mass, _)) in contributions.into_iter().enumerate() {
            let mixed_part_q = if index + 1 == contribution_count {
                checked_surface_liquid_sub(supply_enthalpy, allocated_mixed_enthalpy)
            } else {
                checked_surface_liquid_mul(supply_enthalpy, mass)
                    .and_then(|numerator| checked_surface_liquid_div(numerator, supply_mass))
            }
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    Some(parcel.parcel_id.clone()),
                    "mixed parcel enthalpy allocation is nonfinite or underflowed",
                )
            })?;
            allocated_mixed_enthalpy =
                checked_surface_liquid_add(allocated_mixed_enthalpy, mixed_part_q).ok_or_else(
                    || {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(parcel.parcel_id.clone()),
                            "mixed parcel enthalpy accumulation is nonfinite",
                        )
                    },
                )?;
            let infiltrated = if full_infiltration {
                mass
            } else if index + 1 == contribution_count {
                checked_surface_liquid_sub(total_infiltration, allocated_infiltration).ok_or_else(
                    || {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(parcel.parcel_id.clone()),
                            "infiltration allocation remainder is nonfinite",
                        )
                    },
                )?
            } else {
                let numerator =
                    checked_surface_liquid_mul(total_infiltration, mass).ok_or_else(|| {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(parcel.parcel_id.clone()),
                            "infiltration allocation numerator is nonfinite or underflowed",
                        )
                    })?;
                checked_surface_liquid_div(numerator, supply_mass).ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        arithmetic_key,
                        Some(parcel.parcel_id.clone()),
                        "infiltration allocation share is nonfinite or underflowed",
                    )
                })?
            };
            allocated_infiltration =
                checked_surface_liquid_add(allocated_infiltration, infiltrated).ok_or_else(
                    || {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(parcel.parcel_id.clone()),
                            "infiltration allocation accumulation is nonfinite",
                        )
                    },
                )?;
            let excess = checked_surface_liquid_sub(mass, infiltrated).ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    Some(parcel.parcel_id.clone()),
                    "parcel excess mass is nonfinite",
                )
            })?;
            let infiltration_q = if excess == 0.0 {
                Some(mixed_part_q)
            } else if infiltrated == 0.0 {
                Some(0.0)
            } else {
                checked_surface_liquid_mul(mixed_part_q, infiltrated)
                    .and_then(|numerator| checked_surface_liquid_div(numerator, mass))
            }
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    Some(parcel.parcel_id.clone()),
                    "infiltration enthalpy is nonfinite or underflowed",
                )
            })?;
            let excess_q =
                checked_surface_liquid_sub(mixed_part_q, infiltration_q).ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        arithmetic_key,
                        Some(parcel.parcel_id.clone()),
                        "parcel excess enthalpy remainder is nonfinite",
                    )
                })?;
            infiltration_mass = checked_surface_liquid_add(infiltration_mass, infiltrated)
                .ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        arithmetic_key,
                        Some(parcel.parcel_id.clone()),
                        "infiltration mass accumulation is nonfinite",
                    )
                })?;
            infiltration_enthalpy =
                checked_surface_liquid_add(infiltration_enthalpy, infiltration_q).ok_or_else(
                    || {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(parcel.parcel_id.clone()),
                            "infiltration enthalpy accumulation is nonfinite",
                        )
                    },
                )?;
            receipts.push(receipt(
                parcel,
                DirectSurfaceLiquidReceiptDisposition::Infiltration,
                DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
                    ofe_id: binding.ofe_id.clone(),
                    production_lane_index: binding.production_lane_index,
                    production_lane_id: binding.production_lane_id,
                    ordered_soil_layer_ids: binding.ordered_soil_layer_ids.clone(),
                    soil_thermal_layer_id: binding.infiltration_soil_thermal_layer_id.clone(),
                },
                ofe_id.clone(),
                start_s,
                end_s,
                infiltrated,
                temperature_k,
                infiltration_q,
                transaction_id,
            ));
            excess_parts.push((parcel, excess, excess_q));
        }
        let (retained_parts, runoff_parts) = retain_excess_proportionally(
            configuration,
            ending,
            excess_parts,
            temperature_k,
            start_s,
            end_s,
            transaction_id,
        )?;
        for retained in retained_parts {
            retained_mass =
                checked_surface_liquid_add(retained_mass, retained.mass_kg_m2_basis_ofe_ground)
                    .ok_or_else(|| {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(retained.parcel_id.clone()),
                            "retained mass accumulation is nonfinite",
                        )
                    })?;
            retained_enthalpy = checked_surface_liquid_add(
                retained_enthalpy,
                retained.enthalpy_j_m2_basis_ofe_ground,
            )
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    arithmetic_key,
                    Some(retained.parcel_id.clone()),
                    "retained enthalpy accumulation is nonfinite",
                )
            })?;
            receipts.push(retained);
        }
        for routed in runoff_parts {
            runoff_mass =
                checked_surface_liquid_add(runoff_mass, routed.mass_kg_m2_basis_ofe_ground)
                    .ok_or_else(|| {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(routed.parcel_id.clone()),
                            "runoff mass accumulation is nonfinite",
                        )
                    })?;
            runoff_enthalpy =
                checked_surface_liquid_add(runoff_enthalpy, routed.enthalpy_j_m2_basis_ofe_ground)
                    .ok_or_else(|| {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(routed.parcel_id.clone()),
                            "runoff enthalpy accumulation is nonfinite",
                        )
                    })?;
            runoff.push(routed);
        }
    }
    let ledger = DirectSurfaceLiquidIngressLedger {
        ofe_id: ofe_id.clone(),
        ingress_mass_kg_m2_ofe_ground: ingress_mass,
        ingress_enthalpy_j_m2_ofe_ground: ingress_enthalpy,
        infiltration_mass_kg_m2_ofe_ground: infiltration_mass,
        infiltration_enthalpy_j_m2_ofe_ground: infiltration_enthalpy,
        retained_mass_kg_m2_ofe_ground: retained_mass,
        retained_enthalpy_j_m2_ofe_ground: retained_enthalpy,
        runoff_mass_kg_m2_ofe_ground: runoff_mass,
        runoff_enthalpy_j_m2_ofe_ground: runoff_enthalpy,
    };
    Ok(OfeAdvance {
        receipts,
        runoff,
        ledger: Some(ledger),
        cumulative_supply_m,
        cumulative_infiltration_m,
        wb14_transitions,
    })
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
fn retain_excess_proportionally(
    configuration: &DirectSurfaceLiquidConfiguration,
    ending: &mut DirectSurfaceLiquidOwnedState,
    excess_parts: Vec<(&TimedParcel, f64, f64)>,
    temperature_k: f64,
    start_s: f64,
    end_s: f64,
    transaction_id: TransactionId,
) -> Result<(Vec<DirectSurfaceLiquidParcelReceipt>, Vec<TimedParcel>), DirectSurfaceLiquidError> {
    let mut grouped = BTreeMap::<DirectSurfaceLiquidStoreKey, Vec<(&TimedParcel, f64, f64)>>::new();
    for (parcel, excess, excess_q) in excess_parts {
        grouped
            .entry(parcel.recipient_store_key.clone())
            .or_default()
            .push((parcel, excess, excess_q));
    }
    let mut retained_receipts = Vec::new();
    let mut runoff_parcels = Vec::new();
    for (store_key, mut parts) in grouped {
        parts.sort_by(|left, right| parcel_order(left.0, right.0));
        let index = configuration
            .records
            .iter()
            .position(|row| row.key == store_key)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "parcel recipient store missing",
            ))?;
        let configured = &configuration.records[index];
        let state = &mut ending.records[index];
        let available_tile =
            checked_surface_liquid_sub(configured.capacity_kg_m2_tile, state.liquid_kg_m2_tile)
                .ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        &store_key,
                        None,
                        "surface retention capacity difference is nonfinite",
                    )
                })?;
        let available = checked_surface_liquid_mul(configured.tile_fraction, available_tile)
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    &store_key,
                    None,
                    "surface retention area conversion is nonfinite or underflowed",
                )
            })?;
        if available < 0.0 {
            return Err(DirectSurfaceLiquidError::Bound(
                "negative surface retention capacity",
            ));
        }
        let total_excess =
            checked_surface_liquid_sum(parts.iter().map(|row| row.1)).ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    &store_key,
                    None,
                    "surface excess mass accumulation is nonfinite",
                )
            })?;
        let total_retained = total_excess.min(available);
        let mut allocated_retained = 0.0;
        let count = parts.len();
        for (part_index, (parcel, excess, excess_q)) in parts.into_iter().enumerate() {
            let retained_mass = if part_index + 1 == count {
                checked_surface_liquid_sub(total_retained, allocated_retained).ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        &store_key,
                        Some(parcel.parcel_id.clone()),
                        "retained allocation remainder is nonfinite",
                    )
                })?
            } else if total_excess == 0.0 {
                0.0
            } else {
                let numerator =
                    checked_surface_liquid_mul(total_retained, excess).ok_or_else(|| {
                        ingress_arithmetic_failure(
                            transaction_id,
                            &store_key,
                            Some(parcel.parcel_id.clone()),
                            "retained allocation numerator is nonfinite or underflowed",
                        )
                    })?;
                checked_surface_liquid_div(numerator, total_excess).ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        &store_key,
                        Some(parcel.parcel_id.clone()),
                        "retained allocation share is nonfinite or underflowed",
                    )
                })?
            };
            allocated_retained = checked_surface_liquid_add(allocated_retained, retained_mass)
                .ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        &store_key,
                        Some(parcel.parcel_id.clone()),
                        "retained allocation accumulation is nonfinite",
                    )
                })?;
            let runoff_mass =
                checked_surface_liquid_sub(excess, retained_mass).ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        &store_key,
                        Some(parcel.parcel_id.clone()),
                        "runoff mass difference is nonfinite",
                    )
                })?;
            let retained_q = if runoff_mass == 0.0 {
                Some(excess_q)
            } else if retained_mass == 0.0 {
                Some(0.0)
            } else {
                checked_surface_liquid_mul(excess_q, retained_mass)
                    .and_then(|numerator| checked_surface_liquid_div(numerator, excess))
            }
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    &store_key,
                    Some(parcel.parcel_id.clone()),
                    "retained enthalpy is nonfinite or underflowed",
                )
            })?;
            let runoff_q = checked_surface_liquid_sub(excess_q, retained_q).ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    &store_key,
                    Some(parcel.parcel_id.clone()),
                    "runoff enthalpy remainder is nonfinite",
                )
            })?;
            if retained_mass > 0.0 {
                retained_receipts.push(receipt(
                    parcel,
                    DirectSurfaceLiquidReceiptDisposition::RetainedSurface,
                    DirectSurfaceLiquidReceiptRecipient::SurfaceStore {
                        store_key: parcel.recipient_store_key.clone(),
                    },
                    parcel.basis_ofe_id.clone(),
                    start_s,
                    end_s,
                    retained_mass,
                    temperature_k,
                    retained_q,
                    transaction_id,
                ));
            }
            if runoff_mass > 0.0 {
                runoff_parcels.push(TimedParcel {
                    parcel_id: parcel.parcel_id.clone(),
                    origin_store_key: parcel.origin_store_key.clone(),
                    recipient_store_key: parcel.recipient_store_key.clone(),
                    basis_ofe_id: parcel.basis_ofe_id.clone(),
                    kind: parcel.kind,
                    start_s,
                    end_s,
                    mass_kg_m2_basis_ofe_ground: runoff_mass,
                    enthalpy_j_m2_basis_ofe_ground: runoff_q,
                });
            }
        }
        let retained_tile = checked_surface_liquid_div(total_retained, configured.tile_fraction)
            .ok_or_else(|| {
                ingress_arithmetic_failure(
                    transaction_id,
                    &store_key,
                    None,
                    "retained OFE-to-tile conversion is nonfinite or underflowed",
                )
            })?;
        state.liquid_kg_m2_tile =
            checked_surface_liquid_add(state.liquid_kg_m2_tile, retained_tile).ok_or_else(
                || {
                    ingress_arithmetic_failure(
                        transaction_id,
                        &store_key,
                        None,
                        "ending surface store accumulation is nonfinite",
                    )
                },
            )?;
    }
    Ok((retained_receipts, runoff_parcels))
}

#[allow(clippy::too_many_lines)]
fn route_runoff(
    configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: &OfeId,
    runoff: Vec<TimedParcel>,
    pending: &mut BTreeMap<OfeId, Vec<TimedParcel>>,
    receipts: &mut Vec<DirectSurfaceLiquidParcelReceipt>,
    transaction_id: TransactionId,
) -> Result<(), DirectSurfaceLiquidError> {
    let route = configuration
        .records
        .iter()
        .find(|row| &row.key.ofe_id == ofe_id)
        .ok_or(DirectSurfaceLiquidError::Identity(
            "route source OFE missing",
        ))?;
    match (
        &route.runon_destination_ofe_id,
        &route.runon_destination_tile_id,
    ) {
        (Some(destination_ofe), Some(destination_tile)) => {
            let destination = configuration
                .records
                .iter()
                .find(|row| {
                    &row.key.ofe_id == destination_ofe && &row.key.tile_id == destination_tile
                })
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "route destination store missing",
                ))?;
            let area_ratio = checked_surface_liquid_div(route.ofe_area_m2, destination.ofe_area_m2)
                .ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        &route.key,
                        None,
                        "OFE routing area ratio is nonfinite or underflowed",
                    )
                })?;
            for parcel in runoff {
                let parcel_temperature = parcel_temperature_k(&parcel, transaction_id, &route.key)?;
                let routed_mass =
                    checked_surface_liquid_mul(parcel.mass_kg_m2_basis_ofe_ground, area_ratio)
                        .ok_or_else(|| {
                            ingress_arithmetic_failure(
                                transaction_id,
                                &route.key,
                                Some(parcel.parcel_id.clone()),
                                "routed mass area conversion is nonfinite or underflowed",
                            )
                        })?;
                let routed_enthalpy =
                    checked_surface_liquid_mul(parcel.enthalpy_j_m2_basis_ofe_ground, area_ratio)
                        .ok_or_else(|| {
                        ingress_arithmetic_failure(
                            transaction_id,
                            &route.key,
                            Some(parcel.parcel_id.clone()),
                            "routed enthalpy area conversion is nonfinite or underflowed",
                        )
                    })?;
                receipts.push(receipt(
                    &parcel,
                    DirectSurfaceLiquidReceiptDisposition::RoutedRunoff,
                    DirectSurfaceLiquidReceiptRecipient::RoutedOfe {
                        source_ofe_id: parcel.basis_ofe_id.clone(),
                        destination_ofe_id: destination_ofe.clone(),
                        destination_store_key: destination.key.clone(),
                    },
                    parcel.basis_ofe_id.clone(),
                    parcel.start_s,
                    parcel.end_s,
                    parcel.mass_kg_m2_basis_ofe_ground,
                    parcel_temperature,
                    parcel.enthalpy_j_m2_basis_ofe_ground,
                    transaction_id,
                ));
                pending
                    .entry(destination_ofe.clone())
                    .or_default()
                    .push(TimedParcel {
                        parcel_id: parcel.parcel_id,
                        origin_store_key: parcel.origin_store_key,
                        recipient_store_key: destination.key.clone(),
                        basis_ofe_id: destination_ofe.clone(),
                        kind: DirectSurfaceLiquidParcelKind::UpstreamRunon,
                        start_s: parcel.start_s,
                        end_s: parcel.end_s,
                        mass_kg_m2_basis_ofe_ground: routed_mass,
                        enthalpy_j_m2_basis_ofe_ground: routed_enthalpy,
                    });
            }
        }
        (None, None) => {
            for parcel in runoff {
                let parcel_temperature = parcel_temperature_k(&parcel, transaction_id, &route.key)?;
                receipts.push(receipt(
                    &parcel,
                    DirectSurfaceLiquidReceiptDisposition::OutletRunoff,
                    DirectSurfaceLiquidReceiptRecipient::Outlet {
                        ofe_id: parcel.basis_ofe_id.clone(),
                    },
                    parcel.basis_ofe_id.clone(),
                    parcel.start_s,
                    parcel.end_s,
                    parcel.mass_kg_m2_basis_ofe_ground,
                    parcel_temperature,
                    parcel.enthalpy_j_m2_basis_ofe_ground,
                    transaction_id,
                ));
            }
        }
        _ => {
            return Err(DirectSurfaceLiquidError::Identity(
                "incomplete surface-liquid route",
            ));
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn receipt(
    parcel: &TimedParcel,
    disposition: DirectSurfaceLiquidReceiptDisposition,
    recipient: DirectSurfaceLiquidReceiptRecipient,
    basis_ofe_id: OfeId,
    start_s: f64,
    end_s: f64,
    mass: f64,
    temperature_k: f64,
    enthalpy: f64,
    transaction_id: TransactionId,
) -> DirectSurfaceLiquidParcelReceipt {
    DirectSurfaceLiquidParcelReceipt {
        parcel_id: format!(
            "{}:{disposition:?}:{basis_ofe_id:?}:{:016x}:{:016x}",
            parcel.parcel_id,
            start_s.to_bits(),
            end_s.to_bits()
        ),
        source_parcel_id: parcel.parcel_id.clone(),
        transaction_id,
        origin_store_key: parcel.origin_store_key.clone(),
        recipient_store_key: parcel.recipient_store_key.clone(),
        recipient,
        basis_ofe_id,
        kind: parcel.kind,
        disposition,
        start_s,
        end_s,
        mass_kg_m2_basis_ofe_ground: mass,
        temperature_k,
        enthalpy_j_m2_basis_ofe_ground: enthalpy,
    }
}

fn parcel_order(left: &TimedParcel, right: &TimedParcel) -> std::cmp::Ordering {
    canonical_parcel_order(
        CanonicalParcelOrderKey {
            start_s: left.start_s,
            end_s: left.end_s,
            origin_store_key: &left.origin_store_key,
            kind: left.kind,
            source_parcel_id: &left.parcel_id,
        },
        CanonicalParcelOrderKey {
            start_s: right.start_s,
            end_s: right.end_s,
            origin_store_key: &right.origin_store_key,
            kind: right.kind,
            source_parcel_id: &right.parcel_id,
        },
    )
}

fn liquid_specific_enthalpy(temperature_k: f64) -> f64 {
    LIQUID_HEAT_CAPACITY_J_KG_K * (temperature_k - REFERENCE_TEMPERATURE_K)
}

fn require_temperature(value: f64) -> Result<(), DirectSurfaceLiquidError> {
    if value.is_finite() && (200.0..=350.0).contains(&value) {
        Ok(())
    } else {
        Err(DirectSurfaceLiquidError::Domain("liquid temperature"))
    }
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

#[cfg(test)]
fn mass_tolerance(scale: f64) -> f64 {
    MASS_ABSOLUTE_TOLERANCE_KG_M2 + SCALE_MULTIPLIER * f64::EPSILON * scale
}

#[cfg(test)]
#[path = "surface_liquid_ingress_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "surface_liquid_ingress_context_tests.rs"]
mod context_tests;
