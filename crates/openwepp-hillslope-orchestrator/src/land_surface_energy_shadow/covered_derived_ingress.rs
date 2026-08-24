//! Fixed-cap canopy-liquid release handoff to the persistent surface owner.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{TileId, TransactionId};
use openwepp_land_surface_energy::{
    CoveredLiquidPass, CoveredOccupancyLiquidLedger, FinalCoveredTileCandidate, GroundWaterKey,
    OfeId, Sha256Digest, liquid_enthalpy_j_kg,
};

use crate::{
    DirectCanopyLiquidRelease, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectSurfaceLiquidIngressInput, DirectTileGroundIngress,
};

use super::{
    DirectSurfaceLiquidConfiguration, LandSurfaceEnergyRealHydrologyAdapter,
    LandSurfaceEnergyShadowError, MixedRealHydrologyRequest, PotentialWaterRequestBatch,
    RealHydrologySourceKey, UnifiedLseFinalization, UnifiedRealHydrologyArbitration,
    UnifiedRealHydrologyCandidate, UnifiedReceiverExpectations,
    authorize_surface_liquid_withdrawals, canonicalize_finalized_error, canonicalize_unified_error,
    compose_unified_beginning_hydrology_snapshot_sha256, construct_unified_candidate,
    partition_requests, preflight_request_bounds, preflight_request_cardinality,
    preflight_request_domains, preflight_request_identities, restore_authorization_order,
    validate_final_protocol, validate_native_shadow_exact_one_custody,
    validate_native_shadow_supported_domain, validate_receiver_expectations,
    validate_surface_production_binding,
};

/// Canonical cadence plus WB14 receiver parameters. All tile ingress is
/// derived inside the strict endpoint; covered-canopy release bytes can only
/// come from accepted final E04 ledgers.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CoveredIngressSchedule {
    pub(super) transaction_id: TransactionId,
    pub(super) day_index: usize,
    pub(super) interval_index: u8,
    pub(super) interval_s: f64,
    pub(super) open_tile_ingress: Vec<DirectTileGroundIngress>,
    pub(super) covered_runon: BTreeMap<(OfeId, TileId), Vec<crate::DirectOpenLiquidIngressParcel>>,
    pub(super) wb14_parameters: Vec<DirectOfeWb14Parameters>,
    pub(super) finalize_wb14_parent_interval: bool,
    pub(super) wb14_parent_working_state:
        Option<crate::direct_runtime::DirectWb14ParentWorkingState>,
    pub(super) wb14_coupled_child_binding:
        Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
}

fn ingress_amount(mass: f64, temperature_k: f64, interval_s: f64) -> DirectIngressAmount {
    DirectIngressAmount {
        mass_kg_m2_tile_ground: mass,
        temperature_k,
        specific_liquid_enthalpy_j_kg: liquid_enthalpy_j_kg(temperature_k),
        start_s: 0.0,
        end_s: interval_s,
    }
}

fn mass_weighted_temperature(
    mass_and_temperature: impl IntoIterator<Item = (f64, f64)>,
    zero_mass_temperature_k: f64,
) -> Result<f64, LandSurfaceEnergyShadowError> {
    let mut mass = 0.0;
    let mut temperature_mass = 0.0;
    for (item_mass, temperature_k) in mass_and_temperature {
        if !item_mass.is_finite()
            || item_mass < 0.0
            || !temperature_k.is_finite()
            || temperature_k <= 0.0
        {
            return Err(LandSurfaceEnergyShadowError::Operand(
                "invalid accepted canopy-liquid release operand",
            ));
        }
        mass += item_mass;
        temperature_mass += item_mass * temperature_k;
    }
    if mass == 0.0 {
        Ok(zero_mass_temperature_k)
    } else {
        Ok(temperature_mass / mass)
    }
}

