pub(crate) fn direct_v11_bgc_debit_scope(
    vegetation_configuration: &VegetationConfiguration,
    lse_configuration: &LandSurfaceEnergyConfiguration,
) -> Result<V11BgcDebitScope, DirectV11RealConsumerError> {
    v11_bgc_debit_scope(vegetation_configuration, lse_configuration)
}

#[derive(Debug, Error)]
pub enum DirectV11RealConsumerError {
    #[error(transparent)]
    Runtime(#[from] DirectV10RealConsumerError),
    #[error(transparent)]
    Vegetation(#[from] openwepp_vegetation::v11::V11Error),
    #[error("V11 owner serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("V11 actual-consumer identity mismatch: {0}")]
    Identity(&'static str),
    #[error("V11 zero-duration snow-liquid receiver rejected: {0}")]
    ZeroDurationSnowLiquid(String),
    #[error(
        "V11 open-snow lower-boundary domain at lane {lane_id} {ofe_id}/{tile_id} support {start_ns}..{end_ns} ns: snow_temperature_k={snow_temperature_k:?}, latent_heat_j_kg={latent_heat_j_kg:?}, sensible_outward_w_m2={sensible_outward_w_m2:?}, vapor_outward_kg_m2_s={vapor_outward_kg_m2_s:?}, net_longwave_w_m2={net_longwave_w_m2:?}, shortwave_absorbed_w_m2={shortwave_absorbed_w_m2:?}, albedo={albedo:?}, beginning_stage3={beginning_stage3}, forcing={forcing}, exposure={exposure}, optical={optical}, longwave={longwave}"
    )]
    OpenSnowLowerBoundaryDomain {
        lane_id: u32,
        ofe_id: String,
        tile_id: String,
        start_ns: u128,
        end_ns: u128,
        snow_temperature_k: f64,
        latent_heat_j_kg: f64,
        sensible_outward_w_m2: f64,
        vapor_outward_kg_m2_s: f64,
        net_longwave_w_m2: f64,
        shortwave_absorbed_w_m2: f64,
        albedo: f64,
        beginning_stage3: String,
        forcing: String,
        exposure: String,
        optical: String,
        longwave: String,
    },
    #[error(
        "V11 component-carrier reference-flux custody at {ofe_id}/{tile_id} support {start_ns}..{end_ns} ns boundary={boundary_receipt_sha256:?}: canopy_sensible={canopy_sensible_w_m2:?}, snow_sensible={snow_sensible_w_m2:?}, reconstructed_sensible={reconstructed_sensible_w_m2:?}, stated_sensible={stated_sensible_w_m2:?}, sensible_delta={sensible_delta_w_m2:?}, sensible_allowance={sensible_allowance_w_m2:?}; canopy_vapor={canopy_vapor_kg_m2_s:?}, snow_vapor={snow_vapor_kg_m2_s:?}, reconstructed_vapor={reconstructed_vapor_kg_m2_s:?}, stated_vapor={stated_vapor_kg_m2_s:?}, vapor_delta={vapor_delta_kg_m2_s:?}, vapor_allowance={vapor_allowance_kg_m2_s:?}"
    )]
    ComponentCarrierReferenceFluxCustody {
        ofe_id: String,
        tile_id: String,
        start_ns: u128,
        end_ns: u128,
        boundary_receipt_sha256: Digest32,
        canopy_sensible_w_m2: f64,
        snow_sensible_w_m2: f64,
        reconstructed_sensible_w_m2: f64,
        stated_sensible_w_m2: f64,
        sensible_delta_w_m2: f64,
        sensible_allowance_w_m2: f64,
        canopy_vapor_kg_m2_s: f64,
        snow_vapor_kg_m2_s: f64,
        reconstructed_vapor_kg_m2_s: f64,
        stated_vapor_kg_m2_s: f64,
        vapor_delta_kg_m2_s: f64,
        vapor_allowance_kg_m2_s: f64,
    },
    #[error("V11 adaptive candidate requires refinement: {0}")]
    AdaptiveRefinement(&'static str),
    #[error(transparent)]
    CoveredBoundary(#[from] SnowStage3HandoffError),
    #[error(transparent)]
    Stage3(#[from] DirectSnowStage3EvaluationError),
    #[error("SNOWENERGY-E-PRECIP-001: {0}")]
    Stage3PrecipitationCustody(&'static str),
    #[error("SNOWENERGY-E-SOIL-HEAT-001: {0}")]
    Stage3SnowSoilHeatCustody(&'static str),
}
