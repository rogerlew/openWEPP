use super::common::*;

#[test]
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

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("gday >= gend must fail conformance guard");
    assert_eq!(error.code(), "HS-RUNTIME-E-049");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::PlGrazingWindowOutOfDomain {
            slot_index: 1,
            crop_slot_index: 1,
            cycle_index: 1,
            gday: 220,
            gend: 200,
        }
    ));
}

#[test]
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

    let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect_err("empty grazing cycle cardinality must fail conformance guard");
    assert_eq!(error.code(), "HS-RUNTIME-E-048");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::PlProjectionCardinalityInvalid {
            field: "ncycle",
            slot_index: 1,
            crop_slot_index: 1,
            value: 0,
            expected: ">=1 for mgtopt=2",
        }
    ));
}

#[test]
fn pl13_contract_conformance_scheduler_emits_annual_growth_transition_payload() {
    struct AnnualGrowthProbeKernel {
        saw_annual_payload: bool,
    }

    impl HillslopeKernel for AnnualGrowthProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if request.phase_class
                == openwepp_kernel_contract::HillslopeKernelPhaseClass::GrowthAnnualTransition
            {
                let context = request
                    .growth_context
                    .expect("annual growth phase should carry growth context");
                let payload = context
                    .transition_payload
                    .expect("annual growth context should carry transition payload");
                assert!(matches!(
                    payload.control,
                    HillslopeGrowthTransitionControl::Annual(control)
                        if control.active_action == HillslopeAnnualGrowthAction::HarvestReset
                ));
                assert!(payload.state_after.sumgdd.abs() <= f64::EPSILON);
                assert!(payload.state_after.vdmt.abs() <= f64::EPSILON);
                self.saw_annual_payload = true;
            }

            KernelRunResponse::new(
                SimulationStatus::ok(SimulationPhase::HillslopeKernel, "PL13-INTEGRATION-OK")
                    .expect("status should construct"),
                KernelWritebackPayload::empty(),
            )
        }
    }

    let management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let mut surface = build_hillslope_runtime_surface_from_management(&management)
        .expect("management runtime surface should build");
    seed_pl17_decomposition_symbols(&mut surface);

    let harvest_day = surface
        .state_surface
        .get(&BoundarySymbol::from(
            "pl_growth_slot_0001_crop_0001_jdharv",
        ))
        .expect("annual projection should include jdharv")
        .as_f64();
    surface
        .state_surface
        .insert(BoundarySymbol::from("day"), harvest_day.into());
    surface
        .state_surface
        .insert(BoundarySymbol::from("year"), 1.0.into());

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = AnnualGrowthProbeKernel {
        saw_annual_payload: false,
    };

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("scheduler should execute annual growth payload path");

    assert!(report.scheduler_report.is_success());
    assert!(kernel.saw_annual_payload);
}

#[test]
fn pl13_contract_conformance_scheduler_emits_perennial_growth_transition_payload() {
    struct PerennialGrowthProbeKernel {
        saw_perennial_payload: bool,
    }

    impl HillslopeKernel for PerennialGrowthProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if request.phase_class
                == openwepp_kernel_contract::HillslopeKernelPhaseClass::GrowthPerennialTransition
            {
                let context = request
                    .growth_context
                    .expect("perennial growth phase should carry growth context");
                let payload = context
                    .transition_payload
                    .expect("perennial growth context should carry transition payload");
                assert!(matches!(
                    payload.control,
                    HillslopeGrowthTransitionControl::Perennial(control)
                        if control.active_action == HillslopePerennialGrowthAction::StopReset
                ));
                assert!(payload.state_after.sumgdd.abs() <= f64::EPSILON);
                assert!(payload.state_after.vdmt.abs() <= f64::EPSILON);
                self.saw_perennial_payload = true;
            }

            KernelRunResponse::new(
                SimulationStatus::ok(SimulationPhase::HillslopeKernel, "PL13-INTEGRATION-OK")
                    .expect("status should construct"),
                KernelWritebackPayload::empty(),
            )
        }
    }

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
            gday: 150,
            gend: 200,
        }],
    });

    let mut surface = build_hillslope_runtime_surface_from_management(&management)
        .expect("management runtime surface should build for perennial branch");
    seed_pl17_decomposition_symbols(&mut surface);
    surface
        .state_surface
        .insert(BoundarySymbol::from("day"), 330.0.into());
    surface
        .state_surface
        .insert(BoundarySymbol::from("year"), 1.0.into());

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = PerennialGrowthProbeKernel {
        saw_perennial_payload: false,
    };

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("scheduler should execute perennial growth payload path");

    assert!(report.scheduler_report.is_success());
    assert!(kernel.saw_perennial_payload);
}

