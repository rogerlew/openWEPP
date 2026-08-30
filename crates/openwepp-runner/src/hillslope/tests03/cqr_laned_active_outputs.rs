use super::*;
use openwepp_hillslope_orchestrator::{
    DirectExecutionReport, DirectExecutorMode, DirectLanedActiveConfig,
    DirectLanedActiveLaneConfig, DirectLanedActiveMeshPolicy, DirectLanedActiveRunSummary,
    DirectLanedActiveStageLimiterTrace, DirectLanedActiveStepTraceRecord,
    DirectLanedActiveTraceDetail, DirectLanedActiveTraceRecord, DirectLanedActiveTvdTrace,
    DirectPublicationRunMetadata, DirectRunIdentity, DirectStreamingPublicationExecution,
};

fn step_trace() -> DirectLanedActiveStepTraceRecord {
    let limiter = DirectLanedActiveStageLimiterTrace {
        reductions: 2,
        max_reduction_m3_s: 0.25,
        face_index: 3,
        face_x_m: 4.0,
    };
    DirectLanedActiveStepTraceRecord {
        step_index: 5,
        t_start_s: 6.0,
        t_end_s: 7.0,
        dt_s: 1.0,
        max_courant: 0.4,
        max_courant_cell_index: 8,
        max_courant_cell_center_x_m: 9.0,
        q_up_m3_s: 10.0,
        source_m3: 11.0,
        upstream_inflow_m3: 12.0,
        outflow_m3: 13.0,
        storage_before_m3: 14.0,
        storage_after_m3: 15.0,
        clamp_injected_m3: 16.0,
        pred_out_face_m3_s: 17.0,
        corr_out_face_m3_s: 18.0,
        outlet_depth_m: 19.0,
        outlet_discharge_m3_s: 20.0,
        predictor_limiter: limiter,
        corrector_limiter: limiter,
        tvd: DirectLanedActiveTvdTrace {
            scale: 0.5,
            max_abs_delta_m: 0.6,
            cell_index: 21,
            cell_center_x_m: 22.0,
            signed_delta_m: -0.7,
        },
    }
}

fn trace_record(with_detail: bool) -> DirectLanedActiveTraceRecord {
    DirectLanedActiveTraceRecord {
        day_index: 1,
        lane_index: 2,
        max_dt_s: 300.0,
        is_terminal_lane: true,
        source_m3: 3.0,
        outlet_m3: 2.0,
        terminal_day_outlet_m3: Some(2.0),
        mesh_end_storage_m3: 1.0,
        clamp_m3: 0.0,
        tail_fold_m3: 0.25,
        routed_weights: [1.0 / 24.0; 24],
        uniform_shape: false,
        erosion_source_shape_degenerate: false,
        trace_detail: with_detail.then(|| {
            Box::new(DirectLanedActiveTraceDetail {
                mesh_cell_count: 10,
                mesh_dx_m: 5.0,
                max_dt_s: 300.0,
                outlet_bin_m3: vec![1.0, 2.0],
                outlet_bin_spans_s: vec![900.0, 900.0],
                hydrograph_time_s: vec![0.0, 900.0],
                hydrograph_outlet_m3_s: vec![0.0, 0.1],
                hydrograph_outlet_depth_m: vec![0.0, 0.01],
                step_trace: Some(vec![step_trace()]),
            })
        }),
    }
}

#[test]
fn cqr_laned_active_trace_serialization_preserves_schema_indices_and_nested_detail() {
    let line =
        serialize_laned_active_trace_record(&trace_record(true)).expect("trace row serialization");
    let value: serde_json::Value = serde_json::from_str(&line).expect("valid trace JSON");
    assert_eq!(value["schema"], "openwepp-laned-active-trace-row-v1");
    assert_eq!(value["day_index_zero_based"], 1);
    assert_eq!(value["sim_day_index"], 2);
    assert_eq!(value["lane_index_zero_based"], 2);
    assert_eq!(value["lane_index"], 3);
    assert_eq!(
        value["trace_detail"]["schema"],
        "openwepp-laned-active-trace-detail-v1"
    );
    assert_eq!(value["trace_detail"]["step_trace"][0]["step_index"], 5);
    assert_eq!(
        value["trace_detail"]["step_trace"][0]["predictor_limiter"]["face_index"],
        3
    );
    assert_eq!(
        value["trace_detail"]["step_trace"][0]["tvd"]["cell_index"],
        21
    );
    let sum = value["routed_hourly_weight_sum"]
        .as_f64()
        .expect("weight sum");
    assert!((sum - 1.0).abs() < 1.0e-12);
}

#[test]
fn cqr_laned_active_trace_serialization_preserves_absent_detail_as_null() {
    let line = serialize_laned_active_trace_record(&trace_record(false))
        .expect("trace row serialization without detail");
    let value: serde_json::Value = serde_json::from_str(&line).expect("valid trace JSON");
    assert!(value["trace_detail"].is_null());
    assert_eq!(value["terminal_day_outlet_m3"], 2.0);
}

