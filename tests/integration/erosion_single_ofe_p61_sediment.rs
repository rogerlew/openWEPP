//! SC-SED-001 1b-C regression: the enabled single-OFE Wave-1 sediment-
//! continuity solve produces nonzero erosion through the direct-production
//! runtime. Runs the operator-supplied `p61` fixture (single OFE, real
//! climate with erosion events; legacy WEPP `H61.ebe.dat` reports 4 events)
//! end-to-end and asserts the pass parquet carries nonzero total
//! detachment. Guards against the class of latent bugs that only surface
//! once the seed is live (the activation gate, `rspace` sentinel, and the
//! fractional-vs-meter slope-x normalization).

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use arrow_array::{Array, Float64Array};
use openwepp_runner::{HillslopeRunRequest, SidecarPolicy, execute_hillslope_run};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

#[test]
fn erosion_single_ofe_p61_produces_nonzero_sediment_through_direct_runtime() {
    let fixture = fixture_path("erosion_single_ofe_p61");
    let run_dir = copy_fixture_to_temp(&fixture, "erosion_p61");
    let output_dir = run_dir.join("output");

    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("p61.run"),
            output_dir: output_dir.clone(),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &[
            "openwepp-cli-hill".to_string(),
            "--run-dir".to_string(),
            run_dir.display().to_string(),
            "--run-file".to_string(),
            "p61.run".to_string(),
            "--output-dir".to_string(),
            output_dir.display().to_string(),
            "--direct-production-executor".to_string(),
        ],
    )
    .expect("p61 single-OFE direct-production run should complete");

    let pass_parquet = report
        .optional_outputs
        .iter()
        .find(|path| path.file_name().and_then(|name| name.to_str()) == Some("H61.pass.parquet"))
        .expect("p61 run should emit the pass parquet");

    let (tdet_sum, tdet_max, nonzero_days) = column_summary(pass_parquet, "tdet");

    // The dominant p61 storm clears the Wave-1 `passby` gate and detaches;
    // the runtime must surface it (guarding against a silently-inert flip).
    assert!(
        tdet_sum > 0.0 && tdet_max > 0.0 && nonzero_days >= 1,
        "single-OFE Wave-1 erosion must produce nonzero total detachment \
         (sum={tdet_sum}, max={tdet_max}, nonzero_days={nonzero_days})"
    );

    // Total detachment must be finite and mass-nonnegative.
    assert!(tdet_sum.is_finite(), "total detachment must be finite");
}

/// Sum, max, and nonzero-count of a `f64` parquet column.
fn column_summary(path: &Path, column: &str) -> (f64, f64, usize) {
    let file = File::open(path).expect("open pass parquet");
    let reader = ParquetRecordBatchReaderBuilder::try_new(file)
        .expect("parquet reader builder")
        .build()
        .expect("build parquet reader");
    let mut sum = 0.0_f64;
    let mut max = f64::NEG_INFINITY;
    let mut nonzero = 0_usize;
    for batch in reader {
        let batch = batch.expect("read record batch");
        let index = batch
            .schema()
            .index_of(column)
            .unwrap_or_else(|_| panic!("pass parquet must carry the `{column}` column"));
        let array = batch
            .column(index)
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap_or_else(|| panic!("`{column}` must be Float64"));
        for i in 0..array.len() {
            if array.is_valid(i) {
                let value = array.value(i);
                sum += value;
                if value > max {
                    max = value;
                }
                if value > 0.0 {
                    nonzero += 1;
                }
            }
        }
    }
    (sum, if max.is_finite() { max } else { 0.0 }, nonzero)
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn copy_fixture_to_temp(source_dir: &Path, prefix: &str) -> PathBuf {
    let destination = std::env::temp_dir().join(format!("{prefix}_{}", std::process::id()));
    if destination.exists() {
        fs::remove_dir_all(&destination).expect("clear prior temp fixture");
    }
    copy_dir_recursive(source_dir, &destination);
    destination
}

fn copy_dir_recursive(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create temp fixture dir");
    for entry in fs::read_dir(source).expect("read fixture dir") {
        let entry = entry.expect("read fixture entry");
        let path = entry.path();
        let target = destination.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &target);
        } else {
            fs::copy(&path, &target).expect("copy fixture file");
        }
    }
}