#[test]
fn pl16_contract_conformance_scheduler_emits_equation_updated_annual_growth_state_on_active_day() {
    struct AnnualGrowthEquationProbeKernel {
        saw_equation_update: bool,
    }

    impl HillslopeKernel for AnnualGrowthEquationProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if request.phase_class
                == openwepp_kernel_contract::HillslopeKernelPhaseClass::GrowthAnnualTransition
            {
                let context = request
                    .growth_context
                    .expect("annual growth phase should carry growth context");
                let payload = context
                    .transition_payload
                    .expect("annual growth context should carry transition payload");
                assert!(matches!(
                    payload.control,
                    HillslopeGrowthTransitionControl::Annual(control)
                        if control.active_action == HillslopeAnnualGrowthAction::None
                ));
                assert!(
                    payload.state_after.sumgdd > payload.state_before.sumgdd,
                    "active annual growth day must increase cumulative GDD"
                );
                assert!(
                    payload.state_after.vdmt > payload.state_before.vdmt,
                    "active annual growth day must increase biomass on equation path"
                );
                assert!(
                    payload.state_after.cancov > payload.state_before.cancov,
                    "active annual growth day must update canopy cover on equation path"
                );
                self.saw_equation_update = true;
            }

            KernelRunResponse::new(
                SimulationStatus::ok(SimulationPhase::HillslopeKernel, "PL16-INTEGRATION-OK")
                    .expect("status should construct"),
                KernelWritebackPayload::empty(),
            )
        }
    }

    let management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let surface = build_hillslope_runtime_surface_from_management(&management)
        .expect("management runtime surface should build");
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    let soil_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("soil runtime surface should build");
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse");
    let climate_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("climate runtime surface should build");
    let mut surface = merge_hillslope_runtime_surfaces(
        merge_hillslope_runtime_surfaces(surface, soil_surface),
        climate_surface,
    );
    seed_pl16_equation_symbols(&mut surface, Pl16EquationSeed { ws: 0.85 });
    surface
        .state_surface
        .insert(BoundarySymbol::from("day"), 200.0.into());
    surface
        .state_surface
        .insert(BoundarySymbol::from("year"), 1.0.into());

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = AnnualGrowthEquationProbeKernel {
        saw_equation_update: false,
    };

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("scheduler should execute annual growth equation path");

    assert!(report.scheduler_report.is_success());
    assert!(kernel.saw_equation_update);
}

#[test]
fn pl16_contract_conformance_scheduler_emits_equation_updated_perennial_growth_state_on_active_day()
{
    struct PerennialGrowthEquationProbeKernel {
        saw_equation_update: bool,
    }

    impl HillslopeKernel for PerennialGrowthEquationProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if request.phase_class
                == openwepp_kernel_contract::HillslopeKernelPhaseClass::GrowthPerennialTransition
            {
                let context = request
                    .growth_context
                    .expect("perennial growth phase should carry growth context");
                let payload = context
                    .transition_payload
                    .expect("perennial growth context should carry transition payload");
                assert!(matches!(
                    payload.control,
                    HillslopeGrowthTransitionControl::Perennial(control)
                        if control.active_action == HillslopePerennialGrowthAction::None
                ));
                assert!(
                    payload.state_after.sumgdd > payload.state_before.sumgdd,
                    "active perennial growth day must increase cumulative GDD"
                );
                assert!(
                    payload.state_after.vdmt > payload.state_before.vdmt,
                    "active perennial growth day must increase biomass on equation path"
                );
                assert!(
                    payload.state_after.rtd >= payload.state_before.rtd,
                    "active perennial growth day root depth should be non-decreasing"
                );
                self.saw_equation_update = true;
            }

            KernelRunResponse::new(
                SimulationStatus::ok(SimulationPhase::HillslopeKernel, "PL16-INTEGRATION-OK")
                    .expect("status should construct"),
                KernelWritebackPayload::empty(),
            )
        }
    }

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
            gday: 150,
            gend: 200,
        }],
    });

    let surface = build_hillslope_runtime_surface_from_management(&management)
        .expect("management runtime surface should build for perennial branch");
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    let soil_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("soil runtime surface should build");
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse");
    let climate_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("climate runtime surface should build");
    let mut surface = merge_hillslope_runtime_surfaces(
        merge_hillslope_runtime_surfaces(surface, soil_surface),
        climate_surface,
    );
    seed_pl16_equation_symbols(&mut surface, Pl16EquationSeed { ws: 0.8 });
    surface
        .state_surface
        .insert(BoundarySymbol::from("day"), 220.0.into());
    surface
        .state_surface
        .insert(BoundarySymbol::from("year"), 1.0.into());

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = PerennialGrowthEquationProbeKernel {
        saw_equation_update: false,
    };

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("scheduler should execute perennial growth equation path");

    assert!(report.scheduler_report.is_success());
    assert!(kernel.saw_equation_update);
}

