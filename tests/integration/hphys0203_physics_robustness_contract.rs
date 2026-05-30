use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use openwepp_hillslope_orchestrator::{
    HillslopeWritebackSurface, runtime_inputs::build_hillslope_runtime_surface_from_soil,
};
use openwepp_input_contract::parsers::soil::{
    SoilParserOptions, SoilProfile, TopologyScope, parse_soil,
};
use openwepp_kernel_contract::BoundarySymbol;
use openwepp_runner::{HillslopeRunRequest, SidecarPolicy, execute_hillslope_run};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};

const EPS: f64 = 1.0e-6;
const VALID_9002: &str = include_str!("../fixtures/infile/soil/valid_9002.sol");

fn repo_file(path: &str) -> String {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let full_path = Path::new(repo_root).join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

#[test]
fn hphys0203_package_and_contract_authority_sections_exist() {
    let package = repo_file(
        "docs/work-packages/20260529-hphys0203-physics-robustness-test-suite-001/package.md",
    );
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    let soil = repo_file("docs/specifications/science-contracts/contracts/SC-SOIL-001.md");
    let subhyd = repo_file("docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md");
    let system = repo_file("docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md");

    assert!(
        package.contains("MEASURE-HP203-001")
            && package.contains("MEASURE-HP203-004")
            && package.contains("Mandatory Contract-First Sequence"),
        "HPHYS0203 package must preserve closure measures and contract-first sequencing"
    );
    assert!(
        watbal.contains("### HPHYS0203 Physics-Robustness Validation Addendum"),
        "SC-WATBAL-001 must include HPHYS0203 WB13 robustness obligations"
    );
    assert!(
        soil.contains("## HPHYS0203 Soil-Water Robustness Validation Addendum"),
        "SC-SOIL-001 must include HPHYS0203 soil-water robustness obligations"
    );
    assert!(
        subhyd.contains("## HPHYS0203 Subsurface WB13 Robustness Validation Addendum"),
        "SC-SUBHYD-001 must include HPHYS0203 subsurface WB13 robustness obligations"
    );
    assert!(
        system.contains("## HPHYS0203 WB13 Robustness Governance Addendum"),
        "SC-SYSTEM-001 must include HPHYS0203 system-level robustness governance obligations"
    );
}

#[test]
fn hphys0203_fixture_wat_rows_preserve_targeted_publication_invariants() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .expect("runner execution lock should be acquirable");

    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, "hphys0203_wat_invariants");

    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from("case.run"),
            output_dir: temp_run_dir.join("output"),
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
    assert!(
        !rows.is_empty(),
        "fixture H5.wat.parquet should contain at least one row"
    );

    for (row_index, row) in rows.iter().enumerate() {
        let dp = row_f64_value(row, "Dp");
        let latqcc = row_f64_value(row, "latqcc");
        let total_soil = row_f64_value(row, "Total-Soil");
        let frozwt = row_f64_value(row, "frozwt");
        let soil_water_total = row_f64_value(row, "SoilWaterTotal");
        let profile_depth = row_f64_value(row, "ProfileDepth");
        let profile_porosity_cap = row_f64_value(row, "ProfilePorosityCap");
        let profile_fc_store = row_f64_value(row, "ProfileFCStore");
        let profile_wp_store = row_f64_value(row, "ProfileWPStore");

        assert!(
            dp.is_finite() && dp >= 0.0,
            "row {} must satisfy finite/non-negative Dp, observed {}",
            row_index + 1,
            dp
        );
        assert!(
            latqcc.is_finite() && latqcc >= 0.0,
            "row {} must satisfy finite/non-negative latqcc, observed {}",
            row_index + 1,
            latqcc
        );
        assert!(
            total_soil.is_finite() && total_soil >= 0.0,
            "row {} must satisfy finite/non-negative Total-Soil, observed {}",
            row_index + 1,
            total_soil
        );
        assert!(
            frozwt.is_finite() && frozwt >= 0.0,
            "row {} must satisfy finite/non-negative frozwt, observed {}",
            row_index + 1,
            frozwt
        );
        assert!(
            (soil_water_total - (total_soil + frozwt)).abs() <= EPS,
            "row {} must satisfy SoilWaterTotal closure, observed SoilWaterTotal={}, Total-Soil={}, frozwt={}",
            row_index + 1,
            soil_water_total,
            total_soil,
            frozwt
        );
        assert!(
            profile_depth.is_finite() && profile_depth > 0.0,
            "row {} must satisfy ProfileDepth > 0, observed {}",
            row_index + 1,
            profile_depth
        );
        assert!(
            profile_porosity_cap.is_finite()
                && profile_fc_store.is_finite()
                && profile_wp_store.is_finite()
                && profile_porosity_cap >= profile_fc_store
                && profile_fc_store >= profile_wp_store
                && profile_wp_store >= 0.0,
            "row {} must satisfy ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore >= 0; observed cap={}, fc={}, wp={}",
            row_index + 1,
            profile_porosity_cap,
            profile_fc_store,
            profile_wp_store
        );
    }
}

