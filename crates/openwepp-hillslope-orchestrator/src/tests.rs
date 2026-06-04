use std::cell::Cell;
use std::collections::BTreeMap;

use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeAnnualDecompositionAction,
    HillslopeAnnualDecompositionControl, HillslopeAnnualGrowthAction, HillslopeAnnualGrowthControl,
    HillslopeConsumerAdapter, HillslopeDecompositionKernelContext,
    HillslopeDecompositionManagementClass, HillslopeDecompositionTransitionControl,
    HillslopeDecompositionTransitionPayload, HillslopeGrowthKernelContext,
    HillslopeGrowthManagementClass, HillslopeGrowthStateSurface, HillslopeGrowthTransitionControl,
    HillslopeGrowthTransitionPayload, HillslopeKernel, HillslopeKernelPhaseClass,
    HillslopeKernelRequest, HillslopePerennialDecompositionAction,
    HillslopePerennialDecompositionControl, HillslopePerennialGrowthAction,
    HillslopePerennialGrowthControl, KernelRunResponse, KernelWritebackPayload,
    WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID, WritebackDecisionOutcome, WritebackField,
};
use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, StatusClassification};
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};

use super::schedule_export::{
    ScheduleDiagnostic, ScheduleExport, diff_schedule_json, render_schedule_diff,
    validate_hillslope_schedule_graph,
};
use super::{
    HillslopePhase, HillslopePhaseGraph, HillslopePhaseScheduler, HillslopeWritebackSurface,
    SchedulerOutcomeClass, Wb11HydrologyKernel, hillslope_consumer_adapter_for_phase,
    required_hillslope_consumer_state_symbols, validate_hillslope_consumer_boundary,
};

const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

const INVALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 0 0 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

fn valid_topology_report() -> openwepp_topology::TopologyValidationReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    validate_pre_execution_topology(&graph).expect("topology report should build")
}

fn state_update_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
    let target = BoundarySymbol::from(symbol);
    fields.iter().find_map(|field| {
        if field.symbol == target {
            Some(field.value.as_f64())
        } else {
            None
        }
    })
}

fn flux_update_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
    let target = BoundarySymbol::from(symbol);
    fields.iter().find_map(|field| {
        if field.symbol == target {
            Some(field.value.as_f64())
        } else {
            None
        }
    })
}

fn hphys0246_wb18_aggregate_state_surface() -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.343),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_field_capacity"),
        BoundaryValue::scalar(0.40),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(0.50),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.10),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(0.15),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(0.40),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(1.0e-6),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.05),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.30));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.20),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(0.25),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0002"),
        BoundaryValue::scalar(0.50),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0002"),
        BoundaryValue::scalar(1.0e-6),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.07),
    );
    state_surface.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.40));
    state_surface
}

#[allow(clippy::too_many_lines)]
fn seeded_growth_runtime_surface_for_day_year(
    imngmt: f64,
    day_of_year: f64,
    runtime_year: f64,
) -> HillslopeWritebackSurface {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_count"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_schedule_rotation_years"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_schedule_rotation_repeats"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("day"),
        BoundaryValue::scalar(day_of_year),
    );
    state_surface.insert(
        BoundarySymbol::from("year"),
        BoundaryValue::scalar(runtime_year),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_rotation_index"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_ofe_index"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_year_in_rotation"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_slots"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_0001_imngmt"),
        BoundaryValue::scalar(imngmt),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_order_decomp_before_soil"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_order_growth_after_decomp"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_order_watbal_after_growth"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_imngmt"),
        BoundaryValue::scalar(imngmt),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
        BoundaryValue::scalar(240.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
        BoundaryValue::scalar(120.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_rw"),
        BoundaryValue::scalar(1.3),
    );
    for (root, value) in [
        ("btemp", 10.0),
        ("otemp", 25.0),
        ("gddmax", 1700.0),
        ("dlai", 0.85),
        ("dropfc", 0.98),
        ("decfct", 0.65),
        ("spriod", 30.0),
        ("bb", 3.6),
        ("beinp", 35.00196),
        ("extnct", 0.65),
        ("hi", 0.5),
        ("xmxlai", 3.5),
        ("rsr", 0.25),
        ("rtmmax", 3.0),
        ("rdmax", 1.51995),
    ] {
        state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_slot_0001_crop_0001_{root}")),
            BoundaryValue::scalar(value),
        );
    }
    state_surface.insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(25.0));
    state_surface.insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(13.0));
    state_surface.insert(BoundarySymbol::from("rad"), BoundaryValue::scalar(210.0));
    state_surface.insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.003));
    state_surface.insert(BoundarySymbol::from("Ws"), BoundaryValue::scalar(0.8));
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.15));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.35));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(0.2));
    state_surface.insert(BoundarySymbol::from("sumgdd"), BoundaryValue::scalar(640.0));
    state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(2.4));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.65));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(2.1));
    state_surface.insert(BoundarySymbol::from("rtmass"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.35));
    state_surface.insert(BoundarySymbol::from("hia"), BoundaryValue::scalar(0.45));
    state_surface.insert(
        BoundarySymbol::from("iresd_seed"),
        BoundaryValue::scalar(3.0),
    );
    state_surface.insert(
        BoundarySymbol::from("sumrtm_seed"),
        BoundaryValue::scalar(2.5),
    );
    state_surface.insert(
        BoundarySymbol::from("sumsrm_seed"),
        BoundaryValue::scalar(1.5),
    );
    for (root, value) in [("oratea", 0.0065), ("orater", 0.0065)] {
        state_surface.insert(
            BoundarySymbol::from(format!("pl_decomp_slot_0001_crop_0001_{root}")),
            BoundaryValue::scalar(value),
        );
        state_surface.insert(BoundarySymbol::from(root), BoundaryValue::scalar(value));
    }

    if (imngmt - 2.0).abs() < f64::EPSILON {
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_mgtopt"),
            BoundaryValue::scalar(2.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncut"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncycle"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_gday_0001"),
            BoundaryValue::scalar(150.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_gend_0001"),
            BoundaryValue::scalar(250.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_animal_0001"),
            BoundaryValue::scalar(20.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_bodywt_0001"),
            BoundaryValue::scalar(450.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_area_0001"),
            BoundaryValue::scalar(1200.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_digest_0001"),
            BoundaryValue::scalar(0.62),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdstop"),
            BoundaryValue::scalar(310.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_mgtopt"),
            BoundaryValue::scalar(2.0),
        );
    } else {
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_resmgt"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdherb"),
            BoundaryValue::scalar(200.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdburn"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdslge"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdcut"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdmove"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_fbrnag"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_fbrnog"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_frcut"),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_frmove"),
            BoundaryValue::scalar(0.0),
        );
    }

    HillslopeWritebackSurface {
        state_surface,
        flux_surface: BTreeMap::new(),
    }
}

fn seeded_growth_runtime_surface(imngmt: f64) -> HillslopeWritebackSurface {
    seeded_growth_runtime_surface_for_day_year(imngmt, 200.0, 1.0)
}

#[test]
fn hphys0246_wb18_percolation_preserves_residual_storage_in_aggregate_soil_water() {
    let state_surface = hphys0246_wb18_aggregate_state_surface();
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-OK-001");
    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB18 should publish wb11_soil_water");
    let theta_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
            .expect("WB18 should publish layer 1 theta")
            + state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0002")
                .expect("WB18 should publish layer 2 theta");
    let expected_soilw = theta_after + (0.05 * 0.30) + (0.07 * 0.40);

    assert!(
        (soil_water_after - expected_soilw).abs() < 1.0e-12,
        "WB18 aggregate soil water must follow baseline soilw=sum(st+thetdr*dg), observed {soil_water_after} expected {expected_soilw}"
    );
    assert!(
        (soil_water_after - theta_after).abs() > 1.0e-6,
        "test vector must detect the old sigma-theta-only writeback"
    );
}

#[test]
fn hphys0246_wb18_percolation_requires_residual_storage_symbols_for_aggregate_writeback() {
    let mut state_surface = hphys0246_wb18_aggregate_state_surface();
    state_surface.remove(&BoundarySymbol::from("thetdr_0002"));
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(
        response.status.message_id(),
        "HKERNEL-WB11-PERC-E-001",
        "WB18 must fail closed instead of silently defaulting missing residual storage"
    );
    assert!(
        response.writeback.state_updates.is_empty(),
        "failed WB18 guard must not publish partial state updates"
    );
}

#[test]
fn hphys0264_pmet_evapotranspiration_consumes_evappm_components_without_pt_repartition() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.222),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.004),
    );
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(1.2));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.72));
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.000_2),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_evappm"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.es_m"),
        BoundaryValue::scalar(0.001_1),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.ep_m"),
        BoundaryValue::scalar(0.003_4),
    );
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.050),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.100),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.05));
    state_surface.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.20));
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.04),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.05),
    );
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "evapotranspiration",
        HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
        HillslopeConsumerAdapter::Watbal,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");
    let etp = flux_update_scalar(&response.writeback.flux_updates, "Etp")
        .expect("PMET seam must publish Etp");
    let es = flux_update_scalar(&response.writeback.flux_updates, "Es")
        .expect("PMET seam must publish Es");
    let er = flux_update_scalar(&response.writeback.flux_updates, "Er")
        .expect("PMET seam must publish Er");

    assert!(
        (etp - 0.003_4).abs() < 1.0e-12,
        "PMET mode must pass pmet.ep_m to SWU as Etp, observed {etp}"
    );
    assert!(
        (es + er - 0.001_1).abs() < 1.0e-12,
        "PMET mode must derive Es+Er from pmet.es_m, observed Es={es} Er={er}"
    );
    assert!(
        (etp - (1.2 * 0.004 / 3.0)).abs() > 1.0e-6,
        "test vector must detect the old Priestley-Taylor LAI repartition"
    );
}