#[test]
fn pl16_contract_conformance_rejects_missing_growth_equation_symbol() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            KernelRunResponse::new(
                SimulationStatus::ok(SimulationPhase::HillslopeKernel, "PL16-NOOP-OK")
                    .expect("status should construct"),
                KernelWritebackPayload::empty(),
            )
        }
    }

    let management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let surface = build_hillslope_runtime_surface_from_management(&management)
        .expect("management runtime surface should build");
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    let soil_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("soil runtime surface should build");
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse");
    let climate_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("climate runtime surface should build");
    let mut surface = merge_hillslope_runtime_surfaces(
        merge_hillslope_runtime_surfaces(surface, soil_surface),
        climate_surface,
    );
    seed_pl16_equation_symbols(&mut surface, Pl16EquationSeed { ws: 0.7 });
    surface
        .state_surface
        .remove(&BoundarySymbol::from("pl_growth_slot_0001_crop_0001_btemp"));
    surface
        .state_surface
        .insert(BoundarySymbol::from("day"), 200.0.into());
    surface
        .state_surface
        .insert(BoundarySymbol::from("year"), 1.0.into());

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing growth equation symbol should return typed failure");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(openwepp_hillslope_orchestrator::HillslopePhase::AnnualGrowthTransition)
    );
    assert_eq!(
        report.phase_reports[4].decision_status.message_id(),
        "HS-GROWTH-E-001"
    );
}

#[test]
fn pl17_contract_conformance_requires_decomposition_rate_projection_symbols() {
    let management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
        .expect("management runtime surfaces should build");

    assert_surface_has_symbol(
        &pl_surfaces.pl_decomp_surface,
        "pl_decomp_slot_0001_crop_0001_oratea",
    );
    assert_surface_has_symbol(
        &pl_surfaces.pl_decomp_surface,
        "pl_decomp_slot_0001_crop_0001_orater",
    );
    assert_surface_has_symbol(&pl_surfaces.pl_decomp_surface, "oratea");
    assert_surface_has_symbol(&pl_surfaces.pl_decomp_surface, "orater");
}

#[test]
fn pl17_contract_conformance_scheduler_emits_equation_updated_annual_decomposition_state_on_active_day()
 {
    struct AnnualDecompEquationProbeKernel {
        saw_equation_update: bool,
        before_sumrtm: f64,
        before_sumsrm: f64,
    }

    impl HillslopeKernel for AnnualDecompEquationProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if request.phase_class
                == openwepp_kernel_contract::HillslopeKernelPhaseClass::DecompositionTransition
            {
                let context = request
                    .decomposition_context
                    .expect("decomposition phase should carry decomposition context");
                let payload = context
                    .transition_payload
                    .expect("decomposition context should carry transition payload");
                assert!(matches!(
                    payload.control,
                    HillslopeDecompositionTransitionControl::Annual(control)
                        if control.active_action == HillslopeAnnualDecompositionAction::None
                ));
                assert!(
                    payload.sumrtm_seed < self.before_sumrtm,
                    "active annual decomposition day must decrease dead-root residue mass"
                );
                assert!(
                    payload.sumsrm_seed < self.before_sumsrm,
                    "active annual decomposition day must decrease submerged residue mass"
                );
                self.saw_equation_update = true;
            }

            KernelRunResponse::new(
                SimulationStatus::ok(SimulationPhase::HillslopeKernel, "PL17-INTEGRATION-OK")
                    .expect("status should construct"),
                KernelWritebackPayload::empty(),
            )
        }
    }

    let management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let surface = build_hillslope_runtime_surface_from_management(&management)
        .expect("management runtime surface should build");
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    let soil_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("soil runtime surface should build");
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse");
    let climate_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("climate runtime surface should build");
    let mut surface = merge_hillslope_runtime_surfaces(
        merge_hillslope_runtime_surfaces(surface, soil_surface),
        climate_surface,
    );
    seed_pl16_equation_symbols(&mut surface, Pl16EquationSeed { ws: 0.8 });
    seed_pl17_decomposition_symbols(&mut surface);
    surface
        .state_surface
        .insert(BoundarySymbol::from("day"), 200.0.into());
    surface
        .state_surface
        .insert(BoundarySymbol::from("year"), 1.0.into());

    let before_sumrtm = surface
        .state_surface
        .get(&BoundarySymbol::from("sumrtm_seed"))
        .expect("sumrtm_seed should be present")
        .as_f64();
    let before_sumsrm = surface
        .state_surface
        .get(&BoundarySymbol::from("sumsrm_seed"))
        .expect("sumsrm_seed should be present")
        .as_f64();

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = AnnualDecompEquationProbeKernel {
        saw_equation_update: false,
        before_sumrtm,
        before_sumsrm,
    };

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("scheduler should execute annual decomposition equation path");

    assert!(report.scheduler_report.is_success());
    assert!(kernel.saw_equation_update);
}

