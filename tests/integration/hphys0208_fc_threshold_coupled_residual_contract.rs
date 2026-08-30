use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use openwepp_runner::{HillslopeRunRequest, SidecarPolicy};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};

mod common;

const EPS: f64 = 1.0e-6;

fn repo_file(path: &str) -> String {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let full_path = Path::new(repo_root).join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

#[test]
fn hphys0208_package_and_contract_authority_sections_exist() {
    let package = repo_file(
        "docs/work-packages/20260530-hphys0208-fc-threshold-coupled-residual-closure-001/package.md",
    );
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    let soil = repo_file("docs/specifications/science-contracts/contracts/SC-SOIL-001.md");
    let perc = repo_file("docs/specifications/science-contracts/contracts/SC-PERC-001.md");
    let subhyd = repo_file("docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md");
    let system = repo_file("docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md");

    assert!(
        package.contains("MEASURE-HP208-001")
            && package.contains("MEASURE-HP208-004")
            && package.contains("Mandatory Contract-First Sequence"),
        "HPHYS0208 package must preserve closure measures and contract-first sequencing"
    );
    assert!(
        watbal.contains("### HPHYS0208 FC-Threshold Coupled Residual Closure Addendum"),
        "SC-WATBAL-001 must include HPHYS0208 FC-threshold coupled residual authority"
    );
    assert!(
        soil.contains("## HPHYS0208 WB11 Seed Threshold-Lineage Projection Addendum"),
        "SC-SOIL-001 must include HPHYS0208 threshold-lineage projection authority"
    );
    assert!(
        perc.contains("## HPHYS0208 FC-Threshold Consumer-Lineage Closure Addendum"),
        "SC-PERC-001 must include HPHYS0208 FC-threshold consumer-lineage closure authority"
    );
    assert!(
        subhyd.contains("## HPHYS0208 Coupled Subsurface Residual Closure Addendum"),
        "SC-SUBHYD-001 must include HPHYS0208 coupled subsurface closure authority"
    );
    assert!(
        system.contains("## HPHYS0208 Coupled WB13 Publication Lineage Addendum"),
        "SC-SYSTEM-001 must include HPHYS0208 coupled WB13 publication lineage authority"
    );
}

#[test]
fn hphys0208_sat_perturbation_changes_coupled_wb13_publications() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .expect("runner execution lock should be acquirable");

    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let baseline_dir = copy_fixture_to_temp(&source_fixture_dir, "hphys0208_sat_baseline");
    let perturbed_dir = copy_fixture_to_temp(&source_fixture_dir, "hphys0208_sat_perturbed");

    let baseline_row = execute_fixture_and_load_first_row(&baseline_dir);

    rewrite_primary_ofe_sat(&perturbed_dir.join("case.sol"), 0.80);
    let perturbed_row = execute_fixture_and_load_first_row(&perturbed_dir);

    let baseline_profile_fc = row_f64_value(&baseline_row, "ProfileFCStore");
    let baseline_dp = row_f64_value(&baseline_row, "Dp");
    let baseline_latqcc = row_f64_value(&baseline_row, "latqcc");
    let baseline_total_soil = row_f64_value(&baseline_row, "Total-Soil");
    let baseline_frozwt = row_f64_value(&baseline_row, "frozwt");
    let baseline_soil_water_total = row_f64_value(&baseline_row, "SoilWaterTotal");

    let perturbed_profile_fc = row_f64_value(&perturbed_row, "ProfileFCStore");
    let perturbed_dp = row_f64_value(&perturbed_row, "Dp");
    let perturbed_latqcc = row_f64_value(&perturbed_row, "latqcc");
    let perturbed_total_soil = row_f64_value(&perturbed_row, "Total-Soil");
    let perturbed_frozwt = row_f64_value(&perturbed_row, "frozwt");
    let perturbed_soil_water_total = row_f64_value(&perturbed_row, "SoilWaterTotal");

    for (name, value) in [
        ("ProfileFCStore", baseline_profile_fc),
        ("Dp", baseline_dp),
        ("latqcc", baseline_latqcc),
        ("Total-Soil", baseline_total_soil),
        ("frozwt", baseline_frozwt),
        ("SoilWaterTotal", baseline_soil_water_total),
        ("ProfileFCStore", perturbed_profile_fc),
        ("Dp", perturbed_dp),
        ("latqcc", perturbed_latqcc),
        ("Total-Soil", perturbed_total_soil),
        ("frozwt", perturbed_frozwt),
        ("SoilWaterTotal", perturbed_soil_water_total),
    ] {
        assert!(
            value.is_finite() && value >= 0.0,
            "{name} must remain finite/non-negative under sat perturbation, observed {value}"
        );
    }

    assert!(
        (baseline_soil_water_total - baseline_total_soil).abs() <= EPS,
        "baseline row must satisfy hydout-equivalent SoilWaterTotal alias"
    );
    assert!(
        (perturbed_soil_water_total - perturbed_total_soil).abs() <= EPS,
        "perturbed row must satisfy hydout-equivalent SoilWaterTotal alias"
    );

    let coupled_delta = (perturbed_total_soil - baseline_total_soil).abs();
    assert!(
        coupled_delta > 1.0e-9,
        "sat perturbation must propagate through coupled WB13 seed-lineage publications (Total-Soil unchanged: baseline={baseline_total_soil}, perturbed={perturbed_total_soil})"
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

fn rewrite_primary_ofe_sat(soil_path: &Path, sat: f64) {
    let text = fs::read_to_string(soil_path)
        .unwrap_or_else(|error| panic!("soil fixture should be readable: {error}"));
    let mut lines = text.lines().map(ToString::to_string).collect::<Vec<_>>();
    assert!(
        lines.len() >= 4,
        "unexpected soil fixture shape; expected OFE header on line 4"
    );
    let header = lines
        .get_mut(3)
        .expect("soil fixture should include OFE header line");
    let mut tokens = header
        .split_whitespace()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    assert!(
        tokens.len() >= 5,
        "unexpected OFE header shape; expected sat token at position 5"
    );
    tokens[4] = format!("{sat:.2}");
    *header = tokens.join(" ");
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