#[test]
fn hphys0264_pmet_evapotranspiration_rejects_material_negative_soil_evaporation() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.010),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(2.4));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.10));
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_evappm"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.es_m"),
        BoundaryValue::scalar(-0.000_575_419_020_248_203_2),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.ep_m"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.010),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.20));
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "evapotranspiration",
        HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
        HillslopeConsumerAdapter::Watbal,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(
        response.status.message_id(),
        "HKERNEL-WB11-ET-E-003",
        "material negative PMET Es must fail closed instead of publishing signed Es"
    );
    assert!(response.writeback.flux_updates.is_empty());
    assert!(response.writeback.state_updates.is_empty());
}

#[test]
fn hphys0264_pmet_evapotranspiration_snaps_roundoff_negative_soil_evaporation() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.010),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(2.4));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.10));
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_evappm"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.es_m"),
        BoundaryValue::scalar(-1.0e-13),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.ep_m"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.010),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.20));
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "evapotranspiration",
        HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
        HillslopeConsumerAdapter::Watbal,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");
    let es = flux_update_scalar(&response.writeback.flux_updates, "Es")
        .expect("PMET seam must publish Es");
    assert!(
        es.abs() < f64::EPSILON,
        "near-zero negative PMET Es roundoff must canonicalize to zero"
    );
}

#[test]
fn hphys0281_pmet_evapotranspiration_applies_condensation_storage_return() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.162),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(2.4));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.10));
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.000_2),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_evappm"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.es_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.es_storage_return_m"),
        BoundaryValue::scalar(0.000_3),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.ep_m"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.050),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.100),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.05));
    state_surface.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.20));
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.04),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.05),
    );
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "evapotranspiration",
        HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
        HillslopeConsumerAdapter::Watbal,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");
    let theta_0001 = state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
        .expect("PMET condensation return must update top-layer storage");
    let es = flux_update_scalar(&response.writeback.flux_updates, "Es")
        .expect("PMET seam must publish Es");
    let er = flux_update_scalar(&response.writeback.flux_updates, "Er")
        .expect("PMET seam must publish Er");

    assert!(
        (theta_0001 - 0.050_5).abs() < 1.0e-12,
        "top-layer storage must include explicit condensation return plus residue return"
    );
    assert!(
        es.abs() < f64::EPSILON,
        "zero PMET Es must not trigger soil extraction"
    );
    assert!(
        er.abs() < f64::EPSILON,
        "zero PMET Es under residue interception must return residue to storage instead of evaporating it"
    );
}

#[test]
fn hphys0250_wb11_growth_transition_publishes_state_after_for_ep_lineage() {
    let state_surface = BTreeMap::new();
    let flux_surface = BTreeMap::new();
    let state_after = HillslopeGrowthStateSurface {
        sumgdd: 42.0,
        vdmt: 1.25,
        cancov: 0.45,
        lai: 1.8,
        rtmass: 0.75,
        rtd: 0.62,
        hia: 0.2,
    };
    let context =
        HillslopeGrowthKernelContext::new(HillslopeGrowthManagementClass::Perennial, 1.0, 1.0)
            .with_transition_payload(HillslopeGrowthTransitionPayload {
                active_slot_index: 1,
                active_crop_slot_index: 1,
                runtime_day_of_year: 150,
                state_before: HillslopeGrowthStateSurface {
                    sumgdd: 40.0,
                    vdmt: 1.0,
                    cancov: 0.3,
                    lai: 1.0,
                    rtmass: 0.5,
                    rtd: 0.25,
                    hia: 0.1,
                },
                state_after,
                control: HillslopeGrowthTransitionControl::Perennial(
                    HillslopePerennialGrowthControl {
                        jdharv: 0,
                        jdplt: 0,
                        jdstop: 0,
                        mgtopt: 3,
                        rw: 1.0,
                        active_action: HillslopePerennialGrowthAction::None,
                    },
                ),
            });
    let request = HillslopeKernelRequest::with_phase_context(
        "perennial_growth_transition",
        HillslopeKernelPhaseClass::GrowthPerennialTransition,
        HillslopeConsumerAdapter::Growth,
        Some(context),
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-GROWTH-OK-001");
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "sumgdd"),
        Some(state_after.sumgdd)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "vdmt"),
        Some(state_after.vdmt)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "cancov"),
        Some(state_after.cancov)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "lai"),
        Some(state_after.lai)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "rtmass"),
        Some(state_after.rtmass)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "rtd"),
        Some(state_after.rtd)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "hia"),
        Some(state_after.hia)
    );
    assert!(
        !response.writeback.state_updates.is_empty(),
        "growth transition must not NOP after scheduler computes state_after"
    );
}