fn trace_artifacts(identity: DirectRunIdentity) -> DirectPublicationArtifacts {
    DirectPublicationArtifacts {
        execution: DirectStreamingPublicationExecution {
            report: DirectExecutionReport {
                mode: DirectExecutorMode::ProductionDirect,
                lane_count: 1,
                day_count: 1,
                planned_phase_count: 0,
                canonical_phase_entry_count: 0,
                phase_view_count: 0,
                phase_status_counts: Vec::new(),
                phase_span_run_count: 0,
                direct_phase_entry_count: 0,
                direct_compute_count: 0,
                state_mutation_count: 0,
                downstream_operand_count: 0,
                shadow_projection_count: 0,
                compatibility_edge_invocation_count: 0,
                day_frame_commit_count: 1,
            },
            identity,
            metadata: DirectPublicationRunMetadata {
                run_name: "trace".to_string(),
                runtime_selection: "direct".to_string(),
                output_policy: "test".to_string(),
            },
            row_count: 1,
        },
        summary: DirectPublicationOutputSummary {
            identity,
            metadata: DirectPublicationRunMetadata {
                run_name: "trace".to_string(),
                runtime_selection: "direct".to_string(),
                output_policy: "test".to_string(),
            },
            row_count: 1,
            first_row: None,
            last_row: None,
            hbp_sediment_row: None,
            hbp_event_chain_totals_kg: None,
            hbp_current_day_index: None,
            hbp_current_day_tdet_kg: 0.0,
            hbp_current_day_tdep_kg: 0.0,
            parity_grade_row_seen: false,
            area_by_ofe: BTreeMap::new(),
            sim_day_index_monotonic: true,
            previous_sim_day_index: None,
            upstream_carry_total_mm: 0.0,
        },
        hbp_bytes: Vec::new(),
        wat_rows_written: None,
        pass_projection_rows_written: None,
        loss_text: String::new(),
        manifest_text: String::new(),
    }
}

fn trace_coupling_vectors() -> HillslopeCouplingVectorProvenance {
    HillslopeCouplingVectorProvenance {
        guard_id: "test".to_string(),
        winter: HillslopeWinterCouplingProvenance {
            active: false,
            snow_file_present: false,
            rst: 0.0,
            newsnw: 0.0,
            ssd: 0.0,
            runtime_swe: 0.0,
        },
        soil: HillslopeSoilCouplingProvenance {
            ssc: 0.0,
            infiltration_capacity_frozen: 0.0,
            infcap_within_ssc: true,
        },
        frsoil: HillslopeFrozenSoilCouplingProvenance {
            active: false,
            frost_file_present: false,
            wint_red_enabled: false,
            dfrost: 0.0,
            dthaw: 0.0,
            nft: 0.0,
            ws_frz: 0.0,
            infcap_frz: 0.0,
        },
        hydout_equivalent: HillslopeHydoutEquivalentCouplingProvenance {
            source: "test".to_string(),
            total_soil: 0.0,
            frozwt: 0.0,
            snow_water: 0.0,
            soil_water_total: 0.0,
            closure_delta: 0.0,
            closure_tolerance: 0.0,
            closure_within_tolerance: true,
        },
    }
}

fn trace_execution(
    trace_records: Option<Vec<DirectLanedActiveTraceRecord>>,
    include_artifacts: bool,
) -> HillslopeClimateExecution {
    let identity = DirectRunIdentity::new(301, 501, 1, 1).expect("trace identity");
    let direct_publication = include_artifacts.then(|| trace_artifacts(identity));
    let summary = DirectLanedActiveRunSummary {
        trace_records,
        ..DirectLanedActiveRunSummary::default()
    };
    let day = ClimateDayProjection {
        year: 2026,
        month: 1,
        day_of_month: 1,
        julian_day: 1,
        precipitation_mm: 0.0,
        effective_temperature_c: 0.0,
    };
    HillslopeClimateExecution {
        selected_lane: ExecutionLane::Hourly,
        climate_span: ClimateRunSpanSummary {
            days: vec![day],
            first_day: day,
            last_day: day,
        },
        coupling_vectors: trace_coupling_vectors(),
        multi_ofe_wave1_chained: false,
        laned_shadow: None,
        laned_active: Some(summary),
        scheduler_outcome_class: "completed",
        scheduler_status_message_id: "test".to_string(),
        kernel_phase_message_ids: Vec::new(),
        executed_day_count: 1,
        retained_direct_publication: None,
        direct_publication,
    }
}

fn trace_targets(path: Option<PathBuf>) -> HillslopeOutputTargets {
    HillslopeOutputTargets {
        output_pass: PathBuf::from("unused.pass"),
        output_loss: PathBuf::from("unused.loss"),
        optional_outputs: Vec::new(),
        pass_parquet: None,
        wat: None,
        wat_subhourly: None,
        laned_active_trace: path,
        output_hillslope_id: 501,
    }
}

