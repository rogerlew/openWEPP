//! Timed snow-free ingress, WB14 continuation, retention, and routing.

#![allow(clippy::missing_errors_doc)]

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{SoilLayerId, TileId, TransactionId};
use openwepp_land_surface_energy::{OfeId, SurfaceId};
use serde::{Deserialize, Serialize};

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

const INTERVAL_S: f64 = 1_800.0;
const WATER_DENSITY_KG_M3: f64 = 1_000.0;
const LIQUID_HEAT_CAPACITY_J_KG_K: f64 = 4_218.0;
const REFERENCE_TEMPERATURE_K: f64 = 273.15;
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
    fn validate(&self, require_full_interval: bool) -> Result<(), DirectSurfaceLiquidError> {
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
            || self.end_s > INTERVAL_S
            || (require_full_interval
                && (self.start_s.to_bits() != 0.0_f64.to_bits()
                    || self.end_s.to_bits() != INTERVAL_S.to_bits()))
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "ingress_mode", deny_unknown_fields)]
pub enum DirectTileGroundIngress {
    OpenRawPrecipitation {
        ofe_id: OfeId,
        tile_id: TileId,
        surface_id: SurfaceId,
        raw_precipitation: DirectIngressAmount,
    },
    CoveredCanopyRelease {
        ofe_id: OfeId,
        tile_id: TileId,
        surface_id: SurfaceId,
        release: DirectCanopyLiquidRelease,
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
            | Self::CoveredCanopyRelease {
                ofe_id,
                tile_id,
                surface_id,
                ..
            } => (ofe_id, tile_id, surface_id),
        }
    }

    fn mode(&self) -> DirectGroundIngressMode {
        match self {
            Self::OpenRawPrecipitation { .. } => DirectGroundIngressMode::OpenRawPrecipitation,
            Self::CoveredCanopyRelease { .. } => DirectGroundIngressMode::CoveredCanopyRelease,
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectSurfaceLiquidReceiptDisposition {
    Infiltration,
    RetainedSurface,
    RoutedRunoff,
    OutletRunoff,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
}

impl DirectSurfaceLiquidIngressCandidate {
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

    pub fn validate(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
        resource: &DirectSurfaceLiquidResourceCandidate,
        input: &DirectSurfaceLiquidIngressInput,
    ) -> Result<(), DirectSurfaceLiquidError> {
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
        let expected = execute_surface_liquid_ingress_inner(configuration, resource, input)
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
        super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            configuration,
            resource,
            &self.closure_operands,
            &self.receipts,
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
        if self.receipts != expected.receipts {
            let receipt = first_positional_mismatch(&self.receipts, &expected.receipts);
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
        if self.ledgers != expected.ledgers {
            let ledger = first_positional_mismatch(&self.ledgers, &expected.ledgers);
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
            let ofe_id = self
                .wb14_calls_by_ofe
                .iter()
                .find(|(key, value)| expected.wb14_calls_by_ofe.get(*key) != Some(*value))
                .map(|(key, _)| key)
                .or_else(|| {
                    expected
                        .wb14_calls_by_ofe
                        .keys()
                        .find(|key| !self.wb14_calls_by_ofe.contains_key(*key))
                });
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

fn first_positional_mismatch<'a, T: PartialEq>(
    actual: &'a [T],
    expected: &'a [T],
) -> Option<&'a T> {
    (0..actual.len().max(expected.len()))
        .find(|&index| actual.get(index) != expected.get(index))
        .and_then(|index| actual.get(index).or_else(|| expected.get(index)))
}

fn state_mismatch_context(
    actual: &DirectSurfaceLiquidOwnedState,
    expected: &DirectSurfaceLiquidOwnedState,
    mut context: DirectSurfaceLiquidErrorContext,
) -> DirectSurfaceLiquidErrorContext {
    if let Some(row) = first_positional_mismatch(&actual.records, &expected.records) {
        context.ofe_id = Some(row.key.ofe_id.clone());
        context.tile_id = Some(row.key.tile_id.clone());
        context.surface_id = Some(row.key.surface_id.clone());
        context.source_id = Some(row.key.source_id.clone());
        return context;
    }
    if let Some(row) = first_positional_mismatch(&actual.continuations, &expected.continuations) {
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
}

/// Execute the admitted post-resource ingress transaction against a cloned owner state.
pub fn execute_surface_liquid_ingress(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidError> {
    execute_surface_liquid_ingress_inner(configuration, resource, input).map_err(|error| {
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
) -> Result<DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidError> {
    configuration.validate()?;
    resource.beginning_state().validate(configuration)?;
    if input.transaction_id != resource.transaction_id() || input.transaction_id.0 == 0 {
        return Err(DirectSurfaceLiquidError::Identity(
            "ingress transaction mismatch",
        ));
    }
    if input.interval_s.to_bits() != INTERVAL_S.to_bits() {
        return Err(production_binding_failure(
            input.transaction_id,
            None,
            "ingress cadence is not exactly 1800 seconds",
        ));
    }
    validate_resource_working_state(configuration, resource)?;
    validate_cadence(resource.beginning_state(), input)?;
    let parameters =
        validate_parameters(configuration, input.transaction_id, &input.wb14_parameters)?;
    let mut pending = validate_and_build_local_ingress(configuration, resource, input)?;
    let mut ending = resource.working_state().clone();
    let mut receipts = Vec::new();
    let mut ledgers = Vec::new();
    let mut call_count = BTreeMap::new();

    for ofe_id in &configuration.ofe_topology {
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
        let beginning_continuation = &resource.beginning_state().continuations[continuation_index];
        let (cumulative_supply_m, cumulative_infiltration_m) =
            continuation_start(beginning_continuation, input)?;
        let ofe_parcels = pending.remove(ofe_id).unwrap_or_default();
        let parameter = parameters
            .get(ofe_id)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "missing WB14 parameters",
            ))?;
        let advanced = advance_one_ofe(
            configuration,
            &mut ending,
            ofe_id,
            ofe_parcels,
            parameter,
            cumulative_supply_m,
            cumulative_infiltration_m,
            input.transaction_id,
        )?;
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
        route_runoff(
            configuration,
            ofe_id,
            advanced.runoff,
            &mut pending,
            &mut receipts,
            input.transaction_id,
        )?;
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
    ending.state_sha256 = ending.recomputed_sha256()?;
    ending.validate(configuration)?;
    let closure_operands = capture_and_validate_surface_liquid_closure(
        configuration,
        resource,
        input,
        &ending,
        &receipts,
    )?;
    Ok(DirectSurfaceLiquidIngressCandidate {
        transaction_id: input.transaction_id,
        beginning_state: resource.beginning_state().clone(),
        ending_state: ending,
        receipts,
        ledgers,
        wb14_calls_by_ofe: call_count,
        closure_operands,
    })
}

fn validate_resource_working_state(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
) -> Result<(), DirectSurfaceLiquidError> {
    if resource.working_state().owner_id != resource.beginning_state().owner_id
        || resource.working_state().configuration_sha256
            != resource.beginning_state().configuration_sha256
        || resource.working_state().records.len() != configuration.records.len()
        || resource.working_state().continuations != resource.beginning_state().continuations
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "resource working-state identity mismatch",
        ));
    }
    for ((working, beginning), configured) in resource
        .working_state()
        .records
        .iter()
        .zip(&resource.beginning_state().records)
        .zip(&configuration.records)
    {
        if working.key != configured.key
            || beginning.key != configured.key
            || working.last_accepted_transaction_id != beginning.last_accepted_transaction_id
            || !working.liquid_kg_m2_tile.is_finite()
            || working.liquid_kg_m2_tile < 0.0
            || working.liquid_kg_m2_tile > configured.capacity_kg_m2_tile
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "invalid resource working-state record",
            ));
        }
    }
    Ok(())
}

fn validate_cadence(
    beginning: &DirectSurfaceLiquidOwnedState,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<(), DirectSurfaceLiquidError> {
    let initial = beginning
        .records
        .first()
        .ok_or(DirectSurfaceLiquidError::Schema(
            "empty surface-liquid state",
        ))?
        .last_accepted_transaction_id
        .is_none();
    for continuation in &beginning.continuations {
        if (initial
            && (continuation.next_interval_index != 0
                || continuation.cumulative_supply_m.to_bits() != 0.0_f64.to_bits()
                || continuation.cumulative_infiltration_m.to_bits() != 0.0_f64.to_bits()))
            || (!initial && continuation.next_interval_index == 0)
        {
            return Err(production_binding_failure(
                input.transaction_id,
                Some(continuation.ofe_id.clone()),
                "initial or accepted WB14 continuation mismatch",
            ));
        }
        let expected = if continuation.next_interval_index == 48 {
            (continuation.day_index.checked_add(1), 0)
        } else {
            (
                Some(continuation.day_index),
                continuation.next_interval_index,
            )
        };
        if expected.0 != Some(input.day_index) || expected.1 != input.interval_index {
            return Err(production_binding_failure(
                input.transaction_id,
                Some(continuation.ofe_id.clone()),
                "WB14 day or interval continuation mismatch",
            ));
        }
    }
    Ok(())
}

fn continuation_start(
    beginning: &super::surface_liquid_owner::DirectSurfaceLiquidContinuationState,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<(f64, f64), DirectSurfaceLiquidError> {
    if beginning.next_interval_index == 48 {
        if input.interval_index != 0 || input.day_index != beginning.day_index + 1 {
            return Err(production_binding_failure(
                input.transaction_id,
                Some(beginning.ofe_id.clone()),
                "invalid WB14 day rollover",
            ));
        }
        Ok((0.0, 0.0))
    } else {
        Ok((
            beginning.cumulative_supply_m,
            beginning.cumulative_infiltration_m,
        ))
    }
}

fn validate_parameters<'a>(
    configuration: &DirectSurfaceLiquidConfiguration,
    transaction_id: TransactionId,
    rows: &'a [DirectOfeWb14Parameters],
) -> Result<BTreeMap<OfeId, &'a DirectOfeWb14Parameters>, DirectSurfaceLiquidError> {
    if rows.len() != configuration.ofe_topology.len() {
        return Err(production_binding_failure(
            transaction_id,
            None,
            "WB14 parameter cardinality mismatch",
        ));
    }
    let mut result = BTreeMap::new();
    for (row, expected) in rows.iter().zip(&configuration.ofe_topology) {
        if &row.ofe_id != expected || result.insert(row.ofe_id.clone(), row).is_some() {
            return Err(production_binding_failure(
                transaction_id,
                Some(row.ofe_id.clone()),
                "WB14 parameter order or identity mismatch",
            ));
        }
        require_positive(row.effective_conductivity_m_s, "effective conductivity")?;
        require_nonnegative(row.matric_potential_m, "matric potential")?;
        require_nonnegative(
            row.infiltration_storage_capacity_m,
            "infiltration storage capacity",
        )?;
    }
    Ok(result)
}

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
        let id = format!(
            "condensation:{}:{:?}:{:?}",
            input.transaction_id.0, overflow.store_key.ofe_id, overflow.store_key.tile_id
        );
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
                end_s: INTERVAL_S,
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
            parcels,
        ),
        DirectTileGroundIngress::CoveredCanopyRelease { release, .. } => {
            append_amount(
                configured,
                DirectSurfaceLiquidParcelKind::CanopyThroughfall,
                &release.throughfall,
                true,
                transaction_id,
                parcels,
            )?;
            append_amount(
                configured,
                DirectSurfaceLiquidParcelKind::CanopyInitialDrainage,
                &release.initial_drainage,
                true,
                transaction_id,
                parcels,
            )?;
            append_amount(
                configured,
                DirectSurfaceLiquidParcelKind::CanopySecondDrainage,
                &release.second_drainage,
                true,
                transaction_id,
                parcels,
            )?;
            append_amount(
                configured,
                DirectSurfaceLiquidParcelKind::CanopyStemflow,
                &release.stemflow,
                true,
                transaction_id,
                parcels,
            )
        }
    }
}