#[test]
fn hphys0250_wb11_decomposition_transition_publishes_seed_surface() {
    let state_surface = BTreeMap::new();
    let flux_surface = BTreeMap::new();
    let context = HillslopeDecompositionKernelContext::new(
        HillslopeDecompositionManagementClass::Perennial,
        1.0,
        1.0,
    )
    .with_transition_payload(HillslopeDecompositionTransitionPayload {
        active_slot_index: 1,
        active_crop_slot_index: 1,
        runtime_day_of_year: 150,
        iresd_seed: 3.0,
        sumrtm_seed: 2.25,
        sumsrm_seed: 1.75,
        control: HillslopeDecompositionTransitionControl::Perennial(
            HillslopePerennialDecompositionControl {
                mgtopt: 3,
                ncut: 0,
                ncycle: 0,
                active_action: HillslopePerennialDecompositionAction::None,
                active_grazing_cycle: None,
            },
        ),
    });
    let request = HillslopeKernelRequest::with_transition_context(
        "decomposition_transition",
        HillslopeKernelPhaseClass::DecompositionTransition,
        HillslopeConsumerAdapter::Decomposition,
        Some(context),
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-DECOMP-OK-001");
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "iresd_seed"),
        Some(3.0)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "sumrtm_seed"),
        Some(2.25)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "sumsrm_seed"),
        Some(1.75)
    );
    assert!(
        !response.writeback.state_updates.is_empty(),
        "decomposition transition must not NOP after scheduler computes seed surface"
    );
}

fn seed_legacy_monthly_temperature_vectors(surface: &mut HillslopeWritebackSurface) {
    const OBMAX: [f64; 12] = [
        5.0, 7.0, 11.0, 16.0, 21.0, 25.0, 27.0, 26.0, 22.0, 16.0, 10.0, 6.0,
    ];
    const OBMIN: [f64; 12] = [
        -4.0, -2.0, 1.0, 5.0, 9.0, 13.0, 15.0, 14.0, 10.0, 5.0, 1.0, -3.0,
    ];

    for (month_index, (obmaxt, obmint)) in OBMAX.iter().zip(OBMIN.iter()).enumerate() {
        let month = month_index + 1;
        surface.state_surface.insert(
            BoundarySymbol::from(format!("obmaxt_{month:04}")),
            BoundaryValue::scalar(*obmaxt),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("obmint_{month:04}")),
            BoundaryValue::scalar(*obmint),
        );
    }
}

