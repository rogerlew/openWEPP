use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use openwepp_runner::{
    HillslopeCliError, HillslopeRunReport, HillslopeRunRequest, SidecarPolicy,
    execute_hillslope_run,
};

const RUNFILE_CONTRACT: &str =
    include_str!("../../docs/contracts/openwepp-hillslope-runfile-contract.md");
const RUNNER_CONTRACT: &str = include_str!("../../docs/contracts/openwepp-runner-contract.md");
const HILLSLOPE_CLI_SPEC: &str = include_str!(
    "../../docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md"
);
const CONTRACTS_README: &str = include_str!("../../docs/contracts/README.md");

type DatasetMetadata = HashMap<String, String>;
type FieldMetadata = HashMap<String, HashMap<String, String>>;

#[test]
fn cli04_contract_surface_declares_wat_metadata_parity_and_dependency_posture() {
    for expected in [
        "`outputs.wat` metadata parity requirements",
        "dataset_version_major",
        "dataset_version_minor",
        "schema_version",
        "openwepp-output",
        "arrow2",
        "arrow-array",
        "arrow-schema",
        "InterceptionStorage",
    ] {
        assert!(
            RUNFILE_CONTRACT.contains(expected)
                || RUNNER_CONTRACT.contains(expected)
                || HILLSLOPE_CLI_SPEC.contains(expected)
                || CONTRACTS_README.contains(expected),
            "CLI04 authority surfaces missing expected text: {expected}"
        );
    }
}

#[test]
fn cli04_fixture_run_emits_valid_wat_parquet_with_required_metadata_keys() {
    let runfile = r#"
schema = "openwepp-hillslope-runfile-v1"
run_name = "cli04-wat-parquet-metadata"
unit_system = "metric"

[inputs]
soil = "case.sol"
management = "case.man"
slope = "case.slp"
climate = "case.cli"
wepp_ui = true

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
wat = "output/H1.wat.parquet"
"#;

    let (report, _temp_run_dir) = execute_fixture_with_runfile_report(runfile, "cli04_wat_parquet")
        .expect("fixture run should succeed");

    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());

    let wat_output = report
        .optional_outputs
        .iter()
        .find(|path| path.file_name().and_then(|name| name.to_str()) == Some("H1.wat.parquet"))
        .expect("wat output should be present");

    assert!(wat_output.is_file(), "wat output should exist");

    let (dataset_metadata, field_metadata) = read_wat_schema_metadata(wat_output)
        .expect("wat output should be valid parquet with readable arrow schema metadata");

    for key in [
        "dataset_version",
        "dataset_version_major",
        "dataset_version_minor",
        "schema_version",
    ] {
        assert!(
            dataset_metadata.contains_key(key),
            "missing dataset metadata key: {key}"
        );
    }

    let p_metadata = field_metadata
        .get("P")
        .expect("P field metadata should exist");
    assert_eq!(p_metadata.get("units").map(String::as_str), Some("mm"));
    assert_eq!(
        p_metadata.get("description").map(String::as_str),
        Some("Precipitation")
    );

    let interception_metadata = field_metadata
        .get("InterceptionStorage")
        .expect("InterceptionStorage field metadata should exist");
    assert_eq!(
        interception_metadata.get("units").map(String::as_str),
        Some("mm")
    );
    assert!(
        interception_metadata
            .get("description")
            .is_some_and(|value| value.contains("optional producer-authoritative term")),
        "InterceptionStorage description should document producer-authoritative optional semantics"
    );
}

fn read_wat_schema_metadata(path: &Path) -> Result<(DatasetMetadata, FieldMetadata), String> {
    let file = File::open(path).map_err(|error| error.to_string())?;
    let builder =
        ParquetRecordBatchReaderBuilder::try_new(file).map_err(|error| error.to_string())?;
    let schema = builder.schema();

    let dataset_metadata = schema
        .metadata()
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<DatasetMetadata>();

    let field_metadata = schema
        .fields()
        .iter()
        .map(|field| {
            (
                field.name().clone(),
                field
                    .metadata()
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect::<HashMap<_, _>>(),
            )
        })
        .collect::<FieldMetadata>();

    Ok((dataset_metadata, field_metadata))
}

fn execute_fixture_with_runfile_report(
    runfile_payload: &str,
    prefix: &str,
) -> Result<(HillslopeRunReport, PathBuf), HillslopeCliError> {
    let _execution_guard = runner_execution_lock()
        .lock()
        .expect("runner execution lock should be acquirable");

    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, prefix);
    let run_file_path = temp_run_dir.join("case.run");
    fs::write(&run_file_path, runfile_payload).expect("runfile fixture should be writable");

    let output_dir = temp_run_dir.join("output");
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir,
            sidecar_policy: SidecarPolicy::Strict,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )?;

    Ok((report, temp_run_dir))
}

fn runner_execution_lock() -> &'static Mutex<()> {
    static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    RUN_LOCK.get_or_init(|| Mutex::new(()))
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("integration file parent exists")
        .parent()
        .expect("tests directory exists")
        .join("fixtures")
        .join("cli01")
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
