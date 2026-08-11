#[test]
fn wat5_day_two_source_failure_publishes_no_partial_output_set() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/erosion_single_ofe_p61");
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
    let error = execute_hillslope_run_with_runtime_policy(
        &HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("p61.run"),
            output_dir: output_dir.clone(),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: Some(output_dir.join("manifest.json")),
        },
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
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/erosion_single_ofe_p61");
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
        format!(
            "{header}\n  1  1 2000   0.0   0.0  0.0    0.0  20.0  10.0  200  3.0  180   5.0\n"
        ),
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

    force_wat5_close_failure_once();
    let error = execute_hillslope_run_with_runtime_policy(
        &HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("p61.run"),
            output_dir: output_dir.clone(),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: Some(output_dir.join("manifest.json")),
        },
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
        .filter(|name| name.to_string_lossy().contains(".openwepp-transaction-"))
        .collect::<Vec<_>>();
    assert!(
        transaction_leftovers.is_empty(),
        "transaction leftovers: {transaction_leftovers:?}"
    );
}