#[test]
fn pl17_contract_conformance_scheduler_preserves_seed_masses_when_decomposition_constants_are_zero()
{
    struct AnnualDecompZeroRateProbeKernel {
        saw_equation_update: bool,
        before_sumrtm: f64,
        before_sumsrm: f64,
    }

    impl HillslopeKernel for AnnualDecompZeroRateProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if request.phase_class
                == openwepp_kernel_contract::HillslopeKernelPhaseClass::DecompositionTransition
            {
                let context = request
                    .decomposition_context
                    .expect("decomposition phase should carry decomposition context");
                let payload = context
                    .transition_payload
                    .expect("decomposition context should carry transition payload");
                assert!(matches!(
                    payload.control,
                    HillslopeDecompositionTransitionControl::Annual(control)
                        if control.active_action == HillslopeAnnualDecompositionAction::None
                ));
                assert!(
                    (payload.sumrtm_seed - self.before_sumrtm).abs() < 1e-12,
                    "zero decomposition constants should preserve dead-root residue mass"
                );
                assert!(
                    (payload.sumsrm_seed - self.before_sumsrm).abs() < 1e-12,
                    "zero decomposition constants should preserve submerged residue mass"
                );
                self.saw_equation_update = true;
            }

            KernelRunResponse::new(
                SimulationStatus::ok(SimulationPhase::HillslopeKernel, "PL17-INTEGRATION-OK")
                    .expect("status should construct"),
                KernelWritebackPayload::empty(),
            )
        }
    }

    let management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let surface = build_hillslope_runtime_surface_from_management(&management)
        .expect("management runtime surface should build");
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    let soil_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("soil runtime surface should build");
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse");
    let climate_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("climate runtime surface should build");
    let mut surface = merge_hillslope_runtime_surfaces(
        merge_hillslope_runtime_surfaces(surface, soil_surface),
        climate_surface,
    );
    seed_pl16_equation_symbols(&mut surface, Pl16EquationSeed { ws: 0.8 });
    seed_pl17_decomposition_symbols(&mut surface);
    for symbol in [
        "pl_decomp_slot_0001_crop_0001_oratea",
        "pl_decomp_slot_0001_crop_0001_orater",
        "oratea",
        "orater",
    ] {
        surface
            .state_surface
            .insert(BoundarySymbol::from(symbol), 0.0.into());
    }
    surface
        .state_surface
        .insert(BoundarySymbol::from("day"), 200.0.into());
    surface
        .state_surface
        .insert(BoundarySymbol::from("year"), 1.0.into());

    let before_sumrtm = surface
        .state_surface
        .get(&BoundarySymbol::from("sumrtm_seed"))
        .expect("sumrtm_seed should be present")
        .as_f64();
    let before_sumsrm = surface
        .state_surface
        .get(&BoundarySymbol::from("sumsrm_seed"))
        .expect("sumsrm_seed should be present")
        .as_f64();

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = AnnualDecompZeroRateProbeKernel {
        saw_equation_update: false,
        before_sumrtm,
        before_sumsrm,
    };

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("scheduler should execute annual decomposition equation path");

    assert!(report.scheduler_report.is_success());
    assert!(kernel.saw_equation_update);
}

