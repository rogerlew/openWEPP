const SNOWDENSITY09_DENSITY_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL";
const SNOWDENSITY1035_PHASE_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1035_PHASE_MODEL";
const SNOWDENSITY1037_MELT_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1037_MELT_MODEL";
const SNOWDENSITY1038_MELT_MODEL_ENV: &str = "OPENWEPP_SNOWDENSITY1038_MELT_MODEL";
const PARADIGM2_STAGE3_LIQUID_MODEL_ENV: &str = "OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL";
const SNOW_SURFACE_LONGWAVE_MODEL_ENV: &str = "OPENWEPP_SNOW_SURFACE_LONGWAVE_MODEL";
const SNOW_SURFACE_SUBLIMATION_MODEL_ENV: &str = "OPENWEPP_SNOW_SURFACE_SUBLIMATION_MODEL";

#[allow(clippy::result_large_err)]
fn stage3_v11_nonforest_canopy_cover(
    beginning_canopy_cover_fraction: f64,
) -> Result<f64, DirectSnowStage3V11AttachmentError> {
    if beginning_canopy_cover_fraction.is_finite()
        && (0.0..1.0).contains(&beginning_canopy_cover_fraction)
    {
        Ok(beginning_canopy_cover_fraction)
    } else {
        Err(DirectSnowStage3V11AttachmentError::Support(
            "management-derived nonforest canopy cover domain",
        ))
    }
}

#[derive(Clone, Copy)]
struct Stage3V11NativeForestCanopyOperands {
    evergreen_fraction: f64,
    summer_foliar_biomass_kg_m2: f64,
    structural_canopy_cover_fraction: f64,
    canopy_cover_coefficient_m2_kg: f64,
}

#[allow(clippy::result_large_err)]
fn stage3_v11_canopy_cover_from_authority(
    native_forest: Option<Stage3V11NativeForestCanopyOperands>,
    growing_season_index: f64,
    beginning_canopy_cover_fraction: f64,
) -> Result<f64, DirectSnowStage3V11AttachmentError> {
    let Some(native_forest) = native_forest else {
        return stage3_v11_nonforest_canopy_cover(beginning_canopy_cover_fraction);
    };
    if !growing_season_index.is_finite() || !(0.0..=1.0).contains(&growing_season_index) {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "repository GSI canopy domain",
        ));
    }
    let foliar_activity = native_forest.evergreen_fraction
        + (1.0 - native_forest.evergreen_fraction) * growing_season_index;
    let live_foliar_biomass = native_forest.summer_foliar_biomass_kg_m2 * foliar_activity;
    let cover = native_forest
        .structural_canopy_cover_fraction
        .max(1.0 - (-native_forest.canopy_cover_coefficient_m2_kg * live_foliar_biomass).exp())
        .min(openwepp_plant_phenology::FOREST_CANOPY_COVER_CAP);
    if !cover.is_finite() || !(0.0..1.0).contains(&cover) {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "native canopy cover domain",
        ));
    }
    Ok(cover)
}

fn reject_retired_stage3_snow_selector_envs() -> Result<(), HillslopeCliError> {
    for name in [
        SNOWDENSITY09_DENSITY_MODEL_ENV,
        SNOWDENSITY1035_PHASE_MODEL_ENV,
        SNOWDENSITY1037_MELT_MODEL_ENV,
        SNOWDENSITY1038_MELT_MODEL_ENV,
        PARADIGM2_STAGE3_LIQUID_MODEL_ENV,
        SNOW_SURFACE_LONGWAVE_MODEL_ENV,
        SNOW_SURFACE_SUBLIMATION_MODEL_ENV,
        "OPENWEPP_SNOW_TERMINAL_ENTHALPY_EVENT",
    ] {
        if std::env::var_os(name).is_some() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "stage3_v11_owner",
                detail: format!(
                    "{SIMOUT_GUARD_ID} retired snow selector {name} is not admitted; the adaptive compositional Stage-3 owner has one typed production configuration"
                ),
            });
        }
    }
    Ok(())
}