fn append_amount(
    configured: &DirectSurfaceLiquidConfigurationRecord,
    kind: DirectSurfaceLiquidParcelKind,
    amount: &DirectIngressAmount,
    require_full_interval: bool,
    transaction_id: TransactionId,
    parcels: &mut Vec<TimedParcel>,
) -> Result<(), DirectSurfaceLiquidError> {
    amount.validate(require_full_interval)?;
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
    let id = format!(
        "local:{:?}:{:?}:{kind:?}",
        configured.key.ofe_id, configured.key.tile_id
    );
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
    boundaries.extend([0.0, INTERVAL_S]);
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

    for boundary in boundaries.windows(2) {
        let start_s = boundary[0];
        let end_s = boundary[1];
        if end_s <= start_s {
            continue;
        }
        let mut contributions = Vec::new();
        for parcel in parcels
            .iter()
            .filter(|parcel| parcel.start_s <= start_s && parcel.end_s >= end_s)
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
            let mass = checked_surface_liquid_mul(parcel.mass_kg_m2_basis_ofe_ground, fraction)
                .ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        arithmetic_key,
                        Some(parcel.parcel_id.clone()),
                        "parcel interval mass is nonfinite or underflowed",
                    )
                })?;
            let enthalpy =
                checked_surface_liquid_mul(parcel.enthalpy_j_m2_basis_ofe_ground, fraction)
                    .ok_or_else(|| {
                        ingress_arithmetic_failure(
                            transaction_id,
                            arithmetic_key,
                            Some(parcel.parcel_id.clone()),
                            "parcel interval enthalpy is nonfinite or underflowed",
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
        let outcome = advance_wb14_continuation_interval(DirectWb14ContinuationIntervalInputs {
            cumulative_supply_m,
            cumulative_infiltration_m,
            interval_supply_m,
            interval_duration_s: duration_s,
            effective_conductivity_m_s: parameter.effective_conductivity_m_s,
            matric_potential_m: parameter.matric_potential_m,
            storage_capacity_m: parameter.infiltration_storage_capacity_m,
        })
        .map_err(|_| {
            production_binding_failure(
                transaction_id,
                Some(ofe_id.clone()),
                "WB14 continuation transition rejected",
            )
        })?;
        cumulative_supply_m = outcome.cumulative_supply_m;
        cumulative_infiltration_m = outcome.cumulative_infiltration_m;
        if supply_mass == 0.0 {
            continue;
        }
        let total_infiltration =
            checked_surface_liquid_mul(outcome.interval_infiltration_m, WATER_DENSITY_KG_M3)
                .ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        arithmetic_key,
                        None,
                        "infiltration depth-to-mass conversion is nonfinite or underflowed",
                    )
                })?;
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
        let contribution_count = contributions.len();
        let mut excess_parts = Vec::with_capacity(contribution_count);
        for (index, (parcel, mass, _)) in contributions.into_iter().enumerate() {
            let infiltrated = if index + 1 == contribution_count {
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
            let infiltration_q =
                checked_surface_liquid_mul(infiltrated, h_mix).ok_or_else(|| {
                    ingress_arithmetic_failure(
                        transaction_id,
                        arithmetic_key,
                        Some(parcel.parcel_id.clone()),
                        "infiltration enthalpy is nonfinite or underflowed",
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
            excess_parts.push((parcel, excess));
        }
        let (retained_parts, runoff_parts) = retain_excess_proportionally(
            configuration,
            ending,
            excess_parts,
            h_mix,
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
    })
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
fn retain_excess_proportionally(
    configuration: &DirectSurfaceLiquidConfiguration,
    ending: &mut DirectSurfaceLiquidOwnedState,
    excess_parts: Vec<(&TimedParcel, f64)>,
    h_mix: f64,
    temperature_k: f64,
    start_s: f64,
    end_s: f64,
    transaction_id: TransactionId,
) -> Result<(Vec<DirectSurfaceLiquidParcelReceipt>, Vec<TimedParcel>), DirectSurfaceLiquidError> {
    let mut grouped = BTreeMap::<DirectSurfaceLiquidStoreKey, Vec<(&TimedParcel, f64)>>::new();
    for (parcel, excess) in excess_parts {
        grouped
            .entry(parcel.recipient_store_key.clone())
            .or_default()
            .push((parcel, excess));
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
        for (part_index, (parcel, excess)) in parts.into_iter().enumerate() {
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
                    checked_surface_liquid_mul(retained_mass, h_mix).ok_or_else(|| {
                        ingress_arithmetic_failure(
                            transaction_id,
                            &store_key,
                            Some(parcel.parcel_id.clone()),
                            "retained enthalpy is nonfinite or underflowed",
                        )
                    })?,
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
                    enthalpy_j_m2_basis_ofe_ground: checked_surface_liquid_mul(runoff_mass, h_mix)
                        .ok_or_else(|| {
                            ingress_arithmetic_failure(
                                transaction_id,
                                &store_key,
                                Some(parcel.parcel_id.clone()),
                                "runoff enthalpy is nonfinite or underflowed",
                            )
                        })?,
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
    left.start_s
        .total_cmp(&right.start_s)
        .then_with(|| left.end_s.total_cmp(&right.end_s))
        .then_with(|| left.origin_store_key.cmp(&right.origin_store_key))
        .then_with(|| left.kind.cmp(&right.kind))
        .then_with(|| left.parcel_id.cmp(&right.parcel_id))
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
mod tests {
    use openwepp_kernel_contract::{ResourceOwnerId, TileId};
    use openwepp_land_surface_energy::{
        CondensationCredit, SourceId, StandGroundWaterAmountBasis, SurfaceClass,
    };

    use super::*;
    use crate::direct_runtime::{
        DirectSurfaceLiquidOfeBinding, apply_surface_liquid_resource_phase,
        authorize_surface_liquid_withdrawals,
    };

    fn owner(value: &str) -> ResourceOwnerId {
        ResourceOwnerId::try_new(value).expect("owner")
    }

    fn ofe(value: &str) -> OfeId {
        OfeId::try_new(value).expect("OFE")
    }

    fn tile(value: &str) -> TileId {
        TileId::try_new(value).expect("tile")
    }

    fn surface(value: &str) -> SurfaceId {
        SurfaceId::try_new(value).expect("surface")
    }

    fn source(value: &str) -> SourceId {
        SourceId::try_new(value).expect("source")
    }

    fn binding(ofe_name: &str, lane_index: usize) -> DirectSurfaceLiquidOfeBinding {
        let top_layer = SoilLayerId::try_new(format!("soil-{ofe_name}-top")).expect("soil layer");
        DirectSurfaceLiquidOfeBinding {
            ofe_id: ofe(ofe_name),
            production_lane_index: lane_index,
            production_lane_id: u32::try_from(lane_index + 1).expect("lane id"),
            ordered_soil_layer_ids: vec![top_layer.clone()],
            infiltration_soil_thermal_layer_id: top_layer,
        }
    }

    fn config_record(
        ofe_name: &str,
        tile_name: &str,
        area: f64,
        capacity: f64,
        mode: DirectGroundIngressMode,
        destination: Option<(&str, &str)>,
    ) -> DirectSurfaceLiquidConfigurationRecord {
        DirectSurfaceLiquidConfigurationRecord {
            key: DirectSurfaceLiquidStoreKey {
                run_id: 91,
                ofe_id: ofe(ofe_name),
                tile_id: tile(tile_name),
                surface_id: surface(&format!("surface-{tile_name}")),
                surface_class: SurfaceClass::BareMineralSoil,
                source_type: openwepp_land_surface_energy::WaterSourceType::SurfaceLiquid,
                source_id: source(&format!("source-{tile_name}")),
            },
            tile_fraction: 1.0,
            capacity_kg_m2_tile: capacity,
            ofe_area_m2: area,
            ground_ingress_mode: mode,
            runon_destination_ofe_id: destination.map(|row| ofe(row.0)),
            runon_destination_tile_id: destination.map(|row| tile(row.1)),
        }
    }

    fn routed_configuration() -> DirectSurfaceLiquidConfiguration {
        DirectSurfaceLiquidConfiguration::new(
            owner("surface-water"),
            91,
            vec![ofe("upper"), ofe("lower")],
            vec![binding("upper", 0), binding("lower", 1)],
            vec![
                config_record(
                    "upper",
                    "upper-tile",
                    100.0,
                    0.1,
                    DirectGroundIngressMode::OpenRawPrecipitation,
                    Some(("lower", "lower-tile")),
                ),
                config_record(
                    "lower",
                    "lower-tile",
                    200.0,
                    0.1,
                    DirectGroundIngressMode::OpenRawPrecipitation,
                    None,
                ),
            ],
        )
        .expect("configuration")
    }

    fn one_tile_configuration(mode: DirectGroundIngressMode) -> DirectSurfaceLiquidConfiguration {
        DirectSurfaceLiquidConfiguration::new(
            owner("surface-water"),
            91,
            vec![ofe("only")],
            vec![binding("only", 0)],
            vec![config_record("only", "tile", 100.0, 0.1, mode, None)],
        )
        .expect("configuration")
    }

    fn initial_state(
        configuration: &DirectSurfaceLiquidConfiguration,
        fraction_of_capacity: f64,
    ) -> DirectSurfaceLiquidOwnedState {
        let liquid = configuration
            .records
            .iter()
            .map(|record| {
                (
                    record.key.clone(),
                    fraction_of_capacity * record.capacity_kg_m2_tile,
                )
            })
            .collect();
        DirectSurfaceLiquidOwnedState::new_initial(configuration, &liquid, 3)
            .expect("initial state")
    }

    fn resource_candidate(
        configuration: &DirectSurfaceLiquidConfiguration,
        beginning: &DirectSurfaceLiquidOwnedState,
        transaction_id: TransactionId,
        predecessor: Option<TransactionId>,
        condensation: &[CondensationCredit],
    ) -> DirectSurfaceLiquidResourceCandidate {
        let arbitration = authorize_surface_liquid_withdrawals(
            configuration,
            beginning,
            transaction_id,
            predecessor,
            &[],
        )
        .expect("empty authorization");
        apply_surface_liquid_resource_phase(configuration, &arbitration, &[], condensation)
            .expect("resource candidate")
    }

    fn amount(mass: f64, temperature_k: f64, start_s: f64, end_s: f64) -> DirectIngressAmount {
        DirectIngressAmount {
            mass_kg_m2_tile_ground: mass,
            temperature_k,
            specific_liquid_enthalpy_j_kg: liquid_specific_enthalpy(temperature_k),
            start_s,
            end_s,
        }
    }

    fn open_ingress(
        record: &DirectSurfaceLiquidConfigurationRecord,
        mass: f64,
    ) -> DirectTileGroundIngress {
        DirectTileGroundIngress::OpenRawPrecipitation {
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            surface_id: record.key.surface_id.clone(),
            raw_precipitation: amount(mass, 285.0, 0.0, INTERVAL_S),
        }
    }

    fn parameters(
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Vec<DirectOfeWb14Parameters> {
        configuration
            .ofe_topology
            .iter()
            .map(|ofe_id| DirectOfeWb14Parameters {
                ofe_id: ofe_id.clone(),
                effective_conductivity_m_s: 1.0e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 0.0,
            })
            .collect()
    }

    #[test]
    fn unequal_area_runoff_routes_once_and_preserves_mass_and_enthalpy() {
        let configuration = routed_configuration();
        let beginning = initial_state(&configuration, 1.0);
        let transaction_id = TransactionId(201);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![
                open_ingress(&configuration.records[0], 1.0),
                open_ingress(&configuration.records[1], 0.0),
            ],
            wb14_parameters: parameters(&configuration),
        };
        let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("routed candidate");
        assert_eq!(candidate.wb14_calls_by_ofe.values().copied().sum::<u8>(), 2);
        assert!(
            candidate
                .ending_state
                .records
                .iter()
                .zip(&beginning.records)
                .all(|(ending, start)| ending.liquid_kg_m2_tile.to_bits()
                    == start.liquid_kg_m2_tile.to_bits())
        );
        assert_eq!(
            candidate.ending_state.continuations[0].next_interval_index,
            1
        );
        assert_eq!(
            candidate.ending_state.continuations[1].next_interval_index,
            1
        );
        let routed = candidate
            .receipts
            .iter()
            .find(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::RoutedRunoff)
            .expect("routed receipt");
        let outlet = candidate
            .receipts
            .iter()
            .find(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::OutletRunoff)
            .expect("outlet receipt");
        assert_eq!(routed.transaction_id, transaction_id);
        assert_eq!(
            routed.mass_kg_m2_basis_ofe_ground.to_bits(),
            1.0_f64.to_bits()
        );
        assert_eq!(
            outlet.mass_kg_m2_basis_ofe_ground.to_bits(),
            0.5_f64.to_bits()
        );
        assert_eq!(outlet.basis_ofe_id, ofe("lower"));
        assert_eq!(
            outlet.enthalpy_j_m2_basis_ofe_ground.to_bits(),
            (routed.enthalpy_j_m2_basis_ofe_ground * 0.5).to_bits()
        );
        candidate
            .ending_state
            .validate(&configuration)
            .expect("ending state");
    }

    #[test]
    fn open_and_covered_ingress_are_structurally_exclusive() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let beginning = initial_state(&configuration, 0.0);
        let transaction_id = TransactionId(202);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let zero = amount(0.0, 280.0, 0.0, INTERVAL_S);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![DirectTileGroundIngress::CoveredCanopyRelease {
                ofe_id: configuration.records[0].key.ofe_id.clone(),
                tile_id: configuration.records[0].key.tile_id.clone(),
                surface_id: configuration.records[0].key.surface_id.clone(),
                release: DirectCanopyLiquidRelease {
                    throughfall: zero.clone(),
                    initial_drainage: zero.clone(),
                    second_drainage: zero.clone(),
                    stemflow: zero,
                },
            }],
            wb14_parameters: parameters(&configuration),
        };
        let before = resource.clone();
        let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect_err("wrong ingress mode");
        let failure = error.failure().expect("canonical failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
        assert_eq!(resource, before);
        assert_eq!(&beginning, resource.beginning_state());
    }

    #[test]
    fn condensation_overflow_becomes_timed_outlet_parcel() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let beginning = initial_state(&configuration, 1.0);
        let transaction_id = TransactionId(203);
        let temperature_k = 286.0;
        let condensation = CondensationCredit {
            transaction_id,
            hydrology_owner_id: configuration.owner_id.clone(),
            ofe_id: configuration.records[0].key.ofe_id.clone(),
            tile_id: configuration.records[0].key.tile_id.clone(),
            surface_id: configuration.records[0].key.surface_id.clone(),
            amount_kg_m2_stand_ground: 0.2,
            amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
            temperature_k,
            specific_liquid_enthalpy_j_kg: liquid_specific_enthalpy(temperature_k),
        };
        let resource = resource_candidate(
            &configuration,
            &beginning,
            transaction_id,
            None,
            &[condensation],
        );
        assert_eq!(resource.condensation_overflow().len(), 1);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.0)],
            wb14_parameters: parameters(&configuration),
        };
        let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("condensation ingress");
        let outlet = candidate
            .receipts
            .iter()
            .find(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::OutletRunoff)
            .expect("overflow outlet");
        assert_eq!(
            outlet.kind,
            DirectSurfaceLiquidParcelKind::CondensationOverflow
        );
        assert_eq!(outlet.start_s.to_bits(), 0.0_f64.to_bits());
        assert_eq!(outlet.end_s.to_bits(), INTERVAL_S.to_bits());
        assert!(
            (outlet.mass_kg_m2_basis_ofe_ground - 0.2).abs()
                <= mass_tolerance(outlet.mass_kg_m2_basis_ofe_ground.abs() + 0.2)
        );
        assert_eq!(
            candidate.ending_state.records[0]
                .liquid_kg_m2_tile
                .to_bits(),
            0.1_f64.to_bits()
        );
    }

    #[test]
    fn simultaneous_canopy_sources_share_retention_proportionally() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::CoveredCanopyRelease);
        let beginning = initial_state(&configuration, 0.5);
        let transaction_id = TransactionId(204);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let zero = amount(0.0, 284.0, 0.0, INTERVAL_S);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![DirectTileGroundIngress::CoveredCanopyRelease {
                ofe_id: configuration.records[0].key.ofe_id.clone(),
                tile_id: configuration.records[0].key.tile_id.clone(),
                surface_id: configuration.records[0].key.surface_id.clone(),
                release: DirectCanopyLiquidRelease {
                    throughfall: amount(0.2, 280.0, 0.0, INTERVAL_S),
                    initial_drainage: amount(0.3, 290.0, 0.0, INTERVAL_S),
                    second_drainage: zero.clone(),
                    stemflow: zero,
                },
            }],
            wb14_parameters: parameters(&configuration),
        };
        let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("covered ingress");
        let retained = candidate
            .receipts
            .iter()
            .filter(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::RetainedSurface)
            .map(|row| (row.kind, row.mass_kg_m2_basis_ofe_ground))
            .collect::<BTreeMap<_, _>>();
        assert!((retained[&DirectSurfaceLiquidParcelKind::CanopyThroughfall] - 0.02).abs() < 1e-14);
        assert!(
            (retained[&DirectSurfaceLiquidParcelKind::CanopyInitialDrainage] - 0.03).abs() < 1e-14
        );
        assert!((candidate.ending_state.records[0].liquid_kg_m2_tile - 0.1).abs() < 1e-14);
    }

    #[test]
    fn continuation_advances_48_intervals_then_requires_new_day_zero() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let mut state = initial_state(&configuration, 0.0);
        let mut predecessor = None;
        for interval in 0_u8..48 {
            let transaction_id = TransactionId(300 + u128::from(interval));
            let resource =
                resource_candidate(&configuration, &state, transaction_id, predecessor, &[]);
            let input = DirectSurfaceLiquidIngressInput {
                transaction_id,
                day_index: 3,
                interval_index: interval,
                interval_s: INTERVAL_S,
                tile_ingress: vec![open_ingress(&configuration.records[0], 0.0)],
                wb14_parameters: parameters(&configuration),
            };
            let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
                .expect("daily continuation");
            state = candidate.ending_state;
            predecessor = Some(transaction_id);
        }
        assert_eq!(state.continuations[0].next_interval_index, 48);
        let transaction_id = TransactionId(400);
        let resource = resource_candidate(&configuration, &state, transaction_id, predecessor, &[]);
        let stale_day = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.0)],
            wb14_parameters: parameters(&configuration),
        };
        assert!(execute_surface_liquid_ingress(&configuration, &resource, &stale_day).is_err());
        let next_day = DirectSurfaceLiquidIngressInput {
            day_index: 4,
            ..stale_day
        };
        let candidate = execute_surface_liquid_ingress(&configuration, &resource, &next_day)
            .expect("new-day continuation");
        assert_eq!(candidate.ending_state.continuations[0].day_index, 4);
        assert_eq!(
            candidate.ending_state.continuations[0].next_interval_index,
            1
        );
        assert_eq!(
            candidate.ending_state.continuations[0]
                .cumulative_supply_m
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            candidate.ending_state.continuations[0]
                .cumulative_infiltration_m
                .to_bits(),
            0.0_f64.to_bits()
        );
    }

    #[test]
    fn independent_closure_rejects_wrong_infiltration_recipient() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let beginning = initial_state(&configuration, 0.0);
        let transaction_id = TransactionId(390);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
            wb14_parameters: parameters(&configuration),
        };
        let mut candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("valid candidate");
        let infiltration = candidate
            .receipts
            .iter_mut()
            .find(|receipt| {
                receipt.disposition == DirectSurfaceLiquidReceiptDisposition::Infiltration
            })
            .expect("infiltration receipt");
        if let DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
            production_lane_id, ..
        } = &mut infiltration.recipient
        {
            *production_lane_id += 100;
        } else {
            panic!("wrong receipt variant");
        }
        let offending = candidate
            .receipts
            .iter()
            .find(|receipt| {
                receipt.disposition == DirectSurfaceLiquidReceiptDisposition::Infiltration
            })
            .expect("offending infiltration receipt")
            .clone();
        let expected_attempted = candidate
            .ending_state
            .recomputed_sha256()
            .expect("attempted digest");
        let error = candidate
            .validate(&configuration, &resource, &input)
            .expect_err("wrong infiltration recipient");
        let failure = error.failure().expect("canonical candidate failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E009);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
        assert_eq!(failure.context.transaction_id, Some(transaction_id));
        assert_eq!(
            failure.context.owner_id,
            Some(configuration.owner_id.clone())
        );
        assert_eq!(
            failure.context.ofe_id,
            Some(offending.recipient_store_key.ofe_id.clone())
        );
        assert_eq!(
            failure.context.tile_id,
            Some(offending.recipient_store_key.tile_id.clone())
        );
        assert_eq!(failure.context.parcel_id, Some(offending.parcel_id));
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(resource.beginning_state().state_sha256.as_str())
        );
        assert_eq!(
            failure.rollback.attempted_owner_sha256.as_deref(),
            Some(expected_attempted.as_str())
        );
    }

    #[test]
    fn independent_closure_rejects_poisoned_source_operand() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let beginning = initial_state(&configuration, 0.0);
        let transaction_id = TransactionId(391);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
            wb14_parameters: parameters(&configuration),
        };
        let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("valid candidate");
        let mut poisoned = candidate.closure_operands().clone();
        poisoned.poison_first_beginning_for_test();
        let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            &configuration,
            &resource,
            &poisoned,
            &candidate.receipts,
        )
        .expect_err("ordinary closure mismatch");
        assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);

        let mut producer_poison = candidate;
        producer_poison.closure_operands = poisoned;
        let expected_attempted = producer_poison
            .ending_state
            .recomputed_sha256()
            .expect("attempted digest");
        let error = producer_poison
            .validate(&configuration, &resource, &input)
            .expect_err("public producer mismatch");
        let failure = error.failure().expect("canonical public closure failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E010);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
        assert_eq!(failure.context.transaction_id, Some(transaction_id));
        assert_eq!(
            failure.context.owner_id,
            Some(configuration.owner_id.clone())
        );
        assert_eq!(
            failure.context.ofe_id,
            Some(configuration.records[0].key.ofe_id.clone())
        );
        assert_eq!(
            failure.context.tile_id,
            Some(configuration.records[0].key.tile_id.clone())
        );
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(resource.beginning_state().state_sha256.as_str())
        );
        assert_eq!(
            failure.rollback.attempted_owner_sha256.as_deref(),
            Some(expected_attempted.as_str())
        );
    }

    #[test]
    fn independent_closure_rejects_large_finite_store_arithmetic_overflow() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let beginning = initial_state(&configuration, 0.0);
        let transaction_id = TransactionId(392);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
            wb14_parameters: parameters(&configuration),
        };
        let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("valid candidate");
        let mut poisoned = candidate.closure_operands().clone();
        poisoned.poison_first_store_arithmetic_overflow_for_test();
        let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            &configuration,
            &resource,
            &poisoned,
            &candidate.receipts,
        )
        .expect_err("large finite closure arithmetic must fail closed");
        assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);

        let mut producer_poison = candidate.clone();
        producer_poison
            .closure_operands
            .poison_first_store_arithmetic_overflow_for_test();
        let expected_attempted = producer_poison
            .ending_state
            .recomputed_sha256()
            .expect("attempted digest");
        let error = producer_poison
            .validate(&configuration, &resource, &input)
            .expect_err("public producer closure must retain arithmetic failure");
        let failure = error.failure().expect("canonical public closure failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
        assert_eq!(failure.context.transaction_id, Some(transaction_id));
        assert_eq!(
            failure.context.owner_id,
            Some(configuration.owner_id.clone())
        );
        assert_eq!(
            failure.context.ofe_id,
            Some(configuration.records[0].key.ofe_id.clone())
        );
        assert_eq!(
            failure.context.tile_id,
            Some(configuration.records[0].key.tile_id.clone())
        );
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(resource.beginning_state().state_sha256.as_str())
        );
        assert_eq!(
            failure.rollback.attempted_owner_sha256.as_deref(),
            Some(expected_attempted.as_str())
        );
    }

    #[test]
    fn arithmetic_preflight_finds_later_store_e003_after_earlier_finite_mismatch() {
        let configuration = routed_configuration();
        let beginning = initial_state(&configuration, 0.0);
        let transaction_id = TransactionId(395);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: configuration
                .records
                .iter()
                .map(|record| open_ingress(record, 0.1))
                .collect(),
            wb14_parameters: parameters(&configuration),
        };
        let mut candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("two-store candidate");
        candidate
            .closure_operands
            .poison_first_finite_and_last_arithmetic_for_test();
        let attempted = candidate
            .ending_state
            .recomputed_sha256()
            .expect("attempted digest");
        let error = candidate
            .validate(&configuration, &resource, &input)
            .expect_err("later arithmetic failure must outrank earlier finite mismatch");
        let failure = error.failure().expect("canonical arithmetic failure");
        let last = configuration.records.last().expect("last store");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
        assert_eq!(failure.context.transaction_id, Some(transaction_id));
        assert_eq!(
            failure.context.owner_id,
            Some(configuration.owner_id.clone())
        );
        assert_eq!(failure.context.ofe_id, Some(last.key.ofe_id.clone()));
        assert_eq!(failure.context.tile_id, Some(last.key.tile_id.clone()));
        assert_eq!(failure.context.parcel_id, None);
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(resource.beginning_state().state_sha256.as_str())
        );
        assert_eq!(
            failure.rollback.attempted_owner_sha256.as_deref(),
            Some(attempted.as_str())
        );
    }

    #[test]
    fn producer_second_store_ledger_and_wb14_mismatches_report_exact_identity() {
        let configuration = routed_configuration();
        let beginning = initial_state(&configuration, 0.0);
        let transaction_id = TransactionId(396);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: configuration
                .records
                .iter()
                .map(|record| open_ingress(record, 0.1))
                .collect(),
            wb14_parameters: parameters(&configuration),
        };
        let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("two-store candidate");
        let second = &configuration.records[1];

        let mut ending_poison = candidate.clone();
        ending_poison.ending_state.records[1].liquid_kg_m2_tile += 0.01;
        let ending_attempted = ending_poison
            .ending_state
            .recomputed_sha256()
            .expect("ending attempted digest");
        assert_producer_e009(
            &ending_poison
                .validate(&configuration, &resource, &input)
                .expect_err("second ending store mismatch"),
            transaction_id,
            &configuration,
            Some(&second.key.ofe_id),
            Some(&second.key.tile_id),
            None,
            &resource.beginning_state().state_sha256,
            &ending_attempted,
        );

        let mut ledger_poison = candidate.clone();
        ledger_poison.ledgers[1].runoff_mass_kg_m2_ofe_ground += 0.01;
        let ledger_attempted = ledger_poison
            .ending_state
            .recomputed_sha256()
            .expect("ledger attempted digest");
        assert_producer_e009(
            &ledger_poison
                .validate(&configuration, &resource, &input)
                .expect_err("second ledger mismatch"),
            transaction_id,
            &configuration,
            Some(&second.key.ofe_id),
            None,
            None,
            &resource.beginning_state().state_sha256,
            &ledger_attempted,
        );

        let mut wb14_poison = candidate;
        *wb14_poison
            .wb14_calls_by_ofe
            .get_mut(&second.key.ofe_id)
            .expect("second WB14 counter") += 1;
        let wb14_attempted = wb14_poison
            .ending_state
            .recomputed_sha256()
            .expect("WB14 attempted digest");
        assert_producer_e009(
            &wb14_poison
                .validate(&configuration, &resource, &input)
                .expect_err("second WB14 mismatch"),
            transaction_id,
            &configuration,
            Some(&second.key.ofe_id),
            None,
            None,
            &resource.beginning_state().state_sha256,
            &wb14_attempted,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn assert_producer_e009(
        error: &DirectSurfaceLiquidError,
        transaction_id: TransactionId,
        configuration: &DirectSurfaceLiquidConfiguration,
        ofe_id: Option<&OfeId>,
        tile_id: Option<&TileId>,
        parcel_id: Option<&str>,
        beginning_sha256: &str,
        attempted_sha256: &str,
    ) {
        let failure = error.failure().expect("canonical producer failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E009);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
        assert_eq!(failure.context.transaction_id, Some(transaction_id));
        assert_eq!(
            failure.context.owner_id,
            Some(configuration.owner_id.clone())
        );
        assert_eq!(failure.context.ofe_id.as_ref(), ofe_id);
        assert_eq!(failure.context.tile_id.as_ref(), tile_id);
        assert_eq!(failure.context.parcel_id.as_deref(), parcel_id);
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(beginning_sha256)
        );
        assert_eq!(
            failure.rollback.attempted_owner_sha256.as_deref(),
            Some(attempted_sha256)
        );
    }

    #[test]
    fn finite_ingress_enthalpy_overflow_fails_before_candidate() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let beginning = initial_state(&configuration, 0.0);
        let transaction_id = TransactionId(393);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], f64::MAX / 2.0)],
            wb14_parameters: parameters(&configuration),
        };
        let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect_err("finite parcel enthalpy overflow must fail closed");
        assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
    }

    #[test]
    fn finite_routing_area_underflow_fails_before_receipt() {
        let mut configuration = routed_configuration();
        configuration.records[0].ofe_area_m2 = f64::MIN_POSITIVE;
        configuration.records[1].ofe_area_m2 = f64::MAX;
        configuration.configuration_sha256 = configuration.recomputed_sha256().expect("digest");
        configuration.validate().expect("finite extreme areas");
        let source = &configuration.records[0];
        let parcel = TimedParcel {
            parcel_id: "underflow-route".to_owned(),
            origin_store_key: source.key.clone(),
            recipient_store_key: source.key.clone(),
            basis_ofe_id: source.key.ofe_id.clone(),
            kind: DirectSurfaceLiquidParcelKind::RawPrecipitation,
            start_s: 0.0,
            end_s: INTERVAL_S,
            mass_kg_m2_basis_ofe_ground: 1.0,
            enthalpy_j_m2_basis_ofe_ground: 1.0,
        };
        let mut pending = BTreeMap::new();
        let mut receipts = Vec::new();
        let error = route_runoff(
            &configuration,
            &source.key.ofe_id,
            vec![parcel],
            &mut pending,
            &mut receipts,
            TransactionId(394),
        )
        .expect_err("finite area-ratio underflow must fail closed");
        assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
        assert!(pending.is_empty());
        assert!(receipts.is_empty());
    }

    #[test]
    fn wb14_failure_preserves_every_resource_candidate_byte() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let beginning = initial_state(&configuration, 0.0);
        let first_transaction_id = TransactionId(401);
        let first_resource =
            resource_candidate(&configuration, &beginning, first_transaction_id, None, &[]);
        let first = execute_surface_liquid_ingress(
            &configuration,
            &first_resource,
            &DirectSurfaceLiquidIngressInput {
                transaction_id: first_transaction_id,
                day_index: 3,
                interval_index: 0,
                interval_s: INTERVAL_S,
                tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
                wb14_parameters: vec![DirectOfeWb14Parameters {
                    ofe_id: configuration.ofe_topology[0].clone(),
                    effective_conductivity_m_s: 1.0e-6,
                    matric_potential_m: 0.1,
                    infiltration_storage_capacity_m: 1.0,
                }],
            },
        )
        .expect("first continuation");
        assert!(first.ending_state.continuations[0].cumulative_infiltration_m > 0.0);
        let transaction_id = TransactionId(402);
        let resource = resource_candidate(
            &configuration,
            &first.ending_state,
            transaction_id,
            Some(first_transaction_id),
            &[],
        );
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 1,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
            wb14_parameters: vec![DirectOfeWb14Parameters {
                ofe_id: configuration.ofe_topology[0].clone(),
                effective_conductivity_m_s: 1.0e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 0.0,
            }],
        };
        let before_candidate = resource.clone();
        let before = (
            resource
                .beginning_state()
                .canonical_bytes(&configuration)
                .expect("beginning bytes before"),
            resource
                .working_state()
                .canonical_bytes(&configuration)
                .expect("working bytes before"),
        );
        let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect_err("invalid continuation bound");
        let failure = error.failure().expect("canonical failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E008);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
        assert_eq!(failure.context.transaction_id, Some(transaction_id));
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(resource.beginning_state().state_sha256.as_str())
        );
        assert!(failure.rollback.attempted_owner_sha256.is_some());
        let after = (
            resource
                .beginning_state()
                .canonical_bytes(&configuration)
                .expect("beginning bytes after"),
            resource
                .working_state()
                .canonical_bytes(&configuration)
                .expect("working bytes after"),
        );
        assert_eq!(after, before);
        assert_eq!(resource, before_candidate);
    }

    #[test]
    fn cadence_failure_is_e008_with_exact_transaction_and_attempt_hash() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let beginning = initial_state(&configuration, 0.0);
        let transaction_id = TransactionId(411);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S + 1.0,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
            wb14_parameters: parameters(&configuration),
        };
        let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect_err("wrong cadence");
        let failure = error.failure().expect("canonical failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E008);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
        assert_eq!(failure.context.transaction_id, Some(transaction_id));
        assert!(failure.rollback.attempted_owner_sha256.is_some());
    }

    #[test]
    fn sealed_ingress_candidate_reconstructs_and_rejects_forgery() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let beginning = initial_state(&configuration, 0.0);
        let transaction_id = TransactionId(412);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
            wb14_parameters: parameters(&configuration),
        };
        let mut candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("valid candidate");
        candidate
            .validate(&configuration, &resource, &input)
            .expect("candidate reconstruction");
        candidate.ending_state.records[0].liquid_kg_m2_tile += 0.25;
        let error = candidate
            .validate(&configuration, &resource, &input)
            .expect_err("forged ending state");
        let failure = error.failure().expect("canonical failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E009);
        assert_eq!(failure.context.transaction_id, Some(transaction_id));
        assert!(failure.rollback.beginning_owner_sha256.is_some());
        assert!(failure.rollback.attempted_owner_sha256.is_some());
    }
}
