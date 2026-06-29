#[cfg(test)]
mod tests {
    use super::*;
    use crate::SidecarPolicy;
    use crate::hillslope::HillslopeDirectClimateDayForcing;
    use arrow_array::{Array, RecordBatch};
    use openwepp_hillslope_orchestrator::{
        DIRECT_PHASE_COUNT, DIRECT_R3B_PHASE_SPAN_COUNT, DIRECT_R3C_PHASE_SPAN_COUNT,
        DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT, DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT,
        DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT, DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT,
        DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT,
        DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT,
        direct_runtime_audit_snapshot,
        DIRECT_R4A_PHASE_SPAN_COUNT, DIRECT_R4B_PHASE_SPAN_COUNT, DIRECT_R4C_PHASE_SPAN_COUNT,
        DIRECT_R4G_PHASE_SPAN_COUNT, DIRECT_R4I_PHASE_SPAN_COUNT,
        DIRECT_R4J_PHASE_SPAN_COUNT, DIRECT_R4K_PHASE_SPAN_COUNT, DIRECT_R4L_PHASE_SPAN_COUNT,
        DIRECT_R4M_PHASE_SPAN_COUNT, DIRECT_R4N_PHASE_SPAN_COUNT,
        DIRECT_R4O_PHASE_SPAN_COUNT, DIRECT_R4PQZ_PHASE_SPAN_COUNT,
        reset_direct_runtime_audit_counters,
        DirectPublicationCalendarDay, DirectPublicationClimateOperands, DirectPublicationDayRow,
        DirectPublicationErosionOperands, DirectPublicationEvaporationOperands,
        DirectPublicationInterceptionOperands, DirectPublicationLiquidInputOperands,
        DirectPublicationProfileOperands, DirectPublicationRunMetadata,
        DirectPublicationRunoffOperands, DirectPublicationStorageOperands,
        DirectPublicationSubsurfaceOperands, DirectPublicationTransferOperands,
        DirectRunFrame, DirectRunIdentity, DirectRunPublicationFrame,
    };
    use openwepp_input_contract::parsers::hbp::{
        HbpParseOptions, parse_hbp_from_bytes_with_latest_event_payload, parse_hbp_from_path,
    };
    use openwepp_input_contract::parsers::slope::{
        DatverSource, DistanceMode, SlopeOfe, SlopePoint, SlopeProfile,
    };
    use openwepp_kernel_contract::{
        HillslopeConsumerAdapter, HillslopeKernel, HillslopeKernelPhaseClass,
        HillslopeKernelRequest, SymbolRegistry, WritebackField,
    };
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
    use std::fs;
    use std::path::Path;
    use std::sync::{Mutex, OnceLock};

    mod simimpl {
        include!("tests03/simimpl.rs");
    }
    mod per_ofe_state {
        include!("tests03/per_ofe_state.rs");
    }
    mod publication {
        include!("tests03/publication.rs");
    }
    mod trace {
        include!("tests03/trace.rs");
    }
    include!("tests03/direct_publication_source_guards.rs");

