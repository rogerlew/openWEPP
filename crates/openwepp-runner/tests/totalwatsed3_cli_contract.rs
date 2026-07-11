use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use arrow_array::{Array, ArrayRef, Float64Array, Int8Array, Int16Array, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use openwepp_runner::{Totalwatsed3Config, Totalwatsed3Error, write_totalwatsed3};
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

fn write_record_batch_fixture(path: &Path, batch: &RecordBatch) {
    let fields = batch
        .schema()
        .fields()
        .iter()
        .map(|field| field.as_ref().clone())
        .collect::<Vec<_>>();
    write_fixture_parquet(path, Schema::new(fields), batch.columns().to_vec());
}

fn replace_fixture_column(path: &Path, name: &str, field: Field, column: ArrayRef) {
    let batch = read_first_output_batch(path);
    let index = batch
        .schema()
        .index_of(name)
        .expect("fixture column should exist");
    let mut fields = batch
        .schema()
        .fields()
        .iter()
        .map(|item| item.as_ref().clone())
        .collect::<Vec<_>>();
    let mut columns = batch.columns().to_vec();
    fields[index] = field;
    columns[index] = column;
    write_fixture_parquet(path, Schema::new(fields), columns);
}

fn remove_fixture_column(path: &Path, name: &str) {
    let batch = read_first_output_batch(path);
    let index = batch
        .schema()
        .index_of(name)
        .expect("fixture column should exist");
    let mut fields = batch
        .schema()
        .fields()
        .iter()
        .map(|item| item.as_ref().clone())
        .collect::<Vec<_>>();
    let mut columns = batch.columns().to_vec();
    fields.remove(index);
    columns.remove(index);
    write_fixture_parquet(path, Schema::new(fields), columns);
}

fn append_fixture_column(path: &Path, field: Field, column: ArrayRef) {
    let batch = read_first_output_batch(path);
    let mut fields = batch
        .schema()
        .fields()
        .iter()
        .map(|item| item.as_ref().clone())
        .collect::<Vec<_>>();
    let mut columns = batch.columns().to_vec();
    fields.push(field);
    columns.push(column);
    write_fixture_parquet(path, Schema::new(fields), columns);
}

fn rename_fixture_column(path: &Path, old: &str, new: &str) {
    let batch = read_first_output_batch(path);
    let index = batch
        .schema()
        .index_of(old)
        .expect("fixture column should exist");
    let data_type = batch.schema().field(index).data_type().clone();
    replace_fixture_column(
        path,
        old,
        Field::new(new, data_type, true),
        Arc::clone(batch.column(index)),
    );
}

fn empty_fixture(path: &Path) {
    let batch = read_first_output_batch(path);
    let empty = RecordBatch::new_empty(batch.schema());
    write_record_batch_fixture(path, &empty);
}

fn direct_config(
    base: &Path,
    pass_paths: Vec<PathBuf>,
    wat_paths: Vec<PathBuf>,
) -> Totalwatsed3Config {
    Totalwatsed3Config {
        pass_paths,
        wat_paths,
        soil_paths: Vec::new(),
        element_paths: Vec::new(),
        output_path: base.join("direct-totalwatsed3.parquet"),
    }
}

fn direct_error(base: &Path, pass: &Path, wat: &Path) -> Totalwatsed3Error {
    write_totalwatsed3(&direct_config(
        base,
        vec![pass.to_path_buf()],
        vec![wat.to_path_buf()],
    ))
    .expect_err("mutated fixture should fail closed")
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

fn write_two_day_oracle_pass_fixture(path: &Path) {
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
            Arc::new(Int32Array::from(vec![1, 2, 1, 2])),
            Arc::new(Int16Array::from(vec![2004; 4])),
            Arc::new(Int32Array::from(vec![1, 1, 2, 2])),
            Arc::new(Int16Array::from(vec![1, 1, 2, 2])),
            Arc::new(Int8Array::from(vec![1; 4])),
            Arc::new(Int8Array::from(vec![1, 1, 2, 2])),
            Arc::new(Int16Array::from(vec![2004; 4])),
            Arc::new(Float64Array::from(vec![7.0, 13.0, 11.0, 17.0])),
            Arc::new(Float64Array::from(vec![1.0, 2.0, 4.0, 6.0])),
            Arc::new(Float64Array::from(vec![17.0, 19.0, 23.0, 29.0])),
            Arc::new(Float64Array::from(vec![23.0, 29.0, 31.0, 37.0])),
            Arc::new(Float64Array::from(vec![1.0, 2.0, 3.0, 4.0])),
            Arc::new(Float64Array::from(vec![2.0, 3.0, 5.0, 6.0])),
            Arc::new(Float64Array::from(vec![3.0, 5.0, 7.0, 8.0])),
            Arc::new(Float64Array::from(vec![4.0, 7.0, 11.0, 12.0])),
            Arc::new(Float64Array::from(vec![5.0, 11.0, 13.0, 14.0])),
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

fn write_two_day_oracle_wat_fixture(path: &Path) {
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
        Field::new("Interception", DataType::Float64, true),
    ]);
    write_fixture_parquet(
        path,
        Schema::new(fields),
        vec![
            Arc::new(Int32Array::from(vec![1, 1, 2, 1, 1, 2])),
            Arc::new(Int16Array::from(vec![1, 2, 1, 1, 2, 1])),
            Arc::new(Int16Array::from(vec![2004; 6])),
            Arc::new(Int32Array::from(vec![1, 1, 1, 2, 2, 2])),
            Arc::new(Int16Array::from(vec![1, 1, 1, 2, 2, 2])),
            Arc::new(Int8Array::from(vec![1; 6])),
            Arc::new(Int8Array::from(vec![1, 1, 1, 2, 2, 2])),
            Arc::new(Int16Array::from(vec![2004; 6])),
            Arc::new(Int16Array::from(vec![1, 2, 1, 1, 2, 1])),
            Arc::new(Float64Array::from(vec![50.0, 60.0, 70.0, 40.0, 55.0, 65.0])),
            Arc::new(Float64Array::from(vec![41.0, 53.0, 67.0, 43.0, 59.0, 71.0])),
            Arc::new(Float64Array::from(vec![
                101.0, 103.0, 107.0, 109.0, 113.0, 127.0,
            ])),
            Arc::new(Float64Array::from(vec![1.0, 2.0, 4.0, 1.5, 2.5, 4.5])),
            Arc::new(Float64Array::from(vec![0.5, 1.0, 1.5, 0.7, 1.2, 1.8])),
            Arc::new(Float64Array::from(vec![0.2, 0.4, 0.8, 0.3, 0.5, 0.9])),
            Arc::new(Float64Array::from(vec![3.0, 5.0, 7.0, 4.0, 6.0, 8.0])),
            Arc::new(Float64Array::from(vec![11.0, 13.0, 17.0, 19.0, 23.0, 29.0])),
            Arc::new(Float64Array::from(vec![2.0, 3.0, 5.0, 7.0, 11.0, 13.0])),
            Arc::new(Float64Array::from(vec![100.0, 2.0, 3.0, 200.0, 3.0, 4.0])),
            Arc::new(Float64Array::from(vec![
                100.0, 110.0, 130.0, 90.0, 100.0, 115.0,
            ])),
            Arc::new(Float64Array::from(vec![5.0, 7.0, 11.0, 4.0, 6.0, 9.0])),
            Arc::new(Float64Array::from(vec![2.0, 4.0, 8.0, 1.0, 3.0, 6.0])),
            Arc::new(Float64Array::from(vec![13.0, 17.0, 19.0, 23.0, 29.0, 31.0])),
            Arc::new(Float64Array::from(vec![0.1, 0.3, 0.7, 0.2, 0.5, 0.9])),
            Arc::new(Float64Array::from(vec![0.4, 0.8, 1.6, 0.6, 1.1, 1.9])),
            Arc::new(Float64Array::from(vec![
                100.0, 300.0, 600.0, 200.0, 500.0, 800.0,
            ])),
            Arc::new(Float64Array::from(vec![
                140.0, 150.0, 170.0, 125.0, 135.0, 155.0,
            ])),
            Arc::new(Float64Array::from(vec![
                900.0, 1_000.0, 1_100.0, 920.0, 1_020.0, 1_120.0,
            ])),
            Arc::new(Float64Array::from(vec![
                300.0, 320.0, 350.0, 305.0, 325.0, 355.0,
            ])),
            Arc::new(Float64Array::from(vec![
                210.0, 230.0, 260.0, 215.0, 235.0, 265.0,
            ])),
            Arc::new(Float64Array::from(vec![
                80.0, 90.0, 105.0, 82.0, 92.0, 107.0,
            ])),
            Arc::new(Float64Array::from(vec![1.0, 1.5, 2.5, 0.8, 1.4, 2.2])),
            Arc::new(Float64Array::from(vec![0.4, 0.7, 1.1, 0.5, 0.8, 1.3])),
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

fn write_soil_fixture(path: &Path) {
    let fields = vec![
        Field::new("wepp_id", DataType::Int32, true),
        Field::new("OFE", DataType::Int16, true),
        Field::new("year", DataType::Int16, true),
        Field::new("julian", DataType::Int16, true),
        Field::new("month", DataType::Int8, true),
        Field::new("day_of_month", DataType::Int8, true),
        Field::new("water_year", DataType::Int16, true),
        Field::new("TSMF", DataType::Float64, true),
    ];
    write_fixture_parquet(
        path,
        Schema::new(fields),
        vec![
            Arc::new(Int32Array::from(vec![1, 1, 2, 99])),
            Arc::new(Int16Array::from(vec![1, 2, 1, 1])),
            Arc::new(Int16Array::from(vec![2004; 4])),
            Arc::new(Int16Array::from(vec![1; 4])),
            Arc::new(Int8Array::from(vec![1; 4])),
            Arc::new(Int8Array::from(vec![1; 4])),
            Arc::new(Int16Array::from(vec![2004; 4])),
            Arc::new(Float64Array::from(vec![0.1, 0.3, 0.9, 99.0])),
        ],
    );
}

fn write_element_fixture(path: &Path) {
    let fields = vec![
        Field::new("wepp_id", DataType::Int32, true),
        Field::new("ofe_id", DataType::Int16, true),
        Field::new("year", DataType::Int16, true),
        Field::new("julian", DataType::Int16, true),
        Field::new("month", DataType::Int8, true),
        Field::new("day_of_month", DataType::Int8, true),
        Field::new("water_year", DataType::Int16, true),
        Field::new("QRain", DataType::Float64, true),
        Field::new("QSnow", DataType::Float64, true),
    ];
    write_fixture_parquet(
        path,
        Schema::new(fields),
        vec![
            Arc::new(Int32Array::from(vec![1, 1, 2, 99])),
            Arc::new(Int16Array::from(vec![1, 2, 1, 1])),
            Arc::new(Int16Array::from(vec![2004; 4])),
            Arc::new(Int16Array::from(vec![1; 4])),
            Arc::new(Int8Array::from(vec![1; 4])),
            Arc::new(Int8Array::from(vec![1; 4])),
            Arc::new(Int16Array::from(vec![2004; 4])),
            Arc::new(Float64Array::from(vec![1.0, 3.0, 9.0, 99.0])),
            Arc::new(Float64Array::from(vec![2.0, 4.0, 10.0, 99.0])),
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
    f64_at(batch, name, 0)
}

fn f64_at(batch: &RecordBatch, name: &str, row: usize) -> f64 {
    let schema = batch.schema();
    let column = batch
        .column(schema.index_of(name).expect("column should exist"))
        .as_any()
        .downcast_ref::<Float64Array>()
        .expect("column should be Float64");
    assert!(!column.is_null(row), "{name} row {row} should not be null");
    column.value(row)
}

fn i16_at(batch: &RecordBatch, name: &str, row: usize) -> i16 {
    let schema = batch.schema();
    batch
        .column(schema.index_of(name).expect("column should exist"))
        .as_any()
        .downcast_ref::<Int16Array>()
        .expect("column should be Int16")
        .value(row)
}

fn i32_at(batch: &RecordBatch, name: &str, row: usize) -> i32 {
    let schema = batch.schema();
    batch
        .column(schema.index_of(name).expect("column should exist"))
        .as_any()
        .downcast_ref::<Int32Array>()
        .expect("column should be Int32")
        .value(row)
}

fn weighted_depth(values: &[f64; 3], areas: &[f64; 3]) -> f64 {
    values
        .iter()
        .zip(areas)
        .map(|(value, area)| value * area)
        .sum::<f64>()
        / areas.iter().sum::<f64>()
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

#[test]
fn totalwatsed3_cli_help_and_argument_errors_preserve_cli_contract() {
    let binary = env!("CARGO_BIN_EXE_openwepp-cli-totalwatsed3");

    let help = Command::new(binary)
        .arg("--help")
        .output()
        .expect("totalwatsed3 help process should run");
    assert!(help.status.success(), "--help should succeed");
    assert!(
        String::from_utf8_lossy(&help.stdout).contains("--input-dir <interchange-dir>"),
        "help should preserve the required input directory synopsis"
    );

    let missing_value = Command::new(binary)
        .arg("--input-dir")
        .output()
        .expect("totalwatsed3 missing-value process should run");
    assert!(
        !missing_value.status.success(),
        "missing values should fail closed"
    );
    assert!(
        String::from_utf8_lossy(&missing_value.stderr)
            .contains("CLITW3-E-001 missing value for --input-dir"),
        "missing --input-dir value should retain its typed error"
    );

    let unknown = Command::new(binary)
        .arg("--unknown")
        .output()
        .expect("totalwatsed3 unknown-argument process should run");
    assert!(
        !unknown.status.success(),
        "unknown arguments should fail closed"
    );
    assert!(
        String::from_utf8_lossy(&unknown.stderr)
            .contains("CLITW3-E-001 unrecognized argument --unknown"),
        "unknown arguments should retain their typed error"
    );
}

#[test]
fn totalwatsed3_cli_explicit_relative_and_absolute_inputs_override_default_discovery() {
    let base = unique_temp_dir("totalwatsed3_explicit_input_precedence");
    let input_dir = base.join("interchange");
    fs::create_dir_all(&input_dir).expect("input dir should be created");
    fs::write(
        input_dir.join("H.pass.parquet"),
        "invalid default PASS fixture",
    )
    .expect("invalid default PASS fixture should be written");
    fs::write(
        input_dir.join("H.wat.parquet"),
        "invalid default WAT fixture",
    )
    .expect("invalid default WAT fixture should be written");

    let relative_pass = input_dir.join("selected.pass.parquet");
    write_pass_fixture(&relative_pass);
    let absolute_wat = base.join("external").join("selected.wat.parquet");
    write_wat_fixture(&absolute_wat);
    let output_path = base.join("totalwatsed3.parquet");

    let output = Command::new(env!("CARGO_BIN_EXE_openwepp-cli-totalwatsed3"))
        .arg("--input-dir")
        .arg(&input_dir)
        .arg("--output")
        .arg(&output_path)
        .arg("--pass")
        .arg("selected.pass.parquet")
        .arg("--wat")
        .arg(&absolute_wat)
        .output()
        .expect("totalwatsed3 CLI process should run");

    assert!(
        output.status.success(),
        "explicit inputs should override invalid discovered defaults; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let batch = read_first_output_batch(&output_path);
    assert_eq!(batch.num_rows(), 1);
    assert!((f64_column(&batch, "runvol") - 12.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "Q") - 181.5).abs() <= 1.0e-12);

    if base.exists() {
        fs::remove_dir_all(base).expect("temp dir cleanup should succeed");
    }
}

#[test]
fn totalwatsed3_cli_rejects_missing_explicit_optional_inputs() {
    let base = unique_temp_dir("totalwatsed3_missing_explicit_optional_input");
    let input_dir = base.join("interchange");
    fs::create_dir_all(&input_dir).expect("input dir should be created");
    write_pass_fixture(&input_dir.join("H.pass.parquet"));
    write_wat_fixture(&input_dir.join("H.wat.parquet"));

    for (flag, missing_name) in [
        ("--soil", "missing.soil.parquet"),
        ("--element", "missing.element.parquet"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_openwepp-cli-totalwatsed3"))
            .arg("--input-dir")
            .arg(&input_dir)
            .arg("--output")
            .arg(base.join(format!("{missing_name}.output.parquet")))
            .arg(flag)
            .arg(missing_name)
            .output()
            .expect("totalwatsed3 CLI process should run");

        assert!(
            !output.status.success(),
            "missing explicit {flag} input should fail closed"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        let expected = format!(
            "CLITW3-E-006 optional input was explicitly configured but does not exist: {}",
            input_dir.join(missing_name).display()
        );
        assert!(
            stderr.contains(&expected),
            "missing explicit {flag} input should retain its typed error, observed: {stderr}"
        );
    }

    if base.exists() {
        fs::remove_dir_all(base).expect("temp dir cleanup should succeed");
    }
}

#[test]
fn totalwatsed3_error_codes_display_and_sources_are_stable() {
    let path = PathBuf::from("fixture.parquet");
    let errors = [
        (
            Totalwatsed3Error::MissingInput {
                role: "PASS",
                path: path.clone(),
            },
            "TW3-E-001",
            "missing required PASS input",
        ),
        (
            Totalwatsed3Error::Open {
                path: path.clone(),
                detail: "open detail".to_string(),
            },
            "TW3-E-002",
            "failed opening parquet",
        ),
        (
            Totalwatsed3Error::Read {
                path: path.clone(),
                detail: "read detail".to_string(),
            },
            "TW3-E-003",
            "failed reading parquet",
        ),
        (
            Totalwatsed3Error::MissingColumn {
                path: path.clone(),
                column: "P".to_string(),
            },
            "TW3-E-004",
            "missing required column P",
        ),
        (
            Totalwatsed3Error::UnsupportedColumnType {
                path: path.clone(),
                column: "P".to_string(),
            },
            "TW3-E-005",
            "unsupported type for column P",
        ),
        (
            Totalwatsed3Error::NullValue {
                path: path.clone(),
                column: "P".to_string(),
                row_index: 3,
            },
            "TW3-E-006",
            "null value in column P at row 3",
        ),
        (
            Totalwatsed3Error::InvalidValue {
                path: path.clone(),
                column: "P".to_string(),
                row_index: 3,
                value: f64::NAN,
            },
            "TW3-E-007",
            "invalid value NaN in column P at row 3",
        ),
        (
            Totalwatsed3Error::EmptyWatInput { path: path.clone() },
            "TW3-E-008",
            "has no rows to aggregate",
        ),
        (
            Totalwatsed3Error::Write {
                path,
                detail: "write detail".to_string(),
            },
            "TW3-E-009",
            "failed writing totalwatsed3 parquet",
        ),
    ];
    for (error, code, message) in errors {
        assert_eq!(error.code(), code);
        assert!(error.to_string().contains(message), "error={error}");
        assert!(std::error::Error::source(&error).is_none());
    }
}

#[test]
fn direct_writer_closes_required_empty_existing_read_and_write_paths() {
    let base = unique_temp_dir("totalwatsed3_direct_path_matrix");
    fs::create_dir_all(&base).expect("base should exist");
    let no_pass = direct_config(&base, Vec::new(), vec![base.join("wat")]);
    assert!(
        matches!(write_totalwatsed3(&no_pass), Err(Totalwatsed3Error::MissingInput { role: "PASS", ref path }) if path == Path::new("<none>"))
    );

    let pass = base.join("valid.pass.parquet");
    let wat = base.join("valid.wat.parquet");
    write_pass_fixture(&pass);
    write_wat_fixture(&wat);
    let no_wat = direct_config(&base, vec![pass.clone()], Vec::new());
    assert!(
        matches!(write_totalwatsed3(&no_wat), Err(Totalwatsed3Error::MissingInput { role: "WAT", ref path }) if path == Path::new("<none>"))
    );

    let mut missing_optional = direct_config(&base, vec![pass.clone()], vec![wat.clone()]);
    missing_optional
        .soil_paths
        .push(base.join("missing.soil.parquet"));
    assert!(matches!(
        write_totalwatsed3(&missing_optional),
        Err(Totalwatsed3Error::MissingInput { role: "soil", .. })
    ));

    let empty_wat = base.join("empty.wat.parquet");
    write_wat_fixture(&empty_wat);
    empty_fixture(&empty_wat);
    let empty = direct_config(&base, vec![pass.clone()], vec![empty_wat.clone()]);
    assert!(
        matches!(write_totalwatsed3(&empty), Err(Totalwatsed3Error::EmptyWatInput { path }) if path == empty_wat)
    );

    let invalid = base.join("invalid.pass.parquet");
    fs::write(&invalid, "not parquet").expect("invalid parquet should write");
    let unreadable = direct_config(&base, vec![invalid.clone()], vec![wat.clone()]);
    assert!(
        matches!(write_totalwatsed3(&unreadable), Err(Totalwatsed3Error::Read { path, .. }) if path == invalid)
    );

    let output_directory = base.join("output-is-directory");
    fs::create_dir_all(&output_directory).expect("output directory should exist");
    let mut write_failure = direct_config(&base, vec![pass], vec![wat]);
    write_failure.output_path = output_directory.clone();
    assert!(
        matches!(write_totalwatsed3(&write_failure), Err(Totalwatsed3Error::Write { path, .. }) if path == output_directory)
    );
    fs::remove_dir_all(base).expect("cleanup should succeed");
}

#[test]
fn optional_soil_element_outputs_are_independently_area_reconstructed() {
    let base = unique_temp_dir("totalwatsed3_optional_reconstruction");
    let pass = base.join("H.pass.parquet");
    let wat = base.join("H.wat.parquet");
    let soil = base.join("H.soil.parquet");
    let element = base.join("H.element.parquet");
    write_pass_fixture(&pass);
    write_wat_fixture(&wat);
    write_soil_fixture(&soil);
    write_element_fixture(&element);
    let mut config = direct_config(&base, vec![pass], vec![wat]);
    config.soil_paths.push(soil);
    config.element_paths.push(element);
    let summary = write_totalwatsed3(&config).expect("optional aggregation should succeed");
    assert_eq!(summary.rows_written, 1);
    assert_eq!(summary.output_path, config.output_path);
    let batch = read_first_output_batch(&config.output_path);

    let area = 1_000.0 + 1_000.0 + 500.0;
    let reconstructed_p_mm = (10.0 * 1_000.0 + 20.0 * 1_000.0 + 30.0 * 500.0) / area;
    let reconstructed_q_m3 = (99.0 * 1_000.0 + 77.0 * 1_000.0 + 11.0 * 500.0) / 1_000.0;
    let outlet_latq_m3 = (2.0 * 1_000.0 + 5.0 * 500.0) / 1_000.0;
    let all_ofe_qofe_m3 = (3.0 * 1_000.0 + 4.0 * 1_000.0 + 6.0 * 500.0) / 1_000.0;
    assert!((f64_column(&batch, "Area") - area).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "Precipitation") - reconstructed_p_mm).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "Q") - reconstructed_q_m3).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "latqcc") - outlet_latq_m3).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "QOFE") - all_ofe_qofe_m3).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "runvol") - 12.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "runvol") - reconstructed_q_m3).abs() > 100.0);
    assert!((f64_column(&batch, "TSMF") - 0.34).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "QRain") - 3.4).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "QSnow") - 4.4).abs() <= 1.0e-12);
    fs::remove_dir_all(base).expect("cleanup should succeed");
}