#[allow(clippy::too_many_lines)]
fn seeded_multislot_rotation_surface(
    runtime_year: f64,
    day_of_year: f64,
) -> HillslopeWritebackSurface {
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, day_of_year, runtime_year);
    let state = &mut surface.state_surface;

    state.insert(
        BoundarySymbol::from("pl_schedule_slot_count"),
        BoundaryValue::scalar(6.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_rotation_years"),
        BoundaryValue::scalar(3.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_rotation_repeats"),
        BoundaryValue::scalar(2.0),
    );
    for slot_index in 1..=6 {
        for (root, value) in [
            ("btemp", 10.0),
            ("otemp", 25.0),
            ("gddmax", 1700.0),
            ("dlai", 0.85),
            ("dropfc", 0.98),
            ("decfct", 0.65),
            ("spriod", 30.0),
            ("bb", 3.6),
            ("beinp", 35.00196),
            ("extnct", 0.65),
            ("hi", 0.5),
            ("xmxlai", 3.5),
            ("rsr", 0.25),
            ("rtmmax", 3.0),
            ("rdmax", 1.51995),
        ] {
            state.insert(
                BoundarySymbol::from(format!("pl_growth_slot_{slot_index:04}_crop_0001_{root}")),
                BoundaryValue::scalar(value),
            );
        }
        for (root, value) in [("oratea", 0.0065), ("orater", 0.0065)] {
            state.insert(
                BoundarySymbol::from(format!("pl_decomp_slot_{slot_index:04}_crop_0001_{root}")),
                BoundaryValue::scalar(value),
            );
        }
    }

    // Slot 1 / year 1 / annual.
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_ofe_index"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_rotation_index"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_year_in_rotation"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_slots"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_0001_imngmt"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_imngmt"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
        BoundaryValue::scalar(240.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
        BoundaryValue::scalar(120.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_rw"),
        BoundaryValue::scalar(1.1),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_resmgt"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdherb"),
        BoundaryValue::scalar(200.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdburn"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdslge"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdcut"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdmove"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_fbrnag"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_fbrnog"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_frcut"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_frmove"),
        BoundaryValue::scalar(0.0),
    );

    // Slot 2 / year 2 / annual-fallow.
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0002_ofe_index"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0002_rotation_index"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0002_year_in_rotation"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0002_crop_slots"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0002_crop_0001_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0002_crop_0001_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0002_crop_0001_jdharv"),
        BoundaryValue::scalar(365.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0002_crop_0001_jdplt"),
        BoundaryValue::scalar(228.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0002_crop_0001_rw"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_resmgt"),
        BoundaryValue::scalar(6.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_jdherb"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_jdburn"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_jdslge"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_jdcut"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_jdmove"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_fbrnag"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_fbrnog"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_frcut"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_frmove"),
        BoundaryValue::scalar(0.0),
    );

    // Slot 3 / year 3 / perennial.
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0003_ofe_index"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0003_rotation_index"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0003_year_in_rotation"),
        BoundaryValue::scalar(3.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0003_crop_slots"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0003_crop_0001_imngmt"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0003_crop_0001_imngmt"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0003_crop_0001_jdharv"),
        BoundaryValue::scalar(288.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0003_crop_0001_jdplt"),
        BoundaryValue::scalar(130.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0003_crop_0001_jdstop"),
        BoundaryValue::scalar(310.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0003_crop_0001_rw"),
        BoundaryValue::scalar(0.762),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0003_crop_0001_mgtopt"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_mgtopt"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_ncut"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_ncycle"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_gday_0001"),
        BoundaryValue::scalar(150.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_gend_0001"),
        BoundaryValue::scalar(220.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_animal_0001"),
        BoundaryValue::scalar(20.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_bodywt_0001"),
        BoundaryValue::scalar(450.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_area_0001"),
        BoundaryValue::scalar(1200.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_digest_0001"),
        BoundaryValue::scalar(0.62),
    );

    // Slot 4 / year 1 / annual (rotation repeat 2).
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0004_ofe_index"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0004_rotation_index"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0004_year_in_rotation"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0004_crop_slots"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0004_crop_0001_imngmt"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0004_crop_0001_imngmt"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0004_crop_0001_jdharv"),
        BoundaryValue::scalar(240.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0004_crop_0001_jdplt"),
        BoundaryValue::scalar(120.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0004_crop_0001_rw"),
        BoundaryValue::scalar(1.1),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_resmgt"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_jdherb"),
        BoundaryValue::scalar(200.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_jdburn"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_jdslge"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_jdcut"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_jdmove"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_fbrnag"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_fbrnog"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_frcut"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_frmove"),
        BoundaryValue::scalar(0.0),
    );

    // Slot 5 / year 2 / annual-fallow (rotation repeat 2).
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0005_ofe_index"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0005_rotation_index"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0005_year_in_rotation"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0005_crop_slots"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0005_crop_0001_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0005_crop_0001_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0005_crop_0001_jdharv"),
        BoundaryValue::scalar(365.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0005_crop_0001_jdplt"),
        BoundaryValue::scalar(228.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0005_crop_0001_rw"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_resmgt"),
        BoundaryValue::scalar(6.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_jdherb"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_jdburn"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_jdslge"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_jdcut"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_jdmove"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_fbrnag"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_fbrnog"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_frcut"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_frmove"),
        BoundaryValue::scalar(0.0),
    );

    // Slot 6 / year 3 / perennial (rotation repeat 2).
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0006_ofe_index"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0006_rotation_index"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0006_year_in_rotation"),
        BoundaryValue::scalar(3.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0006_crop_slots"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_schedule_slot_0006_crop_0001_imngmt"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0006_crop_0001_imngmt"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0006_crop_0001_jdharv"),
        BoundaryValue::scalar(288.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0006_crop_0001_jdplt"),
        BoundaryValue::scalar(130.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0006_crop_0001_jdstop"),
        BoundaryValue::scalar(310.0),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0006_crop_0001_rw"),
        BoundaryValue::scalar(0.762),
    );
    state.insert(
        BoundarySymbol::from("pl_growth_slot_0006_crop_0001_mgtopt"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_mgtopt"),
        BoundaryValue::scalar(2.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_ncut"),
        BoundaryValue::scalar(0.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_ncycle"),
        BoundaryValue::scalar(1.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_gday_0001"),
        BoundaryValue::scalar(150.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_gend_0001"),
        BoundaryValue::scalar(220.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_animal_0001"),
        BoundaryValue::scalar(20.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_bodywt_0001"),
        BoundaryValue::scalar(450.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_area_0001"),
        BoundaryValue::scalar(1200.0),
    );
    state.insert(
        BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_digest_0001"),
        BoundaryValue::scalar(0.62),
    );

    surface
}

#[test]
fn canonical_graph_order_is_deterministic() {
    let graph = HillslopePhaseGraph::canonical();
    let order = graph
        .topological_order()
        .expect("canonical graph should always topologically sort");

    assert_eq!(
        order,
        Vec::from(HillslopePhaseGraph::canonical_order()),
        "ARCH05 requires explicit deterministic scheduler order"
    );
    assert_eq!(graph.dependency_edges().len(), 13);
}

#[test]
fn topology_precondition_failure_blocks_phase_execution() {
    let graph = parse_topology_fixture_str(INVALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    assert_eq!(
        topology_report.status.classification(),
        StatusClassification::Failure
    );

    let scheduler = HillslopePhaseScheduler::canonical();
    let call_count = Cell::new(0_usize);

    let report = scheduler
        .execute_with(&topology_report, |_| {
            call_count.set(call_count.get() + 1);
            HillslopePhaseScheduler::nominal_phase_status(HillslopePhase::Normalization)
                .expect("nominal status should build")
        })
        .expect("scheduler should not error");

    assert_eq!(call_count.get(), 0);
    assert_eq!(
        report.outcome_class,
        SchedulerOutcomeClass::TopologyPreconditionFailed
    );
    assert_eq!(
        report.scheduler_status.classification(),
        StatusClassification::Failure
    );
    assert_eq!(
        report.scheduler_status.boundary_class(),
        BoundaryClass::TopologyInvalid
    );
}

#[test]
fn phase_failure_is_typed_and_fail_fast() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();

    let report = scheduler
        .execute_with(&topology_report, |phase| {
            if phase == HillslopePhase::PercolationDeepSeepage {
                return openwepp_sim_contract::status::SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    BoundaryClass::DomainViolation,
                    "HSCHED-PHASE-E-004",
                )
                .expect("failure status should build");
            }

            HillslopePhaseScheduler::nominal_phase_status(phase)
                .expect("nominal status should build")
        })
        .expect("scheduler should not error");

    assert_eq!(report.outcome_class, SchedulerOutcomeClass::PhaseFailure);
    assert_eq!(
        report.scheduler_status.classification(),
        StatusClassification::Failure
    );
    assert_eq!(
        report.scheduler_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
    assert_eq!(
        report.executed_phases(),
        vec![
            HillslopePhase::Normalization,
            HillslopePhase::StorageBounds,
            HillslopePhase::DecompositionTransition,
            HillslopePhase::ResiduePartitionTransition,
            HillslopePhase::AnnualGrowthTransition,
            HillslopePhase::PerennialGrowthTransition,
            HillslopePhase::PercolationDeepSeepage,
        ]
    );
    assert_eq!(
        report.halted_phase,
        Some(HillslopePhase::PercolationDeepSeepage)
    );
}

#[test]
fn phase_status_phase_mismatch_returns_mode_mismatch_failure() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();

    let report = scheduler
        .execute_with(&topology_report, |_| {
            openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::PreExecutionValidation,
                "HSCHED-PHASE-INVALID-STATUS",
            )
            .expect("status should build")
        })
        .expect("scheduler should not error");

    assert_eq!(
        report.outcome_class,
        SchedulerOutcomeClass::SchedulerInvariantFailure
    );
    assert_eq!(
        report.scheduler_status.classification(),
        StatusClassification::Failure
    );
    assert_eq!(
        report.scheduler_status.boundary_class(),
        BoundaryClass::ModeMismatch
    );
    assert_eq!(report.halted_phase, Some(HillslopePhase::Normalization));
}

#[test]
fn nominal_execution_completes_in_canonical_order() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();

    let report = scheduler
        .execute_with(&topology_report, |phase| {
            HillslopePhaseScheduler::nominal_phase_status(phase)
                .expect("nominal status should build")
        })
        .expect("scheduler should not error");

    assert!(report.is_success());
    assert_eq!(report.outcome_class, SchedulerOutcomeClass::Completed);
    assert_eq!(report.halted_phase, None);
    assert_eq!(
        report.executed_phases(),
        Vec::from(HillslopePhaseGraph::canonical_order())
    );
    assert_eq!(
        report.scheduler_status.phase(),
        SimulationPhase::HillslopeKernel
    );
    assert_eq!(
        report.scheduler_status.classification(),
        StatusClassification::Nominal
    );
}

#[test]
fn consumer_adapter_mapping_matches_phase_contract() {
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::Normalization),
        HillslopeConsumerAdapter::Soil
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::StorageBounds),
        HillslopeConsumerAdapter::Soil
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::DecompositionTransition),
        HillslopeConsumerAdapter::Decomposition
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::ResiduePartitionTransition),
        HillslopeConsumerAdapter::Decomposition
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::AnnualGrowthTransition),
        HillslopeConsumerAdapter::Growth
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::PerennialGrowthTransition),
        HillslopeConsumerAdapter::Growth
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::Evapotranspiration),
        HillslopeConsumerAdapter::Watbal
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::PercolationDeepSeepage),
        HillslopeConsumerAdapter::Perc
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::LateralTransfer),
        HillslopeConsumerAdapter::Watbal
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::PlantRootUptake),
        HillslopeConsumerAdapter::Watbal
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::Drainage),
        HillslopeConsumerAdapter::Perc
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::RunoffReconciliation),
        HillslopeConsumerAdapter::Runoff
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::StorageReconciliation),
        HillslopeConsumerAdapter::Watbal
    );
    assert_eq!(
        hillslope_consumer_adapter_for_phase(HillslopePhase::ClosureDiagnostics),
        HillslopeConsumerAdapter::Watbal
    );
}

#[test]
fn wb10_contract_conformance_hydrology_phase_classes_are_not_generic() {
    #[derive(Default)]
    struct ProbeKernel {
        observed_phase_classes: BTreeMap<String, String>,
    }

    impl HillslopeKernel for ProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if matches!(
                request.phase_name,
                "evapotranspiration"
                    | "percolation_deep_seepage"
                    | "lateral_transfer"
                    | "drainage"
                    | "plant_root_uptake"
                    | "runoff_reconciliation"
                    | "storage_reconciliation"
                    | "closure_diagnostics"
            ) {
                self.observed_phase_classes.insert(
                    request.phase_name.to_owned(),
                    request.phase_class.as_str().to_owned(),
                );
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-WB10-PHASE-CLASS",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ProbeKernel::default();
    let surface = seeded_growth_runtime_surface(1.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb10 phase-class conformance probe should execute");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.observed_phase_classes.get("evapotranspiration"),
        Some(&"hydrology_evapotranspiration".to_owned())
    );
    assert_eq!(
        kernel
            .observed_phase_classes
            .get("percolation_deep_seepage"),
        Some(&"hydrology_percolation_deep_seepage".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("lateral_transfer"),
        Some(&"hydrology_lateral_transfer".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("drainage"),
        Some(&"hydrology_drainage".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("plant_root_uptake"),
        Some(&"hydrology_plant_root_uptake".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("runoff_reconciliation"),
        Some(&"hydrology_runoff_reconciliation".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("storage_reconciliation"),
        Some(&"hydrology_storage_reconciliation".to_owned())
    );
    assert_eq!(
        kernel.observed_phase_classes.get("closure_diagnostics"),
        Some(&"hydrology_peak_runoff".to_owned())
    );
}

#[test]
fn wb10_contract_conformance_rejects_unsupported_hydrology_phase_class() {
    let error = super::hydrology_phase_dispatch_for_phase(
        HillslopePhase::Evapotranspiration,
        HillslopeKernelPhaseClass::Hydrology,
    )
    .expect_err("evapotranspiration must not accept generic hydrology class");

    assert_eq!(error.code(), "HS-HYDRO-E-001");
    assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
}

#[test]
fn required_consumer_symbols_are_empty_without_slope_or_soil_families() {
    let empty_surface = BTreeMap::new();

    for phase in HillslopePhaseGraph::canonical_order() {
        let required = required_hillslope_consumer_state_symbols(phase, &empty_surface);
        assert!(
            required.is_empty(),
            "phase {} should not require slope/soil symbols when neither family is seeded",
            phase.as_str()
        );
        validate_hillslope_consumer_boundary(phase, &empty_surface)
            .expect("empty non-slope/non-soil surface should not trigger consumer guard");
    }
}

#[test]
fn consumer_boundary_reports_typed_missing_symbol_for_seeded_family() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.25));
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.31));
    state_surface.insert(
        BoundarySymbol::from("ssc"),
        BoundaryValue::scalar(0.000_004),
    );

    let error = validate_hillslope_consumer_boundary(HillslopePhase::Normalization, &state_surface)
        .expect_err("missing thetdr must fail with typed consumer boundary error");
    assert_eq!(error.code(), "HS-CONSUMER-E-001");
    assert!(matches!(
        error,
        super::HillslopeConsumerBoundaryError::MissingRequiredStateSymbol {
            phase: HillslopePhase::Normalization,
            adapter: HillslopeConsumerAdapter::Soil,
            symbol,
        } if symbol.as_str() == "thetdr"
    ));
}

#[test]
fn annual_growth_phase_emits_typed_growth_context() {
    #[derive(Default)]
    struct ProbeKernel {
        decomp: usize,
        annual: usize,
        perennial: usize,
    }

    impl HillslopeKernel for ProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            match request.phase_class {
                HillslopeKernelPhaseClass::DecompositionTransition
                | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                    let context = request
                        .decomposition_context
                        .expect("decomposition phases should carry decomposition context");
                    assert_eq!(
                        context.management_class,
                        HillslopeDecompositionManagementClass::AnnualOrFallow
                    );
                    let transition_payload = context
                        .transition_payload
                        .expect("decomposition context should carry transition payload");
                    assert!(matches!(
                        transition_payload.control,
                        HillslopeDecompositionTransitionControl::Annual(
                            HillslopeAnnualDecompositionControl {
                                active_action: HillslopeAnnualDecompositionAction::Herbicide,
                                ..
                            }
                        )
                    ));
                    assert!(request.growth_context.is_none());
                    self.decomp += 1;
                }
                HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                    let context = request
                        .growth_context
                        .expect("annual growth phase should carry growth context");
                    assert_eq!(
                        context.management_class,
                        HillslopeGrowthManagementClass::AnnualOrFallow
                    );
                    let transition_payload = context
                        .transition_payload
                        .expect("annual growth context should carry transition payload");
                    assert!(matches!(
                        transition_payload.control,
                        HillslopeGrowthTransitionControl::Annual(HillslopeAnnualGrowthControl {
                            active_action: HillslopeAnnualGrowthAction::None,
                            ..
                        })
                    ));
                    self.annual += 1;
                }
                HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                    assert!(
                        request.growth_context.is_none(),
                        "perennial phase should skip context when annual branch is active"
                    );
                    self.perennial += 1;
                }
                phase_class if phase_class.is_hydrology_phase() => {
                    assert!(request.growth_context.is_none());
                    assert!(request.decomposition_context.is_none());
                }
                _ => unreachable!("unexpected phase class for annual growth test"),
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GROWTH-CONTEXT",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ProbeKernel::default();
    let surface = seeded_growth_runtime_surface(1.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("annual growth context execution should succeed");

    assert!(report.scheduler_report.is_success());
    assert_eq!(kernel.decomp, 2);
    assert_eq!(kernel.annual, 1);
    assert_eq!(kernel.perennial, 1);
}

#[test]
fn perennial_growth_phase_emits_typed_growth_context() {
    #[derive(Default)]
    struct ProbeKernel {
        decomp: usize,
        annual: usize,
        perennial: usize,
    }

    impl HillslopeKernel for ProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            match request.phase_class {
                HillslopeKernelPhaseClass::DecompositionTransition
                | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                    let context = request
                        .decomposition_context
                        .expect("decomposition phases should carry decomposition context");
                    assert_eq!(
                        context.management_class,
                        HillslopeDecompositionManagementClass::Perennial
                    );
                    let transition_payload = context
                        .transition_payload
                        .expect("decomposition context should carry transition payload");
                    assert!(matches!(
                        transition_payload.control,
                        HillslopeDecompositionTransitionControl::Perennial(
                            HillslopePerennialDecompositionControl {
                                active_action: HillslopePerennialDecompositionAction::Grazing {
                                    cycle_index: 1
                                },
                                ..
                            }
                        )
                    ));
                    assert!(request.growth_context.is_none());
                    self.decomp += 1;
                }
                HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                    assert!(
                        request.growth_context.is_none(),
                        "annual phase should skip context when perennial branch is active"
                    );
                    self.annual += 1;
                }
                HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                    let context = request
                        .growth_context
                        .expect("perennial growth phase should carry growth context");
                    assert_eq!(
                        context.management_class,
                        HillslopeGrowthManagementClass::Perennial
                    );
                    let transition_payload = context
                        .transition_payload
                        .expect("perennial growth context should carry transition payload");
                    assert!(matches!(
                        transition_payload.control,
                        HillslopeGrowthTransitionControl::Perennial(
                            HillslopePerennialGrowthControl {
                                active_action: HillslopePerennialGrowthAction::None,
                                ..
                            }
                        )
                    ));
                    self.perennial += 1;
                }
                phase_class if phase_class.is_hydrology_phase() => {
                    assert!(request.growth_context.is_none());
                    assert!(request.decomposition_context.is_none());
                }
                _ => unreachable!("unexpected phase class for perennial growth test"),
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GROWTH-CONTEXT",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ProbeKernel::default();
    let surface = seeded_growth_runtime_surface(2.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("perennial growth context execution should succeed");

    assert!(report.scheduler_report.is_success());
    assert_eq!(kernel.decomp, 2);
    assert_eq!(kernel.annual, 1);
    assert_eq!(kernel.perennial, 1);
}

#[test]
fn pl16_annual_growth_accepts_zero_gddmax_sentinel_for_summer_branch() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GDDMAX-SUMMER",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 200.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_gddmax"),
        BoundaryValue::scalar(0.0),
    );
    seed_legacy_monthly_temperature_vectors(&mut surface);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("annual gddmax sentinel branch should execute");

    assert!(report.scheduler_report.is_success());
}

