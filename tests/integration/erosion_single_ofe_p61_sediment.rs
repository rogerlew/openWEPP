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

// One end-to-end fixture driver validating the E.1 + E.2 publication
// surfaces off a single run; splitting it would re-run the fixture per
// assertion group.
#[allow(clippy::too_many_lines)]
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

    // GAP-SED-009 closure band (the ground-cover authority fix): the
    // dominant event's per-width export must sit in the legacy ORDER —
    // legacy `Sed.Del` is 4.2 kg/m, the fixed runtime lands ~3.97, and
    // the bare-soil defect produced ~25. The band is generous (the
    // magnitude is not an acceptance oracle, ADR-0017) but excludes the
    // zero-cover regression class by a wide margin.
    let fwidth_m = 724.3;
    let max_export_kg_m = read_sediment_rows(pass_parquet)
        .into_iter()
        .map(|row| (row.tdet_kg - row.tdep_kg) / fwidth_m)
        .fold(0.0_f64, f64::max)
        .max(0.0);
    assert!(
        (0.5..=12.0).contains(&max_export_kg_m),
        "the dominant-event per-width export must stay in the legacy \
         order (observed {max_export_kg_m} kg/m; bare-soil regression \
         was ~25, legacy is 4.2)"
    );

    // Total detachment must be finite and mass-nonnegative.
    assert!(tdet_sum.is_finite(), "total detachment must be finite");

    // E.1 per-class publication: the detaching event days must carry a
    // nonzero 5-class concentration split (detached composition × toe
    // concentration) through to the pass parquet.
    let mut sedcon_total = 0.0;
    let mut sedcon_nonzero_columns = 0_usize;
    for column in ["sedcon_1", "sedcon_2", "sedcon_3", "sedcon_4", "sedcon_5"] {
        let (sum, _, _) = column_summary(pass_parquet, column);
        assert!(sum.is_finite(), "{column} must be finite");
        assert!(sum >= 0.0, "{column} must be nonnegative");
        if sum > 0.0 {
            sedcon_nonzero_columns += 1;
        }
        sedcon_total += sum;
    }
    assert!(
        sedcon_total > 0.0 && sedcon_nonzero_columns >= 2,
        "per-class sedcon must publish a nonzero composition split \
         (total={sedcon_total}, nonzero_columns={sedcon_nonzero_columns})"
    );

    // E.1 `field_width_m` output-level reconstruction (Codex round-1): on a
    // zero-deposition event day the exported mass equals the detached mass,
    // so the width-scaled total must reconstruct from two INDEPENDENTLY
    // produced surfaces: `tdet = Σ_i sedcon_i × runvol`. `sedcon` is
    // width-independent (`sloss.for:305-317`) and `runvol` carries the
    // water-path hillslope area, so a seed reverted to unit width (or fed a
    // width alias that disagrees with the water-path geometry) breaks this
    // identity by the width factor (~724× on p61). Observed residual on the
    // real run is machine epsilon (~2.4e-16 rel).
    let mut reconstructed_days = 0_usize;
    for row in read_sediment_rows(pass_parquet) {
        if row.tdet_kg <= 0.0 || row.tdep_kg != 0.0 {
            continue;
        }
        let sedcon_sum: f64 = row.sedcon_kg_m3.iter().sum();
        let reconstructed_kg = sedcon_sum * row.runvol_m3;
        let residual = (row.tdet_kg - reconstructed_kg).abs();
        assert!(
            residual <= 1.0e-9 * row.tdet_kg,
            "field-width reconstruction failed: tdet={} kg must equal \
             Σ sedcon × runvol = {} kg (residual {residual})",
            row.tdet_kg,
            reconstructed_kg
        );
        reconstructed_days += 1;
    }
    assert!(
        reconstructed_days >= 1,
        "the width reconstruction must exercise at least one \
         zero-deposition event day (found {reconstructed_days})"
    );

    // E.2 (ADR-0036 / SC-INFILE-HBP-001 v0.2.0) round-trip: the emitted
    // HBP shard is a minor-1 payload carrying the paired hourly surfaces
    // with their integral closures, npart = 5 per-class arrays, and the
    // true-volumetric peak.
    let hbp_path = &report.output_pass;
    assert!(
        hbp_path.file_name().and_then(|name| name.to_str()) == Some("H61.hbp"),
        "output_pass must be the HBP shard, observed {}",
        hbp_path.display()
    );
    let (parsed, latest_event) =
        openwepp_input_contract::parsers::hbp::parse_hbp_from_path_with_latest_event_payload(
            hbp_path,
            openwepp_input_contract::parsers::hbp::HbpParseOptions::strict(),
        )
        .expect("the emitted minor-1 shard must round-trip through the parser");
    assert_eq!(parsed.schema_major, 1);
    assert_eq!(parsed.schema_minor, 1, "hydrograph lane must write minor 1");
    assert_eq!(parsed.npart, 5, "production per-class arrays are npart = 5");
    let event = latest_event.expect("p61 has runoff events");
    assert_eq!(event.hourly_runoff_volume_m3.len(), 24);
    assert_eq!(event.hourly_sediment_mass_kg.len(), 24);
    let volume_sum: f64 = event.hourly_runoff_volume_m3.iter().sum();
    assert!(volume_sum > 0.0, "the event day carries hourly volume");
    // Writer-side water closure (SC-INFILE-HBP-001 §8.5): Σ V_h equals the
    // pass parquet's runvol on the serialized event day (the max-tdet row).
    let event_row = read_sediment_rows(pass_parquet)
        .into_iter()
        .max_by(|a, b| a.tdet_kg.total_cmp(&b.tdet_kg))
        .expect("pass parquet has rows");
    assert!(
        (volume_sum - event_row.runvol_m3).abs() <= 1.0e-9 * event_row.runvol_m3.max(1.0e-9),
        "Σ V_h must equal the event day's runvol          (Σ={volume_sum}, runvol={})",
        event_row.runvol_m3
    );
    let sediment_sum: f64 = event.hourly_sediment_mass_kg.iter().sum();
    let exported_kg = event.total_detachment_kg - event.total_deposition_kg;
    assert!(
        (sediment_sum - exported_kg).abs() <= 1.0e-6 * exported_kg.abs().max(1.0e-9),
        "Σ S_h must close on the event exported mass \
         (Σ={sediment_sum}, tdet-tdep={exported_kg})"
    );
    assert_eq!(event.sediment_concentration_kg_m3.len(), 5);
    assert_eq!(event.particle_flow_fraction.len(), 5);
    let fraction_sum: f64 = event.particle_flow_fraction.iter().sum();
    assert!(
        (fraction_sum - 1.0).abs() < 1.0e-9,
        "exiting class fractions must be normalized on a sediment event \
         (sum {fraction_sum})"
    );
    assert_eq!(event.particle_diameter_m.len(), 5);
    assert!(
        event
            .particle_diameter_m
            .iter()
            .all(|diameter| *diameter > 0.0),
        "per-class diameters must be the real prtcmp composition"
    );
}