#[test]
fn cqr_laned_active_trace_writer_covers_validation_and_real_file_output() {
    let execution = trace_execution(Some(vec![trace_record(true)]), true);
    write_laned_active_trace_output(&trace_targets(None), &execution).expect("trace disabled");

    let path = std::env::temp_dir().join(format!(
        "openwepp-cqr-ha08-trace-{}-{}.jsonl",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    write_laned_active_trace_output(&trace_targets(Some(path.clone())), &execution)
        .expect("trace output");
    let text = std::fs::read_to_string(&path).expect("trace file");
    assert!(text.ends_with('\n'));
    assert_eq!(text.lines().count(), 1);
    assert!(text.contains("openwepp-laned-active-trace-row-v1"));
    assert_eq!(
        text,
        format!(
            "{}\n",
            serialize_laned_active_trace_record(&trace_record(true)).expect("nominal trace row")
        )
    );
    let _ = std::fs::remove_file(path);

    let missing_records = trace_execution(None, true);
    assert!(
        write_laned_active_trace_output(
            &trace_targets(Some(PathBuf::from("unused.jsonl"))),
            &missing_records
        )
        .is_err()
    );
    let mut missing_summary = trace_execution(None, true);
    missing_summary.laned_active = None;
    assert!(
        write_laned_active_trace_output(
            &trace_targets(Some(PathBuf::from("unused.jsonl"))),
            &missing_summary
        )
        .is_err()
    );
    let missing_artifacts = trace_execution(Some(vec![trace_record(false)]), false);
    assert!(
        write_laned_active_trace_output(
            &trace_targets(Some(PathBuf::from("unused.jsonl"))),
            &missing_artifacts
        )
        .is_err()
    );
    let mismatch = trace_execution(Some(Vec::new()), true);
    assert!(
        write_laned_active_trace_output(
            &trace_targets(Some(PathBuf::from("unused.jsonl"))),
            &mismatch
        )
        .is_err()
    );
}

#[test]
fn cqr_laned_active_trace_writer_rejects_invalid_numeric_values_before_output() {
    let path = std::env::temp_dir().join(format!(
        "openwepp-cqr-ha08-invalid-numerics-{}.jsonl",
        std::process::id()
    ));
    let reject = |record, field: &str| {
        let _ = std::fs::remove_file(&path);
        let execution = trace_execution(Some(vec![record]), true);
        let error = write_laned_active_trace_output(&trace_targets(Some(path.clone())), &execution)
            .expect_err("invalid numeric trace value must fail closed");
        assert!(error.to_string().contains(field), "{error}");
        assert!(!path.exists(), "invalid trace must not create output");
    };

    let mut nonfinite_volume = trace_record(true);
    nonfinite_volume.source_m3 = f64::NAN;
    reject(nonfinite_volume, "source_m3");

    let mut zero_positive_domain = trace_record(false);
    zero_positive_domain.max_dt_s = 0.0;
    reject(zero_positive_domain, "max_dt_s");

    let mut negative_positive_domain = trace_record(true);
    negative_positive_domain
        .trace_detail
        .as_deref_mut()
        .expect("trace detail")
        .mesh_dx_m = -1.0;
    reject(negative_positive_domain, "trace_detail.mesh_dx_m");

    let mut nonfinite_detail = trace_record(true);
    let detail = nonfinite_detail
        .trace_detail
        .as_deref_mut()
        .expect("trace detail");
    detail.outlet_bin_m3[0] = f64::NAN;
    reject(nonfinite_detail, "trace_detail.outlet_bin_m3[0]");

    let mut nonfinite_step = trace_record(true);
    nonfinite_step
        .trace_detail
        .as_deref_mut()
        .expect("trace detail")
        .step_trace
        .as_mut()
        .expect("step trace")[0]
        .predictor_limiter
        .face_x_m = f64::INFINITY;
    reject(nonfinite_step, "step_trace[0].predictor_limiter.face_x_m");

    let mut nonfinite_tvd = trace_record(true);
    nonfinite_tvd
        .trace_detail
        .as_deref_mut()
        .expect("trace detail")
        .step_trace
        .as_mut()
        .expect("step trace")[0]
        .tvd
        .signed_delta_m = f64::NAN;
    reject(nonfinite_tvd, "step_trace[0].tvd.signed_delta_m");

    let mut negative_weight = trace_record(false);
    negative_weight.routed_weights = [0.0; 24];
    negative_weight.routed_weights[0] = -0.25;
    negative_weight.routed_weights[1] = 1.25;
    reject(negative_weight, "routed_hourly_weights[0]");

    let mut non_unit_weights = trace_record(false);
    non_unit_weights.routed_weights = [0.125; 24];
    reject(non_unit_weights, "routed_hourly_weight_sum");

    let mut dry_nonzero_weights = trace_record(false);
    dry_nonzero_weights.source_m3 = 0.0;
    reject(dry_nonzero_weights, "routed_hourly_weights");

    let mut dry = trace_record(true);
    dry.source_m3 = 0.0;
    dry.routed_weights = [0.0; 24];
    let execution = trace_execution(Some(vec![dry]), true);
    write_laned_active_trace_output(&trace_targets(Some(path.clone())), &execution)
        .expect("valid dry full-detail trace must remain writable");
    let value: serde_json::Value = serde_json::from_str(
        std::fs::read_to_string(&path)
            .expect("dry trace output")
            .trim_end(),
    )
    .expect("dry trace JSON");
    assert_eq!(value["routed_hourly_weight_sum"], 0.0);
    assert!(value["trace_detail"].is_object());

    let mut tolerance_edge = trace_record(false);
    tolerance_edge.routed_weights[0] += 5.0e-10;
    let execution = trace_execution(Some(vec![tolerance_edge]), true);
    write_laned_active_trace_output(&trace_targets(Some(path.clone())), &execution)
        .expect("contract-authorized unit-sum drift must remain writable");

    let mut negative_storage_delta = trace_record(false);
    negative_storage_delta.mesh_end_storage_m3 = -0.75;
    let execution = trace_execution(Some(vec![negative_storage_delta]), true);
    write_laned_active_trace_output(&trace_targets(Some(path.clone())), &execution)
        .expect("signed mesh storage delta must remain writable");
    let value: serde_json::Value = serde_json::from_str(
        std::fs::read_to_string(&path)
            .expect("signed storage trace output")
            .trim_end(),
    )
    .expect("signed storage trace JSON");
    assert_eq!(value["mesh_end_storage_m3"], -0.75);

    let _ = std::fs::remove_file(path);
}

#[test]
fn cqr_execute_direct_publication_stream_runs_real_fixture_consumer() {
    let _fixed_point_audit_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_covered_fixed_point_iteration_audit_v1();
    let (report, output_dir) = execute_explicit_stage3_fixture_run("cqr_ha08_execute_stream");
    let fixed_point_audit = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_covered_fixed_point_iteration_audit_v1();
    assert!(fixed_point_audit.iter().any(|entry| {
        entry.converged
            && entry.support.duration_ns() == 60_000_000_000
            && (1..=96).contains(&entry.completed_iterations)
    }));
    assert!(fixed_point_audit.iter().any(|entry| {
        entry.converged
            && entry.support.duration_ns() > 60_000_000_000
            && (1..=96).contains(&entry.completed_iterations)
    }));
    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());
    assert!(report.manifest_path.is_file());
    let manifest = std::fs::read_to_string(&report.manifest_path).expect("run manifest");
    assert!(manifest.contains("R7C-DIRECT-PRODUCTION-EXECUTOR"));
    let _ = std::fs::remove_dir_all(output_dir);
}

#[test]
#[ignore = "diagnostic-only one-day qualification telemetry"]
#[allow(clippy::too_many_lines)]
fn cqr_stage3_one_day_qualification_with_telemetry() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    crate::hillslope::snow_stage3_v11_qualification_audit::begin();
    let _fixed_point_audit_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_covered_fixed_point_iteration_audit_v1();
    openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_adaptive_comparison_test_audit();
    openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_stage3_physical_outcome_closure_audit_v1();
    let run_dir = prepare_explicit_stage3_fixture_dir("cqr_stage3_one_day_telemetry", true);
    let output_dir = run_dir.join("output");
    let _telemetry_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_adaptive_parent_telemetry_v1(
        49,
        std::time::Duration::from_secs(3_600),
    )
    .expect("valid telemetry bound");
    let result =
        crate::hillslope::snow_stage3_v11_production_seed::with_explicit_test_owner_seed(|| {
            execute_hillslope_run_with_runtime_policy(
                &HillslopeRunRequest {
                    run_dir: run_dir.clone(),
                    run_file: PathBuf::from("case.run"),
                    output_dir,
                    sidecar_policy: SidecarPolicy::Compat,
                    legacy_sidecar_discovery: false,
                    manifest_path: None,
                },
                &["openwepp-cli-hill".to_string()],
                HillslopeRuntimeSelectionPolicy::new(
                    HillslopeRuntimeSelection::DirectProductionExecutor,
                    HillslopeDefaultRuntimeActivation::default(),
                ),
            )
        });
    let rows = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_adaptive_parent_telemetry_v1();
    for row in &rows {
        eprintln!(
            "STAGE3_PARENT_TELEMETRY ordinal={} support={}..{} direct={} split={} accepted={} rejected={} owner_joins={} event_groups={} terminal_parcels={} publication_supports={} publication_events={} adaptive_bytes={:?} coupled_inline_bytes={} owner_bytes={:?} parent_ms={} cumulative_ms={}",
            row.parent_ordinal,
            row.support.start_ns().get(),
            row.support.end_ns().get(),
            row.direct_trial_count,
            row.split_child_trial_count,
            row.accepted_microstep_count,
            row.rejected_candidate_count,
            row.owner_join_count,
            row.event_group_count,
            row.terminal_parcel_count,
            row.publication_support_count,
            row.publication_event_count,
            row.adaptive_receipt_bytes,
            row.coupled_receipt_inline_bytes,
            row.retained_complete_owner_bytes,
            row.parent_elapsed.as_millis(),
            row.cumulative_elapsed.as_millis(),
        );
        eprintln!(
            "STAGE3_PARENT_PHASES ordinal={} accepted_widths={:?} rejection_phase={} rejection_event={} rejection_both={} rejection_other={} covered_direct={}/{}ms covered_composed={}/{}ms terminal_direct={}/{}ms terminal_composed={}/{}ms fixed_point={}/{}iter/max{} fixed_point_phases=operands{}ms/envelope{}ms/stage3{}ms/soil{}ms/finalization{}ms envelope_subphases=projection{}ms/solver_ready{}ms/physical{}ms/receipts{}ms/owner{}ms publication_append={}/{}ms/cow{} publication_full_validation={}/{}ms reuse_validation={}/{}ms/hit{}/fallback{} child_memo=hit{}/direct{}/composed{}/fallback{}",
            row.parent_ordinal,
            row.accepted_width_histogram,
            row.phase_rejection_count,
            row.event_rejection_count,
            row.phase_and_event_rejection_count,
            row.other_rejection_count,
            row.covered_direct_trial_phase_count,
            row.covered_direct_trial_phase_elapsed.as_millis(),
            row.covered_composed_trial_phase_count,
            row.covered_composed_trial_phase_elapsed.as_millis(),
            row.terminal_direct_trial_phase_count,
            row.terminal_direct_trial_phase_elapsed.as_millis(),
            row.terminal_composed_trial_phase_count,
            row.terminal_composed_trial_phase_elapsed.as_millis(),
            row.fixed_point_evaluation_count,
            row.fixed_point_iteration_total,
            row.fixed_point_iteration_maximum,
            row.fixed_point_operand_elapsed.as_millis(),
            row.fixed_point_envelope_elapsed.as_millis(),
            row.fixed_point_stage3_elapsed.as_millis(),
            row.fixed_point_soil_elapsed.as_millis(),
            row.fixed_point_finalization_elapsed.as_millis(),
            row.provisional_envelope_projection_elapsed.as_millis(),
            row.provisional_envelope_solver_ready_elapsed.as_millis(),
            row.provisional_envelope_physical_elapsed.as_millis(),
            row.provisional_envelope_receipts_elapsed.as_millis(),
            row.provisional_envelope_owner_elapsed.as_millis(),
            row.publication_append_count,
            row.publication_append_elapsed.as_millis(),
            row.publication_cow_count,
            row.publication_full_validation_count,
            row.publication_full_validation_elapsed.as_millis(),
            row.reuse_validation_count,
            row.reuse_validation_elapsed.as_millis(),
            row.reuse_hit_count,
            row.reuse_fallback_count,
            row.covered_child_memo_hit_count,
            row.covered_child_memo_direct_hit_count,
            row.covered_child_memo_composed_hit_count,
            row.covered_child_memo_fallback_count,
        );
    }
    let fixed_point_audit = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_covered_fixed_point_iteration_audit_v1();
    let comparison_audit = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_adaptive_comparison_test_audit();
    let closure_audit = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_stage3_physical_outcome_closure_audit_v1();
    let comparison_rejections = comparison_audit
        .iter()
        .filter(|entry| {
            entry.maximum_scaled_error > 1.0 || entry.first_discrete_surface_kind.is_some()
        })
        .count();
    let comparison_scaled_rejections = comparison_audit
        .iter()
        .filter(|entry| entry.maximum_scaled_error > 1.0)
        .count();
    let comparison_discrete_rejections = comparison_audit
        .iter()
        .filter(|entry| entry.first_discrete_surface_kind.is_some())
        .count();
    let mut comparison_rejections_by_owner_path = BTreeMap::<(String, String), u64>::new();
    for entry in comparison_audit.iter().filter(|entry| {
        entry.maximum_scaled_error > 1.0 || entry.first_discrete_surface_kind.is_some()
    }) {
        *comparison_rejections_by_owner_path
            .entry((
                entry
                    .maximum_owner_id
                    .clone()
                    .unwrap_or_else(|| "none".to_owned()),
                entry
                    .maximum_path
                    .clone()
                    .unwrap_or_else(|| "none".to_owned()),
            ))
            .or_default() += 1;
    }
    let fixed_point_nonconverged = fixed_point_audit
        .iter()
        .filter(|entry| !entry.converged)
        .count();
    let receipt_reseal_max_abs_residual_j_m2 = fixed_point_audit
        .iter()
        .map(|entry| f64::from_bits(entry.receipt_reseal_max_abs_residual_bits))
        .fold(0.0_f64, f64::max);
    let receipt_reseal_max_abs_temperature_residual_k = fixed_point_audit
        .iter()
        .map(|entry| {
            f64::from_bits(entry.receipt_reseal_max_abs_temperature_residual_bits)
        })
        .fold(0.0_f64, f64::max);
    eprintln!(
        "STAGE3_LIMITING_REJECTIONS fixed_point_nonconverged={fixed_point_nonconverged} comparison_rejections={comparison_rejections} comparison_scaled={comparison_scaled_rejections} comparison_discrete={comparison_discrete_rejections} comparison_by_owner_path={comparison_rejections_by_owner_path:?}",
    );
    eprintln!(
        "STAGE3_LEDGER_CLOSURE validated={} maximum_abs_mass_residual_kg_m2={:.17e} mass_tolerance_kg_m2=1.0e-9 maximum_abs_energy_residual_j_m2={:.17e} energy_tolerance_j_m2=1.0e-6",
        closure_audit.validated_ledger_count,
        closure_audit.maximum_abs_mass_residual_kg_m2,
        closure_audit.maximum_abs_energy_residual_j_m2,
    );
    eprintln!(
        "STAGE3_RECEIPT_RESEAL maximum_abs_energy_residual_j_m2={:.17e} roundoff_bound_j_m2={:.17e} maximum_abs_temperature_residual_k={:.17e} temperature_bound_k={:.17e}",
        receipt_reseal_max_abs_residual_j_m2,
        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_J_M2,
        receipt_reseal_max_abs_temperature_residual_k,
        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_TEMPERATURE_K,
    );
    assert!(
        receipt_reseal_max_abs_residual_j_m2
            <= openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_J_M2
    );
    assert!(
        receipt_reseal_max_abs_temperature_residual_k
            <= openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_TEMPERATURE_K
    );
    let report = result.expect("one-day telemetry qualification must complete");
    let qualification = crate::hillslope::snow_stage3_v11_qualification_audit::take();
    let snapshot = qualification
        .committed_snapshot
        .as_ref()
        .expect("sealed one-day qualification snapshot");
    snapshot
        .validate()
        .expect("valid one-day qualification snapshot");

    assert_eq!(rows.len(), 48);
    assert_eq!(qualification.support_chronology_by_day.len(), 1);
    assert_eq!(qualification.support_chronology_by_day[&0].len(), 48);
    assert_eq!(snapshot.next_day_index, 1);
    assert_eq!(snapshot.committed_day_count, 1);
    assert_eq!(snapshot.total_parent_support_count, 48);
    assert_eq!(snapshot.adaptive_support_receipt_count, 48);
    assert_eq!(snapshot.snow_free_successor_receipt_count, 0);
    assert_eq!(snapshot.snow_free_parent_support_count, 0);
    let rejected_candidate_count = rows
        .iter()
        .map(|row| row.rejected_candidate_count)
        .sum::<u64>();
    let reconciled_rejection_count = rows
        .iter()
        .map(|row| {
            row.phase_rejection_count + row.event_rejection_count
                - row.phase_and_event_rejection_count
                + row.other_rejection_count
        })
        .sum::<u64>();
    assert_eq!(reconciled_rejection_count, rejected_candidate_count);
    assert!(closure_audit.validated_ledger_count > 0);
    assert!(closure_audit.maximum_abs_mass_residual_kg_m2 <= 1.0e-9);
    assert!(closure_audit.maximum_abs_energy_residual_j_m2 <= 1.0e-6);
    assert!(fixed_point_audit.iter().any(|entry| {
        entry.converged
            && entry.support.duration_ns() == 60_000_000_000
            && (1..=96).contains(&entry.completed_iterations)
    }));
    assert!(fixed_point_audit.iter().any(|entry| {
        entry.converged
            && entry.support.duration_ns() > 60_000_000_000
            && (1..=96).contains(&entry.completed_iterations)
    }));
    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());
    assert!(report.manifest_path.is_file());
    let _ = std::fs::remove_dir_all(run_dir);
}

