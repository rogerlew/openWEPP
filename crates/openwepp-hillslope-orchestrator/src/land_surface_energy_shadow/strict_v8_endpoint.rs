//! Sole public composition boundary for the strict snow-free V8/LSE shadow.
//! Binding: `SC-LANDSURFACEENERGY-001` and `SC-VEGETATION-001`.

use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
};

use openwepp_biogeochemistry::BiogeochemistryState;
use openwepp_kernel_contract::ResourceOwnerId;
use openwepp_land_surface_energy::{
    GroundWaterKey, LandSurfaceEnergyConfiguration, LandSurfaceEnergyState, LandSurfaceForcing,
    LiquidParcel, Sha256Digest, SoilThermalSnapshot,
};
use openwepp_vegetation::{
    NitrogenArbiter, V8CoupledOwnedState, V8PersistentForcingReceipt, VegetationConfiguration,
};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{
    DirectGroundIngressMode, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectOpenLiquidIngressParcel, DirectSurfaceLiquidParcelKind, DirectTileGroundIngress,
};

use super::covered_v8_owner::{V8OwnerFailurePhase, construct_multi_tile_v8_owner_envelope};
use super::multi_tile_runtime::{
    MultiTileFailurePhase, PendingPayloadKind, StrictProjectedCoveredTile, StrictProjectedOpenTile,
    StrictProjectedTileProblem, execute_multi_tile_runtime,
};
use super::v8_input_projection::{V8SolverReadyTilePhysics, project_v8_runtime_inputs};
use super::v8_projection::project_multi_tile_v8_passes;
use super::{
    CoveredIngressSchedule, CoveredV8OwnerEnvelopeError, DirectSurfaceLiquidConfiguration,
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError, RealHydrologySourceKey,
    UncommittedCoveredV8OwnerEnvelope, UnifiedReceiverExpectations, V8CanopyForcingReceipt,
    V8InputProjectionError, V8ProjectionError,
};