struct SedimentRow {
    tdet_kg: f64,
    tdep_kg: f64,
    runvol_m3: f64,
    sedcon_kg_m3: [f64; 5],
}

/// Per-row read of the pass-parquet sediment reconstruction columns.
fn read_sediment_rows(path: &Path) -> Vec<SedimentRow> {
    let file = File::open(path).expect("open pass parquet");
    let reader = ParquetRecordBatchReaderBuilder::try_new(file)
        .expect("parquet reader builder")
        .build()
        .expect("build parquet reader");
    let mut rows = Vec::new();
    for batch in reader {
        let batch = batch.expect("read record batch");
        let column = |name: &str| -> Vec<f64> {
            let index = batch
                .schema()
                .index_of(name)
                .unwrap_or_else(|_| panic!("pass parquet must carry the `{name}` column"));
            let array = batch
                .column(index)
                .as_any()
                .downcast_ref::<Float64Array>()
                .unwrap_or_else(|| panic!("`{name}` must be Float64"));
            (0..array.len()).map(|i| array.value(i)).collect()
        };
        let detachment = column("tdet");
        let deposition = column("tdep");
        let runvol = column("runvol");
        let sedcon: Vec<Vec<f64>> = ["sedcon_1", "sedcon_2", "sedcon_3", "sedcon_4", "sedcon_5"]
            .iter()
            .map(|name| column(name))
            .collect();
        for i in 0..detachment.len() {
            rows.push(SedimentRow {
                tdet_kg: detachment[i],
                tdep_kg: deposition[i],
                runvol_m3: runvol[i],
                sedcon_kg_m3: [
                    sedcon[0][i],
                    sedcon[1][i],
                    sedcon[2][i],
                    sedcon[3][i],
                    sedcon[4][i],
                ],
            });
        }
    }
    rows
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
