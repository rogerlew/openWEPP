use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use openwepp_runner::{HillslopeRunRequest, SidecarPolicy};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};

mod common;

const EPS: f64 = 1.0e-9;

fn repo_file(path: &str) -> String {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let full_path = Path::new(repo_root).join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

#[test]
fn hphys0209_package_and_contract_authority_sections_exist() {
    let package = repo_file(
        "docs/work-packages/20260530-hphys0209-profilewp-near-closed-adjudication-001/package.md",
    );
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    let soil = repo_file("docs/specifications/science-contracts/contracts/SC-SOIL-001.md");
    let system = repo_file("docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md");

    assert!(
        package.contains("MEASURE-HP209-001")
            && package.contains("MEASURE-HP209-004")
            && package.contains("Mandatory Contract-First Sequence"),
        "HPHYS0209 package must preserve closure measures and contract-first sequencing"
    );
    assert!(
        watbal.contains("### HPHYS0209 ProfileWP Near-Closed Adjudication Addendum"),
        "SC-WATBAL-001 must include HPHYS0209 ProfileWP adjudication authority"
    );
    assert!(
        soil.contains("## HPHYS0209 ProfileWP Near-Closed Adjudication Addendum"),
        "SC-SOIL-001 must include HPHYS0209 ProfileWP adjudication authority"
    );
    assert!(
        system.contains("## HPHYS0209 ProfileWP Near-Closed Publication Adjudication Addendum"),
        "SC-SYSTEM-001 must include HPHYS0209 ProfileWP publication adjudication authority"
    );
}

#[test]
fn hphys0209_wp_authority_perturbation_changes_profile_wp_without_profile_geometry_regression() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .expect("runner execution lock should be acquirable");

    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let baseline_dir = copy_fixture_to_temp(&source_fixture_dir, "hphys0209_wp_baseline");
    let perturbed_dir = copy_fixture_to_temp(&source_fixture_dir, "hphys0209_wp_perturbed");

    let baseline_row = execute_fixture_and_load_first_row(&baseline_dir);

    rewrite_primary_layer_wp(&perturbed_dir.join("case.sol"), 0.20);
    let perturbed_row = execute_fixture_and_load_first_row(&perturbed_dir);

    let baseline_wp_store = row_f64_value(&baseline_row, "ProfileWPStore");
    let baseline_depth = row_f64_value(&baseline_row, "ProfileDepth");
    let baseline_porosity_cap = row_f64_value(&baseline_row, "ProfilePorosityCap");

    let perturbed_wp_store = row_f64_value(&perturbed_row, "ProfileWPStore");
    let perturbed_fc_store = row_f64_value(&perturbed_row, "ProfileFCStore");
    let perturbed_depth = row_f64_value(&perturbed_row, "ProfileDepth");
    let perturbed_porosity_cap = row_f64_value(&perturbed_row, "ProfilePorosityCap");

    for (name, value) in [
        ("baseline ProfileWPStore", baseline_wp_store),
        ("baseline ProfileDepth", baseline_depth),
        ("baseline ProfilePorosityCap", baseline_porosity_cap),
        ("perturbed ProfileWPStore", perturbed_wp_store),
        ("perturbed ProfileFCStore", perturbed_fc_store),
        ("perturbed ProfileDepth", perturbed_depth),
        ("perturbed ProfilePorosityCap", perturbed_porosity_cap),
    ] {
        assert!(
            value.is_finite() && value >= 0.0,
            "{name} must remain finite/non-negative, observed {value}"
        );
    }

    assert!(
        perturbed_wp_store > baseline_wp_store + 1.0e-9,
        "ProfileWPStore must respond to authoritative wp-lineage perturbation (baseline={baseline_wp_store}, perturbed={perturbed_wp_store})"
    );
    assert!(
        (perturbed_depth - baseline_depth).abs() <= EPS,
        "ProfileDepth must remain geometry-stable under wp-lineage perturbation"
    );
    assert!(
        (perturbed_porosity_cap - baseline_porosity_cap).abs() <= EPS,
        "ProfilePorosityCap must remain geometry-stable under wp-lineage perturbation"
    );
    assert!(
        perturbed_porosity_cap >= perturbed_fc_store && perturbed_fc_store >= perturbed_wp_store,
        "profile storage ordering must remain valid after wp-lineage perturbation"
    );
}

fn execute_fixture_and_load_first_row(run_dir: &Path) -> Row {
    let report = common::execute_with_adaptive_stage3_owner_seed(
        &HillslopeRunRequest {
            run_dir: run_dir.to_path_buf(),
            run_file: PathBuf::from("case.run"),
            output_dir: run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )
    .expect("fixture run should execute");

    let wat_path = report
        .optional_outputs
        .iter()
        .find(|path| path.file_name().and_then(|value| value.to_str()) == Some("H5.wat.parquet"))
        .unwrap_or_else(|| panic!("missing H5.wat.parquet in {:?}", report.optional_outputs));
    let rows = load_wat_rows(wat_path);
    rows.into_iter()
        .next()
        .expect("fixture H5.wat.parquet should contain at least one row")
}

fn rewrite_primary_layer_wp(soil_path: &Path, wp: f64) {
    let text = fs::read_to_string(soil_path)
        .unwrap_or_else(|error| panic!("soil fixture should be readable: {error}"));
    let mut lines = text.lines().map(ToString::to_string).collect::<Vec<_>>();
    assert!(
        lines.len() >= 6,
        "unexpected soil fixture shape; expected first layer line at index 5"
    );
    let first_layer = lines
        .get_mut(5)
        .expect("soil fixture should include first layer line");
    let mut tokens = first_layer
        .split_whitespace()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    assert!(
        tokens.len() >= 6,
        "unexpected first layer shape; expected wp token at position 6"
    );
    tokens[5] = format!("{wp:.6}");
    *first_layer = tokens.join(" ");
    fs::write(soil_path, format!("{}\n", lines.join("\n")))
        .unwrap_or_else(|error| panic!("soil fixture rewrite should succeed: {error}"));
}

fn load_wat_rows(wat_path: &Path) -> Vec<Row> {
    let file = File::open(wat_path)
        .unwrap_or_else(|error| panic!("wat parquet output should be readable: {error}"));
    let reader = SerializedFileReader::new(file)
        .unwrap_or_else(|error| panic!("wat parquet output should parse: {error}"));
    let rows = reader
        .get_row_iter(None)
        .unwrap_or_else(|error| panic!("wat parquet row iterator should open: {error}"));
    rows.map(|value| value.unwrap_or_else(|error| panic!("wat parquet row should decode: {error}")))
        .collect()
}

fn row_index(row: &Row, column_name: &str) -> usize {
    row.get_column_iter()
        .enumerate()
        .find(|(_, (name, _))| name.as_str() == column_name)
        .map_or_else(
            || panic!("missing required wat parquet column '{column_name}'"),
            |(index, _)| index,
        )
}

fn row_f64_value(row: &Row, column_name: &str) -> f64 {
    let column_index = row_index(row, column_name);
    row.get_double(column_index).unwrap_or_else(|error| {
        panic!("column '{column_name}' should decode as f64 from row: {error}")
    })
}

fn runner_execution_lock() -> &'static Mutex<()> {
    static RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    RUN_LOCK.get_or_init(|| Mutex::new(()))
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/cli01")
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
