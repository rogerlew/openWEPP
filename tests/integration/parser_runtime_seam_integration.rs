use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use openwepp_hillslope_orchestrator::{
    HillslopePhaseGraph, HillslopePhaseScheduler, HillslopeWritebackSurface,
    runtime_inputs::{
        HillslopePlRuntimeSurfaces, HillslopeRuntimeInputError,
        build_hillslope_pl_runtime_surfaces_from_management,
        build_hillslope_runtime_surface_from_climate,
        build_hillslope_runtime_surface_from_management,
        build_hillslope_runtime_surface_from_slope, build_hillslope_runtime_surface_from_soil,
    },
};
use openwepp_input_contract::parsers::{
    chaninp::{ChaninpParseOptions, parse_chaninp_from_str},
    climate::{ParserMode as ClimateParserMode, parse_climate_from_str},
    management::{
        ManagementParseOutput, ParseMode as ManagementParseMode, YearlyAnnualExtension,
        YearlyCroplandBranch, YearlyPerennialData, YearlyPerennialGrazingCycle, YearlyScenarioData,
        parse_management_from_path,
    },
    slope::{SlopeParserOptions, parse_slope_str},
    soil::{SoilParserOptions, parse_soil},
};
use openwepp_kernel_contract::{
    BoundarySymbol, HillslopeKernel, HillslopeKernelRequest, KernelRunResponse,
    KernelWritebackPayload, WatershedKernel, WatershedKernelRequest,
};
use openwepp_sim_contract::status::{SimulationPhase, SimulationStatus};
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};
use openwepp_watershed_orchestrator::{
    execute_watershed_dispatch_with_kernel,
    runtime_inputs::{
        build_watershed_runtime_surface_from_chaninp,
        build_watershed_runtime_surface_from_climate_assignments,
    },
};

const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

const SOIL_VALID_9002: &str = include_str!("../fixtures/infile/soil/valid_9002.sol");
const SLOPE_STRICT_VALID_CANONICAL: &str =
    include_str!("../fixtures/infile/slope/strict_valid_canonical.slp");
const CHANINP_STRICT_VALID: &str = include_str!("../fixtures/infile/chaninp/strict_valid.chaninp");
const CLIMATE_STRICT_VALID: &str = include_str!("../fixtures/infile/climate/strict_valid.cli");
const CLIMATE_WC1_DAY1: &str = include_str!("../fixtures/infile/climate/wc1_canoga_day1.cli");
const CLIMATE_WC1_STMDUR_CAP: &str =
    include_str!("../fixtures/infile/climate/wc1_canoga_stmdur_cap.cli");

struct HillslopeSeedProbeKernel {
    invocation_count: usize,
}

impl HillslopeKernel for HillslopeSeedProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "solthk", 0.25);
        assert_state_value(request.state_surface, "dg", 0.1);
        assert_state_value(request.state_surface, "thetdr", 0.05);
        assert_state_value(request.state_surface, "thetfc", 0.31);
        assert_state_value(request.state_surface, "nsl", 2.0);
        assert_state_value(request.state_surface, "ssc", 15.0 / 3.6e6);
        assert_state_value(request.state_surface, "dg_0002", 0.15);
        assert_state_value(request.state_surface, "solthk_0002", 0.25);
        assert_state_value(request.state_surface, "ssc_0002", 8.0 / 3.6e6);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "ARCH17-HS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

struct WatershedSeedProbeKernel {
    invocation_count: usize,
}

impl WatershedKernel for WatershedSeedProbeKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "ipeak", 3.0);
        assert_state_value(request.state_surface, "nchan", 2.0);
        assert_state_value(request.state_surface, "dtchr", 600.0);
        assert_state_value(request.state_surface, "ntchr", 144.0);
        assert_state_value(request.state_surface, "nchnum", 2.0);
        assert_state_value(request.flux_surface, "cbase", 0.000_001);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::WatershedKernel, "ARCH17-WS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

struct HillslopeClimateProbeKernel {
    invocation_count: usize,
}

struct HillslopeSlopeProbeKernel {
    invocation_count: usize,
}

struct HillslopeSlopeSoilProbeKernel {
    invocation_count: usize,
}

