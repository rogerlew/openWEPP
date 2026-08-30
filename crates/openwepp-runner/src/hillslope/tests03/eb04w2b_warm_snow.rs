#[test]
fn eb04w2b_retired_day_builder_partition_entrypoint_fails_closed() {
    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, "eb04w2b_warm_snow");
    let request = HillslopeRunRequest {
        run_dir: temp_run_dir.clone(),
        run_file: PathBuf::from("case.run"),
        output_dir: temp_run_dir.join("output"),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: None,
    };
    let mut inputs = load_hillslope_run_inputs(&request).expect("authoritative fixture inputs");
    let ClimateDailyRecord::NoBreakpoint(day) = &mut inputs.climate.daily_records[0] else {
        panic!("EB-04W2B fixture day must be no-breakpoint forcing");
    };
    day.prcp = 12.0;
    day.stmdur = 24.0;
    day.timep = 0.0;
    day.tmax = 22.0;
    day.tmin = -20.0;
    day.tdpt = -20.0;

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
        build_hillslope_climate_runtime_request(&inputs.climate).expect("EB-04W2B climate request");
    let mut seed_authority = DirectProductionSeedAuthority::from_typed_inputs(
        &climate_request,
        &inputs,
        &sidecars,
        per_ofe_lane_areas_m2.len(),
        lane_context.lane,
    )
    .expect("production seed authority");
    for lane in &mut seed_authority.lanes {
        lane.constructor.snow_lane_state =
            openwepp_hillslope_orchestrator::DirectSnowLaneState::zero();
        lane.day_input.snow_frost.snow_file_present = false;
        lane.day_input.snow_frost.snow_runtime_swe_m = 0.0;
        lane.day_input.snow_frost.snow_runtime_depth_m = 0.0;
        lane.day_input.snow_frost.snow_runtime_density_kg_m3 = 0.0;
        lane.day_input.snow_frost.snow_runtime_settle_day_count = 0.0;
        lane.day_input.snow_frost.snow_phase_model =
            openwepp_hillslope_orchestrator::SnowPhasePartitionModel::HarderPomeroyHourly;
    }
    let frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
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
    let error = builder
        .build(&frame, 0, 0)
        .expect_err("retired day-builder partition entrypoint must fail closed");
    let message = error.to_string();
    assert!(
        message.contains("snow.adaptive_stage3_legacy_sublimation_entry"),
        "unexpected Stage3 V11 cutover error: {message}"
    );
    let _ = std::fs::remove_dir_all(temp_run_dir);
}
