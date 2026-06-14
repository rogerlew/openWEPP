use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use arrow_array::{Array, ArrayRef, Float64Array, Int8Array, Int16Array, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use parquet::arrow::{ArrowWriter, arrow_reader::ParquetRecordBatchReaderBuilder};
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("openwepp_{name}_{nanos}"))
}

fn write_fixture_parquet(path: &Path, schema: Schema, columns: Vec<ArrayRef>) {
    fs::create_dir_all(path.parent().expect("fixture parquet should have parent"))
        .expect("fixture parent should be created");
    let batch = RecordBatch::try_new(Arc::new(schema.clone()), columns)
        .expect("fixture record batch should build");
    let file = File::create(path).expect("fixture parquet should be created");
    let writer_properties = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), Some(writer_properties))
        .expect("fixture parquet writer should initialize");
    writer.write(&batch).expect("fixture batch should write");
    writer.close().expect("fixture parquet should close");
}

fn required_date_fields() -> Vec<Field> {
    vec![
        Field::new("wepp_id", DataType::Int32, true),
        Field::new("year", DataType::Int16, true),
        Field::new("sim_day_index", DataType::Int32, true),
        Field::new("julian", DataType::Int16, true),
        Field::new("month", DataType::Int8, true),
        Field::new("day_of_month", DataType::Int8, true),
        Field::new("water_year", DataType::Int16, true),
    ]
}

fn write_pass_fixture(path: &Path) {
    let mut fields = required_date_fields();
    fields.extend([
        Field::new("runvol", DataType::Float64, true),
        Field::new("sbrunv", DataType::Float64, true),
        Field::new("tdet", DataType::Float64, true),
        Field::new("tdep", DataType::Float64, true),
        Field::new("sedcon_1", DataType::Float64, true),
        Field::new("sedcon_2", DataType::Float64, true),
        Field::new("sedcon_3", DataType::Float64, true),
        Field::new("sedcon_4", DataType::Float64, true),
        Field::new("sedcon_5", DataType::Float64, true),
    ]);
    write_fixture_parquet(
        path,
        Schema::new(fields),
        vec![
            Arc::new(Int32Array::from(vec![1, 2])),
            Arc::new(Int16Array::from(vec![2004, 2004])),
            Arc::new(Int32Array::from(vec![1, 1])),
            Arc::new(Int16Array::from(vec![1, 1])),
            Arc::new(Int8Array::from(vec![1, 1])),
            Arc::new(Int8Array::from(vec![1, 1])),
            Arc::new(Int16Array::from(vec![2004, 2004])),
            Arc::new(Float64Array::from(vec![5.0, 7.0])),
            Arc::new(Float64Array::from(vec![0.5, 0.7])),
            Arc::new(Float64Array::from(vec![1.0, 2.0])),
            Arc::new(Float64Array::from(vec![0.1, 0.2])),
            Arc::new(Float64Array::from(vec![2.0, 1.0])),
            Arc::new(Float64Array::from(vec![3.0, 0.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0])),
        ],
    );
}

fn write_openwepp_per_hill_pass_fixture(path: &Path) {
    let mut fields = required_date_fields();
    fields.extend([
        Field::new("runvol", DataType::Float64, true),
        Field::new("sbrunv", DataType::Float64, true),
        Field::new("tdet", DataType::Float64, true),
        Field::new("tdep", DataType::Float64, true),
        Field::new("sedcon_1", DataType::Float64, true),
        Field::new("sedcon_2", DataType::Float64, true),
        Field::new("sedcon_3", DataType::Float64, true),
        Field::new("sedcon_4", DataType::Float64, true),
        Field::new("sedcon_5", DataType::Float64, true),
    ]);
    write_fixture_parquet(
        path,
        Schema::new(fields),
        vec![
            Arc::new(Int32Array::from(vec![1])),
            Arc::new(Int16Array::from(vec![2004])),
            Arc::new(Int32Array::from(vec![1])),
            Arc::new(Int16Array::from(vec![1])),
            Arc::new(Int8Array::from(vec![1])),
            Arc::new(Int8Array::from(vec![1])),
            Arc::new(Int16Array::from(vec![2004])),
            Arc::new(Float64Array::from(vec![12.0])),
            Arc::new(Float64Array::from(vec![0.0])),
            Arc::new(Float64Array::from(vec![0.0])),
            Arc::new(Float64Array::from(vec![0.0])),
            Arc::new(Float64Array::from(vec![0.0])),
            Arc::new(Float64Array::from(vec![0.0])),
            Arc::new(Float64Array::from(vec![0.0])),
            Arc::new(Float64Array::from(vec![0.0])),
            Arc::new(Float64Array::from(vec![0.0])),
        ],
    );
}