impl HillslopeKernel for HillslopeSlopeProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "nelem", 2.0);
        assert_state_value(request.state_surface, "nwsofe", 2.0);
        assert_state_value(request.state_surface, "nslpts", 3.0);
        assert_state_value(request.state_surface, "slplen", 60.0);
        assert_state_value(request.state_surface, "avgslp", 0.058);
        assert_state_value(request.state_surface, "xinput_0001", 0.0);
        assert_state_value(request.state_surface, "xinput_0002", 0.6);
        assert_state_value(request.state_surface, "slpinp_0002", 0.08);
        assert_state_value(request.state_surface, "ofe2_nslpts", 3.0);
        assert_state_value(request.state_surface, "ofe2_slplen", 40.0);
        assert_state_value(request.state_surface, "ofe2_avgslp", 0.0425);
        assert_state_value(request.state_surface, "ofe2_xinput_0003", 1.0);
        assert_state_value(request.state_surface, "ofe2_slpinp_0003", 0.03);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "ARCH17-HS-SLOPE-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

impl HillslopeKernel for HillslopeSlopeSoilProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "solthk", 0.25);
        assert_state_value(request.state_surface, "dg", 0.1);
        assert_state_value(request.state_surface, "thetdr", 0.05);
        assert_state_value(request.state_surface, "thetfc", 0.31);
        assert_state_value(request.state_surface, "nsl", 2.0);
        assert_state_value(request.state_surface, "ssc", 15.0 / 3.6e6);
        assert_state_value(request.state_surface, "ssc_0002", 8.0 / 3.6e6);
        assert_state_value(request.state_surface, "nelem", 2.0);
        assert_state_value(request.state_surface, "nwsofe", 2.0);
        assert_state_value(request.state_surface, "nslpts", 3.0);
        assert_state_value(request.state_surface, "slplen", 60.0);
        assert_state_value(request.state_surface, "avgslp", 0.058);
        assert_state_value(request.state_surface, "xinput_0002", 0.6);
        assert_state_value(request.state_surface, "slpinp_0002", 0.08);
        assert_state_value(request.state_surface, "ofe2_avgslp", 0.0425);
        assert_state_value(request.state_surface, "ofe2_xinput_0003", 1.0);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "ARCH17-HS-SLOPE-SOIL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

impl HillslopeKernel for HillslopeClimateProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "datver", 5.3);
        assert_state_value(request.state_surface, "iclig", 1.0);
        assert_state_value(request.state_surface, "itemp", 1.0);
        assert_state_value(request.state_surface, "ibrkpt", 0.0);
        assert_state_value(request.state_surface, "iwind", 0.0);
        assert_state_value(request.state_surface, "prcp", 0.01);
        assert_state_value(request.state_surface, "stmdur", 7_200.0);
        assert_state_value(request.state_surface, "timep", 0.25);
        assert_state_value(request.state_surface, "ip", 2.1);
        assert_state_at_least(request.state_surface, "ninten", 2.0);
        assert_state_value(request.state_surface, "timem_0001", 0.0);
        assert_state_value(request.state_surface, "tmax", 12.0);
        assert_state_value(request.state_surface, "tmin", 2.0);
        assert_state_value(request.state_surface, "rad", 200.0);
        assert_state_value(request.state_surface, "tdpt", -1.0);
        assert_state_value(request.state_surface, "vwind", 3.0);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "CLIM02-HS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

struct WatershedClimateProbeKernel {
    invocation_count: usize,
}

impl WatershedKernel for WatershedClimateProbeKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "nclimhs", 3.0);
        assert_state_value(request.state_surface, "hs1_datver", 5.3);
        assert_state_value(request.state_surface, "hs2_datver", 5.3);
        assert_state_value(request.state_surface, "hs3_datver", 5.3);
        assert_state_value(request.state_surface, "hs1_prcp", 0.01);
        assert_state_value(request.state_surface, "hs2_stmdur", 7_200.0);
        assert_state_value(request.state_surface, "hs3_timep", 0.25);
        assert_state_value(request.state_surface, "hs1_ip", 2.1);
        assert_state_at_least(request.state_surface, "hs2_ninten", 2.0);
        assert_state_value(request.state_surface, "hs3_timem_0001", 0.0);
        assert_state_value(request.state_surface, "hs2_tmax", 12.0);
        assert_state_value(request.state_surface, "hs3_tmin", 2.0);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::WatershedKernel, "CLIM02-WS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

#[test]
fn parser_to_hillslope_runtime_surface_closure() {
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse for seam closure");
    let runtime_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("runtime surface should build from soil parser output");

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = HillslopeSeedProbeKernel {
        invocation_count: 0,
    };
    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .expect("hillslope execution should succeed");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.invocation_count,
        HillslopePhaseGraph::canonical_order().len()
    );
}

