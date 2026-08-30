#[test]
fn wat5_day_two_source_failure_publishes_no_partial_output_set() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/erosion_single_ofe_p61");
    let run_dir = copy_fixture_to_temp(&source, "wat5_day_two_transaction");
    let run_path = run_dir.join("p61.run");
    let mut runfile = fs::read_to_string(&run_path).expect("read p61 run file");
    runfile.push_str(
        "wat = \"output/H61.wat.parquet\"\nwat_subhourly = \"output/H61.wat-subhourly.parquet\"\n",
    );
    fs::write(&run_path, runfile).expect("write WAT5 run file");

    let climate_path = run_dir.join("p61.cli");
    let climate = fs::read_to_string(&climate_path).expect("read p61 climate");
    let header = climate.lines().take(15).collect::<Vec<_>>().join("\n");
    let two_day_climate = format!(
        "{header}\n  1  1 2000  80.0   2.0  0.25  24.0  -2.0 -10.0  100  3.0  180 -12.0\n  2  1 2000   0.0   0.0  0.0    0.0  20.0  10.0  200  3.0  180   5.0\n"
    );
    fs::write(&climate_path, two_day_climate).expect("write two-day climate");

    let output_dir = run_dir.join("output");
    let request = HillslopeRunRequest {
        run_dir: run_dir.clone(),
        run_file: PathBuf::from("p61.run"),
        output_dir: output_dir.clone(),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: Some(output_dir.join("manifest.json")),
    };
    test_fixture_authority::author_stage3_v11_owner_seed_fixture(
        &request,
        test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
        test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
    )
    .expect("day-two WAT5 fixture should bind its exact Stage-3 owner seed");
    let error = execute_hillslope_run_with_runtime_policy(
        &request,
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    )
    .expect_err("day-two untimed melt must fail WAT5");
    assert!(error.to_string().contains("lane 1 day 1"));
    assert!(error.to_string().contains("WAT5-E-001"));
    for path in [
        "H61.hbp",
        "H61.loss.json",
        "H61.pass.parquet",
        "H61.wat.parquet",
        "H61.wat-subhourly.parquet",
        "manifest.json",
    ] {
        assert!(
            !output_dir.join(path).exists(),
            "failed run published partial output {path}"
        );
    }
    if output_dir.exists() {
        let leftovers = fs::read_dir(output_dir)
            .expect("read output directory")
            .map(|entry| entry.expect("output entry").path())
            .collect::<Vec<_>>();
        assert!(leftovers.is_empty(), "transaction leftovers: {leftovers:?}");
    }
}

#[test]
fn forced_wat5_close_failure_preserves_preexisting_sibling_output_set() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/erosion_single_ofe_p61");
    let run_dir = copy_fixture_to_temp(&source, "wat5_close_transaction");
    let run_path = run_dir.join("p61.run");
    let mut runfile = fs::read_to_string(&run_path).expect("read p61 run file");
    runfile.push_str(
        "wat = \"output/H61.wat.parquet\"\nwat_subhourly = \"output/H61.wat-subhourly.parquet\"\n",
    );
    fs::write(&run_path, runfile).expect("write WAT5 run file");

    let climate_path = run_dir.join("p61.cli");
    let climate = fs::read_to_string(&climate_path).expect("read p61 climate");
    let header = climate.lines().take(15).collect::<Vec<_>>().join("\n");
    fs::write(
        &climate_path,
        format!("{header}\n  1  1 2000   0.0   0.0  0.0    0.0  20.0  10.0  200  3.0  180   5.0\n"),
    )
    .expect("write source-complete dry climate");

    let output_dir = run_dir.join("output");
    fs::create_dir_all(&output_dir).expect("create output directory");
    let sentinel_paths = [
        output_dir.join("H61.hbp"),
        output_dir.join("H61.loss.json"),
        output_dir.join("H61.pass.parquet"),
        output_dir.join("H61.wat.parquet"),
        output_dir.join("manifest.json"),
    ];
    for (index, path) in sentinel_paths.iter().enumerate() {
        fs::write(path, format!("sentinel-{index}")).expect("write sentinel output");
    }

    let request = HillslopeRunRequest {
        run_dir: run_dir.clone(),
        run_file: PathBuf::from("p61.run"),
        output_dir: output_dir.clone(),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: false,
        manifest_path: Some(output_dir.join("manifest.json")),
    };
    test_fixture_authority::author_stage3_v11_owner_seed_fixture(
        &request,
        test_fixture_authority::Stage3TestFixtureSeedProfile::CompleteOwner,
        test_fixture_authority::Stage3TestFixtureSeedBinding::ExplicitRunfile,
    )
    .expect("forced-close WAT5 fixture should bind its exact Stage-3 owner seed");
    force_wat5_close_failure_once();
    let error = execute_hillslope_run_with_runtime_policy(
        &request,
        &["openwepp-cli-hill".to_string()],
        HillslopeRuntimeSelectionPolicy::new(
            HillslopeRuntimeSelection::DirectProductionExecutor,
            HillslopeDefaultRuntimeActivation::default(),
        ),
    )
    .expect_err("forced WAT5 close failure must fail the run");
    assert!(
        error.to_string().contains("forced WAT5 close failure"),
        "unexpected failure: {error}"
    );
    for (index, path) in sentinel_paths.iter().enumerate() {
        assert_eq!(
            fs::read(path).expect("read preserved sentinel"),
            format!("sentinel-{index}").as_bytes()
        );
    }
    assert!(!output_dir.join("H61.wat-subhourly.parquet").exists());
    let transaction_leftovers = fs::read_dir(&output_dir)
        .expect("read output directory")
        .map(|entry| entry.expect("output entry").file_name())
        .filter(|name| name.to_string_lossy().contains(".openwepp-"))
        .collect::<Vec<_>>();
    assert!(
        transaction_leftovers.is_empty(),
        "transaction leftovers: {transaction_leftovers:?}"
    );
}