fn write_wat_fixture(path: &Path) {
    let mut fields = required_date_fields();
    fields.insert(1, Field::new("ofe_id", DataType::Int16, true));
    fields.extend([
        Field::new("OFE", DataType::Int16, true),
        Field::new("P", DataType::Float64, true),
        Field::new("RM", DataType::Float64, true),
        Field::new("Q", DataType::Float64, true),
        Field::new("Ep", DataType::Float64, true),
        Field::new("Es", DataType::Float64, true),
        Field::new("Er", DataType::Float64, true),
        Field::new("Dp", DataType::Float64, true),
        Field::new("UpStrmQ", DataType::Float64, true),
        Field::new("SubRIn", DataType::Float64, true),
        Field::new("latqcc", DataType::Float64, true),
        Field::new("Total-Soil Water", DataType::Float64, true),
        Field::new("frozwt", DataType::Float64, true),
        Field::new("Snow-Water", DataType::Float64, true),
        Field::new("QOFE", DataType::Float64, true),
        Field::new("Tile", DataType::Float64, true),
        Field::new("Irr", DataType::Float64, true),
        Field::new("Area", DataType::Float64, true),
        Field::new("SoilWaterTotal", DataType::Float64, true),
        Field::new("ProfileDepth", DataType::Float64, true),
        Field::new("ProfilePorosityCap", DataType::Float64, true),
        Field::new("ProfileFCStore", DataType::Float64, true),
        Field::new("ProfileWPStore", DataType::Float64, true),
        Field::new("InterceptionStorage", DataType::Float64, true),
    ]);
    write_fixture_parquet(
        path,
        Schema::new(fields),
        vec![
            Arc::new(Int32Array::from(vec![1, 1, 2])),
            Arc::new(Int16Array::from(vec![1, 2, 1])),
            Arc::new(Int16Array::from(vec![2004, 2004, 2004])),
            Arc::new(Int32Array::from(vec![1, 1, 1])),
            Arc::new(Int16Array::from(vec![1, 1, 1])),
            Arc::new(Int8Array::from(vec![1, 1, 1])),
            Arc::new(Int8Array::from(vec![1, 1, 1])),
            Arc::new(Int16Array::from(vec![2004, 2004, 2004])),
            Arc::new(Int16Array::from(vec![1, 2, 1])),
            Arc::new(Float64Array::from(vec![10.0, 20.0, 30.0])),
            Arc::new(Float64Array::from(vec![8.0, 18.0, 28.0])),
            Arc::new(Float64Array::from(vec![99.0, 77.0, 11.0])),
            Arc::new(Float64Array::from(vec![1.0, 2.0, 3.0])),
            Arc::new(Float64Array::from(vec![0.5, 1.0, 1.5])),
            Arc::new(Float64Array::from(vec![0.25, 0.5, 0.75])),
            Arc::new(Float64Array::from(vec![0.1, 0.2, 0.3])),
            Arc::new(Float64Array::from(vec![0.0, 0.0, 0.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0, 0.0])),
            Arc::new(Float64Array::from(vec![100.0, 2.0, 5.0])),
            Arc::new(Float64Array::from(vec![100.0, 110.0, 120.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0, 0.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0, 0.0])),
            Arc::new(Float64Array::from(vec![3.0, 4.0, 6.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0, 0.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0, 0.0])),
            Arc::new(Float64Array::from(vec![1_000.0, 1_000.0, 500.0])),
            Arc::new(Float64Array::from(vec![100.0, 110.0, 120.0])),
            Arc::new(Float64Array::from(vec![1_000.0, 1_000.0, 1_000.0])),
            Arc::new(Float64Array::from(vec![250.0, 260.0, 270.0])),
            Arc::new(Float64Array::from(vec![180.0, 190.0, 200.0])),
            Arc::new(Float64Array::from(vec![60.0, 70.0, 80.0])),
            Arc::new(Float64Array::from(vec![0.2, 0.4, 0.6])),
        ],
    );
}

