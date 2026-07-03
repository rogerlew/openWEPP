use std::collections::BTreeSet;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use openwepp_runner::{
    HillslopeRunReport, HillslopeRunRequest, SidecarPolicy, execute_hillslope_run,
};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};
use serde_json::Value;

#[derive(Debug, Clone)]
struct MatrixCell {
    wepp_id: usize,
    texture: String,
    veg_type: String,
    severity: u8,
    severity_name: String,
    disturbed_class: String,
    management_file: String,
}

#[derive(Debug)]
struct PassMetrics {
    rows: usize,
    total_runoff_m3: f64,
    max_daily_runoff_m3: f64,
    max_peak_runoff_m3_s: f64,
    total_detachment_kg: f64,
    total_deposition_kg: f64,
    max_sediment_concentration_kg_m3: f64,
}

#[test]
fn dff_ws3_mckenzie_bridge_matrix_fixture_catalog_is_complete() {
    let root = matrix_fixture_path();
    assert!(root.join("common/mckenzie_bridge.cli").is_file());
    assert!(root.join("common/canonical_201m.slp").is_file());
    assert!(root.join("SHA256SUMS").is_file());

    let cells = matrix_cells();
    assert_eq!(cells.len(), 80, "WS-3 matrix must cover 80 cells");

    let ids = cells
        .iter()
        .map(|cell| cell.wepp_id)
        .collect::<BTreeSet<_>>();
    assert_eq!(ids, (1..=80).collect::<BTreeSet<_>>());
    assert_eq!(
        cells
            .iter()
            .map(|cell| cell.texture.as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["clay loam", "loam", "sand loam", "silt loam"])
    );
    assert_eq!(
        cells
            .iter()
            .map(|cell| cell.veg_type.as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "deciduous forest",
            "forest",
            "mixed forest",
            "shrub",
            "tall grass",
        ])
    );
    assert_eq!(
        cells
            .iter()
            .map(|cell| (cell.severity, cell.severity_name.as_str()))
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([(0, "unburned"), (1, "low"), (2, "moderate"), (3, "high"),])
    );

    for cell in &cells {
        let prefix = format!("inputs/p{}", cell.wepp_id);
        assert!(
            root.join(format!("{prefix}.run")).is_file(),
            "missing p{} run recipe",
            cell.wepp_id
        );
        assert!(
            root.join(format!("{prefix}.man")).is_file(),
            "missing p{} management file",
            cell.wepp_id
        );
        assert!(
            root.join(format!("{prefix}.sol")).is_file(),
            "missing p{} soil file",
            cell.wepp_id
        );
        assert!(!cell.disturbed_class.trim().is_empty());
        assert!(cell.management_file.ends_with(".man"));
    }
}

#[test]
fn dff_ws3_representative_clay_loam_documents_runoff_peak_and_sediment_hold() {
    let unburned = run_matrix_cell(1, "dff_ws3_p1_unburned");
    let high_burn = run_matrix_cell(4, "dff_ws3_p4_high_burn");

    assert_direct_production_run(&unburned.report, 1);
    assert_direct_production_run(&high_burn.report, 4);
    assert_eq!(unburned.metrics.rows, 2192);
    assert_eq!(high_burn.metrics.rows, 2192);

    assert!(
        high_burn.metrics.total_runoff_m3 > unburned.metrics.total_runoff_m3,
        "WS-3 representative runoff law failed: high burn {} m3 must exceed unburned {} m3",
        high_burn.metrics.total_runoff_m3,
        unburned.metrics.total_runoff_m3
    );
    assert!(
        high_burn.metrics.max_peak_runoff_m3_s > unburned.metrics.max_peak_runoff_m3_s,
        "WS-3 representative peak law failed: high burn {} m3/s must exceed unburned {} m3/s",
        high_burn.metrics.max_peak_runoff_m3_s,
        unburned.metrics.max_peak_runoff_m3_s
    );
    assert!(
        high_burn.metrics.max_daily_runoff_m3 > unburned.metrics.max_daily_runoff_m3,
        "WS-3 representative max daily runoff law failed: high burn {} m3 must exceed unburned {} m3",
        high_burn.metrics.max_daily_runoff_m3,
        unburned.metrics.max_daily_runoff_m3
    );

    assert_eq!(
        high_burn.metrics.total_detachment_kg, 0.0,
        "HOLD-DFF-WS3-SEDIMENT-PRODUCTION: sediment ordering waits on proper Wave-1/Wave-2 production"
    );
    assert_eq!(
        unburned.metrics.total_detachment_kg, 0.0,
        "HOLD-DFF-WS3-SEDIMENT-PRODUCTION: sediment ordering waits on proper Wave-1/Wave-2 production"
    );
    assert_eq!(
        high_burn.metrics.total_deposition_kg, 0.0,
        "HOLD-DFF-WS3-SEDIMENT-PRODUCTION: sediment ordering waits on proper Wave-1/Wave-2 production"
    );
    assert_eq!(
        unburned.metrics.total_deposition_kg, 0.0,
        "HOLD-DFF-WS3-SEDIMENT-PRODUCTION: sediment ordering waits on proper Wave-1/Wave-2 production"
    );
    assert_eq!(
        high_burn.metrics.max_sediment_concentration_kg_m3, 0.0,
        "HOLD-DFF-WS3-SEDIMENT-PRODUCTION: sediment ordering waits on proper Wave-1/Wave-2 production"
    );
}