#[test]
fn optional_join_partial_coverage_uses_last_duplicate_wat_key_area() {
    let base = unique_temp_dir("totalwatsed3_optional_partial_duplicate_key");
    let pass = base.join("H.pass.parquet");
    let wat = base.join("H.wat.parquet");
    let soil = base.join("H.soil.parquet");
    let element = base.join("H.element.parquet");
    write_pass_fixture(&pass);
    write_wat_fixture(&wat);
    replace_fixture_column(
        &wat,
        "ofe_id",
        Field::new("ofe_id", DataType::Int16, true),
        Arc::new(Int16Array::from(vec![1, 1, 1])),
    );
    replace_fixture_column(
        &wat,
        "Area",
        Field::new("Area", DataType::Float64, true),
        Arc::new(Float64Array::from(vec![400.0, 1_600.0, 500.0])),
    );
    write_soil_fixture(&soil);
    write_element_fixture(&element);
    let mut config = direct_config(&base, vec![pass], vec![wat]);
    config.soil_paths.push(soil);
    config.element_paths.push(element);
    write_totalwatsed3(&config).expect("partial optional join should aggregate");
    let batch = read_first_output_batch(&config.output_path);

    let matched_area = 1_600.0 + 500.0;
    let expected_tsmf = (0.1 * 1_600.0 + 0.9 * 500.0) / matched_area;
    let expected_qrain = (1.0 * 1_600.0 + 9.0 * 500.0) / matched_area;
    let expected_qsnow = (2.0 * 1_600.0 + 10.0 * 500.0) / matched_area;
    assert!((f64_column(&batch, "TSMF") - expected_tsmf).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "QRain") - expected_qrain).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "QSnow") - expected_qsnow).abs() <= 1.0e-12);

    let first_duplicate_area_alias = (0.1 * 400.0 + 0.9 * 500.0) / (400.0 + 500.0);
    let total_wat_area_alias = (0.1 * 1_600.0 + 0.9 * 500.0) / 2_500.0;
    assert!((expected_tsmf - first_duplicate_area_alias).abs() > 0.2);
    assert!((expected_tsmf - total_wat_area_alias).abs() > 0.04);
    fs::remove_dir_all(base).expect("cleanup should succeed");
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "literal source operands remain adjacent to their independent assertions"
)]
fn two_day_water_storage_and_sediment_oracle_rejects_wrong_aliases() {
    let base = unique_temp_dir("totalwatsed3_two_day_independent_oracle");
    let pass = base.join("H.pass.parquet");
    let wat = base.join("H.wat.parquet");
    write_two_day_oracle_pass_fixture(&pass);
    write_two_day_oracle_wat_fixture(&wat);
    let config = direct_config(&base, vec![pass], vec![wat]);
    let summary = write_totalwatsed3(&config).expect("two-day oracle input should aggregate");
    assert_eq!(summary.rows_written, 2);
    let batch = read_first_output_batch(&config.output_path);
    assert_eq!(batch.num_rows(), 2);
    assert_eq!(
        [
            (
                i16_at(&batch, "year", 0),
                i32_at(&batch, "sim_day_index", 0),
                i16_at(&batch, "julian", 0)
            ),
            (
                i16_at(&batch, "year", 1),
                i32_at(&batch, "sim_day_index", 1),
                i16_at(&batch, "julian", 1)
            ),
        ],
        [(2004, 1, 1), (2004, 2, 2)]
    );
    let output_schema = batch.schema();
    for name in [
        "runvol",
        "sbrunv",
        "tdet",
        "tdep",
        "seddep_1",
        "seddep_2",
        "seddep_3",
        "seddep_4",
        "seddep_5",
        "sed_del",
        "sed_vol_conc",
        "Area",
        "P",
        "RM",
        "Q",
        "Dp",
        "latqcc",
        "QOFE",
        "Ep",
        "Es",
        "Er",
        "UpStrmQ",
        "SubRIn",
        "Total-Soil Water",
        "SoilWaterTotal",
        "ProfileDepth",
        "ProfilePorosityCap",
        "ProfileFCStore",
        "ProfileWPStore",
        "Interception",
        "InterceptionStorage",
        "TSMF",
        "frozwt",
        "Snow-Water",
        "QRain",
        "QSnow",
        "Tile",
        "Irr",
        "Precipitation",
        "Rain+Melt",
        "Percolation",
        "Lateral Flow",
        "Runoff",
        "Transpiration",
        "Evaporation",
        "ET",
        "Interception",
        "Baseflow",
    ] {
        let field = output_schema
            .field_with_name(name)
            .expect("oracle field should exist");
        assert_eq!(field.data_type(), &DataType::Float64, "{name} type drifted");
        assert!(field.is_nullable(), "{name} nullability drifted");
        for row in 0..2 {
            assert!(
                f64_at(&batch, name, row).is_finite(),
                "{name} row {row} should be finite"
            );
        }
    }

    let day_1_areas = [100.0, 300.0, 600.0];
    let day_2_areas = [200.0, 500.0, 800.0];
    let day_1_area = day_1_areas.iter().sum::<f64>();
    let day_2_area = day_2_areas.iter().sum::<f64>();
    let day_1_runvol = 7.0 + 13.0;
    let day_2_runvol = 11.0 + 17.0;
    let day_1_runoff = day_1_runvol / day_1_area * 1_000.0;
    let day_2_runoff = day_2_runvol / day_2_area * 1_000.0;
    let day_1_q = weighted_depth(&[101.0, 103.0, 107.0], &day_1_areas) * day_1_area / 1_000.0;
    let day_2_q = weighted_depth(&[109.0, 113.0, 127.0], &day_2_areas) * day_2_area / 1_000.0;
    let day_1_qofe = weighted_depth(&[13.0, 17.0, 19.0], &day_1_areas) * day_1_area / 1_000.0;
    let day_2_qofe = weighted_depth(&[23.0, 29.0, 31.0], &day_2_areas) * day_2_area / 1_000.0;
    let day_1_lateral = (2.0 * 300.0 + 3.0 * 600.0) / day_1_area;
    let day_2_lateral = (3.0 * 500.0 + 4.0 * 800.0) / day_2_area;

    let day_fields = [
        (
            0,
            day_1_areas,
            [50.0, 60.0, 70.0],
            [41.0, 53.0, 67.0],
            [3.0, 5.0, 7.0],
            [1.0, 2.0, 4.0],
            [0.5, 1.0, 1.5],
            [0.2, 0.4, 0.8],
            [11.0, 13.0, 17.0],
            [2.0, 3.0, 5.0],
            [100.0, 110.0, 130.0],
            [140.0, 150.0, 170.0],
            [900.0, 1_000.0, 1_100.0],
            [300.0, 320.0, 350.0],
            [210.0, 230.0, 260.0],
            [80.0, 90.0, 105.0],
            [1.0, 1.5, 2.5],
            [5.0, 7.0, 11.0],
            [2.0, 4.0, 8.0],
            [0.1, 0.3, 0.7],
            [0.4, 0.8, 1.6],
            [0.4, 0.7, 1.1],
        ),
        (
            1,
            day_2_areas,
            [40.0, 55.0, 65.0],
            [43.0, 59.0, 71.0],
            [4.0, 6.0, 8.0],
            [1.5, 2.5, 4.5],
            [0.7, 1.2, 1.8],
            [0.3, 0.5, 0.9],
            [19.0, 23.0, 29.0],
            [7.0, 11.0, 13.0],
            [90.0, 100.0, 115.0],
            [125.0, 135.0, 155.0],
            [920.0, 1_020.0, 1_120.0],
            [305.0, 325.0, 355.0],
            [215.0, 235.0, 265.0],
            [82.0, 92.0, 107.0],
            [0.8, 1.4, 2.2],
            [4.0, 6.0, 9.0],
            [1.0, 3.0, 6.0],
            [0.2, 0.5, 0.9],
            [0.6, 1.1, 1.9],
            [0.5, 0.8, 1.3],
        ),
    ];
    for (
        row,
        areas,
        precipitation,
        rain_melt,
        percolation,
        transpiration,
        soil_evaporation,
        residue_evaporation,
        upstream_q,
        subsurface_runon,
        total_soil_water,
        soil_water_total,
        profile_depth,
        profile_porosity,
        profile_fc,
        profile_wp,
        interception_storage,
        frozen_water,
        snow_water,
        tile,
        irrigation,
        interception,
    ) in day_fields
    {
        let area = areas.iter().sum::<f64>();
        for (volume_name, depth_name, values) in [
            ("P", "Precipitation", precipitation),
            ("RM", "Rain+Melt", rain_melt),
            ("Dp", "Percolation", percolation),
            ("Ep", "Transpiration", transpiration),
        ] {
            let expected_depth = weighted_depth(&values, &areas);
            let expected_volume = expected_depth * area / 1_000.0;
            assert!((f64_at(&batch, depth_name, row) - expected_depth).abs() <= 1.0e-12);
            assert!((f64_at(&batch, volume_name, row) - expected_volume).abs() <= 1.0e-12);
        }
        let soil_evaporation_depth = weighted_depth(&soil_evaporation, &areas);
        let residue_evaporation_depth = weighted_depth(&residue_evaporation, &areas);
        let transpiration_depth = weighted_depth(&transpiration, &areas);
        assert!(
            (f64_at(&batch, "Es", row) - soil_evaporation_depth * area / 1_000.0).abs() <= 1.0e-12
        );
        assert!(
            (f64_at(&batch, "Er", row) - residue_evaporation_depth * area / 1_000.0).abs()
                <= 1.0e-12
        );
        assert!(
            (f64_at(&batch, "Evaporation", row)
                - (soil_evaporation_depth + residue_evaporation_depth))
                .abs()
                <= 1.0e-12
        );
        assert!(
            (f64_at(&batch, "ET", row)
                - (transpiration_depth + soil_evaporation_depth + residue_evaporation_depth))
                .abs()
                <= 1.0e-12
        );
        for (name, values) in [
            ("UpStrmQ", upstream_q),
            ("SubRIn", subsurface_runon),
            ("Total-Soil Water", total_soil_water),
            ("SoilWaterTotal", soil_water_total),
            ("ProfileDepth", profile_depth),
            ("ProfilePorosityCap", profile_porosity),
            ("ProfileFCStore", profile_fc),
            ("ProfileWPStore", profile_wp),
            ("InterceptionStorage", interception_storage),
            ("frozwt", frozen_water),
            ("Snow-Water", snow_water),
            ("Tile", tile),
            ("Irr", irrigation),
            ("Interception", interception),
        ] {
            assert!((f64_at(&batch, name, row) - weighted_depth(&values, &areas)).abs() <= 1.0e-12);
        }
        assert!(
            (f64_at(&batch, "SoilWaterTotal", row) - f64_at(&batch, "Total-Soil Water", row)).abs()
                > 20.0
        );
        assert!(
            (f64_at(&batch, "InterceptionStorage", row) - f64_at(&batch, "Interception", row))
                .abs()
                > 0.5
        );
        for name in ["TSMF", "QRain", "QSnow", "Baseflow"] {
            assert!(f64_at(&batch, name, row).abs() <= f64::EPSILON);
        }
    }

    for (row, area, runvol, runoff, q, qofe, lateral) in [
        (
            0,
            day_1_area,
            day_1_runvol,
            day_1_runoff,
            day_1_q,
            day_1_qofe,
            day_1_lateral,
        ),
        (
            1,
            day_2_area,
            day_2_runvol,
            day_2_runoff,
            day_2_q,
            day_2_qofe,
            day_2_lateral,
        ),
    ] {
        assert!((f64_at(&batch, "Area", row) - area).abs() <= 1.0e-12);
        assert!((f64_at(&batch, "runvol", row) - runvol).abs() <= 1.0e-12);
        assert!((f64_at(&batch, "Runoff", row) - runoff).abs() <= 1.0e-12);
        assert!((f64_at(&batch, "Q", row) - q).abs() <= 1.0e-12);
        assert!((f64_at(&batch, "QOFE", row) - qofe).abs() <= 1.0e-12);
        assert!((f64_at(&batch, "Lateral Flow", row) - lateral).abs() <= 1.0e-12);
        assert!((runvol - q).abs() > 1.0);
        assert!((runvol - qofe).abs() > 1.0);
        assert!((lateral - weighted_depth(&[100.0, 2.0, 3.0], &day_1_areas)).abs() > 1.0);
    }

    let day_1_class_masses = [33.0, 53.0, 86.0, 119.0, 178.0];
    let day_2_class_masses = [101.0, 157.0, 213.0, 325.0, 381.0];
    let densities = [2_600.0, 2_650.0, 1_800.0, 1_600.0, 2_650.0];
    for (row, class_masses, runvol) in [
        (0, day_1_class_masses, day_1_runvol),
        (1, day_2_class_masses, day_2_runvol),
    ] {
        for (class_index, expected_mass) in class_masses.iter().enumerate() {
            let name = format!("seddep_{}", class_index + 1);
            assert!((f64_at(&batch, &name, row) - expected_mass).abs() <= 1.0e-12);
        }
        let sediment_delivery = class_masses.iter().sum::<f64>();
        let solids_volume = class_masses
            .iter()
            .zip(densities)
            .map(|(mass, density)| mass / density)
            .sum::<f64>();
        let expected_volume_concentration = solids_volume / runvol;
        assert!((f64_at(&batch, "sed_del", row) - sediment_delivery).abs() <= 1.0e-12);
        assert!(
            (f64_at(&batch, "sed_vol_conc", row) - expected_volume_concentration).abs() <= 1.0e-12
        );
        let concentration_sum_alias = if row == 0 { 43.0 } else { 83.0 };
        let common_density_alias = sediment_delivery / 2_650.0 / runvol;
        assert!((sediment_delivery - concentration_sum_alias).abs() > 100.0);
        assert!((expected_volume_concentration - common_density_alias).abs() > 1.0e-4);
        assert!((expected_volume_concentration - sediment_delivery / runvol).abs() > 1.0);
    }
    for (name, expected) in [
        ("sbrunv", [3.0, 10.0]),
        ("tdet", [36.0, 52.0]),
        ("tdep", [52.0, 68.0]),
    ] {
        for (row, value) in expected.into_iter().enumerate() {
            assert!((f64_at(&batch, name, row) - value).abs() <= 1.0e-12);
        }
    }

    let day_1_precipitation = weighted_depth(&[50.0, 60.0, 70.0], &day_1_areas);
    let day_2_precipitation = weighted_depth(&[40.0, 55.0, 65.0], &day_2_areas);
    let day_1_et = weighted_depth(&[1.0, 2.0, 4.0], &day_1_areas)
        + weighted_depth(&[0.5, 1.0, 1.5], &day_1_areas)
        + weighted_depth(&[0.2, 0.4, 0.8], &day_1_areas);
    let day_2_et = weighted_depth(&[1.5, 2.5, 4.5], &day_2_areas)
        + weighted_depth(&[0.7, 1.2, 1.8], &day_2_areas)
        + weighted_depth(&[0.3, 0.5, 0.9], &day_2_areas);
    let day_1_storage = weighted_depth(&[100.0, 110.0, 130.0], &day_1_areas)
        + weighted_depth(&[5.0, 7.0, 11.0], &day_1_areas)
        + weighted_depth(&[2.0, 4.0, 8.0], &day_1_areas);
    let day_2_storage = weighted_depth(&[90.0, 100.0, 115.0], &day_2_areas)
        + weighted_depth(&[4.0, 6.0, 9.0], &day_2_areas)
        + weighted_depth(&[1.0, 3.0, 6.0], &day_2_areas);
    let storage_delta = day_2_storage - day_1_storage;
    let expected_day_2_residual = day_2_precipitation
        - (day_2_runoff
            + day_2_lateral
            + day_2_et
            + weighted_depth(&[4.0, 6.0, 8.0], &day_2_areas)
            + weighted_depth(&[0.5, 0.8, 1.3], &day_2_areas))
        - storage_delta;
    let emitted_day_1_storage = f64_at(&batch, "Total-Soil Water", 0)
        + f64_at(&batch, "frozwt", 0)
        + f64_at(&batch, "Snow-Water", 0);
    let emitted_day_2_storage = f64_at(&batch, "Total-Soil Water", 1)
        + f64_at(&batch, "frozwt", 1)
        + f64_at(&batch, "Snow-Water", 1);
    let emitted_day_2_residual = f64_at(&batch, "Precipitation", 1)
        - (f64_at(&batch, "Runoff", 1)
            + f64_at(&batch, "Lateral Flow", 1)
            + f64_at(&batch, "ET", 1)
            + f64_at(&batch, "Percolation", 1)
            + f64_at(&batch, "Interception", 1))
        - (emitted_day_2_storage - emitted_day_1_storage);
    assert!((f64_at(&batch, "Precipitation", 0) - day_1_precipitation).abs() <= 1.0e-12);
    assert!((f64_at(&batch, "ET", 0) - day_1_et).abs() <= 1.0e-12);
    assert!((emitted_day_1_storage - day_1_storage).abs() <= 1.0e-12);
    assert!((emitted_day_2_storage - day_2_storage).abs() <= 1.0e-12);
    assert!((emitted_day_2_residual - expected_day_2_residual).abs() <= 1.0e-12);
    assert!(emitted_day_2_residual.abs() > 1.0);

    fs::remove_dir_all(base).expect("cleanup should succeed");
}