#[test]
#[allow(clippy::too_many_lines)]
fn pl17_contract_conformance_scheduler_emits_equation_updated_perennial_decomposition_state_on_active_day()
 {
    struct PerennialDecompEquationProbeKernel {
        saw_equation_update: bool,
        before_sumrtm: f64,
        before_sumsrm: f64,
    }

    impl HillslopeKernel for PerennialDecompEquationProbeKernel {
        fn run_hillslope_phase(
            &mut self,
            request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            if request.phase_class
                == openwepp_kernel_contract::HillslopeKernelPhaseClass::DecompositionTransition
            {
                let context = request
                    .decomposition_context
                    .expect("decomposition phase should carry decomposition context");
                let payload = context
                    .transition_payload
                    .expect("decomposition context should carry transition payload");
                assert!(matches!(
                    payload.control,
                    HillslopeDecompositionTransitionControl::Perennial(control)
                        if control.active_action
                            == HillslopePerennialDecompositionAction::Grazing { cycle_index: 1 }
                ));
                assert!(
                    payload.sumrtm_seed < self.before_sumrtm,
                    "active perennial decomposition day must decrease dead-root residue mass"
                );
                assert!(
                    payload.sumsrm_seed < self.before_sumsrm,
                    "active perennial decomposition day must decrease submerged residue mass"
                );
                self.saw_equation_update = true;
            }

            KernelRunResponse::new(
                SimulationStatus::ok(SimulationPhase::HillslopeKernel, "PL17-INTEGRATION-OK")
                    .expect("status should construct"),
                KernelWritebackPayload::empty(),
            )
        }
    }

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
            gday: 150,
            gend: 200,
        }],
    });

    let surface = build_hillslope_runtime_surface_from_management(&management)
        .expect("management runtime surface should build for perennial branch");
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    let soil_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("soil runtime surface should build");
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse");
    let climate_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("climate runtime surface should build");
    let mut surface = merge_hillslope_runtime_surfaces(
        merge_hillslope_runtime_surfaces(surface, soil_surface),
        climate_surface,
    );
    seed_pl17_decomposition_symbols(&mut surface);
    surface
        .state_surface
        .insert(BoundarySymbol::from("day"), 180.0.into());
    surface
        .state_surface
        .insert(BoundarySymbol::from("year"), 1.0.into());

    let before_sumrtm = surface
        .state_surface
        .get(&BoundarySymbol::from("sumrtm_seed"))
        .expect("sumrtm_seed should be present")
        .as_f64();
    let before_sumsrm = surface
        .state_surface
        .get(&BoundarySymbol::from("sumsrm_seed"))
        .expect("sumsrm_seed should be present")
        .as_f64();

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = PerennialDecompEquationProbeKernel {
        saw_equation_update: false,
        before_sumrtm,
        before_sumsrm,
    };

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("scheduler should execute perennial decomposition equation path");

    assert!(kernel.saw_equation_update);
    if let Some(halted_phase) = report.scheduler_report.halted_phase {
        assert!(
            halted_phase.rank()
                >= openwepp_hillslope_orchestrator::HillslopePhase::PerennialGrowthTransition
                    .rank(),
            "unexpected halt before perennial growth transition: {halted_phase:?}"
        );
    }
}

#[test]
fn pl17_contract_conformance_rejects_missing_decomposition_equation_symbol() {
    #[derive(Default)]
    struct NoopKernel;

    impl HillslopeKernel for NoopKernel {
        fn run_hillslope_phase(
            &mut self,
            _request: &HillslopeKernelRequest<'_>,
        ) -> KernelRunResponse {
            KernelRunResponse::new(
                SimulationStatus::ok(SimulationPhase::HillslopeKernel, "PL17-NOOP-OK")
                    .expect("status should construct"),
                KernelWritebackPayload::empty(),
            )
        }
    }

    let management = parse_management_fixture("canonical_cropland_nonzero_98_4.man");
    let surface = build_hillslope_runtime_surface_from_management(&management)
        .expect("management runtime surface should build");
    let soil = parse_soil(SOIL_VALID_9002, SoilParserOptions::default())
        .expect("soil fixture should parse");
    let soil_surface = build_hillslope_runtime_surface_from_soil(&soil)
        .expect("soil runtime surface should build");
    let climate = parse_climate_from_str(CLIMATE_STRICT_VALID, ClimateParserMode::Strict)
        .expect("climate fixture should parse");
    let climate_surface = build_hillslope_runtime_surface_from_climate(&climate, 0)
        .expect("climate runtime surface should build");
    let mut surface = merge_hillslope_runtime_surfaces(
        merge_hillslope_runtime_surfaces(surface, soil_surface),
        climate_surface,
    );
    seed_pl17_decomposition_symbols(&mut surface);
    surface.state_surface.remove(&BoundarySymbol::from(
        "pl_decomp_slot_0001_crop_0001_oratea",
    ));
    surface
        .state_surface
        .insert(BoundarySymbol::from("day"), 200.0.into());
    surface
        .state_surface
        .insert(BoundarySymbol::from("year"), 1.0.into());

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("topology should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = NoopKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing decomposition equation symbol should return typed failure");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(openwepp_hillslope_orchestrator::HillslopePhase::DecompositionTransition)
    );
    assert_eq!(
        report.phase_reports[2].decision_status.message_id(),
        "HS-DECOMP-E-001"
    );
}