#[test]
fn production_runner_publishes_positive_depression_storage_for_independent_reconstruction() {
    let identity = DirectRunIdentity::new(42, 2637, 1, 1).expect("WAT5 test identity");
    let producer = openwepp_hillslope_orchestrator::DirectWb14InfiltrationProducerInputs {
        hyetograph: vec![
            openwepp_hillslope_orchestrator::DirectWb14HyetographInterval {
                start_s: 0.0,
                end_s: 7_200.0,
                intensity_m_s: 0.08 / 7_200.0,
            },
        ],
        hourly_additional_supply_m: [0.0; 24],
        effective_conductivity_m_s: 1.0e-8,
        matric_potential_m: 0.1,
        storage_capacity_m: 1.0,
        depression_storage_capacity_m: 0.002,
    };
    let mut day = openwepp_hillslope_orchestrator::DirectDayFrame::seed(identity, 0, 0)
        .expect("seed WAT5 day");
    day.infiltration_depression_inputs =
        openwepp_hillslope_orchestrator::DirectInfiltrationDepressionInputs {
            cumulative_infiltration_handoff_m: 0.0,
            depression_storage_delta_handoff_m: 0.0,
            producer_inputs: Some(producer),
        };
    day.run_r4k_infiltration_depression_span()
        .expect("run production WB14 span");
    day.runoff_downstream_operands.q_runoff_m = day.wb14_hourly_excess_m.iter().sum();
    day.wat5_subhourly_requested = true;
    day.run_wat5_subhourly_generation()
        .expect("run production WAT5 projection");

    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("test clock")
        .as_nanos();
    let output_dir = std::env::temp_dir().join(format!("wat5-real-storage-{nonce}"));
    fs::create_dir(&output_dir).expect("create WAT5 test output directory");
    let wat5_path = output_dir.join("H2637.wat-subhourly.parquet");
    let mut sink = DirectPublicationStreamingSink::create(
        identity,
        DirectPublicationRunMetadata {
            run_name: "wat5-positive-depression-storage".to_string(),
            runtime_selection: "direct-production-executor".to_string(),
            output_policy: "test".to_string(),
        },
        &DirectPublicationStreamingTargets {
            wat: None,
            wat_subhourly: Some(wat5_path.clone()),
            pass_parquet: None,
        },
    )
    .expect("create real WAT5 streaming sink");
    let row = r6j_multiofe_publication_row(1, 1);
    sink.observe_row(&row).expect("observe publication row");
    sink.observe_subhourly_generation(&row, &day)
        .expect("map real WAT5 event into public rows");
    sink.finish().expect("finish real WAT5 streaming sink");

    let batches = ParquetRecordBatchReaderBuilder::try_new(
        File::open(&wat5_path).expect("open production WAT5 Parquet"),
    )
    .expect("read production WAT5 metadata")
    .build()
    .expect("build production WAT5 reader")
    .map(|batch| batch.expect("valid production WAT5 batch"))
    .collect::<Vec<_>>();
    let sum_column = |column: usize| {
        batches
            .iter()
            .map(|batch| {
                batch
                    .column(column)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .expect("WAT5 depth column")
                    .values()
                    .iter()
                    .sum::<f64>()
            })
            .sum::<f64>()
    };
    let rainfall_mm = sum_column(10);
    let infiltration_mm = sum_column(12);
    let depression_storage_mm = sum_column(13);
    let post_depression_generation_mm = sum_column(14);
    assert!(depression_storage_mm > 0.0);
    assert!(
        (rainfall_mm - infiltration_mm - depression_storage_mm - post_depression_generation_mm)
            .abs()
            <= 1.0e-9
    );

    fs::remove_dir_all(output_dir).expect("remove WAT5 test output directory");
}
use arrow_array::{Array, Float64Array};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;