#[test]
fn alternate_column_names_and_optional_defaults_preserve_output_identity() {
    let base = unique_temp_dir("totalwatsed3_alternate_columns");
    let pass = base.join("selected.pass.parquet");
    let wat = base.join("selected.wat.parquet");
    write_pass_fixture(&pass);
    write_wat_fixture(&wat);
    rename_fixture_column(&pass, "sim_day_index", "day");
    rename_fixture_column(&wat, "sim_day_index", "day");
    remove_fixture_column(&wat, "ofe_id");
    rename_fixture_column(&wat, "Total-Soil Water", "Total-Soil");
    let config = direct_config(&base, vec![pass], vec![wat]);
    write_totalwatsed3(&config).expect("alternate columns should parse");
    let batch = read_first_output_batch(&config.output_path);
    assert!((f64_column(&batch, "runvol") - 12.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "TSMF") - 0.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "QRain") - 0.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "QSnow") - 0.0).abs() <= 1.0e-12);
    fs::remove_dir_all(base).expect("cleanup should succeed");
}

#[test]
fn required_schema_type_and_null_failures_have_exact_priority() {
    let base = unique_temp_dir("totalwatsed3_schema_type_null");
    let pass = base.join("H.pass.parquet");
    let wat = base.join("H.wat.parquet");
    write_pass_fixture(&pass);
    write_wat_fixture(&wat);

    remove_fixture_column(&pass, "runvol");
    assert!(
        matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::MissingColumn { column, .. } if column == "runvol")
    );

    write_pass_fixture(&pass);
    replace_fixture_column(
        &pass,
        "runvol",
        Field::new("runvol", DataType::Int32, true),
        Arc::new(Int32Array::from(vec![5, 7])),
    );
    assert!(
        matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::UnsupportedColumnType { column, .. } if column == "runvol")
    );

    write_pass_fixture(&pass);
    replace_fixture_column(
        &pass,
        "runvol",
        Field::new("runvol", DataType::Float64, true),
        Arc::new(Float64Array::from(vec![None, Some(7.0)])),
    );
    assert!(
        matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::NullValue { column, row_index: 0, .. } if column == "runvol")
    );

    write_pass_fixture(&pass);
    remove_fixture_column(&wat, "Total-Soil Water");
    assert!(
        matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::MissingColumn { column, .. } if column == "Total-Soil Water|Total-Soil")
    );

    write_wat_fixture(&wat);
    replace_fixture_column(
        &wat,
        "ofe_id",
        Field::new("ofe_id", DataType::Int32, true),
        Arc::new(Int32Array::from(vec![1, 2, 1])),
    );
    assert!(
        matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::UnsupportedColumnType { column, .. } if column == "ofe_id")
    );

    write_wat_fixture(&wat);
    replace_fixture_column(
        &wat,
        "month",
        Field::new("month", DataType::Int8, true),
        Arc::new(Int8Array::from(vec![None, Some(1), Some(1)])),
    );
    assert!(
        matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::NullValue { column, row_index: 0, .. } if column == "month")
    );

    write_wat_fixture(&wat);
    replace_fixture_column(
        &wat,
        "sim_day_index",
        Field::new("sim_day_index", DataType::Int32, true),
        Arc::new(Int32Array::from(vec![None, Some(1), Some(1)])),
    );
    assert!(
        matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::NullValue { column, row_index: 0, .. } if column == "sim_day_index")
    );

    write_wat_fixture(&wat);
    replace_fixture_column(
        &wat,
        "year",
        Field::new("year", DataType::Int16, true),
        Arc::new(Int16Array::from(vec![None, Some(2004), Some(2004)])),
    );
    assert!(
        matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::NullValue { column, row_index: 0, .. } if column == "year")
    );

    write_wat_fixture(&wat);
    remove_fixture_column(&pass, "sim_day_index");
    assert!(matches!(
        direct_error(&base, &pass, &wat),
        Totalwatsed3Error::MissingColumn { column, .. } if column == "sim_day_index|day"
    ));

    write_pass_fixture(&pass);
    let soil = base.join("missing-ofe.soil.parquet");
    write_soil_fixture(&soil);
    remove_fixture_column(&soil, "OFE");
    let mut soil_config = direct_config(&base, vec![pass.clone()], vec![wat.clone()]);
    soil_config.soil_paths.push(soil);
    assert!(matches!(
        write_totalwatsed3(&soil_config),
        Err(Totalwatsed3Error::MissingColumn { column, .. }) if column == "ofe_id|OFE"
    ));
    fs::remove_dir_all(base).expect("cleanup should succeed");
}

