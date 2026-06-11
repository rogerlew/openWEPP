use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use openwepp_hillslope_orchestrator::{
    HillslopeWritebackSurface,
    runtime_inputs::{HillslopeRuntimeInputError, build_hillslope_runtime_surface_from_soil},
};
use openwepp_input_contract::parsers::soil::{
    SoilParserOptions, SoilProfile, TopologyScope, parse_soil,
};
use openwepp_kernel_contract::BoundarySymbol;
use openwepp_runner::{HillslopeRunRequest, SidecarPolicy, execute_hillslope_run};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};

const EPS: f64 = 1.0e-9;
const VALID_9002: &str = include_str!("../fixtures/infile/soil/valid_9002.sol");

#[derive(Debug, Clone, Copy)]
struct ProfileAggregation {
    fc_store_mm: f64,
    wp_store_mm: f64,
}

fn repo_file(path: &str) -> String {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let full_path = Path::new(repo_root).join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

#[test]
#[allow(clippy::too_many_lines)]
fn hphys0202_package_and_contract_authority_sections_exist() {
    let package = repo_file(
        "docs/work-packages/20260529-hphys0202-profile-fc-wp-lineage-closure-001/package.md",
    );
    let package_hphys0206 = repo_file(
        "docs/work-packages/20260530-hphys0206-fcwp-layer-normalization-mapping-closure-001/package.md",
    );
    let package_hphys0207 = repo_file(
        "docs/work-packages/20260530-hphys0207-fcwp-depth-authority-tail-closure-001/package.md",
    );
    let package_hphys0216 = repo_file(
        "docs/work-packages/20260531-hphys0216-profilefc-layer-authority-realignment-001/package.md",
    );
    let package_hphys0216d = repo_file(
        "docs/work-packages/20260531-hphys0216d-profilefc-normalized-tail-authority-reconciliation-001/package.md",
    );
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    let soil = repo_file("docs/specifications/science-contracts/contracts/SC-SOIL-001.md");
    let perc = repo_file("docs/specifications/science-contracts/contracts/SC-PERC-001.md");
    let system = repo_file("docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md");
    let system_provenance = repo_file(
        "docs/specifications/science-contracts/contracts/provenance/SC-SYSTEM-001-provenance.md",
    );

    assert!(
        package.contains("MEASURE-HP202-001")
            && package.contains("MEASURE-HP202-004")
            && package.contains("Mandatory Contract-First Sequence"),
        "HPHYS0202 package must preserve closure measures and contract-first sequencing"
    );
    assert!(
        package_hphys0206.contains("MEASURE-HP206-001")
            && package_hphys0206.contains("MEASURE-HP206-004")
            && package_hphys0206.contains("Mandatory Contract-First Sequence"),
        "HPHYS0206 package must preserve closure measures and contract-first sequencing"
    );
    assert!(
        package_hphys0207.contains("MEASURE-HP207-001")
            && package_hphys0207.contains("MEASURE-HP207-004")
            && package_hphys0207.contains("Mandatory Contract-First Sequence"),
        "HPHYS0207 package must preserve closure measures and contract-first sequencing"
    );
    assert!(
        package_hphys0216.contains("MEASURE-HP216-001")
            && package_hphys0216.contains("MEASURE-HP216-004")
            && package_hphys0216.contains("Mandatory Contract-First Sequence"),
        "HPHYS0216 package must preserve closure measures and contract-first sequencing"
    );
    assert!(
        package_hphys0216d.contains("MEASURE-HP216D-001")
            && package_hphys0216d.contains("MEASURE-HP216D-004")
            && package_hphys0216d.contains("Mandatory Contract-First Sequence"),
        "HPHYS0216D package must preserve closure measures and contract-first sequencing"
    );
    assert!(
        watbal.contains("HPHYS0202-PROFILEFC-PROFILEWP-LAYER-AGGREGATION-LINEAGE-CLOSURE-HISTORIC")
            || watbal
                .contains("### HPHYS0202 ProfileFC/ProfileWP Layer-Aggregation Lineage Closure"),
        "SC-WATBAL-001 must include HPHYS0202 FC/WP layer-aggregation authority"
    );
    assert!(
        watbal.contains("HPHYS0206-CORRECTED-LAYER-NORMALIZATION-AND-MAPPING-CLOSURE-HISTORICAL")
            || watbal.contains("### HPHYS0206 Corrected-Layer Normalization and Mapping Closure"),
        "SC-WATBAL-001 must include HPHYS0206 normalized corrected-layer mapping authority"
    );
    assert!(
        watbal.contains("### HPHYS0207 FC/WP Depth-Authority and Normalized-Tail Closure"),
        "SC-WATBAL-001 must include HPHYS0207 FC/WP depth-authority closure section"
    );
    assert!(
        watbal.contains("HPHYS0216-PROFILEFC-LAYER-AUTHORITY-REALIGNMENT")
            || watbal.contains("### HPHYS0216 ProfileFC Layer-Authority Realignment"),
        "SC-WATBAL-001 must include HPHYS0216 FC publication authority realignment section"
    );
    assert!(
        watbal.contains("HPHYS0216D-PROFILEFC-LAYER-TAIL-AUTHORITY-RECONCILIATION")
            || watbal.contains("### HPHYS0216D ProfileFC Layer+Tail Authority Reconciliation"),
        "SC-WATBAL-001 must include HPHYS0216D FC layer+tail authority reconciliation section"
    );
    assert!(
        (watbal
            .contains("HPHYS0202-PROFILEFC-PROFILEWP-LAYER-AGGREGATION-LINEAGE-CLOSURE-HISTORIC")
            || watbal.contains(
                "HPHYS0206-CORRECTED-LAYER-NORMALIZATION-AND-MAPPING-CLOSURE-HISTORICAL"
            )
            || watbal.contains("HPHYS0216-PROFILEFC-LAYER-AUTHORITY-REALIGNMENT"))
            && watbal.contains("superseded"),
        "SC-WATBAL-001 historical FC/WP authority rows must be marked superseded"
    );
    assert!(
        soil.contains("HPHYS0202 narrows publication authority"),
        "SC-SOIL-001 must mark FC/WP seeds as non-authoritative publication sources"
    );
    assert!(
        soil.contains("HPHYS0206 requires those authoritative `thetfc_####`/`thetdr_####` symbols"),
        "SC-SOIL-001 must include HPHYS0206 normalized corrected-layer projection authority"
    );
    assert!(
        soil.contains("## HPHYS0216 ProfileFC Layer-Authority Realignment Addendum"),
        "SC-SOIL-001 must include HPHYS0216 FC layer-authority addendum"
    );
    assert!(
        soil.contains("## HPHYS0216D ProfileFC Normalized-Tail Contribution Addendum"),
        "SC-SOIL-001 must include HPHYS0216D FC normalized-tail contribution addendum"
    );
    assert!(
        perc.contains("HPHYS0206"),
        "SC-PERC-001 must include HPHYS0206 normalized mapping/no-fallback authority"
    );
    assert!(
        perc.contains("HPHYS0207"),
        "SC-PERC-001 must include HPHYS0207 normalized-profile storage authority"
    );
    assert!(
        perc.contains("HPHYS0216"),
        "SC-PERC-001 must include HPHYS0216 FC layer-authority realignment"
    );
    assert!(
        perc.contains("HPHYS0216D"),
        "SC-PERC-001 must include HPHYS0216D FC layer+tail authority reconciliation"
    );
    assert!(
        system.contains("HPHYS0202-WB13-PROFILE-FC-WP-PUBLICATION-LINEAGE-ADDENDUM-HISTORICAL")
            && system.contains("INV-SYSTEM-027")
            && system_provenance.contains(
                "## HPHYS0202-WB13-PROFILE-FC-WP-PUBLICATION-LINEAGE-ADDENDUM-HISTORICAL"
            ),
        "SC-SYSTEM-001 must expose HPHYS0202 historical system-boundary residue through BEI and provenance"
    );
    assert!(
        system.contains("HPHYS0206-NORMALIZED-LAYER-MAPPING-AND-FAIL-CLOSED-ADDENDUM-HISTORICAL")
            && system.contains("INV-SYSTEM-027")
            && system_provenance.contains(
                "## HPHYS0206-NORMALIZED-LAYER-MAPPING-AND-FAIL-CLOSED-ADDENDUM-HISTORICAL"
            ),
        "SC-SYSTEM-001 must expose HPHYS0206 historical normalized-layer residue through BEI and provenance"
    );
    assert!(
        system.contains("## HPHYS0207 Normalized-Profile FC/WP Depth-Authority Addendum"),
        "SC-SYSTEM-001 must include HPHYS0207 normalized-profile storage authority"
    );
    assert!(
        system.contains("## HPHYS0216D ProfileFC Layer+Tail Boundary Authority Addendum"),
        "SC-SYSTEM-001 must include HPHYS0216D system-boundary FC layer+tail authority addendum"
    );
    assert!(
        system_provenance.contains("superseded_by: HPHYS0207")
            && system.contains("HPHYS0207-NORMALIZED-PROFILE-FC-WP-DEPTH-AUTHORITY-ADDENDUM"),
        "SC-SYSTEM-001 historical FC/WP authority notes must be explicitly superseded by HPHYS0207 through provenance"
    );
}

#[test]
fn hphys0216d_profile_fc_layer_plus_tail_and_wp_projected_storage_authority() {
    let _execution_guard = runner_execution_lock()
        .lock()
        .expect("runner execution lock should be acquirable");

    let source_fixture_dir = fixture_path("hillslope_run_dir");
    let temp_run_dir = copy_fixture_to_temp(&source_fixture_dir, "hphys0202_layer_aggregation");
    let soil_path = temp_run_dir.join("case.sol");
    let soil_text = fs::read_to_string(&soil_path).expect("soil fixture should be readable");

    let soil_profile = parse_soil(&soil_text, soil_parser_options_for_fixture())
        .expect("soil fixture should parse");
    let soil_surface = build_hillslope_runtime_surface_from_soil(&soil_profile)
        .expect("soil fixture should project runtime state");
    let expected_layer = expected_profile_aggregation_from_layers(&soil_surface);
    let expected_fc_tail = required_surface_scalar(&soil_surface, "wb13_profile_fc_tail_mm");
    let expected_projected_fc = required_surface_scalar(&soil_surface, "wb13_profile_fc_store_mm");
    let expected_projected_wp = required_surface_scalar(&soil_surface, "wb13_profile_wp_store_mm");

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
    let row = load_first_wat_row(wat_path);
    let observed_fc = row_f64_value(&row, "ProfileFCStore");
    let observed_wp = row_f64_value(&row, "ProfileWPStore");

    assert_close(
        observed_fc,
        expected_layer.fc_store_mm + expected_fc_tail,
        "ProfileFCStore must follow normalized primary layer aggregation plus residual tail contribution",
    );
    assert_close(
        observed_fc,
        expected_projected_fc,
        "ProfileFCStore combined layer+tail authority must reconcile to normalized-profile projected storage",
    );
    assert_close(
        observed_wp,
        expected_projected_wp,
        "ProfileWPStore must follow wb13_profile_wp_store_mm projected storage authority",
    );
    assert!(
        expected_fc_tail.abs() < 1.0e-9
            && (observed_fc - expected_layer.fc_store_mm).abs() < 1.0e-9,
        "HPHYS0254 primary layers must cover normalized-profile depth so FC tail residual is zero"
    );
    assert!(
        (observed_wp - expected_layer.wp_store_mm).abs() < 1.0e-9,
        "HPHYS0254 primary layers must cover normalized-profile depth for WP storage"
    );
}

#[test]
fn hphys0202_invalid_layer_storage_state_hard_fails_runtime_surface() {
    let mut soil_profile = parse_soil(VALID_9002, soil_parser_options_for_fixture())
        .expect("9002 fixture should parse");
    soil_profile.ofes[0].layers[0].bulk_density_g_cm3 = None;

    let error = build_hillslope_runtime_surface_from_soil(&soil_profile)
        .expect_err("missing normalized corrected-lineage input must hard-fail runtime surface");
    assert_eq!(error.code(), "HS-RUNTIME-E-060");
    assert!(matches!(
        error,
        HillslopeRuntimeInputError::MissingCorrectedLayerNormalizationInput {
            ofe_index: 1,
            layer_index: 1,
            field: "bulk_density_g_cm3"
        }
    ));
}

#[test]
fn hphys0207_profile_storage_projection_differs_from_parser_layer_depth_aggregation() {
    let soil_profile = parse_soil(VALID_9002, soil_parser_options_for_fixture())
        .expect("9002 fixture should parse");
    let raw_parser_aggregation = raw_profile_aggregation_from_parser_layers(&soil_profile);

    let runtime_surface = build_hillslope_runtime_surface_from_soil(&soil_profile)
        .expect("runtime surface should build from parsed soil");
    let parser_layer_aggregation = expected_profile_aggregation_from_layers(&runtime_surface);
    let projected_fc_seed = required_surface_scalar(&runtime_surface, "wb13_profile_fc_store_mm");
    let projected_wp_seed = required_surface_scalar(&runtime_surface, "wb13_profile_wp_store_mm");

    assert!(
        (parser_layer_aggregation.fc_store_mm - raw_parser_aggregation.fc_store_mm).abs() > 1.0e-6
            || (parser_layer_aggregation.wp_store_mm - raw_parser_aggregation.wp_store_mm).abs()
                > 1.0e-6,
        "authoritative layer FC/WP symbols must not remain raw parser theta lineage"
    );
    assert!(
        (projected_fc_seed - parser_layer_aggregation.fc_store_mm).abs() < 1.0e-9,
        "projected FC storage must be represented by normalized primary WB11 layer aggregation"
    );
    assert!(
        (projected_wp_seed - parser_layer_aggregation.wp_store_mm).abs() < 1.0e-9,
        "projected WP storage must be represented by normalized primary WB11 layer aggregation"
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

fn expected_profile_aggregation_from_layers(
    surface: &HillslopeWritebackSurface,
) -> ProfileAggregation {
    let nsl_raw = required_surface_scalar(surface, "wb11_nsl");
    let nsl_text = format!("{nsl_raw:.0}");
    let nsl = nsl_text
        .parse::<usize>()
        .unwrap_or_else(|error| panic!("nsl must convert to usize from {nsl_raw}: {error}"));
    assert!(nsl >= 1, "nsl must be >= 1, observed {nsl}");

    let mut fc_store_mm = 0.0_f64;
    let mut wp_store_mm = 0.0_f64;
    for layer_index in 1..=nsl {
        let dg_m = required_surface_scalar(surface, &format!("wb19_dg_{layer_index:04}"));
        let thetfc = required_surface_scalar(surface, &format!("wb19_thetfc_{layer_index:04}"));
        let thetdr = required_surface_scalar(surface, &format!("wb19_thetdr_{layer_index:04}"));
        fc_store_mm += thetfc * dg_m * 1_000.0;
        wp_store_mm += thetdr * dg_m * 1_000.0;
    }

    ProfileAggregation {
        fc_store_mm,
        wp_store_mm,
    }
}

fn raw_profile_aggregation_from_parser_layers(soil: &SoilProfile) -> ProfileAggregation {
    let primary_ofe = soil
        .ofes
        .first()
        .expect("soil profile must include at least one OFE");
    let mut previous_depth_mm = 0.0_f64;
    let mut fc_store_mm = 0.0_f64;
    let mut wp_store_mm = 0.0_f64;

    for layer in &primary_ofe.layers {
        let layer_depth_mm = layer.depth_mm;
        let dg_m = (layer_depth_mm - previous_depth_mm) / 1_000.0;
        let thetfc = layer
            .fc_rosetta
            .or(layer.fc_measured)
            .expect("fixture layer must provide field-capacity theta");
        let thetdr = layer
            .theta_r_rosetta
            .or(layer.wp_measured)
            .expect("fixture layer must provide wilting-point theta");
        fc_store_mm += thetfc * dg_m * 1_000.0;
        wp_store_mm += thetdr * dg_m * 1_000.0;
        previous_depth_mm = layer_depth_mm;
    }

    ProfileAggregation {
        fc_store_mm,
        wp_store_mm,
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

fn load_first_wat_row(wat_path: &Path) -> Row {
    let file = File::open(wat_path)
        .unwrap_or_else(|error| panic!("wat parquet output should be readable: {error}"));
    let reader = SerializedFileReader::new(file)
        .unwrap_or_else(|error| panic!("wat parquet output should parse: {error}"));
    let mut rows = reader
        .get_row_iter(None)
        .unwrap_or_else(|error| panic!("wat parquet row iterator should open: {error}"));
    rows.next()
        .expect("wat parquet output should contain at least one row")
        .unwrap_or_else(|error| panic!("wat parquet row should decode: {error}"))
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
    let index = row_index(row, column_name);
    if let Ok(value) = row.get_double(index) {
        return value;
    }
    if let Ok(value) = row.get_float(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_int(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_short(index) {
        return f64::from(value);
    }
    if let Ok(value) = row.get_long(index) {
        return value
            .to_string()
            .parse::<f64>()
            .unwrap_or_else(|error| panic!("i64 column '{column_name}' parse failure: {error}"));
    }
    panic!("column '{column_name}' does not decode as numeric");
}

fn assert_close(observed: f64, expected: f64, context: &str) {
    let delta = (observed - expected).abs();
    assert!(
        delta <= EPS,
        "{context}: observed={observed}, expected={expected}, |delta|={delta}, tolerance={EPS}"
    );
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