fn write_openwepp_per_hill_wat_fixture(path: &Path) {
    let mut fields = required_date_fields();
    fields.insert(1, Field::new("ofe_id", DataType::Int16, true));
    fields.extend([
        Field::new("OFE", DataType::Int16, true),
        Field::new("P", DataType::Float64, true),
        Field::new("RM", DataType::Float64, true),
        Field::new("Q", DataType::Float64, true),
        Field::new("Ep", DataType::Float64, true),
        Field::new("Es", DataType::Float64, true),
        Field::new("Er", DataType::Float64, true),
        Field::new("Dp", DataType::Float64, true),
        Field::new("UpStrmQ", DataType::Float64, true),
        Field::new("SubRIn", DataType::Float64, true),
        Field::new("latqcc", DataType::Float64, true),
        Field::new("Total-Soil Water", DataType::Float64, true),
        Field::new("frozwt", DataType::Float64, true),
        Field::new("Snow-Water", DataType::Float64, true),
        Field::new("QOFE", DataType::Float64, true),
        Field::new("Tile", DataType::Float64, true),
        Field::new("Irr", DataType::Float64, true),
        Field::new("Area", DataType::Float64, true),
        Field::new("SoilWaterTotal", DataType::Float64, true),
        Field::new("ProfileDepth", DataType::Float64, true),
        Field::new("ProfilePorosityCap", DataType::Float64, true),
        Field::new("ProfileFCStore", DataType::Float64, true),
        Field::new("ProfileWPStore", DataType::Float64, true),
        Field::new("InterceptionStorage", DataType::Float64, true),
    ]);
    write_fixture_parquet(
        path,
        Schema::new(fields),
        vec![
            Arc::new(Int32Array::from(vec![1, 1])),
            Arc::new(Int16Array::from(vec![1, 2])),
            Arc::new(Int16Array::from(vec![2004, 2004])),
            Arc::new(Int32Array::from(vec![1, 1])),
            Arc::new(Int16Array::from(vec![1, 1])),
            Arc::new(Int8Array::from(vec![1, 1])),
            Arc::new(Int8Array::from(vec![1, 1])),
            Arc::new(Int16Array::from(vec![2004, 2004])),
            Arc::new(Int16Array::from(vec![1, 2])),
            Arc::new(Float64Array::from(vec![10.0, 20.0])),
            Arc::new(Float64Array::from(vec![8.0, 18.0])),
            Arc::new(Float64Array::from(vec![99.0, 77.0])),
            Arc::new(Float64Array::from(vec![1.0, 2.0])),
            Arc::new(Float64Array::from(vec![0.5, 1.0])),
            Arc::new(Float64Array::from(vec![0.25, 0.5])),
            Arc::new(Float64Array::from(vec![0.1, 0.2])),
            Arc::new(Float64Array::from(vec![0.0, 0.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0])),
            Arc::new(Float64Array::from(vec![100.0, 4.5])),
            Arc::new(Float64Array::from(vec![100.0, 110.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0])),
            Arc::new(Float64Array::from(vec![3.0, 4.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0])),
            Arc::new(Float64Array::from(vec![0.0, 0.0])),
            Arc::new(Float64Array::from(vec![1_500.0, 1_000.0])),
            Arc::new(Float64Array::from(vec![100.0, 110.0])),
            Arc::new(Float64Array::from(vec![1_000.0, 1_000.0])),
            Arc::new(Float64Array::from(vec![250.0, 260.0])),
            Arc::new(Float64Array::from(vec![180.0, 190.0])),
            Arc::new(Float64Array::from(vec![60.0, 70.0])),
            Arc::new(Float64Array::from(vec![0.2, 0.4])),
        ],
    );
}

fn read_first_output_batch(path: &Path) -> RecordBatch {
    let file = File::open(path).expect("output parquet should be readable");
    let builder =
        ParquetRecordBatchReaderBuilder::try_new(file).expect("output parquet should have footer");
    let mut reader = builder
        .build()
        .expect("output parquet record batch reader should build");
    reader
        .next()
        .expect("output parquet should contain a batch")
        .expect("output parquet batch should decode")
}