    fn wb11_seed_test_surface(symbols: &[(&str, f64)]) -> HillslopeWritebackSurface {
        let mut runtime_surface = HillslopeWritebackSurface::default();
        for (symbol, value) in symbols {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(*symbol), BoundaryValue::scalar(*value));
        }
        runtime_surface
    }

    fn state_field_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
        fields
            .iter()
            .find(|field| field.symbol.as_str() == symbol)
            .map(|field| field.value.as_f64())
    }

    fn flux_field_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
        fields
            .iter()
            .find(|field| field.symbol.as_str() == symbol)
            .map(|field| field.value.as_f64())
    }

    fn insert_wb11_primary_layer_lineage_symbols(
        runtime_surface: &mut HillslopeWritebackSurface,
        sat: f64,
        include_cpm: bool,
    ) {
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.25));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("wb11_nsl"), BoundaryValue::scalar(1.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("wb19_nsl"), BoundaryValue::scalar(1.0));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_dg_0001"),
            BoundaryValue::scalar(0.25),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("por_0001"),
            BoundaryValue::scalar(0.45),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_por_0001"),
            BoundaryValue::scalar(0.45),
        );
        if include_cpm {
            runtime_surface.state_surface.insert(
                BoundarySymbol::from("cpm_0001"),
                BoundaryValue::scalar(0.90),
            );
        }
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("sat"), BoundaryValue::scalar(sat));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(0.30),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_thetfc_0001"),
            BoundaryValue::scalar(0.30),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetdr_0001"),
            BoundaryValue::scalar(0.12),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_thetdr_0001"),
            BoundaryValue::scalar(0.12),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("ssc_0001"),
            BoundaryValue::scalar(2.0e-6),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
            BoundaryValue::scalar(1.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb19_drain_enabled"),
            BoundaryValue::scalar(0.0),
        );
    }


















    #[derive(Clone, Copy)]
    struct Wb16OfeSeedVector {
        avgslp: f64,
        slplen: f64,
        inrcov: f64,
        rilcov: f64,
        rrinit: f64,
        rspace: f64,
        width: f64,
        rtyp: f64,
        cancov: f64,
        canhgt: f64,
        bb_seed: f64,
        bbb_seed: f64,
        flivmx_seed: f64,
        hmax_seed: f64,
    }

    fn insert_wb16_ofe_projection_symbols(
        runtime_surface: &mut HillslopeWritebackSurface,
        ofe_index: usize,
        seed: Wb16OfeSeedVector,
    ) {
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_avgslp")),
            BoundaryValue::scalar(seed.avgslp),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_slplen")),
            BoundaryValue::scalar(seed.slplen),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_inrcov")),
            BoundaryValue::scalar(seed.inrcov),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rilcov")),
            BoundaryValue::scalar(seed.rilcov),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rrinit")),
            BoundaryValue::scalar(seed.rrinit),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rspace")),
            BoundaryValue::scalar(seed.rspace),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_width")),
            BoundaryValue::scalar(seed.width),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_rtyp")),
            BoundaryValue::scalar(seed.rtyp),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_cancov")),
            BoundaryValue::scalar(seed.cancov),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("ofe{ofe_index}_canhgt")),
            BoundaryValue::scalar(seed.canhgt),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_bb_seed")),
            BoundaryValue::scalar(seed.bb_seed),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_bbb_seed")),
            BoundaryValue::scalar(seed.bbb_seed),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_flivmx_seed")),
            BoundaryValue::scalar(seed.flivmx_seed),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_hmax_seed")),
            BoundaryValue::scalar(seed.hmax_seed),
        );
    }

    #[allow(clippy::too_many_arguments, clippy::similar_names)]
    fn wb16_expected_frcteq(
        inrcov: f64,
        rilcov: f64,
        rrinit: f64,
        rspace: f64,
        width: f64,
        flivmx_seed: f64,
        canhgt: f64,
        hmax_seed: f64,
    ) -> f64 {
        let inrfo = (3.024 - 5.042 * (-161.0 * rrinit).exp()).exp();
        let mut inrrou = 0.5 * inrfo.powf(1.128);
        if inrrou < WB16_INRFSO_CROPLAND {
            inrrou = WB16_INRFSO_CROPLAND;
        }
        let inrfro = inrrou - WB16_INRFSO_CROPLAND;
        let inrfco = if inrcov > 0.0 {
            14.5 * inrcov.powf(1.5544)
        } else {
            0.0
        };
        let frlive = if hmax_seed > 0.0 {
            (canhgt / hmax_seed) * flivmx_seed
        } else {
            0.0
        };
        let inrfto = inrfro + inrfco + WB16_INRFSO_CROPLAND + frlive;
        let frccov = if rilcov > 0.0 {
            4.5 * rilcov.powf(1.5544)
        } else {
            0.0
        };
        let frctrl = frccov + frlive + WB16_FRCSOL_CROPLAND;
        let width_ratio = width / rspace;
        if width_ratio < 1.0 {
            inrfto + width_ratio * (frctrl - inrfto)
        } else {
            inrfto
        }
    }

    fn wb16_expected_multiofe_ealpha(slplens: [f64; 2], alphas: [f64; 2], m: f64) -> f64 {
        let power2 = 1.0 / m;
        let power3 = power2 + 1.0;
        let sum_length = slplens.iter().sum::<f64>();
        let mut cumulative_length = 0.0;
        let mut storage_integral = 0.0;
        let mut last_power = 0.0;
        for (slope_length, alpha_value) in slplens.into_iter().zip(alphas) {
            cumulative_length += slope_length;
            let current_power = cumulative_length.powf(power3);
            storage_integral += (current_power - last_power) / alpha_value.powf(power2);
            last_power = current_power;
        }
        (sum_length / storage_integral).powf(m) * sum_length
    }

    #[allow(clippy::too_many_lines)]
    fn seeded_wb13_runtime_surface_probe() -> HillslopeWritebackSurface {
        let mut runtime_surface = HillslopeWritebackSurface::default();
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.004));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(12.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(2.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.25));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.25));
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetfc_0001"),
            BoundaryValue::scalar(0.30),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("thetdr_0001"),
            BoundaryValue::scalar(0.12),
        );

        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_depth_mm"),
            BoundaryValue::scalar(250.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_porosity_cap_mm"),
            BoundaryValue::scalar(120.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_fc_store_mm"),
            BoundaryValue::scalar(75.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_fc_tail_mm"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb13_profile_wp_store_mm"),
            BoundaryValue::scalar(30.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("wb11_soil_water"),
            BoundaryValue::scalar(0.075),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_ws_frz"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_frwatc_frozen_water_after_m"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_frdp_m"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_swe"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_depth_m"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.routed_melt_m"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface.flux_surface.insert(
            BoundarySymbol::from("snow.post_winter_rain_m"),
            BoundaryValue::scalar(0.0),
        );
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("S"), BoundaryValue::scalar(0.0));
        runtime_surface
            .flux_surface
            .insert(BoundarySymbol::from("I"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Irr"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Ep"), BoundaryValue::scalar(0.000_20));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Es"), BoundaryValue::scalar(0.000_10));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Er"), BoundaryValue::scalar(0.000_05));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("D"), BoundaryValue::scalar(0.000_10));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("q"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qdd"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("Qd"), BoundaryValue::scalar(0.0));
        runtime_surface
            .state_surface
            .insert(BoundarySymbol::from("SubRIn"), BoundaryValue::scalar(0.0));
        runtime_surface
    }

    fn canonical_calendar_day_probe() -> ClimateDayProjection {
        ClimateDayProjection {
            year: 2000,
            month: 1,
            day_of_month: 1,
            julian_day: 1,
            precipitation_mm: 4.0,
            effective_temperature_c: 2.0,
        }
    }

    #[test]
    fn fdhp01_wb13_publication_converts_runtime_frdp_to_wat_mm() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_frdp_m"),
            BoundaryValue::scalar(0.123),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("snow.runtime_depth_m"),
            BoundaryValue::scalar(0.456),
        );

        let wb13_row = build_simulation_owned_wb13_row(
            &runtime_surface,
            1.0,
            2000,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect("WB13 row should build from valid nonzero frost depth");
        let wat_row =
            build_hillslope_wat_row(&wb13_row).expect("WAT row should build from WB13 row");

        assert!((wb13_row.frdp_mm - 123.0).abs() < 1.0e-12);
        assert!((wat_row.frdp - 123.0).abs() < 1.0e-12);
        assert_eq!(wb13_row.snow_depth_mm, Some(456.0));
        assert_eq!(wat_row.snow_depth, Some(456.0));
    }

    #[test]
    fn fdhp01_wb13_publication_rejects_frdp_beyond_profile_depth() {
        let mut runtime_surface = seeded_wb13_runtime_surface_probe();
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("frost.runtime_frdp_m"),
            BoundaryValue::scalar(0.300),
        );

        let error = build_simulation_owned_wb13_row(
            &runtime_surface,
            1.0,
            2000,
            1,
            &canonical_calendar_day_probe(),
            0.0,
        )
        .expect_err("WAT publication must reject frost depth beyond physical profile");

        assert!(
            error.to_string().contains("frost.runtime_frdp_m must be <="),
            "unexpected error: {error}"
        );
    }

    fn execute_fixture_run(prefix: &str) -> (HillslopeRunReport, PathBuf) {
        execute_fixture_run_with_runtime_selection(prefix, HillslopeRuntimeSelection::Compatibility)
    }

    fn execute_fixture_run_with_runtime_selection(
        prefix: &str,
        runtime_selection: HillslopeRuntimeSelection,
    ) -> (HillslopeRunReport, PathBuf) {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        execute_fixture_run_with_runtime_selection_unlocked(prefix, runtime_selection)
    }

    fn execute_fixture_run_with_runtime_selection_unlocked(
        prefix: &str,
        runtime_selection: HillslopeRuntimeSelection,
    ) -> (HillslopeRunReport, PathBuf) {
        execute_fixture_run_with_runtime_policy_unlocked(
            prefix,
            HillslopeRuntimeSelectionPolicy::new(
                runtime_selection,
                HillslopeDefaultRuntimeActivation::Disabled,
            ),
        )
    }

    fn execute_fixture_run_with_runtime_policy_unlocked(
        prefix: &str,
        runtime_policy: HillslopeRuntimeSelectionPolicy,
    ) -> (HillslopeRunReport, PathBuf) {
        reset_direct_runtime_audit_counters();

        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);
        let output_dir = temp_run_dir.join("output");

        let report = execute_hillslope_run_with_runtime_policy(
            &HillslopeRunRequest {
                run_dir: temp_run_dir.clone(),
                run_file: PathBuf::from("case.run"),
                output_dir,
                sidecar_policy: SidecarPolicy::Compat,
                legacy_sidecar_discovery: false,
                manifest_path: None,
            },
            &["openwepp-cli-hill".to_string()],
            runtime_policy,
        )
        .expect("fixture run should complete");

        (report, temp_run_dir)
    }

    fn r5c_day_span_run_count() -> u64 {
        22
    }

    fn r5c_day_phase_entry_count() -> u64 {
        (DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT
            + DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT
            + DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT
            + DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT
            + DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT
            + DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT
            + DIRECT_R4A_PHASE_SPAN_COUNT
            + 3
            + DIRECT_R4B_PHASE_SPAN_COUNT
            + DIRECT_R4C_PHASE_SPAN_COUNT
            + DIRECT_R4G_PHASE_SPAN_COUNT
            + DIRECT_R4I_PHASE_SPAN_COUNT
            + DIRECT_R4J_PHASE_SPAN_COUNT
            + DIRECT_R4K_PHASE_SPAN_COUNT
            + DIRECT_R4L_PHASE_SPAN_COUNT
            + DIRECT_R4M_PHASE_SPAN_COUNT
            + DIRECT_R4N_PHASE_SPAN_COUNT
            + DIRECT_R4O_PHASE_SPAN_COUNT
            + DIRECT_R4PQZ_PHASE_SPAN_COUNT
            + 3
            + DIRECT_R3B_PHASE_SPAN_COUNT) as u64
    }

    #[test]
    fn r7e_default_candidate_disabled_uses_compatibility_rollback_manifest() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (report, _temp_run_dir) = execute_fixture_run_with_runtime_policy_unlocked(
            "r7e_default_candidate_disabled",
            HillslopeRuntimeSelectionPolicy::default(),
        );

        assert!(report.output_pass.is_file());
        assert!(report.output_loss.is_file());
        let manifest_json = read_manifest_json(&report);
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/requested")
                .and_then(serde_json::Value::as_str),
            Some("default-candidate")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/selected")
                .and_then(serde_json::Value::as_str),
            Some("compatibility")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/fallback_reason")
                .and_then(serde_json::Value::as_str),
            Some("direct-default-candidate-gate-disabled")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/output_policy")
                .and_then(serde_json::Value::as_str),
            Some("compatibility-public-output")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/compatibility_rollback_available")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/scheduler_kernel_executed")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert!(
            manifest_json.pointer("/direct_runtime_counters").is_none(),
            "disabled default candidate must not construct direct runtime counters"
        );
        let audit = direct_runtime_audit_snapshot();
        assert_eq!(audit.run_frame_constructions, 0);
        assert_eq!(audit.executor_constructions, 0);
        assert_eq!(audit.compatibility_edge_invocations, 0);
    }

    #[test]
    fn r7e_default_candidate_activation_selects_direct_runtime_manifest() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (report, _temp_run_dir) = execute_fixture_run_with_runtime_policy_unlocked(
            "r7e_default_candidate_activation",
            HillslopeRuntimeSelectionPolicy::new(
                HillslopeRuntimeSelection::DefaultCandidate,
                HillslopeDefaultRuntimeActivation::DirectProductionCandidate,
            ),
        );

        assert!(report.output_pass.is_file());
        assert!(report.output_loss.is_file());
        let manifest_json = read_manifest_json(&report);
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/requested")
                .and_then(serde_json::Value::as_str),
            Some("default-candidate")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/selected")
                .and_then(serde_json::Value::as_str),
            Some("direct-production-executor")
        );
        assert_eq!(
            manifest_json
                .pointer("/runtime_selection/default_activation_gate")
                .and_then(serde_json::Value::as_str),
            Some("direct-production-candidate")
        );
        assert!(
            manifest_json
                .pointer("/runtime_selection/fallback_reason")
                .is_none()
                || manifest_json
                    .pointer("/runtime_selection/fallback_reason")
                    .is_some_and(serde_json::Value::is_null),
            "activated default candidate should not report a fallback reason"
        );
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/scheduler_kernel_executed")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/compatibility_edge_invocations",
            0,
        );
    }

    #[test]
    fn r7e_runtime_selection_policy_resolves_default_direct_and_rollback_modes() {
        let default_disabled = HillslopeRuntimeSelectionPolicy::default().resolve();
        assert_eq!(
            default_disabled.requested(),
            HillslopeRuntimeSelection::DefaultCandidate
        );
        assert_eq!(
            default_disabled.selected(),
            HillslopeRuntimeSelection::Compatibility
        );
        assert_eq!(
            default_disabled.fallback_reason(),
            Some("direct-default-candidate-gate-disabled")
        );

        let default_activated = HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DefaultCandidate,
            HillslopeDefaultRuntimeActivation::DirectProductionCandidate,
        )
        .resolve();
        assert_eq!(
            default_activated.selected(),
            HillslopeRuntimeSelection::DirectProductionExecutor
        );
        assert_eq!(default_activated.fallback_reason(), None);

        let explicit_rollback = HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::Compatibility,
            HillslopeDefaultRuntimeActivation::Disabled,
        )
        .resolve();
        assert_eq!(
            explicit_rollback.selection_reason(),
            "explicit-compatibility-rollback"
        );
        assert_eq!(
            explicit_rollback.selected(),
            HillslopeRuntimeSelection::Compatibility
        );

        let explicit_shadow = HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectPublicationFrameShadow,
            HillslopeDefaultRuntimeActivation::Disabled,
        )
        .resolve();
        assert_eq!(
            explicit_shadow.selected(),
            HillslopeRuntimeSelection::DirectPublicationFrameShadow
        );
    }

    #[test]
    fn r2a_default_fixture_run_constructs_no_direct_runtime_skeleton() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (report, _temp_run_dir) = execute_fixture_run_with_runtime_selection_unlocked(
            "r2a_default_direct_skeleton_disabled",
            HillslopeRuntimeSelection::Compatibility,
        );

        assert!(report.output_pass.is_file());
        assert!(report.output_loss.is_file());
        let audit = direct_runtime_audit_snapshot();
        assert_eq!(audit.run_frame_constructions, 0);
        assert_eq!(audit.day_frame_constructions, 0);
        assert_eq!(audit.day_frame_commits, 0);
        assert_eq!(audit.executor_constructions, 0);
        assert_eq!(audit.skeleton_runs, 0);
        assert_eq!(audit.publication_capture_runs, 0);
        assert_eq!(audit.phase_view_constructions, 0);
        assert_eq!(audit.phase_span_runs, 0);
        assert_eq!(audit.direct_phase_entries, 0);
        assert_eq!(audit.direct_compute_operations, 0);
        assert_eq!(audit.direct_state_mutations, 0);
        assert_eq!(audit.downstream_operand_productions, 0);
        assert_eq!(audit.shadow_projections, 0);
        assert_eq!(audit.compatibility_edge_invocations, 0);
    }

    #[test]
    fn r2a_explicit_direct_skeleton_selection_runs_before_compatibility_outputs() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (report, _temp_run_dir) = execute_fixture_run_with_runtime_selection_unlocked(
            "r2a_direct_skeleton_opt_in",
            HillslopeRuntimeSelection::DirectSkeletonNoop,
        );

        assert!(report.output_pass.is_file());
        assert!(report.output_loss.is_file());
        let manifest_json = read_manifest_json(&report);
        let expected_day_frames = manifest_json
            .pointer("/execution_provenance/climate_day_count")
            .and_then(serde_json::Value::as_u64)
            .expect("fixture manifest should include climate day count for R5A");
        let audit = direct_runtime_audit_snapshot();
        assert_eq!(audit.run_frame_constructions, 1);
        assert_eq!(audit.executor_constructions, 1);
        assert_eq!(audit.skeleton_runs, 1);
        assert_eq!(audit.publication_capture_runs, 0);
        assert_eq!(audit.day_frame_constructions, expected_day_frames);
        assert_eq!(audit.day_frame_commits, expected_day_frames);
        assert_eq!(
            audit.phase_view_constructions,
            expected_day_frames * DIRECT_PHASE_COUNT as u64
        );
        assert_eq!(
            audit.phase_span_runs,
            1 + expected_day_frames * r5c_day_span_run_count()
        );
        assert_eq!(
            audit.direct_phase_entries,
            DIRECT_R3C_PHASE_SPAN_COUNT as u64
                + expected_day_frames * r5c_day_phase_entry_count()
        );
        assert_eq!(
            audit.direct_compute_operations,
            1 + expected_day_frames * r5c_day_span_run_count()
        );
        assert_eq!(
            audit.direct_state_mutations,
            1 + expected_day_frames * r5c_day_span_run_count()
        );
        assert_eq!(
            audit.downstream_operand_productions,
            1 + expected_day_frames * r5c_day_span_run_count()
        );
        assert_eq!(
            audit.shadow_projections,
            1 + expected_day_frames * r5c_day_span_run_count()
        );
        assert_eq!(audit.compatibility_edge_invocations, 1);
    }

    #[test]
    fn r6a_direct_publication_frame_shadow_runs_without_skeleton_counter() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (report, _temp_run_dir) = execute_fixture_run_with_runtime_selection_unlocked(
            "r6a_direct_publication_frame_shadow",
            HillslopeRuntimeSelection::DirectPublicationFrameShadow,
        );

        assert!(report.output_pass.is_file());
        assert!(report.output_loss.is_file());
        let manifest_json = read_manifest_json(&report);
        let expected_day_frames = manifest_json
            .pointer("/execution_provenance/climate_day_count")
            .and_then(serde_json::Value::as_u64)
            .expect("fixture manifest should include climate day count for R6A");
        let audit = direct_runtime_audit_snapshot();
        assert_eq!(audit.run_frame_constructions, 1);
        assert_eq!(audit.executor_constructions, 1);
        assert_eq!(audit.skeleton_runs, 0);
        assert_eq!(audit.publication_capture_runs, 1);
        assert_eq!(audit.day_frame_constructions, expected_day_frames);
        assert_eq!(audit.day_frame_commits, expected_day_frames);
        assert_eq!(
            audit.phase_view_constructions,
            expected_day_frames * DIRECT_PHASE_COUNT as u64
        );
        assert_eq!(audit.compatibility_edge_invocations, 0);
    }

    #[test]
    fn r6j_cutover_candidate_writes_direct_outputs_and_manifest() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        reset_direct_runtime_audit_counters();

        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(
            &source_fixture_dir,
            "r6_direct_publication_cutover_candidate",
        );
        enable_r6j_pass_parquet_output(&temp_run_dir);
        let output_dir = temp_run_dir.join("output");
        let report = execute_hillslope_run_with_runtime_selection(
            &HillslopeRunRequest {
                run_dir: temp_run_dir,
                run_file: PathBuf::from("case.run"),
                output_dir: output_dir.clone(),
                sidecar_policy: SidecarPolicy::Compat,
                legacy_sidecar_discovery: false,
                manifest_path: None,
            },
            &["openwepp-cli-hill".to_string()],
            HillslopeRuntimeSelection::DirectPublicationFrameCutover,
        )
        .expect("R6J cutover candidate should write direct publication outputs");
        let audit = direct_runtime_audit_snapshot();
        assert_eq!(audit.run_frame_constructions, 0);
        assert_eq!(audit.executor_constructions, 0);
        assert_eq!(audit.skeleton_runs, 0);
        assert_eq!(audit.publication_capture_runs, 0);
        assert_eq!(audit.day_frame_constructions, 0);
        assert_eq!(audit.day_frame_constructions, audit.day_frame_commits);
        assert_eq!(audit.direct_compute_operations, 0);
        assert_eq!(audit.direct_state_mutations, 0);
        assert_eq!(audit.downstream_operand_productions, 0);
        assert_eq!(audit.shadow_projections, 0);
        assert_eq!(audit.compatibility_edge_invocations, 0);
        assert_r6j_cutover_report_outputs(&report, &output_dir);
    }

    fn assert_r6j_cutover_report_outputs(report: &HillslopeRunReport, output_dir: &Path) {
        for output_name in [
            "H5.hbp",
            "H5.loss.json",
            "H5.pass.parquet",
            "H5.wat.parquet",
            "H5.plot.parquet",
            "openwepp_hillslope_run_manifest.json",
        ] {
            assert!(
                output_dir.join(output_name).is_file(),
                "R6J cutover candidate must write {output_name}"
            );
        }
        let manifest_json = read_manifest_json(report);
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/publication_source")
                .and_then(serde_json::Value::as_str),
            Some("direct-publication-frame")
        );
        assert_eq!(
            manifest_json
                .pointer("/wb13_publication/source")
                .and_then(serde_json::Value::as_str),
            Some("direct-publication-frame")
        );
        assert_eq!(
            manifest_json
                .pointer("/wb13_publication/replay_candidate_surfaces")
                .and_then(serde_json::Value::as_array)
                .map(Vec::len),
            Some(0)
        );
        assert_json_i64(&manifest_json, "/wb13_publication/row_count", 2);
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/run_frame_constructions",
            0,
        );
        assert_json_i64(&manifest_json, "/direct_runtime_counters/skeleton_runs", 0);
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/publication_capture_runs",
            0,
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/compatibility_edge_invocations",
            0,
        );
        let output_checksums = manifest_json
            .pointer("/output_checksums")
            .and_then(serde_json::Value::as_object)
            .expect("manifest must include output checksum object");
        for output_path in std::iter::once(report.output_pass.as_path())
            .chain(std::iter::once(report.output_loss.as_path()))
            .chain(report.optional_outputs.iter().map(PathBuf::as_path))
        {
            assert_manifest_checksum_matches_file(output_checksums, output_path);
        }
    }

    #[test]
    fn r6j_manifest_direct_runtime_counters_are_run_local_after_prior_activity() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        reset_direct_runtime_audit_counters();
        let (_shadow_report, _shadow_run_dir) =
            execute_fixture_run_with_runtime_selection_unlocked(
                "r6j_dirty_direct_counter_shadow",
                HillslopeRuntimeSelection::DirectPublicationFrameShadow,
            );
        let dirty_audit = direct_runtime_audit_snapshot();
        assert_eq!(dirty_audit.publication_capture_runs, 1);

        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(
            &source_fixture_dir,
            "r6j_run_local_counter_cutover",
        );
        enable_r6j_pass_parquet_output(&temp_run_dir);
        let output_dir = temp_run_dir.join("output");
        let report = execute_hillslope_run_with_runtime_selection(
            &HillslopeRunRequest {
                run_dir: temp_run_dir,
                run_file: PathBuf::from("case.run"),
                output_dir,
                sidecar_policy: SidecarPolicy::Compat,
                legacy_sidecar_discovery: false,
                manifest_path: None,
            },
            &["openwepp-cli-hill".to_string()],
            HillslopeRuntimeSelection::DirectPublicationFrameCutover,
        )
        .expect("R6J cutover should run after prior direct runtime activity");

        let cumulative_audit = direct_runtime_audit_snapshot();
        assert_eq!(cumulative_audit.publication_capture_runs, 1);
        let manifest_json = read_manifest_json(&report);
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/run_frame_constructions",
            0,
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/publication_capture_runs",
            0,
        );
        assert_json_i64(&manifest_json, "/direct_runtime_counters/skeleton_runs", 0);
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/compatibility_edge_invocations",
            0,
        );
    }

    #[test]
    fn r7c_direct_production_executor_reports_no_day_input_compatibility_edges() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (report, _temp_run_dir) = execute_fixture_run_with_runtime_selection_unlocked(
            "r7c_direct_production_executor",
            HillslopeRuntimeSelection::DirectProductionExecutor,
        );

        assert!(report.output_pass.is_file());
        assert!(report.output_loss.is_file());
        let manifest_json = read_manifest_json(&report);
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/scheduler_kernel_executed")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/publication_source")
                .and_then(serde_json::Value::as_str),
            Some("direct-publication-frame")
        );
        assert_eq!(
            manifest_json
                .pointer("/execution_provenance/scheduler_status_message_id")
                .and_then(serde_json::Value::as_str),
            Some("R7C-DIRECT-PRODUCTION-EXECUTOR")
        );
        let expected_day_frames = manifest_json
            .pointer("/execution_provenance/climate_day_count")
            .and_then(serde_json::Value::as_u64)
            .expect("R7C fixture manifest must include direct climate day count");
        let expected_phase_spans = 1 + expected_day_frames * r5c_day_span_run_count();
        let expected_phase_entries =
            DIRECT_R3C_PHASE_SPAN_COUNT as u64 + expected_day_frames * r5c_day_phase_entry_count();
        assert_json_i64(&manifest_json, "/direct_runtime_counters/run_frame_constructions", 1);
        assert_json_i64(&manifest_json, "/direct_runtime_counters/executor_constructions", 1);
        assert_json_i64(&manifest_json, "/direct_runtime_counters/skeleton_runs", 0);
        assert_json_i64(&manifest_json, "/direct_runtime_counters/publication_capture_runs", 1);
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/day_frame_constructions",
            i64::try_from(expected_day_frames).expect("fixture day frame count fits i64"),
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/day_frame_commits",
            i64::try_from(expected_day_frames).expect("fixture day frame count fits i64"),
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/phase_span_runs",
            i64::try_from(expected_phase_spans).expect("fixture phase span count fits i64"),
        );
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/direct_phase_entries",
            i64::try_from(expected_phase_entries).expect("fixture phase entry count fits i64"),
        );
        for counter in [
            "/direct_runtime_counters/direct_compute_operations",
            "/direct_runtime_counters/direct_state_mutations",
            "/direct_runtime_counters/downstream_operand_productions",
            "/direct_runtime_counters/shadow_projections",
        ] {
            let value = manifest_json
                .pointer(counter)
                .and_then(serde_json::Value::as_i64)
                .unwrap_or_else(|| panic!("manifest must include {counter}"));
            assert!(value > 0, "{counter} must be nonzero for R7C direct production");
        }
        assert_json_i64(
            &manifest_json,
            "/direct_runtime_counters/compatibility_edge_invocations",
            0,
        );
    }

    #[test]
    fn r7d2_direct_seed_authority_is_lane_indexed_for_multiofe_profiles() {
        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(
            &source_fixture_dir,
            "r7d2_lane_indexed_seed_authority",
        );
        let request = HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir: temp_run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };
        let inputs = load_hillslope_run_inputs(&request).expect("fixture inputs should parse");
        let climate_request = build_hillslope_climate_runtime_request(&inputs.climate)
            .expect("fixture climate request should build");
        let climate_span =
            build_climate_run_span_summary(&inputs.climate).expect("climate span should build");
        let seed_surfaces = vec![
            r7d2_profile_seed_surface(500.0, 520.0, 90.0, 0.30, 0.0),
            r7d2_profile_seed_surface(640.0, 690.0, 120.0, 0.34, 2.0),
        ];
        let climate_context_surface = seed_surfaces
            .first()
            .expect("fixture has at least one seed surface")
            .clone();
        let builder = DirectPublicationDayInputBuilder::new_with_seed_surfaces(
            &climate_request,
            &climate_span,
            seed_surfaces,
            &climate_context_surface,
            ExecutionLane::Daily,
        )
        .expect("lane-indexed direct day-input builder should construct");

        let lane_one_profile = builder
            .profile_inputs(0)
            .expect("lane 1 profile seed should exist");
        let lane_two_profile = builder
            .profile_inputs(1)
            .expect("lane 2 profile seed should exist");

        assert_ne!(
            lane_one_profile.profile_depth_m,
            lane_two_profile.profile_depth_m,
            "R7D2 must not alias lane 2 profile depth to lane 1"
        );
        assert_ne!(
            lane_one_profile.profile_porosity_cap_m,
            lane_two_profile.profile_porosity_cap_m,
            "R7D2 must not alias lane 2 porosity storage to lane 1"
        );
        assert_ne!(
            lane_one_profile.profile_field_capacity_m,
            lane_two_profile.profile_field_capacity_m,
            "R7D2 must not alias lane 2 field-capacity storage to lane 1"
        );
        assert_ne!(
            lane_one_profile.profile_wilting_point_m,
            lane_two_profile.profile_wilting_point_m,
            "R7D2 must not alias lane 2 wilting-point storage to lane 1"
        );
    }

    #[test]
    fn r6f_wat_hold_marker_is_reserved_for_exact_producer_gap_fields() {
        let compatibility = r6f_wat_marker_sample_row();
        let mut direct = compatibility.clone();
        direct.wepp_id = 19;
        direct.year = 2026;
        direct.es = 0.0;
        direct.total_soil_water = 0.0;
        direct.soil_water_total = None;
        direct.profile_depth = None;
        direct.profile_porosity_cap = None;
        direct.profile_fc_store = None;
        direct.profile_wp_store = None;

        let reduced_fields = reduced_wat_mismatch_fields(
            std::slice::from_ref(&direct),
            std::slice::from_ref(&compatibility),
        );
        assert_eq!(
            reduced_fields,
            vec![
                "wepp_id",
                "year",
                "Es",
                "Total-Soil",
                "SoilWaterTotal",
                "ProfileDepth",
                "ProfilePorosityCap",
                "ProfileFCStore",
                "ProfileWPStore",
            ]
        );
        assert!(r6f_wat_direct_process_producer_authority_gap(
            &reduced_fields
        ));

        let mut unrelated_direct = direct;
        unrelated_direct.p = 9.5;
        let unrelated_fields =
            reduced_wat_mismatch_fields(&[unrelated_direct], &[compatibility]);
        assert!(unrelated_fields.contains(&"P"));
        assert!(!r6f_wat_direct_process_producer_authority_gap(
            &unrelated_fields
        ));
    }

    #[test]
    fn r6g_wat_hold_marker_is_reserved_for_exact_pmet_day_state_carry_fields() {
        let compatibility = r6f_wat_marker_sample_row();
        let mut direct = compatibility.clone();
        direct.es = 0.9;
        direct.total_soil_water = 99.9;
        direct.soil_water_total = Some(99.9);

        let reduced_fields = reduced_wat_mismatch_fields(
            std::slice::from_ref(&direct),
            std::slice::from_ref(&compatibility),
        );
        assert_eq!(reduced_fields, vec!["Es", "Total-Soil", "SoilWaterTotal"]);
        assert!(r6g_wat_direct_et_storage_producer_gap(&reduced_fields));
        assert!(!r6g_wat_pmet_day_state_carry_gap(
            std::slice::from_ref(&direct),
            std::slice::from_ref(&compatibility),
            &reduced_fields
        ));

        let mut unrelated_direct = direct;
        unrelated_direct.dp = 2.0;
        let unrelated_fields =
            reduced_wat_mismatch_fields(&[unrelated_direct], &[compatibility]);
        assert!(unrelated_fields.contains(&"Dp"));
        assert!(!r6g_wat_direct_et_storage_producer_gap(
            &unrelated_fields
        ));
    }

    #[test]
    fn r6h_wat_hold_marker_is_reserved_for_exact_pmet_layer_ulp_gap() {
        let compatibility_first = r6f_wat_marker_sample_row();
        let direct_first = compatibility_first.clone();
        let mut compatibility_second = compatibility_first.clone();
        compatibility_second.sim_day_index = 2;
        compatibility_second.es = 0.767_760_184_372_260_8;
        let mut direct_second = compatibility_second.clone();
        direct_second.es = 0.767_760_184_372_260_5;

        let direct_rows = [direct_first, direct_second.clone()];
        let compatibility_rows = [compatibility_first, compatibility_second.clone()];
        let reduced_fields = reduced_wat_mismatch_fields(&direct_rows, &compatibility_rows);
        assert_eq!(reduced_fields, vec!["Es"]);
        assert!(r6h_wat_pmet_layer_carry_ulp_gap(
            &direct_rows,
            &compatibility_rows,
            &reduced_fields
        ));

        direct_second.es = 0.5;
        let direct_rows = [direct_rows[0].clone(), direct_second];
        let reduced_fields = reduced_wat_mismatch_fields(&direct_rows, &compatibility_rows);
        assert_eq!(reduced_fields, vec!["Es"]);
        assert!(!r6h_wat_pmet_layer_carry_ulp_gap(
            &direct_rows,
            &compatibility_rows,
            &reduced_fields
        ));

        let mut direct_first = compatibility_rows[0].clone();
        direct_first.es += 1.0e-13;
        let direct_rows = [direct_first, compatibility_rows[1].clone()];
        let reduced_fields = reduced_wat_mismatch_fields(&direct_rows, &compatibility_rows);
        assert_eq!(reduced_fields, vec!["Es"]);
        assert!(!r6h_wat_pmet_layer_carry_ulp_gap(
            &direct_rows,
            &compatibility_rows,
            &reduced_fields
        ));

        let mut compatibility_third = compatibility_rows[1].clone();
        compatibility_third.sim_day_index = 3;
        compatibility_third.es = 0.9;
        let mut direct_second = compatibility_rows[1].clone();
        direct_second.es = 0.767_760_184_372_260_5;
        let mut direct_third = compatibility_third.clone();
        direct_third.es = 0.5;
        let direct_rows = [
            compatibility_rows[0].clone(),
            direct_second,
            direct_third,
        ];
        let compatibility_rows = [
            compatibility_rows[0].clone(),
            compatibility_rows[1].clone(),
            compatibility_third,
        ];
        let reduced_fields = reduced_wat_mismatch_fields(&direct_rows, &compatibility_rows);
        assert_eq!(reduced_fields, vec!["Es"]);
        assert!(!r6h_wat_pmet_layer_carry_ulp_gap(
            &direct_rows,
            &compatibility_rows,
            &reduced_fields
        ));
    }

    #[test]
    fn r6h_day_input_overlay_requires_committed_layers_after_day_zero() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let identity =
            DirectRunIdentity::new(1, 1, 1, 2).expect("valid identity should construct");
        let frame = DirectRunFrame::skeleton(identity).expect("skeleton frame should construct");
        let lane = frame.lanes.first().expect("skeleton frame should include lane");
        let mut day_zero_seed = HillslopeWritebackSurface::default();
        overlay_direct_publication_lane_state(&mut day_zero_seed, 0, 0, lane)
            .expect("day-zero static seed may start before direct layers commit");

        let mut later_day_seed = HillslopeWritebackSurface::default();
        let error = overlay_direct_publication_lane_state(&mut later_day_seed, 1, 0, lane)
            .expect_err("later-day seed requires direct-carried layers");
        assert!(
            error
                .to_string()
                .contains("requires committed direct-carried layers"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn r7h_growth_overlay_replaces_stale_interception_plant_aliases() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("wb12_rainfall_input", 0.010),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 3_600.0),
            ("intsty_0001", 0.010 / 3_600.0),
            ("sumgdd", 1.0),
            ("cancov", 0.1),
            ("lai", 0.1),
            ("vdmt", 0.01),
            ("rtmass", 0.02),
            ("rtd", 0.03),
            ("hia", 0.0),
            ("I", 0.009),
        ]);
        let carried_growth = openwepp_hillslope_orchestrator::DirectGrowthStateSurface {
            sumgdd: 12.0,
            live_biomass_kg_m2: 0.2,
            interception_live_biomass_kg_m2: 0.18,
            canopy_cover_fraction: 0.5,
            leaf_area_index: 1.0,
            root_mass_kg_m2: 0.25,
            root_depth_m: 0.30,
            harvest_index: 0.1,
        };

        overlay_direct_publication_growth_state(&mut runtime_surface, 0, carried_growth)
            .expect("carried growth state should overlay stale plant aliases");

        let hyetograph = direct_publication_hyetograph(&runtime_surface)
            .expect("hyetograph should be available for direct WB15");
        let interception = direct_publication_interception_state(&runtime_surface, 0.010, &hyetograph)
            .expect("interception should consume overlaid carried growth state");
        let expected_interception_m =
            0.5 * ((0.000_627 * 1_800.0 - 3.733_49e-8 * 1_800.0_f64.powi(2)) / 1_000.0);
        let stale_interception_m =
            0.1 * ((0.000_627 * 100.0 - 3.733_49e-8 * 100.0_f64.powi(2)) / 1_000.0);

        assert!((interception.interception_m - expected_interception_m).abs() < 1.0e-12);
        assert_ne!(
            interception.interception_m.to_bits(),
            stale_interception_m.to_bits(),
            "direct WB15 must not consume stale static plant aliases after day-zero"
        );
    }

    #[test]
    fn r7h_growth_authority_selects_current_ofe_slot_not_primary_ofe() {
        let mut day = canonical_calendar_day_probe();
        day.julian_day = 165;
        let forcing = r7g_snow_forcing(20.0, 10.0);
        let authority = DirectProductionGrowthAuthority {
            active: true,
            rotation_years: 1,
            rotation_repeats: 1,
            slots: vec![
                DirectProductionGrowthSlotAuthority {
                    ofe_index: 1,
                    year_in_rotation: 1,
                    rotation_index: 1,
                    crops: vec![r7h_growth_crop_with_bb(0.11)],
                },
                DirectProductionGrowthSlotAuthority {
                    ofe_index: 2,
                    year_in_rotation: 1,
                    rotation_index: 1,
                    crops: vec![r7h_growth_crop_with_bb(0.22)],
                },
            ],
            monthly_temperature_max_c: [20.0; 12],
            monthly_temperature_min_c: [10.0; 12],
            soil_depth_m: 1.0,
        };

        let (annual_inputs, perennial_inputs) = authority
            .inputs(
                &day,
                1,
                2,
                &forcing,
                openwepp_hillslope_orchestrator::DirectGrowthStateSurface {
                    sumgdd: 1.0,
                    live_biomass_kg_m2: 0.1,
                    interception_live_biomass_kg_m2: 0.1,
                    canopy_cover_fraction: 0.2,
                    leaf_area_index: 0.3,
                    root_mass_kg_m2: 0.4,
                    root_depth_m: 0.5,
                    harvest_index: 0.0,
                },
                1.0,
                &DirectEvapotranspirationComputeInputs::zero(),
            )
            .expect("OFE-specific active growth selection should succeed");

        assert_eq!(annual_inputs.bb.to_bits(), 0.22_f64.to_bits());
        assert_eq!(
            perennial_inputs.active_context,
            DirectGrowthActiveContext::Inactive
        );
    }

    #[test]
    fn r7h_growth_authority_accepts_unambiguous_lane_local_slot() {
        let mut day = canonical_calendar_day_probe();
        day.julian_day = 165;
        let forcing = r7g_snow_forcing(20.0, 10.0);
        let authority = DirectProductionGrowthAuthority {
            active: true,
            rotation_years: 1,
            rotation_repeats: 1,
            slots: vec![DirectProductionGrowthSlotAuthority {
                ofe_index: 1,
                year_in_rotation: 1,
                rotation_index: 1,
                crops: vec![r7h_growth_crop_with_bb(0.33)],
            }],
            monthly_temperature_max_c: [20.0; 12],
            monthly_temperature_min_c: [10.0; 12],
            soil_depth_m: 1.0,
        };

        let (annual_inputs, _) = authority
            .inputs(
                &day,
                1,
                4,
                &forcing,
                openwepp_hillslope_orchestrator::DirectGrowthStateSurface {
                    sumgdd: 1.0,
                    live_biomass_kg_m2: 0.1,
                    interception_live_biomass_kg_m2: 0.1,
                    canopy_cover_fraction: 0.2,
                    leaf_area_index: 0.3,
                    root_mass_kg_m2: 0.4,
                    root_depth_m: 0.5,
                    harvest_index: 0.0,
                },
                1.0,
                &DirectEvapotranspirationComputeInputs::zero(),
            )
            .expect("lane-local OFE-1 authority should apply to its lane");

        assert_eq!(annual_inputs.bb.to_bits(), 0.33_f64.to_bits());
    }

    fn r7h_growth_crop_with_bb(bb: f64) -> DirectProductionGrowthCropAuthority {
        DirectProductionGrowthCropAuthority {
            schedule_imngmt: 1,
            imngmt: 1,
            jdharv: 300,
            jdplt: 1,
            jdstop: 0,
            btemp: 0.0,
            otemp: 30.0,
            gddmax: 1_000.0,
            dlai: 0.8,
            dropfc: 1.0,
            decfct: 1.0,
            spriod: 1.0,
            bb,
            beinp: 1.0,
            extnct: 0.5,
            hi: 0.4,
            xmxlai: 3.0,
            rsr: 0.2,
            rtmmax: 1.0,
            rdmax: 1.0,
        }
    }

    fn r6f_wat_marker_sample_row() -> HillslopeWatRow {
        HillslopeWatRow {
            wepp_id: 1,
            ofe_id: 1,
            year: 1987,
            sim_day_index: 1,
            julian: 1,
            month: 1,
            day_of_month: 1,
            water_year: 1987,
            ofe: 1,
            p: 10.0,
            rm: 0.0,
            q: 0.0,
            ep: 1.0,
            es: 1.0,
            er: 0.1,
            dp: 0.1,
            up_strm_q: 0.0,
            sub_r_in: 0.0,
            latqcc: 0.0,
            total_soil_water: 100.0,
            frozwt: 0.0,
            frdp: 0.0,
            snow_water: 0.0,
            snow_depth: None,
            meltwater_temperature: None,
            qofe: 0.0,
            tile: 0.0,
            irr: 0.0,
            area: 1.0,
            soil_water_total: Some(100.0),
            profile_depth: Some(1_200.0),
            profile_porosity_cap: Some(400.0),
            profile_fc_store: Some(300.0),
            profile_wp_store: Some(150.0),
            interception: Some(0.25),
            interception_storage: None,
        }
    }

    #[test]
    fn r6j_cutover_parity_evidence_covers_hbp_wat_pass_and_loss() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        reset_direct_runtime_audit_counters();
        let evidence = r6f_hbp_wat_fixture_evidence();

        assert_r6f_hbp_identity(&evidence);
        assert_r6i_wat_identity(&evidence);
        assert_r6j_pass_identity(&evidence);
        assert_r6j_loss_identity(&evidence);
        assert_r6j_parquet_disk_identity(&evidence);
    }

    #[test]
    fn r7d_direct_day_two_pmet_seed_keeps_direct_wb14_lineage_boundary() {
        let _execution_guard = runner_execution_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        reset_direct_runtime_audit_counters();

        let direct_evidence = r7d_direct_day_two_seed_evidence();
        let compatibility_execution = r6i_compatibility_fixture_execution();
        let compatibility_day_one = compatibility_execution
            .wb13_rows
            .iter()
            .find(|row| row.sim_day_index == 1 && row.wb13_row.ofe == 1)
            .expect("fixture compatibility day-1 row should exist");

        assert!(
            direct_evidence.day_one_runoff_mm < compatibility_day_one.wb13_row.q,
            "direct WB14 lineage must not be forced through compatibility day-1 stale infiltration"
        );

        let top_theta_m =
            runtime_surface_symbol_value(&direct_evidence.day_two_seed_surface, "wb18_perc_theta_0001")
                .expect("direct day-2 seed must carry top-layer theta");
        let direct_wfevp_mm =
            runtime_surface_symbol_value(&direct_evidence.day_two_seed_surface, "pmet.wfevp_mm")
                .expect("direct day-2 seed must publish PMET wfevp");
        assert!(
            (direct_wfevp_mm - top_theta_m * 500.0).abs() < 1.0e-12,
            "direct PMET wfevp must be reconstructed from the direct carried top layer"
        );

        for symbol in ["pmet.wfevp_mm", "pmet.etkr", "pmet.es_m"] {
            let direct =
                runtime_surface_symbol_value(&direct_evidence.day_two_seed_surface, symbol)
                .unwrap_or_else(|| panic!("direct day-2 seed must publish {symbol}"));
            let compatibility = runtime_surface_symbol_value(
                &compatibility_execution.runtime_surface,
                symbol,
            )
            .unwrap_or_else(|| panic!("compatibility day-2 runtime must publish {symbol}"));
            assert_ne!(
                direct.to_bits(),
                compatibility.to_bits(),
                "{symbol} should expose the direct/compatibility WB14 lineage boundary"
            );
        }
    }

    struct R6fHbpWatEvidence {
        output_pass: PathBuf,
        direct_hbp_bytes: Vec<u8>,
        compatibility_hbp_bytes: Vec<u8>,
        direct_wat_rows: Vec<HillslopeWatRow>,
        compatibility_wat_rows: Vec<HillslopeWatRow>,
        direct_pass_rows: Vec<HillslopePassRow>,
        compatibility_pass_rows: Vec<HillslopePassRow>,
        direct_loss_text: String,
        compatibility_loss_text: String,
    }

    fn r6f_hbp_wat_fixture_evidence() -> R6fHbpWatEvidence {
        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(
            &source_fixture_dir,
            "r6f_direct_publication_hbp_payload",
        );
        let request = HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir: temp_run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };
        let inputs = load_hillslope_run_inputs(&request).expect("fixture inputs should parse");
        let targets = resolve_hillslope_output_targets(&inputs.runfile)
            .expect("fixture output targets should resolve");
        let mut sidecars = resolve_hillslope_sidecars(&request, &inputs, &targets)
            .expect("fixture sidecars should resolve");
        let runtime_setup =
            build_static_hillslope_runtime_setup(&request, &inputs, &mut sidecars)
                .expect("fixture runtime setup should build");
        let mut execution = execute_hillslope_climate_days(
            &inputs.runfile.run_name,
            targets.output_hillslope_id,
            HillslopeRuntimeSelection::DirectPublicationFrameCutover,
            runtime_setup.execution_state,
            &inputs.climate,
        )
        .expect("fixture climate execution should complete");
        execution.direct_publication = build_direct_publication_artifacts(
            HillslopeRuntimeSelection::DirectPublicationFrameCutover,
            &inputs,
            &targets,
            &sidecars,
            &execution,
        )
        .expect("direct publication artifacts should build");
        let artifacts = execution
            .direct_publication
            .as_ref()
            .expect("direct publication artifacts should be retained");
        let compatibility_hbp = build_hbp_output(
            &targets.output_pass,
            &execution.wb13_rows,
            &execution.runtime_surface,
            execution.contributor_ofe_count,
        )
        .expect("compatibility HBP should build");
        let compatibility_wat_rows = build_hillslope_wat_rows(&execution.wb13_rows)
            .expect("compatibility WAT rows should build");
        let compatibility_loss_text = build_loss_output_json(
            &inputs.runfile.run_name,
            &inputs.soil,
            &sidecars.snow,
            &sidecars.frost,
            &execution.climate_span,
            execution.executed_day_count,
        )
        .expect("compatibility loss JSON should build");

        R6fHbpWatEvidence {
            output_pass: targets.output_pass,
            direct_hbp_bytes: artifacts.hbp_bytes.clone(),
            compatibility_hbp_bytes: compatibility_hbp,
            direct_wat_rows: artifacts.wat_rows.clone(),
            compatibility_wat_rows,
            direct_pass_rows: artifacts.pass_projection_rows.clone(),
            compatibility_pass_rows: execution.pass_rows.clone(),
            direct_loss_text: artifacts.loss_text.clone(),
            compatibility_loss_text,
        }
    }

    struct R7dDirectDayTwoSeedEvidence {
        day_two_seed_surface: HillslopeWritebackSurface,
        day_one_runoff_mm: f64,
    }

    fn r7d_direct_day_two_seed_evidence() -> R7dDirectDayTwoSeedEvidence {
        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(
            &source_fixture_dir,
            "r7d_direct_day_two_pmet_seed",
        );
        let request = HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir: temp_run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };
        let inputs = load_hillslope_run_inputs(&request).expect("fixture inputs should parse");
        let targets = resolve_hillslope_output_targets(&inputs.runfile)
            .expect("fixture output targets should resolve");
        let mut sidecars = resolve_hillslope_sidecars(&request, &inputs, &targets)
            .expect("fixture sidecars should resolve");
        let runtime_setup =
            build_static_hillslope_runtime_setup(&request, &inputs, &mut sidecars)
                .expect("fixture runtime setup should build");
        let climate_request = build_hillslope_climate_runtime_request(&inputs.climate)
            .expect("fixture climate request should build");
        let lane_seed_surfaces = direct_production_lane_seed_surfaces(
            &runtime_setup.execution_state.runtime_surface,
            runtime_setup.execution_state.persistent_lane_state.as_ref(),
            runtime_setup.execution_state.per_ofe_lane_areas_m2.len(),
        )
        .expect("direct lane seed surfaces should build");
        let mut frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
            output_hillslope_id: targets.output_hillslope_id,
            lane_areas_m2: &runtime_setup.execution_state.per_ofe_lane_areas_m2,
            runoff_publication_geometries: &runtime_setup
                .execution_state
                .per_ofe_runoff_publication_geometries,
            day_count: runtime_setup.execution_state.climate_span.days.len(),
            climate_request: &climate_request,
            climate_span: &runtime_setup.execution_state.climate_span,
            climate_context_surface: &runtime_setup.execution_state.runtime_surface,
            lane_seed_surfaces: &lane_seed_surfaces,
            execution_lane: runtime_setup.execution_state.lane_context.lane,
        })
        .expect("direct production frame should construct");
        let day_input_builder =
            DirectPublicationDayInputBuilder::new_with_seed_surfaces_and_erosion_guard(
                &climate_request,
                &runtime_setup.execution_state.climate_span,
                lane_seed_surfaces,
                &runtime_setup.execution_state.runtime_surface,
                runtime_setup.execution_state.lane_context.lane,
                true,
            )
            .expect("direct day-input builder should construct");
        let metadata = DirectPublicationRunMetadata {
            run_name: inputs.runfile.run_name.clone(),
            runtime_selection: HillslopeRuntimeSelection::DirectPublicationFrameCutover
                .as_str()
                .to_string(),
            output_policy: direct_publication_output_policy(
                HillslopeRuntimeSelection::DirectPublicationFrameCutover,
            )
            .to_string(),
        };
        let mut day_two_seed_surface = None;
        let execution = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
            .run_publication_capture_with_interleaved_day_inputs(
                &mut frame,
                metadata,
                |frame, day_index, lane_index| {
                    let (day_input, seed_surface) = day_input_builder
                        .build_with_seed_surface(frame, day_index, lane_index)
                        .map_err(|error| direct_publication_day_input_build_error(&error))?;
                    if day_index == 1 && lane_index == 0 {
                        day_two_seed_surface = Some(seed_surface);
                    }
                    Ok(day_input)
                },
            )
            .expect("direct publication capture should complete");
        let day_one_runoff_mm = execution
            .publication_frame
            .rows()
            .iter()
            .find(|row| row.sim_day_index == 1 && row.ofe_id == 1)
            .map(|row| row.runoff.q_mm)
            .expect("direct day-1 publication row should exist");
        R7dDirectDayTwoSeedEvidence {
            day_two_seed_surface: day_two_seed_surface
                .expect("fixture should include a day-2 direct seed surface"),
            day_one_runoff_mm,
        }
    }

    fn r7d2_profile_seed_surface(
        depth_mm: f64,
        porosity_cap_mm: f64,
        wilting_point_mm: f64,
        field_capacity_theta: f64,
        fc_tail_mm: f64,
    ) -> HillslopeWritebackSurface {
        wb11_seed_test_surface(&[
            ("wb13_profile_depth_mm", depth_mm),
            ("wb13_profile_porosity_cap_mm", porosity_cap_mm),
            ("wb13_profile_wp_store_mm", wilting_point_mm),
            ("wb13_profile_fc_tail_mm", fc_tail_mm),
            ("wb11_nsl", 1.0),
            ("wb19_dg_0001", depth_mm / 1_000.0),
            ("wb19_thetfc_0001", field_capacity_theta),
        ])
    }

    fn r6i_compatibility_fixture_execution() -> HillslopeClimateExecution {
        let source_fixture_dir = fixture_path("hillslope_run_dir");
        let temp_run_dir = copy_fixture_to_temp(
            &source_fixture_dir,
            "r6i_compatibility_day_two_pmet_seed",
        );
        let request = HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir: temp_run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        };
        let inputs = load_hillslope_run_inputs(&request).expect("fixture inputs should parse");
        let targets = resolve_hillslope_output_targets(&inputs.runfile)
            .expect("fixture output targets should resolve");
        let mut sidecars = resolve_hillslope_sidecars(&request, &inputs, &targets)
            .expect("fixture sidecars should resolve");
        let runtime_setup =
            build_static_hillslope_runtime_setup(&request, &inputs, &mut sidecars)
                .expect("fixture runtime setup should build");
        execute_hillslope_climate_days(
            &inputs.runfile.run_name,
            targets.output_hillslope_id,
            HillslopeRuntimeSelection::DirectPublicationFrameCutover,
            runtime_setup.execution_state,
            &inputs.climate,
        )
        .expect("fixture climate execution should complete")
    }

    fn assert_r6f_hbp_identity(evidence: &R6fHbpWatEvidence) {
        let (_, direct_payload) = parse_hbp_from_bytes_with_latest_event_payload(
            &evidence.direct_hbp_bytes,
            &evidence.output_pass,
            HbpParseOptions::strict(),
        )
        .expect("direct HBP should parse");
        let (_, compatibility_payload) = parse_hbp_from_bytes_with_latest_event_payload(
            &evidence.compatibility_hbp_bytes,
            &evidence.output_pass,
            HbpParseOptions::strict(),
        )
        .expect("compatibility HBP should parse");

        assert_eq!(
            direct_payload, compatibility_payload,
            "direct latest event payload must match compatibility latest event payload"
        );
        assert_eq!(
            evidence.direct_hbp_bytes, evidence.compatibility_hbp_bytes,
            "direct HBP bytes must match compatibility HBP bytes"
        );
    }

    fn assert_r6i_wat_identity(evidence: &R6fHbpWatEvidence) {
        assert_eq!(
            evidence.direct_wat_rows.len(),
            evidence.compatibility_wat_rows.len()
        );
        let reduced_fields =
            reduced_wat_mismatch_fields(&evidence.direct_wat_rows, &evidence.compatibility_wat_rows);
        assert!(
            reduced_fields.is_empty(),
            "R6I WAT parity should leave no reduced mismatch fields, observed {reduced_fields:?}"
        );
        assert!(!r6g_wat_direct_et_storage_producer_gap(
            &reduced_fields
        ));
        assert!(!r6g_wat_pmet_day_state_carry_gap(
            &evidence.direct_wat_rows,
            &evidence.compatibility_wat_rows,
            &reduced_fields
        ));
        assert!(!r6h_wat_pmet_layer_carry_ulp_gap(
            &evidence.direct_wat_rows,
            &evidence.compatibility_wat_rows,
            &reduced_fields
        ));
        assert_eq!(
            evidence.direct_wat_rows, evidence.compatibility_wat_rows,
            "direct WAT rows must be byte-equivalent at row value level after R6I"
        );
    }

    fn assert_r6j_pass_identity(evidence: &R6fHbpWatEvidence) {
        assert_eq!(
            evidence.direct_pass_rows.len(),
            evidence.compatibility_pass_rows.len()
        );
        let reduced_fields = reduced_pass_mismatch_fields(
            &evidence.direct_pass_rows,
            &evidence.compatibility_pass_rows,
        );
        assert!(
            reduced_fields.is_empty(),
            "R6J PASS parity should leave no reduced mismatch fields, observed {reduced_fields:?}"
        );
        assert_eq!(
            evidence.direct_pass_rows, evidence.compatibility_pass_rows,
            "direct PASS rows must match compatibility PASS rows in test evidence"
        );
    }

    fn assert_r6j_loss_identity(evidence: &R6fHbpWatEvidence) {
        assert_eq!(
            evidence.direct_loss_text, evidence.compatibility_loss_text,
            "direct loss JSON must match compatibility loss JSON in test evidence"
        );
    }

    #[test]
    fn r6b_absent_operand_detector_suppresses_marker_for_nonzero_direct_operands() {
        let identity =
            DirectRunIdentity::new(19, 2637, 1, 1).expect("valid direct identity should construct");
        let zero_row = DirectPublicationDayRow {
            run_id: 19,
            hillslope_id: 2637,
            lane_id: 1,
            ofe_id: 1,
            lane_index: 0,
            day_index: 0,
            sim_day_index: 1,
            calendar: DirectPublicationCalendarDay {
                year: 2026,
                julian_day: 172,
                month: 6,
                day_of_month: 21,
                water_year: 2026,
            },
            area_m2: 400.0,
            climate: DirectPublicationClimateOperands {
                precipitation_mm: 0.0,
            },
            liquid_input: DirectPublicationLiquidInputOperands {
                rm_mm: 0.0,
                irrigation_mm: 0.0,
            },
            runoff: DirectPublicationRunoffOperands {
                q_mm: 0.0,
                qofe_mm: 0.0,
                runvol_m3: 0.0,
                peak_runoff_m3_s: None,
                runoff_duration_s: None,
            },
            evaporation: DirectPublicationEvaporationOperands {
                ep_mm: 0.0,
                es_mm: 0.0,
                er_mm: 0.0,
                total_evapotranspiration_mm: 0.0,
            },
            subsurface: DirectPublicationSubsurfaceOperands {
                dp_mm: 0.0,
                latqcc_mm: 0.0,
                tile_mm: 0.0,
                sbrunv_m3: 0.0,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: 0.0,
                upstream_lateral_mm: 0.0,
            },
            storage: DirectPublicationStorageOperands {
                total_soil_mm: 0.0,
                soil_water_total_mm: 0.0,
                frozwt_mm: 0.0,
                frdp_mm: None,
                snow_water_mm: 0.0,
                snow_depth_mm: 0.0,
            },
            water_temperature: r6_test_no_water_temperature(),
            profile: r6_test_empty_profile(),
            interception: DirectPublicationInterceptionOperands {
                interception_mm: 0.0,
                interception_storage_mm: None,
            },
            erosion: DirectPublicationErosionOperands::absent_authority(),
        };

        assert!(direct_publication_has_only_zero_or_absent_operands(
            &r6b_publication_frame_with_row(identity, zero_row.clone())
        ));
        assert!(direct_publication_lacks_parity_grade_output_producers(
            &r6b_publication_frame_with_row(identity, zero_row.clone())
        ));

        let mut climate_only_row = zero_row.clone();
        climate_only_row.climate.precipitation_mm = 12.5;
        assert!(!direct_publication_has_only_zero_or_absent_operands(
            &r6b_publication_frame_with_row(identity, climate_only_row.clone())
        ));
        assert!(direct_publication_lacks_parity_grade_output_producers(
            &r6b_publication_frame_with_row(identity, climate_only_row)
        ));

        let mut scalar_row = zero_row.clone();
        scalar_row.runoff.q_mm = 1.0;
        assert!(!direct_publication_has_only_zero_or_absent_operands(
            &r6b_publication_frame_with_row(identity, scalar_row.clone())
        ));
        assert!(!direct_publication_lacks_parity_grade_output_producers(
            &r6b_publication_frame_with_row(identity, scalar_row)
        ));

        let mut optional_row = zero_row.clone();
        optional_row.runoff.peak_runoff_m3_s = Some(0.25);
        assert!(!direct_publication_has_only_zero_or_absent_operands(
            &r6b_publication_frame_with_row(identity, optional_row)
        ));

        let mut erosion_row = zero_row;
        erosion_row.erosion.sediment_concentration_kg_m3 = Some([0.0, 0.1, 0.0, 0.0, 0.0]);
        assert!(!direct_publication_has_only_zero_or_absent_operands(
            &r6b_publication_frame_with_row(identity, erosion_row)
        ));
    }

    fn r6b_publication_frame_with_row(
        identity: DirectRunIdentity,
        row: DirectPublicationDayRow,
    ) -> DirectRunPublicationFrame {
        DirectRunPublicationFrame {
            identity,
            metadata: DirectPublicationRunMetadata {
                run_name: "r6b_absent_operands".to_string(),
                runtime_selection: "direct-publication-frame-cutover".to_string(),
                output_policy: "test".to_string(),
            },
            rows: vec![row],
        }
    }

    #[test]
    fn r6a_direct_projection_consumers_read_publication_frame_operands() {
        let frame = r6a_direct_projection_fixture_frame();

        let wat_rows = build_hillslope_wat_rows_from_direct_publication(&frame)
            .expect("direct WAT projection should build");
        let pass_rows = build_hillslope_pass_rows_from_direct_publication(&frame)
            .expect("direct PASS projection should build");
        let loss = build_loss_output_json_from_direct_publication(&frame, 1, false, 0)
            .expect("direct loss projection should build");
        let manifest = build_manifest_text_from_direct_publication(&frame)
            .expect("direct manifest projection should build");

        assert_eq!(wat_rows[0].q.to_bits(), 12.5_f64.to_bits());
        assert_eq!(wat_rows[0].qofe.to_bits(), 10.0_f64.to_bits());
        assert_eq!(wat_rows[0].rm.to_bits(), 8.25_f64.to_bits());
        assert_eq!(pass_rows[0].runvol_m3.to_bits(), 4.0_f64.to_bits());
        assert_eq!(pass_rows[0].peakro_m3_s.to_bits(), 0.75_f64.to_bits());
        let loss_json: serde_json::Value =
            serde_json::from_str(&loss).expect("direct loss projection should be JSON");
        assert_eq!(loss_json["run_name"], "r6a_projection");
        assert!(manifest.contains("row_count=1"));
    }

    fn r6a_direct_projection_fixture_frame() -> DirectRunPublicationFrame {
        let identity =
            DirectRunIdentity::new(19, 2637, 1, 1).expect("valid direct identity should construct");
        let row = DirectPublicationDayRow {
            run_id: 19,
            hillslope_id: 2637,
            lane_id: 1,
            ofe_id: 1,
            lane_index: 0,
            day_index: 0,
            sim_day_index: 1,
            calendar: DirectPublicationCalendarDay {
                year: 2026,
                julian_day: 172,
                month: 6,
                day_of_month: 21,
                water_year: 2026,
            },
            area_m2: 400.0,
            climate: DirectPublicationClimateOperands {
                precipitation_mm: 7.5,
            },
            liquid_input: DirectPublicationLiquidInputOperands {
                rm_mm: 8.25,
                irrigation_mm: 1.25,
            },
            runoff: DirectPublicationRunoffOperands {
                q_mm: 12.5,
                qofe_mm: 10.0,
                runvol_m3: 4.0,
                peak_runoff_m3_s: Some(0.75),
                runoff_duration_s: Some(1800.0),
            },
            evaporation: DirectPublicationEvaporationOperands {
                ep_mm: 2.0,
                es_mm: 3.0,
                er_mm: 4.0,
                total_evapotranspiration_mm: 9.0,
            },
            subsurface: DirectPublicationSubsurfaceOperands {
                dp_mm: 1.5,
                latqcc_mm: 2.5,
                tile_mm: 0.5,
                sbrunv_m3: 1.0,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: 0.25,
                upstream_lateral_mm: 0.125,
            },
            storage: DirectPublicationStorageOperands {
                total_soil_mm: 110.0,
                soil_water_total_mm: 105.0,
                frozwt_mm: 1.0,
                frdp_mm: Some(2.0),
                snow_water_mm: 3.0,
                snow_depth_mm: 4.0,
            },
            water_temperature: r6_test_no_water_temperature(),
            profile: DirectPublicationProfileOperands {
                depth_mm: Some(1000.0),
                porosity_cap_mm: Some(450.0),
                fc_store_mm: Some(300.0),
                wp_store_mm: Some(150.0),
            },
            interception: DirectPublicationInterceptionOperands {
                interception_mm: 0.75,
                interception_storage_mm: Some(0.5),
            },
            erosion: DirectPublicationErosionOperands {
                peak_runoff_m3_s: Some(0.75),
                runoff_duration_s: Some(1800.0),
                total_detachment_kg: Some(2.25),
                total_deposition_kg: Some(1.25),
                hbp_total_detachment_kg: Some(2.25),
                hbp_total_deposition_kg: Some(1.25),
                hbp_sediment_concentration_kg_m3: Some(0.1),
                sediment_concentration_kg_m3: Some([0.1, 0.2, 0.3, 0.4, 0.5]),
            },
        };
        DirectRunPublicationFrame {
            identity,
            metadata: DirectPublicationRunMetadata {
                run_name: "r6a_projection".to_string(),
                runtime_selection: "direct-publication-frame-shadow".to_string(),
                output_policy: "test".to_string(),
            },
            rows: vec![row],
        }
    }

    #[test]
    fn r6j_direct_manifest_provenance_accepts_multiofe_direct_rows() {
        let identity =
            DirectRunIdentity::new(42, 2637, 2, 2).expect("valid direct identity should construct");
        let frame = DirectRunPublicationFrame {
            identity,
            metadata: DirectPublicationRunMetadata {
                run_name: "r6j_multiofe_manifest".to_string(),
                runtime_selection: "direct-publication-frame-cutover".to_string(),
                output_policy: "test".to_string(),
            },
            rows: vec![
                r6j_multiofe_publication_row(1, 1),
                r6j_multiofe_publication_row(2, 1),
                r6j_multiofe_publication_row(1, 2),
                r6j_multiofe_publication_row(2, 2),
            ],
        };

        let (wb13_publication, mofe_hourly_carry) =
            build_direct_publication_manifest_provenance(&frame)
                .expect("direct multi-OFE manifest provenance should build");
        assert_eq!(wb13_publication.source, "direct-publication-frame");
        assert_eq!(wb13_publication.contributor_ofe_count, 2);
        assert_eq!(wb13_publication.row_count, 4);
        assert_eq!(wb13_publication.publication_area_m2.to_bits(), 1200.0_f64.to_bits());
        assert_eq!(wb13_publication.per_ofe_record_count, 4);
        assert_eq!(wb13_publication.per_ofe_internal_day_count, 2);
        assert_eq!(wb13_publication.per_ofe_expected_record_count, 4);
        assert_eq!(wb13_publication.first_row_key.ofe, 1);
        assert_eq!(wb13_publication.first_row_key.sim_day_index, 1);
        assert_eq!(wb13_publication.last_row_key.ofe, 2);
        assert_eq!(wb13_publication.last_row_key.sim_day_index, 2);
        assert!(mofe_hourly_carry.active);
        assert_eq!(mofe_hourly_carry.substep_count, 24);

        let wat_rows = build_hillslope_wat_rows_from_direct_publication(&frame)
            .expect("direct multi-OFE WAT rows should build");
        assert_eq!(wat_rows.len(), 4);
        assert_eq!(wat_rows[0].ofe_id, 1);
        assert_eq!(wat_rows[1].ofe_id, 2);
        assert_eq!(wat_rows[2].sim_day_index, 2);
        assert_eq!(wat_rows[3].ofe_id, 2);
        let pass_rows = build_hillslope_pass_rows_from_direct_publication(&frame)
            .expect("direct multi-OFE PASS rows should build");
        assert_eq!(pass_rows.len(), 2);
        assert_eq!(pass_rows[0].year, 1);
        assert_eq!(pass_rows[0].sim_day_index, 1);
        assert_eq!(pass_rows[1].sim_day_index, 2);
    }

    #[test]
    fn r7d4_direct_publication_layer_state_uses_hourly_lateral_ssh_not_vertical_ssc() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("wb18_perc_theta_0001", 0.150),
            ("wb18_perc_fc_0001", 0.100),
            ("wb18_perc_ul_0001", 0.250),
            ("wb18_perc_ssc_0001", 1.0e-6),
            ("wb19_lateral_ssh_0001", 4.0e-6),
            ("wb19_dg_0001", 0.200),
            ("wb19_thetdr_0001", 0.010),
            ("wb19_por_0001", 0.500),
            ("wb19_thetfc_0001", 0.250),
            ("wb19_coca_0001", 0.750),
            ("wb19_lateral_drain_lane_substeps", 24.0),
            ("solwpv", 9002.0),
        ]);

        let layer = direct_publication_layer_state(&runtime_surface, 1)
            .expect("hourly lateral conductivity should be consumed");
        assert_eq!(layer.conductivity_m_s.to_bits(), 1.0e-6_f64.to_bits());
        assert_eq!(
            layer.lateral_conductivity_m_s.to_bits(),
            4.0e-6_f64.to_bits()
        );

        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("wb19_lateral_ssh_0001"));
        let error = direct_publication_layer_state(&runtime_surface, 1)
            .expect_err("modern hourly lanes must not fall back to vertical ssc");
        assert!(
            error
                .to_string()
                .contains("direct hourly WB19 lateral conductivity requires wb19_lateral_ssh_0001"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn r7d4_direct_publication_percolation_inputs_carry_restrictive_layer_authority() {
        let mut runtime_surface = wb11_seed_test_surface(&[
            ("wb11_soil_water", 0.150),
            ("wb18_perc_lane_substeps", 24.0),
            ("slflag", 1.0),
            ("kslast", 1.0e-6),
            ("ui_bdrkth", 0.750),
            ("wb18_perc_theta_0001", 0.150),
            ("wb18_perc_fc_0001", 0.100),
            ("wb18_perc_ul_0001", 0.250),
            ("wb18_perc_ssc_0001", 8.0e-5),
            ("wb19_lateral_ssh_0001", 4.0e-6),
            ("wb19_dg_0001", 0.200),
            ("wb19_thetdr_0001", 0.010),
            ("wb19_por_0001", 0.500),
            ("wb19_thetfc_0001", 0.250),
            ("wb19_coca_0001", 0.750),
            ("wb11_nsl", 1.0),
            ("wb19_nsl", 1.0),
        ]);

        let inputs = direct_publication_percolation_inputs(&runtime_surface, 0.045)
            .expect("restrictive layer inputs should be carried into direct WB18");
        assert!(inputs.restrictive_layer_enabled);
        assert_eq!(
            inputs.restrictive_layer_conductivity_m_s.to_bits(),
            1.0e-6_f64.to_bits()
        );
        assert_ne!(
            inputs.restrictive_layer_conductivity_m_s.to_bits(),
            inputs.layers[0].conductivity_m_s.to_bits()
        );
        assert_eq!(
            inputs.restrictive_layer_thickness_m.to_bits(),
            0.750_f64.to_bits()
        );
        assert_eq!(inputs.lane_substeps, 24);

        runtime_surface
            .state_surface
            .remove(&BoundarySymbol::from("ui_bdrkth"));
        let error = direct_publication_percolation_inputs(&runtime_surface, 0.045)
            .expect_err("hourly restrictive-layer direct WB18 must require bedrock thickness");
        assert!(
            error.to_string().contains("ui_bdrkth"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn r7d4_direct_publication_interception_uses_wb15_operands_not_i_alias() {
        let runtime_surface = wb11_seed_test_surface(&[
            ("wb12_rainfall_input", 0.010),
            ("ninten", 2.0),
            ("timem_0001", 0.0),
            ("timem_0002", 3_600.0),
            ("intsty_0001", 0.010 / 3_600.0),
            ("cancov", 0.5),
            ("lai", 1.0),
            ("vdmt", 0.2),
            ("hia", 0.0),
            ("I", 0.009),
        ]);

        let hyetograph = direct_publication_hyetograph(&runtime_surface)
            .expect("hyetograph should be available for direct WB15");
        let interception = direct_publication_interception_state(&runtime_surface, 0.010, &hyetograph)
            .expect("direct WB15 interception should compute from plant operands");
        let expected_interception_m =
            0.5 * ((0.000_627 * 2_000.0 - 3.733_49e-8 * 2_000.0_f64.powi(2)) / 1_000.0);

        assert!((interception.interception_m - expected_interception_m).abs() < 1.0e-12);
        assert_ne!(
            interception.interception_m.to_bits(),
            0.009_f64.to_bits(),
            "direct WB15 must not consume a stale publication I alias"
        );
        assert!(
            (interception.liquid_after_interception_m - (0.010 - expected_interception_m)).abs()
                < 1.0e-12
        );

        let scaled_hyetograph =
            direct_publication_scaled_hyetograph(&hyetograph, interception.rainfall_scale)
                .expect("direct WB15 scale should produce a post-interception hyetograph");
        assert!((scaled_hyetograph[0].intensity_m_s * 3_600.0
            - interception.liquid_after_interception_m)
            .abs()
            < 1.0e-12);
        let liquid_inputs = direct_publication_liquid_input_inputs(
            interception.liquid_after_interception_m,
        )
        .expect("post-interception liquid input should be valid");
        assert_eq!(
            liquid_inputs.liquid_input_handoff_m.to_bits(),
            interception.liquid_after_interception_m.to_bits()
        );
    }

    fn r6j_multiofe_publication_row(ofe_id: u32, sim_day_index: i32) -> DirectPublicationDayRow {
        let lane_index = usize::try_from(ofe_id - 1).expect("test OFE id should fit usize");
        let day_index = usize::try_from(sim_day_index - 1).expect("test day should fit usize");
        let offset = f64::from(ofe_id) + f64::from(sim_day_index) / 10.0;
        DirectPublicationDayRow {
            run_id: 42,
            hillslope_id: 2637,
            lane_id: ofe_id,
            ofe_id,
            lane_index,
            day_index,
            sim_day_index,
            calendar: DirectPublicationCalendarDay {
                year: 2026,
                julian_day: u16::try_from(sim_day_index).expect("test day should fit u16"),
                month: 1,
                day_of_month: i8::try_from(sim_day_index).expect("test day should fit i8"),
                water_year: 2026,
            },
            area_m2: 400.0 * f64::from(ofe_id),
            climate: DirectPublicationClimateOperands {
                precipitation_mm: 7.5 + offset,
            },
            liquid_input: DirectPublicationLiquidInputOperands {
                rm_mm: 8.25 + offset,
                irrigation_mm: 1.25,
            },
            runoff: DirectPublicationRunoffOperands {
                q_mm: 12.5 + offset,
                qofe_mm: 10.0 + offset,
                runvol_m3: 4.0 + offset,
                peak_runoff_m3_s: Some(0.75 + offset),
                runoff_duration_s: Some(1800.0 + offset),
            },
            evaporation: DirectPublicationEvaporationOperands {
                ep_mm: 2.0 + offset,
                es_mm: 3.0 + offset,
                er_mm: 4.0 + offset,
                total_evapotranspiration_mm: 9.0 + offset,
            },
            subsurface: DirectPublicationSubsurfaceOperands {
                dp_mm: 1.5 + offset,
                latqcc_mm: 2.5 + offset,
                tile_mm: 0.5 + offset,
                sbrunv_m3: 1.0 + offset,
            },
            transfer: DirectPublicationTransferOperands {
                upstream_surface_mm: 0.25 + offset,
                upstream_lateral_mm: 0.125 + offset,
            },
            storage: DirectPublicationStorageOperands {
                total_soil_mm: 110.0 + offset,
                soil_water_total_mm: 105.0 + offset,
                frozwt_mm: 1.0 + offset,
                frdp_mm: Some(2.0 + offset),
                snow_water_mm: 3.0 + offset,
                snow_depth_mm: 4.0 + offset,
            },
            water_temperature: r6_test_no_water_temperature(),
            profile: DirectPublicationProfileOperands {
                depth_mm: Some(1000.0 + offset),
                porosity_cap_mm: Some(450.0 + offset),
                fc_store_mm: Some(300.0 + offset),
                wp_store_mm: Some(150.0 + offset),
            },
            interception: DirectPublicationInterceptionOperands {
                interception_mm: 0.75 + offset,
                interception_storage_mm: Some(0.5 + offset),
            },
            erosion: DirectPublicationErosionOperands {
                peak_runoff_m3_s: Some(0.75 + offset),
                runoff_duration_s: Some(1800.0 + offset),
                total_detachment_kg: Some(2.25 + offset),
                total_deposition_kg: Some(1.25 + offset),
                hbp_total_detachment_kg: Some(2.25 + offset),
                hbp_total_deposition_kg: Some(1.25 + offset),
                hbp_sediment_concentration_kg_m3: Some(0.1 + offset),
                sediment_concentration_kg_m3: Some([0.1, 0.2, 0.3, 0.4, 0.5]),
            },
        }
    }

    fn runner_execution_lock() -> &'static Mutex<()> {
        static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        RUN_LOCK.get_or_init(|| Mutex::new(()))
    }

    fn r6_test_no_water_temperature() -> DirectPublicationWaterTemperatureOperands {
        DirectPublicationWaterTemperatureOperands {
            meltwater_temperature_c: None,
        }
    }

    fn r6_test_empty_profile() -> DirectPublicationProfileOperands {
        DirectPublicationProfileOperands {
            depth_mm: None,
            porosity_cap_mm: None,
            fc_store_mm: None,
            wp_store_mm: None,
        }
    }

    fn fixture_path(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/cli01")
            .join(name)
    }

    fn copy_fixture_to_temp(source_dir: &Path, prefix: &str) -> PathBuf {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("unix epoch should be before now")
            .as_nanos();
        let destination = std::env::temp_dir().join(format!("{prefix}_{timestamp}"));
        copy_dir_recursive(source_dir, &destination);
        destination
    }

    fn enable_r6j_pass_parquet_output(run_dir: &Path) {
        let run_file = run_dir.join("case.run");
        let mut text = fs::read_to_string(&run_file)
            .unwrap_or_else(|error| panic!("case.run should be readable: {error}"));
        if !text.contains("pass_parquet") {
            text.push_str("pass_parquet = \"output/H5.pass.parquet\"\n");
        }
        fs::write(&run_file, text)
            .unwrap_or_else(|error| panic!("case.run should be writable: {error}"));
    }

    fn assert_manifest_checksum_matches_file(
        output_checksums: &serde_json::Map<String, serde_json::Value>,
        output_path: &Path,
    ) {
        let key = output_path.display().to_string();
        let observed = output_checksums
            .get(&key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or_else(|| panic!("manifest must include checksum for {key}"));
        let expected = sha256_file_hex(output_path)
            .unwrap_or_else(|error| panic!("checksum should compute for {key}: {error}"));
        assert_eq!(observed, expected, "manifest checksum mismatch for {key}");
    }

    fn assert_r6j_parquet_disk_identity(evidence: &R6fHbpWatEvidence) {
        let temp_dir = copy_fixture_to_temp(
            &fixture_path("hillslope_run_dir"),
            "r6j_parquet_disk_identity",
        );
        let direct_wat = temp_dir.join("direct.wat.parquet");
        let compatibility_wat = temp_dir.join("compatibility.wat.parquet");
        let direct_pass = temp_dir.join("direct.pass.parquet");
        let compatibility_pass = temp_dir.join("compatibility.pass.parquet");
        write_hillslope_wat_parquet(
            &direct_wat,
            &evidence.direct_wat_rows,
            InterchangeVersion::default(),
        )
        .expect("direct WAT disk evidence should write");
        write_hillslope_wat_parquet(
            &compatibility_wat,
            &evidence.compatibility_wat_rows,
            InterchangeVersion::default(),
        )
        .expect("compatibility WAT disk evidence should write");
        write_hillslope_pass_parquet(
            &direct_pass,
            &evidence.direct_pass_rows,
            InterchangeVersion::default(),
        )
        .expect("direct PASS disk evidence should write");
        write_hillslope_pass_parquet(
            &compatibility_pass,
            &evidence.compatibility_pass_rows,
            InterchangeVersion::default(),
        )
        .expect("compatibility PASS disk evidence should write");

        assert_parquet_batches_identical("WAT", &direct_wat, &compatibility_wat);
        assert_parquet_batches_identical("PASS", &direct_pass, &compatibility_pass);
    }

    fn assert_parquet_batches_identical(label: &str, left_path: &Path, right_path: &Path) {
        let left = read_single_parquet_batch(left_path);
        let right = read_single_parquet_batch(right_path);
        assert_eq!(left.schema(), right.schema(), "{label} schema mismatch");
        assert_eq!(left.num_rows(), right.num_rows(), "{label} row count mismatch");
        assert_eq!(
            left.num_columns(),
            right.num_columns(),
            "{label} column count mismatch"
        );
        for column_index in 0..left.num_columns() {
            assert_eq!(
                left.column(column_index).to_data(),
                right.column(column_index).to_data(),
                "{label} serialized column {column_index} mismatch"
            );
        }
    }

    fn read_single_parquet_batch(path: &Path) -> RecordBatch {
        let file = fs::File::open(path)
            .unwrap_or_else(|error| panic!("parquet file should be readable: {error}"));
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)
            .unwrap_or_else(|error| panic!("parquet footer should be readable: {error}"));
        let mut reader = builder
            .build()
            .unwrap_or_else(|error| panic!("parquet reader should build: {error}"));
        reader
            .next()
            .unwrap_or_else(|| panic!("parquet file should contain one batch: {}", path.display()))
            .unwrap_or_else(|error| panic!("parquet batch should read: {error}"))
    }

    fn copy_dir_recursive(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).expect("destination directory should be creatable");

        for entry in fs::read_dir(source).expect("source directory should be readable") {
            let entry = entry.expect("directory entry should be readable");
            let source_path = entry.path();
            let destination_path = destination.join(entry.file_name());
            if source_path.is_dir() {
                copy_dir_recursive(&source_path, &destination_path);
            } else {
                fs::copy(&source_path, &destination_path).expect("file copy should succeed");
            }
        }
    }















    fn read_manifest_json(report: &HillslopeRunReport) -> serde_json::Value {
        let manifest_text = fs::read_to_string(&report.manifest_path).unwrap_or_else(|error| {
            panic!(
                "manifest should be readable at {}: {error}",
                report.manifest_path.display()
            )
        });
        serde_json::from_str(&manifest_text)
            .unwrap_or_else(|error| panic!("manifest should parse as JSON: {error}"))
    }

    fn assert_json_i64(document: &serde_json::Value, pointer: &str, expected: i64) {
        let observed = document
            .pointer(pointer)
            .and_then(serde_json::Value::as_i64)
            .unwrap_or_else(|| panic!("missing integer JSON pointer {pointer}"));
        assert_eq!(observed, expected, "unexpected value at {pointer}");
    }
}
