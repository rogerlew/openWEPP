use openwepp_input_contract::parsers::{
    chaninp::{ChaninpParseOptions, parse_chaninp_from_str},
    watershed_channel::{WatershedChannelParseOptions, parse_watershed_channel_from_str},
    watershed_impoundment::{
        WatershedImpoundmentParseOptions, parse_watershed_impoundment_from_str,
    },
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
use openwepp_sim_contract::status::BoundaryClass;
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};
use openwepp_watershed_orchestrator::{
    WatershedWritebackSurface, Ws10ChannelImpoundmentKernel,
    execute_watershed_dispatch_with_kernel,
    runtime_inputs::{
        build_watershed_runtime_surface_from_chaninp,
        seed_watershed_runtime_surface_from_watershed_channel,
        seed_watershed_runtime_surface_from_watershed_impoundment,
    },
};

const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 2 0 0 C 1 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 0 0 0 I 1 0 0
";

const STRICT_VALID_CHANINP: &str = include_str!("../fixtures/infile/chaninp/strict_valid.chaninp");
const STRICT_VALID_WATERSHED_CHANNEL: &str =
    include_str!("../fixtures/infile/watershed_channel/strict_valid_single_channel.chn");
const STRICT_VALID_WATERSHED_IMPOUNDMENT: &str =
    include_str!("../fixtures/infile/watershed_impoundment/strict_valid_minimal.imp");
const STRICT_VALID_WATERSHED_IMPOUNDMENT_ACTIVE: &str =
    include_str!("../fixtures/infile/watershed_impoundment/strict_valid_active_payloads.imp");
const EROD15_CLASS_COUNT_SCALAR: f64 = 3.0;

fn class_index_scalar(class: usize) -> f64 {
    f64::from(u32::try_from(class).expect("class index should fit within u32 range"))
}

fn seed_erod15_hillslope_payload(
    surface: &mut WatershedWritebackSurface,
    hillslope_id: u32,
    peakro: f64,
    watdur: f64,
) {
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_peakro")),
        BoundaryValue::scalar(peakro),
    );
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_watdur")),
        BoundaryValue::scalar(watdur),
    );
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_total_detachment_kg")),
        BoundaryValue::scalar((peakro * watdur * 0.01).max(0.0)),
    );
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_total_deposition_kg")),
        BoundaryValue::scalar((peakro * watdur * 0.0025).max(0.0)),
    );
    surface.state_surface.insert(
        BoundarySymbol::from(format!("hs{hillslope_id}_particle_class_count")),
        BoundaryValue::scalar(EROD15_CLASS_COUNT_SCALAR),
    );

    for (class_index, fraction) in [0.2, 0.3, 0.5].into_iter().enumerate() {
        let class = class_index + 1;
        surface.state_surface.insert(
            BoundarySymbol::from(format!(
                "hs{hillslope_id}_sediment_concentration_kg_m3_{class:04}"
            )),
            BoundaryValue::scalar(0.25 + (class_index_scalar(class) * 0.1)),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("hs{hillslope_id}_particle_diameter_m_{class:04}")),
            BoundaryValue::scalar(0.000_01 * class_index_scalar(class)),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!(
                "hs{hillslope_id}_particle_flow_fraction_{class:04}"
            )),
            BoundaryValue::scalar(fraction),
        );
    }
}