#[test]
fn soil_runtime_surface_rejects_missing_saturated_conductivity_projection() {
    let mut soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    soil.ofes[0].layers[0].ksat_mm_h = None;

    let error = build_hillslope_runtime_surface_from_soil(&soil)
        .expect_err("missing ksat must fail with typed seam guard");
    assert_eq!(error.code(), "HS-RUNTIME-E-033");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::MissingSaturatedConductivity {
            ofe_index: 1,
            layer_index: 1
        }
    ));
}

#[test]
fn slope_and_soil_parser_outputs_propagate_to_hillslope_runtime_surface_closure() {
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse for seam closure");
    let slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
        .expect("slope fixture should parse for seam closure");

    let soil_runtime_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("soil runtime surface should build from parser output");
    let slope_runtime_surface = build_hillslope_runtime_surface_from_slope(&slope)
        .expect("slope runtime surface should build from parser output");
    let runtime_surface =
        merge_hillslope_runtime_surfaces(soil_runtime_surface, slope_runtime_surface);

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = HillslopeSlopeSoilProbeKernel {
        invocation_count: 0,
    };
    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .expect("hillslope execution should consume both slope and soil runtime symbols");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.invocation_count,
        HillslopePhaseGraph::canonical_order().len()
    );
}

#[test]
fn slope_parser_to_hillslope_runtime_surface_closure() {
    let slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
        .expect("slope fixture should parse for seam closure");
    let runtime_surface = build_hillslope_runtime_surface_from_slope(&slope)
        .expect("runtime surface should build from slope parser output");

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = HillslopeSlopeProbeKernel {
        invocation_count: 0,
    };
    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .expect("hillslope execution should consume slope runtime symbols");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.invocation_count,
        HillslopePhaseGraph::canonical_order().len()
    );
}

#[test]
fn slope_runtime_surface_rejects_declared_nslpts_mismatch_projection() {
    let mut slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
        .expect("slope fixture should parse");
    slope.ofes[0].nslpts += 1;

    let error = build_hillslope_runtime_surface_from_slope(&slope)
        .expect_err("nslpts mismatch must fail with typed seam guard");
    assert_eq!(error.code(), "HS-RUNTIME-E-014");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::SlopePointCountMismatch {
            ofe_index: 1,
            declared_nslpts,
            observed_points
        } if declared_nslpts == observed_points + 1
    ));
}

#[test]
fn slope_runtime_surface_rejects_non_positive_avgslp_projection() {
    let mut slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
        .expect("slope fixture should parse");
    for point in &mut slope.ofes[0].points {
        point.slpinp = 0.0;
    }

    let error = build_hillslope_runtime_surface_from_slope(&slope)
        .expect_err("non-positive avgslp projection must fail with typed guard");
    assert_eq!(error.code(), "HS-RUNTIME-E-023");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::NonPositiveDerivedAverageSlope {
            ofe_index: 1,
            value
        } if value.abs() < 1e-12
    ));
}

#[test]
fn soil_runtime_surface_rejects_declared_nsl_mismatch_projection() {
    let mut soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    soil.ofes[0].nsl += 1;

    let error = build_hillslope_runtime_surface_from_soil(&soil)
        .expect_err("nsl mismatch must fail with typed seam guard");
    assert_eq!(error.code(), "HS-RUNTIME-E-028");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::SoilLayerCountMismatch {
            ofe_index: 1,
            declared_nsl,
            observed_layers
        } if declared_nsl == observed_layers + 1
    ));
}

#[test]
fn parser_to_watershed_runtime_surface_closure() {
    let valid_channel_ids = BTreeSet::from([4, 5]);
    let chaninp = parse_chaninp_from_str(
        CHANINP_STRICT_VALID,
        ChaninpParseOptions::strict(3, 2),
        &valid_channel_ids,
    )
    .expect("chan.inp fixture should parse for seam closure");
    let runtime_surface = build_watershed_runtime_surface_from_chaninp(&chaninp)
        .expect("runtime surface should build from chan.inp parser output");

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let mut kernel = WatershedSeedProbeKernel {
        invocation_count: 0,
    };
    let report = execute_watershed_dispatch_with_kernel(
        &graph,
        &topology_report,
        &mut kernel,
        runtime_surface,
    )
    .expect("watershed execution should succeed");

    assert!(report.dispatch_report.is_success());
    assert_eq!(kernel.invocation_count, report.dispatch_report.steps.len());
}

