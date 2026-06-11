
/// WB11 hydrology production kernel for ET/perc/lateral/drain lanes.
#[derive(Debug, Clone, Default)]
pub struct Wb11HydrologyKernel;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SnowHourlyState {
    hour: usize,
    depth_before_m: f64,
    depth_available_m: f64,
    density_before_kg_m3: f64,
    depth_after_m: f64,
    density_after_kg_m3: f64,
    rain_retained_m: f64,
    rain_released_m: f64,
    melt_raw_m: f64,
    melt_m: f64,
    melt_amelt_in: f64,
    melt_bmelt_in: f64,
    melt_cmelt_in: f64,
    melt_dmelt_in: f64,
    melt_hrtef_f: f64,
    melt_hrdtf_f: f64,
    melt_vwmph: f64,
    melt_rainin: f64,
    melt_wind_adjustment: f64,
    melt_branch_active: f64,
    dewpoint_c: f64,
    wind_m_s: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SnowMeltTerms {
    amelt_in: f64,
    bmelt_in: f64,
    cmelt_in: f64,
    dmelt_in: f64,
    hrtef_f: f64,
    hrdtf_f: f64,
    vwmph: f64,
    rainin: f64,
    wind_adjustment: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SnowMeltComputation {
    wmelt_m: f64,
    terms: SnowMeltTerms,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snowsci_stage1_hourly_state(melt_m: f64) -> SnowHourlyState {
        SnowHourlyState {
            hour: 1,
            depth_before_m: 0.0,
            depth_available_m: 0.0,
            density_before_kg_m3: 0.0,
            depth_after_m: 0.0,
            density_after_kg_m3: 0.0,
            rain_retained_m: 0.0,
            rain_released_m: 0.0,
            melt_raw_m: melt_m,
            melt_m,
            melt_amelt_in: 0.0,
            melt_bmelt_in: 0.0,
            melt_cmelt_in: 0.0,
            melt_dmelt_in: 0.0,
            melt_hrtef_f: 0.0,
            melt_hrdtf_f: 0.0,
            melt_vwmph: 0.0,
            melt_rainin: 0.0,
            melt_wind_adjustment: 0.0,
            melt_branch_active: 1.0,
            dewpoint_c: 0.0,
            wind_m_s: 0.0,
        }
    }

    #[test]
    fn snowsci_stage1_mixed_signed_melt_routes_authoritative_pack_loss() {
        let positive_pack_loss_m = 0.007_376_104_224;
        let negative_raw_melt_m = -0.006_171_157_610;
        let mut hourly_state = [
            snowsci_stage1_hourly_state(positive_pack_loss_m),
            snowsci_stage1_hourly_state(negative_raw_melt_m),
        ];

        let redistribution =
            Wb11HydrologyKernel::redistribute_daily_signed_snowmelt(&mut hourly_state);
        let routed_hourly_sum_m = hourly_state
            .iter()
            .map(|hourly| hourly.melt_m)
            .sum::<f64>();

        assert!(
            (redistribution.routed_melt_total_m - positive_pack_loss_m).abs() <= 1.0e-12
        );
        assert!(
            (redistribution.snowpack_state_loss_m - positive_pack_loss_m).abs() <= 1.0e-12
        );
        assert!((routed_hourly_sum_m - positive_pack_loss_m).abs() <= 1.0e-12);
        assert!(hourly_state.iter().all(|hourly| hourly.melt_m >= 0.0));
    }

    #[test]
    fn hphys0250_wb15_interception_scale_canonicalizes_near_zero_liquid_roundoff() {
        let (liquid_after_interception, rainfall_scale) =
            Wb11HydrologyKernel::resolve_interception_rainfall_scale(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                0.001_08,
                0.0,
                2.168_404_344_971_009e-19,
            )
            .expect("within-tolerance liquid roundoff should canonicalize");

        assert!(liquid_after_interception.abs() < f64::EPSILON);
        assert!(rainfall_scale.abs() < f64::EPSILON);
    }

    #[test]
    fn fq3dc_wb15_accepts_finite_non_negative_corn_vdmt_above_legacy_cap() {
        let mut state_surface = std::collections::BTreeMap::new();
        let flux_surface = std::collections::BTreeMap::new();
        state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.72));
        state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(2.4));
        state_surface.insert(
            BoundarySymbol::from("vdmt"),
            BoundaryValue::scalar(2.4),
        );
        let request = HillslopeKernelRequest::with_phase_context(
            "runoff_reconciliation",
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            HillslopeConsumerAdapter::Runoff,
            None,
            &state_surface,
            &flux_surface,
        );