#[test]
fn every_pass_float_guard_rejects_nonfinite_and_negative_nonnegative_fields() {
    let pass_columns = [
        "runvol", "sbrunv", "tdet", "tdep", "sedcon_1", "sedcon_2", "sedcon_3", "sedcon_4",
        "sedcon_5",
    ];
    for (case_index, column) in pass_columns.into_iter().enumerate() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let base = unique_temp_dir(&format!("tw3_pass_nonfinite_{case_index}"));
            let pass = base.join("H.pass.parquet");
            let wat = base.join("H.wat.parquet");
            write_pass_fixture(&pass);
            write_wat_fixture(&wat);
            replace_fixture_column(
                &pass,
                column,
                Field::new(column, DataType::Float64, true),
                Arc::new(Float64Array::from(vec![value, 0.0])),
            );
            let expected_column = if column.starts_with("sedcon_") {
                "sedcon"
            } else {
                pass_columns[case_index]
            };
            assert!(
                matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::InvalidValue { ref column, row_index: 0, .. } if column == expected_column)
            );
            fs::remove_dir_all(base).expect("cleanup should succeed");
        }
    }

    for (case_index, column) in [
        "runvol", "sbrunv", "sedcon_1", "sedcon_2", "sedcon_3", "sedcon_4", "sedcon_5",
    ]
    .into_iter()
    .enumerate()
    {
        let base = unique_temp_dir(&format!("tw3_pass_negative_{case_index}"));
        let pass = base.join("H.pass.parquet");
        let wat = base.join("H.wat.parquet");
        write_pass_fixture(&pass);
        write_wat_fixture(&wat);
        replace_fixture_column(
            &pass,
            column,
            Field::new(column, DataType::Float64, true),
            Arc::new(Float64Array::from(vec![-1.0, 0.0])),
        );
        assert!(
            matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::InvalidValue { column: error_column, row_index: 0, .. } if error_column == column || error_column == "sedcon")
        );
        fs::remove_dir_all(base).expect("cleanup should succeed");
    }
}