struct FixtureRun {
    report: HillslopeRunReport,
    metrics: PassMetrics,
}

fn run_matrix_cell(wepp_id: usize, temp_prefix: &str) -> FixtureRun {
    let temp_run_dir = copy_matrix_cell_to_temp(wepp_id, temp_prefix);
    normalize_legacy_nan_dewpoint_tokens(&temp_run_dir);
    let run_file = format!("p{wepp_id}.run");
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.clone(),
            run_file: PathBuf::from(&run_file),
            output_dir: temp_run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )
    .unwrap_or_else(|error| {
        panic!("DFF-WS3 matrix cell p{wepp_id} should run end-to-end: {error}")
    });
    let pass_parquet = report
        .optional_outputs
        .iter()
        .find(|path| {
            path.file_name()
                .and_then(|value| value.to_str())
                .is_some_and(|name| name.ends_with(".pass.parquet"))
        })
        .unwrap_or_else(|| panic!("DFF-WS3 p{wepp_id} should publish pass parquet: {report:?}"));
    let metrics = read_pass_metrics(pass_parquet);
    FixtureRun { report, metrics }
}

fn copy_matrix_cell_to_temp(wepp_id: usize, prefix: &str) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix epoch should be before now")
        .as_nanos();
    let destination = std::env::temp_dir().join(format!("{prefix}_{timestamp}"));
    fs::create_dir_all(&destination).expect("destination directory should be creatable");
    fs::create_dir_all(destination.join("output")).expect("output directory should be creatable");

    let root = matrix_fixture_path();
    fs::copy(
        root.join(format!("inputs/p{wepp_id}.man")),
        destination.join(format!("p{wepp_id}.man")),
    )
    .expect("management copy should succeed");
    fs::copy(
        root.join(format!("inputs/p{wepp_id}.sol")),
        destination.join(format!("p{wepp_id}.sol")),
    )
    .expect("soil copy should succeed");
    fs::copy(
        root.join("common/canonical_201m.slp"),
        destination.join(format!("p{wepp_id}.slp")),
    )
    .expect("slope copy should succeed");
    fs::copy(
        root.join("common/mckenzie_bridge.cli"),
        destination.join(format!("p{wepp_id}.cli")),
    )
    .expect("climate copy should succeed");

    fs::write(
        destination.join(format!("p{wepp_id}.run")),
        matrix_runfile(wepp_id),
    )
    .expect("temporary runfile should be writable");
    destination
}

fn matrix_runfile(wepp_id: usize) -> String {
    format!(
        r#"schema = "openwepp-hillslope-runfile-v1"
run_name = "dff-ws3-mckenzie-bridge-{wepp_id}"
unit_system = "metric"

[inputs]
soil = "p{wepp_id}.sol"
management = "p{wepp_id}.man"
slope = "p{wepp_id}.slp"
climate = "p{wepp_id}.cli"
wepp_ui = false

[outputs]
pass = "output/H{wepp_id}.hbp"
pass_parquet = "output/H{wepp_id}.pass.parquet"
loss = "output/H{wepp_id}.loss.json"
wat = "output/H{wepp_id}.wat.parquet"
"#
    )
}

fn normalize_legacy_nan_dewpoint_tokens(run_dir: &Path) {
    for entry in fs::read_dir(run_dir).expect("temporary fixture directory should be readable") {
        let entry = entry.expect("temporary fixture entry should be readable");
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("cli") {
            continue;
        }
        let contents = fs::read_to_string(&path).expect("temporary climate file should read");
        let updated = contents.replace(" nan", " 0.0");
        if updated != contents {
            fs::write(&path, updated).expect("temporary climate file should be writable");
        }
    }
}

