    #[test]
    fn typed_snow_projection_matches_runtime_surface_adapter() {
        let snow = parse_snow_from_str(
            SNOW_STRICT_VALID,
            SnowParseOptions {
                mode: SnowParseMode::Strict,
            },
        )
        .expect("strict snow fixture should parse");

        let typed = project_typed_snow_runtime(&snow)
            .expect("typed snow runtime projection should build from parsed snow");
        let surface = build_hillslope_runtime_surface_from_snow(&snow)
            .expect("snow runtime surface should build from parsed snow");

        assert_eq!(
            typed.rst_c.to_bits(),
            soil_runtime_scalar(&surface, "snow.options.rst").to_bits()
        );
        assert_eq!(
            typed.newsnw_kg_m3.to_bits(),
            soil_runtime_scalar(&surface, "snow.options.newsnw").to_bits()
        );
        assert_eq!(
            typed.ssd_kg_m3.to_bits(),
            soil_runtime_scalar(&surface, "snow.options.ssd").to_bits()
        );
        assert_eq!(
            (if typed.snow_file_present { 1.0_f64 } else { 0.0_f64 }).to_bits(),
            soil_runtime_scalar(&surface, "snow.options.snow_file_present").to_bits()
        );
        assert_eq!(
            typed.runtime_swe_m.to_bits(),
            soil_runtime_scalar(&surface, "snow.runtime_swe").to_bits()
        );
        assert_eq!(
            typed.runtime_depth_m.to_bits(),
            soil_runtime_scalar(&surface, "snow.runtime_depth_m").to_bits()
        );
        assert_eq!(
            typed.runtime_density_kg_m3.to_bits(),
            soil_runtime_scalar(&surface, "snow.runtime_density_kg_m3").to_bits()
        );
        assert_eq!(
            typed.runtime_settle_day_count.to_bits(),
            soil_runtime_scalar(&surface, "snow.runtime_settle_day_count").to_bits()
        );
    }

    #[test]
    fn typed_frost_projection_matches_runtime_surface_adapter() {
        let frost = parse_frost_from_str(FROST_STRICT_VALID_TWO_LINE, FrostParseMode::Strict)
            .expect("strict frost fixture should parse");

        let typed = project_typed_frost_runtime(&frost)
            .expect("typed frost runtime projection should build from parsed frost");
        let surface = build_hillslope_runtime_surface_from_frost(&frost)
            .expect("frost runtime surface should build from parsed frost");

        assert_eq!(
            (if typed.wint_red { 1.0_f64 } else { 0.0_f64 }).to_bits(),
            soil_runtime_scalar(&surface, "frost.options.wintRed").to_bits()
        );
        assert_eq!(
            f64::from(typed.fine_top).to_bits(),
            soil_runtime_scalar(&surface, "frost.options.fineTop").to_bits()
        );
        assert_eq!(
            f64::from(typed.fine_bot).to_bits(),
            soil_runtime_scalar(&surface, "frost.options.fineBot").to_bits()
        );
        for (field, typed_value) in [
            ("frost.options.ksnowf", typed.ksnowf),
            ("frost.options.kresf", typed.kresf),
            ("frost.options.ksoilf", typed.ksoilf),
            ("frost.options.kfactor1", typed.kfactor1),
            ("frost.options.kfactor2", typed.kfactor2),
            ("frost.options.kfactor3", typed.kfactor3),
            ("frost.runtime_dfrost", typed.dfrost_m),
            ("frost.runtime_dthaw", typed.dthaw_m),
            ("frost.runtime_nft", typed.nft),
            ("frost.runtime_ws_frz", typed.ws_frz_m),
            (
                "frost.runtime_frwatc_soil_water_before_m",
                typed.frwatc_soil_water_before_m,
            ),
            (
                "frost.runtime_frwatc_soil_water_after_m",
                typed.frwatc_soil_water_after_m,
            ),
            (
                "frost.runtime_frwatc_frozen_water_before_m",
                typed.frwatc_frozen_water_before_m,
            ),
            (
                "frost.runtime_frwatc_frozen_water_after_m",
                typed.frwatc_frozen_water_after_m,
            ),
            (
                "frost.runtime_frwatc_freeze_debit_m",
                typed.frwatc_freeze_debit_m,
            ),
            (
                "frost.runtime_frwatc_thaw_credit_m",
                typed.frwatc_thaw_credit_m,
            ),
            (
                "frost.runtime_frwatc_net_liquid_delta_m",
                typed.frwatc_net_liquid_delta_m,
            ),
            ("frost.runtime_infcap_frz", typed.infcap_frz_m_s),
            ("frost.runtime_frdp_m", typed.frdp_m),
            ("frost.runtime_thdp_m", typed.thdp_m),
            ("frost.runtime_tfrdp_m", typed.tfrdp_m),
            ("frost.runtime_tthawd_m", typed.tthawd_m),
            ("frost.runtime_fgthwd_flag", typed.fgthwd_flag),
            (
                "frost.runtime_total_fine_layer_count",
                typed.total_fine_layer_count,
            ),
            ("frost.runtime_kftill_w_m_k", typed.kftill_w_m_k),
            ("frost.runtime_kfutil_w_m_k", typed.kfutil_w_m_k),
            ("frost.runtime_kres_w_m_k", typed.kres_w_m_k),
            ("frost.runtime_residue_depth_m", typed.residue_depth_m),
        ] {
            assert_eq!(
                typed_value.to_bits(),
                soil_runtime_scalar(&surface, field).to_bits(),
                "{field}"
            );
        }
        assert_eq!(
            (if typed.frost_file_present { 1.0_f64 } else { 0.0_f64 }).to_bits(),
            soil_runtime_scalar(&surface, "frost.options.frost_file_present").to_bits()
        );
    }