#[test]
fn hphys0203_profile_regression_fixture_perturbation_preserves_ordering_stability() {
    let baseline_soil = parse_soil(VALID_9002, soil_parser_options_for_fixture())
        .expect("9002 fixture should parse for baseline profile projection");
    let baseline_surface = build_hillslope_runtime_surface_from_soil(&baseline_soil)
        .expect("baseline runtime surface should project");

    let baseline_profile_fc =
        required_surface_scalar(&baseline_surface, "wb13_profile_fc_store_mm");
    let baseline_profile_wp =
        required_surface_scalar(&baseline_surface, "wb13_profile_wp_store_mm");
    let baseline_profile_cap =
        required_surface_scalar(&baseline_surface, "wb13_profile_porosity_cap_mm");

    let mut perturbed_soil: SoilProfile = baseline_soil.clone();
    let baseline_fc_measured = perturbed_soil.ofes[0].layers[0]
        .fc_measured
        .expect("9002 first layer must provide fc_measured for perturbation vector");
    perturbed_soil.ofes[0].layers[0].fc_measured = Some(baseline_fc_measured + 1.0e-4);

    let perturbed_surface = build_hillslope_runtime_surface_from_soil(&perturbed_soil)
        .expect("perturbed runtime surface should project");
    let perturbed_profile_fc =
        required_surface_scalar(&perturbed_surface, "wb13_profile_fc_store_mm");
    let perturbed_profile_wp =
        required_surface_scalar(&perturbed_surface, "wb13_profile_wp_store_mm");
    let perturbed_profile_cap =
        required_surface_scalar(&perturbed_surface, "wb13_profile_porosity_cap_mm");

    assert!(
        perturbed_profile_cap >= perturbed_profile_fc
            && perturbed_profile_fc >= perturbed_profile_wp,
        "profile ordering must remain valid under bounded FC perturbation"
    );
    assert!(
        perturbed_profile_fc >= baseline_profile_fc,
        "positive fc_measured perturbation should not decrease projected profile FC storage"
    );
    assert!(
        (perturbed_profile_fc - baseline_profile_fc) <= 5.0,
        "bounded fc_measured perturbation produced unstable profile FC delta: baseline={baseline_profile_fc}, perturbed={perturbed_profile_fc}"
    );
    assert!(
        (perturbed_profile_cap - baseline_profile_cap).abs() <= 1.0e-9,
        "porosity-cap projection should remain stable for FC-only perturbation"
    );
    assert!(
        perturbed_profile_wp >= 0.0 && baseline_profile_wp >= 0.0,
        "profile WP storage must remain non-negative under perturbation"
    );
}

fn soil_parser_options_for_fixture() -> SoilParserOptions {
    SoilParserOptions {
        mode: SidecarPolicy::Compat.as_soil_parser_mode(),
        allow_legacy_aliases: true,
        expected_topology_count: Some(1),
        topology_scope: Some(TopologyScope::Hillslope),
    }
}

fn required_surface_scalar(surface: &HillslopeWritebackSurface, symbol: &str) -> f64 {
    surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .map_or_else(
            || panic!("missing required runtime symbol {symbol}"),
            |value| value.as_f64(),
        )
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