fn derive_release_from_ledgers<'a>(
    ledgers: impl IntoIterator<Item = (&'a str, &'a CoveredOccupancyLiquidLedger)>,
    expected_ground_release: f64,
    expected_stemflow: f64,
    interval_s: f64,
) -> Result<DirectCanopyLiquidRelease, LandSurfaceEnergyShadowError> {
    let ledgers = ledgers.into_iter().collect::<Vec<_>>();
    if ledgers.is_empty() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "missing fixed-cap canopy-liquid ledger",
        ));
    }
    let mut identities = BTreeSet::new();
    let mut expected_incident = ledgers[0].1.incident_rain_kg_m2_tile;
    let mut stemflow = 0.0;
    for (identity, ledger) in &ledgers {
        ledger.validate()?;
        if ledger.pass != CoveredLiquidPass::FixedAuthorizationFinal {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "potential-pass canopy liquid cannot become ingress",
            ));
        }
        if !identities.insert(*identity)
            || ledger.incident_rain_kg_m2_tile.to_bits() != expected_incident.to_bits()
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "duplicate or misrouted canopy-liquid ledger",
            ));
        }
        expected_incident = ledger.throughfall_kg_m2_tile
            + ledger.initial_drainage_kg_m2_tile
            + ledger.second_drainage_kg_m2_tile;
        stemflow += ledger.stemflow_kg_m2_tile;
    }
    let bottom = ledgers
        .last()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing fixed-cap canopy-liquid ledger",
        ))?
        .1;
    let bottom_temperature = bottom.wet_surface_temperature_k;
    let stemflow_temperature = mass_weighted_temperature(
        ledgers
            .iter()
            .map(|(_, ledger)| (ledger.stemflow_kg_m2_tile, ledger.wet_surface_temperature_k)),
        bottom_temperature,
    )?;
    let release = DirectCanopyLiquidRelease {
        throughfall: ingress_amount(
            bottom.throughfall_kg_m2_tile,
            bottom_temperature,
            interval_s,
        ),
        initial_drainage: ingress_amount(
            bottom.initial_drainage_kg_m2_tile,
            bottom_temperature,
            interval_s,
        ),
        second_drainage: ingress_amount(
            bottom.second_drainage_kg_m2_tile,
            bottom_temperature,
            interval_s,
        ),
        stemflow: ingress_amount(stemflow, stemflow_temperature, interval_s),
    };
    let ground_release = release.throughfall.mass_kg_m2_tile_ground
        + release.initial_drainage.mass_kg_m2_tile_ground
        + release.second_drainage.mass_kg_m2_tile_ground;
    if ground_release.to_bits() != expected_ground_release.to_bits()
        || stemflow.to_bits() != expected_stemflow.to_bits()
    {
        return Err(LandSurfaceEnergyShadowError::Bound(
            "fixed-cap canopy ingress mass mismatch",
        ));
    }
    Ok(release)
}

fn direct_identity(row: &DirectTileGroundIngress) -> (&str, &str, &str) {
    match row {
        DirectTileGroundIngress::OpenRawPrecipitation {
            ofe_id,
            tile_id,
            surface_id,
            ..
        }
        | DirectTileGroundIngress::OpenLiquidParcels {
            ofe_id,
            tile_id,
            surface_id,
            ..
        }
        | DirectTileGroundIngress::CoveredCanopyRelease {
            ofe_id,
            tile_id,
            surface_id,
            ..
        }
        | DirectTileGroundIngress::CoveredCanopyReleaseAndRunon {
            ofe_id,
            tile_id,
            surface_id,
            ..
        } => (ofe_id.as_str(), tile_id.as_str(), surface_id.as_str()),
    }
}

