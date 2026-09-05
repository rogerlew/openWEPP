#[test]
#[ignore = "complete-season adaptive qualification; run only after the short chronology gate releases"]
#[allow(clippy::too_many_lines)]
fn complete_season_adaptive_reappearance_routed_water_and_bgc_runner_fixture() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let run_dir =
        prepare_two_ofe_complete_season_fixture("stage3_complete_season_reappearance_routing_bgc");
    let started = std::time::Instant::now();
    crate::hillslope::snow_stage3_v11_qualification_audit::begin();
    let report =
        crate::hillslope::snow_stage3_v11_production_seed::with_explicit_two_ofe_test_owner_seed(
            || {
                execute_hillslope_run_with_runtime_policy(
                    &HillslopeRunRequest {
                        run_dir: run_dir.clone(),
                        run_file: PathBuf::from("case.run"),
                        output_dir: run_dir.join("output"),
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
                .expect("two-OFE complete-season adaptive runner")
            },
        );
    let elapsed = started.elapsed();
    let audit = crate::hillslope::snow_stage3_v11_qualification_audit::take();
    assert_complete_season_jit_supports(&audit);
    let snapshot = audit
        .committed_snapshot
        .as_ref()
        .expect("sealed two-OFE season qualification snapshot");
    snapshot.validate().expect("sealed qualification evidence");
    assert_eq!(snapshot.next_day_index, COMPLETE_SEASON_DAY_COUNT);
    assert_eq!(snapshot.lanes.len(), 2);
    assert_eq!(snapshot.soil_thermal_ofes.len(), 2);
    assert!(snapshot.terminal_event_count > 0);
    assert!(
        snapshot.lanes.iter().all(|lane| {
            lane.cumulative_snowfall_kg_m2 > 0.0 && lane.cumulative_melt_kg_m2 > 0.0
        })
    );
    assert!(snapshot.routed_runoff_mass_kg_m2 > 0.0);
    assert!(snapshot.upstream_runon_mass_kg_m2 > 0.0);
    assert!(
        snapshot
            .surface_flow_by_route
            .keys()
            .any(|route| { route.source_ofe_id == "ofe-1" && route.destination_ofe_id == "ofe-2" })
    );
    let bgc_delta = snapshot.biogeochemistry_delta;
    let bgc_beginning = snapshot.beginning_biogeochemistry;
    let bgc_ending = snapshot.ending_biogeochemistry;
    for (beginning, delta, ending) in [
        (
            bgc_beginning.ammonium_n,
            bgc_delta.ammonium_n,
            bgc_ending.ammonium_n,
        ),
        (
            bgc_beginning.nitrate_n,
            bgc_delta.nitrate_n,
            bgc_ending.nitrate_n,
        ),
        (
            bgc_beginning.receiver_carbon,
            bgc_delta.receiver_carbon,
            bgc_ending.receiver_carbon,
        ),
        (
            bgc_beginning.receiver_nitrogen,
            bgc_delta.receiver_nitrogen,
            bgc_ending.receiver_nitrogen,
        ),
        (
            bgc_beginning.receiver_dry_matter,
            bgc_delta.receiver_dry_matter,
            bgc_ending.receiver_dry_matter,
        ),
    ] {
        assert!((beginning + delta - ending).abs() <= 1.0e-12);
    }
    assert!(
        bgc_delta.ammonium_n != 0.0
            || bgc_delta.nitrate_n != 0.0
            || bgc_delta.receiver_carbon != 0.0
            || bgc_delta.receiver_nitrogen != 0.0
            || bgc_delta.receiver_dry_matter != 0.0,
        "the real season must advance a positive BGC follower operand"
    );
    assert!(snapshot.ending_biogeochemistry_last_transaction_id > 0);
    let beginning_owner = openwepp_persisted_restart_v1::restart_authority_two_ofe_owner_fixture();
    let beginning_soil = beginning_owner
        .runtime
        .shadow
        .restart_authority_soil_thermal()
        .expect("qualification fixture retains the explicit V1 soil owner");
    assert!(
        snapshot
            .soil_thermal_ofes
            .iter()
            .zip(&beginning_soil.ofes)
            .any(|(ending_ofe, beginning_ofe)| {
                ending_ofe
                    .ordered_layers
                    .iter()
                    .zip(&beginning_ofe.ordered_layers)
                    .any(|(ending, beginning)| {
                        ending.temperature_k.to_bits() != beginning.temperature_k.to_bits()
                            || ending.enthalpy_j_m2.to_bits()
                                != beginning.enthalpy_j_m2_ofe_ground.to_bits()
                    })
            })
    );
    assert!(
        report
            .optional_outputs
            .iter()
            .any(|path| path.ends_with("H83.wat.parquet"))
    );

    let wat = run_dir.join("output/H83.wat.parquet");
    let snow = read_wat_f64_column(&wat, "Snow-Water");
    let frost = read_wat_f64_column(&wat, "frozwt");
    let routed = read_wat_f64_column(&wat, "QOFE");
    assert_eq!(snow.len(), 2 * COMPLETE_SEASON_DAY_COUNT);
    let snow_days = snow
        .chunks_exact(2)
        .map(|lanes| lanes.iter().copied().fold(0.0_f64, f64::max))
        .collect::<Vec<_>>();
    let first_snow = snow_days
        .iter()
        .position(|value| *value > 0.0)
        .expect("initial accumulation");
    let first_meltout = snow_days
        .iter()
        .enumerate()
        .skip(first_snow + 1)
        .find_map(|(day, value)| (*value == 0.0).then_some(day))
        .expect("first meltout");
    assert!(
        snow_days
            .iter()
            .skip(first_meltout + 1)
            .any(|value| *value > 0.0),
        "later solid precipitation must reappear through the same owner"
    );
    assert!(
        frost.iter().any(|value| *value > 0.0),
        "frost/soil season evidence"
    );
    assert!(
        routed.iter().any(|value| *value > 0.0),
        "two-OFE routed water evidence"
    );
    eprintln!(
        "STAGE3_COMPLETE_SEASON_QUALIFICATION profile=reappearance-routing-bgc days={} lanes=2 supports={} elapsed={elapsed:?}",
        COMPLETE_SEASON_DAY_COUNT,
        COMPLETE_SEASON_DAY_COUNT * 48,
    );
    let _ = std::fs::remove_dir_all(run_dir);
}
