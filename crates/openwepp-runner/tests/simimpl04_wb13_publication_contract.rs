use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use arrow_array::{Float64Array, Int16Array, Int32Array};
use openwepp_hillslope_output::hillslope_wat::{InterchangeVersion, hillslope_wat_schema};
use openwepp_runner::{
    HillslopeDefaultRuntimeActivation, HillslopeRunReport, HillslopeRunRequest,
    HillslopeRuntimeSelection, HillslopeRuntimeSelectionPolicy, SidecarPolicy,
    execute_hillslope_run_with_runtime_policy,
};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

mod common;

#[test]
#[ignore = "full two-day Stage-3 production qualification; run optimized with a 1800s bound"]
fn simimpl04_contract_requires_simulation_owned_wb13_publication_provenance() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "simimpl04-simout"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = true
pmetpara = "pmetpara.txt"

[outputs]
pass = "output/H5.hbp"
loss = "output/H5.loss.json"
wat = "output/H5.wat.parquet"
plot = "output/H5.plot.parquet"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report(runfile, "simimpl04_simout");

    let manifest_json = read_manifest_json(&report);

    assert_json_string(
        &manifest_json,
        "/wb13_publication/source",
        "direct-publication-frame",
    );
    assert_json_bool(
        &manifest_json,
        "/wb13_publication/projection_fallback_used",
        false,
    );
    assert_json_string(
        &manifest_json,
        "/wb13_publication/guard_id",
        "HS-SIMOUT-E-001",
    );
    assert_json_array_len(
        &manifest_json,
        "/wb13_publication/replay_candidate_surfaces",
        0,
    );

    assert_production_wat_readback(&report);
}