#[test]
fn climate_parser_to_hillslope_runtime_surface_closure() {
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse for hillslope seam closure");
    let runtime_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("hillslope climate runtime surface should build from parser output");

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = HillslopeClimateProbeKernel {
        invocation_count: 0,
    };
    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
        .expect("hillslope execution should consume climate runtime symbols");

    assert!(report.scheduler_report.is_success());
    assert_eq!(
        kernel.invocation_count,
        HillslopePhaseGraph::canonical_order().len()
    );
}

#[test]
fn climate_parser_to_watershed_runtime_surface_closure() {
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse for watershed seam closure");
    let assignments = BTreeMap::from([
        (1_u32, climate.clone()),
        (2_u32, climate.clone()),
        (3_u32, climate),
    ]);
    let runtime_surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("watershed climate runtime surface should build from parser outputs");

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");

    let mut kernel = WatershedClimateProbeKernel {
        invocation_count: 0,
    };
    let report = execute_watershed_dispatch_with_kernel(
        &graph,
        &topology_report,
        &mut kernel,
        runtime_surface,
    )
    .expect("watershed execution should consume climate assignment runtime symbols");

    assert!(report.dispatch_report.is_success());
    assert_eq!(kernel.invocation_count, report.dispatch_report.steps.len());
}

#[test]
fn climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path() {
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse for parity check");
    let hillslope_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("hillslope climate runtime surface should build");

    let assignments = BTreeMap::from([(7_u32, climate)]);
    let watershed_surface =
        build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
            .expect("watershed climate runtime surface should build");
    assert_state_value(&watershed_surface.state_surface, "nclimhs", 1.0);

    for (symbol, value) in &hillslope_surface.state_surface {
        let watershed_symbol = format!("hs7_{}", symbol.as_str());
        let watershed_value = watershed_surface
            .state_surface
            .get(&BoundarySymbol::from(watershed_symbol.as_str()))
            .unwrap_or_else(|| panic!("missing watershed parity symbol {watershed_symbol}"))
            .as_f64();
        let hillslope_value = (*value).as_f64();

        assert!(
            (hillslope_value - watershed_value).abs() < 1e-12,
            "parity mismatch for {} / {}: hillslope={} watershed={}",
            symbol.as_str(),
            watershed_symbol,
            hillslope_value,
            watershed_value
        );
    }
}

#[test]
fn climate_wc1_fixture_applies_timep_floor_and_ip_policy_scaling() {
    let climate = parse_climate_from_str(CLIMATE_WC1_DAY1, ClimateParserMode::Strict)
        .expect("wc1 climate fixture should parse");
    let surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("hillslope runtime surface should build from wc1 fixture");
    assert_state_value(&surface.state_surface, "timep", 0.01);
    assert_state_value(&surface.state_surface, "ip", 2.94);
}

#[test]
fn climate_wc1_fixture_caps_storm_duration_before_runtime_projection() {
    let climate = parse_climate_from_str(CLIMATE_WC1_STMDUR_CAP, ClimateParserMode::Strict)
        .expect("wc1 duration-cap fixture should parse");
    let assignments = BTreeMap::from([(9_u32, climate)]);
    let surface = build_watershed_runtime_surface_from_climate_assignments(&assignments, 0)
        .expect("watershed runtime surface should build from wc1 duration-cap fixture");
    assert_state_value(&surface.state_surface, "hs9_stmdur", 23.999 * 3_600.0);
    assert_state_value(&surface.state_surface, "hs9_ip", 22.589);
}

#[test]
fn management_fixture_projects_full_pl_runtime_surface_families() {
    let management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect("canonical 98.4 fixture should project PL runtime surfaces");
    assert_full_pl_family_coverage(&management, &pl_surfaces);

    let merged = build_hillslope_runtime_surface_from_management(&management)
        .expect("merged PL runtime surface should build");
    assert_merged_pl_seed_aliases(&merged.state_surface);
}

#[test]
fn management_rotation_fixture_projects_schedule_growth_and_decomp_runtime_surface_families() {
    let management = parse_management_fixture("canonical_rotation_nonzero_98_4.man");
    let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect("canonical rotation fixture should project PL runtime surfaces");
    assert_full_pl_family_coverage(&management, &pl_surfaces);

    assert_state_value(
        &pl_surfaces.pl_schedule_surface,
        "pl_schedule_nofe",
        usize_to_scalar(management.topology_count),
    );
    assert_state_value(
        &pl_surfaces.pl_schedule_surface,
        "pl_schedule_rotation_repeats",
        usize_to_scalar(management.schedule.rotation_repeats),
    );
    assert_state_value(
        &pl_surfaces.pl_schedule_surface,
        "pl_schedule_rotation_years",
        usize_to_scalar(management.schedule.rotation_years),
    );
    assert_state_value(
        &pl_surfaces.pl_schedule_surface,
        "pl_schedule_slot_count",
        usize_to_scalar(management.schedule.slots.len()),
    );
}