        let interception = Wb11HydrologyKernel::compute_canopy_interception_depth(
            &request,
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            0.004,
        )
        .expect("finite non-negative plant biomass should be valid WB15 input");

        assert!(interception.is_finite());
        assert!(interception > 0.0);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SnowCouplingOutcome {
    signed_s: f64,
    accumulation: f64,
    rain_retained: f64,
    rain_released: f64,
    runtime_swe: f64,
    runtime_depth_m: f64,
    runtime_density_kg_m3: f64,
    runtime_settle_day_count: f64,
    hourly_state: Vec<SnowHourlyState>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SnowMeltRedistributionOutcome {
    routed_melt_total_m: f64,
    snowpack_state_loss_m: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct FrostCouplingOutcome {
    dfrost: f64,
    dthaw: f64,
    nft: f64,
    ws_frz: f64,
    infcap_frz: f64,
    soil_water_after_frwatc: Option<f64>,
    frwatc_soil_water_before: f64,
    frwatc_soil_water_after: f64,
    frwatc_frozen_water_before: f64,
    frwatc_frozen_water_after: f64,
    frwatc_freeze_debit: f64,
    frwatc_thaw_credit: f64,
    frwatc_net_liquid_delta: f64,
    frdp_m: f64,
    thdp_m: f64,
    tfrdp_m: f64,
    tthawd_m: f64,
    profile_depth_m: f64,
    fgthwd_flag: f64,
    total_fine_layer_count: f64,
    conductivity_tilled_w_m_k: f64,
    conductivity_untilled_w_m_k: f64,
    conductivity_residue_w_m_k: f64,
    hourly_state: [FrostHourlyState; SIMIMPL29_HOURS_PER_DAY],
    layer_topology_state: Vec<FrostLayerTopologyState>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FrostHourlyState {
    hour: usize,
    qsrf_w_m2: f64,
    quf_w_m2: f64,
    ksrf_w_m_k: f64,
    snow_depth_m: f64,
    residue_depth_m: f64,
    tilled_frozen_depth_m: f64,
    untilled_frozen_depth_m: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FrostLayerTopologyState {
    layer_index: usize,
    fine_layer_count: usize,
    fine_layer_thickness_m: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum IrrigationScheduleSource {
    Depletion,
    FixedDate,
}

impl IrrigationScheduleSource {
    const fn as_scalar(self) -> f64 {
        match self {
            Self::Depletion => 1.0,
            Self::FixedDate => 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ActiveIrrigationEvent {
    source: IrrigationScheduleSource,
    event_index: usize,
    system_type: f64,
    depth_m: f64,
    duration_s: f64,
    rate_m_per_s: f64,
}

const SNOW_RUNTIME_DEPTH_M_SYMBOL: &str = "snow.runtime_depth_m";
const SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL: &str = "snow.runtime_density_kg_m3";
const SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL: &str = "snow.runtime_settle_day_count";

const SNOW_HOURLY_DEPTH_BEFORE_ROOT: &str = "snow.hourly.depth_before_m";
const SNOW_HOURLY_DEPTH_AVAILABLE_ROOT: &str = "snow.hourly.depth_available_m";
const SNOW_HOURLY_DENSITY_BEFORE_ROOT: &str = "snow.hourly.density_before_kg_m3";
const SNOW_HOURLY_DEPTH_AFTER_ROOT: &str = "snow.hourly.depth_after_m";
const SNOW_HOURLY_DENSITY_AFTER_ROOT: &str = "snow.hourly.density_after_kg_m3";
const SNOW_HOURLY_MELT_ROOT: &str = "snow.hourly.melt_m";
const SNOW_HOURLY_MELT_RAW_ROOT: &str = "snow.hourly.melt_raw_m";
const SNOW_HOURLY_MELT_AMELT_ROOT: &str = "snow.hourly.melt_amelt_in";
const SNOW_HOURLY_MELT_BMELT_ROOT: &str = "snow.hourly.melt_bmelt_in";
const SNOW_HOURLY_MELT_CMELT_ROOT: &str = "snow.hourly.melt_cmelt_in";
const SNOW_HOURLY_MELT_DMELT_ROOT: &str = "snow.hourly.melt_dmelt_in";
const SNOW_HOURLY_MELT_HRTEF_ROOT: &str = "snow.hourly.melt_hrtef_f";
const SNOW_HOURLY_MELT_HRDTF_ROOT: &str = "snow.hourly.melt_hrdtf_f";
const SNOW_HOURLY_MELT_VWMPH_ROOT: &str = "snow.hourly.melt_vwmph";
const SNOW_HOURLY_MELT_RAININ_ROOT: &str = "snow.hourly.melt_rainin";
const SNOW_HOURLY_MELT_WIND_ADJUSTMENT_ROOT: &str = "snow.hourly.melt_wind_adjustment";
const SNOW_HOURLY_MELT_BRANCH_ACTIVE_ROOT: &str = "snow.hourly.melt_branch_active";
const SNOW_HOURLY_RAIN_ROOT: &str = "snow.hourly.rain_m";
const SNOW_HOURLY_RAIN_RETAINED_ROOT: &str = "snow.hourly.rain_retained_m";
const SNOW_HOURLY_RAIN_RELEASED_ROOT: &str = "snow.hourly.rain_released_m";
const SNOW_HOURLY_SNOWFALL_ROOT: &str = "snow.hourly.snowfall_m";

const WINTER_HOURLY_RAD_ROOT: &str = "winter.hourly.rad_mj_m2";
const WINTER_HOURLY_AIR_TEMP_ROOT: &str = "winter.hourly.air_temp_c";
const WINTER_HOURLY_CLOUD_ROOT: &str = "winter.hourly.cloud_fraction";
const WINTER_HOURLY_DEWPOINT_ROOT: &str = "winter.hourly.dewpoint_c";
const WINTER_HOURLY_WIND_ROOT: &str = "winter.hourly.wind_m_s";
const FROST_HOURLY_QSRF_ROOT: &str = "frost.hourly.qsrf_w_m2";
const FROST_HOURLY_QUF_ROOT: &str = "frost.hourly.quf_w_m2";
const FROST_HOURLY_KSRF_ROOT: &str = "frost.hourly.ksrf_w_m_k";
const FROST_HOURLY_SNOW_DEPTH_ROOT: &str = "frost.hourly.snow_depth_m";
const FROST_HOURLY_RESIDUE_DEPTH_ROOT: &str = "frost.hourly.residue_depth_m";
const FROST_HOURLY_TILLED_FROZEN_DEPTH_ROOT: &str = "frost.hourly.tilled_frozen_depth_m";
const FROST_HOURLY_UNTILLED_FROZEN_DEPTH_ROOT: &str = "frost.hourly.untilled_frozen_depth_m";
const FROST_RUNTIME_FRDP_M_SYMBOL: &str = "frost.runtime_frdp_m";
const FROST_RUNTIME_THDP_M_SYMBOL: &str = "frost.runtime_thdp_m";
const FROST_RUNTIME_TFRDP_M_SYMBOL: &str = "frost.runtime_tfrdp_m";
const FROST_RUNTIME_TTHAWD_M_SYMBOL: &str = "frost.runtime_tthawd_m";
const FROST_RUNTIME_FGTHWD_FLAG_SYMBOL: &str = "frost.runtime_fgthwd_flag";
const FROST_RUNTIME_TOTAL_FINE_LAYER_COUNT_SYMBOL: &str = "frost.runtime_total_fine_layer_count";
const FROST_RUNTIME_LAYER_FINE_COUNT_ROOT: &str = "frost.runtime_nfine";
const FROST_RUNTIME_LAYER_FINE_THICKNESS_ROOT: &str = "frost.runtime_fine_thickness_m";
const FROST_RUNTIME_CONDUCTIVITY_TILLED_SYMBOL: &str = "frost.runtime_kftill_w_m_k";
const FROST_RUNTIME_CONDUCTIVITY_UNTILLED_SYMBOL: &str = "frost.runtime_kfutil_w_m_k";
const FROST_RUNTIME_CONDUCTIVITY_RESIDUE_SYMBOL: &str = "frost.runtime_kres_w_m_k";
const FROST_RUNTIME_FRWATC_SOIL_WATER_BEFORE_SYMBOL: &str =
    "frost.runtime_frwatc_soil_water_before_m";
const FROST_RUNTIME_FRWATC_SOIL_WATER_AFTER_SYMBOL: &str =
    "frost.runtime_frwatc_soil_water_after_m";
const FROST_RUNTIME_FRWATC_FROZEN_WATER_BEFORE_SYMBOL: &str =
    "frost.runtime_frwatc_frozen_water_before_m";
const FROST_RUNTIME_FRWATC_FROZEN_WATER_AFTER_SYMBOL: &str =
    "frost.runtime_frwatc_frozen_water_after_m";
const FROST_RUNTIME_FRWATC_FREEZE_DEBIT_SYMBOL: &str =
    "frost.runtime_frwatc_freeze_debit_m";
const FROST_RUNTIME_FRWATC_THAW_CREDIT_SYMBOL: &str =
    "frost.runtime_frwatc_thaw_credit_m";
const FROST_RUNTIME_FRWATC_NET_LIQUID_DELTA_SYMBOL: &str =
    "frost.runtime_frwatc_net_liquid_delta_m";
const FROST_RUNTIME_SNOW_DEPTH_SYMBOL: &str = "snow.runtime_depth_m";
const FROST_RUNTIME_RESIDUE_DEPTH_SYMBOL: &str = "frost.runtime_residue_depth_m";
const FROST_LANDUSE_CLASS_PROXY_SYMBOL: &str = "landuse.class_proxy";
const FROST_RUNTIME_TILLAGE_DEPTH_M: f64 = 0.20;
const FROST_RUNTIME_KFTILL_W_M_K: f64 = 1.75;
const FROST_RUNTIME_KFUTIL_W_M_K: f64 = 2.1;
const FROST_RUNTIME_KRES_BASE_W_M_K: f64 = 0.05;
const FROST_RUNTIME_LATENT_HEAT_WATER_J_M3: f64 = 3.35e8;
const FROST_RUNTIME_SECONDS_PER_HOUR: f64 = 3_600.0;
const FROST_RUNTIME_UNFROZEN_HEAT_PATH_M: f64 = 1.0;
const FROST_RUNTIME_STABLE_SOIL_TEMP_C: f64 = 7.0;

const SIMIMPL29_HOURS_PER_DAY: usize = 24;
const SIMIMPL29_SNOW_DENSITY_CAP_KG_M3: f64 = 522.0;
const SIMIMPL29_DENSITY_MELT_GATE_KG_M3: f64 = 350.0;
const SIMIMPL29_SNOWPACK_SETTLE_BASE: f64 = 0.041_666_7;
const SIMIMPL29_CANOPY_FACTOR: f64 = 1.0;
const SIMIMPL29_WIND_MEASUREMENT_HEIGHT_M: f64 = 10.0;
const SIMIMPL29_SNOWPACK_STATE_LOSS_OVERDRAW_TOLERANCE_M: f64 = 0.005;
const WB14_INTERVAL_INFILTRATION_ROUNDOFF_TOLERANCE_M: f64 = 1.0e-9;
// UNIT-CONVERSION-ALLOW: mm_m_scale legacy minimum snow-depth threshold in meters, not conversion.
const SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M: f64 = 0.001;


mod support_helpers_mod;