fn assert_production_wat_readback(report: &HillslopeRunReport) {
    let wat_path = report
        .optional_outputs
        .iter()
        .find(|path| {
            path.file_name()
                .is_some_and(|name| name.to_string_lossy().ends_with(".wat.parquet"))
        })
        .expect("production report should retain the streaming WAT parquet path");
    let mut reader = ParquetRecordBatchReaderBuilder::try_new(
        File::open(wat_path).expect("production WAT parquet should open"),
    )
    .expect("production WAT parquet metadata should parse")
    .build()
    .expect("production WAT parquet reader should build");
    let batch = reader
        .next()
        .expect("production WAT parquet should contain a batch")
        .expect("production WAT parquet batch should read");
    assert!(reader.next().is_none(), "fixture should emit one row group");
    assert_eq!(batch.num_rows(), 2);
    let expected_schema = hillslope_wat_schema(InterchangeVersion::default())
        .expect("canonical WAT schema should build");
    let observed_fields = batch
        .schema()
        .fields()
        .iter()
        .map(|field| {
            (
                field.name().clone(),
                field.data_type().clone(),
                field.is_nullable(),
            )
        })
        .collect::<Vec<_>>();
    let expected_fields = expected_schema
        .fields()
        .iter()
        .map(|field| {
            (
                field.name().clone(),
                field.data_type().clone(),
                field.is_nullable(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(observed_fields, expected_fields);

    let int32 = |name: &str| {
        batch
            .column_by_name(name)
            .unwrap_or_else(|| panic!("missing {name}"))
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap_or_else(|| panic!("{name} should be Int32"))
    };
    let int16 = |name: &str| {
        batch
            .column_by_name(name)
            .unwrap_or_else(|| panic!("missing {name}"))
            .as_any()
            .downcast_ref::<Int16Array>()
            .unwrap_or_else(|| panic!("{name} should be Int16"))
    };
    let float64 = |name: &str| {
        batch
            .column_by_name(name)
            .unwrap_or_else(|| panic!("missing {name}"))
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap_or_else(|| panic!("{name} should be Float64"))
    };
    assert_eq!(
        (
            int32("sim_day_index").value(0),
            int32("sim_day_index").value(1)
        ),
        (1, 2)
    );
    assert_eq!((int16("julian").value(0), int16("julian").value(1)), (1, 2));
    assert_eq!((int16("year").value(0), int16("year").value(1)), (1, 1));
    for row in 0..batch.num_rows() {
        assert_eq!(
            float64("Q").value(row).to_bits(),
            float64("QOFE").value(row).to_bits()
        );
        assert_eq!(
            float64("Total-Soil").value(row).to_bits(),
            float64("SoilWaterTotal").value(row).to_bits()
        );
    }
}

#[test]
#[ignore = "full two-day Stage-3 production qualification; run optimized with a 1800s bound"]
fn simimpl14_contract_requires_continuous_wb13_span_and_simulation_year_row_keys() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "simimpl14-continuous-span"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = true
pmetpara = "pmetpara.txt"

[outputs]
pass = "output/H5.hbp"
loss = "output/H5.loss.json"
wat = "output/H5.wat.parquet"
plot = "output/H5.plot.parquet"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report(runfile, "simimpl14_span");

    let manifest_json = read_manifest_json(&report);
    assert_json_i64(&manifest_json, "/execution_provenance/climate_day_count", 2);
    assert_json_i64(
        &manifest_json,
        "/execution_provenance/executed_day_count",
        2,
    );
    assert_json_i64(&manifest_json, "/wb13_publication/row_count", 2);
    assert_json_bool(
        &manifest_json,
        "/wb13_publication/sim_day_index_monotonic",
        true,
    );
    assert_json_i64(&manifest_json, "/wb13_publication/first_row_key/year", 2000);
    assert_json_i64(
        &manifest_json,
        "/wb13_publication/last_row_key/julian_day",
        2,
    );
}

#[test]
#[ignore = "full two-day Stage-3 production qualification; run optimized with a 1800s bound"]
fn simimpl14_contract_requires_run_span_truthful_loss_output_summary() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "simimpl14-loss-span"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = true
pmetpara = "pmetpara.txt"

[outputs]
pass = "output/H5.hbp"
loss = "output/H5.loss.json"
plot = "output/H5.plot.parquet"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report(runfile, "simimpl14_loss");
    let loss_text = fs::read_to_string(&report.output_loss).unwrap_or_else(|error| {
        panic!(
            "loss output should be readable at {}: {error}",
            report.output_loss.display()
        )
    });
    let loss_json: serde_json::Value =
        serde_json::from_str(&loss_text).expect("loss output should parse as JSON");

    assert_json_i64(&loss_json, "/climate_day_count", 2);
    assert_json_i64(&loss_json, "/executed_day_count", 2);
    assert_json_i64(&loss_json, "/first_day_julian", 1);
    assert_json_i64(&loss_json, "/last_day_julian", 2);
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

fn assert_json_string(document: &serde_json::Value, pointer: &str, expected: &str) {
    let observed = document
        .pointer(pointer)
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("missing string JSON pointer {pointer}"));
    assert_eq!(observed, expected, "unexpected value at {pointer}");
}

fn assert_json_bool(document: &serde_json::Value, pointer: &str, expected: bool) {
    let observed = document
        .pointer(pointer)
        .and_then(serde_json::Value::as_bool)
        .unwrap_or_else(|| panic!("missing bool JSON pointer {pointer}"));
    assert_eq!(observed, expected, "unexpected value at {pointer}");
}

fn assert_json_i64(document: &serde_json::Value, pointer: &str, expected: i64) {
    let observed = document
        .pointer(pointer)
        .and_then(serde_json::Value::as_i64)
        .unwrap_or_else(|| panic!("missing integer JSON pointer {pointer}"));
    assert_eq!(observed, expected, "unexpected value at {pointer}");
}

fn assert_json_array_len(document: &serde_json::Value, pointer: &str, expected: usize) {
    let observed = document
        .pointer(pointer)
        .and_then(serde_json::Value::as_array)
        .unwrap_or_else(|| panic!("missing array JSON pointer {pointer}"))
        .len();
    assert_eq!(observed, expected, "unexpected array length at {pointer}");
}

fn execute_fixture_with_runfile_report(
    runfile_payload: &str,
    prefix: &str,
) -> (HillslopeRunReport, PathBuf) {
    let _execution_guard = runner_execution_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);
    let run_file_path = temp_run_dir.join("case.run");
    let runfile_payload = common::stage3_owner_seed::install(&temp_run_dir, runfile_payload);
    fs::write(&run_file_path, runfile_payload).expect("runfile fixture should be writable");

    let output_dir = temp_run_dir.join("output");
    let report = common::stage3_owner_seed::with_large_stack(|| {
        execute_hillslope_run_with_runtime_policy(
            &HillslopeRunRequest {
                run_dir: temp_run_dir.clone(),
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
    })
    .expect("fixture run should succeed before WB13 provenance assertions");

    (report, temp_run_dir)
}

fn runner_execution_lock() -> &'static Mutex<()> {
    static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    RUN_LOCK.get_or_init(|| Mutex::new(()))
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