#[test]
fn management_runtime_surface_rejects_topology_count_mismatch_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    management.schedule.ofe_initial_refs.pop();

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("topology mismatch must fail with typed seam error");
    assert_eq!(error.code(), "HS-RUNTIME-E-036");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::ManagementTopologyCountMismatch {
            expected_ofes: 1,
            schedule_initial_refs: 0,
        }
    ));
}

#[test]
fn management_runtime_surface_rejects_slot_count_mismatch_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    management.schedule.slots.pop();

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("slot-count mismatch must fail with typed seam error");
    assert_eq!(error.code(), "HS-RUNTIME-E-037");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::ManagementScheduleSlotCountMismatch {
            expected_slots: 1,
            observed_slots: 0,
        }
    ));
}

#[test]
fn management_runtime_surface_rejects_slot_arity_mismatch_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    management.schedule.slots[0].crop_slots += 1;

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("slot arity mismatch must fail with typed seam error");
    assert_eq!(error.code(), "HS-RUNTIME-E-038");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::ManagementScheduleSlotArityMismatch {
            slot_index: 1,
            crop_slots: 2,
            yearly_refs: 1,
        }
    ));
}

#[test]
fn management_runtime_surface_rejects_schedule_ofe_index_out_of_range_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    management.schedule.slots[0].ofe_index = management.topology_count;

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("schedule OFE index overflow must fail with typed seam error");
    assert_eq!(error.code(), "HS-RUNTIME-E-045");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::ManagementScheduleOfeIndexOutOfRange {
            slot_index: 1,
            ofe_index: 2,
            max_ofe_index: 1,
        }
    ));
}

#[test]
fn management_runtime_surface_rejects_out_of_range_initial_reference_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    management.schedule.ofe_initial_refs[0] = 0;

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("zero initial reference must fail with typed seam error");
    assert_eq!(error.code(), "HS-RUNTIME-E-039");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::ManagementInitialReferenceOutOfRange {
            ofe_index: 1,
            initial_ref: 0,
            max_initial_ref: 1,
        }
    ));
}

#[test]
fn management_runtime_surface_rejects_out_of_range_yearly_reference_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    management.schedule.slots[0].yearly_refs[0] = 0;

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("zero yearly reference must fail with typed seam error");
    assert_eq!(error.code(), "HS-RUNTIME-E-040");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::ManagementYearlyReferenceOutOfRange {
            slot_index: 1,
            crop_slot_index: 1,
            yearly_ref: 0,
            max_yearly_ref: 1,
        }
    ));
}

#[test]
fn management_runtime_surface_rejects_unsupported_landuse_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    management.registries.initials[0].meta.landuse = 2;

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("unsupported PL landuse must fail with typed seam error");
    assert_eq!(error.code(), "HS-RUNTIME-E-041");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::UnsupportedPlLanduse {
            section: "initial",
            value: 2,
        }
    ));
}

#[test]
fn management_runtime_surface_rejects_non_finite_required_growth_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let yearly = &mut management.registries.yearlies[0];
    let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
    match &mut cropland.branch {
        YearlyCroplandBranch::AnnualOrFallow(annual) => annual.rw = f64::NAN,
        YearlyCroplandBranch::Perennial(_) => panic!("fixture should use annual branch"),
    }

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("non-finite required growth value must fail with typed seam error");
    assert_eq!(error.code(), "HS-RUNTIME-E-043");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::NonFinitePlProjectionField {
            field: "rw",
            slot_index: 1,
            crop_slot_index: 1,
            value,
        } if value.is_nan()
    ));
}

#[test]
fn management_runtime_surface_rejects_overflowed_projection_count() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    management.schedule.rotation_repeats = usize::MAX;
    management.schedule.rotation_years = 2;

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("overflowed projection count must fail with typed seam error");
    assert_eq!(error.code(), "HS-RUNTIME-E-044");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::PlProjectionCountOutOfRange {
            field: "schedule.expected_slots",
            value: usize::MAX,
        }
    ));
}