/// Derive one complete ingress input after every configured covered tile has
/// produced an accepted fixed-cap result. This is the only canopy-release
/// constructor reachable from the covered public path.
pub(super) fn derive_fixed_cap_canopy_ingress(
    configuration: &DirectSurfaceLiquidConfiguration,
    final_tiles: &[FinalCoveredTileCandidate],
    schedule: &CoveredIngressSchedule,
) -> Result<DirectSurfaceLiquidIngressInput, LandSurfaceEnergyShadowError> {
    let mut actual_identities = BTreeSet::new();
    let mut tile_ingress = schedule.open_tile_ingress.clone();
    for row in &schedule.open_tile_ingress {
        if !matches!(
            row,
            DirectTileGroundIngress::OpenRawPrecipitation { .. }
                | DirectTileGroundIngress::OpenLiquidParcels { .. }
        ) || !actual_identities.insert(direct_identity(row))
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "duplicate or caller-supplied covered canopy ingress",
            ));
        }
    }

    for final_tile in final_tiles {
        final_tile.vegetation_operands.validate()?;
        let identity = &final_tile.identity;
        if schedule.transaction_id != identity.transaction_id
            || schedule.interval_s.to_bits() != identity.interval_s.to_bits()
            || final_tile.transaction_id != identity.transaction_id
            || !actual_identities.insert((
                identity.ofe_id.as_str(),
                identity.tile_id.as_str(),
                identity.surface_id.as_str(),
            ))
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "covered ingress transaction, interval, or duplicate tile",
            ));
        }
        let release = derive_release_from_ledgers(
            final_tile
                .vegetation_operands
                .occupancies
                .iter()
                .map(|row| (row.occupancy_id.as_str(), &row.liquid)),
            final_tile
                .vegetation_operands
                .ground_canopy_release_kg_m2_tile_ground,
            final_tile
                .vegetation_operands
                .ground_stemflow_kg_m2_tile_ground,
            schedule.interval_s,
        )?;
        let runon_parcels = schedule
            .covered_runon
            .get(&(identity.ofe_id.clone(), identity.tile_id.clone()))
            .cloned()
            .unwrap_or_default();
        tile_ingress.push(if runon_parcels.is_empty() {
            DirectTileGroundIngress::CoveredCanopyRelease {
                ofe_id: identity.ofe_id.clone(),
                tile_id: identity.tile_id.clone(),
                surface_id: identity.surface_id.clone(),
                release,
            }
        } else {
            DirectTileGroundIngress::CoveredCanopyReleaseAndRunon {
                ofe_id: identity.ofe_id.clone(),
                tile_id: identity.tile_id.clone(),
                surface_id: identity.surface_id.clone(),
                release,
                runon_parcels,
            }
        });
    }

    let configured_identities = configuration
        .records
        .iter()
        .map(|record| {
            (
                record.key.ofe_id.as_str(),
                record.key.tile_id.as_str(),
                record.key.surface_id.as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    if actual_identities != configured_identities {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "missing or unexpected fixed-cap tile ingress",
        ));
    }
    Ok(DirectSurfaceLiquidIngressInput {
        transaction_id: schedule.transaction_id,
        day_index: schedule.day_index,
        interval_index: schedule.interval_index,
        interval_s: schedule.interval_s,
        tile_ingress,
        wb14_parameters: schedule.wb14_parameters.clone(),
    })
}

struct RequestOnlyPreflight {
    actual_snapshot: Sha256Digest,
    attempted_sha256: String,
    soil_requests: Vec<MixedRealHydrologyRequest>,
    surface_requests: Vec<openwepp_land_surface_energy::WaterAmount>,
}

fn validate_request_before_authorization(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    schedule: &CoveredIngressSchedule,
) -> Result<RequestOnlyPreflight, LandSurfaceEnergyShadowError> {
    configuration.preflight_schema_and_identity_structure()?;
    let surface_state = soil_adapter
        .owner
        .beginning_frame()
        .surface_liquid_shadow
        .as_deref()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing beginning surface-liquid owner",
        ))?;
    surface_state.preflight_schema_and_identity_structure(configuration)?;
    validate_surface_production_binding(soil_adapter.owner, configuration)?;
    let actual_snapshot = compose_unified_beginning_hydrology_snapshot_sha256(
        soil_adapter,
        configuration,
        surface_state,
    )?;
    let attempted_sha256 = actual_snapshot.to_string();
    if schedule.transaction_id != request_batch.transaction_id
        || schedule.day_index != soil_adapter.owner.day_index()
        || schedule.interval_s.to_bits() != soil_adapter.owner.interval_s().to_bits()
        || actual_snapshot != receiver_expectations.beginning_hydrology_snapshot_sha256
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "derived-ingress request snapshot, transaction, or cadence",
        ));
    }
    preflight_request_identities(request_batch, &actual_snapshot)?;
    preflight_request_domains(request_batch, &actual_snapshot)?;
    preflight_request_cardinality(request_batch, &actual_snapshot)?;
    preflight_request_bounds(request_batch, &actual_snapshot)?;
    request_batch.validate()?;
    validate_native_shadow_supported_domain(
        soil_adapter.owner,
        configuration,
        &actual_snapshot,
        &attempted_sha256,
    )?;
    validate_native_shadow_exact_one_custody(
        soil_adapter.owner,
        configuration,
        &actual_snapshot,
        &attempted_sha256,
    )?;
    validate_receiver_expectations(
        soil_adapter.owner,
        configuration,
        receiver_expectations,
        request_batch,
        &actual_snapshot,
    )?;
    let (soil_requests, surface_requests) =
        partition_requests(request_batch, soil_sources, configuration, &actual_snapshot)?;
    Ok(RequestOnlyPreflight {
        actual_snapshot,
        attempted_sha256,
        soil_requests,
        surface_requests,
    })
}