fn f64_column(batch: &RecordBatch, name: &str) -> f64 {
    let schema = batch.schema();
    let column = batch
        .column(schema.index_of(name).expect("column should exist"))
        .as_any()
        .downcast_ref::<Float64Array>()
        .expect("column should be Float64");
    assert!(!column.is_null(0), "{name} should not be null");
    column.value(0)
}

#[test]
fn totalwatsed3_cli_fails_closed_when_required_pass_input_is_missing() {
    let base = unique_temp_dir("totalwatsed3_missing_pass");
    let input_dir = base.join("interchange");
    fs::create_dir_all(&input_dir).expect("input dir should be created");
    write_wat_fixture(&input_dir.join("H.wat.parquet"));

    let output = Command::new(env!("CARGO_BIN_EXE_openwepp-cli-totalwatsed3"))
        .arg("--input-dir")
        .arg(&input_dir)
        .arg("--output")
        .arg(base.join("totalwatsed3.parquet"))
        .output()
        .expect("totalwatsed3 CLI process should run");

    assert!(
        !output.status.success(),
        "missing required PASS input should fail closed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CLITW3-E-004"),
        "missing PASS input should emit typed CLI code, observed: {stderr}"
    );

    if base.exists() {
        fs::remove_dir_all(base).expect("temp dir cleanup should succeed");
    }
}

#[test]
fn totalwatsed3_cli_uses_pass_runvol_and_outlet_lateral_flow() {
    let base = unique_temp_dir("totalwatsed3_pass_runvol");
    let input_dir = base.join("interchange");
    fs::create_dir_all(&input_dir).expect("input dir should be created");
    write_pass_fixture(&input_dir.join("H.pass.parquet"));
    write_wat_fixture(&input_dir.join("H.wat.parquet"));
    let output_path = base.join("totalwatsed3.parquet");

    let output = Command::new(env!("CARGO_BIN_EXE_openwepp-cli-totalwatsed3"))
        .arg("--input-dir")
        .arg(&input_dir)
        .arg("--output")
        .arg(&output_path)
        .output()
        .expect("totalwatsed3 CLI process should run");

    assert!(
        output.status.success(),
        "totalwatsed3 CLI should succeed; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let batch = read_first_output_batch(&output_path);
    assert_eq!(batch.num_rows(), 1);

    assert!((f64_column(&batch, "Area") - 2_500.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "runvol") - 12.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "Runoff") - 4.8).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "Q") - 181.5).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "latqcc") - 4.5).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "Lateral Flow") - 1.8).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "QOFE") - 10.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "Interception") - 0.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "seddep_1") - 17.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "sed_del") - 32.0).abs() <= 1.0e-12);

    if base.exists() {
        fs::remove_dir_all(base).expect("temp dir cleanup should succeed");
    }
}

#[test]
fn totalwatsed3_cli_reads_openwepp_per_hillslope_pass_and_wat_surfaces() {
    let base = unique_temp_dir("totalwatsed3_openwepp_native_per_hill");
    let input_dir = base.join("interchange");
    fs::create_dir_all(&input_dir).expect("input dir should be created");
    write_openwepp_per_hill_pass_fixture(&input_dir.join("H1.pass.parquet"));
    write_openwepp_per_hill_wat_fixture(&input_dir.join("H1.wat.parquet"));
    write_openwepp_per_hill_pass_fixture(&input_dir.join("H2.pass.parquet"));
    write_openwepp_per_hill_wat_fixture(&input_dir.join("H2.wat.parquet"));
    let output_path = base.join("totalwatsed3.parquet");

    let output = Command::new(env!("CARGO_BIN_EXE_openwepp-cli-totalwatsed3"))
        .arg("--input-dir")
        .arg(&input_dir)
        .arg("--output")
        .arg(&output_path)
        .output()
        .expect("totalwatsed3 CLI process should run");

    assert!(
        output.status.success(),
        "totalwatsed3 CLI should consume per-hillslope native files; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let batch = read_first_output_batch(&output_path);
    assert_eq!(batch.num_rows(), 1);

    assert!((f64_column(&batch, "Area") - 5_000.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "runvol") - 24.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "Runoff") - 4.8).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "latqcc") - 9.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "Lateral Flow") - 1.8).abs() <= 1.0e-12);

    if base.exists() {
        fs::remove_dir_all(base).expect("temp dir cleanup should succeed");
    }
}