#[test]
fn management_runtime_surface_rejects_unsupported_perennial_option_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let yearly = &mut management.registries.yearlies[0];
    let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
    cropland.imngmt = 2;
    cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
        jdharv: 288,
        jdplt: 130,
        jdstop: 0,
        rw: 0.762,
        mgtopt: 4,
        cut_days: Vec::new(),
        grazing_cycles: Vec::new(),
    });

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("unsupported perennial mgtopt must fail with typed seam error");
    assert_eq!(error.code(), "HS-RUNTIME-E-042");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::UnsupportedPlManagementOption {
            field: "mgtopt",
            value: 4,
            allowed: "1..3",
        }
    ));
}

#[test]
#[ignore = "PL10b contract conformance gate; expected to fail until PL11 projection implementation"]
fn pl10b_contract_conformance_requires_annual_extension_projection_symbols() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let yearly = &mut management.registries.yearlies[0];
    let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
    cropland.imngmt = 1;
    match &mut cropland.branch {
        YearlyCroplandBranch::AnnualOrFallow(annual) => {
            annual.resmgt = 2;
            annual.extension = Some(YearlyAnnualExtension::Burn {
                jdburn: 250,
                fbmag: 0.30,
                fbrnog: 0.45,
            });
        }
        YearlyCroplandBranch::Perennial(_) => panic!("fixture should use annual branch"),
    }

    let surface = build_hillslope_runtime_surface_from_management(&management)
        .expect("PL runtime projection should build for annual branch");

    for symbol in [
        "jdherb", "jdburn", "jdslge", "jdcut", "jdmove", "fbrnag", "fbrnog", "frcut", "frmove",
    ] {
        assert_surface_has_symbol(&surface.state_surface, symbol);
    }
}

#[test]
#[ignore = "PL10b contract conformance gate; expected to fail until PL11 projection implementation"]
fn pl10b_contract_conformance_requires_perennial_cutday_indexed_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let yearly = &mut management.registries.yearlies[0];
    let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
    cropland.imngmt = 2;
    cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
        jdharv: 288,
        jdplt: 130,
        jdstop: 330,
        rw: 0.762,
        mgtopt: 1,
        cut_days: vec![180, 240],
        grazing_cycles: Vec::new(),
    });

    let surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect("PL runtime projection should build for perennial cut branch");

    assert_surface_has_symbol(
        &surfaces.pl_decomp_surface,
        "pl_decomp_slot_0001_crop_0001_cutday_0001",
    );
    assert_surface_has_symbol(
        &surfaces.pl_decomp_surface,
        "pl_decomp_slot_0001_crop_0001_cutday_0002",
    );
}

#[test]
#[ignore = "PL10b contract conformance gate; expected to fail until PL11 projection implementation"]
fn pl10b_contract_conformance_requires_perennial_grazing_cycle_payload_projection() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let yearly = &mut management.registries.yearlies[0];
    let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
    cropland.imngmt = 2;
    cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
        jdharv: 288,
        jdplt: 130,
        jdstop: 330,
        rw: 0.762,
        mgtopt: 2,
        cut_days: Vec::new(),
        grazing_cycles: vec![
            YearlyPerennialGrazingCycle {
                animal: 20.0,
                area: 1200.0,
                bodywt: 450.0,
                digest: 0.62,
                gday: 150,
                gend: 170,
            },
            YearlyPerennialGrazingCycle {
                animal: 18.0,
                area: 1150.0,
                bodywt: 430.0,
                digest: 0.60,
                gday: 200,
                gend: 220,
            },
        ],
    });

    let surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect("PL runtime projection should build for perennial grazing branch");

    for symbol in [
        "pl_decomp_slot_0001_crop_0001_gday_0001",
        "pl_decomp_slot_0001_crop_0001_gend_0001",
        "pl_decomp_slot_0001_crop_0001_animal_0001",
        "pl_decomp_slot_0001_crop_0001_bodywt_0001",
        "pl_decomp_slot_0001_crop_0001_area_0001",
        "pl_decomp_slot_0001_crop_0001_digest_0001",
        "pl_decomp_slot_0001_crop_0001_gday_0002",
        "pl_decomp_slot_0001_crop_0001_gend_0002",
        "pl_decomp_slot_0001_crop_0001_animal_0002",
        "pl_decomp_slot_0001_crop_0001_bodywt_0002",
        "pl_decomp_slot_0001_crop_0001_area_0002",
        "pl_decomp_slot_0001_crop_0001_digest_0002",
    ] {
        assert_surface_has_symbol(&surfaces.pl_decomp_surface, symbol);
    }
}

