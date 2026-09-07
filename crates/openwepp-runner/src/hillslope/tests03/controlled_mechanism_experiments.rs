// Common observation-only workload for EXP-STAGE3-20260906; no treatment selector.
#[test]
#[ignore = "controlled release experiment; fresh process and explicit environment required"]
fn stage3_controlled_mechanism_experiment() {
    assert!(!cfg!(debug_assertions), "release build required");
    let ofes: usize = std::env::var("OPENWEPP_EXPERIMENT_OFES")
        .unwrap_or_else(|_| "1".to_owned())
        .parse()
        .expect("OFE integer");
    assert!([1, 10, 19].contains(&ofes), "bounded workload");
    let repeats: usize = std::env::var("OPENWEPP_EXPERIMENT_REPEATS")
        .unwrap_or_else(|_| "1".to_owned())
        .parse()
        .expect("repeat integer");
    assert!([1, 10].contains(&repeats), "bounded teardown horizon");
    let memory = std::env::var("OPENWEPP_EXPERIMENT_MEMORY").is_ok_and(|value| value == "1");
    assert_eq!(
        release_scale_cpu_affinity_count(&release_scale_cpu_affinity().expect("affinity")),
        Some(1)
    );
    for iteration in 0..repeats {
        let record = controlled_mechanism_run(ofes, iteration, memory);
        println!("STAGE3_CONTROLLED_MECHANISM {record}");
        drop(record);
        controlled_mechanism_phase("post_drop", iteration, memory);
    }
}

fn controlled_mechanism_phase(phase: &str, iteration: usize, enabled: bool) {
    if !enabled {
        return;
    }
    let monotonic_ns = controlled_mechanism_monotonic_ns();
    let status = std::fs::read_to_string("/proc/self/status").expect("memory status");
    let mappings =
        std::fs::read_to_string("/proc/self/smaps_rollup").expect("boundary mapping snapshot");
    let mapping_fields = [
        "Rss:",
        "Anonymous:",
        "Private_Clean:",
        "Private_Dirty:",
        "Pss_Anon:",
        "Pss_File:",
    ];
    let mut values = std::collections::BTreeMap::new();
    for field in mapping_fields {
        values.insert(
            field,
            release_scale_status_kib(&mappings, field).expect("required mapping field"),
        );
    }
    let record = serde_json::json!({
        "phase":phase, "iteration":iteration, "pid":std::process::id(), "monotonic_ns":monotonic_ns,
        "rss_kib":release_scale_status_kib(&status, "VmRSS:").expect("VmRSS"),
        "hwm_kib":release_scale_status_kib(&status, "VmHWM:").expect("VmHWM"),
        "mapping_kib":values,
    });
    println!("STAGE3_CONTROLLED_MEMORY {record}");
}

// Linux experiment clock shared with Python time.monotonic_ns in the parent.
// Runtime physics never calls this observation-only helper.
fn controlled_mechanism_monotonic_ns() -> u64 {
    let time = rustix::time::clock_gettime(rustix::time::ClockId::Monotonic);
    let seconds = u64::try_from(time.tv_sec).expect("nonnegative monotonic seconds");
    let nanos = u64::try_from(time.tv_nsec).expect("nonnegative nanoseconds");
    assert!(nanos < 1_000_000_000);
    seconds
        .checked_mul(1_000_000_000)
        .and_then(|value| value.checked_add(nanos))
        .expect("monotonic time fits u64")
}

#[test]
fn controlled_mechanism_clock_is_monotonic() {
    let first = controlled_mechanism_monotonic_ns();
    let second = controlled_mechanism_monotonic_ns();
    assert!(second >= first);
}

fn controlled_mechanism_output_evidence(
    root: &Path,
) -> (
    std::collections::BTreeMap<String, String>,
    serde_json::Value,
) {
    fn collect(root: &Path, dir: &Path, out: &mut std::collections::BTreeMap<String, String>) {
        for entry in std::fs::read_dir(dir).expect("output inventory") {
            let path = entry.expect("output entry").path();
            if path.is_dir() {
                collect(root, &path, out);
            } else if path
                .file_name()
                .is_none_or(|name| name != "openwepp_hillslope_run_manifest.json")
            {
                out.insert(
                    path.strip_prefix(root)
                        .expect("relative output")
                        .to_string_lossy()
                        .into_owned(),
                    release_scale_sha256_file(&path),
                );
            }
        }
    }
    let mut files = std::collections::BTreeMap::new();
    collect(root, root, &mut files);
    let manifest = serde_json::from_slice(
        &std::fs::read(root.join("openwepp_hillslope_run_manifest.json")).expect("manifest bytes"),
    )
    .expect("manifest JSON");
    // Keep the complete unnormalized manifest. Offline comparison uses an explicit field allowlist.
    (files, manifest)
}