#[test]
fn pl16_annual_growth_accepts_zero_gddmax_sentinel_for_winter_branch() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GDDMAX-WINTER",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 20.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
        BoundaryValue::scalar(300.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
        BoundaryValue::scalar(100.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_gddmax"),
        BoundaryValue::scalar(0.0),
    );
    seed_legacy_monthly_temperature_vectors(&mut surface);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("winter annual gddmax sentinel branch should execute");

    assert!(report.scheduler_report.is_success());
}

#[test]
fn pl16_perennial_growth_accepts_zero_gddmax_sentinel() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GDDMAX-PERENNIAL",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(2.0, 200.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_gddmax"),
        BoundaryValue::scalar(0.0),
    );
    seed_legacy_monthly_temperature_vectors(&mut surface);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("perennial gddmax sentinel branch should execute");

    assert!(report.scheduler_report.is_success());
}

#[test]
fn pl16_gddmax_sentinel_requires_monthly_temperature_vectors() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-GDDMAX-MISSING-MONTHLY",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 200.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_gddmax"),
        BoundaryValue::scalar(0.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing monthly temperature vectors should return typed failure report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(report.phase_reports.len(), 5);
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-001"
    );
    assert_eq!(
        report.phase_reports[4].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn active_slot_resolution_uses_year_three_perennial_slot() {
    #[derive(Default)]
    struct ProbeKernel {
        saw_decomp_perennial: bool,
        saw_annual_context: bool,
        saw_perennial_context: bool,
    }

    impl HillslopeKernel for ProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            match request.phase_class {
                HillslopeKernelPhaseClass::DecompositionTransition
                | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                    let context = request
                        .decomposition_context
                        .expect("decomposition phases should carry decomposition context");
                    self.saw_decomp_perennial = context.management_class
                        == HillslopeDecompositionManagementClass::Perennial;
                }
                HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                    self.saw_annual_context = request.growth_context.is_some();
                }
                HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                    self.saw_perennial_context = request.growth_context.is_some();
                }
                phase_class if phase_class.is_hydrology_phase() => {}
                _ => unreachable!("unexpected phase class for active-slot perennial test"),
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-ACTIVE-SLOT",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ProbeKernel::default();
    let surface = seeded_multislot_rotation_surface(3.0, 200.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("year-three slot resolution should succeed");

    assert!(report.scheduler_report.is_success());
    assert!(kernel.saw_decomp_perennial);
    assert!(!kernel.saw_annual_context);
    assert!(kernel.saw_perennial_context);
}