#[test]
#[ignore = "diagnostic-only first covered-parent fixed-point attribution"]
fn cqr_stage3_first_covered_parent_fixed_point_diagnostic() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let _fixed_point_audit_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_covered_fixed_point_iteration_audit_v1();
    openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_adaptive_comparison_test_audit();
    let run_dir = prepare_explicit_stage3_fixture_dir("cqr_stage3_first_covered_parent", true);
    let output_dir = run_dir.join("output");
    let _telemetry_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_adaptive_parent_telemetry_v1(
        5,
        std::time::Duration::from_secs(590),
    )
    .expect("valid telemetry bound");
    let result =
        crate::hillslope::snow_stage3_v11_production_seed::with_explicit_test_owner_seed(|| {
            execute_hillslope_run_with_runtime_policy(
                &HillslopeRunRequest {
                    run_dir: run_dir.clone(),
                    run_file: PathBuf::from("case.run"),
                    output_dir,
                    sidecar_policy: SidecarPolicy::Compat,
                    legacy_sidecar_discovery: false,
                    manifest_path: None,
                },
                &["openwepp-cli-hill".to_string()],
                HillslopeRuntimeSelectionPolicy::new(
                    HillslopeRuntimeSelection::DirectProductionExecutor,
                    HillslopeDefaultRuntimeActivation::default(),
                ),
            )
        });
    let fixed_point_audit = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_covered_fixed_point_iteration_audit_v1();
    let comparison_audit = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_adaptive_comparison_test_audit();
    for entry in &comparison_audit {
        eprintln!("ADAPTIVE_ERROR_DIAGNOSTIC {entry:?}");
    }
    for entry in &fixed_point_audit {
        eprintln!(
            "COVERED_FP_DIAGNOSTIC support={}..{} width={} iterations={} converged={} receipt_reseal_max_abs_residual_j_m2={:.17e} receipt_reseal_max_abs_temperature_residual_k={:.17e} limit_detail={:?}",
            entry.support.start_ns().get(),
            entry.support.end_ns().get(),
            entry.support.end_ns().get() - entry.support.start_ns().get(),
            entry.completed_iterations,
            entry.converged,
            f64::from_bits(entry.receipt_reseal_max_abs_residual_bits),
            f64::from_bits(entry.receipt_reseal_max_abs_temperature_residual_bits),
            entry.limit_detail,
        );
    }
    let error = result.expect_err("five-parent gate must stop at its result-blind bound");
    assert!(
        format!("{error:?}").contains("diagnostic completed-parent telemetry stop"),
        "unexpected stop: {error:?}",
    );
    assert!(fixed_point_audit.iter().any(|entry| {
        entry.support.duration_ns() == 120_000_000_000
            && entry.converged
            && entry.completed_iterations <= 64
    }));
    assert!(fixed_point_audit.iter().any(|entry| {
        entry.support.duration_ns() > 120_000_000_000
            && !entry.converged
            && entry.completed_iterations == 96
    }));
    let _ = std::fs::remove_dir_all(run_dir);
}

