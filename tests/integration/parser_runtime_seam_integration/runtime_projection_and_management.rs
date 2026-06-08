use super::common::*;
use std::collections::{BTreeMap, BTreeSet};

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
fn parser_to_hillslope_runtime_surface_7778_measured_theta_fallback_closure() {
    let soil = parse_soil(SOIL_VALID_7778, SoilParserOptions::default())
        .expect("7778 soil fixture should parse for seam closure");
    let runtime_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("runtime surface should build with measured theta fallback");
    let raw_layer1 = soil
        .ofes
        .first()
        .and_then(|ofe| ofe.layers.first())
        .expect("7778 fixture should include layer 1");
    let raw_layer2 = soil
        .ofes
        .first()
        .and_then(|ofe| ofe.layers.get(1))
        .expect("7778 fixture should include layer 2");
    let raw_layer1_thetdr = raw_layer1
        .theta_r_rosetta
        .or(raw_layer1.wp_measured)
        .expect("layer 1 should include theta residual source");
    let raw_layer1_thetfc = raw_layer1
        .fc_rosetta
        .or(raw_layer1.fc_measured)
        .expect("layer 1 should include theta field-capacity source");
    let raw_layer2_thetdr = raw_layer2
        .theta_r_rosetta
        .or(raw_layer2.wp_measured)
        .expect("layer 2 should include theta residual source");
    let raw_layer2_thetfc = raw_layer2
        .fc_rosetta
        .or(raw_layer2.fc_measured)
        .expect("layer 2 should include theta field-capacity source");

    let thetdr = runtime_surface
        .state_surface
        .get(&BoundarySymbol::from("thetdr"))
        .expect("thetdr should be present")
        .as_f64();
    let thetfc = runtime_surface
        .state_surface
        .get(&BoundarySymbol::from("thetfc"))
        .expect("thetfc should be present")
        .as_f64();
    let layer2_thetdr = runtime_surface
        .state_surface
        .get(&BoundarySymbol::from("thetdr_0002"))
        .expect("thetdr_0002 should be present")
        .as_f64();
    let layer2_thetfc = runtime_surface
        .state_surface
        .get(&BoundarySymbol::from("thetfc_0002"))
        .expect("thetfc_0002 should be present")
        .as_f64();

    assert!(thetdr.is_finite() && thetdr > 0.0);
    assert!(thetfc.is_finite() && thetfc > 0.0);
    assert!(layer2_thetdr.is_finite() && layer2_thetdr > 0.0);
    assert!(layer2_thetfc.is_finite() && layer2_thetfc > 0.0);
    assert!(
        (thetdr - raw_layer1_thetdr).abs() > 1.0e-9
            || (thetfc - raw_layer1_thetfc).abs() > 1.0e-9
            || (layer2_thetdr - raw_layer2_thetdr).abs() > 1.0e-9
            || (layer2_thetfc - raw_layer2_thetfc).abs() > 1.0e-9,
        "7778 authoritative theta symbols should carry corrected lineage, not raw measured-theta values"
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
fn soil_runtime_surface_rejects_missing_normalized_corrected_lineage_input() {
    let mut soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    soil.ofes[0].layers[0].bulk_density_g_cm3 = None;

    let error = build_hillslope_runtime_surface_from_soil(&soil)
        .expect_err("missing normalized corrected-lineage input must fail with typed seam guard");
    assert_eq!(error.code(), "HS-RUNTIME-E-060");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::MissingCorrectedLayerNormalizationInput {
            ofe_index: 1,
            layer_index: 1,
            field: "bulk_density_g_cm3"
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

    assert!(
        report.scheduler_report.is_success(),
        "scheduler failed at {:?} with {:?}",
        report.scheduler_report.halted_phase,
        report
            .phase_reports
            .last()
            .map(|phase_report| phase_report.decision_status.message_id())
    );
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
fn slope_runtime_surface_compatibility_floor_accepts_non_positive_avgslp_projection() {
    let mut slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
        .expect("slope fixture should parse");
    for point in &mut slope.ofes[0].points {
        point.slpinp = 0.0;
    }

    let surface = build_hillslope_runtime_surface_from_slope_with_options(
        &slope,
        SlopeRuntimeSurfaceOptions::compatibility(),
    )
    .expect("compatibility floor should avoid non-positive avgslp hard-fail");

    let avgslp = surface
        .state_surface
        .get(&BoundarySymbol::from("avgslp"))
        .expect("avgslp should be present")
        .as_f64();
    let floor_applied = surface
        .state_surface
        .get(&BoundarySymbol::from("avgslp_floor_applied"))
        .expect("compatibility floor flag should be present for floored OFE")
        .as_f64();
    assert!((avgslp - 0.000_001).abs() < 1e-12);
    assert!((floor_applied - 1.0).abs() < 1e-12);
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
fn snow_parser_to_hillslope_runtime_surface_closure() {
    let snow = parse_snow_file(
        snow_fixture_path("strict_valid.txt"),
        SnowParseOptions::default(),
    )
    .expect("snow fixture should parse for seam closure");
    let runtime_surface = build_hillslope_runtime_surface_from_snow(&snow)
        .expect("snow runtime surface should build from parser output");

    assert_state_value(&runtime_surface.state_surface, "snow.options.rst", 0.0);
    assert_state_value(&runtime_surface.state_surface, "snow.options.newsnw", 100.0);
    assert_state_value(&runtime_surface.state_surface, "snow.options.ssd", 250.0);
    assert_state_value(
        &runtime_surface.state_surface,
        "snow.options.snow_file_present",
        1.0,
    );
    assert_state_value(&runtime_surface.state_surface, "snow.runtime_swe", 0.0);
    assert_state_value(&runtime_surface.state_surface, "snow.runtime_depth_m", 0.0);
    assert_state_value(
        &runtime_surface.state_surface,
        "snow.runtime_density_kg_m3",
        0.0,
    );
    assert_state_value(
        &runtime_surface.state_surface,
        "snow.runtime_settle_day_count",
        0.0,
    );
}

#[test]
fn snow_runtime_surface_projects_missing_file_flag_as_inactive() {
    let snow = parse_snow_file(
        snow_fixture_path("does_not_exist.txt"),
        SnowParseOptions::default(),
    )
    .expect("missing snow sidecar should parse via default branch");
    let runtime_surface = build_hillslope_runtime_surface_from_snow(&snow)
        .expect("snow runtime surface should build from default parser output");

    assert_state_value(
        &runtime_surface.state_surface,
        "snow.options.snow_file_present",
        0.0,
    );
}

#[test]
fn snow_runtime_surface_rejects_invalid_density_domain_projection() {
    let mut snow = parse_snow_file(
        snow_fixture_path("strict_valid.txt"),
        SnowParseOptions::default(),
    )
    .expect("snow fixture should parse");
    snow.newsnw = 300.0;

    let error = build_hillslope_runtime_surface_from_snow(&snow)
        .expect_err("new snow density above settled density must fail runtime seam");
    assert_eq!(error.code(), "HS-RUNTIME-E-053");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::SnowControlOutOfDomain {
            field: "snow.options.newsnw",
            ..
        }
    ));
}

#[test]
fn frost_parser_to_hillslope_runtime_surface_closure() {
    let frost = parse_frost_from_path(
        frost_fixture_path("strict_valid_two_line.txt"),
        FrostParseMode::Strict,
    )
    .expect("frost fixture should parse for seam closure");
    let runtime_surface = build_hillslope_runtime_surface_from_frost(&frost)
        .expect("frost runtime surface should build from parser output");

    assert_state_value(&runtime_surface.state_surface, "frost.options.wintRed", 1.0);
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.options.fineTop",
        10.0,
    );
    assert_state_value(&runtime_surface.state_surface, "frost.options.fineBot", 8.0);
    assert_state_value(&runtime_surface.state_surface, "frost.options.ksnowf", 0.1);
    assert_state_value(&runtime_surface.state_surface, "frost.options.kresf", 0.2);
    assert_state_value(&runtime_surface.state_surface, "frost.options.ksoilf", 0.3);
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.options.kfactor1",
        0.00001,
    );
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.options.kfactor2",
        0.00002,
    );
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.options.kfactor3",
        0.5,
    );
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.options.frost_file_present",
        1.0,
    );
    assert_state_value(&runtime_surface.state_surface, "frost.runtime_dfrost", 0.0);
    assert_state_value(&runtime_surface.state_surface, "frost.runtime_dthaw", 0.0);
    assert_state_value(&runtime_surface.state_surface, "frost.runtime_nft", 0.0);
    assert_state_value(&runtime_surface.state_surface, "frost.runtime_ws_frz", 0.0);
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.runtime_infcap_frz",
        0.0,
    );
    assert_state_value(&runtime_surface.state_surface, "frost.runtime_frdp_m", 0.0);
    assert_state_value(&runtime_surface.state_surface, "frost.runtime_thdp_m", 0.0);
    assert_state_value(&runtime_surface.state_surface, "frost.runtime_tfrdp_m", 0.0);
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.runtime_tthawd_m",
        0.0,
    );
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.runtime_fgthwd_flag",
        0.0,
    );
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.runtime_total_fine_layer_count",
        0.0,
    );
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.runtime_kftill_w_m_k",
        1.75,
    );
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.runtime_kfutil_w_m_k",
        2.1,
    );
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.runtime_kres_w_m_k",
        0.01,
    );
    assert_state_value(
        &runtime_surface.state_surface,
        "frost.runtime_residue_depth_m",
        0.0,
    );
}

#[test]
fn frost_runtime_surface_projects_missing_file_flag_as_inactive() {
    let frost = parse_frost_from_path(
        frost_fixture_path("does_not_exist_frost.txt"),
        FrostParseMode::Strict,
    )
    .expect("missing frost sidecar should parse via default branch");
    let runtime_surface = build_hillslope_runtime_surface_from_frost(&frost)
        .expect("frost runtime surface should build from default parser output");

    assert_state_value(
        &runtime_surface.state_surface,
        "frost.options.frost_file_present",
        0.0,
    );
}

#[test]
fn frost_runtime_surface_rejects_invalid_kfactor_domain_projection() {
    let mut frost = parse_frost_from_path(
        frost_fixture_path("strict_valid_two_line.txt"),
        FrostParseMode::Strict,
    )
    .expect("frost fixture should parse");
    frost.kfactor1 = 1.5;

    let error = build_hillslope_runtime_surface_from_frost(&frost)
        .expect_err("kfactor above unity must fail runtime seam");
    assert_eq!(error.code(), "HS-RUNTIME-E-055");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::FrostControlOutOfDomain {
            field: "frost.options.kfactor1",
            ..
        }
    ));
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
