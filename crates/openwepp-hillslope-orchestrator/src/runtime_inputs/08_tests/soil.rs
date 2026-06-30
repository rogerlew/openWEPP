    #[test]
    fn auth12_fc_wp_policy_matches_producer_corrected_measured_theta_contract() {
        for datver in [
            SoilDatver::V7777,
            SoilDatver::V7778,
            SoilDatver::V9002,
            SoilDatver::V9003,
            SoilDatver::V9005,
        ] {
            assert!(
                matches!(
                    super::fc_wp_rock_multiplier_policy(datver),
                    super::FcWpRockMultiplierPolicy::ApplyToMeasuredFcWp
                ),
                "measured-theta datver {datver:?} must apply FC/WP cpm multiplier (producer pre-adjust contract)"
            );
        }

        for datver in [SoilDatver::V97_5, SoilDatver::V2006_2] {
            assert!(
                matches!(
                    super::fc_wp_rock_multiplier_policy(datver),
                    super::FcWpRockMultiplierPolicy::SkipForMeasuredFcWp
                ),
                "legacy datver {datver:?} remains outside measured-theta policy families"
            );
        }
    }

    #[test]
    fn soil_runtime_surface_contains_canonical_state_symbols() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");
        let raw_top_layer = soil
            .ofes
            .first()
            .and_then(|ofe| ofe.layers.first())
            .expect("9002 soil fixture should include a top layer");
        let raw_top_thetdr = raw_top_layer
            .theta_r_rosetta
            .or(raw_top_layer.wp_measured)
            .expect("top layer should include theta residual source");
        let raw_top_thetfc = raw_top_layer
            .fc_rosetta
            .or(raw_top_layer.fc_measured)
            .expect("top layer should include theta field-capacity source");

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");

        let solthk = soil_runtime_scalar(&surface, "solthk");
        let dg = soil_runtime_scalar(&surface, "dg");
        let thetdr = soil_runtime_scalar(&surface, "thetdr");
        let thetfc = soil_runtime_scalar(&surface, "thetfc");
        let wb19_lateral_anisotropy_ratio =
            soil_runtime_scalar(&surface, "wb19_lateral_anisotropy_ratio");

        assert!((solthk - 0.4).abs() < 1.0e-12);
        assert!((dg - 0.1).abs() < 1.0e-12);
        assert!(thetdr.is_finite());
        assert!(thetfc.is_finite());
        assert!(
            (wb19_lateral_anisotropy_ratio - 1.0).abs() < 1.0e-12,
            "modern ui_anisrt soils must not double-apply layer anisotropy as profile anisrt"
        );
        assert!(
            (thetdr - raw_top_thetdr).abs() > 1.0e-9 || (thetfc - raw_top_thetfc).abs() > 1.0e-9,
            "authoritative theta symbols should be correction-lineage projected, not raw parser-theta values"
        );

        let exact_checks = [
            ("nsl", 2.0),
            ("wb11_nsl", 2.0),
            ("ssc", 15.0 / 3.6e6),
            ("ssc_0001", 15.0 / 3.6e6),
            ("wb19_lateral_ssh_0001", 17.25 / 3.6e6),
            ("wb19_lateral_ssh_0002", 8.8 / 3.6e6),
            ("dg_0002", 0.15),
            ("solthk_0002", 0.25),
            ("wb19_dg_0002", 0.2),
            ("wb19_solthk_0002", 0.4),
            ("ssc_0002", 8.0 / 3.6e6),
        ];
        for (symbol, expected) in exact_checks {
            assert!((soil_runtime_scalar(&surface, symbol) - expected).abs() < 1.0e-12);
        }
        assert!(
            (soil_runtime_scalar(&surface, "ssc_0001")
                - soil_runtime_scalar(&surface, "wb19_lateral_ssh_0001"))
            .abs()
                > 1.0e-9,
            "split-layer vertical ssc must not alias hourly horizontal ui_ssh"
        );

        for symbol in [
            "por",
            "cpm",
            "coca",
            "wb19_por_0002",
            "cpm_0002",
            "wb19_coca_0002",
        ] {
            let value = soil_runtime_scalar(&surface, symbol);
            assert!(
                value.is_finite() && value > 0.0 && value <= 1.0,
                "{symbol} should be finite and inside (0,1]"
            );
        }
        if let Some(restrictive_layer) = soil.restrictive_layer.as_ref() {
            assert!(
                (soil_runtime_scalar(&surface, "slflag")
                    - if restrictive_layer.slflag { 1.0 } else { 0.0 })
                    .abs()
                    < 1.0e-12
            );
            assert!(
                (soil_runtime_scalar(&surface, "kslast") - (restrictive_layer.kslast_mm_h / 3.6e6))
                    .abs()
                    < 1.0e-12
            );
            assert!(
                (soil_runtime_scalar(&surface, "ui_bdrkth")
                    - (restrictive_layer.ui_bdrkth_mm / 1_000.0))
                    .abs()
                    < 1.0e-12
            );
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn typed_soil_wb11_projection_matches_runtime_surface_adapter() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");

        let typed = project_typed_soil_wb11_runtime(&soil)
            .expect("typed soil WB11 projection should build from parsed soil");
        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");

        assert_eq!(typed.nsl, 2);
        assert_eq!(
            f64::from(u32::try_from(typed.nsl).expect("nsl should fit u32")).to_bits(),
            soil_runtime_scalar(&surface, "wb11_nsl").to_bits()
        );
        assert_eq!(typed.sat.to_bits(), soil_runtime_scalar(&surface, "sat").to_bits());
        assert_eq!(
            typed.solwpv.to_bits(),
            soil_runtime_scalar(&surface, "solwpv").to_bits()
        );
        assert_eq!(typed.salb.to_bits(), soil_runtime_scalar(&surface, "salb").to_bits());
        assert_eq!(
            typed.solthk_m.to_bits(),
            soil_runtime_scalar(&surface, "solthk").to_bits()
        );
        assert_eq!(
            typed.lateral_anisotropy_ratio.to_bits(),
            soil_runtime_scalar(&surface, "wb19_lateral_anisotropy_ratio").to_bits()
        );
        assert_eq!(
            (if typed.ksatadj { 1.0_f64 } else { 0.0_f64 }).to_bits(),
            soil_runtime_scalar(&surface, "ksatadj").to_bits()
        );
        assert_eq!(
            typed.profile_depth_mm.map(f64::to_bits),
            Some(soil_runtime_scalar(&surface, "wb13_profile_depth_mm").to_bits())
        );
        assert_eq!(
            typed.profile_porosity_cap_mm.map(f64::to_bits),
            Some(soil_runtime_scalar(&surface, "wb13_profile_porosity_cap_mm").to_bits())
        );
        assert_eq!(
            typed.profile_fc_store_mm.map(f64::to_bits),
            Some(soil_runtime_scalar(&surface, "wb13_profile_fc_store_mm").to_bits())
        );
        assert_eq!(
            typed.profile_wp_store_mm.map(f64::to_bits),
            Some(soil_runtime_scalar(&surface, "wb13_profile_wp_store_mm").to_bits())
        );
        assert_eq!(
            typed.profile_fc_tail_mm.map(f64::to_bits),
            Some(soil_runtime_scalar(&surface, "wb13_profile_fc_tail_mm").to_bits())
        );

        for (offset, layer) in typed.layers.iter().enumerate() {
            let layer_index = offset + 1;
            assert_eq!(
                layer.solthk_m.to_bits(),
                soil_runtime_scalar(&surface, &format!("wb19_solthk_{layer_index:04}")).to_bits()
            );
            assert_eq!(
                layer.dg_m.to_bits(),
                soil_runtime_scalar(&surface, &format!("wb19_dg_{layer_index:04}")).to_bits()
            );
            assert_eq!(
                layer.porosity.to_bits(),
                soil_runtime_scalar(&surface, &format!("wb19_por_{layer_index:04}")).to_bits()
            );
            assert_eq!(
                layer.cpm.to_bits(),
                soil_runtime_scalar(&surface, &format!("cpm_{layer_index:04}")).to_bits()
            );
            assert_eq!(
                layer.coca.to_bits(),
                soil_runtime_scalar(&surface, &format!("wb19_coca_{layer_index:04}")).to_bits()
            );
            assert_eq!(
                layer.bulk_density_kg_m3.to_bits(),
                soil_runtime_scalar(
                    &surface,
                    &format!("wb19_bulk_density_kg_m3_{layer_index:04}")
                )
                .to_bits()
            );
            assert_eq!(
                layer.thetfc.to_bits(),
                soil_runtime_scalar(&surface, &format!("wb19_thetfc_{layer_index:04}")).to_bits()
            );
            assert_eq!(
                layer.thetdr.to_bits(),
                soil_runtime_scalar(&surface, &format!("wb19_thetdr_{layer_index:04}")).to_bits()
            );
            assert_eq!(
                layer.ssc_m_s.to_bits(),
                soil_runtime_scalar(&surface, &format!("ssc_{layer_index:04}")).to_bits()
            );
            assert_eq!(
                layer.lateral_ssh_m_s.to_bits(),
                soil_runtime_scalar(&surface, &format!("wb19_lateral_ssh_{layer_index:04}"))
                    .to_bits()
            );
        }
    }

    #[test]
    fn soil_runtime_surface_projects_harmonic_vertical_ssc_below_top_interval() {
        let mut soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");
        let ofe = soil
            .ofes
            .first_mut()
            .expect("9002 fixture should include a primary OFE");
        let top_layer = ofe
            .layers
            .first()
            .expect("9002 fixture should include layer 1")
            .clone();
        let mut high_ksat_layer = top_layer.clone();
        high_ksat_layer.depth_mm = 560.0;
        high_ksat_layer.ksat_mm_h = Some(330.2755);
        high_ksat_layer.anisotropy_ratio = Some(1.25);
        let mut low_ksat_layer = top_layer.clone();
        low_ksat_layer.depth_mm = 1140.0;
        low_ksat_layer.ksat_mm_h = Some(33.0275);
        low_ksat_layer.anisotropy_ratio = Some(0.65);
        let mut tail_layer = low_ksat_layer.clone();
        tail_layer.depth_mm = 1600.0;

        ofe.nsl = 4;
        ofe.layers = vec![top_layer, high_ksat_layer, low_ksat_layer, tail_layer];

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from synthetic split conductivity fixture");
        let harmonic_split_layer_ssc_mm_h = 200.0 / (160.0 / 330.2755 + 40.0 / 33.0275);
        let arithmetic_split_layer_ssh_mm_h =
            (160.0 * 330.2755 * 1.25 + 40.0 * 33.0275 * 0.65) / 200.0;

        assert!((soil_runtime_scalar(&surface, "ssc_0002") - 330.2755 / 3.6e6).abs() < 1.0e-12);
        assert!(
            (soil_runtime_scalar(&surface, "wb19_lateral_ssh_0002")
                - (330.2755 * 1.25) / 3.6e6)
                .abs()
                < 1.0e-12
        );
        assert!(
            (soil_runtime_scalar(&surface, "ssc_0003")
                - harmonic_split_layer_ssc_mm_h / 3.6e6)
                .abs()
                < 1.0e-12
        );
        assert!(
            (soil_runtime_scalar(&surface, "wb19_lateral_ssh_0003")
                - arithmetic_split_layer_ssh_mm_h / 3.6e6)
                .abs()
                < 1.0e-12
        );
        assert!(
            (soil_runtime_scalar(&surface, "ssc_0003")
                - soil_runtime_scalar(&surface, "wb19_lateral_ssh_0003"))
            .abs()
                > 1.0e-9,
            "H2637-shaped split layer must separate vertical ssc from hourly ui_ssh"
        );
        assert!((soil_runtime_scalar(&surface, "ssc_0004") - 33.0275 / 3.6e6).abs() < 1.0e-12);
        assert!(
            (soil_runtime_scalar(&surface, "wb19_lateral_ssh_0004")
                - (33.0275 * 0.65) / 3.6e6)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn soil_runtime_surface_projects_wb13_profile_lineage_symbols() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");
        let ofe = soil
            .ofes
            .first()
            .expect("9002 fixture should include a primary OFE");
        let expected_profile =
            expected_wb13_profile_symbols_from_normalized_correction(soil.datver, ofe);

        let profile_depth_mm = surface
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_depth_mm"))
            .expect("wb13_profile_depth_mm should be present")
            .as_f64();
        let profile_porosity_cap_mm = surface
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_porosity_cap_mm"))
            .expect("wb13_profile_porosity_cap_mm should be present")
            .as_f64();
        let profile_fc_store_mm = surface
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_fc_store_mm"))
            .expect("wb13_profile_fc_store_mm should be present")
            .as_f64();
        let profile_fc_tail_mm = surface
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_fc_tail_mm"))
            .expect("wb13_profile_fc_tail_mm should be present")
            .as_f64();
        let profile_wp_store_mm = surface
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_wp_store_mm"))
            .expect("wb13_profile_wp_store_mm should be present")
            .as_f64();
        let (layer_fc_store_mm, layer_wp_store_mm) =
            aggregated_profile_storage_from_layer_symbols(&surface);

        assert!((profile_depth_mm - expected_profile.depth).abs() < 1e-9);
        assert!((profile_porosity_cap_mm - expected_profile.porosity_cap).abs() < 1e-9);
        assert!((profile_fc_store_mm - expected_profile.fc_store).abs() < 1e-9);
        assert!((profile_wp_store_mm - expected_profile.wp_store).abs() < 1e-9);
        assert!((profile_fc_tail_mm - (profile_fc_store_mm - layer_fc_store_mm)).abs() < 1e-9);
        assert!(
            (profile_fc_store_mm - layer_fc_store_mm).abs() < 1.0e-9,
            "profile FC storage must now be represented by normalized primary WB11 layers"
        );
        assert!(
            (profile_wp_store_mm - layer_wp_store_mm).abs() < 1.0e-9,
            "profile WP storage must now be represented by normalized primary WB11 layers"
        );
        assert!(profile_porosity_cap_mm >= profile_fc_store_mm);
        assert!(profile_fc_store_mm >= profile_wp_store_mm);
    }

    #[test]
    fn hphys0207_profile_fc_wp_projection_preserves_normalized_depth_authority() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");
        let ofe = soil
            .ofes
            .first()
            .expect("9002 fixture should include a primary OFE");
        let expected_profile =
            expected_wb13_profile_symbols_from_normalized_correction(soil.datver, ofe);

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");
        let (layer_fc_store_mm, layer_wp_store_mm) =
            aggregated_profile_storage_from_layer_symbols(&surface);

        let projected_fc_store_mm = surface
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_fc_store_mm"))
            .expect("wb13_profile_fc_store_mm should be present")
            .as_f64();
        let projected_fc_tail_mm = surface
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_fc_tail_mm"))
            .expect("wb13_profile_fc_tail_mm should be present")
            .as_f64();
        let projected_wp_store_mm = surface
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_wp_store_mm"))
            .expect("wb13_profile_wp_store_mm should be present")
            .as_f64();

        assert!(
            (projected_fc_store_mm - expected_profile.fc_store).abs() < 1e-9,
            "projected FC storage must match normalized-profile corrected aggregate"
        );
        assert!(
            (projected_wp_store_mm - expected_profile.wp_store).abs() < 1e-9,
            "projected WP storage must match normalized-profile corrected aggregate"
        );
        assert!(
            (projected_fc_store_mm - layer_fc_store_mm).abs() < 1.0e-9,
            "projected FC storage must be represented by normalized primary WB11 layers"
        );
        assert!(
            (projected_fc_tail_mm - (projected_fc_store_mm - layer_fc_store_mm)).abs() < 1.0e-9,
            "projected FC tail storage must reconcile to zero residual normalized-primary layer coverage"
        );
        assert!(
            (projected_wp_store_mm - layer_wp_store_mm).abs() < 1.0e-9,
            "projected WP storage must be represented by normalized primary WB11 layers"
        );
    }

    #[test]
    fn hphys0207_corrected_layer_moisture_preserves_per_layer_storage_ordering() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");
        let ofe = soil
            .ofes
            .first()
            .expect("9002 fixture should include a primary OFE");

        let corrected_layers = normalized_corrected_layers_from_ofe(soil.datver, ofe);
        assert!(
            !corrected_layers.is_empty(),
            "normalized corrected layer set must be non-empty"
        );
        for (layer_position, (_thickness_m, porosity, cpm, coca, thetfc, thetdr)) in
            corrected_layers.into_iter().enumerate()
        {
            assert!(
                porosity >= thetfc,
                "corrected layer {} must satisfy porosity >= thetfc (observed porosity={}, thetfc={})",
                layer_position + 1,
                porosity,
                thetfc
            );
            assert!(
                thetfc >= thetdr,
                "corrected layer {} must satisfy thetfc >= thetdr (observed thetfc={}, thetdr={})",
                layer_position + 1,
                thetfc,
                thetdr
            );
            assert!(
                thetdr > 0.0,
                "corrected layer {} must satisfy thetdr > 0 (observed {})",
                layer_position + 1,
                thetdr
            );
            assert!(
                cpm > 0.0 && cpm <= 1.0,
                "corrected layer {} must satisfy cpm in (0,1] (observed {})",
                layer_position + 1,
                cpm
            );
            assert!(
                coca > 0.0 && coca <= 1.0,
                "corrected layer {} must satisfy coca in (0,1] (observed {})",
                layer_position + 1,
                coca
            );
        }
    }

    #[test]
    fn hphys0206_authoritative_theta_uses_normalized_overlap_mapping() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");
        let ofe = soil
            .ofes
            .first()
            .expect("9002 fixture should include a primary OFE");
        let expected = normalized_corrected_layers_from_ofe(soil.datver, ofe);
        assert!(
            expected.len() >= 2,
            "expected at least two normalized layers for 9002 fixture"
        );

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");
        for (
            layer_position,
            (
                expected_dg,
                expected_porosity,
                expected_cpm,
                expected_coca,
                expected_thetfc,
                expected_thetdr,
            ),
        ) in expected.iter().enumerate()
        {
            let layer_index = layer_position + 1;
            let observed_dg = surface
                .state_surface
                .get(&BoundarySymbol::from(format!("wb19_dg_{layer_index:04}")))
                .unwrap_or_else(|| panic!("wb19_dg_{layer_index:04} should be present"))
                .as_f64();
            let observed_porosity = surface
                .state_surface
                .get(&BoundarySymbol::from(format!("wb19_por_{layer_index:04}")))
                .unwrap_or_else(|| panic!("wb19_por_{layer_index:04} should be present"))
                .as_f64();
            let observed_cpm = surface
                .state_surface
                .get(&BoundarySymbol::from(format!("cpm_{layer_index:04}")))
                .unwrap_or_else(|| panic!("cpm_{layer_index:04} should be present"))
                .as_f64();
            let observed_coca = surface
                .state_surface
                .get(&BoundarySymbol::from(format!("wb19_coca_{layer_index:04}")))
                .unwrap_or_else(|| panic!("wb19_coca_{layer_index:04} should be present"))
                .as_f64();
            let observed_thetfc = surface
                .state_surface
                .get(&BoundarySymbol::from(format!("wb19_thetfc_{layer_index:04}")))
                .unwrap_or_else(|| panic!("wb19_thetfc_{layer_index:04} should be present"))
                .as_f64();
            let observed_thetdr = surface
                .state_surface
                .get(&BoundarySymbol::from(format!("wb19_thetdr_{layer_index:04}")))
                .unwrap_or_else(|| panic!("wb19_thetdr_{layer_index:04} should be present"))
                .as_f64();

            assert!(
                (observed_dg - expected_dg).abs() < 1.0e-9,
                "layer {layer_index} authoritative dg must follow normalized primary WB11 grid"
            );
            assert!(
                (observed_porosity - expected_porosity).abs() < 1.0e-9,
                "layer {layer_index} authoritative porosity must follow normalized primary WB11 grid"
            );
            assert!(
                (observed_cpm - expected_cpm).abs() < 1.0e-9,
                "layer {layer_index} authoritative cpm must follow normalized primary WB11 grid"
            );
            assert!(
                (observed_coca - expected_coca).abs() < 1.0e-9,
                "layer {layer_index} authoritative coca must follow normalized primary WB11 grid"
            );
            assert!(
                (observed_thetfc - expected_thetfc).abs() < 1.0e-9,
                "layer {layer_index} authoritative thetfc must follow normalized primary WB11 grid"
            );
            assert!(
                (observed_thetdr - expected_thetdr).abs() < 1.0e-9,
                "layer {layer_index} authoritative thetdr must follow normalized primary WB11 grid"
            );
        }

        let projected_sat = surface
            .state_surface
            .get(&BoundarySymbol::from("sat"))
            .expect("sat should be present")
            .as_f64();
        let expected_sat = ofe.sat;
        assert!(
            (projected_sat - expected_sat).abs() < 1.0e-12,
            "projected sat must preserve authoritative parser saturation fraction"
        );
    }

    #[test]
    fn hphys0206_soil_runtime_surface_fail_closed_when_normalized_correction_input_missing() {
        let mut soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");
        soil.ofes[0].layers[0].bulk_density_g_cm3 = None;

        let error = build_hillslope_runtime_surface_from_soil(&soil)
            .expect_err("missing normalized corrected-lineage input must hard-fail");
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
    fn soil_runtime_surface_projects_ksatadj_policy_symbols_for_9002() {
        let soil = parse_soil(
            VALID_9002,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("9002 soil fixture should parse");

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");

        let solwpv = surface
            .state_surface
            .get(&BoundarySymbol::from("solwpv"))
            .expect("solwpv should be present")
            .as_f64();
        assert!((solwpv - 9002.0).abs() < 1e-12);

        let ksatadj = surface
            .state_surface
            .get(&BoundarySymbol::from("ksatadj"))
            .expect("ksatadj should be present")
            .as_f64();
        assert!((ksatadj - 1.0).abs() < 1e-12);

        let ksatfac = surface
            .state_surface
            .get(&BoundarySymbol::from("ksatfac"))
            .expect("ksatfac should be present")
            .as_f64();
        assert!((ksatfac - 0.20).abs() < 1e-12);

        let ksatrec = surface
            .state_surface
            .get(&BoundarySymbol::from("ksatrec"))
            .expect("ksatrec should be present")
            .as_f64();
        assert!((ksatrec - 0.001).abs() < 1e-12);

        let ofe_ksatadj = surface
            .state_surface
            .get(&BoundarySymbol::from("ofe1_ksatadj"))
            .expect("ofe1_ksatadj should be present")
            .as_f64();
        assert!((ofe_ksatadj - 1.0).abs() < 1e-12);
    }

    #[test]
    fn soil_runtime_surface_defaults_ksatadj_to_zero_without_policy_block() {
        let soil = parse_soil(
            VALID_7778,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("7778 soil fixture should parse");

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from parsed soil");

        let solwpv = surface
            .state_surface
            .get(&BoundarySymbol::from("solwpv"))
            .expect("solwpv should be present")
            .as_f64();
        assert!((solwpv - 7778.0).abs() < 1e-12);

        let ksatadj = surface
            .state_surface
            .get(&BoundarySymbol::from("ksatadj"))
            .expect("ksatadj should be present")
            .as_f64();
        assert!(ksatadj.abs() < 1e-12);

        assert!(
            !surface
                .state_surface
                .contains_key(&BoundarySymbol::from("ksatfac")),
            "ksatfac should be absent when datver policy block is absent"
        );
        assert!(
            !surface
                .state_surface
                .contains_key(&BoundarySymbol::from("ksatrec")),
            "ksatrec should be absent when datver policy block is absent"
        );
        assert!(
            !surface
                .state_surface
                .contains_key(&BoundarySymbol::from("lkeff")),
            "lkeff should be absent when datver policy block is absent"
        );
    }

    #[test]
    fn soil_runtime_surface_uses_measured_theta_fallback_for_7778() {
        let soil = parse_soil(
            VALID_7778,
            SoilParserOptions {
                mode: ParserMode::Strict,
                allow_legacy_aliases: false,
                expected_topology_count: None,
                topology_scope: None,
            },
        )
        .expect("7778 soil fixture should parse");
        let raw_layer1 = soil
            .ofes
            .first()
            .and_then(|ofe| ofe.layers.first())
            .expect("7778 soil fixture should include layer 1");
        let raw_layer2 = soil
            .ofes
            .first()
            .and_then(|ofe| ofe.layers.get(1))
            .expect("7778 soil fixture should include layer 2");
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

        let surface = build_hillslope_runtime_surface_from_soil(&soil)
            .expect("runtime surface should build from 7778 measured theta fields");

        let thetdr = surface
            .state_surface
            .get(&BoundarySymbol::from("thetdr"))
            .expect("thetdr should be present")
            .as_f64();
        let thetfc = surface
            .state_surface
            .get(&BoundarySymbol::from("thetfc"))
            .expect("thetfc should be present")
            .as_f64();
        let layer2_thetdr = surface
            .state_surface
            .get(&BoundarySymbol::from("thetdr_0002"))
            .expect("thetdr_0002 should be present")
            .as_f64();
        let layer2_thetfc = surface
            .state_surface
            .get(&BoundarySymbol::from("thetfc_0002"))
            .expect("thetfc_0002 should be present")
            .as_f64();
        assert!(thetdr.is_finite());
        assert!(thetfc.is_finite());
        assert!(layer2_thetdr.is_finite());
        assert!(layer2_thetfc.is_finite());
        assert!(
            (thetdr - raw_layer1_thetdr).abs() > 1.0e-9
                || (thetfc - raw_layer1_thetfc).abs() > 1.0e-9
                || (layer2_thetdr - raw_layer2_thetdr).abs() > 1.0e-9
                || (layer2_thetfc - raw_layer2_thetfc).abs() > 1.0e-9,
            "7778 authoritative theta symbols should carry corrected lineage, not raw measured-theta values"
        );
    }

    #[test]
    fn soil_runtime_surface_rejects_missing_theta_fields() {
        let soil = parse_soil(VALID_97_5, SoilParserOptions::default())
            .expect("97.5 soil fixture should parse");

        let error = build_hillslope_runtime_surface_from_soil(&soil)
            .expect_err("missing theta fields must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-003");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::MissingThetaResidual
        ));
    }

    #[test]
    fn soil_runtime_surface_rejects_missing_saturated_conductivity() {
        let mut soil = parse_soil(VALID_9002, SoilParserOptions::default())
            .expect("9002 soil fixture should parse");
        soil.ofes[0].layers[0].ksat_mm_h = None;

        let error = build_hillslope_runtime_surface_from_soil(&soil)
            .expect_err("missing ksat must fail runtime adaptation");
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
    fn soil_runtime_surface_rejects_non_finite_saturated_conductivity() {
        let mut soil = parse_soil(VALID_9002, SoilParserOptions::default())
            .expect("9002 soil fixture should parse");
        soil.ofes[0].layers[1].ksat_mm_h = Some(f64::NAN);

        let error = build_hillslope_runtime_surface_from_soil(&soil)
            .expect_err("non-finite ksat must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-034");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::NonFiniteSaturatedConductivity {
                ofe_index: 1,
                layer_index: 2,
                value_mm_h
            } if value_mm_h.is_nan()
        ));
    }

    #[test]
    fn soil_runtime_surface_rejects_non_positive_saturated_conductivity() {
        let mut soil = parse_soil(VALID_9002, SoilParserOptions::default())
            .expect("9002 soil fixture should parse");
        soil.ofes[0].layers[1].ksat_mm_h = Some(0.0);

        let error = build_hillslope_runtime_surface_from_soil(&soil)
            .expect_err("non-positive ksat must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-035");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::NonPositiveSaturatedConductivity {
                ofe_index: 1,
                layer_index: 2,
                value_mm_h: 0.0
            }
        ));
    }
