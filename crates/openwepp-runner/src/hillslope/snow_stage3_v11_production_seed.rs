use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use openwepp_coupled_time::Digest32;
use openwepp_hillslope_orchestrator::runtime_inputs::restart_authority_restore_gsi_state;
#[cfg(any(test, feature = "test-fixture-authority"))]
use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectRootZoneLayerConfiguration;
use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::{
    DirectRootZoneHydraulicConfiguration, DirectV10RealConsumerShadow,
    restart_authority_wb14_parameter_sha256,
};
use openwepp_hillslope_orchestrator::vegetation_real_hydrology_shadow::{
    RealHydrologyLaneLayerMap, RealHydrologyOfeLaneId,
};
use openwepp_hillslope_orchestrator::{
    DirectOfeWb14Parameters, DirectRunFrame, DirectSnowStage3V11ProductionConfigurationV1,
    DirectSnowTerminalEventRequest, Wb11HydrologyKernel,
};
use openwepp_kernel_contract::ResourceOwnerId;
#[cfg(any(test, feature = "test-fixture-authority"))]
use openwepp_kernel_contract::{SoilLayerId, TileId};
use openwepp_land_surface_energy::{LandSurfaceEnergyConfiguration, Sha256Digest};
#[cfg(any(test, feature = "test-fixture-authority"))]
use openwepp_land_surface_energy::{OfeId, SourceId, SurfaceId};
use openwepp_persisted_restart_v1::{
    DirectV10CheckpointPhaseV1, DirectV10RealConsumerCheckpointV1,
    ExpectedDirectHydrologyRestartContext, Sha256Hex, checkpoint_identities_v1,
    from_canonical_bytes,
};
use openwepp_vegetation::VegetationConfiguration;
use serde::{Deserialize, Serialize};

use super::{HillslopeCliError, SIMOUT_GUARD_ID};

#[path = "snow_stage3_v11_production_seed_v2_bootstrap.rs"]
mod snow_stage3_v11_production_seed_v2_bootstrap;
use snow_stage3_v11_production_seed_v2_bootstrap::bootstrap_soil_thermal_v2;
#[path = "snow_stage3_v11_production_seed_frozen_litter_v3.rs"]
mod snow_stage3_v11_production_seed_frozen_litter_v3;
#[path = "snow_stage3_v11_production_seed_frozen_litter_v4.rs"]
mod snow_stage3_v11_production_seed_frozen_litter_v4;
use snow_stage3_v11_production_seed_frozen_litter_v4::bootstrap_frozen_litter_v4_resident;

const SCHEMA: &str = "OPENWEPP_SNOW_STAGE3_V11_PRODUCTION_SEED_V1";
const VERSION: u16 = 1;
const SUPPORT_STATIC_SCHEMA: &str = "OPENWEPP_SNOW_STAGE3_V11_SUPPORT_STATIC_AUTHORITY_V1";
const SUPPORT_STATIC_VERSION: u16 = 1;

/// Complete non-climate interval authority used to project each sealed
/// repository receipt into the native snow-free and covered V11 inputs.
/// Dynamic beginning snow state is deliberately absent from this schema.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DirectSnowStage3V11SupportStaticAuthorityV1 {
    schema: String,
    version: u16,
    interval_template:
        openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectV9ShadowIntervalInput,
    rho_air_kg_m3: f64,
    cp_air_j_kg_k: f64,
    underlying_surface_albedo: f64,
}

impl DirectSnowStage3V11SupportStaticAuthorityV1 {
    #[must_use]
    pub(super) const fn interval_template(
        &self,
    ) -> &openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectV9ShadowIntervalInput
    {
        &self.interval_template
    }

    #[must_use]
    pub(super) const fn rho_air_kg_m3(&self) -> f64 {
        self.rho_air_kg_m3
    }

    #[must_use]
    pub(super) const fn cp_air_j_kg_k(&self) -> f64 {
        self.cp_air_j_kg_k
    }

    #[must_use]
    pub(super) const fn underlying_surface_albedo(&self) -> f64 {
        self.underlying_surface_albedo
    }
}

/// External, versioned authority required to bootstrap the constitutive
/// Stage-3 owner. Dynamic owners remain in the sealed restart envelope;
/// static configurations are native DTOs so their own validators and identity
/// joins remain authoritative.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct DirectSnowStage3V11ProductionSeedArtifactV1 {
    schema: String,
    version: u16,
    vegetation_configuration: VegetationConfiguration,
    vegetation_owner_id: ResourceOwnerId,
    lse_configuration: LandSurfaceEnergyConfiguration,
    root_zone_hydraulic_configuration: DirectRootZoneHydraulicConfiguration,
    wb14_parameters: Vec<DirectOfeWb14Parameters>,
    calendar_receipt: Digest32,
    controller_policy: Digest32,
    support_static_authority: DirectSnowStage3V11SupportStaticAuthorityV1,
    checkpoint: DirectV10RealConsumerCheckpointV1,
}

#[derive(Debug)]
pub(super) struct DirectSnowStage3V11ProductionSeedV1 {
    artifact: DirectSnowStage3V11ProductionSeedArtifactV1,
}