fn seeded_ws12_surface_with_impoundment_fixture(
    impoundment_fixture: &str,
) -> WatershedWritebackSurface {
    let valid_channel_element_ids = std::collections::BTreeSet::from([4, 5]);
    let chaninp = parse_chaninp_from_str(
        STRICT_VALID_CHANINP,
        ChaninpParseOptions::strict(3, 2),
        &valid_channel_element_ids,
    )
    .expect("strict chan.inp fixture should parse");
    let mut runtime_surface = build_watershed_runtime_surface_from_chaninp(&chaninp)
        .expect("chan.inp runtime surface should build");

    let watershed_channel = parse_watershed_channel_from_str(
        STRICT_VALID_WATERSHED_CHANNEL,
        WatershedChannelParseOptions::default(),
    )
    .expect("strict watershed channel fixture should parse");
    seed_watershed_runtime_surface_from_watershed_channel(&mut runtime_surface, &watershed_channel)
        .expect("watershed channel runtime seed should project ws10 symbols");

    // Duplicate channel-1 control symbols for channel:2 so the topology fixture
    // has per-channel controls on both channel nodes.
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chnn"),
        BoundaryValue::scalar(0.05),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_ctlslp"),
        BoundaryValue::scalar(0.03),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chnk"),
        BoundaryValue::scalar(0.000_002),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_ishape"),
        BoundaryValue::scalar(1.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_ienslp"),
        BoundaryValue::scalar(1.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chnz"),
        BoundaryValue::scalar(19.99),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chnnbr"),
        BoundaryValue::scalar(0.03),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chntcr"),
        BoundaryValue::scalar(19.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chnedm"),
        BoundaryValue::scalar(900.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_chneds"),
        BoundaryValue::scalar(0.0001),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_ctlz"),
        BoundaryValue::scalar(4.0),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("ws10_channel_2_ctln"),
        BoundaryValue::scalar(0.04),
    );

    let impoundment = parse_watershed_impoundment_from_str(
        impoundment_fixture,
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect("strict watershed impoundment fixture should parse");
    seed_watershed_runtime_surface_from_watershed_impoundment(&mut runtime_surface, &impoundment)
        .expect("watershed impoundment runtime seed should project ws10 symbols");

    seed_erod15_hillslope_payload(&mut runtime_surface, 1, 2.0, 300.0);
    seed_erod15_hillslope_payload(&mut runtime_surface, 2, 1.5, 400.0);
    seed_erod15_hillslope_payload(&mut runtime_surface, 3, 0.5, 200.0);
    runtime_surface
}

fn seeded_ws12_surface() -> WatershedWritebackSurface {
    seeded_ws12_surface_with_impoundment_fixture(STRICT_VALID_WATERSHED_IMPOUNDMENT)
}

fn seeded_ws12_active_surface() -> WatershedWritebackSurface {
    seeded_ws12_surface_with_impoundment_fixture(STRICT_VALID_WATERSHED_IMPOUNDMENT_ACTIVE)
}

fn run_ws12_surface(
    surface: WatershedWritebackSurface,
) -> openwepp_watershed_orchestrator::WatershedKernelExecutionReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let mut kernel = Ws10ChannelImpoundmentKernel;
    execute_watershed_dispatch_with_kernel(&graph, &topology_report, &mut kernel, surface)
        .expect("ws10 execution should return typed report")
}

fn state_value(
    report: &openwepp_watershed_orchestrator::WatershedKernelExecutionReport,
    symbol: &str,
) -> f64 {
    report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing state symbol {symbol}"))
        .as_f64()
}

#[test]
fn ws12_contract_conformance_deauthorizes_surrogate_when_structures_are_inactive() {
    let report = run_ws12_surface(seeded_ws12_surface());
    assert!(report.dispatch_report.is_success());

    let qo = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_qo"))
        .expect("ws12 impoundment must publish qo")
        .as_f64();

    // Contract vector: inactive structures contribute zero to outflow
    // composition under continuity/stage-discharge authority.
    assert!(qo.abs() <= 1.0e-12);
}

#[test]
fn ws12_contract_conformance_rejects_missing_required_coefficient_payload() {
    let mut surface = seeded_ws12_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("ws10_impoundment_1_a0"));

    let report = run_ws12_surface(surface);
    assert_eq!(report.step_reports.len(), 2);
    assert_eq!(
        report.step_reports[1].decision_status.message_id(),
        "WKERNEL-WS10-IMPOUNDMENT-E-001"
    );
    assert_eq!(
        report.step_reports[1].decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn ws12_contract_conformance_rejects_non_finite_coefficient_payload() {
    let mut surface = seeded_ws12_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_a1"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = run_ws12_surface(surface);
    assert_eq!(report.step_reports.len(), 2);
    assert_eq!(
        report.step_reports[1].decision_status.message_id(),
        "WKERNEL-WS10-IMPOUNDMENT-E-002"
    );
    assert_eq!(
        report.step_reports[1].decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn ws12_contract_conformance_rejects_invalid_area_denominator() {
    let mut surface = seeded_ws12_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_a0"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_a1"),
        BoundaryValue::scalar(0.0),
    );

    let report = run_ws12_surface(surface);
    assert_eq!(report.step_reports.len(), 2);
    assert_eq!(
        report.step_reports[1].decision_status.message_id(),
        "WKERNEL-WS10-IMPOUNDMENT-E-003"
    );
    assert_eq!(
        report.step_reports[1].decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn wshed03_contract_ws12_vector_requires_parser_projected_coefficients_without_manual_seed() {
    let surface = seeded_ws12_surface();

    let report = run_ws12_surface(surface);
    assert!(
        report.dispatch_report.is_success(),
        "ws12 execution should succeed with parser-projected coefficients and no manual seed; step_reports={:?}",
        report.step_reports
    );

    let qo = state_value(&report, "ws10_impoundment_1_qo");
    let durout = state_value(&report, "ws10_impoundment_1_durout");
    let hnext = state_value(&report, "ws10_impoundment_1_hnext");
    let outflow_volume = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("ws10_impoundment_1_outflow_volume"))
        .unwrap_or_else(|| panic!("missing flux symbol ws10_impoundment_1_outflow_volume"))
        .as_f64();

    assert!(qo.is_finite() && qo >= 0.0);
    assert!(durout.is_finite() && durout >= 0.0);
    assert!(hnext.is_finite() && hnext >= 0.0);
    assert!(outflow_volume.is_finite() && outflow_volume >= 0.0);
}

#[test]
fn wshed11_contract_ws12_vector_projects_active_structure_payloads() {
    let report = run_ws12_surface(seeded_ws12_active_surface());
    assert!(
        report.dispatch_report.is_success(),
        "ws12 execution should succeed on active payload projection surfaces; step_reports={:?}",
        report.step_reports
    );

    let qo = state_value(&report, "ws10_impoundment_1_qo");
    let durout = state_value(&report, "ws10_impoundment_1_durout");
    let hnext = state_value(&report, "ws10_impoundment_1_hnext");

    assert!(qo.is_finite() && qo >= 0.0);
    assert!(durout.is_finite() && durout >= 0.0);
    assert!(hnext.is_finite() && hnext >= 0.0);
}

#[test]
#[allow(clippy::many_single_char_names, clippy::too_many_lines)]
fn wshed13_contract_ws12_vector_uses_full_min_controller_outflow_composition() {
    let report = run_ws12_surface(seeded_ws12_active_surface());
    assert!(
        report.dispatch_report.is_success(),
        "ws12 active run should succeed for min-controller composition verification"
    );

    let hnext = state_value(&report, "ws10_impoundment_1_hnext");
    let qo = state_value(&report, "ws10_impoundment_1_qo");

    let families = |index: usize, suffix: &str| -> f64 {
        state_value(&report, &format!("ws10_impoundment_1_f{index:02}_{suffix}"))
    };

    let a: [f64; 15] = std::array::from_fn(|i| families(i + 1, "a"));
    let b: [f64; 15] = std::array::from_fn(|i| families(i + 1, "b"));
    let c: [f64; 15] = std::array::from_fn(|i| families(i + 1, "c"));
    let d: [f64; 15] = std::array::from_fn(|i| families(i + 1, "d"));
    let e: [f64; 15] = std::array::from_fn(|i| families(i + 1, "e"));
    let ha: [f64; 15] = std::array::from_fn(|i| families(i + 1, "ha"));

    let htw = 0.0;
    let q1 = if hnext > ha[0] {
        b[0] * (hnext - ha[0]).powf(c[0])
    } else {
        0.0
    };
    let q2 = if hnext > ha[1] {
        b[1] * (hnext - ha[1]).powf(c[1])
    } else {
        0.0
    };
    let q3 = if hnext > ha[2] {
        let head = if htw > a[2] {
            hnext - (ha[2] + htw - a[2])
        } else {
            hnext - ha[2]
        };
        if head > 0.0 {
            b[2] * head.powf(c[2])
        } else {
            0.0
        }
    } else {
        0.0
    };
    let q4 = if hnext > ha[3] {
        let base = (hnext - ha[3]) / b[3];
        if base > 0.0 {
            a[3] * base.powf(c[3])
        } else {
            0.0
        }
    } else {
        0.0
    };
    let q5 = if hnext > ha[4] {
        let base = (((hnext - ha[4]) / b[4]) + c[4]) / d[4];
        if base > 0.0 { a[4] * base.sqrt() } else { 0.0 }
    } else {
        0.0
    };
    let q6 = if hnext > ha[5] {
        let head = if htw > a[5] {
            hnext - (ha[5] + htw - a[5])
        } else {
            hnext - ha[5]
        };
        if head > 0.0 {
            b[5] * head.powf(c[5])
        } else {
            0.0
        }
    } else {
        0.0
    };
    let q7 = if hnext > ha[6] {
        let base = (hnext - ha[6]) / b[6];
        if base > 0.0 {
            a[6] * base.powf(c[6])
        } else {
            0.0
        }
    } else {
        0.0
    };
    let q8 = if hnext > ha[7] {
        let base = (((hnext - ha[7]) / b[7]) + c[7]) / d[7];
        if base > 0.0 { a[7] * base.sqrt() } else { 0.0 }
    } else {
        0.0
    };
    let q9 = if hnext > ha[8] {
        let head = if htw > a[8] {
            hnext - (ha[8] + htw - a[8])
        } else {
            hnext - ha[8]
        };
        if head > 0.0 {
            b[8] * head.powf(c[8])
        } else {
            0.0
        }
    } else {
        0.0
    };
    let mut q10 = if hnext > ha[9] {
        let base = (hnext - ha[9]) / b[9];
        if base > 0.0 {
            a[9] * base.powf(c[9])
        } else {
            0.0
        }
    } else {
        0.0
    };
    if hnext > e[9] {
        q10 += d[9] * (hnext - e[9]).powf(1.5);
    }
    let q11 = if hnext > ha[10] {
        let x = hnext - ha[10];
        let poly = a[10] + b[10] * x + c[10] * x.powi(2) + d[10] * x.powi(3) + e[10] * x.powi(4);
        poly.max(0.0)
    } else {
        0.0
    };
    let mut q12 = if hnext > ha[11] {
        a[11] * (hnext - ha[11])
    } else {
        0.0
    };
    if hnext > d[11] {
        let dx = hnext - d[11];
        q12 += (b[11] + c[11] * dx) * dx.powf(1.5);
    }
    let q13 = if hnext > ha[12] {
        let x = hnext - ha[12];
        a[12] / (b[12] + c[12] / x.powf(1.5))
    } else {
        0.0
    };
    let q14 = if hnext > ha[13] {
        a[13] * (hnext - ha[13]).sqrt()
    } else {
        0.0
    };
    let q15 = if hnext > ha[14] {
        b[14] * (hnext - ha[14]).powf(c[14])
    } else {
        0.0
    };

    let expected_qo = q1.min(q2).min(q3)
        + q4.min(q5).min(q6)
        + q7.min(q8).min(q9)
        + q10
        + q11
        + q12
        + q13.min(q14).min(q15);
    assert!(
        (qo - expected_qo).abs() <= 1.0e-9,
        "qo should match 15-function min-controller composition (expected={expected_qo}, observed={qo})"
    );
}

#[test]
fn wshed03_contract_ws12_vector_requires_regime_transition_timestep_stability() {
    let mut fine_step = seeded_ws12_surface();
    fine_step.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_deltat"),
        BoundaryValue::scalar(0.1),
    );
    fine_step.state_surface.insert(
        BoundarySymbol::from("hs1_peakro"),
        BoundaryValue::scalar(4.0),
    );
    fine_step.state_surface.insert(
        BoundarySymbol::from("hs2_peakro"),
        BoundaryValue::scalar(3.0),
    );
    fine_step.state_surface.insert(
        BoundarySymbol::from("hs3_peakro"),
        BoundaryValue::scalar(2.0),
    );

    let mut coarse_step = fine_step.clone();
    coarse_step.state_surface.insert(
        BoundarySymbol::from("ws10_impoundment_1_deltat"),
        BoundaryValue::scalar(1.0),
    );

    let fine_report = run_ws12_surface(fine_step);
    let coarse_report = run_ws12_surface(coarse_step);
    assert!(
        fine_report.dispatch_report.is_success(),
        "fine-step ws12 run should succeed for RK4/regime-transition comparison"
    );
    assert!(
        coarse_report.dispatch_report.is_success(),
        "coarse-step ws12 run should succeed for RK4/regime-transition comparison"
    );

    let fine_hnext = state_value(&fine_report, "ws10_impoundment_1_hnext");
    let coarse_hnext = state_value(&coarse_report, "ws10_impoundment_1_hnext");
    assert!(
        (fine_hnext - coarse_hnext).abs() <= 1.0e-3,
        "RK4/adaptive regime-transition lineage requires timestep stability (fine={fine_hnext}, coarse={coarse_hnext})"
    );
}
