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

use crate::schedule_export::{
    ScheduleDiagnostic, ScheduleExport, diff_schedule_json, render_schedule_diff,
    validate_hillslope_schedule_graph,
};
use crate::{
    Wb11HydrologyKernel,
    consumer_boundary::{
        hillslope_consumer_adapter_for_phase, required_hillslope_consumer_state_symbols,
        validate_hillslope_consumer_boundary,
    },
    phase::HillslopePhase,
    scheduler::HillslopePhaseGraph,
    scheduler::HillslopePhaseScheduler,
    scheduler::HillslopeWritebackSurface,
    scheduler::SchedulerOutcomeClass,
};

pub(super) const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

pub(super) const INVALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 0 0 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

pub(super) fn valid_topology_report() -> openwepp_topology::TopologyValidationReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    validate_pre_execution_topology(&graph).expect("topology report should build")
}

pub(super) fn state_update_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
    let target = BoundarySymbol::from(symbol);
    fields.iter().find_map(|field| {
        if field.symbol == target {
            Some(field.value.as_f64())
        } else {
            None
        }
    })
}

pub(super) fn flux_update_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
    let target = BoundarySymbol::from(symbol);
    fields.iter().find_map(|field| {
        if field.symbol == target {
            Some(field.value.as_f64())
        } else {
            None
        }
    })
}

pub(super) fn hphys0246_wb18_aggregate_state_surface() -> BTreeMap<BoundarySymbol, BoundaryValue> {
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
pub(super) fn seeded_growth_runtime_surface_for_day_year(
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

pub(super) fn seeded_growth_runtime_surface(imngmt: f64) -> HillslopeWritebackSurface {
    seeded_growth_runtime_surface_for_day_year(imngmt, 200.0, 1.0)
}

pub(super) fn seed_legacy_monthly_temperature_vectors(surface: &mut HillslopeWritebackSurface) {
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
pub(super) fn seeded_multislot_rotation_surface(
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