#[derive(Clone, Debug, Error, PartialEq)]
pub enum ExecuteV8LseRuntimeShadowError {
    #[error("strict V8 endpoint identity failure: {0}")]
    Identity(&'static str),
    #[error(transparent)]
    Projection(#[from] V8InputProjectionError),
    #[error(transparent)]
    Physical(#[from] LandSurfaceEnergyShadowError),
    #[error(transparent)]
    V8Projection(#[from] V8ProjectionError),
    #[error(transparent)]
    Owner(#[from] CoveredV8OwnerEnvelopeError),
    #[error(transparent)]
    Rollback(#[from] super::V8RollbackError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum V8EndpointFailureInjection {
    AfterProjection,
    AfterSolverReady,
    AfterPotentialTile(usize),
    AfterCombinedRequests,
    AfterAuthorization,
    AfterFinalTile(usize),
    AfterE04Ingress,
    AfterOpenIngress,
    AfterUnifiedHydrology,
    AfterLocalEnergy,
    AfterOfeEnergy,
    AfterV8Receipts,
    AfterPersistentPhase,
    AfterVegetationCandidate,
    AfterBiogeochemistryCandidate,
    AfterEnvelopeValidation,
    BeforeReturn,
}

#[derive(Default)]
pub(super) struct PendingEndpointEnvelopes {
    protocol: Vec<u8>,
    ingress: Vec<u8>,
    diagnostic: Vec<u8>,
    actual_payload_count: usize,
}

/// Execute the complete configured heterogeneous tile set from canonical owner
/// configuration, beginning state, forcing, and immutable owner snapshots.
/// The returned envelope is sealed but deliberately has no commit operation.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn execute_v8_lse_runtime_shadow(
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    vegetation_owner_id: &ResourceOwnerId,
    canopy_forcing: &V8CanopyForcingReceipt,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    lse_beginning: &LandSurfaceEnergyState,
    lse_forcing: &LandSurfaceForcing,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    day_index: usize,
    interval_index: u8,
    wb14_parameters: &[DirectOfeWb14Parameters],
    soil_thermal: &SoilThermalSnapshot,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
) -> Result<UncommittedCoveredV8OwnerEnvelope, ExecuteV8LseRuntimeShadowError> {
    execute_v8_lse_runtime_shadow_internal(
        vegetation_configuration,
        vegetation_beginning,
        vegetation_owner_id,
        canopy_forcing,
        lse_configuration,
        lse_beginning,
        lse_forcing,
        soil_adapter,
        surface_configuration,
        day_index,
        interval_index,
        wb14_parameters,
        soil_thermal,
        nitrogen,
        biogeochemistry_beginning,
        None,
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn execute_v8_lse_runtime_shadow_internal(
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    vegetation_owner_id: &ResourceOwnerId,
    canopy_forcing: &V8CanopyForcingReceipt,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    lse_beginning: &LandSurfaceEnergyState,
    lse_forcing: &LandSurfaceForcing,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    day_index: usize,
    interval_index: u8,
    wb14_parameters: &[DirectOfeWb14Parameters],
    soil_thermal: &SoilThermalSnapshot,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
    injection: Option<V8EndpointFailureInjection>,
) -> Result<UncommittedCoveredV8OwnerEnvelope, ExecuteV8LseRuntimeShadowError> {
    let biogeochemistry_owner_id = ResourceOwnerId::try_new("biogeochemistry")
        .map_err(|_| ExecuteV8LseRuntimeShadowError::Identity("BGC rollback owner identity"))?;
    let pending_owner_id = ResourceOwnerId::try_new("strict-v8-pending-envelopes")
        .map_err(|_| ExecuteV8LseRuntimeShadowError::Identity("pending rollback owner identity"))?;
    let pending = RefCell::new(PendingEndpointEnvelopes::default());
    let beginning = super::V8RollbackSnapshot::capture_endpoint_beginning(
        vegetation_owner_id,
        vegetation_beginning,
        soil_adapter,
        lse_beginning,
        soil_thermal,
        &biogeochemistry_owner_id,
        biogeochemistry_beginning,
        &pending_owner_id,
        &[],
        &[],
        &[],
    )?;
    let result = execute_v8_lse_runtime_shadow_phases(
        vegetation_configuration,
        vegetation_beginning,
        vegetation_owner_id,
        canopy_forcing,
        lse_configuration,
        lse_beginning,
        lse_forcing,
        soil_adapter,
        surface_configuration,
        day_index,
        interval_index,
        wb14_parameters,
        soil_thermal,
        nitrogen,
        biogeochemistry_beginning,
        injection,
        &pending,
    );
    if let Err(error) = &result {
        pending.borrow_mut().diagnostic = error.to_string().into_bytes();
        let dirty = pending.borrow();
        if injection_requires_actual_pending_payload(injection) && dirty.actual_payload_count == 0 {
            return Err(ExecuteV8LseRuntimeShadowError::Identity(
                "injected phase observed no actual pending transaction payload",
            ));
        }
        let dirty_snapshot = super::V8RollbackSnapshot::capture_endpoint_beginning(
            vegetation_owner_id,
            vegetation_beginning,
            soil_adapter,
            lse_beginning,
            soil_thermal,
            &biogeochemistry_owner_id,
            biogeochemistry_beginning,
            &pending_owner_id,
            &dirty.protocol,
            &dirty.ingress,
            &dirty.diagnostic,
        )?;
        if beginning.check_snapshot(&dirty_snapshot)
            != Err(super::V8RollbackError::OwnerMutation {
                owner_id: pending_owner_id.clone(),
            })
        {
            return Err(ExecuteV8LseRuntimeShadowError::Identity(
                "failed phase pending bytes were not actual owner mutation",
            ));
        }
        drop(dirty);
        *pending.borrow_mut() = PendingEndpointEnvelopes::default();
        let after = super::V8RollbackSnapshot::capture_endpoint_beginning(
            vegetation_owner_id,
            vegetation_beginning,
            soil_adapter,
            lse_beginning,
            soil_thermal,
            &biogeochemistry_owner_id,
            biogeochemistry_beginning,
            &pending_owner_id,
            &[],
            &[],
            &[],
        )?;
        beginning.check_snapshot(&after)?;
    }
    result
}

fn injection_requires_actual_pending_payload(
    injection: Option<V8EndpointFailureInjection>,
) -> bool {
    matches!(
        injection,
        Some(
            V8EndpointFailureInjection::AfterCombinedRequests
                | V8EndpointFailureInjection::AfterAuthorization
                | V8EndpointFailureInjection::AfterFinalTile(_)
                | V8EndpointFailureInjection::AfterE04Ingress
                | V8EndpointFailureInjection::AfterOpenIngress
                | V8EndpointFailureInjection::AfterUnifiedHydrology
                | V8EndpointFailureInjection::AfterLocalEnergy
                | V8EndpointFailureInjection::AfterOfeEnergy
                | V8EndpointFailureInjection::AfterV8Receipts
                | V8EndpointFailureInjection::AfterPersistentPhase
                | V8EndpointFailureInjection::AfterVegetationCandidate
                | V8EndpointFailureInjection::AfterBiogeochemistryCandidate
                | V8EndpointFailureInjection::AfterEnvelopeValidation
                | V8EndpointFailureInjection::BeforeReturn
        )
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn execute_v8_lse_runtime_shadow_phases(
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    vegetation_owner_id: &ResourceOwnerId,
    canopy_forcing: &V8CanopyForcingReceipt,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    lse_beginning: &LandSurfaceEnergyState,
    lse_forcing: &LandSurfaceForcing,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    day_index: usize,
    interval_index: u8,
    wb14_parameters: &[DirectOfeWb14Parameters],
    soil_thermal: &SoilThermalSnapshot,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
    injection: Option<V8EndpointFailureInjection>,
    pending: &RefCell<PendingEndpointEnvelopes>,
) -> Result<UncommittedCoveredV8OwnerEnvelope, ExecuteV8LseRuntimeShadowError> {
    let projected = project_v8_runtime_inputs(
        vegetation_configuration,
        vegetation_beginning,
        vegetation_owner_id,
        &ResourceOwnerId::try_new("biogeochemistry")
            .map_err(|_| ExecuteV8LseRuntimeShadowError::Identity("BGC owner identity"))?,
        &Sha256Digest::try_new(format!(
            "{:x}",
            Sha256::digest(
                serde_json::to_vec(biogeochemistry_beginning)
                    .map_err(|_| ExecuteV8LseRuntimeShadowError::Identity("BGC state bytes"))?
            )
        ))
        .map_err(|_| ExecuteV8LseRuntimeShadowError::Identity("BGC state digest"))?,
        canopy_forcing,
        lse_configuration,
        lse_beginning,
        lse_forcing,
        soil_adapter,
        surface_configuration,
        soil_thermal,
    )?;
    injected(injection, V8EndpointFailureInjection::AfterProjection)?;
    let ingress_schedule = derive_ingress_schedule(
        lse_forcing,
        surface_configuration,
        day_index,
        interval_index,
        wb14_parameters,
    )?;
    let receiver_expectations = receiver_expectations(
        lse_configuration,
        lse_beginning,
        &projected.hydrology_snapshot_sha256,
        soil_thermal,
    )?;
    let solver_ready = projected.solver_ready_tiles(vegetation_owner_id)?;
    injected(injection, V8EndpointFailureInjection::AfterSolverReady)?;
    let mut problems = Vec::with_capacity(solver_ready.len());
    let mut soil_sources = BTreeMap::<GroundWaterKey, RealHydrologySourceKey>::new();
    let mut bindings = Vec::new();
    for tile in solver_ready {
        for (key, source) in tile.soil_sources {
            if soil_sources.insert(key, source).is_some() {
                return Err(ExecuteV8LseRuntimeShadowError::Identity(
                    "duplicate projected soil source",
                ));
            }
        }
        bindings.extend(tile.vegetation_bindings);
        let problem = match tile.physics {
            V8SolverReadyTilePhysics::Open(beginning) => {
                StrictProjectedTileProblem::Open(StrictProjectedOpenTile {
                    identity: tile.identity,
                    beginning,
                    potential_initial_trial: Some(tile.beginning_trial.clone()),
                    final_initial_trial: Some(tile.beginning_trial),
                    soil_thermal: soil_thermal.clone(),
                })
            }
            V8SolverReadyTilePhysics::Covered(beginning) => {
                StrictProjectedTileProblem::Covered(StrictProjectedCoveredTile {
                    identity: tile.identity,
                    beginning,
                    roots: tile.root_identities,
                    potential_initial_trial: tile.beginning_trial.clone(),
                    final_initial_trial: tile.beginning_trial,
                    soil_thermal: soil_thermal.clone(),
                })
            }
        };
        problems.push(problem);
    }
    let runtime_hook = |phase| {
        let endpoint_phase = match phase {
            MultiTileFailurePhase::PotentialTile(index) => {
                V8EndpointFailureInjection::AfterPotentialTile(index)
            }
            MultiTileFailurePhase::CombinedRequests => {
                V8EndpointFailureInjection::AfterCombinedRequests
            }
            MultiTileFailurePhase::Authorization => V8EndpointFailureInjection::AfterAuthorization,
            MultiTileFailurePhase::FinalTile(index) => {
                V8EndpointFailureInjection::AfterFinalTile(index)
            }
            MultiTileFailurePhase::E04Ingress => V8EndpointFailureInjection::AfterE04Ingress,
            MultiTileFailurePhase::OpenIngress => V8EndpointFailureInjection::AfterOpenIngress,
            MultiTileFailurePhase::UnifiedHydrology => {
                V8EndpointFailureInjection::AfterUnifiedHydrology
            }
            MultiTileFailurePhase::LocalEnergy => V8EndpointFailureInjection::AfterLocalEnergy,
            MultiTileFailurePhase::OfeEnergy => V8EndpointFailureInjection::AfterOfeEnergy,
        };
        if injection == Some(endpoint_phase) {
            Err(LandSurfaceEnergyShadowError::Identity(
                "test-injected strict V8 multi-tile phase failure",
            ))
        } else {
            Ok(())
        }
    };
    let runtime_hook_ref = injection.map(|_| {
        &runtime_hook as &dyn Fn(MultiTileFailurePhase) -> Result<(), LandSurfaceEnergyShadowError>
    });
    let pending_hook = |kind: PendingPayloadKind, bytes: &[u8]| {
        let mut journal = pending.borrow_mut();
        let destination = if kind == PendingPayloadKind::Ingress {
            &mut journal.ingress
        } else {
            &mut journal.protocol
        };
        destination.extend_from_slice(&(kind as u8).to_be_bytes());
        destination.extend_from_slice(&(bytes.len() as u64).to_be_bytes());
        destination.extend_from_slice(bytes);
        journal.actual_payload_count += 1;
        Ok(())
    };
    let physical = execute_multi_tile_runtime(
        soil_adapter,
        surface_configuration,
        &receiver_expectations,
        problems,
        &soil_sources,
        &ingress_schedule,
        runtime_hook_ref,
        Some(&pending_hook),
    )?;
    let potentials = physical
        .potential_tiles()
        .iter()
        .filter_map(|value| value.covered())
        .map(|value| &value.potential_vegetation_operands)
        .collect::<Vec<_>>();
    let finals = physical
        .finalized_tiles()
        .iter()
        .filter_map(|value| value.covered())
        .map(|value| &value.vegetation_operands)
        .collect::<Vec<_>>();
    let receipts = project_multi_tile_v8_passes(
        &potentials,
        &finals,
        &bindings,
        physical.hydrology_candidate(),
        vegetation_configuration,
        vegetation_beginning,
    )?;
    injected(injection, V8EndpointFailureInjection::AfterV8Receipts)?;
    let configured_root_layers = vegetation_configuration
        .strata
        .iter()
        .flat_map(|stratum| stratum.root_layers.iter().map(|root| root.layer_id.clone()))
        .collect::<BTreeSet<_>>();
    let persistent_forcing = V8PersistentForcingReceipt {
        model_definition_sha256: vegetation_beginning.model_definition_sha256.clone(),
        configuration_sha256: vegetation_configuration.configuration_sha256.clone(),
        transaction_id: projected.transaction_id,
        vegetation_beginning_state_sha256: vegetation_beginning.state_sha256.clone(),
        air_temperature_k: canopy_forcing.forcing().air_temperature_k,
        gsi: canopy_forcing.forcing().gsi,
        soil_temperature_k_by_layer: canopy_forcing
            .forcing()
            .soil_layers
            .iter()
            .filter(|layer| configured_root_layers.contains(&layer.layer_id))
            .map(|layer| (layer.layer_id.clone(), layer.temperature_k))
            .collect(),
    };
    let owner_hook = |phase| {
        let endpoint_phase = match phase {
            V8OwnerFailurePhase::Persistent => V8EndpointFailureInjection::AfterPersistentPhase,
            V8OwnerFailurePhase::VegetationCandidate => {
                V8EndpointFailureInjection::AfterVegetationCandidate
            }
            V8OwnerFailurePhase::BiogeochemistryCandidate => {
                V8EndpointFailureInjection::AfterBiogeochemistryCandidate
            }
            V8OwnerFailurePhase::EnvelopeValidation => {
                V8EndpointFailureInjection::AfterEnvelopeValidation
            }
        };
        if injection == Some(endpoint_phase) {
            Err(CoveredV8OwnerEnvelopeError::Identity(
                "test-injected strict V8 owner phase failure",
            ))
        } else {
            Ok(())
        }
    };
    let owner_hook_ref = injection.map(|_| {
        &owner_hook as &dyn Fn(V8OwnerFailurePhase) -> Result<(), CoveredV8OwnerEnvelopeError>
    });
    let envelope = construct_multi_tile_v8_owner_envelope(
        physical,
        &receipts,
        vegetation_configuration,
        vegetation_beginning,
        &persistent_forcing,
        nitrogen,
        biogeochemistry_beginning,
        owner_hook_ref,
    )?;
    injected(injection, V8EndpointFailureInjection::BeforeReturn)?;
    Ok(envelope)
}

fn injected(
    actual: Option<V8EndpointFailureInjection>,
    expected: V8EndpointFailureInjection,
) -> Result<(), ExecuteV8LseRuntimeShadowError> {
    if actual == Some(expected) {
        return Err(ExecuteV8LseRuntimeShadowError::Identity(
            "test-injected strict V8 endpoint phase failure",
        ));
    }
    Ok(())
}

fn derive_ingress_schedule(
    forcing: &LandSurfaceForcing,
    configuration: &DirectSurfaceLiquidConfiguration,
    day_index: usize,
    interval_index: u8,
    wb14_parameters: &[DirectOfeWb14Parameters],
) -> Result<CoveredIngressSchedule, ExecuteV8LseRuntimeShadowError> {
    let mut open_tile_ingress = Vec::new();
    let mut covered_runon = BTreeMap::new();
    for record in &configuration.records {
        let destination = (&record.key.ofe_id, &record.key.tile_id);
        let parcels = forcing
            .precipitation_parcels
            .iter()
            .chain(forcing.runon_parcels.iter())
            .filter(|parcel| {
                (&parcel.destination_ofe_id, &parcel.destination_tile_id) == destination
            })
            .collect::<Vec<_>>();
        match record.ground_ingress_mode {
            DirectGroundIngressMode::OpenRawPrecipitation => {
                open_tile_ingress.push(DirectTileGroundIngress::OpenLiquidParcels {
                    ofe_id: record.key.ofe_id.clone(),
                    tile_id: record.key.tile_id.clone(),
                    surface_id: record.key.surface_id.clone(),
                    parcels: parcels
                        .into_iter()
                        .filter(|parcel| parcel.amount_kg_m2_destination_tile_ground != 0.0)
                        .map(strict_open_ingress_parcel)
                        .collect::<Result<Vec<_>, _>>()?,
                });
            }
            DirectGroundIngressMode::CoveredCanopyRelease => {
                let runon_parcels = forcing
                    .runon_parcels
                    .iter()
                    .filter(|parcel| {
                        (&parcel.destination_ofe_id, &parcel.destination_tile_id) == destination
                            && parcel.amount_kg_m2_destination_tile_ground != 0.0
                    })
                    .map(strict_open_ingress_parcel)
                    .collect::<Result<Vec<_>, _>>()?;
                if !runon_parcels.is_empty() {
                    covered_runon.insert(
                        (record.key.ofe_id.clone(), record.key.tile_id.clone()),
                        runon_parcels,
                    );
                }
            }
        }
    }
    for parcel in forcing
        .precipitation_parcels
        .iter()
        .chain(forcing.runon_parcels.iter())
    {
        if !configuration.records.iter().any(|record| {
            record.key.ofe_id == parcel.destination_ofe_id
                && record.key.tile_id == parcel.destination_tile_id
        }) {
            return Err(ExecuteV8LseRuntimeShadowError::Identity(
                "liquid parcel destination is not a configured surface",
            ));
        }
    }
    Ok(CoveredIngressSchedule {
        transaction_id: forcing.transaction_id,
        day_index,
        interval_index,
        interval_s: forcing.interval_s,
        open_tile_ingress,
        covered_runon,
        wb14_parameters: wb14_parameters.to_vec(),
    })
}

fn strict_open_ingress_parcel(
    parcel: &LiquidParcel,
) -> Result<DirectOpenLiquidIngressParcel, ExecuteV8LseRuntimeShadowError> {
    let specific_liquid_enthalpy_j_kg =
        parcel
            .specific_liquid_enthalpy_j_kg
            .ok_or(ExecuteV8LseRuntimeShadowError::Identity(
                "positive open-ingress parcel missing accepted enthalpy",
            ))?;
    let temperature_k = parcel
        .temperature_k
        .ok_or(ExecuteV8LseRuntimeShadowError::Identity(
            "positive open-ingress parcel missing accepted temperature",
        ))?;
    let accepted_source_state_sha256 =
        parcel
            .source_state_sha256
            .clone()
            .ok_or(ExecuteV8LseRuntimeShadowError::Identity(
                "positive open-ingress parcel missing accepted source-state digest",
            ))?;
    let kind = match parcel.parcel_kind {
        openwepp_land_surface_energy::LiquidParcelKind::Precipitation => {
            DirectSurfaceLiquidParcelKind::RawPrecipitation
        }
        openwepp_land_surface_energy::LiquidParcelKind::RoutedRunon => {
            DirectSurfaceLiquidParcelKind::UpstreamRunon
        }
    };
    Ok(DirectOpenLiquidIngressParcel {
        kind,
        parcel_id: parcel.parcel_id.clone(),
        source_owner_id: parcel.source_owner_id.clone(),
        source_ofe_id: parcel.source_ofe_id.clone(),
        source_tile_id: parcel.source_tile_id.clone(),
        destination_ofe_id: parcel.destination_ofe_id.clone(),
        destination_tile_id: parcel.destination_tile_id.clone(),
        accepted_source_state_sha256,
        amount: DirectIngressAmount {
            mass_kg_m2_tile_ground: parcel.amount_kg_m2_destination_tile_ground,
            temperature_k,
            specific_liquid_enthalpy_j_kg,
            start_s: parcel.start_s,
            end_s: parcel.end_s,
        },
    })
}

fn receiver_expectations(
    configuration: &LandSurfaceEnergyConfiguration,
    beginning: &LandSurfaceEnergyState,
    hydrology_snapshot_sha256: &openwepp_land_surface_energy::Sha256Digest,
    soil_thermal: &SoilThermalSnapshot,
) -> Result<UnifiedReceiverExpectations, LandSurfaceEnergyShadowError> {
    let mut ordered_layers = Vec::new();
    for ofe in &configuration.ofes {
        let thermal = soil_thermal
            .ofes
            .iter()
            .find(|value| value.ofe_id == ofe.ofe_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "missing receiver soil-thermal OFE",
            ))?;
        for tile in &ofe.tiles {
            ordered_layers.push((
                ofe.ofe_id.clone(),
                tile.tile_id.clone(),
                thermal
                    .ordered_layers
                    .iter()
                    .map(|layer| layer.layer_id.clone())
                    .collect(),
            ));
        }
    }
    UnifiedReceiverExpectations::try_new(
        configuration.owner_id.clone(),
        beginning.state_sha256.clone(),
        configuration.hydrology_configuration.owner_id.clone(),
        hydrology_snapshot_sha256.clone(),
        configuration.soil_thermal_configuration.owner_id.clone(),
        soil_thermal.state_sha256.clone(),
        ordered_layers,
    )
}

#[cfg(test)]
#[path = "strict_v8_endpoint_tests.rs"]
pub(crate) mod endpoint_rollback_tests;

#[cfg(test)]
mod tests {
    use openwepp_kernel_contract::{ResourceOwnerId, TileId};
    use openwepp_land_surface_energy::{
        LiquidParcelKind, LiquidTemperatureProvider, OfeId, ParcelId, Sha256Digest,
        liquid_enthalpy_j_kg,
    };

    use super::*;

    fn parcel(mass: f64, temperature_k: f64) -> LiquidParcel {
        LiquidParcel {
            parcel_kind: LiquidParcelKind::Precipitation,
            parcel_id: ParcelId::try_new("rain-1").expect("parcel identity"),
            source_owner_id: ResourceOwnerId::try_new("meteorology").expect("owner identity"),
            source_ofe_id: OfeId::try_new("ofe-1").expect("source OFE"),
            source_tile_id: TileId::try_new("open").expect("source tile"),
            destination_ofe_id: OfeId::try_new("ofe-1").expect("destination OFE"),
            destination_tile_id: TileId::try_new("open").expect("destination tile"),
            start_s: 0.0,
            end_s: 1800.0,
            amount_kg_m2_destination_tile_ground: mass,
            temperature_provider: LiquidTemperatureProvider::HarderPomeroyHourly,
            temperature_k: Some(temperature_k),
            specific_liquid_enthalpy_j_kg: Some(liquid_enthalpy_j_kg(temperature_k)),
            source_state_sha256: Some(
                Sha256Digest::try_new(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                )
                .expect("digest"),
            ),
        }
    }

    #[test]
    fn public_endpoint_has_no_raw_open_ingress_parameter() {
        let source = include_str!("strict_v8_endpoint.rs");
        let signature = source
            .split("pub fn execute_v8_lse_runtime_shadow(")
            .nth(1)
            .and_then(|tail| tail.split(") -> Result").next())
            .expect("strict endpoint signature");
        assert!(!signature.contains("CoveredIngressSchedule"));
        assert!(!signature.contains("DirectTileGroundIngress"));
        assert!(!signature.contains("DirectSurfaceLiquidIngressInput"));
    }

    #[test]
    fn open_ingress_is_rederived_when_forcing_parcel_mass_changes() {
        let original = parcel(0.25, 281.5);
        let mut mutated = original.clone();
        mutated.amount_kg_m2_destination_tile_ground = 0.5;
        let original_ingress = strict_open_ingress_parcel(&original).expect("original");
        let mutated_ingress = strict_open_ingress_parcel(&mutated).expect("mutated");
        assert_eq!(
            original_ingress.amount.mass_kg_m2_tile_ground.to_bits(),
            0.25_f64.to_bits()
        );
        assert_eq!(
            mutated_ingress.amount.mass_kg_m2_tile_ground.to_bits(),
            0.5_f64.to_bits()
        );
        assert_ne!(original_ingress, mutated_ingress);
    }

    #[test]
    fn precipitation_and_runon_retain_distinct_type_thermal_and_lineage() {
        let rain = parcel(0.25, 281.0);
        let mut runon = parcel(0.5, 289.0);
        runon.parcel_kind = LiquidParcelKind::RoutedRunon;
        runon.temperature_provider = LiquidTemperatureProvider::AcceptedUpstreamOutletParcel;
        runon.parcel_id = ParcelId::try_new("runon-1").expect("parcel identity");
        runon.source_owner_id = ResourceOwnerId::try_new("upstream-surface").expect("owner");
        runon.source_ofe_id = OfeId::try_new("ofe-0").expect("source OFE");
        runon.source_tile_id = TileId::try_new("upstream-open").expect("source tile");
        runon.source_state_sha256 = Some(
            Sha256Digest::try_new(
                "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            )
            .expect("digest"),
        );
        runon.start_s = 240.0;
        runon.end_s = 960.0;
        let rain_ingress = strict_open_ingress_parcel(&rain).expect("rain ingress");
        let runon_ingress = strict_open_ingress_parcel(&runon).expect("runon ingress");
        assert_eq!(
            rain_ingress.kind,
            DirectSurfaceLiquidParcelKind::RawPrecipitation
        );
        assert_eq!(
            runon_ingress.kind,
            DirectSurfaceLiquidParcelKind::UpstreamRunon
        );
        assert_eq!(runon_ingress.parcel_id, runon.parcel_id);
        assert_eq!(runon_ingress.source_owner_id, runon.source_owner_id);
        assert_eq!(runon_ingress.source_ofe_id, runon.source_ofe_id);
        assert_eq!(runon_ingress.source_tile_id, runon.source_tile_id);
        assert_eq!(runon_ingress.destination_ofe_id, runon.destination_ofe_id);
        assert_eq!(runon_ingress.destination_tile_id, runon.destination_tile_id);
        assert_eq!(
            runon_ingress.accepted_source_state_sha256,
            runon.source_state_sha256.unwrap()
        );
        assert_eq!(runon_ingress.amount.start_s.to_bits(), 240.0_f64.to_bits());
        assert_eq!(runon_ingress.amount.end_s.to_bits(), 960.0_f64.to_bits());
        assert_eq!(
            runon_ingress.amount.specific_liquid_enthalpy_j_kg.to_bits(),
            liquid_enthalpy_j_kg(289.0).to_bits()
        );
    }

    #[test]
    fn source_lineage_and_type_mutations_change_strict_ingress_bytes() {
        let original = parcel(0.5, 289.0);
        let baseline = strict_open_ingress_parcel(&original).expect("baseline");
        let mut poisons = Vec::new();
        let mut source_owner = original.clone();
        source_owner.source_owner_id = ResourceOwnerId::try_new("poison-owner").expect("owner");
        poisons.push(source_owner);
        let mut source_ofe = original.clone();
        source_ofe.source_ofe_id = OfeId::try_new("poison-ofe").expect("OFE");
        poisons.push(source_ofe);
        let mut source_tile = original.clone();
        source_tile.source_tile_id = TileId::try_new("poison-tile").expect("tile");
        poisons.push(source_tile);
        let mut digest = original.clone();
        digest.source_state_sha256 = Some(
            Sha256Digest::try_new(
                "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
            )
            .expect("digest"),
        );
        poisons.push(digest);
        let mut kind = original;
        kind.parcel_kind = LiquidParcelKind::RoutedRunon;
        kind.temperature_provider = LiquidTemperatureProvider::AcceptedUpstreamOutletParcel;
        poisons.push(kind);
        for poison in poisons {
            assert_ne!(
                strict_open_ingress_parcel(&poison).expect("poison"),
                baseline
            );
        }
    }
}
