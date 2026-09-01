use openwepp_hillslope_orchestrator::{
    runtime_inputs::restart_authority_project_gsi_state,
    v9_real_consumer_shadow::DirectV10RealConsumerShadow,
};

use crate::{
    BiogeochemistryStateRestartV1, CompleteCommittedOwnerStateV1,
    DirectGsiOwnerConfigurationRestartV1, DirectGsiOwnerStateRestartV1,
    DirectHydrologyExactEnthalpyRestartV2, DirectHydrologyRestartV1,
    DirectSurfaceLiquidConfigurationRestartV1, ExpectedSnowStage3V11ExactResidentContextsV4,
    ExpectedStage3CommittedDayArchiveV3, LseV2StateRestartV1, ScientificOwnerStateSetV1, Sha256Hex,
    SnowFreeHalfHourProviderCursorRestartV1, SnowFreeHalfHourStaticConfigurationRestartV1,
    SnowStage3V11ExactResidentSetV4, SoilThermalStateRestartV1, VegetationV10StateRestartV1,
};

/// Complete projection inputs for the additive hydrology V2 supplement. The
/// resident set is independently reconstructed by the checkpoint caller and
/// rejoined to the live attachment by `DirectHydrologyExactEnthalpyRestartV2`.
pub struct DirectHydrologyExactEnthalpyProjectionInputsV2<'a> {
    pub archive: &'a ExpectedStage3CommittedDayArchiveV3<'a>,
    pub exact_residents: SnowStage3V11ExactResidentSetV4,
    pub exact_contexts: &'a ExpectedSnowStage3V11ExactResidentContextsV4<'a>,
}

fn frame_contains_v4(shadow: &DirectV10RealConsumerShadow) -> bool {
    exact_v4_custody_present(
        shadow.frozen_litter_v4_resident().is_some(),
        shadow
            .restart_authority_hydrology_frame()
            .snow_stage3_v11_attachment
            .as_deref()
            .is_some_and(|attachment| attachment.restart_authority_contains_frozen_litter_v4()),
    )
}

#[inline]
const fn exact_v4_custody_present(direct_resident: bool, nested_resident: bool) -> bool {
    direct_resident || nested_resident
}

fn guard_legacy_projection(
    exact_v4_custody: bool,
    allow_exact_parent: bool,
) -> Result<(), &'static str> {
    if exact_v4_custody && !allow_exact_parent {
        Err("hydrology V1 projection would omit exact V4 custody")
    } else {
        Ok(())
    }
}

/// Project every scientific owner from the actual default-off consumer.
pub fn project_scientific_owner_state_v1(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) -> Result<ScientificOwnerStateSetV1, &'static str> {
    project_scientific_owner_state_v1_inner(shadow, phase_plan_sha256, day_input_digests, false)
}

fn project_scientific_owner_state_v1_inner(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
    allow_exact_parent: bool,
) -> Result<ScientificOwnerStateSetV1, &'static str> {
    guard_legacy_projection(frame_contains_v4(shadow), allow_exact_parent)?;
    Ok(ScientificOwnerStateSetV1 {
        vegetation_v10: VegetationV10StateRestartV1::project(
            shadow.vegetation_state(),
            shadow.restart_authority_vegetation_configuration(),
            shadow.restart_authority_vegetation_owner_id(),
        )
        .map_err(|_| "vegetation projection")?,
        lse_v2: LseV2StateRestartV1::project(
            shadow.lse_state(),
            shadow.restart_authority_lse_configuration(),
        )
        .map_err(|_| "LSE projection")?,
        direct_hydrology: if allow_exact_parent {
            DirectHydrologyRestartV1::project_for_exact_parent(
                shadow.restart_authority_hydrology_frame(),
                phase_plan_sha256.clone(),
                day_input_digests,
            )
        } else {
            DirectHydrologyRestartV1::project(
                shadow.restart_authority_hydrology_frame(),
                phase_plan_sha256.clone(),
                day_input_digests,
            )
        }
        .map_err(|_| "hydrology projection")?,
        soil_thermal: SoilThermalStateRestartV1::project(
            shadow
                .restart_authority_soil_thermal()
                .map_err(|_| "V1 soil resident projection")?,
        )
        .map_err(|_| "soil projection")?,
        biogeochemistry: BiogeochemistryStateRestartV1::project(
            shadow.restart_authority_biogeochemistry(),
        )
        .map_err(|_| "biogeochemistry projection")?,
    })
}

pub fn project_exact_hydrology_state_v2(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
    inputs: DirectHydrologyExactEnthalpyProjectionInputsV2<'_>,
) -> Result<DirectHydrologyExactEnthalpyRestartV2, &'static str> {
    if !frame_contains_v4(shadow) {
        return Err("hydrology V2 projection requires live exact V4 custody");
    }
    DirectHydrologyExactEnthalpyRestartV2::project(
        shadow.restart_authority_hydrology_frame(),
        phase_plan_sha256.clone(),
        day_input_digests,
        inputs.archive,
        inputs.exact_residents,
        inputs.exact_contexts,
    )
    .map_err(|_| "hydrology exact V2 projection")
}

pub(crate) fn project_scientific_owner_state_v1_for_exact_parent(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) -> Result<ScientificOwnerStateSetV1, &'static str> {
    project_scientific_owner_state_v1_inner(shadow, phase_plan_sha256, day_input_digests, true)
}