#[test]
#[ignore = "PL10b contract conformance gate; expected to fail until PL11 projection implementation"]
fn pl10b_contract_conformance_rejects_invalid_grazing_window_domain() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let yearly = &mut management.registries.yearlies[0];
    let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
    cropland.imngmt = 2;
    cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
        jdharv: 288,
        jdplt: 130,
        jdstop: 330,
        rw: 0.762,
        mgtopt: 2,
        cut_days: Vec::new(),
        grazing_cycles: vec![YearlyPerennialGrazingCycle {
            animal: 20.0,
            area: 1200.0,
            bodywt: 450.0,
            digest: 0.62,
            gday: 220,
            gend: 200,
        }],
    });

    build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("gday >= gend must fail conformance guard");
}

#[test]
#[ignore = "PL10b contract conformance gate; expected to fail until PL11 projection implementation"]
fn pl10b_contract_conformance_rejects_empty_perennial_grazing_cardinality() {
    let mut management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let yearly = &mut management.registries.yearlies[0];
    let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
    cropland.imngmt = 2;
    cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
        jdharv: 288,
        jdplt: 130,
        jdstop: 330,
        rw: 0.762,
        mgtopt: 2,
        cut_days: Vec::new(),
        grazing_cycles: Vec::new(),
    });

    build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("empty grazing cycle cardinality must fail conformance guard");
}

fn assert_state_value(
    surface: &std::collections::BTreeMap<BoundarySymbol, openwepp_kernel_contract::BoundaryValue>,
    symbol: &str,
    expected: f64,
) {
    let value = surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing runtime symbol {symbol}"))
        .as_f64();
    assert!(
        (value - expected).abs() < 1e-12,
        "{symbol} mismatch: {value}"
    );
}

fn assert_state_at_least(
    surface: &std::collections::BTreeMap<BoundarySymbol, openwepp_kernel_contract::BoundaryValue>,
    symbol: &str,
    minimum: f64,
) {
    let value = surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing runtime symbol {symbol}"))
        .as_f64();
    assert!(
        value >= minimum,
        "{symbol} expected >= {minimum}, got {value}"
    );
}

fn merge_hillslope_runtime_surfaces(
    mut primary: HillslopeWritebackSurface,
    overlay: HillslopeWritebackSurface,
) -> HillslopeWritebackSurface {
    primary.state_surface.extend(overlay.state_surface);
    primary.flux_surface.extend(overlay.flux_surface);
    primary
}

fn parse_management_fixture(name: &str) -> ManagementParseOutput {
    parse_management_from_path(management_fixture_path(name), ManagementParseMode::Strict)
        .unwrap_or_else(|error| panic!("management fixture {name} should parse: {error}"))
}

fn management_fixture_path(name: &str) -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("integration file parent exists")
        .parent()
        .expect("tests directory exists")
        .join("fixtures")
        .join("infile")
        .join("management")
        .join(name)
}

fn assert_full_pl_family_coverage(
    management: &ManagementParseOutput,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    assert_pl_ordering_flags(pl_surfaces);
    assert_pl_ofe_seed_coverage(management, pl_surfaces);
    assert_pl_slot_projection_coverage(management, pl_surfaces);
}

fn assert_pl_ordering_flags(pl_surfaces: &HillslopePlRuntimeSurfaces) {
    assert_state_value(
        &pl_surfaces.pl_schedule_surface,
        "pl_order_decomp_before_soil",
        1.0,
    );
    assert_state_value(
        &pl_surfaces.pl_schedule_surface,
        "pl_order_growth_after_decomp",
        1.0,
    );
    assert_state_value(
        &pl_surfaces.pl_schedule_surface,
        "pl_order_watbal_after_growth",
        1.0,
    );
}