#[test]
fn every_wat_float_family_rejects_nonfinite_and_area_rejects_nonpositive() {
    let columns = [
        "Area",
        "P",
        "RM",
        "Q",
        "Dp",
        "latqcc",
        "QOFE",
        "Ep",
        "Es",
        "Er",
        "UpStrmQ",
        "SubRIn",
        "Total-Soil Water",
        "SoilWaterTotal",
        "ProfileDepth",
        "ProfilePorosityCap",
        "ProfileFCStore",
        "ProfileWPStore",
        "Interception",
        "InterceptionStorage",
        "frozwt",
        "Snow-Water",
        "Tile",
        "Irr",
    ];
    for (case_index, column) in columns.into_iter().enumerate() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let base = unique_temp_dir(&format!("tw3_wat_nonfinite_{case_index}"));
            let pass = base.join("H.pass.parquet");
            let wat = base.join("H.wat.parquet");
            write_pass_fixture(&pass);
            write_wat_fixture(&wat);
            if column == "Interception" {
                append_fixture_column(
                    &wat,
                    Field::new(column, DataType::Float64, true),
                    Arc::new(Float64Array::from(vec![0.0, 1.0, 1.0])),
                );
            }
            replace_fixture_column(
                &wat,
                column,
                Field::new(column, DataType::Float64, true),
                Arc::new(Float64Array::from(vec![value, 1.0, 1.0])),
            );
            assert!(
                matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::InvalidValue { ref column, row_index: 0, .. } if column == columns[case_index])
            );
            fs::remove_dir_all(base).expect("cleanup should succeed");
        }
    }

    for (index, value) in [0.0, -1.0].into_iter().enumerate() {
        let base = unique_temp_dir(&format!("tw3_area_nonpositive_{index}"));
        let pass = base.join("H.pass.parquet");
        let wat = base.join("H.wat.parquet");
        write_pass_fixture(&pass);
        write_wat_fixture(&wat);
        replace_fixture_column(
            &wat,
            "Area",
            Field::new("Area", DataType::Float64, true),
            Arc::new(Float64Array::from(vec![value, 1_000.0, 500.0])),
        );
        assert!(
            matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::InvalidValue { column, row_index: 0, .. } if column == "Area")
        );
        fs::remove_dir_all(base).expect("cleanup should succeed");
    }
}