#[test]
#[ignore = "focused real parent-0 adaptive publication cross-join"]
fn cqr_stage3_parent0_adaptive_publication_crossjoin() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let run_dir = prepare_explicit_stage3_fixture_dir("cqr_stage3_parent0_crossjoin", true);
    let output_dir = run_dir.join("output");
    let _telemetry_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_adaptive_parent_telemetry_v1(
        1,
        std::time::Duration::from_secs(30),
    )
    .expect("valid telemetry bound");
    let result =
        crate::hillslope::snow_stage3_v11_production_seed::with_explicit_test_owner_seed(|| {
            execute_hillslope_run_with_runtime_policy(
                &HillslopeRunRequest {
                    run_dir: run_dir.clone(),
                    run_file: PathBuf::from("case.run"),
                    output_dir,
                    sidecar_policy: SidecarPolicy::Compat,
                    legacy_sidecar_discovery: false,
                    manifest_path: None,
                },
                &["openwepp-cli-hill".to_string()],
                HillslopeRuntimeSelectionPolicy::new(
                    HillslopeRuntimeSelection::DirectProductionExecutor,
                    HillslopeDefaultRuntimeActivation::default(),
                ),
            )
        });
    let rows = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_adaptive_parent_telemetry_v1();
    let error = result.expect_err("one-parent gate must stop at its result-blind bound");
    let detail = format!("{error:?}");
    assert!(
        detail.contains("diagnostic completed-parent telemetry stop"),
        "unexpected stop: {detail}",
    );
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].parent_ordinal, 0);
    assert_eq!(rows[0].accepted_microstep_count, 24);
    assert_eq!(rows[0].owner_join_count, 25);
    assert_eq!(rows[0].publication_support_count, 25);
    assert_eq!(
        rows[0].accepted_width_histogram,
        vec![(60_000_000_000, 23), (420_000_000_000, 1)],
    );
    let _ = std::fs::remove_dir_all(run_dir);
}