#[test]
fn active_slot_resolution_wraps_rotation_boundary_to_year_one() {
    #[derive(Default)]
    struct ProbeKernel {
        saw_decomp_annual: bool,
        saw_annual_context: bool,
        saw_perennial_context: bool,
    }

    impl HillslopeKernel for ProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            match request.phase_class {
                HillslopeKernelPhaseClass::DecompositionTransition
                | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                    let context = request
                        .decomposition_context
                        .expect("decomposition phases should carry decomposition context");
                    self.saw_decomp_annual = context.management_class
                        == HillslopeDecompositionManagementClass::AnnualOrFallow;
                }
                HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                    self.saw_annual_context = request.growth_context.is_some();
                }
                HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                    self.saw_perennial_context = request.growth_context.is_some();
                }
                phase_class if phase_class.is_hydrology_phase() => {}
                _ => unreachable!("unexpected phase class for active-slot annual test"),
            }

            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-ACTIVE-SLOT",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = ProbeKernel::default();
    let surface = seeded_multislot_rotation_surface(4.0, 200.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("rotation-boundary slot resolution should succeed");

    assert!(report.scheduler_report.is_success());
    assert!(kernel.saw_decomp_annual);
    assert!(kernel.saw_annual_context);
    assert!(!kernel.saw_perennial_context);
}

