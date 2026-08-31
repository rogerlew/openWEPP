// Restart-authority fixture normalization for the production adaptive gate.
// This is deliberately feature-gated evidence plumbing. It changes no
// production forcing or serialization surface.

#[cfg(feature = "restart-authority-evidence")]
pub fn restart_authority_equilibrate_complete_owner_fixture(
    shadow: &mut DirectV10RealConsumerShadow,
) -> Result<(), DirectV10RealConsumerError> {
    const EQUILIBRIUM_TEMPERATURE_K: f64 = 273.15;
    const EQUILIBRIUM_PRESSURE_PA: f64 = 101_325.0;

    // SC-VEGETATION-001 Table 5.2 at exactly 273.15 K: every
    // positive-order polynomial term is zero.
    let saturation_pressure_pa = 100.0 * 6.112_134_76;
    let humidity =
        0.622 * saturation_pressure_pa / (EQUILIBRIUM_PRESSURE_PA - 0.378 * saturation_pressure_pa);

    for shared in shadow.vegetation_state.0.strata.values_mut() {
        for pool in shared.tissues.values_mut() {
            *pool = openwepp_vegetation::carbon_nitrogen::TissuePool::default();
        }
        shared.retranslocation_n = 0.0;
        shared.nsc_c = 0.0;
        shared.xs_c = 0.0;
        shared.phase = openwepp_vegetation::transaction::PhenologyPhase::Dormant;
        shared.onset_remaining_s = 0.0;
        shared.offset_remaining_s = 0.0;
        shared.previous_gsi = 1.0;
        shared.pending_transfers.clear();
        shared.leaf_area = 0.0;
        shared.stem_area = 0.0;
        shared.root_area = 0.0;
    }
    for occupancy in shadow.vegetation_state.0.occupancies.values_mut() {
        occupancy.sun_leaf_temperature_k = EQUILIBRIUM_TEMPERATURE_K;
        occupancy.shade_leaf_temperature_k = EQUILIBRIUM_TEMPERATURE_K;
        occupancy.dry_stem_temperature_k = EQUILIBRIUM_TEMPERATURE_K;
        occupancy.wet_surface_temperature_k = EQUILIBRIUM_TEMPERATURE_K;
        occupancy.canopy_liquid_kg_h2o_m2_tile_ground = 0.0;
        occupancy.root_node_potential_mm = 0.0;
        occupancy.stem_potential_mm = 0.0;
        occupancy.sun_leaf_potential_mm = 0.0;
        occupancy.shade_leaf_potential_mm = 0.0;
        occupancy.beta_hyd = 1.0;
    }
    for canopy_air in shadow.vegetation_state.0.tile_canopy_air.values_mut() {
        canopy_air.canopy_air_temperature_k = EQUILIBRIUM_TEMPERATURE_K;
        canopy_air.canopy_air_specific_humidity_kg_kg = humidity;
    }
    shadow.vegetation_state.0.state_sha256 = shadow.vegetation_state.0.canonical_sha256();
    shadow
        .vegetation_state
        .validate(&shadow.vegetation_configuration)?;
    let (v9_configuration, v9_state) =
        project_v10_runtime_to_v9(&shadow.vegetation_configuration, &shadow.vegetation_state)?;
    if v9_configuration != shadow.inner.vegetation_configuration {
        return Err(DirectV10RealConsumerError::RootConfigurationIdentity(
            "restart equilibrium vegetation projection",
        ));
    }
    shadow.inner.vegetation_state = v9_state;

    for tile in &mut shadow.lse_state.0.tiles {
        tile.surface_enthalpy_j_m2_tile_ground = 0.0;
        tile.surface_temperature_warm_start_k = EQUILIBRIUM_TEMPERATURE_K;
    }
    shadow.lse_state.0.state_sha256 = shadow.lse_state.0.canonical_sha256()?;
    shadow.lse_state.validate(&shadow.lse_configuration)?;
    let (v8_configuration, _) =
        project_v9_runtime_to_v8(&v9_configuration, &shadow.inner.vegetation_state)
            .map_err(DirectV9RealConsumerError::V9)?;
    let v8_configuration_sha256 = Sha256Digest::try_new(v8_configuration.configuration_sha256)?;
    let (v1_configuration, v1_state) = project_v2_runtime_to_v1(
        &shadow.lse_configuration,
        &shadow.lse_state,
        &v8_configuration_sha256,
    )?;
    if v1_configuration != shadow.inner.lse_configuration {
        return Err(DirectV10RealConsumerError::RootConfigurationIdentity(
            "restart equilibrium LSE projection",
        ));
    }
    shadow.inner.lse_state = v1_state;

    let transaction_id = shadow.vegetation_state.0.last_transaction_id;
    if shadow.inner.biogeochemistry.last_transaction_id != transaction_id {
        return Err(DirectV10RealConsumerError::RootConfigurationIdentity(
            "restart equilibrium beginning-owner lineage",
        ));
    }
    {
        let soil_thermal = shadow.inner.soil_thermal.v1_mut()?;
        for ofe in &mut soil_thermal.ofes {
            for layer in &mut ofe.ordered_layers {
                layer.temperature_k = EQUILIBRIUM_TEMPERATURE_K;
                layer.enthalpy_j_m2_ofe_ground = 0.0;
            }
        }
        soil_thermal.last_accepted_transaction_id = Some(TransactionId(transaction_id));
        restart_authority_seal_soil_thermal_digests(soil_thermal)?;
        soil_thermal.validate()?;
        restart_authority_validate_soil_thermal_digests(soil_thermal)?;
    }
    Ok(())
}