#[test]
fn cqr_laned_active_pure_selector_decisions_cover_priority_and_configuration() {
    assert!(
        resolve_laned_active_decision(true, true, DirectLanedActiveDefaultEligibility::Absent)
            .expect_err("explicit selectors conflict")
            .to_string()
            .contains("mutually exclusive")
    );
    for (active, disabled, eligibility, expected) in [
        (
            true,
            false,
            DirectLanedActiveDefaultEligibility::Mixed {
                present: 1,
                absent: 1,
            },
            true,
        ),
        (
            false,
            true,
            DirectLanedActiveDefaultEligibility::Mixed {
                present: 1,
                absent: 1,
            },
            false,
        ),
        (
            false,
            false,
            DirectLanedActiveDefaultEligibility::Complete,
            true,
        ),
        (
            false,
            false,
            DirectLanedActiveDefaultEligibility::Absent,
            false,
        ),
    ] {
        assert_eq!(
            resolve_laned_active_decision(active, disabled, eligibility)
                .expect("selector decision"),
            expected
        );
    }
    let mixed = resolve_laned_active_decision(
        false,
        false,
        DirectLanedActiveDefaultEligibility::Mixed {
            present: 2,
            absent: 3,
        },
    )
    .expect_err("mixed default authority must fail closed");
    assert!(mixed.to_string().contains("2 lane(s) with coefficients"));
    assert!(mixed.to_string().contains("3 lane(s) without coefficients"));

    assert!(
        resolve_laned_active_configuration(true, true, false)
            .expect_err("active and shadow conflict")
            .to_string()
            .contains("mutually exclusive")
    );
    for (active, shadow, profile, expected) in [
        (true, false, true, (true, true)),
        (true, false, false, (true, false)),
        (false, true, true, (false, false)),
        (false, false, false, (false, false)),
    ] {
        assert_eq!(
            resolve_laned_active_configuration(active, shadow, profile)
                .expect("configuration decision"),
            expected
        );
    }
}