#[test]
fn optional_columns_cover_all_null_mixed_null_nonfinite_and_missing_partitions() {
    let base = unique_temp_dir("totalwatsed3_optional_value_matrix");
    let pass = base.join("H.pass.parquet");
    let wat = base.join("H.wat.parquet");
    let soil = base.join("H.soil.parquet");
    let element = base.join("H.element.parquet");
    write_pass_fixture(&pass);
    write_wat_fixture(&wat);
    append_fixture_column(
        &wat,
        Field::new("Interception", DataType::Float64, true),
        Arc::new(Float64Array::from(vec![0.2, 0.4, 0.6])),
    );
    write_soil_fixture(&soil);
    write_element_fixture(&element);
    replace_fixture_column(
        &element,
        "QRain",
        Field::new("QRain", DataType::Float64, true),
        Arc::new(Float64Array::from(vec![None, None, None, None])),
    );
    let mut config = direct_config(&base, vec![pass.clone()], vec![wat.clone()]);
    config.soil_paths.push(soil.clone());
    config.element_paths.push(element.clone());
    write_totalwatsed3(&config).expect("all-null optional QRain should mean absent");
    let batch = read_first_output_batch(&config.output_path);
    assert!((f64_column(&batch, "QRain") - 0.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "Interception") - 0.36).abs() <= 1.0e-12);

    write_element_fixture(&element);
    replace_fixture_column(
        &element,
        "QRain",
        Field::new("QRain", DataType::Float64, true),
        Arc::new(Float64Array::from(vec![
            Some(1.0),
            None,
            Some(9.0),
            Some(99.0),
        ])),
    );
    assert!(
        matches!(write_totalwatsed3(&config), Err(Totalwatsed3Error::NullValue { column, row_index: 1, .. }) if column == "QRain")
    );

    write_element_fixture(&element);
    replace_fixture_column(
        &element,
        "QRain",
        Field::new("QRain", DataType::Float64, true),
        Arc::new(Float64Array::from(vec![f64::INFINITY, 4.0, 10.0, 99.0])),
    );
    assert!(
        matches!(write_totalwatsed3(&config), Err(Totalwatsed3Error::InvalidValue { column, row_index: 0, .. }) if column == "QRain")
    );

    write_element_fixture(&element);
    replace_fixture_column(
        &element,
        "QSnow",
        Field::new("QSnow", DataType::Float64, true),
        Arc::new(Float64Array::from(vec![f64::NAN, 4.0, 10.0, 99.0])),
    );
    assert!(
        matches!(write_totalwatsed3(&config), Err(Totalwatsed3Error::InvalidValue { column, row_index: 0, .. }) if column == "QSnow")
    );

    write_element_fixture(&element);
    remove_fixture_column(&element, "QRain");
    remove_fixture_column(&element, "QSnow");
    write_totalwatsed3(&config).expect("missing optional partition columns should default");

    replace_fixture_column(
        &soil,
        "TSMF",
        Field::new("TSMF", DataType::Float64, true),
        Arc::new(Float64Array::from(vec![
            None,
            Some(0.3),
            Some(0.9),
            Some(99.0),
        ])),
    );
    assert!(
        matches!(write_totalwatsed3(&config), Err(Totalwatsed3Error::NullValue { column, row_index: 0, .. }) if column == "TSMF")
    );

    write_soil_fixture(&soil);
    replace_fixture_column(
        &soil,
        "TSMF",
        Field::new("TSMF", DataType::Float64, true),
        Arc::new(Float64Array::from(vec![f64::NEG_INFINITY, 0.3, 0.9, 99.0])),
    );
    assert!(
        matches!(write_totalwatsed3(&config), Err(Totalwatsed3Error::InvalidValue { column, row_index: 0, .. }) if column == "TSMF")
    );
    fs::remove_dir_all(base).expect("cleanup should succeed");
}