fn assert_pl_ofe_seed_coverage(
    management: &ManagementParseOutput,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    for (ofe_position, initial_ref) in management.schedule.ofe_initial_refs.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_ofe{ofe_index}_initial_ref"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_ofe{ofe_index}_lanuse"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_growth_surface,
            &format!("pl_growth_ofe{ofe_index}_imngmt_seed"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_growth_surface,
            &format!("pl_growth_ofe{ofe_index}_rtyp_seed"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_decomp_surface,
            &format!("pl_decomp_ofe{ofe_index}_iresd_seed"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_decomp_surface,
            &format!("pl_decomp_ofe{ofe_index}_sumrtm_seed"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_decomp_surface,
            &format!("pl_decomp_ofe{ofe_index}_sumsrm_seed"),
        );

        let initial = &management.registries.initials[*initial_ref - 1];
        let openwepp_input_contract::parsers::management::InitialScenarioData::Cropland(data) =
            &initial.data;
        if data.understory_line.is_some() {
            assert_surface_has_symbol(
                &pl_surfaces.pl_decomp_surface,
                &format!("pl_decomp_ofe{ofe_index}_usinrcol_seed"),
            );
            assert_surface_has_symbol(
                &pl_surfaces.pl_decomp_surface,
                &format!("pl_decomp_ofe{ofe_index}_usrilcol_seed"),
            );
        }
    }
}

fn assert_pl_slot_projection_coverage(
    management: &ManagementParseOutput,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    for (slot_position, slot) in management.schedule.slots.iter().enumerate() {
        let slot_index = slot_position + 1;
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_slot_{slot_index:04}_rotation_index"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_slot_{slot_index:04}_year_in_rotation"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_slot_{slot_index:04}_ofe_index"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_slot_{slot_index:04}_crop_slots"),
        );

        for (crop_slot_position, yearly_ref) in slot.yearly_refs.iter().enumerate() {
            let crop_slot_index = crop_slot_position + 1;
            let yearly = &management.registries.yearlies[*yearly_ref - 1];
            let YearlyScenarioData::Cropland(cropland) = &yearly.data;
            assert_slot_crop_schedule_symbols(slot_index, crop_slot_index, pl_surfaces);
            assert_slot_crop_growth_common_symbols(slot_index, crop_slot_index, pl_surfaces);
            assert_slot_crop_branch_symbols(
                slot_index,
                crop_slot_index,
                &cropland.branch,
                pl_surfaces,
            );
        }
    }
}

fn assert_slot_crop_schedule_symbols(
    slot_index: usize,
    crop_slot_index: usize,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    for schedule_root in [
        "yearly_ref",
        "lanuse",
        "itype",
        "tilseq",
        "conset",
        "drset",
        "imngmt",
    ] {
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{schedule_root}"),
        );
    }
}

fn assert_slot_crop_growth_common_symbols(
    slot_index: usize,
    crop_slot_index: usize,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    for growth_root in ["itype", "imngmt"] {
        assert_surface_has_symbol(
            &pl_surfaces.pl_growth_surface,
            &format!("pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{growth_root}"),
        );
    }
}

fn assert_slot_crop_branch_symbols(
    slot_index: usize,
    crop_slot_index: usize,
    branch: &YearlyCroplandBranch,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    match branch {
        YearlyCroplandBranch::AnnualOrFallow(_) => {
            for growth_root in ["jdharv", "jdplt", "rw"] {
                assert_surface_has_symbol(
                    &pl_surfaces.pl_growth_surface,
                    &format!(
                        "pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{growth_root}"
                    ),
                );
            }
            assert_surface_has_symbol(
                &pl_surfaces.pl_decomp_surface,
                &format!("pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_resmgt"),
            );
        }
        YearlyCroplandBranch::Perennial(_) => {
            for growth_root in ["jdharv", "jdplt", "jdstop", "rw", "mgtopt"] {
                assert_surface_has_symbol(
                    &pl_surfaces.pl_growth_surface,
                    &format!(
                        "pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{growth_root}"
                    ),
                );
            }
            for decomp_root in ["mgtopt", "ncut", "ncycle"] {
                assert_surface_has_symbol(
                    &pl_surfaces.pl_decomp_surface,
                    &format!(
                        "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{decomp_root}"
                    ),
                );
            }
        }
    }
}

fn assert_surface_has_symbol(
    surface: &std::collections::BTreeMap<BoundarySymbol, openwepp_kernel_contract::BoundaryValue>,
    symbol: &str,
) {
    assert!(
        surface.contains_key(&BoundarySymbol::from(symbol)),
        "missing projected runtime symbol {symbol}"
    );
}

fn assert_merged_pl_seed_aliases(
    surface: &std::collections::BTreeMap<BoundarySymbol, openwepp_kernel_contract::BoundaryValue>,
) {
    for symbol in [
        "lanuse",
        "itype",
        "imngmt",
        "jdharv",
        "jdplt",
        "rw",
        "resmgt",
        "iresd_seed",
        "sumrtm_seed",
        "sumsrm_seed",
    ] {
        assert_surface_has_symbol(surface, symbol);
    }
}

fn usize_to_scalar(value: usize) -> f64 {
    let value_u32 = u32::try_from(value)
        .unwrap_or_else(|_| panic!("value {value} exceeds lossless u32->f64 conversion"));
    f64::from(value_u32)
}
