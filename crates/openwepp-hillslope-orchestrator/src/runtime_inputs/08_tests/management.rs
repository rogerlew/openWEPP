    #[test]
    #[allow(clippy::too_many_lines)]
    fn management_runtime_surfaces_project_required_pl_controls_and_seeds() {
        let management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("pl runtime surface projection should succeed");
        let merged_surface = build_hillslope_runtime_surface_from_management(&management)
            .expect("merged pl runtime state surface should build");

        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("lanuse")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("itype")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("pl_order_decomp_before_soil")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("jdharv")),
            Some(&BoundaryValue::scalar(288.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("jdplt")),
            Some(&BoundaryValue::scalar(130.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("rw")),
            Some(&BoundaryValue::scalar(0.762))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("resmgt")),
            Some(&BoundaryValue::scalar(6.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("sumrtm_seed")),
            Some(&BoundaryValue::scalar(0.50003))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("inrcov")),
            Some(&BoundaryValue::scalar(0.9))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("rilcov")),
            Some(&BoundaryValue::scalar(0.9))
        );
        assert_eq!(
            merged_surface.state_surface.get(&BoundarySymbol::from("rrinit")),
            Some(&BoundaryValue::scalar(0.02))
        );
        assert_eq!(
            merged_surface.state_surface.get(&BoundarySymbol::from("rspace")),
            Some(&BoundaryValue::scalar(0.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("width")),
            Some(&BoundaryValue::scalar(0.0254))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("flivmx_seed")),
            Some(&BoundaryValue::scalar(0.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("hmax_seed")),
            Some(&BoundaryValue::scalar(2.60099))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("sumsrm_seed")),
            Some(&BoundaryValue::scalar(0.19997))
        );
        let expected_residue_depth_m =
            ((1.0_f64 - 0.9).ln() / -2.3) / (0.174 * 100.0);
        let residue_depth_m = merged_surface
            .state_surface
            .get(&BoundarySymbol::from("frost.runtime_residue_depth_m"))
            .expect("management projection must publish legacy resdep lineage")
            .as_f64();
        assert!(
            (residue_depth_m - expected_residue_depth_m).abs() <= 1.0e-12,
            "management projection must derive frost residue depth from legacy init1/res_dp lineage; actual={residue_depth_m}, expected={expected_residue_depth_m}"
        );
        assert_eq!(
            merged_surface.state_surface.get(&BoundarySymbol::from("drset")),
            Some(&BoundaryValue::scalar(0.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("wb19_drain_enabled")),
            Some(&BoundaryValue::scalar(0.0))
        );
        assert!(
            !merged_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("wb19_drain_depth")),
            "wb19_drain_depth should not be projected when drain is disabled"
        );
        assert!(
            !merged_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("wb19_drain_spacing")),
            "wb19_drain_spacing should not be projected when drain is disabled"
        );
        assert!(
            !merged_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("wb19_drain_diameter")),
            "wb19_drain_diameter should not be projected when drain is disabled"
        );

        assert_eq!(
            pl_surfaces.pl_schedule_surface.get(&BoundarySymbol::from(
                "pl_schedule_slot_0001_crop_0001_itype"
            )),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            pl_surfaces
                .pl_growth_surface
                .get(&BoundarySymbol::from("pl_growth_slot_0001_crop_0001_rw")),
            Some(&BoundaryValue::scalar(0.762))
        );
        assert_eq!(
            pl_surfaces.pl_decomp_surface.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_resmgt"
            )),
            Some(&BoundaryValue::scalar(6.0))
        );
        assert_eq!(
            pl_surfaces
                .pl_growth_surface
                .get(&BoundarySymbol::from("ofe1_inrcov")),
            Some(&BoundaryValue::scalar(0.9))
        );
        assert_eq!(
            pl_surfaces
                .pl_growth_surface
                .get(&BoundarySymbol::from("ofe1_rspace")),
            Some(&BoundaryValue::scalar(0.0))
        );
        assert_eq!(
            pl_surfaces
                .pl_growth_surface
                .get(&BoundarySymbol::from("pl_growth_ofe1_bbb_seed")),
            Some(&BoundaryValue::scalar(3.0))
        );
    }

    #[test]
    fn management_runtime_projection_preserves_wepppy_corn_no_till_growth_coefficients() {
        let management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("WEPPpy corn-no till management fixture should parse");
        let PlantScenarioData::Cropland(plant) = &management.registries.plants[0].data;

        assert!((plant.canopy_line[2] - 35.00196).abs() <= 1.0e-12);
        assert!((plant.growth_line[5] - 1700.0).abs() <= 1.0e-12);
        assert!((plant.growth_line[7] - 2.60099).abs() <= 1.0e-12);
        assert!((plant.residue_line[5] - 1.51995).abs() <= 1.0e-12);
        assert!((plant.terminal_line[1] - 3.5).abs() <= 1.0e-12);

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("WEPPpy corn-no till growth coefficients should project");
        let growth = &pl_surfaces.pl_growth_surface;

        assert_eq!(
            growth.get(&BoundarySymbol::from(
                "pl_growth_slot_0001_crop_0001_beinp"
            )),
            Some(&BoundaryValue::scalar(35.00196))
        );
        assert_eq!(
            growth.get(&BoundarySymbol::from(
                "pl_growth_slot_0001_crop_0001_gddmax"
            )),
            Some(&BoundaryValue::scalar(1700.0))
        );
        assert_eq!(
            growth.get(&BoundarySymbol::from(
                "pl_growth_slot_0001_crop_0001_xmxlai"
            )),
            Some(&BoundaryValue::scalar(3.5))
        );
        assert_eq!(
            growth.get(&BoundarySymbol::from(
                "pl_growth_slot_0001_crop_0001_rdmax"
            )),
            Some(&BoundaryValue::scalar(1.51995))
        );
        assert_eq!(
            growth.get(&BoundarySymbol::from("beinp")),
            Some(&BoundaryValue::scalar(35.00196))
        );
        assert_eq!(
            growth.get(&BoundarySymbol::from("gddmax")),
            Some(&BoundaryValue::scalar(1700.0))
        );
        assert_eq!(
            growth.get(&BoundarySymbol::from("xmxlai")),
            Some(&BoundaryValue::scalar(3.5))
        );
        assert_eq!(
            growth.get(&BoundarySymbol::from("rdmax")),
            Some(&BoundaryValue::scalar(1.51995))
        );
    }

    #[test]
    fn management_runtime_projection_projects_initial_understory_seed_symbols() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let initial = &mut management.registries.initials[0];
        let InitialScenarioData::Cropland(initial_data) = &mut initial.data;
        initial_data.understory_line = Some([0.35, 0.45]);

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("finite understory cover should project");

        assert_eq!(
            pl_surfaces
                .pl_decomp_surface
                .get(&BoundarySymbol::from("pl_decomp_ofe1_usinrcol_seed")),
            Some(&BoundaryValue::scalar(0.35))
        );
        assert_eq!(
            pl_surfaces
                .pl_decomp_surface
                .get(&BoundarySymbol::from("pl_decomp_ofe1_usrilcol_seed")),
            Some(&BoundaryValue::scalar(0.45))
        );
    }

    #[test]
    fn management_runtime_projection_rejects_non_finite_initial_seed_fields() {
        for field in [
            "sumrtm_seed",
            "sumsrm_seed",
            "usinrcol_seed",
            "usrilcol_seed",
        ] {
            let mut management = parse_management_from_str(
                MANAGEMENT_CANONICAL_NONZERO_98_4,
                ManagementParseMode::Strict,
            )
            .expect("management fixture should parse");
            let initial = &mut management.registries.initials[0];
            let InitialScenarioData::Cropland(initial_data) = &mut initial.data;
            match field {
                "sumrtm_seed" => initial_data.terminal_line[0] = f64::NAN,
                "sumsrm_seed" => initial_data.terminal_line[1] = f64::NAN,
                "usinrcol_seed" => initial_data.understory_line = Some([f64::NAN, 0.2]),
                "usrilcol_seed" => initial_data.understory_line = Some([0.2, f64::NAN]),
                _ => unreachable!("test field set is exhaustive"),
            }

            let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
                .expect_err("non-finite initial seed field must fail runtime seam");
            assert_eq!(error.code(), "HS-RUNTIME-E-043");
            assert!(matches!(
                error,
                HillslopeRuntimeInputError::NonFinitePlProjectionField {
                    field: observed,
                    slot_index: 0,
                    crop_slot_index: 0,
                    value,
                } if observed == field && value.is_nan()
            ));
        }
    }

    #[test]
    fn management_runtime_projection_rejects_out_of_range_initial_residue_plant_reference() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let initial = &mut management.registries.initials[0];
        let InitialScenarioData::Cropland(initial_data) = &mut initial.data;
        initial_data.iresd = 0;

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("iresd seed reference must stay in plant registry range");
        assert_eq!(error.code(), "HS-RUNTIME-E-050");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "iresd_seed",
                slot_index: 0,
                crop_slot_index: 0,
                value,
                allowed: "1..=plant_scenario_count",
            } if value == 0.0
        ));
    }

    #[test]
    fn management_runtime_projection_projects_primary_annual_extension_aliases() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.branch = YearlyCroplandBranch::AnnualOrFallow(YearlyAnnualFallowData {
            jdharv: 288,
            jdplt: 130,
            rw: 0.762,
            resmgt: 2,
            extension: Some(YearlyAnnualExtension::Burn {
                jdburn: 140,
                fbmag: 0.25,
                fbrnog: 0.75,
            }),
        });

        let merged_surface = build_hillslope_runtime_surface_from_management(&management)
            .expect("burn extension should project slot and primary aliases");
        let state = &merged_surface.state_surface;

        assert_eq!(
            state.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_jdburn"
            )),
            Some(&BoundaryValue::scalar(140.0))
        );
        assert_eq!(
            state.get(&BoundarySymbol::from("jdburn")),
            Some(&BoundaryValue::scalar(140.0))
        );
        assert_eq!(
            state.get(&BoundarySymbol::from("fbrnag")),
            Some(&BoundaryValue::scalar(0.25))
        );
        assert_eq!(
            state.get(&BoundarySymbol::from("fbrnog")),
            Some(&BoundaryValue::scalar(0.75))
        );
    }

    #[test]
    fn management_runtime_projection_rejects_yearly_landuse_and_plant_reference_domain() {
        let mut yearly_landuse = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        yearly_landuse.registries.yearlies[0].meta.landuse = 2;
        let error = build_hillslope_pl_runtime_surfaces_from_management(&yearly_landuse)
            .expect_err("unsupported yearly landuse must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-041");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::UnsupportedPlLanduse {
                section: "yearly",
                value: 2,
            }
        ));

        let mut itype_zero = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let YearlyScenarioData::Cropland(cropland) = &mut itype_zero.registries.yearlies[0].data;
        cropland.itype = 0;
        let error = build_hillslope_pl_runtime_surfaces_from_management(&itype_zero)
            .expect_err("itype must reference a registered plant");
        assert_eq!(error.code(), "HS-RUNTIME-E-050");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "itype",
                slot_index: 1,
                crop_slot_index: 1,
                value,
                allowed: "1..=plant_scenario_count",
            } if value == 0.0
        ));
    }

    #[test]
    fn hphys0251_management_projection_preserves_crop_pltol() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("WEPPpy corn-no till management fixture should parse");
        let plant = &mut management.registries.plants[0];
        let PlantScenarioData::Cropland(plant_data) = &mut plant.data;
        plant_data.residue_line[3] = 0.37;

        let merged_surface = build_hillslope_runtime_surface_from_management(&management)
            .expect("crop-specific pltol should project into runtime state");
        let state = &merged_surface.state_surface;

        assert_eq!(
            state.get(&BoundarySymbol::from(
                "pl_growth_slot_0001_crop_0001_pltol"
            )),
            Some(&BoundaryValue::scalar(0.37))
        );
        assert_eq!(
            state.get(&BoundarySymbol::from("pltol")),
            Some(&BoundaryValue::scalar(0.37))
        );
    }

    #[test]
    fn management_runtime_projection_rejects_out_of_range_initial_reference() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        management.schedule.ofe_initial_refs[0] = 0;

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("out-of-range initial reference must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-039");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::ManagementInitialReferenceOutOfRange {
                ofe_index: 1,
                initial_ref: 0,
                max_initial_ref: 1
            }
        ));
    }

    #[test]
    fn management_runtime_projection_rejects_unsupported_pl_landuse() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        management.registries.initials[0].meta.landuse = 2;

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("unsupported landuse must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-041");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::UnsupportedPlLanduse {
                section: "initial",
                value: 2
            }
        ));
    }

    #[test]
    fn management_runtime_projection_projects_wb19_drain_controls_from_primary_slot() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");

        management.registries.drains.push(DrainScenario {
            meta: ScenarioMeta {
                name: "test-drain".to_string(),
                description: [String::new(), String::new(), String::new()],
                landuse: 1,
            },
            ddrain: 0.65,
            drainc: 0.0,
            drdiam: 0.08,
            sdrain: 15.0,
        });
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.drset = 1;

        let merged_surface = build_hillslope_runtime_surface_from_management(&management)
            .expect("valid drain scenario should project WB19 drain controls");

        assert_eq!(
            merged_surface.state_surface.get(&BoundarySymbol::from("drset")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("wb19_drain_enabled")),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("wb19_drain_depth")),
            Some(&BoundaryValue::scalar(0.65))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("wb19_drain_spacing")),
            Some(&BoundaryValue::scalar(15.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("wb19_drain_diameter")),
            Some(&BoundaryValue::scalar(0.08))
        );
    }

    #[test]
    fn management_runtime_projection_rejects_non_positive_primary_drain_geometry() {
        for (field, ddrain, sdrain, drdiam) in [
            ("wb19_drain_depth", 0.0, 15.0, 0.08),
            ("wb19_drain_spacing", 0.65, 0.0, 0.08),
            ("wb19_drain_diameter", 0.65, 15.0, 0.0),
        ] {
            let mut management = parse_management_from_str(
                MANAGEMENT_CANONICAL_NONZERO_98_4,
                ManagementParseMode::Strict,
            )
            .expect("management fixture should parse");
            management.registries.drains.push(DrainScenario {
                meta: ScenarioMeta {
                    name: format!("invalid-{field}"),
                    description: [String::new(), String::new(), String::new()],
                    landuse: 1,
                },
                ddrain,
                drainc: 0.0,
                drdiam,
                sdrain,
            });
            let yearly = &mut management.registries.yearlies[0];
            let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
            cropland.drset = 1;

            let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
                .expect_err("non-positive enabled drain geometry must fail runtime seam");
            assert_eq!(error.code(), "HS-RUNTIME-E-050");
            assert!(matches!(
                error,
                HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                    field: observed,
                    slot_index: 1,
                    crop_slot_index: 1,
                    ..
                } if observed == field
            ));
        }
    }

    #[test]
    fn management_runtime_projection_projects_wb19_controls_for_primary_perennial_slot() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.imngmt = 2;
        cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
            jdharv: 288,
            jdplt: 130,
            jdstop: 0,
            rw: 0.762,
            mgtopt: 3,
            cut_days: Vec::new(),
            grazing_cycles: Vec::new(),
        });

        let merged_surface = build_hillslope_runtime_surface_from_management(&management)
            .expect("primary perennial slot should still project WB19 controls");

        assert_eq!(
            merged_surface.state_surface.get(&BoundarySymbol::from("drset")),
            Some(&BoundaryValue::scalar(0.0))
        );
        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("wb19_drain_enabled")),
            Some(&BoundaryValue::scalar(0.0))
        );
    }

    #[test]
    fn management_runtime_projection_projects_perennial_cutday_payload() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.imngmt = 2;
        cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
            jdharv: 288,
            jdplt: 130,
            jdstop: 0,
            rw: 0.762,
            mgtopt: 1,
            cut_days: vec![120, 240],
            grazing_cycles: Vec::new(),
        });

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("valid perennial cut-day payload should project");
        let decomp = &pl_surfaces.pl_decomp_surface;

        assert_eq!(
            decomp.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_ncut"
            )),
            Some(&BoundaryValue::scalar(2.0))
        );
        assert_eq!(
            decomp.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_cutday_0002"
            )),
            Some(&BoundaryValue::scalar(240.0))
        );
    }

    #[test]
    fn management_runtime_projection_projects_perennial_grazing_payload() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.imngmt = 2;
        cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
            jdharv: 288,
            jdplt: 130,
            jdstop: 0,
            rw: 0.762,
            mgtopt: 2,
            cut_days: Vec::new(),
            grazing_cycles: vec![YearlyPerennialGrazingCycle {
                animal: 12.0,
                area: 2.5,
                bodywt: 425.0,
                digest: 0.62,
                gday: 120,
                gend: 150,
            }],
        });

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("valid perennial grazing payload should project");
        let decomp = &pl_surfaces.pl_decomp_surface;

        assert_eq!(
            decomp.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_ncycle"
            )),
            Some(&BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            decomp.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_gday_0001"
            )),
            Some(&BoundaryValue::scalar(120.0))
        );
        assert_eq!(
            decomp.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_digest_0001"
            )),
            Some(&BoundaryValue::scalar(0.62))
        );
    }

    #[test]
    fn management_runtime_projection_rejects_invalid_perennial_payload_cardinality() {
        for (mgtopt, cut_days, grazing_cycles, expected_field) in [
            (1, Vec::new(), Vec::new(), "ncut"),
            (
                2,
                Vec::new(),
                Vec::new(),
                "ncycle",
            ),
        ] {
            let mut management = parse_management_from_str(
                MANAGEMENT_CANONICAL_NONZERO_98_4,
                ManagementParseMode::Strict,
            )
            .expect("management fixture should parse");
            let yearly = &mut management.registries.yearlies[0];
            let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
            cropland.imngmt = 2;
            cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
                jdharv: 288,
                jdplt: 130,
                jdstop: 0,
                rw: 0.762,
                mgtopt,
                cut_days,
                grazing_cycles,
            });

            let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
                .expect_err("missing perennial payload cardinality must fail");
            assert_eq!(error.code(), "HS-RUNTIME-E-048");
            assert!(matches!(
                error,
                HillslopeRuntimeInputError::PlProjectionCardinalityInvalid {
                    field,
                    slot_index: 1,
                    crop_slot_index: 1,
                    value: 0,
                    ..
                } if field == expected_field
            ));
        }
    }

    #[test]
    fn management_runtime_projection_rejects_incompatible_perennial_payloads() {
        for (mgtopt, cut_days, grazing_cycles, expected_field) in [
            (
                1,
                vec![120],
                vec![YearlyPerennialGrazingCycle {
                    animal: 12.0,
                    area: 2.5,
                    bodywt: 425.0,
                    digest: 0.62,
                    gday: 120,
                    gend: 150,
                }],
                "grazing_cycles",
            ),
            (
                2,
                vec![120],
                vec![YearlyPerennialGrazingCycle {
                    animal: 12.0,
                    area: 2.5,
                    bodywt: 425.0,
                    digest: 0.62,
                    gday: 120,
                    gend: 150,
                }],
                "cut_days",
            ),
            (3, vec![120], Vec::new(), "cut_days"),
            (
                3,
                Vec::new(),
                vec![YearlyPerennialGrazingCycle {
                    animal: 12.0,
                    area: 2.5,
                    bodywt: 425.0,
                    digest: 0.62,
                    gday: 120,
                    gend: 150,
                }],
                "grazing_cycles",
            ),
        ] {
            let mut management = parse_management_from_str(
                MANAGEMENT_CANONICAL_NONZERO_98_4,
                ManagementParseMode::Strict,
            )
            .expect("management fixture should parse");
            let yearly = &mut management.registries.yearlies[0];
            let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
            cropland.imngmt = 2;
            cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
                jdharv: 288,
                jdplt: 130,
                jdstop: 0,
                rw: 0.762,
                mgtopt,
                cut_days,
                grazing_cycles,
            });

            let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
                .expect_err("incompatible perennial payload must fail");
            assert_eq!(error.code(), "HS-RUNTIME-E-051");
            assert!(matches!(
                error,
                HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                    field,
                    slot_index: 1,
                    crop_slot_index: 1,
                    ..
                } if field == expected_field
            ));
        }
    }

    #[test]
    fn management_runtime_projection_assimilates_initial_perennial_live_canopy() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");

        let initial = &mut management.registries.initials[0];
        let InitialScenarioData::Cropland(initial_data) = &mut initial.data;
        initial_data.base_line[1] = 0.9;
        initial_data.imngmt = 2;

        let plant = &mut management.registries.plants[0];
        let PlantScenarioData::Cropland(plant_data) = &mut plant.data;
        plant_data.canopy_line[0] = 14.0;
        plant_data.canopy_line[1] = 3.0;
        plant_data.growth_line[7] = 2.6;
        plant_data.residue_line[5] = 1.6;
        plant_data.residue_line[7] = 2.0;
        plant_data.terminal_line[1] = 5.0;

        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.imngmt = 2;
        cropland.branch = YearlyCroplandBranch::Perennial(YearlyPerennialData {
            jdharv: 0,
            jdplt: 0,
            jdstop: 0,
            rw: 0.0,
            mgtopt: 3,
            cut_days: Vec::new(),
            grazing_cycles: Vec::new(),
        });

        let merged_surface = build_hillslope_runtime_surface_from_management(&management)
            .expect("established perennial initial canopy should project");

        let state = &merged_surface.state_surface;
        let cancov = state
            .get(&BoundarySymbol::from("cancov"))
            .expect("cancov should be published")
            .as_f64();
        let vdmt = state
            .get(&BoundarySymbol::from("vdmt"))
            .expect("vdmt should be published")
            .as_f64();
        let lai = state
            .get(&BoundarySymbol::from("lai"))
            .expect("lai should be published")
            .as_f64();
        let rtd = state
            .get(&BoundarySymbol::from("rtd"))
            .expect("rtd should be published")
            .as_f64();
        let rtmass = state
            .get(&BoundarySymbol::from("rtmass"))
            .expect("rtmass should be published")
            .as_f64();

        assert!((cancov - 0.9).abs() < 1e-12);
        assert!(vdmt > 0.0, "initial cancov must seed live biomass");
        assert!(lai > 0.0, "initial cancov must seed live LAI");
        assert!((rtd - 1.6).abs() < 1e-12);
        assert!((rtmass - 2.0).abs() < 1e-12);
    }

    #[test]
    fn management_runtime_projection_assimilates_initial_annual_live_canopy() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");

        let initial = &mut management.registries.initials[0];
        let InitialScenarioData::Cropland(initial_data) = &mut initial.data;
        initial_data.base_line[1] = 0.6;
        initial_data.imngmt = 1;

        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.imngmt = 1;
        cropland.branch = YearlyCroplandBranch::AnnualOrFallow(YearlyAnnualFallowData {
            jdharv: 120,
            jdplt: 150,
            rw: 0.762,
            resmgt: 6,
            extension: None,
        });

        let merged_surface = build_hillslope_runtime_surface_from_management(&management)
            .expect("annual positive initial canopy should project");
        let state = &merged_surface.state_surface;
        let vdmt = state
            .get(&BoundarySymbol::from("vdmt"))
            .expect("vdmt should be published")
            .as_f64();
        let lai = state
            .get(&BoundarySymbol::from("lai"))
            .expect("lai should be published")
            .as_f64();
        let rtd = state
            .get(&BoundarySymbol::from("rtd"))
            .expect("rtd should be published")
            .as_f64();
        let rtmass = state
            .get(&BoundarySymbol::from("rtmass"))
            .expect("rtmass should be published")
            .as_f64();

        assert!(vdmt > 0.0);
        assert!(lai > 0.0);
        assert!(rtd > 0.0);
        assert!(rtmass > 0.0);
    }

    #[test]
    fn management_runtime_projection_resets_initial_canopy_for_fallow_management() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");

        let initial = &mut management.registries.initials[0];
        let InitialScenarioData::Cropland(initial_data) = &mut initial.data;
        initial_data.base_line[1] = 0.9;
        initial_data.imngmt = 3;

        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.imngmt = 3;

        let merged_surface = build_hillslope_runtime_surface_from_management(&management)
            .expect("fallow management should project with reset initial canopy");

        assert_eq!(
            merged_surface
                .state_surface
                .get(&BoundarySymbol::from("cancov")),
            Some(&BoundaryValue::scalar(0.0))
        );
        assert_eq!(
            merged_surface.state_surface.get(&BoundarySymbol::from("lai")),
            Some(&BoundaryValue::scalar(0.0))
        );
    }

    #[test]
    fn management_runtime_projection_rejects_initial_canopy_assimilation_domains() {
        for field in ["bb", "xmxlai"] {
            let mut management = parse_management_from_str(
                MANAGEMENT_CANONICAL_NONZERO_98_4,
                ManagementParseMode::Strict,
            )
            .expect("management fixture should parse");

            let initial = &mut management.registries.initials[0];
            let InitialScenarioData::Cropland(initial_data) = &mut initial.data;
            initial_data.base_line[1] = 0.6;
            initial_data.imngmt = 1;

            let plant = &mut management.registries.plants[0];
            let PlantScenarioData::Cropland(plant_data) = &mut plant.data;
            match field {
                "bb" => plant_data.canopy_line[0] = 0.0,
                "xmxlai" => plant_data.terminal_line[1] = 0.0,
                _ => unreachable!("test field set is exhaustive"),
            }

            let yearly = &mut management.registries.yearlies[0];
            let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
            cropland.imngmt = 1;
            cropland.branch = YearlyCroplandBranch::AnnualOrFallow(YearlyAnnualFallowData {
                jdharv: 120,
                jdplt: 150,
                rw: 0.762,
                resmgt: 6,
                extension: None,
            });

            let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
                .expect_err("invalid initial canopy assimilation domain must fail");
            assert_eq!(error.code(), "HS-RUNTIME-E-050");
            assert!(matches!(
                error,
                HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                    field: observed,
                    slot_index: 1,
                    crop_slot_index: 1,
                    ..
                } if observed == field
            ));
        }
    }

    #[test]
    fn management_runtime_projection_rejects_drain_reference_without_registered_drain() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        cropland.drset = 1;

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("drset reference beyond registered drains must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-050");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "drset",
                slot_index: 1,
                crop_slot_index: 1,
                value,
                ..
            } if (value - 1.0).abs() < 1e-12
        ));
    }

    #[test]
    fn management_runtime_projection_rejects_non_finite_row_width() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let yearly = &mut management.registries.yearlies[0];
        let YearlyScenarioData::Cropland(cropland) = &mut yearly.data;
        match &mut cropland.branch {
            YearlyCroplandBranch::AnnualOrFallow(annual) => annual.rw = f64::NAN,
            YearlyCroplandBranch::Perennial(_) => panic!("fixture should use annual branch"),
        }

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("non-finite row width must fail runtime seam");
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
    fn management_runtime_projection_rejects_unsupported_perennial_option() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
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
            .expect_err("unsupported perennial mgtopt must fail runtime seam");
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
    fn management_runtime_projection_allows_zero_gddmax_sentinel_for_legacy_resolution() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let plant = &mut management.registries.plants[0];
        let PlantScenarioData::Cropland(cropland) = &mut plant.data;
        cropland.growth_line[5] = 0.0;

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("gddmax zero sentinel should project for runtime resolution");
        assert_eq!(
            pl_surfaces.pl_growth_surface.get(&BoundarySymbol::from(
                "pl_growth_slot_0001_crop_0001_gddmax"
            )),
            Some(&BoundaryValue::scalar(0.0))
        );
    }

    #[test]
    fn management_runtime_projection_allows_zero_oratea_orater_for_legacy_no_decay() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let plant = &mut management.registries.plants[0];
        let PlantScenarioData::Cropland(cropland) = &mut plant.data;
        cropland.residue_line[0] = 0.0;
        cropland.residue_line[1] = 0.0;

        let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect("zero decomposition constants should project for legacy no-decay semantics");
        assert_eq!(
            pl_surfaces.pl_decomp_surface.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_oratea"
            )),
            Some(&BoundaryValue::scalar(0.0))
        );
        assert_eq!(
            pl_surfaces.pl_decomp_surface.get(&BoundarySymbol::from(
                "pl_decomp_slot_0001_crop_0001_orater"
            )),
            Some(&BoundaryValue::scalar(0.0))
        );
    }

    #[test]
    fn management_runtime_projection_rejects_negative_oratea_projection_field() {
        let mut management = parse_management_from_str(
            MANAGEMENT_CANONICAL_NONZERO_98_4,
            ManagementParseMode::Strict,
        )
        .expect("management fixture should parse");
        let plant = &mut management.registries.plants[0];
        let PlantScenarioData::Cropland(cropland) = &mut plant.data;
        cropland.residue_line[0] = -0.1;

        let error = build_hillslope_pl_runtime_surfaces_from_management(&management)
            .expect_err("negative decomposition constants must fail runtime seam");
        assert_eq!(error.code(), "HS-RUNTIME-E-050");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "oratea",
                slot_index: 1,
                crop_slot_index: 1,
                value,
                ..
            } if (value + 0.1).abs() < 1e-12
        ));
    }