fn assert_direct_production_run(report: &HillslopeRunReport, wepp_id: usize) {
    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());
    assert!(
        report
            .optional_outputs
            .iter()
            .any(|path| path.ends_with(format!("H{wepp_id}.pass.parquet"))),
        "DFF-WS3 fixture should publish H{wepp_id}.pass.parquet"
    );

    let manifest = fs::read_to_string(&report.manifest_path).expect("manifest should read");
    let manifest: Value = serde_json::from_str(&manifest).expect("manifest should parse");
    assert_eq!(
        manifest
            .pointer("/runtime_selection/selected")
            .and_then(Value::as_str),
        Some("direct-production-executor")
    );
    assert_eq!(
        manifest
            .pointer("/execution_provenance/climate_day_count")
            .and_then(Value::as_u64),
        Some(2192)
    );
    assert_eq!(
        manifest
            .pointer("/execution_provenance/executed_day_count")
            .and_then(Value::as_u64),
        Some(2192)
    );
}

fn read_pass_metrics(pass_parquet: &Path) -> PassMetrics {
    let rows = read_parquet_rows(pass_parquet);
    assert!(!rows.is_empty(), "pass parquet should contain daily rows");

    let mut metrics = PassMetrics {
        rows: rows.len(),
        total_runoff_m3: 0.0,
        max_daily_runoff_m3: 0.0,
        max_peak_runoff_m3_s: 0.0,
        total_detachment_kg: 0.0,
        total_deposition_kg: 0.0,
        max_sediment_concentration_kg_m3: 0.0,
    };

    for row in rows {
        let runoff = row_f64_value(&row, "runvol");
        metrics.total_runoff_m3 += runoff;
        metrics.max_daily_runoff_m3 = metrics.max_daily_runoff_m3.max(runoff);
        metrics.max_peak_runoff_m3_s = metrics
            .max_peak_runoff_m3_s
            .max(row_f64_value(&row, "peakro"));
        metrics.total_detachment_kg += row_f64_value(&row, "tdet");
        metrics.total_deposition_kg += row_f64_value(&row, "tdep");
        for column in ["sedcon_1", "sedcon_2", "sedcon_3", "sedcon_4", "sedcon_5"] {
            metrics.max_sediment_concentration_kg_m3 = metrics
                .max_sediment_concentration_kg_m3
                .max(row_f64_value(&row, column));
        }
    }

    metrics
}

fn read_parquet_rows(path: &Path) -> Vec<Row> {
    let file = File::open(path).unwrap_or_else(|error| {
        panic!(
            "parquet output should be readable ({}): {error}",
            path.display()
        )
    });
    let reader = SerializedFileReader::new(file).unwrap_or_else(|error| {
        panic!("parquet output should parse ({}): {error}", path.display())
    });
    reader
        .get_row_iter(None)
        .unwrap_or_else(|error| {
            panic!(
                "parquet row iterator should open ({}): {error}",
                path.display()
            )
        })
        .map(|row| {
            row.unwrap_or_else(|error| {
                panic!("parquet row should decode ({}): {error}", path.display())
            })
        })
        .collect()
}

fn row_index(row: &Row, column_name: &str) -> usize {
    row.get_column_iter()
        .enumerate()
        .find(|(_, (name, _))| name.as_str() == column_name)
        .map_or_else(
            || panic!("missing required parquet column '{column_name}'"),
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

fn matrix_cells() -> Vec<MatrixCell> {
    let matrix = fs::read_to_string(matrix_fixture_path().join("matrix.csv"))
        .expect("WS-3 matrix catalog should be readable");
    matrix
        .lines()
        .enumerate()
        .skip(1)
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(line_index, line)| {
            let fields = line.split(',').collect::<Vec<_>>();
            assert_eq!(
                fields.len(),
                7,
                "matrix.csv line {} should have 7 fields",
                line_index + 1
            );
            MatrixCell {
                wepp_id: fields[0].parse().unwrap_or_else(|error| {
                    panic!("invalid wepp_id on line {}: {error}", line_index + 1)
                }),
                texture: fields[1].to_string(),
                veg_type: fields[2].to_string(),
                severity: fields[3].parse().unwrap_or_else(|error| {
                    panic!("invalid severity on line {}: {error}", line_index + 1)
                }),
                severity_name: fields[4].to_string(),
                disturbed_class: fields[5].to_string(),
                management_file: fields[6].to_string(),
            }
        })
        .collect()
}

fn matrix_fixture_path() -> PathBuf {
    fixture_path("mckenzie_bridge_80_cell_matrix")
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("integration file parent exists")
        .parent()
        .expect("tests directory exists")
        .join("fixtures")
        .join("disturbed_burn")
        .join(name)
}