fn cqr_laned_active_configuration() -> DirectLanedActiveConfig {
    DirectLanedActiveConfig {
        lanes: vec![DirectLanedActiveLaneConfig {
            slplen_m: 100.0,
            width_m: 10.0,
            mean_gradient: 0.1,
            skin_friction_coefficient_ko: 500.0,
            form_drag_coefficient: 0.0,
            roughness_element_height_m: 0.0,
            roughness_concentration: 0.0,
            vegetation_drag_coefficient: 0.2,
            canopy_height_m: Some(0.5),
        }],
        mesh_policy: DirectLanedActiveMeshPolicy::production_default(),
        max_dt_s: 300.0,
        trace_enabled: false,
        trace_detail_filter: None,
        step_trace_enabled: false,
    }
}

fn cqr_laned_active_frame() -> DirectRunFrame {
    let identity = DirectRunIdentity::new(901, 902, 1, 1).expect("valid test identity");
    DirectRunFrame::skeleton(identity).expect("valid test frame")
}

#[test]
fn cqr_laned_active_apply_configuration_covers_inactive_active_and_profile_reset() {
    let config = cqr_laned_active_configuration();
    config
        .validate(1)
        .expect("test configuration must be valid");

    let mut inactive = cqr_laned_active_frame();
    apply_laned_active_configuration(&mut inactive, None, false);
    assert!(inactive.laned_active.is_none());

    openwepp_hillslope_orchestrator::ofe_routing::profile::set_enabled(false);
    let mut active = cqr_laned_active_frame();
    apply_laned_active_configuration(&mut active, Some(config.clone()), false);
    assert_eq!(active.laned_active.as_deref(), Some(&config));
    assert!(!openwepp_hillslope_orchestrator::ofe_routing::profile::enabled());

    openwepp_hillslope_orchestrator::ofe_routing::profile::set_enabled(true);
    openwepp_hillslope_orchestrator::ofe_routing::profile::count_solver_steps(7);
    let mut profiled = cqr_laned_active_frame();
    apply_laned_active_configuration(&mut profiled, Some(config.clone()), true);
    assert_eq!(profiled.laned_active.as_deref(), Some(&config));
    assert!(openwepp_hillslope_orchestrator::ofe_routing::profile::enabled());
    assert_eq!(
        openwepp_hillslope_orchestrator::ofe_routing::profile::snapshot_and_reset(),
        openwepp_hillslope_orchestrator::ofe_routing::profile::RoutingProfileSnapshot::default()
    );
    openwepp_hillslope_orchestrator::ofe_routing::profile::set_enabled(false);
}

#[test]
fn cqr_laned_active_summary_validation_covers_required_and_optional_states() {
    let summary = DirectLanedActiveRunSummary::default();
    assert_eq!(
        validate_laned_active_summary(true, Some(summary.clone())).expect("active summary"),
        Some(summary)
    );
    assert!(
        validate_laned_active_summary(false, None)
            .expect("inactive summary")
            .is_none()
    );
    let error = validate_laned_active_summary(true, None)
        .expect_err("active execution must require its summary");
    assert!(error.to_string().contains("active loop not engaged"));
}

#[test]
fn cqr_laned_active_configure_subprocess_probe() {
    if std::env::var("CQR_HA08_CONFIGURE_PROBE").as_deref() != Ok("1") {
        return;
    }
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/laned_shadow_h2637");
    let run_dir = copy_fixture_to_temp(&source, "cqr_ha08_configure_active");
    enable_native_routing_coefficients(&run_dir);
    let request = HillslopeRunRequest {
        run_dir: run_dir.clone(),
        run_file: PathBuf::from("p2637.run.toml"),
        output_dir: run_dir.join("output"),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: None,
    };
    let inputs = load_hillslope_run_inputs(&request).expect("authoritative fixture inputs");
    let targets = resolve_hillslope_output_targets(&inputs.runfile).expect("output targets");
    let sidecars = resolve_hillslope_sidecars(&request, &inputs, &targets).expect("sidecars");
    let setup = build_static_hillslope_runtime_setup(
        &request,
        &inputs,
        &sidecars,
        HillslopeRuntimeSelection::DirectProductionExecutor,
    )
    .expect("static runtime setup");
    let HillslopeClimateExecutionState {
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        lane_context,
        climate_span,
    } = setup.execution_state;
    let climate_request =
        build_hillslope_climate_runtime_request(&inputs.climate).expect("climate runtime request");
    let seed_authority = DirectProductionSeedAuthority::from_typed_inputs(
        &climate_request,
        &inputs,
        &sidecars,
        per_ofe_lane_areas_m2.len(),
        lane_context.lane,
    )
    .expect("production seed authority");
    let mut frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
        output_hillslope_id: targets.output_hillslope_id,
        lane_areas_m2: &per_ofe_lane_areas_m2,
        runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
        day_count: climate_span.days.len(),
        seed_authority: &seed_authority,
    })
    .expect("production frame");
    let builder =
        DirectProductionDayInputBuilder::new(&climate_request, &climate_span, &seed_authority)
            .expect("day input builder");
    assert_eq!(
        configure_laned_active_execution(&mut frame, &builder).expect("active configuration"),
        (true, false)
    );
    assert!(frame.laned_active.is_some());
    let _ = std::fs::remove_dir_all(run_dir);
}

