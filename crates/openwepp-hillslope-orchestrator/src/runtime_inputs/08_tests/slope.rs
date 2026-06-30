    #[test]
    fn slope_runtime_surface_contains_canonical_state_symbols() {
        let slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
            .expect("strict slope fixture should parse");
        let surface = build_hillslope_runtime_surface_from_slope(&slope)
            .expect("slope runtime surface should build");

        let nelem = surface
            .state_surface
            .get(&BoundarySymbol::from("nelem"))
            .expect("nelem should be present")
            .as_f64();
        let slplen = surface
            .state_surface
            .get(&BoundarySymbol::from("slplen"))
            .expect("slplen should be present")
            .as_f64();
        let nslpts = surface
            .state_surface
            .get(&BoundarySymbol::from("nslpts"))
            .expect("nslpts should be present")
            .as_f64();
        let avgslp = surface
            .state_surface
            .get(&BoundarySymbol::from("avgslp"))
            .expect("avgslp should be present")
            .as_f64();
        let azm = surface
            .state_surface
            .get(&BoundarySymbol::from("azm"))
            .expect("azm should be present")
            .as_f64();
        let xinput_2 = surface
            .state_surface
            .get(&BoundarySymbol::from("xinput_0002"))
            .expect("xinput_0002 should be present")
            .as_f64();
        let slpinp_2 = surface
            .state_surface
            .get(&BoundarySymbol::from("slpinp_0002"))
            .expect("slpinp_0002 should be present")
            .as_f64();
        let ofe2_avgslp = surface
            .state_surface
            .get(&BoundarySymbol::from("ofe2_avgslp"))
            .expect("ofe2_avgslp should be present")
            .as_f64();

        assert!((nelem - 2.0).abs() < 1e-12);
        assert!((slplen - 60.0).abs() < 1e-12);
        assert!((nslpts - 3.0).abs() < 1e-12);
        assert!((avgslp - 0.058).abs() < 1e-12);
        assert!((azm - 180.0).abs() < 1e-12);
        assert!((xinput_2 - 0.6).abs() < 1e-12);
        assert!((slpinp_2 - 0.08).abs() < 1e-12);
        assert!((ofe2_avgslp - 0.0425).abs() < 1e-12);
    }

    #[test]
    fn typed_slope_projection_matches_runtime_surface_adapter() {
        let slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
            .expect("strict slope fixture should parse");
        let typed = project_typed_slope_runtime(&slope)
            .expect("typed slope runtime projection should build");
        let surface = build_hillslope_runtime_surface_from_slope(&slope)
            .expect("slope runtime surface should build");

        assert_eq!(typed.ofe_count, 2);
        assert_eq!(
            f64::from(u32::try_from(typed.ofe_count).expect("ofe count should fit u32")).to_bits(),
            soil_runtime_scalar(&surface, "nelem").to_bits()
        );
        for (ofe_offset, ofe) in typed.ofes.iter().enumerate() {
            let ofe_index = ofe_offset + 1;
            assert_eq!(
                f64::from(u32::try_from(ofe.nslpts).expect("slope point count should fit u32"))
                    .to_bits(),
                soil_runtime_scalar(&surface, &format!("ofe{ofe_index}_nslpts")).to_bits()
            );
            assert_eq!(
                ofe.slplen_m.to_bits(),
                soil_runtime_scalar(&surface, &format!("ofe{ofe_index}_slplen")).to_bits()
            );
            assert_eq!(
                ofe.avgslp.to_bits(),
                soil_runtime_scalar(&surface, &format!("ofe{ofe_index}_avgslp")).to_bits()
            );
            assert_eq!(
                ofe.azimuth_deg.to_bits(),
                soil_runtime_scalar(&surface, &format!("ofe{ofe_index}_azm")).to_bits()
            );
            for (point_offset, point) in ofe.points.iter().enumerate() {
                let point_index = point_offset + 1;
                assert_eq!(
                    point.xinput.to_bits(),
                    soil_runtime_scalar(
                        &surface,
                        &format!("ofe{ofe_index}_xinput_{point_index:04}")
                    )
                    .to_bits()
                );
                assert_eq!(
                    point.slpinp.to_bits(),
                    soil_runtime_scalar(
                        &surface,
                        &format!("ofe{ofe_index}_slpinp_{point_index:04}")
                    )
                    .to_bits()
                );
            }
        }

        let first = &typed.ofes[0];
        assert_eq!(
            first.slplen_m.to_bits(),
            soil_runtime_scalar(&surface, "slplen").to_bits()
        );
        assert_eq!(
            first.avgslp.to_bits(),
            soil_runtime_scalar(&surface, "avgslp").to_bits()
        );
        assert_eq!(
            first.azimuth_deg.to_bits(),
            soil_runtime_scalar(&surface, "azm").to_bits()
        );
    }

    #[test]
    fn slope_runtime_surface_rejects_non_positive_derived_avgslp() {
        let mut slope = parse_slope_str(SLOPE_STRICT_VALID_CANONICAL, SlopeParserOptions::strict())
            .expect("strict slope fixture should parse");

        for point in &mut slope.ofes[0].points {
            point.slpinp = 0.0;
        }

        let error = build_hillslope_runtime_surface_from_slope(&slope)
            .expect_err("non-positive derived avgslp must fail runtime adaptation");
        assert_eq!(error.code(), "HS-RUNTIME-E-023");
        assert!(matches!(
            error,
            HillslopeRuntimeInputError::NonPositiveDerivedAverageSlope {
                ofe_index: 1,
                value
            } if value.abs() < 1e-12
        ));
    }