/// Unified authorization whose actual ingress is produced only alongside the
/// accepted fixed-cap finalization. No placeholder ingress is hashed or
/// preflighted.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(super) fn execute_unified_with_derived_ingress<F>(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    schedule: &CoveredIngressSchedule,
    finalize_fixed_caps: F,
) -> Result<UnifiedRealHydrologyCandidate, LandSurfaceEnergyShadowError>
where
    F: FnOnce(
        &[openwepp_land_surface_energy::WaterAuthorization],
    ) -> Result<
        (UnifiedLseFinalization, DirectSurfaceLiquidIngressInput),
        LandSurfaceEnergyShadowError,
    >,
{
    let request_preflight = validate_request_before_authorization(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        request_batch,
        soil_sources,
        schedule,
    )?;
    let beginning_surface = soil_adapter
        .owner
        .beginning_frame()
        .surface_liquid_shadow
        .as_deref()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing beginning surface-liquid owner",
        ))?;
    let soil = soil_adapter
        .authorize(&request_preflight.soil_requests)
        .map_err(|error| {
            super::unified_entry_preflight::complete_unified_failure(
                canonicalize_unified_error(
                    error,
                    request_batch,
                    &request_preflight.actual_snapshot,
                ),
                &request_preflight.actual_snapshot,
                &request_preflight.attempted_sha256,
            )
        })?;
    let surface = authorize_surface_liquid_withdrawals(
        surface_configuration,
        beginning_surface,
        request_batch.transaction_id,
        beginning_surface
            .records
            .first()
            .and_then(|record| record.last_accepted_transaction_id),
        &request_preflight.surface_requests,
    )?;
    let authorizations = restore_authorization_order(
        request_batch,
        &soil,
        &surface,
        &request_preflight.actual_snapshot,
    )?;
    let arbitration = UnifiedRealHydrologyArbitration {
        transaction_id: request_batch.transaction_id,
        requests: request_batch.requests.clone(),
        authorizations,
        soil,
        surface,
    };
    let (finalized, ingress) = finalize_fixed_caps(&arbitration.authorizations)?;

    // Bind the exact derived bytes—not a fabricated placeholder—to the full
    // ingress identity, attempted-operation hash, and cadence preflight.
    let actual_entry = super::unified_entry_preflight::validate_unified_entry(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        request_batch,
        soil_sources,
        &ingress,
        &request_preflight.actual_snapshot,
    )?;
    if actual_entry.actual_snapshot != request_preflight.actual_snapshot
        || actual_entry.soil_requests != request_preflight.soil_requests
        || actual_entry.surface_requests != request_preflight.surface_requests
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "derived ingress changed immutable request preflight",
        ));
    }
    validate_final_protocol(
        &finalized.water_protocol,
        &arbitration,
        &request_preflight.actual_snapshot,
        &surface_configuration.owner_id,
    )?;
    let finalized_protocol = finalized.water_protocol.clone();
    construct_unified_candidate(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        request_batch,
        arbitration,
        finalized,
        &ingress,
        schedule.finalize_wb14_parent_interval,
        schedule.wb14_parent_working_state.as_ref(),
        schedule.wb14_coupled_child_binding,
    )
    .map_err(|error| canonicalize_finalized_error(error, &finalized_protocol))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ledger(
        incident: f64,
        ending: f64,
        throughfall: f64,
        stemflow: f64,
        initial: f64,
        second: f64,
        temperature_k: f64,
    ) -> CoveredOccupancyLiquidLedger {
        CoveredOccupancyLiquidLedger {
            pass: CoveredLiquidPass::FixedAuthorizationFinal,
            beginning_store_kg_m2_tile: 0.0,
            incident_rain_kg_m2_tile: incident,
            ending_store_kg_m2_tile: ending,
            evaporation_kg_m2_tile: 0.0,
            condensation_kg_m2_tile: 0.0,
            throughfall_kg_m2_tile: throughfall,
            stemflow_kg_m2_tile: stemflow,
            initial_drainage_kg_m2_tile: initial,
            second_drainage_kg_m2_tile: second,
            wet_fraction: 0.5,
            wet_surface_temperature_k: temperature_k,
            wet_surface_specific_enthalpy_j_kg: liquid_enthalpy_j_kg(temperature_k),
        }
    }

    #[test]
    fn fixed_cap_release_preserves_bottom_releases_and_mass_weighted_stemflow_heat() {
        let upper = ledger(1.0, 0.0, 0.6, 0.1, 0.2, 0.1, 290.0);
        let lower = ledger(0.9, 0.1, 0.4, 0.1, 0.2, 0.1, 300.0);
        let expected_ground = lower.throughfall_kg_m2_tile
            + lower.initial_drainage_kg_m2_tile
            + lower.second_drainage_kg_m2_tile;
        let expected_stemflow = upper.stemflow_kg_m2_tile + lower.stemflow_kg_m2_tile;
        let release = derive_release_from_ledgers(
            [("upper", &upper), ("lower", &lower)],
            expected_ground,
            expected_stemflow,
            1_800.0,
        )
        .expect("derived final release");
        assert_eq!(
            release.throughfall.mass_kg_m2_tile_ground.to_bits(),
            0.4_f64.to_bits()
        );
        assert_eq!(
            release.initial_drainage.mass_kg_m2_tile_ground.to_bits(),
            0.2_f64.to_bits()
        );
        assert_eq!(
            release.second_drainage.mass_kg_m2_tile_ground.to_bits(),
            0.1_f64.to_bits()
        );
        assert_eq!(
            release.stemflow.mass_kg_m2_tile_ground.to_bits(),
            expected_stemflow.to_bits()
        );
        assert_eq!(
            release.stemflow.temperature_k.to_bits(),
            295.0_f64.to_bits()
        );
        assert_eq!(
            release.stemflow.specific_liquid_enthalpy_j_kg.to_bits(),
            liquid_enthalpy_j_kg(295.0).to_bits()
        );
    }

    #[test]
    fn derived_release_rejects_wrong_mass_temperature_missing_duplicate_and_potential() {
        let upper = ledger(1.0, 0.0, 0.6, 0.1, 0.2, 0.1, 290.0);
        let lower = ledger(0.9, 0.1, 0.4, 0.1, 0.2, 0.1, 300.0);

        let mut wrong_mass = upper;
        wrong_mass.throughfall_kg_m2_tile = 0.61;
        assert!(
            derive_release_from_ledgers(
                [("upper", &wrong_mass), ("lower", &lower)],
                0.7,
                0.2,
                1_800.0,
            )
            .is_err()
        );

        let mut wrong_temperature = upper;
        wrong_temperature.wet_surface_temperature_k = 291.0;
        assert!(
            derive_release_from_ledgers(
                [("upper", &wrong_temperature), ("lower", &lower)],
                0.7,
                0.2,
                1_800.0,
            )
            .is_err()
        );

        assert!(
            derive_release_from_ledgers(
                std::iter::empty::<(&str, &CoveredOccupancyLiquidLedger)>(),
                0.0,
                0.0,
                1_800.0,
            )
            .is_err()
        );
        assert!(
            derive_release_from_ledgers([("same", &upper), ("same", &lower)], 0.7, 0.2, 1_800.0,)
                .is_err()
        );

        let mut potential = upper;
        potential.pass = CoveredLiquidPass::Potential;
        assert!(
            derive_release_from_ledgers(
                [("upper", &potential), ("lower", &lower)],
                0.7,
                0.2,
                1_800.0,
            )
            .is_err()
        );
    }
}