#[test]
fn active_slot_resolution_rejects_ambiguous_slot_candidates() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_multislot_rotation_surface(1.0, 200.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0002_year_in_rotation"),
        BoundaryValue::scalar(1.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("ambiguous slot candidate must return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-PLDISP-E-006"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn active_slot_resolution_rejects_missing_active_crop_for_day() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 30.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_slots"),
        BoundaryValue::scalar(2.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_0002_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
        BoundaryValue::scalar(120.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
        BoundaryValue::scalar(150.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdplt"),
        BoundaryValue::scalar(200.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdharv"),
        BoundaryValue::scalar(240.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_rw"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0002_resmgt"),
        BoundaryValue::scalar(6.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing active crop must return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-PLDISP-E-008"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn hphys0250_zero_date_perennial_slot_remains_active_for_growth_dispatch() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-ZERO-DATE-PERENNIAL",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(2.0, 1.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdstop"),
        BoundaryValue::scalar(0.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("zero-date perennial slot should dispatch under baseline ptgrp semantics");

    assert!(
        report.scheduler_report.is_success(),
        "zero-date perennial dispatch should not fail active crop resolution: {:?}",
        report.scheduler_report.scheduler_status
    );
}

#[test]
fn active_slot_resolution_rejects_ambiguous_active_crops_for_day() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 210.0, 1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_slots"),
        BoundaryValue::scalar(2.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_0001_crop_0002_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_imngmt"),
        BoundaryValue::scalar(3.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
        BoundaryValue::scalar(180.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
        BoundaryValue::scalar(300.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdplt"),
        BoundaryValue::scalar(200.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdharv"),
        BoundaryValue::scalar(240.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_growth_slot_0001_crop_0002_rw"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0002_resmgt"),
        BoundaryValue::scalar(6.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("ambiguous active crop must return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-PLDISP-E-009"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn decomposition_boundary_missing_required_symbol_returns_typed_failure() {
    #[derive(Default)]
    struct NoopKernel {
        invocation_count: usize,
    }

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.invocation_count += 1;
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface.state_surface.remove(&BoundarySymbol::from(
        "pl_decomp_slot_0001_crop_0001_resmgt",
    ));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed decomposition guard failure should produce report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(kernel.invocation_count, 2);
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-001"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn decomposition_boundary_invalid_ordering_flag_returns_typed_failure() {
    #[derive(Default)]
    struct NoopKernel {
        invocation_count: usize,
    }

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.invocation_count += 1;
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_order_decomp_before_soil"),
        BoundaryValue::scalar(0.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed decomposition guard failure should produce report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(kernel.invocation_count, 2);
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-003"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn decomposition_boundary_rejects_negative_oratea_with_typed_failure() {
    #[derive(Default)]
    struct NoopKernel {
        invocation_count: usize,
    }

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.invocation_count += 1;
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_oratea"),
        BoundaryValue::scalar(-0.1),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed decomposition guard failure should produce report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(kernel.invocation_count, 2);
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-010"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn pl12_contract_conformance_rejects_missing_perennial_cutday_payload() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface(2.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_mgtopt"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncut"),
        BoundaryValue::scalar(2.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncycle"),
        BoundaryValue::scalar(0.0),
    );
    for symbol in [
        "pl_decomp_slot_0001_crop_0001_gday_0001",
        "pl_decomp_slot_0001_crop_0001_gend_0001",
        "pl_decomp_slot_0001_crop_0001_animal_0001",
        "pl_decomp_slot_0001_crop_0001_bodywt_0001",
        "pl_decomp_slot_0001_crop_0001_area_0001",
        "pl_decomp_slot_0001_crop_0001_digest_0001",
    ] {
        surface.state_surface.remove(&BoundarySymbol::from(symbol));
    }

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing perennial cutday payload should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-007"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn pl12_contract_conformance_rejects_invalid_perennial_grazing_window() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface(2.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_mgtopt"),
        BoundaryValue::scalar(2.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncut"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncycle"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_gday_0001"),
        BoundaryValue::scalar(220.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_gend_0001"),
        BoundaryValue::scalar(200.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_animal_0001"),
        BoundaryValue::scalar(20.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_bodywt_0001"),
        BoundaryValue::scalar(450.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_area_0001"),
        BoundaryValue::scalar(1200.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_digest_0001"),
        BoundaryValue::scalar(0.62),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("invalid perennial grazing window should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::DecompositionTransition)
    );
    assert_eq!(report.phase_reports.len(), 3);
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-009"
    );
    assert_eq!(
        report.phase_reports[2].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn pl13_contract_conformance_rejects_missing_growth_state_surface() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface
        .state_surface
        .remove(&BoundarySymbol::from("sumgdd"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing growth transition state should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(report.phase_reports.len(), 5);
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-001"
    );
    assert_eq!(
        report.phase_reports[4].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn pl13_contract_conformance_rejects_growth_state_domain_violation() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface
        .state_surface
        .insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(1.1));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("invalid growth transition state should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(report.phase_reports.len(), 5);
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-007"
    );
    assert_eq!(
        report.phase_reports[4].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn growth_boundary_missing_required_symbol_returns_typed_failure() {
    #[derive(Default)]
    struct NoopKernel {
        invocation_count: usize,
    }

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.invocation_count += 1;
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface
        .state_surface
        .remove(&BoundarySymbol::from("pl_growth_slot_0001_crop_0001_rw"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed growth guard failure should produce report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(kernel.invocation_count, 4);
    assert_eq!(report.phase_reports.len(), 5);
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-001"
    );
    assert_eq!(
        report.phase_reports[4].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn growth_boundary_non_finite_ordering_flag_returns_typed_failure() {
    #[derive(Default)]
    struct NoopKernel {
        invocation_count: usize,
    }

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.invocation_count += 1;
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HSCHED-TEST-NOOP",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let topology_report = valid_topology_report();
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel::default();
    let mut surface = seeded_growth_runtime_surface(1.0);
    surface.state_surface.insert(
        BoundarySymbol::from("pl_order_watbal_after_growth"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed growth guard failure should produce report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(kernel.invocation_count, 4);
    assert_eq!(report.phase_reports.len(), 5);
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-002"
    );
    assert_eq!(
        report.phase_reports[4].decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn execute_with_kernel_applies_writeback_updates() {
    #[derive(Default)]
    struct NominalKernel {
        call_index: u32,
    }

    impl HillslopeKernel for NominalKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.call_index += 1;
            let call_value = f64::from(self.call_index);
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                format!("HKERNEL-PHASE-OK-{}", self.call_index),
            )
            .expect("status should construct");
            let writeback = KernelWritebackPayload::with_updates(
                vec![WritebackField::bounded(
                    "soil_storage",
                    call_value,
                    Some(0.0),
                    Some(1000.0),
                )],
                vec![WritebackField::bounded(
                    "runoff_total",
                    call_value * 0.25,
                    Some(0.0),
                    None,
                )],
            );

            KernelRunResponse::new(status, writeback)
        }
    }

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NominalKernel::default();

    let report = scheduler
        .execute_with_kernel(
            &topology_report,
            &mut kernel,
            HillslopeWritebackSurface::default(),
        )
        .expect("kernel execution should succeed");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        report.scheduler_report.executed_phases(),
        Vec::from(HillslopePhaseGraph::canonical_order())
    );
    assert_eq!(
        report.phase_reports.len(),
        HillslopePhaseGraph::canonical_order().len()
    );
    assert!(report.phase_reports.iter().all(|phase| {
        phase.decision_outcome == WritebackDecisionOutcome::Apply && phase.apply_result.is_some()
    }));
    let phase_count =
        u32::try_from(HillslopePhaseGraph::canonical_order().len()).expect("phase count fits u32");
    let final_call_value = f64::from(phase_count);
    assert_eq!(
        report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from("soil_storage"))
            .copied(),
        Some(BoundaryValue::from(final_call_value))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("runoff_total"))
            .copied(),
        Some(BoundaryValue::from(final_call_value * 0.25))
    );
}

#[test]
fn execute_with_kernel_lends_stable_surface_references() {
    #[derive(Default)]
    struct PointerProbeKernel {
        call_index: u32,
        state_surface_ptrs: Vec<usize>,
        flux_surface_ptrs: Vec<usize>,
    }

    impl HillslopeKernel for PointerProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            self.call_index += 1;
            self.state_surface_ptrs
                .push(std::ptr::from_ref(request.state_surface) as usize);
            self.flux_surface_ptrs
                .push(std::ptr::from_ref(request.flux_surface) as usize);
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                format!("HKERNEL-PHASE-POINTER-{}", self.call_index),
            )
            .expect("status should construct");

            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = PointerProbeKernel::default();

    let report = scheduler
        .execute_with_kernel(
            &topology_report,
            &mut kernel,
            HillslopeWritebackSurface::default(),
        )
        .expect("kernel execution should succeed");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.state_surface_ptrs.len(),
        HillslopePhaseGraph::canonical_order().len()
    );
    assert_eq!(
        kernel.flux_surface_ptrs.len(),
        HillslopePhaseGraph::canonical_order().len()
    );
    assert!(
        kernel
            .state_surface_ptrs
            .windows(2)
            .all(|pair| pair[0] == pair[1]),
        "state surface reference should remain stable across phase calls"
    );
    assert!(
        kernel
            .flux_surface_ptrs
            .windows(2)
            .all(|pair| pair[0] == pair[1]),
        "flux surface reference should remain stable across phase calls"
    );
}

#[test]
fn execute_with_kernel_rejects_non_finite_writeback() {
    struct RejectKernel;

    impl HillslopeKernel for RejectKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-PHASE-OK-REJECT",
            )
            .expect("status should construct");
            let writeback = KernelWritebackPayload::with_updates(
                vec![WritebackField::unbounded("soil_storage", f64::NAN)],
                Vec::new(),
            );
            KernelRunResponse::new(status, writeback)
        }
    }

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = RejectKernel;

    let report = scheduler
        .execute_with_kernel(
            &topology_report,
            &mut kernel,
            HillslopeWritebackSurface::default(),
        )
        .expect("execution should return typed report");

    assert_eq!(
        report.scheduler_report.outcome_class,
        SchedulerOutcomeClass::PhaseFailure
    );
    assert_eq!(report.phase_reports.len(), 1);
    assert_eq!(
        report.phase_reports[0].decision_outcome,
        WritebackDecisionOutcome::Reject
    );
    assert_eq!(
        report.phase_reports[0].decision_status.message_id(),
        WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
    );
    assert!(
        !report
            .writeback_surface
            .state_surface
            .contains_key(&BoundarySymbol::from("soil_storage")),
        "rejected payload must not mutate orchestrator writeback state"
    );
}

#[test]
fn execute_with_kernel_rejects_kernel_phase_mismatch() {
    struct PhaseMismatchKernel;

    impl HillslopeKernel for PhaseMismatchKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            let status = openwepp_sim_contract::status::SimulationStatus::ok(
                SimulationPhase::PreExecutionValidation,
                "HKERNEL-PHASE-INVALID",
            )
            .expect("status should construct");
            KernelRunResponse::new(status, KernelWritebackPayload::empty())
        }
    }

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = PhaseMismatchKernel;

    let report = scheduler
        .execute_with_kernel(
            &topology_report,
            &mut kernel,
            HillslopeWritebackSurface::default(),
        )
        .expect("execution should return typed report");

    assert_eq!(
        report.scheduler_report.outcome_class,
        SchedulerOutcomeClass::PhaseFailure
    );
    assert_eq!(
        report.scheduler_report.scheduler_status.boundary_class(),
        BoundaryClass::ModeMismatch
    );
    assert_eq!(report.phase_reports.len(), 1);
    assert_eq!(
        report.phase_reports[0].decision_outcome,
        WritebackDecisionOutcome::Reject
    );
}

fn canonical_dependency_map_for_test() -> BTreeMap<HillslopePhase, Vec<HillslopePhase>> {
    let order = HillslopePhaseGraph::canonical_order();
    let mut dependencies = BTreeMap::new();
    for phase in order {
        dependencies.insert(phase, Vec::new());
    }
    for pair in order.windows(2) {
        dependencies
            .entry(pair[1])
            .or_insert_with(Vec::new)
            .push(pair[0]);
    }
    dependencies
}

#[test]
fn schedule_export_formats_reflect_canonical_graph() {
    let export = ScheduleExport::from_graph(&HillslopePhaseGraph::canonical())
        .expect("canonical graph should export");
    let json = export.render_json();
    let mermaid = export.render_mermaid();
    let dot = export.render_dot();

    assert_eq!(
        export.nodes.len(),
        HillslopePhaseGraph::canonical_order().len()
    );
    assert_eq!(
        export.edges.len(),
        HillslopePhaseGraph::canonical_order().len() - 1
    );
    assert!(json.contains("\"phase\": \"normalization\""));
    assert!(json.contains("\"consumer_adapter\": \"soil\""));
    assert!(json.contains("\"topological_order\""));
    assert!(mermaid.starts_with("flowchart TD\n"));
    assert!(mermaid.contains("normalization --> storage_bounds"));
    assert!(dot.starts_with("digraph hillslope_phase_schedule {\n"));
    assert!(dot.contains("\"normalization\" -> \"storage_bounds\";"));
}

#[test]
fn schedule_export_validation_reports_cycle() {
    let mut dependencies = canonical_dependency_map_for_test();
    dependencies.insert(
        HillslopePhase::Normalization,
        vec![HillslopePhase::ClosureDiagnostics],
    );
    let graph = HillslopePhaseGraph::from_dependencies_for_test(dependencies);
    let report = validate_hillslope_schedule_graph(&graph);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic == &ScheduleDiagnostic::GraphCycle)
    );
    assert!(
        ScheduleExport::from_graph(&graph).is_err(),
        "malformed graph must not export silently"
    );
}

#[test]
fn schedule_export_validation_reports_disconnected_phase() {
    let mut dependencies = canonical_dependency_map_for_test();
    dependencies.insert(HillslopePhase::StorageBounds, Vec::new());
    let graph = HillslopePhaseGraph::from_dependencies_for_test(dependencies);
    let report = validate_hillslope_schedule_graph(&graph);

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic
            == &ScheduleDiagnostic::UnreachableFromCanonicalRoot {
                phase: HillslopePhase::StorageBounds,
            }
    }));
}

#[test]
fn schedule_export_validation_reports_topological_order_drift() {
    let mut dependencies = canonical_dependency_map_for_test();
    dependencies.insert(
        HillslopePhase::Evapotranspiration,
        vec![HillslopePhase::PerennialGrowthTransition],
    );
    dependencies.insert(
        HillslopePhase::PercolationDeepSeepage,
        vec![HillslopePhase::Evapotranspiration],
    );
    let graph = HillslopePhaseGraph::from_dependencies_for_test(dependencies);
    let report = validate_hillslope_schedule_graph(&graph);

    assert!(report.diagnostics.iter().any(|diagnostic| matches!(
        diagnostic,
        ScheduleDiagnostic::CanonicalOrderMismatch { .. }
    )));
}

#[test]
fn schedule_diff_reports_added_and_removed_nodes_and_edges() {
    let base = r#"{
  "nodes": [
    {"phase": "normalization", "rank": 0, "consumer_adapter": "soil"}
  ],
  "edges": [
    {"from": "normalization", "to": "storage_bounds"}
  ],
  "topological_order": ["normalization"]
}"#;
    let head = r#"{
  "nodes": [
    {"phase": "storage_bounds", "rank": 1, "consumer_adapter": "soil"}
  ],
  "edges": [
    {"from": "storage_bounds", "to": "decomposition_transition"}
  ],
  "topological_order": ["storage_bounds"]
}"#;

    let diff = diff_schedule_json(base, head).expect("synthetic exports should diff");
    assert_eq!(diff.added_nodes, vec!["storage_bounds"]);
    assert_eq!(diff.removed_nodes, vec!["normalization"]);
    assert_eq!(diff.added_edges[0].from, "storage_bounds");
    assert_eq!(diff.added_edges[0].to, "decomposition_transition");
    assert_eq!(diff.removed_edges[0].from, "normalization");
    assert_eq!(diff.removed_edges[0].to, "storage_bounds");

    let rendered = render_schedule_diff(&diff);
    assert!(rendered.contains("Added nodes:"));
    assert!(rendered.contains("- storage_bounds"));
    assert!(rendered.contains("Removed edges:"));
    assert!(rendered.contains("- normalization -> storage_bounds"));
}