#[test]
fn aggregated_area_overflow_and_zero_runoff_paths_fail_or_zero_exactly() {
    let base = unique_temp_dir("totalwatsed3_aggregate_edge_paths");
    let pass = base.join("H.pass.parquet");
    let wat = base.join("H.wat.parquet");
    write_pass_fixture(&pass);
    write_wat_fixture(&wat);
    replace_fixture_column(
        &wat,
        "Area",
        Field::new("Area", DataType::Float64, true),
        Arc::new(Float64Array::from(vec![f64::MAX, f64::MAX, 1.0])),
    );
    assert!(
        matches!(direct_error(&base, &pass, &wat), Totalwatsed3Error::InvalidValue { ref path, ref column, .. } if path == Path::new("<aggregated WAT>") && column == "Area")
    );

    write_wat_fixture(&wat);
    empty_fixture(&pass);
    let config = direct_config(&base, vec![pass], vec![wat]);
    write_totalwatsed3(&config).expect("empty PASS domain should default daily pass operands");
    let batch = read_first_output_batch(&config.output_path);
    assert!((f64_column(&batch, "runvol") - 0.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "sed_del") - 0.0).abs() <= 1.0e-12);
    assert!((f64_column(&batch, "sed_vol_conc") - 0.0).abs() <= 1.0e-12);
    fs::remove_dir_all(base).expect("cleanup should succeed");
}