fn controlled_mechanism_run(ofe_count: usize, iteration: usize, memory: bool) -> serde_json::Value {
    assert!(
        !cfg!(debug_assertions),
        "this ignored comparator probe must execute under cargo test --release"
    );
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    controlled_mechanism_phase("pre_fixture", iteration, memory);
    let areas_m2 = (1..=ofe_count)
        .map(|ofe| 100.0 * release_scale_usize_f64(ofe))
        .collect::<Vec<_>>();
    let fixture_started = std::time::Instant::now();
    let run_dir = prepare_native_stage3_lane_d_scale_fixture(
        "stage3_laned_release_one_ofe_positive_baseline",
        &areas_m2,
    );
    let fixture_wall_us = release_probe_elapsed_us(fixture_started.elapsed());
    let request = HillslopeRunRequest {
        run_dir: run_dir.clone(),
        run_file: PathBuf::from("case.run"),
        output_dir: run_dir.join("output"),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: None,
    };

    let bootstrap_started = std::time::Instant::now();
    let seed_path = crate::hillslope::test_fixture_authority::author_stage3_v11_owner_seed_fixture(
        &request,
        crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
        crate::hillslope::test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
    )
    .expect("author exact one-OFE release-probe Stage-3 owner seed");
    assert_authored_stage3_owner_cardinality(&request, &seed_path, ofe_count);
    let bootstrap_wall_us = release_probe_elapsed_us(bootstrap_started.elapsed());

    crate::hillslope::snow_stage3_v11_qualification_audit::begin();
    let telemetry_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_adaptive_parent_telemetry_v1(
        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::STAGE3_V11_PARENT_SUPPORT_COUNT * ofe_count,
        std::time::Duration::from_secs(3_600),
    )
    .expect("enable bounded Stage-3 owner-evaluation telemetry");
    let qualification_telemetry_guard = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_release_qualification_telemetry_v1()
        .expect("enable one-OFE release attribution telemetry");
    let (input_sha256, input_files) = release_scale_tree_hashes(&run_dir);
    let clock_hz = release_scale_clock_tick_hz().expect("Linux CLK_TCK");
    let trace_dir = std::env::var_os("OPENWEPP_EXPERIMENT_TRACE_DIR").map(PathBuf::from);
    let carrier_mode = if trace_dir.is_some() {
        openwepp_hillslope_orchestrator::stage3_mechanism_experiment_audit::Mode::Detailed {
            max_records: 100_000,
        }
    } else {
        openwepp_hillslope_orchestrator::stage3_mechanism_experiment_audit::Mode::Compact
    };
    let carrier_session =
        openwepp_hillslope_orchestrator::stage3_mechanism_experiment_audit::begin(carrier_mode)
            .expect("begin carrier observation");
    let lse_session = openwepp_land_surface_energy::solver_mechanism_audit::begin_mechanism_audit(
        trace_dir.is_some(),
        1_000_000,
    )
    .expect("begin LSE observation");
    controlled_mechanism_phase("pre_run", iteration, memory);
    let cpu_before = release_scale_process_cpu_ticks().expect("pre-run CPU ticks");
    let run_start_monotonic_ns = controlled_mechanism_monotonic_ns();
    let run_started = std::time::Instant::now();
    let report = execute_hillslope_run_with_runtime_policy(
        &request,
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    )
    .expect("one-OFE positive Stage-3 -> Lane-D release probe");
    let run_wall_us = release_probe_elapsed_us(run_started.elapsed());
    let run_end_monotonic_ns = controlled_mechanism_monotonic_ns();
    let cpu_after = release_scale_process_cpu_ticks().expect("end-run CPU ticks");
    controlled_mechanism_phase("end_run", iteration, memory);
    let mut carrier_audit = carrier_session
        .finish()
        .expect("finish carrier observation");
    let mut lse_audit = lse_session.finish().expect("finish LSE observation");
    assert_eq!(
        carrier_audit.dropped_records, 0,
        "complete carrier evidence"
    );
    assert_eq!(lse_audit.dropped_events, 0, "complete LSE evidence");
    if let Some(trace_dir) = trace_dir {
        std::fs::create_dir_all(&trace_dir).expect("create explicit trace directory");
        std::fs::write(
            trace_dir.join(format!("carrier-{iteration}.json")),
            serde_json::to_vec(&carrier_audit).expect("carrier trace JSON"),
        )
        .expect("write carrier trace");
        std::fs::write(
            trace_dir.join(format!("lse-{iteration}.json")),
            serde_json::to_vec(&lse_audit).expect("LSE trace JSON"),
        )
        .expect("write LSE trace");
    }
    carrier_audit.records.clear();
    lse_audit.events.clear();
    let qualification_telemetry = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_release_qualification_telemetry_v1()
        .expect("take one-OFE release attribution telemetry");
    drop(qualification_telemetry_guard);

    let validation_started = std::time::Instant::now();
    let stage3_audit = crate::hillslope::snow_stage3_v11_qualification_audit::take();
    let telemetry = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_adaptive_parent_telemetry_v1();
    drop(telemetry_guard);
    let snapshot = stage3_audit
        .committed_snapshot
        .as_ref()
        .expect("release probe committed Stage-3 qualification snapshot");
    snapshot
        .validate()
        .expect("release probe committed Stage-3 qualification evidence");
    assert_eq!(snapshot.committed_day_count, 1);
    assert_eq!(snapshot.lanes.len(), ofe_count);
    assert_eq!(
        stage3_audit
            .support_chronology_by_day
            .get(&0)
            .expect("release probe day-zero parent supports")
            .len(),
        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::STAGE3_V11_PARENT_SUPPORT_COUNT
            * ofe_count,
    );

    let manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&report.manifest_path)
            .expect("read release-probe Lane-D manifest"),
    )
    .expect("parse release-probe Lane-D manifest");
    let active = &manifest["execution_provenance"]["laned_active"];
    assert_eq!(active["days_seen"].as_u64(), Some(1));
    assert_eq!(active["days_routed"].as_u64(), Some(1));

    let hbp_bytes = std::fs::read(&report.output_pass).expect("read release-probe routed HBP");
    let (hbp, latest_event) = parse_hbp_from_bytes_with_latest_event_payload(
        &hbp_bytes,
        &report.output_pass,
        HbpParseOptions {
            expected_hillslope_id: Some(83),
        },
    )
    .expect("release-probe routed HBP must parse");
    assert_eq!(usize::try_from(hbp.nofe).expect("OFE count"), ofe_count);
    let event = latest_event.expect("release probe must publish a positive routed HBP event");
    let wat_path = run_dir.join("output/H83.wat.parquet");
    let evidence = RealLaneDPublicEvidence {
        ofe_count,
        areas_m2: read_wat_f64_column(&wat_path, "Area"),
        source_depths_mm: read_wat5_hourly_sources(
            &run_dir.join("output/H83.wat-subhourly.parquet"),
            ofe_count,
        ),
        pass_runvol_m3: read_wat_f64_column(&run_dir.join("output/H83.pass.parquet"), "runvol"),
        pass_peakro_m3_s: read_wat_f64_column(&run_dir.join("output/H83.pass.parquet"), "peakro"),
        hbp_hourly_outlet_m3: event.hourly_runoff_volume_m3,
        hbp_peak_outlet_m3_s: event.peak_runoff_m3_s,
        manifest_source_m3: positive_manifest_f64(active, "total_source_m3"),
        manifest_outlet_m3: positive_manifest_f64(active, "total_routed_outlet_m3"),
        manifest_storage_m3: nonnegative_manifest_f64(active, "total_end_window_storage_m3"),
        manifest_clamp_m3: nonnegative_manifest_f64(active, "total_clamp_m3"),
    };
    assert_real_lane_d_public_closure(&evidence, &areas_m2);
    let fixed_point_evaluation_count = telemetry
        .iter()
        .map(|row| row.fixed_point_evaluation_count)
        .sum::<u64>();
    let direct_trial_count = telemetry
        .iter()
        .map(|row| row.direct_trial_count)
        .sum::<u64>();
    let split_child_trial_count = telemetry
        .iter()
        .map(|row| row.split_child_trial_count)
        .sum::<u64>();
    let accepted_microstep_count = telemetry
        .iter()
        .map(|row| row.accepted_microstep_count)
        .sum::<u64>();
    let telemetry_elapsed_us = |project: fn(
        &openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::AdaptiveParentTelemetryV1,
    ) -> std::time::Duration| {
        release_probe_elapsed_us(telemetry.iter().map(project).sum())
    };
    let parent_phase_wall_us = serde_json::json!({
        "parent": telemetry_elapsed_us(|row| row.parent_elapsed),
        "covered_direct_trial": telemetry_elapsed_us(|row| row.covered_direct_trial_phase_elapsed),
        "covered_composed_trial": telemetry_elapsed_us(|row| row.covered_composed_trial_phase_elapsed),
        "terminal_direct_trial": telemetry_elapsed_us(|row| row.terminal_direct_trial_phase_elapsed),
        "terminal_composed_trial": telemetry_elapsed_us(|row| row.terminal_composed_trial_phase_elapsed),
        "provisional_projection": telemetry_elapsed_us(|row| row.provisional_envelope_projection_elapsed),
        "provisional_solver_ready": telemetry_elapsed_us(|row| row.provisional_envelope_solver_ready_elapsed),
        "provisional_physical": telemetry_elapsed_us(|row| row.provisional_envelope_physical_elapsed),
        "provisional_receipts": telemetry_elapsed_us(|row| row.provisional_envelope_receipts_elapsed),
        "provisional_owner": telemetry_elapsed_us(|row| row.provisional_envelope_owner_elapsed),
        "publication_append": telemetry_elapsed_us(|row| row.publication_append_elapsed),
        "publication_full_validation": telemetry_elapsed_us(|row| row.publication_full_validation_elapsed),
        "reuse_validation": telemetry_elapsed_us(|row| row.reuse_validation_elapsed),
    });
    let physical_phase_wall_us = serde_json::json!({
        "topology": telemetry_elapsed_us(|row| row.profile_detail.physical_topology_elapsed),
        "potential": telemetry_elapsed_us(|row| row.profile_detail.physical_potential_elapsed),
        "request": telemetry_elapsed_us(|row| row.profile_detail.physical_request_elapsed),
        "unified": telemetry_elapsed_us(|row| row.profile_detail.physical_unified_elapsed),
        "final_tile": telemetry_elapsed_us(|row| row.profile_detail.physical_final_tile_elapsed),
        "protocol": telemetry_elapsed_us(|row| row.profile_detail.physical_protocol_elapsed),
        "ingress": telemetry_elapsed_us(|row| row.profile_detail.physical_ingress_elapsed),
        "post": telemetry_elapsed_us(|row| row.profile_detail.physical_post_elapsed),
    });
    let unified_phase_wall_us = serde_json::json!({
        "preflight": telemetry_elapsed_us(|row| row.profile_detail.unified_preflight_elapsed),
        "authorization": telemetry_elapsed_us(|row| row.profile_detail.unified_authorization_elapsed),
        "entry_validation": telemetry_elapsed_us(|row| row.profile_detail.unified_entry_validation_elapsed),
        "protocol_validation": telemetry_elapsed_us(|row| row.profile_detail.unified_protocol_validation_elapsed),
        "candidate": telemetry_elapsed_us(|row| row.profile_detail.unified_candidate_elapsed),
        "candidate_soil": telemetry_elapsed_us(|row| row.profile_detail.candidate_soil_elapsed),
        "candidate_surface_resource": telemetry_elapsed_us(|row| row.profile_detail.candidate_surface_resource_elapsed),
        "candidate_surface_ingress": telemetry_elapsed_us(|row| row.profile_detail.candidate_surface_ingress_elapsed),
        "candidate_receivers": telemetry_elapsed_us(|row| row.profile_detail.candidate_receivers_elapsed),
        "candidate_validation": telemetry_elapsed_us(|row| row.profile_detail.candidate_validation_elapsed),
    });
    let finalization_phase_wall_us = serde_json::json!({
        "candidate": telemetry_elapsed_us(|row| row.profile_detail.finalization_candidate_elapsed),
        "sealed_source": telemetry_elapsed_us(|row| row.profile_detail.finalization_sealed_source_elapsed),
        "install": telemetry_elapsed_us(|row| row.profile_detail.finalization_install_elapsed),
        "identity_replay": telemetry_elapsed_us(|row| row.profile_detail.finalization_identity_replay_elapsed),
    });
    let imported_stack_phase_wall_us = serde_json::json!({
        "entry_validation": telemetry_elapsed_us(|row| row.profile_detail.imported_entry_validation_elapsed),
        "physical_candidate": telemetry_elapsed_us(|row| row.profile_detail.imported_physical_candidate_elapsed),
        "physical_setup": telemetry_elapsed_us(|row| row.profile_detail.imported_physical_setup_elapsed),
        "frozen_evaluation": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_evaluation_elapsed),
        "frozen_preparation": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_preparation_elapsed),
        "frozen_execute_accept": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_execute_accept_elapsed),
        "frozen_execution_setup": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_execution_setup_elapsed),
        "frozen_runtime": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_runtime_elapsed),
        "frozen_acceptance": telemetry_elapsed_us(|row| row.profile_detail.imported_frozen_acceptance_elapsed),
        "envelope_construction": telemetry_elapsed_us(|row| row.profile_detail.imported_envelope_construction_elapsed),
        "envelope_validation": telemetry_elapsed_us(|row| row.profile_detail.imported_envelope_validation_elapsed),
        "accepted_candidate": telemetry_elapsed_us(|row| row.profile_detail.imported_accepted_candidate_elapsed),
        "owner_publication": telemetry_elapsed_us(|row| row.profile_detail.imported_owner_publication_elapsed),
        "install": telemetry_elapsed_us(|row| row.profile_detail.imported_install_elapsed),
        "reuse_validation": telemetry_elapsed_us(|row| row.profile_detail.imported_reuse_validation_elapsed),
        "reuse_reseal": telemetry_elapsed_us(|row| row.profile_detail.imported_reuse_reseal_elapsed),
        "reuse_install": telemetry_elapsed_us(|row| row.profile_detail.imported_reuse_install_elapsed),
    });
    let terminal_candidate_phase_wall_us = serde_json::json!({
        "setup": telemetry_elapsed_us(|row| row.profile_detail.terminal_candidate_setup_elapsed),
        "provider_custody": telemetry_elapsed_us(|row| row.profile_detail.terminal_provider_custody_elapsed),
        "provider_projection": telemetry_elapsed_us(|row| row.profile_detail.terminal_provider_projection_elapsed),
        "provider_carrier": telemetry_elapsed_us(|row| row.profile_detail.terminal_provider_carrier_elapsed),
        "provider_retention": telemetry_elapsed_us(|row| row.profile_detail.terminal_provider_retention_elapsed),
        "result_finalization": telemetry_elapsed_us(|row| row.profile_detail.terminal_result_finalization_elapsed),
        "carrier_physical": telemetry_elapsed_us(|row| row.profile_detail.carrier_physical_phase_elapsed),
        "carrier_complete": telemetry_elapsed_us(|row| row.profile_detail.carrier_complete_phase_elapsed),
        "carrier_physical_setup": telemetry_elapsed_us(|row| row.profile_detail.carrier_physical_setup_elapsed),
        "carrier_physical_evidence": telemetry_elapsed_us(|row| row.profile_detail.carrier_physical_evidence_elapsed),
        "carrier_physical_completion": telemetry_elapsed_us(|row| row.profile_detail.carrier_physical_completion_elapsed),
        "carrier_complete_envelope": telemetry_elapsed_us(|row| row.profile_detail.carrier_complete_envelope_elapsed),
        "carrier_complete_adoption": telemetry_elapsed_us(|row| row.profile_detail.carrier_complete_adoption_elapsed),
        "carrier_complete_projection": telemetry_elapsed_us(|row| row.profile_detail.carrier_complete_projection_elapsed),
        "carrier_complete_owner": telemetry_elapsed_us(|row| row.profile_detail.carrier_complete_owner_elapsed),
        "carrier_owner_vegetation_validation": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_vegetation_validation_elapsed),
        "carrier_owner_hydrology_projection": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_hydrology_projection_elapsed),
        "carrier_owner_surface_canonical": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_surface_canonical_elapsed),
        "carrier_owner_vegetation_encoding": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_vegetation_encoding_elapsed),
        "carrier_owner_surface_encoding": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_surface_encoding_elapsed),
        "carrier_owner_soil_encoding": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_soil_encoding_elapsed),
        "carrier_owner_other_encoding": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_other_encoding_elapsed),
        "carrier_owner_joint_map": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_joint_map_elapsed),
        "carrier_owner_joint_seal": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_joint_seal_elapsed),
        "carrier_owner_soil_custody": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_soil_custody_elapsed),
        "carrier_owner_candidate_bytes": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_candidate_bytes_elapsed),
        "carrier_owner_ephemeral_assembly": telemetry_elapsed_us(|row| row.profile_detail.carrier_owner_ephemeral_assembly_elapsed),
    });
    let validation_wall_us = release_probe_elapsed_us(validation_started.elapsed());

    let (output_files, output_manifest) = controlled_mechanism_output_evidence(&request.output_dir);
    let mut record = serde_json::json!({
        "experiment": "EXP-STAGE3-20260906",
        "carrier_counts": carrier_audit,
        "lse_counts": lse_audit,
        "ofe_count": ofe_count,
        "iteration": iteration,
        "cpu_ticks": cpu_after.checked_sub(cpu_before).expect("monotonic CPU"),
        "clock_tick_hz": clock_hz,
        "run_start_monotonic_ns": run_start_monotonic_ns,
        "run_end_monotonic_ns": run_end_monotonic_ns,
        "input_sha256": input_sha256,
        "input_files": input_files,
        "output_files": output_files,
        "output_manifest": output_manifest,
        "closure_operands": {
            "areas_m2": evidence.areas_m2,
            "wat5_source_depths_mm": evidence.source_depths_mm,
            "hbp_hourly_outlet_m3": evidence.hbp_hourly_outlet_m3,
            "pass_runvol_m3": evidence.pass_runvol_m3,
            "pass_peakro_m3_s": evidence.pass_peakro_m3_s,
        },
    });
    let original_record = serde_json::json!({
        "record": "stage3_laned_release_one_ofe_positive_baseline_v1",
        "fixture_wall_us": fixture_wall_us,
        "bootstrap_wall_us": bootstrap_wall_us,
        "run_wall_us": run_wall_us,
        "validation_wall_us": validation_wall_us,
        "rss_kib": release_probe_rss_kib(),
        "committed_day_count": snapshot.committed_day_count,
        "total_parent_support_count": snapshot.total_parent_support_count,
        "covered_parent_support_count": snapshot.adaptive_support_receipt_count,
        "snow_free_parent_support_count": snapshot.snow_free_parent_support_count,
        "accepted_publication_support_count": snapshot.accepted_publication_support_count,
        "fixed_point_evaluation_count": fixed_point_evaluation_count,
        "direct_trial_count": direct_trial_count,
        "split_child_trial_count": split_child_trial_count,
        "accepted_microstep_count": accepted_microstep_count,
        "parent_phase_wall_us": parent_phase_wall_us,
        "physical_phase_wall_us": physical_phase_wall_us,
        "unified_phase_wall_us": unified_phase_wall_us,
        "finalization_phase_wall_us": finalization_phase_wall_us,
        "imported_stack_phase_wall_us": imported_stack_phase_wall_us,
        "terminal_candidate_phase_wall_us": terminal_candidate_phase_wall_us,
        "native_vegetation_et_wall_us": release_probe_elapsed_us(qualification_telemetry.native_vegetation_et_elapsed),
        "stage3_lse_soil_wall_us": release_probe_elapsed_us(qualification_telemetry.stage3_lse_soil_elapsed),
        "lane_d_wall_us": release_probe_elapsed_us(qualification_telemetry.lane_d_elapsed),
        "remaining_runner_wall_us": release_probe_elapsed_us(qualification_telemetry.remaining_runner_elapsed),
        "qualification_telemetry_total_wall_us": release_probe_elapsed_us(qualification_telemetry.total_elapsed),
        "qualification_scopes_balanced": qualification_telemetry.scopes_balanced,
        "qualification_counters_complete": qualification_telemetry.counters_complete,
        "laned_days_seen": active["days_seen"].as_u64().expect("Lane-D days_seen"),
        "laned_days_routed": active["days_routed"].as_u64().expect("Lane-D days_routed"),
        "laned_source_m3": evidence.manifest_source_m3,
        "laned_outlet_m3": evidence.manifest_outlet_m3,
        "laned_end_window_storage_m3": evidence.manifest_storage_m3,
        "laned_clamp_m3": evidence.manifest_clamp_m3,
    });
    record
        .as_object_mut()
        .expect("experiment object")
        .extend(original_record.as_object().expect("probe object").clone());
    drop(original_record);
    controlled_mechanism_phase("post_validation", iteration, memory);
    // These explicit drops precede cleanup observation; unlinking files is not heap teardown.
    drop(report);
    drop(stage3_audit);
    drop(telemetry);
    drop(qualification_telemetry);
    drop(carrier_audit);
    drop(lse_audit);
    drop(manifest);
    drop(hbp_bytes);
    drop(hbp);
    drop(evidence);
    std::fs::remove_dir_all(run_dir).expect("remove experiment-owned fixture after validation");
    record
}