/// Project the complete between-days owner envelope from the actual consumer.
pub fn project_complete_owner_state_v1(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
    expected_next_day_index: usize,
) -> Result<CompleteCommittedOwnerStateV1, &'static str> {
    project_complete_owner_state_v1_inner(
        shadow,
        phase_plan_sha256,
        day_input_digests,
        expected_next_day_index,
        false,
    )
}

pub(crate) fn project_complete_owner_state_v1_for_exact_parent(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
    expected_next_day_index: usize,
) -> Result<CompleteCommittedOwnerStateV1, &'static str> {
    project_complete_owner_state_v1_inner(
        shadow,
        phase_plan_sha256,
        day_input_digests,
        expected_next_day_index,
        true,
    )
}

fn project_complete_owner_state_v1_inner(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
    expected_next_day_index: usize,
    allow_exact_parent: bool,
) -> Result<CompleteCommittedOwnerStateV1, &'static str> {
    let native_gsi_state = restart_authority_project_gsi_state(shadow.gsi_state())
        .map_err(|_| "GSI native projection")?;
    Ok(CompleteCommittedOwnerStateV1 {
        gsi_configuration: DirectGsiOwnerConfigurationRestartV1::project(
            shadow.gsi_owner_configuration(),
        )
        .map_err(|_| "GSI configuration projection")?,
        gsi_state: DirectGsiOwnerStateRestartV1::project(&native_gsi_state)
            .map_err(|_| "GSI state projection")?,
        static_forcing_configuration: SnowFreeHalfHourStaticConfigurationRestartV1::project(
            shadow.provider_static_configuration(),
        )
        .map_err(|_| "forcing configuration projection")?,
        provider_cursor: SnowFreeHalfHourProviderCursorRestartV1::project(
            shadow.provider_cursor(),
            shadow.provider_static_configuration(),
            expected_next_day_index,
        )
        .map_err(|_| "provider cursor projection")?,
        surface_liquid_configuration: DirectSurfaceLiquidConfigurationRestartV1::project(
            shadow.restart_authority_surface_configuration(),
        )
        .map_err(|_| "surface configuration projection")?,
        scientific: project_scientific_owner_state_v1_inner(
            shadow,
            phase_plan_sha256,
            day_input_digests,
            allow_exact_parent,
        )?,
    })
}

/// Derive the released run and topology identities from a complete owner set.
pub fn checkpoint_identities_v1(
    committed: &CompleteCommittedOwnerStateV1,
    root_zone_hydraulic_configuration: &openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectRootZoneHydraulicConfiguration,
) -> Result<(Sha256Hex, Sha256Hex), &'static str> {
    let hydrology = &committed.scientific.direct_hydrology;
    let run = Sha256Hex::try_new(
        crate::canonical_sha256(&(
            hydrology.run_id,
            hydrology.hillslope_id,
            hydrology.lane_count,
            hydrology.day_count,
        ))
        .map_err(|_| "run identity projection")?,
    )
    .map_err(|_| "run identity projection")?;
    let topology = serde_json::json!({
        "ordered_lanes": hydrology.lanes.iter().map(|lane| serde_json::json!({
            "lane_id": lane.lane_id,
            "upstream_lane_id": lane.upstream_lane_id,
            "downstream_lane_id": lane.downstream_lane_id,
            "soil_layer_count": lane.subsurface_layers.len(),
        })).collect::<Vec<_>>(),
        "ordered_ofe_tiles": committed.static_forcing_configuration.destinations.iter().map(|destination| (
            &destination.ofe_id, &destination.tile_id, &destination.wb14_configuration_sha256,
        )).collect::<Vec<_>>(),
        "lse_tiles": committed.scientific.lse_v2.tiles.iter().map(|tile| (&tile.ofe_id, &tile.tile_id)).collect::<Vec<_>>(),
        "soil_thermal_layer_maps": committed.scientific.soil_thermal.ofes.iter().map(|ofe| (
            &ofe.ofe_id,
            ofe.ordered_layers.iter().map(|layer| &layer.layer_id).collect::<Vec<_>>(),
        )).collect::<Vec<_>>(),
        "root_zone_hydraulic_configuration_sha256": root_zone_hydraulic_configuration
            .restart_identity_sha256()
            .map_err(|_| "root-zone configuration identity projection")?,
    });
    let topology = Sha256Hex::try_new(
        crate::canonical_sha256(&topology).map_err(|_| "topology identity projection")?,
    )
    .map_err(|_| "topology identity projection")?;
    Ok((run, topology))
}

#[cfg(test)]
mod tests {
    use super::{exact_v4_custody_present, guard_legacy_projection};

    #[test]
    fn restart_selection_detects_direct_and_nested_v4_custody() {
        assert!(exact_v4_custody_present(true, false));
        assert!(exact_v4_custody_present(false, true));
        assert!(exact_v4_custody_present(true, true));
        assert!(!exact_v4_custody_present(false, false));
    }

    #[test]
    fn ordinary_v1_refuses_exact_custody_but_v3_only_and_exact_parent_are_admitted() {
        let direct_v4 = exact_v4_custody_present(true, false);
        let nested_v4 = exact_v4_custody_present(false, true);
        let v3_only = exact_v4_custody_present(false, false);

        assert!(guard_legacy_projection(direct_v4, false).is_err());
        assert!(guard_legacy_projection(nested_v4, false).is_err());
        assert!(guard_legacy_projection(v3_only, false).is_ok());
        assert!(guard_legacy_projection(direct_v4, true).is_ok());
        assert!(guard_legacy_projection(nested_v4, true).is_ok());
    }
}