#[test]
fn cqr_laned_active_configure_wrapper_builds_authoritative_active_config_without_routing() {
    let test_name =
        "hillslope::tests::cqr_laned_active_outputs::cqr_laned_active_configure_subprocess_probe";
    let status = std::process::Command::new(std::env::current_exe().expect("test executable"))
        .args(["--exact", test_name, "--nocapture"])
        .env("CQR_HA08_CONFIGURE_PROBE", "1")
        .env("OPENWEPP_LANED_ACTIVE", "1")
        .env_remove("OPENWEPP_LANED_ACTIVE_DISABLE")
        .env_remove("OPENWEPP_LANED_SHADOW")
        .env_remove("OPENWEPP_LANED_SHADOW_PROFILE")
        .status()
        .expect("configure probe process");
    assert!(status.success(), "configure subprocess failed");
}

fn execute_scoped_selector_fixture(
    prefix: &str,
) -> (Result<HillslopeRunReport, HillslopeCliError>, PathBuf) {
    reset_direct_runtime_audit_counters();
    let uses_laned_fixture = prefix.contains("profile") || prefix.contains("shadow_only");
    let source_fixture_dir = if uses_laned_fixture {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/laned_shadow_h2637")
    } else {
        fixture_path("hillslope_run_dir")
    };
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);
    if uses_laned_fixture {
        enable_native_routing_coefficients(&temp_run_dir);
    }
    let result = execute_hillslope_run_with_runtime_policy(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from(if uses_laned_fixture {
                "p2637.run.toml"
            } else {
                "case.run"
            }),
            output_dir: temp_run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    );
    (result, temp_run_dir)
}

fn enable_native_routing_coefficients(run_dir: &Path) {
    let path = run_dir.join("p2637.man");
    let text = std::fs::read_to_string(&path).expect("management fixture readable");
    let mut patched = String::with_capacity(text.len() + 19 * 64);
    let mut inserted = 0_usize;
    for (line_index, line) in text.lines().enumerate() {
        let line = if line_index == 0 {
            "ow-lanuse-1".to_string()
        } else {
            line.replace("1 # Landuse - <Cropland>", "4 # Landuse - <NativeCropland>")
        };
        patched.push_str(&line);
        patched.push('\n');
        if line.starts_with("-40.00000 ") && line.trim_end().ends_with(" 0.00000") {
            patched.push_str("routing_coefficients\n500.0 0.0 0.0 0.0 0.0\n");
            inserted += 1;
        }
    }
    assert_eq!(inserted, 19, "all H2637 plant scenarios must be patched");
    std::fs::write(path, patched).expect("patched management fixture writable");
}

#[test]
fn cqr_selector_subprocess_probe() {
    let Ok(case) = std::env::var("CQR_HA08_SELECTOR_CASE") else {
        return;
    };
    let (result, temp) = execute_scoped_selector_fixture(&format!("cqr_ha08_{case}"));
    assert!(
        matches!(
            case.as_str(),
            "conflict" | "disable" | "shadow" | "shadow_only" | "profile"
        ),
        "unknown selector probe {case}"
    );
    let error = result.expect_err("retired Lane-D selector must fail closed at the V11 owner");
    let detail = error.to_string();
    for required in [
        "stage3_v11_owner",
        "constitutive Stage-3/V11 owner is the sole production hydrology/routing path",
        "retired Lane-D selector",
        "is not admitted",
    ] {
        assert!(
            detail.contains(required),
            "selector case {case} missing fail-closed detail {required}: {detail}"
        );
    }
    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn cqr_laned_active_selectors_cover_disable_conflict_shadow_and_profile_paths() {
    let test_name = "hillslope::tests::cqr_laned_active_outputs::cqr_selector_subprocess_probe";
    for case in ["conflict", "disable", "shadow", "shadow_only", "profile"] {
        let status = std::process::Command::new(std::env::current_exe().expect("test executable"))
            .args(["--exact", test_name, "--nocapture"])
            .env("CQR_HA08_SELECTOR_CASE", case)
            .env_remove("OPENWEPP_LANED_ACTIVE")
            .env_remove("OPENWEPP_LANED_ACTIVE_DISABLE")
            .env_remove("OPENWEPP_LANED_SHADOW")
            .env_remove("OPENWEPP_LANED_SHADOW_PROFILE")
            .envs(match case {
                "conflict" => vec![
                    ("OPENWEPP_LANED_ACTIVE", "1"),
                    ("OPENWEPP_LANED_ACTIVE_DISABLE", "1"),
                ],
                "disable" => vec![("OPENWEPP_LANED_ACTIVE_DISABLE", "1")],
                "shadow" => vec![
                    ("OPENWEPP_LANED_ACTIVE", "1"),
                    ("OPENWEPP_LANED_SHADOW", "1"),
                ],
                "shadow_only" => vec![
                    ("OPENWEPP_LANED_ACTIVE_DISABLE", "1"),
                    ("OPENWEPP_LANED_SHADOW", "1"),
                ],
                "profile" => vec![
                    ("OPENWEPP_LANED_ACTIVE", "1"),
                    ("OPENWEPP_LANED_SHADOW_PROFILE", "1"),
                ],
                _ => Vec::new(),
            })
            .status()
            .expect("selector probe process");
        assert!(status.success(), "selector subprocess {case} failed");
    }
}