impl DirectSnowStage3V11ProductionSeedV1 {
    pub(super) fn load_required(path: Option<&Path>) -> Result<Self, HillslopeCliError> {
        let path = path.ok_or_else(|| failure(
            "direct production requires inputs.snow_stage3_v11_owner_seed (or the legacy-discovery snow_stage3_v11_owner_seed.json); no fixture/default owner seed is admitted",
        ))?;
        let bytes = fs::read(path).map_err(|source| HillslopeCliError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let artifact: DirectSnowStage3V11ProductionSeedArtifactV1 = from_canonical_bytes(&bytes)
            .map_err(|error| {
                failure(format!(
                    "Stage-3 production seed '{}' is not strict canonical JSON: {error}",
                    path.display()
                ))
            })?;
        let value = Self { artifact };
        value.validate_envelope()?;
        Ok(value)
    }

    #[cfg(test)]
    pub(super) const fn test_fixture_vegetation_authorities(
        &self,
    ) -> (&VegetationConfiguration, &LandSurfaceEnergyConfiguration) {
        (
            &self.artifact.vegetation_configuration,
            &self.artifact.lse_configuration,
        )
    }

    fn validate_envelope(&self) -> Result<(), HillslopeCliError> {
        let artifact = &self.artifact;
        if artifact.schema != SCHEMA || artifact.version != VERSION {
            return Err(failure(format!(
                "Stage-3 production seed schema/version must be {SCHEMA}/{VERSION}, observed {}/{}",
                artifact.schema, artifact.version
            )));
        }
        if artifact.checkpoint.schema != "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1"
            || artifact.checkpoint.version != 1
            || artifact.checkpoint.compute_digest().map_err(nested)?
                != artifact.checkpoint.payload_sha256
        {
            return Err(failure(
                "Stage-3 production seed carries an unsealed or unsupported V10 checkpoint",
            ));
        }
        let committed = self.day_zero_committed()?;
        let (run_identity, topology_identity) =
            checkpoint_identities_v1(committed, &artifact.root_zone_hydraulic_configuration)
                .map_err(nested)?;
        if run_identity != artifact.checkpoint.run_identity_sha256
            || topology_identity != artifact.checkpoint.topology_sha256
        {
            return Err(failure(
                "Stage-3 production seed checkpoint run/topology identities do not match its complete owner set",
            ));
        }
        artifact
            .vegetation_configuration
            .validate_v10()
            .map_err(nested)?;
        let biogeochemistry = committed
            .scientific
            .biogeochemistry
            .restore()
            .map_err(nested)?;
        validate_bgc_inventory_covers_vegetation_roots(
            &artifact.vegetation_configuration,
            &biogeochemistry.layers,
        )?;
        artifact.lse_configuration.validate_v2().map_err(nested)?;
        validate_root_zone_configuration(
            &artifact.root_zone_hydraulic_configuration,
            &artifact.vegetation_configuration,
        )?;
        validate_wb14_authority(committed, &artifact.wb14_parameters)?;
        validate_support_static_authority(
            committed,
            &artifact.root_zone_hydraulic_configuration,
            &artifact.wb14_parameters,
            &artifact.support_static_authority,
        )?;
        if artifact.calendar_receipt == Digest32::zero()
            || artifact.controller_policy == Digest32::zero()
        {
            return Err(failure(
                "Stage-3 production seed calendar/controller digests must be nonzero",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub(super) const fn support_static_authority(
        &self,
    ) -> &DirectSnowStage3V11SupportStaticAuthorityV1 {
        &self.artifact.support_static_authority
    }

    #[must_use]
    pub(super) fn destination_is_canopy_covered(&self, tile_id: &str) -> bool {
        self.artifact
            .vegetation_configuration
            .strata
            .iter()
            .any(|stratum| {
                stratum
                    .tile_ids
                    .iter()
                    .any(|configured| configured.as_str() == tile_id)
            })
    }

    fn day_zero_committed(
        &self,
    ) -> Result<&openwepp_persisted_restart_v1::CompleteCommittedOwnerStateV1, HillslopeCliError>
    {
        match &self.artifact.checkpoint.phase {
            DirectV10CheckpointPhaseV1::BetweenDays {
                next_day_index,
                accepted_interval_count,
                committed,
            } if next_day_index.0 == 0 && accepted_interval_count.get() == 0 => Ok(committed),
            DirectV10CheckpointPhaseV1::BetweenDays { .. }
            | DirectV10CheckpointPhaseV1::InProgressDay { .. } => Err(failure(
                "Stage-3 production seed must be a between-days day-zero checkpoint with zero accepted intervals",
            )),
        }
    }

    /// Reconstruct every embedded owner through the canonical restart DTOs,
    /// require byte-exact live hydrology equality, migrate the retained V1/V2
    /// bootstrap provenance into a mandatory native frozen-litter V3 resident,
    /// and install the production attachment. No value is inferred from runner
    /// fixtures or defaults, and the old wire is not execution evidence.
    #[cfg(test)]
    #[allow(clippy::too_many_lines)]
    pub(super) fn bootstrap(&self, frame: &mut DirectRunFrame) -> Result<(), HillslopeCliError> {
        self.bootstrap_with_laned_active_surface_owner(frame, false)
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn bootstrap_with_laned_active_surface_owner(
        &self,
        frame: &mut DirectRunFrame,
        laned_active_surface_owner: bool,
    ) -> Result<(), HillslopeCliError> {
        let artifact = &self.artifact;
        let committed = self.day_zero_committed()?;
        let scientific = &committed.scientific;

        let mut surface_configuration = committed
            .surface_liquid_configuration
            .restore()
            .map_err(nested)?;
        let gsi_configuration = committed.gsi_configuration.restore().map_err(nested)?;
        let gsi_native_state = committed.gsi_state.restore().map_err(nested)?;
        let gsi_state = restart_authority_restore_gsi_state(&gsi_native_state).map_err(nested)?;
        let provider_static_configuration = committed
            .static_forcing_configuration
            .restore()
            .map_err(nested)?;
        let provider_cursor = committed
            .provider_cursor
            .restore(&provider_static_configuration, 0)
            .map_err(nested)?;
        let vegetation_state = scientific
            .vegetation_v10
            .restore(
                &artifact.vegetation_configuration,
                &artifact.vegetation_owner_id,
            )
            .map_err(nested)?;
        let lse_state = scientific
            .lse_v2
            .restore(&artifact.lse_configuration)
            .map_err(nested)?;
        let soil_configuration = &artifact.lse_configuration.soil_thermal_configuration;
        let soil_thermal_v1 = scientific
            .soil_thermal
            .restore(
                &soil_configuration.owner_id,
                &soil_configuration.configuration_sha256,
            )
            .map_err(nested)?;
        let receipt_chain_sha256 = Sha256Digest::try_new(
            scientific
                .soil_thermal
                .restart_payload_sha256
                .as_str()
                .to_owned(),
        )
        .map_err(nested)?;
        let (prepared_soil_thermal, soil_thermal_seals) = bootstrap_soil_thermal_v2(
            &soil_thermal_v1,
            &artifact.lse_configuration,
            frame.lanes.len(),
            artifact
                .support_static_authority
                .interval_template
                .lse_forcing
                .transaction_id,
            &provider_static_configuration.run_id,
            receipt_chain_sha256,
        )?;
        let biogeochemistry = scientific.biogeochemistry.restore().map_err(nested)?;

        if frame.identity.run_id.to_string() != provider_static_configuration.run_id
            || surface_configuration.run_id != frame.identity.run_id
        {
            return Err(failure(
                "Stage-3 production seed provider/surface run identity does not match the live run",
            ));
        }

        let mut surface_state = scientific
            .direct_hydrology
            .surface_liquid_owned_state
            .as_deref()
            .ok_or_else(|| failure("Stage-3 production seed omits the surface-liquid owner"))?
            .restore_with_configuration(&surface_configuration)
            .map_err(nested)?;
        frame
            .configure_surface_liquid_shadow(&surface_configuration, surface_state.clone())
            .map_err(nested)?;

        let day_inputs = frame
            .lanes
            .iter()
            .map(|lane| lane.day_inputs.clone())
            .collect::<Vec<_>>();
        let day_input_digests = scientific
            .direct_hydrology
            .lanes
            .iter()
            .map(|lane| lane.day_inputs_sha256.clone())
            .collect::<Vec<Sha256Hex>>();
        let hydrology_context = ExpectedDirectHydrologyRestartContext {
            phase_plan: &frame.phase_plan,
            phase_plan_sha256: &scientific.direct_hydrology.phase_plan_sha256,
            day_inputs: &day_inputs,
            day_input_digests: &day_input_digests,
            surface_liquid_configuration: &surface_configuration,
        };
        let restored_hydrology = scientific
            .direct_hydrology
            .restore(&hydrology_context)
            .map_err(nested)?;
        if restored_hydrology != *frame {
            return Err(failure(
                "Stage-3 production seed hydrology owner is not exactly the freshly built live frame",
            ));
        }

        if laned_active_surface_owner {
            (surface_configuration, surface_state) = project_laned_active_day_zero_surface_owner(
                &surface_configuration,
                &surface_state,
            )?;
            frame
                .configure_surface_liquid_shadow(&surface_configuration, surface_state.clone())
                .map_err(nested)?;
        }

        let (frozen_litter_v3, frozen_litter_v4) = bootstrap_frozen_litter_v4_resident(
            &artifact.lse_configuration,
            &lse_state,
            &surface_configuration,
            &surface_state,
        )
        .map_err(nested)?;

        let layer_maps = frame
            .lanes
            .iter()
            .zip(&prepared_soil_thermal.beginning_owner().state.ofes)
            .enumerate()
            .map(|(lane_index, (lane, ofe))| RealHydrologyLaneLayerMap {
                ofe_lane: RealHydrologyOfeLaneId {
                    lane_index,
                    lane_id: lane.lane_id,
                },
                layer_ids: ofe
                    .ordered_layers
                    .iter()
                    .map(|layer| layer.layer_id.clone())
                    .collect(),
            })
            .collect::<Vec<_>>();
        let mut real_consumer = DirectV10RealConsumerShadow::try_new_v2(
            artifact.vegetation_configuration.clone(),
            vegetation_state,
            artifact.vegetation_owner_id.clone(),
            artifact.lse_configuration.clone(),
            lse_state,
            surface_configuration.clone(),
            layer_maps,
            prepared_soil_thermal,
            soil_thermal_seals,
            biogeochemistry,
            frame.clone(),
            0,
            gsi_configuration,
            gsi_state,
            provider_static_configuration,
            provider_cursor,
            artifact.root_zone_hydraulic_configuration.clone(),
        )
        .map_err(nested)?;
        real_consumer
            .install_frozen_litter_v4_resident(frozen_litter_v3, frozen_litter_v4)
            .map_err(nested)?;

        let stage3_by_lane = frame
            .lanes
            .iter()
            .map(|lane| {
                Wb11HydrologyKernel::initialize_stage3_persistent_state_with_retained_liquid_and_terminal_event(
                    lane.lane_id,
                    lane.winter_column.snow.layers.clone(),
                    lane.winter_column.snow.liquid_water_retained_m * 1_000.0,
                    DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
                )
                .map(|state| (lane.lane_id, state))
                .map_err(nested)
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        let production_configuration = DirectSnowStage3V11ProductionConfigurationV1 {
            run_identity: digest32(&artifact.checkpoint.run_identity_sha256)?,
            topology_identity: digest32(&artifact.checkpoint.topology_sha256)?,
            calendar_receipt: artifact.calendar_receipt,
            controller_policy: artifact.controller_policy,
            surface_liquid_configuration: surface_configuration,
            wb14_parameters: artifact.wb14_parameters.clone(),
        };
        frame
            .initialize_snow_stage3_v11_production(
                production_configuration,
                stage3_by_lane,
                real_consumer,
            )
            .map_err(nested)
    }
}

#[cfg(test)]
thread_local! {
    static EXPLICIT_TEST_OWNER_SEED: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
struct ExplicitTestOwnerSeedReset<'a>(&'a std::cell::Cell<u8>);

#[cfg(test)]
impl Drop for ExplicitTestOwnerSeedReset<'_> {
    fn drop(&mut self) {
        self.0.set(0);
    }
}

/// Admit the repository's explicit owner fixture for one scoped runner test.
/// This hook is unavailable in production builds and never changes the
/// fail-closed sidecar requirement.
#[cfg(test)]
pub(super) fn with_explicit_test_owner_seed<T>(run: impl FnOnce() -> T) -> T {
    with_explicit_test_owner_seed_kind(1, run)
}

#[cfg(test)]
pub(super) fn with_explicit_two_ofe_test_owner_seed<T>(run: impl FnOnce() -> T) -> T {
    with_explicit_test_owner_seed_kind(2, run)
}

/// Admit the repository's sealed no-strata adaptive owner for cold-season
/// qualification. This remains test-only and cannot bypass the production
/// sidecar requirement.
#[cfg(test)]
pub(super) fn with_explicit_adaptive_test_owner_seed<T>(run: impl FnOnce() -> T) -> T {
    with_explicit_test_owner_seed_kind(3, run)
}

#[cfg(test)]
fn with_explicit_test_owner_seed_kind<T>(kind: u8, run: impl FnOnce() -> T) -> T {
    EXPLICIT_TEST_OWNER_SEED.with(|enabled| {
        assert_eq!(
            enabled.replace(kind),
            0,
            "nested explicit Stage-3 test seed"
        );
        let _reset = ExplicitTestOwnerSeedReset(enabled);
        run()
    })
}

#[cfg(test)]
pub(super) fn load_required_or_explicit_test(
    path: Option<&Path>,
    frame: &DirectRunFrame,
) -> Result<DirectSnowStage3V11ProductionSeedV1, HillslopeCliError> {
    match EXPLICIT_TEST_OWNER_SEED.with(std::cell::Cell::get) {
        0 => DirectSnowStage3V11ProductionSeedV1::load_required(path),
        1 => explicit_repository_test_seed(frame, None),
        2 => explicit_two_ofe_repository_test_seed(frame, None, false, false),
        3 => explicit_adaptive_repository_test_seed(frame, None),
        _ => unreachable!("closed explicit test seed kind"),
    }
}

#[cfg(any(test, feature = "test-fixture-authority"))]
fn explicit_repository_test_seed(
    frame: &DirectRunFrame,
    authoring_latitude_degrees: Option<f64>,
) -> Result<DirectSnowStage3V11ProductionSeedV1, HillslopeCliError> {
    use openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture;

    let mut fixture = restart_authority_prepared_day_fixture();
    openwepp_hillslope_orchestrator::v9_real_consumer_shadow::restart_authority_equilibrate_complete_owner_fixture(
        &mut fixture.owners.runtime.shadow,
    )
    .map_err(nested)?;
    let interval_template = fixture
        .template
        .intervals
        .first()
        .ok_or_else(|| failure("explicit test fixture omits interval template"))?
        .clone();
    explicit_repository_test_seed_from_owner(
        frame,
        &fixture.owners,
        interval_template,
        authoring_latitude_degrees,
        false,
        false,
    )
}

#[cfg(any(test, feature = "test-fixture-authority"))]
fn explicit_adaptive_repository_test_seed(
    frame: &DirectRunFrame,
    authoring_latitude_degrees: Option<f64>,
) -> Result<DirectSnowStage3V11ProductionSeedV1, HillslopeCliError> {
    use openwepp_persisted_restart_v1::restart_authority_adaptive_prepared_day_fixture;

    let mut fixture = restart_authority_adaptive_prepared_day_fixture();
    openwepp_hillslope_orchestrator::v9_real_consumer_shadow::restart_authority_equilibrate_complete_owner_fixture(
        &mut fixture.owners.runtime.shadow,
    )
    .map_err(nested)?;
    let interval_template = fixture
        .template
        .intervals
        .first()
        .ok_or_else(|| failure("explicit adaptive test fixture omits interval template"))?
        .clone();
    explicit_repository_test_seed_from_owner(
        frame,
        &fixture.owners,
        interval_template,
        authoring_latitude_degrees,
        true,
        false,
    )
}

#[cfg(any(test, feature = "test-fixture-authority"))]
fn explicit_two_ofe_repository_test_seed(
    frame: &DirectRunFrame,
    authoring_latitude_degrees: Option<f64>,
    adaptive_owner: bool,
    duplicate_configured_mapping_poison: bool,
) -> Result<DirectSnowStage3V11ProductionSeedV1, HillslopeCliError> {
    use openwepp_persisted_restart_v1::restart_authority_two_ofe_owner_fixture;

    let mut owners = restart_authority_two_ofe_owner_fixture();
    openwepp_hillslope_orchestrator::v9_real_consumer_shadow::restart_authority_equilibrate_complete_owner_fixture(
        &mut owners.runtime.shadow,
    )
    .map_err(nested)?;
    let endpoint = &owners.runtime.endpoint;
    let interval_template =
        openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectV9ShadowIntervalInput {
            lse_forcing: endpoint.forcing.clone(),
            vegetation_forcing: endpoint.receipt.forcing().clone(),
            wb14_parameters: endpoint
                .lse_configuration
                .ofes
                .iter()
                .map(|ofe| DirectOfeWb14Parameters {
                    ofe_id: ofe.ofe_id.clone(),
                    effective_conductivity_m_s: 1.0e-6,
                    matric_potential_m: 0.1,
                    infiltration_storage_capacity_m: 0.04,
                })
                .collect(),
        };
    explicit_repository_test_seed_from_owner(
        frame,
        &owners,
        interval_template,
        authoring_latitude_degrees,
        adaptive_owner,
        duplicate_configured_mapping_poison,
    )
}

#[cfg(any(test, feature = "test-fixture-authority"))]
fn rebind_fixture_layer_inventory<T: Clone>(
    layers: &BTreeMap<String, T>,
    rebindings: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, T>, HillslopeCliError> {
    let source_layers = rebindings.keys().collect::<BTreeSet<_>>();
    let mut rebound = layers
        .iter()
        .filter(|(layer_id, _)| !source_layers.contains(layer_id))
        .map(|(layer_id, layer)| (layer_id.clone(), layer.clone()))
        .collect::<BTreeMap<_, _>>();
    for (source_layer_id, destination_layer_id) in rebindings {
        let layer = layers.get(source_layer_id).ok_or_else(|| {
            failure(format!(
                "fixture mineral inventory omits rebound source layer {source_layer_id}"
            ))
        })?;
        if rebound
            .insert(destination_layer_id.clone(), layer.clone())
            .is_some()
        {
            return Err(failure(format!(
                "fixture mineral inventory rebound target collides at {destination_layer_id}"
            )));
        }
    }
    if rebound.len() != layers.len() {
        return Err(failure(
            "fixture mineral inventory rebind is not an exact bijection",
        ));
    }
    Ok(rebound)
}

#[cfg(any(test, feature = "test-fixture-authority"))]
fn synthesize_fixture_zero_bgc_inventory_for_synthetic_roots<T: Default>(
    biogeochemistry_layers: &mut BTreeMap<String, T>,
    vegetation_configuration: &VegetationConfiguration,
    synthetic_root_ids: &BTreeSet<String>,
) -> Result<(), HillslopeCliError> {
    let configured_root_ids = vegetation_configuration
        .strata
        .iter()
        .flat_map(|stratum| {
            stratum
                .root_layers
                .iter()
                .map(|root| root.layer_id.as_str().to_owned())
        })
        .collect::<BTreeSet<_>>();
    let missing_root_ids = configured_root_ids
        .iter()
        .filter(|layer_id| !biogeochemistry_layers.contains_key(*layer_id))
        .cloned()
        .collect::<Vec<_>>();

    for layer_id in missing_root_ids {
        let proven_synthetic_zero = synthetic_root_ids.contains(&layer_id)
            && !vegetation_configuration.strata.is_empty()
            && vegetation_configuration.strata.iter().all(|stratum| {
                stratum
                    .root_layers
                    .iter()
                    .find(|root| root.layer_id.as_str() == layer_id)
                    .is_some_and(|root| {
                        root.root_fraction.to_bits() == 0.0_f64.to_bits()
                            && root.mineral_n_root_fraction.to_bits() == 0.0_f64.to_bits()
                    })
            });
        if !proven_synthetic_zero {
            return Err(failure(format!(
                "fixture vegetation root {layer_id} lacks BGC inventory and is not an exact-zero synthetic root"
            )));
        }
        if biogeochemistry_layers
            .insert(layer_id, T::default())
            .is_some()
        {
            return Err(failure(
                "fixture zero BGC inventory synthesis would overwrite an existing layer",
            ));
        }
    }

    validate_bgc_inventory_covers_vegetation_roots(vegetation_configuration, biogeochemistry_layers)
}

fn validate_bgc_inventory_covers_vegetation_roots<T>(
    vegetation_configuration: &VegetationConfiguration,
    biogeochemistry_layers: &BTreeMap<String, T>,
) -> Result<(), HillslopeCliError> {
    let missing = vegetation_configuration
        .strata
        .iter()
        .flat_map(|stratum| &stratum.root_layers)
        .map(|root| root.layer_id.as_str())
        .filter(|layer_id| !biogeochemistry_layers.contains_key(*layer_id))
        .collect::<BTreeSet<_>>();
    if !missing.is_empty() {
        return Err(failure(format!(
            "Stage-3 production seed vegetation roots omit exact BGC inventory identities: {missing:?}"
        )));
    }
    Ok(())
}

#[cfg(any(test, feature = "test-fixture-authority"))]
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn expand_checked_two_lane_owner_to_live_test_topology(
    frame: &DirectRunFrame,
    retain_downstream_configured_vegetation: bool,
    surface_configuration: &mut openwepp_hillslope_orchestrator::DirectSurfaceLiquidConfiguration,
    surface_state: &openwepp_hillslope_orchestrator::DirectSurfaceLiquidOwnedState,
    records: &mut Vec<openwepp_hillslope_orchestrator::DirectSurfaceLiquidConfigurationRecord>,
    lse_configuration: &mut LandSurfaceEnergyConfiguration,
    lse_state: &mut openwepp_land_surface_energy::LandSurfaceEnergyState,
    soil_thermal: &mut openwepp_land_surface_energy::SoilThermalSnapshot,
    interval_template: &mut openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectV9ShadowIntervalInput,
) -> Result<
    Option<BTreeMap<openwepp_hillslope_orchestrator::DirectSurfaceLiquidStoreKey, f64>>,
    HillslopeCliError,
> {
    if frame.lanes.len() < 2 {
        return Ok(None);
    }
    if surface_configuration.ofe_topology.len() != 2
        || surface_configuration.ofe_bindings.len() != 2
        || lse_configuration.ofes.len() != 2
        || soil_thermal.ofes.len() != 2
        || interval_template.wb14_parameters.len() != 2
    {
        return Err(failure(
            "multi-lane fixture authoring requires the checked routed two-OFE owner authority",
        ));
    }

    let template_topology = surface_configuration.ofe_topology.clone();
    let template_bindings = surface_configuration.ofe_bindings.clone();
    let template_records = records.clone();
    let template_lse_ofes = lse_configuration.ofes.clone();
    let template_lse_tiles = lse_state.tiles.clone();
    let template_thermal_ofes = soil_thermal.ofes.clone();
    let template_wb14 = interval_template.wb14_parameters.clone();
    let template_liquid = surface_state
        .records
        .iter()
        .map(|record| (record.key.clone(), record.liquid_kg_m2_tile))
        .collect::<BTreeMap<_, _>>();
    let routed_receiver_tile = template_records
        .iter()
        .find_map(|record| record.runon_destination_tile_id.clone())
        .ok_or_else(|| failure("checked two-OFE owner omits its routed receiver tile"))?;
    let receiver_tile_index = template_lse_ofes[1]
        .tiles
        .iter()
        .position(|tile| tile.tile_id == routed_receiver_tile)
        .ok_or_else(|| failure("checked routed receiver is absent from the LSE tile authority"))?;
    let upstream_open_tile = template_lse_ofes[0]
        .tiles
        .iter()
        .find(|tile| {
            matches!(
                tile.surface,
                openwepp_land_surface_energy::SurfaceConfiguration::BareMineralSoil { .. }
            )
        })
        .ok_or_else(|| failure("checked two-OFE owner omits its open LSE tile template"))?;
    let upstream_open_record = template_records
        .iter()
        .find(|record| {
            record.key.ofe_id == template_topology[0]
                && record.key.tile_id == upstream_open_tile.tile_id
        })
        .ok_or_else(|| failure("checked two-OFE owner omits its open surface store template"))?;

    let target_ofes = frame
        .lanes
        .iter()
        .map(|lane| OfeId::try_new(format!("ofe-{}", lane.lane_id)).map_err(nested))
        .collect::<Result<Vec<_>, _>>()?;
    let target_tile_ids = frame
        .lanes
        .iter()
        .enumerate()
        .map(|(lane_index, lane)| {
            let template_index = usize::from(lane_index != 0);
            template_lse_ofes[template_index]
                .tiles
                .iter()
                .enumerate()
                .map(|(tile_index, _)| {
                    TileId::try_new(format!(
                        "fixture-lane-{}-tile-{}",
                        lane.lane_id,
                        tile_index + 1
                    ))
                    .map_err(nested)
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut rebound_records = Vec::new();
    let mut rebound_liquid = BTreeMap::new();
    let mut rebound_bindings = Vec::with_capacity(frame.lanes.len());
    let mut rebound_lse_ofes = Vec::with_capacity(frame.lanes.len());
    let mut rebound_lse_tiles = Vec::new();
    let mut rebound_thermal_ofes = Vec::with_capacity(frame.lanes.len());
    let mut rebound_wb14 = Vec::with_capacity(frame.lanes.len());
    for (lane_index, lane) in frame.lanes.iter().enumerate() {
        let template_index = usize::from(lane_index != 0);
        let is_downstream = lane_index + 1 == frame.lanes.len();
        let template_ofe = &template_topology[template_index];
        let target_ofe = &target_ofes[lane_index];
        let template_lse = &template_lse_ofes[template_index];
        let target_tiles = &target_tile_ids[lane_index];
        if template_lse.tiles.len() != target_tiles.len() {
            return Err(failure(
                "checked two-OFE LSE tile authority changed while rebinding live lanes",
            ));
        }

        let mut lse_ofe = template_lse.clone();
        lse_ofe.ofe_id.clone_from(target_ofe);
        lse_ofe.area_m2 = lane.area_m2;
        for (tile_index, (tile, target_tile)) in
            lse_ofe.tiles.iter_mut().zip(target_tiles).enumerate()
        {
            if !retain_downstream_configured_vegetation || !is_downstream {
                tile.surface.clone_from(&upstream_open_tile.surface);
                tile.surface_heat_storage_mode = upstream_open_tile.surface_heat_storage_mode;
            }
            tile.tile_id.clone_from(target_tile);
            if !retain_downstream_configured_vegetation || !is_downstream {
                tile.vegetation_tile_id = TileId::try_new(format!(
                    "fixture-open-lane-{}-vegetation-{}",
                    lane.lane_id,
                    tile_index + 1
                ))
                .map_err(nested)?;
            }
        }
        rebound_lse_ofes.push(lse_ofe);

        for (template_tile_index, template_tile) in template_lse.tiles.iter().enumerate() {
            let mut state = template_lse_tiles
                .iter()
                .find(|state| {
                    state.ofe_id == *template_ofe && state.tile_id == template_tile.tile_id
                })
                .cloned()
                .ok_or_else(|| failure("checked two-OFE owner omits an LSE tile state"))?;
            state.ofe_id.clone_from(target_ofe);
            state.tile_id.clone_from(&target_tiles[template_tile_index]);
            rebound_lse_tiles.push(state);
        }

        let template_thermal = &template_thermal_ofes[template_index];
        if template_thermal.ofe_id != *template_ofe {
            return Err(failure(
                "checked two-OFE soil-thermal order differs from surface topology",
            ));
        }
        let mut thermal = template_thermal.clone();
        thermal.ofe_id.clone_from(target_ofe);
        rebound_thermal_ofes.push(thermal);

        let mut wb14 = template_wb14[template_index].clone();
        if wb14.ofe_id != *template_ofe {
            return Err(failure(
                "checked two-OFE WB14 order differs from surface topology",
            ));
        }
        wb14.ofe_id.clone_from(target_ofe);
        rebound_wb14.push(wb14);

        let mut binding = template_bindings[template_index].clone();
        binding.ofe_id.clone_from(target_ofe);
        binding.production_lane_index = lane_index;
        binding.production_lane_id = lane.lane_id;
        rebound_bindings.push(binding);

        let template_lane_records = template_records
            .iter()
            .filter(|record| record.key.ofe_id == *template_ofe)
            .collect::<Vec<_>>();
        if template_lane_records.len() != target_tiles.len() {
            return Err(failure(
                "checked surface and LSE tile cardinalities differ for a fixture OFE",
            ));
        }
        for template_record in template_lane_records {
            let tile_index = template_lse
                .tiles
                .iter()
                .position(|tile| tile.tile_id == template_record.key.tile_id)
                .ok_or_else(|| failure("checked surface tile is absent from LSE authority"))?;
            let mut record = if !retain_downstream_configured_vegetation || !is_downstream {
                let mut open = upstream_open_record.clone();
                open.tile_fraction = template_record.tile_fraction;
                open
            } else {
                template_record.clone()
            };
            record.key.run_id = frame.identity.run_id;
            record.key.ofe_id.clone_from(target_ofe);
            record.key.tile_id.clone_from(&target_tiles[tile_index]);
            record.key.surface_id = SurfaceId::try_new(format!(
                "fixture:surface:lane-{}:tile-{}",
                lane.lane_id,
                tile_index + 1
            ))
            .map_err(nested)?;
            record.key.source_id = SourceId::try_new(format!(
                "fixture:liquid:lane-{}:tile-{}",
                lane.lane_id,
                tile_index + 1
            ))
            .map_err(nested)?;
            record.ofe_area_m2 = lane.area_m2;
            if !retain_downstream_configured_vegetation || !is_downstream {
                record.ground_ingress_mode =
                    openwepp_hillslope_orchestrator::DirectGroundIngressMode::OpenRawPrecipitation;
            }
            if lane_index + 1 < frame.lanes.len() {
                record.runon_destination_ofe_id = Some(target_ofes[lane_index + 1].clone());
                record.runon_destination_tile_id =
                    Some(target_tile_ids[lane_index + 1][receiver_tile_index].clone());
            } else {
                record.runon_destination_ofe_id = None;
                record.runon_destination_tile_id = None;
            }
            let liquid = template_liquid
                .get(
                    if !retain_downstream_configured_vegetation || !is_downstream {
                        &upstream_open_record.key
                    } else {
                        &template_record.key
                    },
                )
                .copied()
                .ok_or_else(|| failure("checked surface owner omits a configured store"))?;
            if rebound_liquid.insert(record.key.clone(), liquid).is_some() {
                return Err(failure(
                    "live test topology produced a duplicate surface store identity",
                ));
            }
            rebound_records.push(record);
        }
    }

    surface_configuration.ofe_topology = target_ofes;
    surface_configuration.ofe_bindings = rebound_bindings;
    *records = rebound_records;
    lse_configuration.ofes = rebound_lse_ofes;
    lse_state.tiles = rebound_lse_tiles;
    soil_thermal.ofes = rebound_thermal_ofes;
    interval_template.wb14_parameters = rebound_wb14;
    Ok(Some(rebound_liquid))
}

#[cfg(any(test, feature = "test-fixture-authority"))]
#[allow(clippy::too_many_lines)]
fn explicit_repository_test_seed_from_owner(
    frame: &DirectRunFrame,
    fixture: &openwepp_persisted_restart_v1::RestartAuthorityOwnerFixture,
    mut interval_template: openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectV9ShadowIntervalInput,
    authoring_latitude_degrees: Option<f64>,
    adaptive_owner: bool,
    duplicate_configured_mapping_poison: bool,
) -> Result<DirectSnowStage3V11ProductionSeedV1, HillslopeCliError> {
    use openwepp_coupled_time::digest_bytes;
    use openwepp_persisted_restart_v1::{
        AcceptedIntervalCount, WireDayIndex, project_complete_owner_state_v1,
        restart_authority_identities,
    };

    let authority = &fixture.runtime.shadow;
    let mut surface_configuration = authority.restart_authority_surface_configuration().clone();
    let mut hydrology_frame = frame.clone();
    let mut surface_state = authority
        .restart_authority_hydrology_frame()
        .surface_liquid_shadow
        .as_deref()
        .ok_or_else(|| failure("explicit test owner fixture omits surface-liquid state"))?
        .clone();
    let mut gsi_owner_configuration = authority.gsi_owner_configuration().clone();
    let mut provider_static_configuration = authority.provider_static_configuration().clone();
    let mut provider_cursor = authority.provider_cursor().clone();
    let mut vegetation_configuration = authority
        .restart_authority_vegetation_configuration()
        .clone();
    let mut vegetation_state = authority.vegetation_state().clone();
    let mut lse_configuration = authority.restart_authority_lse_configuration().clone();
    let mut lse_state = authority.lse_state().clone();
    let mut soil_thermal = authority
        .restart_authority_soil_thermal()
        .map_err(nested)?
        .clone();
    let mut biogeochemistry = authority.restart_authority_biogeochemistry().clone();
    let mut layer_maps = fixture
        .runtime
        .endpoint
        .hydrology
        .restart_authority_layer_maps()
        .to_vec();
    let mut root_zone_hydraulic_configuration =
        authority.root_zone_hydraulic_configuration().clone();
    if let Some(latitude_degrees) = authoring_latitude_degrees {
        if adaptive_owner {
            vegetation_configuration.strata.clear();
            vegetation_state.0.occupancies.clear();
            vegetation_state.0.strata.clear();
            vegetation_state.0.tile_canopy_air.clear();
        }
        let run_id = frame.identity.run_id;
        let mut records = surface_configuration.records.clone();
        if adaptive_owner {
            for record in &mut records {
                record.ground_ingress_mode =
                    openwepp_hillslope_orchestrator::DirectGroundIngressMode::OpenRawPrecipitation;
            }
        }
        let expanded_surface_liquid = expand_checked_two_lane_owner_to_live_test_topology(
            frame,
            !adaptive_owner,
            &mut surface_configuration,
            &surface_state,
            &mut records,
            &mut lse_configuration,
            &mut lse_state.0,
            &mut soil_thermal,
            &mut interval_template,
        )?;
        if duplicate_configured_mapping_poison {
            let configured_tile = vegetation_configuration
                .expected_occupancies()
                .into_iter()
                .next()
                .ok_or_else(|| failure("duplicate mapping poison requires configured vegetation"))?
                .tile_id;
            let upstream_ofe = lse_configuration
                .ofes
                .first_mut()
                .ok_or_else(|| failure("duplicate mapping poison requires an upstream OFE"))?;
            let upstream_tile = upstream_ofe
                .tiles
                .first_mut()
                .ok_or_else(|| failure("duplicate mapping poison requires an upstream LSE tile"))?;
            upstream_tile.vegetation_tile_id = configured_tile;
            let upstream_record = records
                .iter_mut()
                .find(|record| {
                    record.key.ofe_id == upstream_ofe.ofe_id
                        && record.key.tile_id == upstream_tile.tile_id
                })
                .ok_or_else(|| failure("duplicate mapping poison omits its surface store"))?;
            upstream_record.ground_ingress_mode =
                openwepp_hillslope_orchestrator::DirectGroundIngressMode::CoveredCanopyRelease;
        }
        for record in &mut records {
            record.key.run_id = run_id;
            let topology_index = surface_configuration
                .ofe_topology
                .iter()
                .position(|ofe_id| ofe_id == &record.key.ofe_id)
                .ok_or_else(|| failure("fixture surface record omits its OFE topology"))?;
            record.ofe_area_m2 = frame
                .lanes
                .get(topology_index)
                .ok_or_else(|| failure("live frame omits fixture surface OFE lane"))?
                .area_m2;
        }
        if surface_configuration.ofe_bindings.len() != frame.lanes.len() {
            return Err(failure(format!(
                "test fixture owner lane cardinality {} does not match live frame {}",
                surface_configuration.ofe_bindings.len(),
                frame.lanes.len()
            )));
        }
        let live_layer_count = frame
            .lanes
            .first()
            .map(|lane| lane.subsurface_layers.len())
            .ok_or_else(|| failure("live test fixture frame omits lanes"))?;
        if live_layer_count == 0
            || frame
                .lanes
                .iter()
                .any(|lane| lane.subsurface_layers.len() != live_layer_count)
        {
            return Err(failure(
                "test fixture authority requires a common nonempty soil-layer topology",
            ));
        }
        let fixture_layer_ids = surface_configuration
            .ofe_bindings
            .first()
            .ok_or_else(|| failure("fixture surface owner omits OFE binding"))?
            .ordered_soil_layer_ids
            .clone();
        let common_layer_ids = (0..live_layer_count)
            .map(|layer_index| {
                fixture_layer_ids
                    .get(layer_index)
                    .cloned()
                    .map_or_else(
                        || SoilLayerId::try_new(format!("fixture-live-soil-{}", layer_index + 1)),
                        Ok,
                    )
                    .map_err(nested)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut rebound_layer_maps = Vec::with_capacity(frame.lanes.len());
        for (lane_index, (binding, lane)) in surface_configuration
            .ofe_bindings
            .iter_mut()
            .zip(&frame.lanes)
            .enumerate()
        {
            let layer_ids = common_layer_ids.clone();
            binding.production_lane_index = lane_index;
            binding.production_lane_id = lane.lane_id;
            binding.ordered_soil_layer_ids.clone_from(&layer_ids);
            binding.infiltration_soil_thermal_layer_id = layer_ids
                .first()
                .cloned()
                .ok_or_else(|| failure("live frame fixture lane omits subsurface layers"))?;
            rebound_layer_maps.push(RealHydrologyLaneLayerMap {
                ofe_lane: RealHydrologyOfeLaneId {
                    lane_index,
                    lane_id: lane.lane_id,
                },
                layer_ids,
            });
        }
        layer_maps = rebound_layer_maps;
        let mut mineral_layer_rebindings = BTreeMap::<String, String>::new();
        let mut synthetic_mineral_root_ids = BTreeSet::<String>::new();
        for stratum in &mut vegetation_configuration.strata {
            let root_template = stratum.root_layers.clone();
            if root_template.is_empty() {
                return Err(failure(
                    "vegetated fixture authority omits its root-layer distribution",
                ));
            }
            if root_template
                .iter()
                .skip(common_layer_ids.len())
                .any(|root| {
                    root.root_fraction.to_bits() != 0.0_f64.to_bits()
                        || root.mineral_n_root_fraction.to_bits() != 0.0_f64.to_bits()
                })
            {
                return Err(failure(
                    "live soil topology would omit a nonzero fixture root-layer fraction",
                ));
            }
            let trailing_template = root_template
                .last()
                .cloned()
                .ok_or_else(|| failure("vegetated fixture root template is empty"))?;
            for (source, destination) in root_template.iter().zip(&common_layer_ids) {
                let source = source.layer_id.as_str().to_owned();
                let destination = destination.as_str().to_owned();
                if mineral_layer_rebindings
                    .insert(source, destination.clone())
                    .is_some_and(|observed| observed != destination)
                {
                    return Err(failure(
                        "fixture vegetation strata disagree on live mineral-layer identity",
                    ));
                }
            }
            stratum.root_layers = common_layer_ids
                .iter()
                .enumerate()
                .map(|(layer_index, layer_id)| {
                    let mut root = root_template.get(layer_index).cloned().unwrap_or_else(|| {
                        synthetic_mineral_root_ids.insert(layer_id.as_str().to_owned());
                        let mut root = trailing_template.clone();
                        root.root_fraction = 0.0;
                        root.mineral_n_root_fraction = 0.0;
                        root
                    });
                    root.layer_id.clone_from(layer_id);
                    root
                })
                .collect();
        }
        biogeochemistry.layers =
            rebind_fixture_layer_inventory(&biogeochemistry.layers, &mineral_layer_rebindings)?;
        synthesize_fixture_zero_bgc_inventory_for_synthetic_roots(
            &mut biogeochemistry.layers,
            &vegetation_configuration,
            &synthetic_mineral_root_ids,
        )?;
        vegetation_configuration.configuration_sha256 = vegetation_configuration
            .canonical_sha256()
            .map_err(nested)?;
        vegetation_state
            .0
            .configuration_sha256
            .clone_from(&vegetation_configuration.configuration_sha256);
        vegetation_state.0.state_sha256 = vegetation_state.0.canonical_sha256();
        lse_configuration
            .vegetation_configuration
            .configuration_sha256 = openwepp_land_surface_energy::Sha256Digest::try_new(
            vegetation_configuration.configuration_sha256.clone(),
        )
        .map_err(nested)?;
        surface_configuration =
            openwepp_hillslope_orchestrator::DirectSurfaceLiquidConfiguration::new(
                surface_configuration.owner_id.clone(),
                run_id,
                surface_configuration.ofe_topology.clone(),
                surface_configuration.ofe_bindings.clone(),
                records,
            )
            .map_err(nested)?;
        let liquid_by_key = expanded_surface_liquid.unwrap_or_else(|| {
            surface_state
                .records
                .iter()
                .map(|record| {
                    let mut key = record.key.clone();
                    key.run_id = run_id;
                    (key, record.liquid_kg_m2_tile)
                })
                .collect::<BTreeMap<_, _>>()
        });
        let day_index = surface_state
            .continuations
            .first()
            .map_or(0, |continuation| continuation.day_index);
        surface_state =
            openwepp_hillslope_orchestrator::DirectSurfaceLiquidOwnedState::new_initial(
                &surface_configuration,
                &liquid_by_key,
                day_index,
            )
            .map_err(nested)?;

        if lse_configuration.ofes.len() != frame.lanes.len()
            || soil_thermal.ofes.len() != frame.lanes.len()
        {
            return Err(failure(
                "fixture LSE/soil OFE cardinality does not match the live frame",
            ));
        }
        for ((ofe, thermal_ofe), lane) in lse_configuration
            .ofes
            .iter_mut()
            .zip(&mut soil_thermal.ofes)
            .zip(&frame.lanes)
        {
            let interface_template = ofe.soil_interface_layers.clone();
            let thermal_template = thermal_ofe.ordered_layers.clone();
            let interface_fallback = interface_template
                .last()
                .cloned()
                .ok_or_else(|| failure("fixture LSE OFE omits soil interface layers"))?;
            let thermal_fallback = thermal_template
                .last()
                .cloned()
                .ok_or_else(|| failure("fixture soil owner omits thermal layers"))?;
            ofe.area_m2 = lane.area_m2;
            ofe.soil_interface_layers = common_layer_ids
                .iter()
                .zip(&lane.subsurface_layers)
                .enumerate()
                .map(|(layer_index, (layer_id, live_layer))| {
                    let mut layer = interface_template
                        .get(layer_index)
                        .cloned()
                        .unwrap_or_else(|| interface_fallback.clone());
                    layer.layer_id.clone_from(layer_id);
                    layer.thickness_m = live_layer.depth_m;
                    layer
                })
                .collect();
            thermal_ofe.ordered_layers = common_layer_ids
                .iter()
                .enumerate()
                .map(|(layer_index, layer_id)| {
                    let mut layer = thermal_template
                        .get(layer_index)
                        .cloned()
                        .unwrap_or_else(|| thermal_fallback.clone());
                    layer.layer_id.clone_from(layer_id);
                    layer
                })
                .collect();
        }
        lse_configuration.configuration_sha256 =
            openwepp_land_surface_energy::Sha256Digest::try_new("0".repeat(64)).map_err(nested)?;
        lse_configuration.configuration_sha256 =
            lse_configuration.canonical_sha256().map_err(nested)?;
        lse_state
            .0
            .configuration_sha256
            .clone_from(&lse_configuration.configuration_sha256);
        lse_state.0.state_sha256 = lse_state.0.canonical_sha256().map_err(nested)?;
        openwepp_hillslope_orchestrator::v9_real_consumer_shadow::restart_authority_seal_soil_thermal_digests(
            &mut soil_thermal,
        )
        .map_err(nested)?;

        let soil_forcing_template = interval_template.vegetation_forcing.soil_layers.clone();
        let soil_forcing_fallback = soil_forcing_template
            .last()
            .cloned()
            .ok_or_else(|| failure("fixture interval template omits soil forcing"))?;
        interval_template.vegetation_forcing.soil_layers = common_layer_ids
            .iter()
            .enumerate()
            .map(|(layer_index, layer_id)| {
                let mut forcing = soil_forcing_template
                    .get(layer_index)
                    .cloned()
                    .unwrap_or_else(|| soil_forcing_fallback.clone());
                forcing.layer_id.clone_from(layer_id);
                forcing
            })
            .collect();
        gsi_owner_configuration = openwepp_hillslope_orchestrator::runtime_inputs::DirectGsiOwnerConfigurationV1::try_new(
            gsi_owner_configuration.owner_id.clone(),
            gsi_owner_configuration.parameters(),
            latitude_degrees,
        )
        .map_err(nested)?;
        provider_static_configuration.run_id = run_id.to_string();
        provider_static_configuration
            .gsi_owner_configuration_sha256
            .clone_from(&gsi_owner_configuration.configuration_sha256);
        provider_static_configuration.destinations = lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                let wb14_configuration_sha256 = interval_template
                    .wb14_parameters
                    .iter()
                    .find(|parameter| parameter.ofe_id == ofe.ofe_id)
                    .map(restart_authority_wb14_parameter_sha256);
                ofe.tiles.iter().map(move |tile| {
                    wb14_configuration_sha256.as_ref().map(|wb14_sha256| {
                        openwepp_hillslope_orchestrator::runtime_inputs::SnowFreeHalfHourDestination {
                            ofe_id: ofe.ofe_id.as_str().to_owned(),
                            tile_id: tile.tile_id.as_str().to_owned(),
                            wb14_configuration_sha256: wb14_sha256.clone(),
                        }
                    })
                })
            })
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| failure("live LSE OFE omits its exact WB14 provider authority"))?;
        provider_cursor = openwepp_hillslope_orchestrator::runtime_inputs::SnowFreeHalfHourProviderCursor::default();

        let root_parameters = root_zone_hydraulic_configuration
            .ordered_layers()
            .iter()
            .map(|layer| {
                let (_, _, _, saturated_matric_potential_mm, clapp_hornberger_b) =
                    layer.restart_identity_fields();
                (saturated_matric_potential_mm, clapp_hornberger_b)
            })
            .collect::<Vec<_>>();
        let fallback_root_parameters = root_parameters
            .last()
            .copied()
            .ok_or_else(|| failure("fixture root-zone owner omits layer parameters"))?;
        let mut rebound_root_layers = Vec::new();
        for map in &layer_maps {
            for (layer_index, layer_id) in map.layer_ids.iter().enumerate() {
                let (saturated_matric_potential_mm, clapp_hornberger_b) = root_parameters
                    .get(layer_index)
                    .copied()
                    .unwrap_or(fallback_root_parameters);
                rebound_root_layers.push(
                    DirectRootZoneLayerConfiguration::try_new(
                        map.ofe_lane.lane_index,
                        map.ofe_lane.lane_id,
                        layer_id.clone(),
                        saturated_matric_potential_mm,
                        clapp_hornberger_b,
                    )
                    .map_err(nested)?,
                );
            }
        }
        let rebound_root_strata = if adaptive_owner {
            Vec::new()
        } else {
            root_zone_hydraulic_configuration.ordered_strata().to_vec()
        };
        root_zone_hydraulic_configuration =
            DirectRootZoneHydraulicConfiguration::try_new(rebound_root_layers, rebound_root_strata)
                .map_err(nested)?;
    }
    hydrology_frame
        .configure_surface_liquid_shadow(&surface_configuration, surface_state)
        .map_err(nested)?;
    let real_consumer = DirectV10RealConsumerShadow::try_new(
        vegetation_configuration,
        vegetation_state,
        authority.restart_authority_vegetation_owner_id().clone(),
        lse_configuration,
        lse_state,
        surface_configuration,
        layer_maps,
        soil_thermal,
        biogeochemistry,
        hydrology_frame,
        0,
        gsi_owner_configuration,
        authority.gsi_state().clone(),
        provider_static_configuration,
        provider_cursor,
        root_zone_hydraulic_configuration,
    )
    .map_err(nested)?;
    let day_inputs = real_consumer
        .restart_authority_hydrology_frame()
        .lanes
        .iter()
        .map(|lane| lane.day_inputs.clone())
        .collect::<Vec<_>>();
    let phase_plan_sha256 = fixture.phase_plan_sha256.clone();
    let day_input_digests = day_inputs
        .iter()
        .map(explicit_test_day_input_sha256)
        .collect::<Result<Vec<_>, _>>()?;
    let committed =
        project_complete_owner_state_v1(&real_consumer, &phase_plan_sha256, &day_input_digests, 0)
            .map_err(nested)?;
    let (run_identity_sha256, topology_sha256) = restart_authority_identities(
        &committed,
        real_consumer.root_zone_hydraulic_configuration(),
    );
    let mut checkpoint = DirectV10RealConsumerCheckpointV1 {
        schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".to_owned(),
        version: 1,
        run_identity_sha256,
        topology_sha256,
        phase: DirectV10CheckpointPhaseV1::BetweenDays {
            next_day_index: WireDayIndex(0),
            accepted_interval_count: AcceptedIntervalCount::try_new(0).map_err(nested)?,
            committed,
        },
        payload_sha256: openwepp_persisted_restart_v1::Sha256Hex::try_new("0".repeat(64))
            .map_err(nested)?,
    };
    checkpoint.seal().map_err(nested)?;
    let artifact = DirectSnowStage3V11ProductionSeedArtifactV1 {
        schema: SCHEMA.to_owned(),
        version: VERSION,
        vegetation_configuration: real_consumer.vegetation_configuration().clone(),
        vegetation_owner_id: real_consumer
            .restart_authority_vegetation_owner_id()
            .clone(),
        lse_configuration: real_consumer.lse_configuration().clone(),
        root_zone_hydraulic_configuration: real_consumer
            .root_zone_hydraulic_configuration()
            .clone(),
        wb14_parameters: interval_template.wb14_parameters.clone(),
        calendar_receipt: digest_bytes(b"runner-explicit-test-calendar-v1"),
        controller_policy: digest_bytes(b"runner-explicit-test-controller-v1"),
        support_static_authority: DirectSnowStage3V11SupportStaticAuthorityV1 {
            schema: SUPPORT_STATIC_SCHEMA.to_owned(),
            version: SUPPORT_STATIC_VERSION,
            interval_template,
            rho_air_kg_m3: 1.2,
            cp_air_j_kg_k: 1_005.0,
            underlying_surface_albedo: 0.2,
        },
        checkpoint,
    };
    let seed = DirectSnowStage3V11ProductionSeedV1 { artifact };
    seed.validate_envelope()?;
    Ok(seed)
}

#[cfg(any(test, feature = "test-fixture-authority"))]
pub(super) fn author_explicit_test_seed_bytes(
    frame: &DirectRunFrame,
    adaptive_owner: bool,
    latitude_degrees: f64,
) -> Result<Vec<u8>, HillslopeCliError> {
    let seed = match (adaptive_owner, frame.identity.lane_count) {
        (false, 1) => explicit_repository_test_seed(frame, Some(latitude_degrees))?,
        (false, 2..) => {
            explicit_two_ofe_repository_test_seed(frame, Some(latitude_degrees), false, false)?
        }
        (true, 1) => explicit_adaptive_repository_test_seed(frame, Some(latitude_degrees))?,
        (true, 2..) => {
            explicit_two_ofe_repository_test_seed(frame, Some(latitude_degrees), true, false)?
        }
        (_, 0) => {
            return Err(failure(
                "test fixture owner requires a nonempty live lane set",
            ));
        }
    };
    openwepp_persisted_restart_v1::to_canonical_bytes(&seed.artifact).map_err(nested)
}

/// Author the exact live-topology owner, then introduce one duplicate
/// configured vegetation mapping while keeping every restart/configuration
/// digest internally valid. This is a qualification poison only: the normal
/// adaptive transaction must be the boundary that rejects it.
#[cfg(test)]
pub(crate) fn duplicate_configured_vegetation_mapping_test_seed(
    frame: &DirectRunFrame,
    latitude_degrees: f64,
) -> Result<DirectSnowStage3V11ProductionSeedV1, HillslopeCliError> {
    if frame.identity.lane_count < 2 {
        return Err(failure(
            "duplicate configured-vegetation poison requires at least two OFEs",
        ));
    }
    explicit_two_ofe_repository_test_seed(frame, Some(latitude_degrees), false, true)
}

#[cfg(test)]
pub(crate) fn duplicate_configured_vegetation_mapping_test_seed_bytes(
    frame: &DirectRunFrame,
    latitude_degrees: f64,
) -> Result<Vec<u8>, HillslopeCliError> {
    let seed = duplicate_configured_vegetation_mapping_test_seed(frame, latitude_degrees)?;
    openwepp_persisted_restart_v1::to_canonical_bytes(&seed.artifact).map_err(nested)
}

#[cfg(any(test, feature = "test-fixture-authority"))]
fn explicit_test_day_input_sha256(
    inputs: &Vec<openwepp_hillslope_orchestrator::DirectDayConstructorInputs>,
) -> Result<Sha256Hex, HillslopeCliError> {
    use sha2::{Digest, Sha256};

    fn hexify_floats(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Number(number) if number.is_f64() => {
                if let Some(float) = number.as_f64() {
                    *value = serde_json::Value::String(format!("0x{:016x}", float.to_bits()));
                }
            }
            serde_json::Value::Array(values) => values.iter_mut().for_each(hexify_floats),
            serde_json::Value::Object(values) => values.values_mut().for_each(hexify_floats),
            _ => {}
        }
    }

    let mut projected = serde_json::to_value(inputs).map_err(nested)?;
    hexify_floats(&mut projected);
    let bytes = openwepp_persisted_restart_v1::to_canonical_bytes(&(
        "DirectDayConstructorInputsV1",
        projected,
    ))
    .map_err(nested)?;
    Sha256Hex::try_new(format!("{:x}", Sha256::digest(bytes))).map_err(nested)
}

fn validate_wb14_authority(
    committed: &openwepp_persisted_restart_v1::CompleteCommittedOwnerStateV1,
    parameters: &[DirectOfeWb14Parameters],
) -> Result<(), HillslopeCliError> {
    let parameter_ofes = parameters
        .iter()
        .map(|value| value.ofe_id.as_str())
        .collect::<Vec<_>>();
    let authenticated_topology_ofes = committed
        .surface_liquid_configuration
        .ofe_topology
        .iter()
        .map(openwepp_land_surface_energy::OfeId::as_str)
        .collect::<Vec<_>>();
    if parameters.is_empty()
        || parameter_ofes != authenticated_topology_ofes
        || parameter_ofes
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
            != parameters.len()
        || parameters.iter().any(|parameter| {
            !parameter.effective_conductivity_m_s.is_finite()
                || parameter.effective_conductivity_m_s <= 0.0
                || !parameter.matric_potential_m.is_finite()
                || parameter.matric_potential_m < 0.0
                || !parameter.infiltration_storage_capacity_m.is_finite()
                || parameter.infiltration_storage_capacity_m < 0.0
        })
    {
        return Err(failure(
            "Stage-3 production seed WB14 parameters must be nonempty, duplicate-free, and exactly match authenticated physical OFE topology order",
        ));
    }
    let destination_ofes = committed
        .static_forcing_configuration
        .destinations
        .iter()
        .map(|destination| destination.ofe_id.as_str())
        .collect::<BTreeSet<_>>();
    if destination_ofes != parameter_ofes.iter().copied().collect()
        || committed
            .static_forcing_configuration
            .destinations
            .iter()
            .any(|destination| {
                parameters
                    .iter()
                    .find(|parameter| parameter.ofe_id.as_str() == destination.ofe_id)
                    .is_none_or(|parameter| {
                        restart_authority_wb14_parameter_sha256(parameter)
                            != destination.wb14_configuration_sha256.as_str()
                    })
            })
    {
        return Err(failure(
            "Stage-3 production seed WB14 parameters do not match provider destination receipts",
        ));
    }
    Ok(())
}

fn validate_support_static_authority(
    committed: &openwepp_persisted_restart_v1::CompleteCommittedOwnerStateV1,
    root_zone: &DirectRootZoneHydraulicConfiguration,
    wb14_parameters: &[DirectOfeWb14Parameters],
    authority: &DirectSnowStage3V11SupportStaticAuthorityV1,
) -> Result<(), HillslopeCliError> {
    if authority.schema != SUPPORT_STATIC_SCHEMA || authority.version != SUPPORT_STATIC_VERSION {
        return Err(failure(format!(
            "Stage-3 support static authority schema/version must be {SUPPORT_STATIC_SCHEMA}/{SUPPORT_STATIC_VERSION}, observed {}/{}",
            authority.schema, authority.version
        )));
    }
    let template = &authority.interval_template;
    let lse = &template.lse_forcing;
    let vegetation = &template.vegetation_forcing;
    if !finite_positive(authority.rho_air_kg_m3)
        || !finite_positive(authority.cp_air_j_kg_k)
        || !authority.underlying_surface_albedo.is_finite()
        || !(0.0..=1.0).contains(&authority.underlying_surface_albedo)
        || lse.interval_s.to_bits() != 1_800.0_f64.to_bits()
        || lse.snow_present_at_beginning
        || lse.snow_present_at_end
        || lse.snow_terminal_payload_present
        || !lse.precipitation_parcels.is_empty()
        || !lse.runon_parcels.is_empty()
        || lse.validate(lse.transaction_id).is_err()
        || lse.canonical_sha256().ok().as_ref() != Some(&lse.forcing_sha256)
        || vegetation.co2_pa.to_bits()
            != committed
                .static_forcing_configuration
                .co2_pa
                .to_f64()
                .to_bits()
        || vegetation.reference_height_m.to_bits()
            != committed
                .static_forcing_configuration
                .reference_height_m
                .to_f64()
                .to_bits()
        || template.wb14_parameters != wb14_parameters
    {
        return Err(failure(
            "Stage-3 support static authority must carry an exact snow-free 1,800-second native interval template, explicit air properties, no precipitation/runon, and checkpoint provider/WB14 identity",
        ));
    }

    let template_layers = vegetation
        .soil_layers
        .iter()
        .map(|layer| layer.layer_id.as_str())
        .collect::<Vec<_>>();
    let configured_soil_layers = committed
        .scientific
        .soil_thermal
        .ofes
        .first()
        .map(|ofe| {
            ofe.ordered_layers
                .iter()
                .map(|layer| layer.layer_id.as_str())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let root_layer_sets_are_exact = committed
        .scientific
        .direct_hydrology
        .lanes
        .iter()
        .enumerate()
        .all(|(lane_index, lane)| {
            root_zone
                .ordered_layers()
                .iter()
                .filter_map(|layer| {
                    let (configured_lane_index, configured_lane_id, layer_id, _, _) =
                        layer.restart_identity_fields();
                    (configured_lane_index == lane_index && configured_lane_id == lane.lane_id)
                        .then_some(layer_id.as_str())
                })
                .eq(template_layers.iter().copied())
        });
    if template_layers.is_empty()
        || template_layers
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
            != template_layers.len()
        || template_layers != configured_soil_layers
        || !committed.scientific.soil_thermal.ofes.iter().all(|ofe| {
            ofe.ordered_layers
                .iter()
                .map(|layer| layer.layer_id.as_str())
                .eq(template_layers.iter().copied())
        })
        || !root_layer_sets_are_exact
    {
        return Err(failure(format!(
            "Stage-3 support interval template soil layers do not exactly match the committed soil/root topology: template={template_layers:?}, soil={configured_soil_layers:?}, root_exact={root_layer_sets_are_exact}"
        )));
    }

    Ok(())
}

fn finite_positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn validate_root_zone_configuration(
    configuration: &DirectRootZoneHydraulicConfiguration,
    vegetation_configuration: &VegetationConfiguration,
) -> Result<(), HillslopeCliError> {
    let layers = configuration.ordered_layers();
    let strata = configuration.ordered_strata();
    let layer_keys = layers
        .iter()
        .map(|layer| {
            let (lane_index, lane_id, layer_id, psi_sat_mm, b) = layer.restart_identity_fields();
            if !psi_sat_mm.is_finite() || psi_sat_mm >= 0.0 || !b.is_finite() || b <= 0.0 {
                return Err(failure(
                    "Stage-3 production seed root-zone layer scalar domain",
                ));
            }
            Ok((lane_index, lane_id, layer_id.clone()))
        })
        .collect::<Result<Vec<_>, HillslopeCliError>>()?;
    let stratum_keys = strata
        .iter()
        .map(|stratum| {
            let (stratum_id, path_m) = stratum.restart_identity_fields();
            if !path_m.is_finite() || path_m < 0.0 {
                return Err(failure(
                    "Stage-3 production seed root-zone stratum scalar domain",
                ));
            }
            Ok(stratum_id.clone())
        })
        .collect::<Result<Vec<_>, HillslopeCliError>>()?;
    if layers.is_empty()
        || strata.is_empty() != vegetation_configuration.strata.is_empty()
        || layer_keys.iter().cloned().collect::<BTreeSet<_>>().len() != layer_keys.len()
        || stratum_keys.iter().cloned().collect::<BTreeSet<_>>().len() != stratum_keys.len()
    {
        return Err(failure(
            "Stage-3 production seed root-zone configuration is empty or has duplicate identities",
        ));
    }
    Ok(())
}

fn digest32(value: &Sha256Hex) -> Result<Digest32, HillslopeCliError> {
    let text = value.as_str();
    let mut bytes = [0_u8; 32];
    for (index, chunk) in text.as_bytes().chunks_exact(2).enumerate() {
        let digits = std::str::from_utf8(chunk).map_err(nested)?;
        bytes[index] = u8::from_str_radix(digits, 16).map_err(nested)?;
    }
    Ok(Digest32::from_bytes(bytes))
}

fn project_laned_active_day_zero_surface_owner(
    configuration: &openwepp_hillslope_orchestrator::DirectSurfaceLiquidConfiguration,
    state: &openwepp_hillslope_orchestrator::DirectSurfaceLiquidOwnedState,
) -> Result<
    (
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidConfiguration,
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidOwnedState,
    ),
    HillslopeCliError,
> {
    if state
        .records
        .iter()
        .any(|record| record.last_accepted_transaction_id.is_some())
        || state.continuations.iter().any(|continuation| {
            continuation.day_index != 0
                || continuation.next_interval_index != 0
                || continuation.cumulative_supply_m.to_bits() != 0.0_f64.to_bits()
                || continuation.cumulative_infiltration_m.to_bits() != 0.0_f64.to_bits()
                || continuation.last_accepted_transaction_id.is_some()
        })
    {
        return Err(failure(
            "INV-OFEROUTE-015 active surface-owner projection requires an untouched day-zero state",
        ));
    }
    let mut records = configuration.records.clone();
    for record in &mut records {
        record.runon_destination_ofe_id = None;
        record.runon_destination_tile_id = None;
    }
    let projected_configuration =
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidConfiguration::new(
            configuration.owner_id.clone(),
            configuration.run_id,
            configuration.ofe_topology.clone(),
            configuration.ofe_bindings.clone(),
            records,
        )
        .map_err(nested)?;
    let liquid_by_key = state
        .records
        .iter()
        .map(|record| (record.key.clone(), record.liquid_kg_m2_tile))
        .collect::<BTreeMap<_, _>>();
    let projected_state =
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidOwnedState::new_initial(
            &projected_configuration,
            &liquid_by_key,
            0,
        )
        .map_err(nested)?;
    Ok((projected_configuration, projected_state))
}

fn failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "snow_stage3_v11_owner_seed",
        detail: format!("{SIMOUT_GUARD_ID} {}", detail.into()),
    }
}

fn nested(error: impl std::fmt::Display) -> HillslopeCliError {
    failure(error.to_string())
}

#[cfg(test)]
#[path = "snow_stage3_v11_production_seed_inventory_tests.rs"]
mod snow_stage3_v11_production_seed_inventory_tests;

#[cfg(test)]
#[path = "snow_stage3_v11_production_seed_v2_bootstrap_tests.rs"]
mod snow_stage3_v11_production_seed_v2_bootstrap_tests;

#[cfg(test)]
#[path = "snow_stage3_v11_production_seed_frozen_litter_v3_tests.rs"]
mod snow_stage3_v11_production_seed_frozen_litter_v3_tests;

#[cfg(test)]
#[path = "snow_stage3_v11_production_seed_frozen_litter_v4_tests.rs"]
mod snow_stage3_v11_production_seed_frozen_litter_v4_tests;
#[cfg(test)]
#[path = "snow_stage3_v11_production_seed_snow_enthalpy_v5_tests.rs"]
mod snow_stage3_v11_production_seed_snow_enthalpy_v5_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectV9ShadowIntervalInput;
    use openwepp_kernel_contract::{SoilLayerId, TransactionId};
    use openwepp_land_surface_energy::{LandSurfaceForcing, OfeId};
    use openwepp_vegetation::transaction::{SnowFreeForcing, SoilLayerForcing};

    fn explicit_support_static_authority() -> DirectSnowStage3V11SupportStaticAuthorityV1 {
        let mut lse_forcing = LandSurfaceForcing {
            forcing_sha256: Sha256Digest::try_new("0".repeat(64)).expect("digest"),
            transaction_id: TransactionId(1),
            interval_s: 1_800.0,
            air_temperature_k: 275.0,
            air_specific_humidity_kg_kg: 0.002,
            air_pressure_pa: 90_000.0,
            reference_wind_m_s: 2.0,
            neutral_stability: true,
            snow_present_at_beginning: false,
            snow_present_at_end: false,
            snow_terminal_payload_present: false,
            direct_vis_w_m2: 10.0,
            diffuse_vis_w_m2: 20.0,
            direct_nir_w_m2: 30.0,
            diffuse_nir_w_m2: 40.0,
            atmospheric_downward_longwave_w_m2: 250.0,
            precipitation_parcels: Vec::new(),
            runon_parcels: Vec::new(),
        };
        lse_forcing.forcing_sha256 = lse_forcing.canonical_sha256().expect("forcing digest");
        DirectSnowStage3V11SupportStaticAuthorityV1 {
            schema: SUPPORT_STATIC_SCHEMA.to_owned(),
            version: SUPPORT_STATIC_VERSION,
            interval_template: DirectV9ShadowIntervalInput {
                lse_forcing,
                vegetation_forcing: SnowFreeForcing {
                    air_temperature_k: 275.0,
                    pressure_pa: 90_000.0,
                    co2_pa: 40.0,
                    vapor_pressure_deficit_kpa: 0.5,
                    wind_m_s: 2.0,
                    rain_kg_m2: 0.0,
                    direct_par_w_m2: 10.0,
                    diffuse_par_w_m2: 20.0,
                    direct_nir_w_m2: 30.0,
                    diffuse_nir_w_m2: 40.0,
                    solar_zenith_cosine: 0.5,
                    ground_albedo_vis: 0.2,
                    ground_albedo_nir: 0.3,
                    longwave_down_w_m2: 250.0,
                    longwave_up_w_m2: 300.0,
                    specific_humidity: 0.002,
                    reference_height_m: 5.0,
                    soil_layers: vec![SoilLayerForcing {
                        layer_id: SoilLayerId::try_new("layer-1").expect("layer"),
                        water_beginning_kg_m2: 20.0,
                        matric_potential_mm: -100.0,
                        hydraulic_conductivity_mm_s: 0.01,
                        root_path_length_mm: 100.0,
                        gravity_root_mm: 10.0,
                        temperature_k: 274.0,
                        accessible: true,
                        frozen: false,
                    }],
                    gsi: 0.4,
                },
                wb14_parameters: vec![DirectOfeWb14Parameters {
                    ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                    effective_conductivity_m_s: 1.0e-6,
                    matric_potential_m: 0.1,
                    infiltration_storage_capacity_m: 0.04,
                }],
            },
            rho_air_kg_m3: 1.2,
            cp_air_j_kg_k: 1_005.0,
            underlying_surface_albedo: 0.2,
        }
    }

    fn wb14_physical_topology_fixture(
        ofe_count: usize,
    ) -> (
        openwepp_persisted_restart_v1::CompleteCommittedOwnerStateV1,
        Vec<DirectOfeWb14Parameters>,
    ) {
        let fixture = openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture();
        let mut committed = fixture.owners.committed.clone();
        let mut parameters = Vec::with_capacity(ofe_count);
        for lane_id in 1..=ofe_count {
            parameters.push(DirectOfeWb14Parameters {
                ofe_id: OfeId::try_new(format!("ofe-{lane_id}")).expect("physical OFE"),
                effective_conductivity_m_s: 1.0e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 0.04,
            });
        }
        committed.surface_liquid_configuration.ofe_topology = parameters
            .iter()
            .map(|parameter| parameter.ofe_id.clone())
            .collect();
        let destination_template = committed
            .static_forcing_configuration
            .destinations
            .first()
            .expect("fixture destination")
            .clone();
        committed.static_forcing_configuration.destinations = parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                let mut destination = destination_template.clone();
                destination.ofe_id = parameter.ofe_id.as_str().to_owned();
                destination.tile_id = format!("fixture-tile-{}", index + 1);
                destination.wb14_configuration_sha256 =
                    Sha256Hex::try_new(restart_authority_wb14_parameter_sha256(parameter))
                        .expect("WB14 receipt digest");
                destination
            })
            .collect();
        (committed, parameters)
    }

    fn assert_wb14_physical_topology_count(ofe_count: usize) {
        let (committed, parameters) = wb14_physical_topology_fixture(ofe_count);
        validate_wb14_authority(&committed, &parameters)
            .expect("numeric physical OFE topology order must be accepted exactly");
    }

    #[test]
    fn wb14_physical_topology_order_accepts_one_ofe() {
        assert_wb14_physical_topology_count(1);
    }

    #[test]
    fn wb14_physical_topology_order_accepts_nine_ofes() {
        assert_wb14_physical_topology_count(9);
    }

    #[test]
    fn wb14_physical_topology_order_accepts_ten_ofes() {
        assert_wb14_physical_topology_count(10);
    }

    #[test]
    fn wb14_physical_topology_order_accepts_nineteen_ofes() {
        assert_wb14_physical_topology_count(19);
    }

    #[test]
    fn wb14_physical_topology_rejects_duplicate_without_mutation() {
        let (committed, mut parameters) = wb14_physical_topology_fixture(10);
        parameters[9] = parameters[8].clone();
        let before_committed = committed.clone();
        let before_parameters = parameters.clone();
        assert!(validate_wb14_authority(&committed, &parameters).is_err());
        assert_eq!(committed, before_committed);
        assert_eq!(parameters, before_parameters);
    }

    #[test]
    fn wb14_physical_topology_rejects_reordered_without_mutation() {
        let (committed, mut parameters) = wb14_physical_topology_fixture(10);
        parameters.swap(8, 9);
        let before_committed = committed.clone();
        let before_parameters = parameters.clone();
        assert!(validate_wb14_authority(&committed, &parameters).is_err());
        assert_eq!(committed, before_committed);
        assert_eq!(parameters, before_parameters);
    }

    #[test]
    fn wb14_physical_topology_rejects_missing_without_mutation() {
        let (committed, mut parameters) = wb14_physical_topology_fixture(10);
        parameters.pop().expect("parameter to omit");
        let before_committed = committed.clone();
        let before_parameters = parameters.clone();
        assert!(validate_wb14_authority(&committed, &parameters).is_err());
        assert_eq!(committed, before_committed);
        assert_eq!(parameters, before_parameters);
    }

    #[test]
    fn wb14_physical_topology_rejects_foreign_without_mutation() {
        let (committed, mut parameters) = wb14_physical_topology_fixture(10);
        parameters[9].ofe_id = OfeId::try_new("ofe-foreign").expect("foreign OFE");
        let before_committed = committed.clone();
        let before_parameters = parameters.clone();
        assert!(validate_wb14_authority(&committed, &parameters).is_err());
        assert_eq!(committed, before_committed);
        assert_eq!(parameters, before_parameters);
    }

    fn persisted_day_zero_surface_owner() -> (
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidConfiguration,
        openwepp_hillslope_orchestrator::DirectSurfaceLiquidOwnedState,
    ) {
        let fixture = openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture();
        let configuration = fixture
            .owners
            .committed
            .surface_liquid_configuration
            .restore()
            .expect("persisted surface configuration");
        let state = fixture
            .owners
            .committed
            .scientific
            .direct_hydrology
            .surface_liquid_owned_state
            .as_deref()
            .expect("persisted surface state")
            .restore_with_configuration(&configuration)
            .expect("validated persisted surface state");
        (configuration, state)
    }

    #[test]
    fn active_laned_bootstrap_removes_surface_runon_and_preserves_day_zero_liquid_bits() {
        let (configuration, state) = persisted_day_zero_surface_owner();
        let source_ofe = configuration.records[0].key.ofe_id.clone();
        let destination_ofe = OfeId::try_new("ofe-2").expect("destination OFE");
        let destination_tile = configuration.records[0].key.tile_id.clone();
        let mut routed_records = configuration.records.clone();
        for record in routed_records
            .iter_mut()
            .filter(|record| record.key.ofe_id == source_ofe)
        {
            record.runon_destination_ofe_id = Some(destination_ofe.clone());
            record.runon_destination_tile_id = Some(destination_tile.clone());
        }
        let mut liquid_by_key = state
            .records
            .iter()
            .map(|record| (record.key.clone(), record.liquid_kg_m2_tile))
            .collect::<BTreeMap<_, _>>();
        for (index, source_record) in configuration.records.iter().enumerate() {
            let mut destination_record = source_record.clone();
            destination_record.key.ofe_id.clone_from(&destination_ofe);
            destination_record.key.surface_id =
                SurfaceId::try_new(format!("surface:ofe-2:{}", index + 1))
                    .expect("destination surface");
            destination_record.key.source_id =
                SourceId::try_new(format!("liquid:ofe-2:{}", index + 1))
                    .expect("destination source");
            destination_record.runon_destination_ofe_id = None;
            destination_record.runon_destination_tile_id = None;
            liquid_by_key.insert(
                destination_record.key.clone(),
                state.records[index].liquid_kg_m2_tile,
            );
            routed_records.push(destination_record);
        }
        let mut topology = configuration.ofe_topology.clone();
        topology.push(destination_ofe.clone());
        let mut bindings = configuration.ofe_bindings.clone();
        let mut destination_binding = bindings[0].clone();
        destination_binding.ofe_id = destination_ofe;
        destination_binding.production_lane_index = 1;
        destination_binding.production_lane_id = 2;
        bindings.push(destination_binding);
        let configuration = openwepp_hillslope_orchestrator::DirectSurfaceLiquidConfiguration::new(
            configuration.owner_id,
            configuration.run_id,
            topology,
            bindings,
            routed_records,
        )
        .expect("routed day-zero configuration");
        let state = openwepp_hillslope_orchestrator::DirectSurfaceLiquidOwnedState::new_initial(
            &configuration,
            &liquid_by_key,
            0,
        )
        .expect("routed day-zero state");
        let (projected_configuration, projected_state) =
            project_laned_active_day_zero_surface_owner(&configuration, &state)
                .expect("active Lane-D day-zero projection");

        assert_ne!(
            projected_configuration.configuration_sha256,
            configuration.configuration_sha256,
        );
        assert!(projected_configuration.records.iter().all(|record| {
            record.runon_destination_ofe_id.is_none() && record.runon_destination_tile_id.is_none()
        }));
        let before = state
            .records
            .iter()
            .map(|record| (record.key.clone(), record.liquid_kg_m2_tile.to_bits()))
            .collect::<BTreeMap<_, _>>();
        let after = projected_state
            .records
            .iter()
            .map(|record| (record.key.clone(), record.liquid_kg_m2_tile.to_bits()))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(after, before, "routing projection changed physical liquid");
        projected_state
            .validate(&projected_configuration)
            .expect("projected owner identity");
    }

    #[test]
    fn active_laned_bootstrap_rejects_non_day_zero_owner_without_mutation() {
        let (configuration, mut state) = persisted_day_zero_surface_owner();
        state.continuations[0].day_index = 1;
        let before_configuration = configuration.clone();
        let before_state = state.clone();
        assert!(project_laned_active_day_zero_surface_owner(&configuration, &state).is_err());
        assert_eq!(configuration, before_configuration);
        assert_eq!(state, before_state);
    }

    #[test]
    fn production_seed_is_required_and_has_no_default() {
        let error = DirectSnowStage3V11ProductionSeedV1::load_required(None)
            .expect_err("missing production seed must fail");
        assert!(error.to_string().contains("no fixture/default owner seed"));
    }

    #[test]
    fn explicit_repository_test_owner_seed_bootstraps_exact_fresh_frame() {
        let fixture = openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture();
        let mut frame = fixture
            .owners
            .runtime
            .shadow
            .restart_authority_hydrology_frame()
            .clone();
        frame.surface_liquid_shadow = None;
        let seed = explicit_repository_test_seed(&frame, None).expect("explicit owner seed");
        seed.bootstrap(&mut frame)
            .expect("bootstrap explicit owner seed");
        assert!(frame.snow_stage3_v11_attachment.is_some());
    }

    #[test]
    fn explicit_adaptive_owner_seed_bootstraps_exact_no_strata_frame() {
        let fixture =
            openwepp_persisted_restart_v1::restart_authority_adaptive_prepared_day_fixture();
        let mut frame = fixture
            .owners
            .runtime
            .shadow
            .restart_authority_hydrology_frame()
            .clone();
        frame.surface_liquid_shadow = None;
        let seed = explicit_adaptive_repository_test_seed(&frame, None)
            .expect("sealed adaptive no-strata owner seed");
        assert!(seed.artifact.vegetation_configuration.strata.is_empty());
        assert!(
            seed.artifact
                .root_zone_hydraulic_configuration
                .ordered_strata()
                .is_empty()
        );
        let populated_root_zone =
            openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture();
        let mismatch = validate_root_zone_configuration(
            populated_root_zone
                .owners
                .runtime
                .shadow
                .root_zone_hydraulic_configuration(),
            &seed.artifact.vegetation_configuration,
        )
        .expect_err("root strata without vegetation strata must fail closed");
        assert!(mismatch.to_string().contains("root-zone configuration"));
        seed.bootstrap(&mut frame)
            .expect("bootstrap adaptive no-strata owner seed");
        assert!(frame.snow_stage3_v11_attachment.is_some());
    }

    #[test]
    fn live_root_layer_rebind_preserves_exact_mineral_inventory_and_rejects_aliases() {
        let layers = [
            ("soil-1".to_owned(), (1_u64, 2_u64)),
            ("soil-2".to_owned(), (3_u64, 4_u64)),
            ("soil-dry".to_owned(), (5_u64, 6_u64)),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>();
        let rebindings = [
            ("soil-1".to_owned(), "thermal-1".to_owned()),
            ("soil-2".to_owned(), "thermal-2".to_owned()),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>();
        let rebound = rebind_fixture_layer_inventory(&layers, &rebindings)
            .expect("fixture-only live-root inventory rebind");
        assert_eq!(rebound.len(), layers.len());
        assert_eq!(rebound.get("thermal-1"), Some(&(1, 2)));
        assert_eq!(rebound.get("thermal-2"), Some(&(3, 4)));
        assert_eq!(rebound.get("soil-dry"), Some(&(5, 6)));
        assert!(!rebound.contains_key("soil-1"));
        assert!(!rebound.contains_key("soil-2"));

        let missing = [("soil-missing".to_owned(), "thermal-1".to_owned())]
            .into_iter()
            .collect();
        assert!(
            rebind_fixture_layer_inventory(&layers, &missing)
                .expect_err("missing mineral source must fail closed")
                .to_string()
                .contains("omits rebound source layer")
        );

        let collision = [("soil-1".to_owned(), "soil-dry".to_owned())]
            .into_iter()
            .collect();
        assert!(
            rebind_fixture_layer_inventory(&layers, &collision)
                .expect_err("mineral target alias must fail closed")
                .to_string()
                .contains("rebound target collides")
        );
    }

    #[test]
    fn live_two_layer_seed_binds_vegetation_roots_to_exact_bgc_inventory() {
        let fixture = openwepp_persisted_restart_v1::restart_authority_prepared_day_fixture();
        let authority_bgc = fixture
            .owners
            .runtime
            .shadow
            .restart_authority_biogeochemistry();
        let mut frame = fixture
            .owners
            .runtime
            .shadow
            .restart_authority_hydrology_frame()
            .clone();
        for lane in &mut frame.lanes {
            lane.subsurface_layers.truncate(2);
            lane.water.soil_water_m = lane
                .subsurface_layers
                .iter()
                .map(|layer| layer.theta_m)
                .sum();
        }
        frame.surface_liquid_shadow = None;
        let seed = explicit_repository_test_seed(&frame, Some(41.1))
            .expect("live two-layer explicit owner seed");
        let DirectV10CheckpointPhaseV1::BetweenDays { committed, .. } =
            &seed.artifact.checkpoint.phase
        else {
            panic!("explicit owner seed must begin between days");
        };
        let projected = &committed.scientific.biogeochemistry.layers;
        assert_eq!(
            projected
                .iter()
                .map(|layer| layer.layer_id.as_str())
                .collect::<Vec<_>>(),
            vec!["soil-dry", "soil-frozen", "thermal-1", "thermal-2"]
        );
        for (source, destination) in [("soil-1", "thermal-1"), ("soil-2", "thermal-2")] {
            let source = authority_bgc
                .layers
                .get(source)
                .expect("fixture source mineral inventory");
            let destination = projected
                .iter()
                .find(|layer| layer.layer_id == destination)
                .expect("projected live mineral inventory");
            assert_eq!(
                destination.ammonium_n.to_f64().to_bits(),
                source.ammonium_n.to_bits()
            );
            assert_eq!(
                destination.nitrate_n.to_f64().to_bits(),
                source.nitrate_n.to_bits()
            );
        }
        for stratum in &seed.artifact.vegetation_configuration.strata {
            assert!(stratum.root_layers.iter().all(|root| {
                projected
                    .iter()
                    .any(|layer| layer.layer_id == root.layer_id.as_str())
            }));
        }
    }

    #[test]
    fn exact_sha256_identity_converts_to_digest_bytes() {
        let sha = Sha256Hex::try_new("01".repeat(32)).expect("sha");
        let digest = digest32(&sha).expect("digest");
        assert_eq!(digest.as_bytes(), &[1_u8; 32]);
    }

    #[test]
    fn explicit_support_static_artifact_round_trips_without_defaults() {
        let artifact = explicit_support_static_authority();
        let bytes = serde_json::to_vec(&artifact).expect("serialize authority");
        let restored: DirectSnowStage3V11SupportStaticAuthorityV1 =
            serde_json::from_slice(&bytes).expect("deserialize authority");
        assert_eq!(restored, artifact);
        assert_eq!(
            restored
                .interval_template()
                .lse_forcing
                .interval_s
                .to_bits(),
            1_800.0_f64.to_bits()
        );
        assert_eq!(restored.rho_air_kg_m3().to_bits(), 1.2_f64.to_bits());
        assert_eq!(restored.cp_air_j_kg_k().to_bits(), 1_005.0_f64.to_bits());
        assert_eq!(
            restored.underlying_surface_albedo().to_bits(),
            0.2_f64.to_bits()
        );
    }

    #[test]
    fn support_static_artifact_rejects_runtime_snow_and_missing_air_authority() {
        let mut value =
            serde_json::to_value(explicit_support_static_authority()).expect("authority value");
        value["runtime_swe_m"] = serde_json::json!(0.1);
        assert!(
            serde_json::from_value::<DirectSnowStage3V11SupportStaticAuthorityV1>(value).is_err()
        );

        let mut missing =
            serde_json::to_value(explicit_support_static_authority()).expect("authority value");
        missing
            .as_object_mut()
            .expect("authority object")
            .remove("cp_air_j_kg_k");
        assert!(
            serde_json::from_value::<DirectSnowStage3V11SupportStaticAuthorityV1>(missing).is_err()
        );
    }
}
