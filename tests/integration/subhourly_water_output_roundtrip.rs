use std::fs::{self, File};
use std::path::PathBuf;

use arrow_array::{Array, Float64Array, Int32Array};
use openwepp_hillslope_output::hillslope_wat_subhourly::{
    HILLSLOPE_WAT_SUBHOURLY_SCHEMA_ID, HillslopeWatSubhourlyParquetRowGroupWriter,
    HillslopeWatSubhourlyRow,
};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

fn row(bin: i32) -> HillslopeWatSubhourlyRow {
    let hour = bin / 12;
    let closed_mm = if hour == 0 { 0.1 } else { 0.2 };
    let saturation_mm = if hour == 0 { 0.01 } else { 0.02 };
    let closing_mm = closed_mm + saturation_mm;
    HillslopeWatSubhourlyRow {
        wepp_id: 61,
        ofe_id: 1,
        year: 2026,
        sim_day_index: 4,
        julian: 5,
        event_ordinal: 0,
        hour_index: hour,
        subinterval_index: bin,
        interval_start_s: f64::from(bin) * 300.0,
        interval_duration_s: 300.0,
        rainfall_depth_mm: closed_mm + 0.05,
        additional_supply_depth_mm: 0.0,
        raw_green_ampt_infiltration_depth_mm: 0.05,
        depression_storage_retention_depth_mm: 0.0,
        raw_wb14_post_depression_generation_depth_mm: closed_mm,
        closed_wb14_generation_depth_mm: closed_mm,
        saturation_return_depth_mm: saturation_mm,
        closing_surface_generation_depth_mm: closing_mm,
        closing_surface_generation_intensity_mm_h: closing_mm * 12.0,
        hourly_authoritative_runoff_depth_mm: closing_mm * 12.0,
        hourly_mean_generation_intensity_mm_h: closing_mm * 12.0,
        hourly_power_equivalent_generation_intensity_mm_h: None,
        hourly_power_equivalent_duration_s: None,
        power_exponent: None,
        method_code: "water_only_no_erosion_adoption".to_string(),
        source_completeness_code: "rainfall_complete_saturation_hourly_zero_order_hold".to_string(),
        hourly_closure_residual_mm: 0.0,
    }
}

#[test]
fn typed_multi_hour_parquet_roundtrip_preserves_keys_nulls_and_closure() {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("test clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("openwepp-wat5-roundtrip-{nonce}.parquet"));
    let rows: Vec<_> = (0..24).map(row).collect();
    let mut writer =
        HillslopeWatSubhourlyParquetRowGroupWriter::create(&path).expect("create WAT5 writer");
    writer.write_rows(&rows[..7]).expect("first row group");
    writer.write_rows(&rows[7..]).expect("second row group");
    let summary = writer.close().expect("close WAT5 writer");
    assert_eq!(summary.rows_written, 24);

    let builder =
        ParquetRecordBatchReaderBuilder::try_new(File::open(&path).expect("open WAT5 output"))
            .expect("read WAT5 metadata");
    assert_eq!(builder.schema().fields().len(), 27);
    assert_eq!(
        builder
            .schema()
            .metadata()
            .get("dataset_id")
            .map(String::as_str),
        Some(HILLSLOPE_WAT_SUBHOURLY_SCHEMA_ID)
    );
    let batches: Vec<_> = builder
        .build()
        .expect("build reader")
        .map(|batch| batch.expect("valid WAT5 batch"))
        .collect();
    assert_eq!(
        batches
            .iter()
            .map(arrow_array::RecordBatch::num_rows)
            .sum::<usize>(),
        24
    );

    let mut next_bin = 0_i32;
    let mut hourly_observed_mm = [0.0_f64; 2];
    for batch in &batches {
        let hours = batch
            .column(6)
            .as_any()
            .downcast_ref::<Int32Array>()
            .expect("hour index column");
        let bins = batch
            .column(7)
            .as_any()
            .downcast_ref::<Int32Array>()
            .expect("subinterval index column");
        let closing = batch
            .column(17)
            .as_any()
            .downcast_ref::<Float64Array>()
            .expect("closing depth column");
        for index in 0..batch.num_rows() {
            assert_eq!(bins.value(index), next_bin);
            next_bin += 1;
            let hour = usize::try_from(hours.value(index)).expect("nonnegative hour");
            hourly_observed_mm[hour] += closing.value(index);
        }
        for candidate_column in [21, 22, 23] {
            assert_eq!(
                batch.column(candidate_column).null_count(),
                batch.num_rows()
            );
        }
    }
    assert!((hourly_observed_mm[0] - 1.32).abs() <= 1.0e-12);
    assert!((hourly_observed_mm[1] - 2.64).abs() <= 1.0e-12);
    fs::remove_file(path).expect("remove WAT5 roundtrip output");
}

#[test]
fn output_catalog_declares_every_dimensional_wat5_column() {
    let catalog = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("crates/openwepp-sim-contract/src/units_mod/output_catalog.rs"),
    )
    .expect("output catalog");
    for required in [
        "hillslope_wat_subhourly",
        "rainfall_depth_mm",
        "raw_green_ampt_infiltration_depth_mm",
        "depression_storage_retention_depth_mm",
        "raw_wb14_post_depression_generation_depth_mm",
        "closed_wb14_generation_depth_mm",
        "closing_surface_generation_intensity_mm_h",
        "hourly_authoritative_runoff_depth_mm",
        "hourly_closure_residual_mm",
    ] {
        assert!(
            catalog.contains(required),
            "missing output-unit registry entry: {required}"
        );
    }
}

#[test]
fn positive_depression_storage_rows_reconstruct_raw_water_closure() {
    let mut rows: Vec<_> = (0..12).map(row).collect();
    rows[0].rainfall_depth_mm = 0.20;
    rows[0].raw_green_ampt_infiltration_depth_mm = 0.05;
    rows[0].depression_storage_retention_depth_mm = 0.05;
    rows[0].raw_wb14_post_depression_generation_depth_mm = 0.10;

    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("test clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("openwepp-wat5-storage-{nonce}.parquet"));
    let mut writer =
        HillslopeWatSubhourlyParquetRowGroupWriter::create(&path).expect("create WAT5 writer");
    writer
        .write_rows(&rows)
        .expect("write positive-storage rows");
    writer.close().expect("publish positive-storage Parquet");

    let batches = ParquetRecordBatchReaderBuilder::try_new(
        File::open(&path).expect("open positive-storage Parquet"),
    )
    .expect("read positive-storage metadata")
    .build()
    .expect("build positive-storage reader")
    .map(|batch| batch.expect("valid positive-storage batch"))
    .collect::<Vec<_>>();
    let sum_column = |column: usize| {
        batches
            .iter()
            .map(|batch| {
                batch
                    .column(column)
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .expect("depth column")
                    .values()
                    .iter()
                    .sum::<f64>()
            })
            .sum::<f64>()
    };
    let rainfall_mm = sum_column(10);
    let reconstructed_mm = sum_column(12) + sum_column(13) + sum_column(14);
    assert!((rainfall_mm - reconstructed_mm).abs() <= 1.0e-12);
    assert!(sum_column(13) > 0.0);
    fs::remove_file(path).expect("remove positive-storage Parquet");
}
