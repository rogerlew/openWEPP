//! E.3 / SC-SED-001 rev 44 `INV-SED-016` regression: the multi-OFE Wave-1
//! chain produces sediment end-to-end through the direct-production
//! runtime on a REAL 2-OFE disturbed-forest soil substrate from the
//! WSHED-W7DC01 substrate (`insensible-aliquot` H102). The checked management
//! authority is cropland, so this fixture uses the typed open/no-strata Stage-3
//! owner and makes no active-canopy claim. Asserts:
//!
//! 1. the run completes with nonzero outlet detachment (the W7DC01 proof
//!    class — the retired EROD14 multi-OFE path published zero sediment);
//! 2. the emitted HBP EVENT is minor-1 and satisfies the CHAIN-form
//!    intake closure `Σ S_h(exit) = Σ_lanes(tdet − tdep)` plus the
//!    writer-side water closure `Σ V_h = runvol` on the outlet event row;
//! 3. the manifest publishes the `INV-RUNOFFPART-030` disposition
//!    surfaces (`erod14_qin_sediment_coupled = true`, the
//!    `wave1-hourly-sediment-coupled-handoff` policy);
//! 4. the Increment-2 entry-gate §4a observable: coarsening OFE-2's soil
//!    texture (the in-test soil-edit pattern from `dff_ws2`) changes the
//!    published per-class composition — per-OFE particle-class sourcing,
//!    not a hillslope-global distribution.

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use arrow_array::{Array, Float64Array, Int16Array};
use openwepp_runner::{HillslopeRunRequest, SidecarPolicy};

mod common;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

#[allow(clippy::too_many_lines)]
#[test]
fn erosion_multi_ofe_p102_wave1_chain_routes_sediment() {
    let fixture = fixture_path("erosion_multi_ofe_p102");
    let run_dir = copy_fixture_to_temp(&fixture, "erosion_p102_chain");
    let management = fs::read_to_string(run_dir.join("p102.man"))
        .expect("P102 fixture management should be readable");
    assert!(
        management.contains("# Landuse - <Cropland>"),
        "P102 is an erosion/hydrology fixture with typed open/no-strata Stage-3 authority; it must not silently acquire an active-canopy claim"
    );
    let report = run_p102(&run_dir);

    let pass_parquet = report
        .optional_outputs
        .iter()
        .find(|path| path.file_name().and_then(|name| name.to_str()) == Some("H102.pass.parquet"))
        .expect("p102 run should emit the pass parquet");

    // 1. Multi-OFE Wave-1 sediment exists (the W7DC01 proof class).
    let (tdet_sum, tdet_max, tdet_days) = column_summary(pass_parquet, "tdet");
    let (deposition_sum, _, _) = column_summary(pass_parquet, "tdep");
    assert!(
        tdet_sum > 0.0 && tdet_max > 0.0 && tdet_days >= 1,
        "the multi-OFE Wave-1 chain must publish nonzero outlet detachment \
         (sum={tdet_sum}, max={tdet_max}, days={tdet_days})"
    );
    // The outlet lane deposits routed upstream load — the physics the
    // chain adds. On this substrate deposition is material, not epsilon.
    assert!(
        deposition_sum > 0.0,
        "the outlet lane must deposit routed upstream sediment (sum={deposition_sum})"
    );

    // 2. Chain-form HBP EVENT closures.
    let (parsed, latest_event) =
        openwepp_input_contract::parsers::hbp::parse_hbp_from_path_with_latest_event_payload(
            &report.output_pass,
            openwepp_input_contract::parsers::hbp::HbpParseOptions::strict(),
        )
        .expect("the emitted minor-1 shard must round-trip through the parser");
    assert_eq!(parsed.schema_major, 1);
    assert_eq!(parsed.schema_minor, 1, "the chain writes minor 1");
    assert_eq!(parsed.npart, 5);
    let event = latest_event.expect("p102 has erosion events");
    assert_eq!(event.hourly_runoff_volume_m3.len(), 24);
    assert_eq!(event.hourly_sediment_mass_kg.len(), 24);

    let sediment_sum: f64 = event.hourly_sediment_mass_kg.iter().sum();
    let exported_kg = event.total_detachment_kg - event.total_deposition_kg;
    assert!(
        exported_kg > 0.0,
        "the serialized event must export sediment (tdet={}, tdep={})",
        event.total_detachment_kg,
        event.total_deposition_kg
    );
    assert!(
        (sediment_sum - exported_kg).abs() <= 1.0e-6 * exported_kg.abs(),
        "chain-form intake closure: Σ S_h(exit) must equal the CHAIN \
         aggregated tdet − tdep (Σ={sediment_sum}, exported={exported_kg})"
    );

    // Writer-side water closure on the OUTLET row of the serialized event
    // day: Σ V_h = runvol (pass rows are outlet-scoped by construction).
    let rows = read_outlet_rows(pass_parquet);
    let event_sim_year = event.calendar_year - climate_start_year(&run_dir.join("p102.cli")) + 1;
    let event_row = rows
        .iter()
        .filter(|row| {
            row.sim_year_index == i64::from(event_sim_year)
                && row.julian == i64::from(event.julian_day)
                && row.runvol_m3 > 0.0
        })
        .max_by(|left, right| left.runvol_m3.total_cmp(&right.runvol_m3))
        .expect("the serialized event day must exist in the pass parquet");
    let volume_sum: f64 = event.hourly_runoff_volume_m3.iter().sum();
    assert!(
        (volume_sum - event_row.runvol_m3).abs() <= 1.0e-9 * event_row.runvol_m3.max(1.0e-9),
        "Σ V_h must equal the outlet event-day runvol (Σ={volume_sum}, runvol={})",
        event_row.runvol_m3
    );
    let reconstructed_peak_m3_s = event
        .hourly_runoff_volume_m3
        .iter()
        .copied()
        .fold(0.0_f64, f64::max)
        / 3_600.0;
    let peak_tolerance = 1.0e-12 * event.peak_runoff_m3_s.max(1.0);
    assert!(
        (reconstructed_peak_m3_s - event.peak_runoff_m3_s).abs() <= peak_tolerance,
        "multi-OFE HBP peak must reconstruct from the routed outlet V_h series"
    );
    assert!(
        (event_row.peakro_m3_s - event.peak_runoff_m3_s).abs() <= peak_tolerance,
        "outlet pass parquet and HBP must publish the same routed event peak"
    );

    // The chain totals must be at least the outlet row's own totals
    // (they aggregate every lane on the event day).
    assert!(
        event.total_detachment_kg >= event_row.tdet_kg - 1.0e-9,
        "chain tdet ({}) must cover the outlet row's own tdet ({})",
        event.total_detachment_kg,
        event_row.tdet_kg
    );

    // 3. The INV-RUNOFFPART-030 disposition manifest surfaces.
    let manifest = fs::read_to_string(&report.manifest_path).expect("read manifest");
    assert!(
        manifest.contains("\"erod14_qin_sediment_coupled\":true")
            || manifest.contains("\"erod14_qin_sediment_coupled\": true"),
        "multi-OFE Wave-1 chain manifests must publish erod14_qin_sediment_coupled = true"
    );
    assert!(
        manifest.contains("wave1-hourly-sediment-coupled-handoff"),
        "the qin source policy must be the Wave-1 coupled handoff"
    );
    // INV-SED-016 (f): the flux-diagnostic skip count is a SURFACED
    // manifest counter (zero on runs with no refusals), never
    // internal-only.
    assert!(
        manifest.contains("\"wave1_flux_refused_quanta\""),
        "the flux_refused_quanta count must surface in the manifest"
    );

    // 4. §4a per-OFE particle-class sourcing observable: coarsen OFE-2's
    // surface texture and the published event composition must move.
    let base_fractions = event.particle_flow_fraction.clone();
    let variant_dir = copy_fixture_to_temp(&fixture, "erosion_p102_chain_coarse");
    coarsen_second_ofe_soil(&variant_dir.join("p102.sol"));
    let variant_report = run_p102(&variant_dir);
    let (_, variant_event) =
        openwepp_input_contract::parsers::hbp::parse_hbp_from_path_with_latest_event_payload(
            &variant_report.output_pass,
            openwepp_input_contract::parsers::hbp::HbpParseOptions::strict(),
        )
        .expect("the variant shard must round-trip");
    let variant_event = variant_event.expect("the variant still routes events");
    let max_fraction_delta = base_fractions
        .iter()
        .zip(variant_event.particle_flow_fraction.iter())
        .map(|(base, variant)| (base - variant).abs())
        .fold(0.0_f64, f64::max);
    assert!(
        max_fraction_delta > 1.0e-3,
        "coarsening OFE-2's soil must change the exit composition \
         (per-OFE class sourcing, entry-gate §4a); max delta {max_fraction_delta}"
    );

    fs::remove_dir_all(&run_dir).ok();
    fs::remove_dir_all(&variant_dir).ok();
}

fn run_p102(run_dir: &Path) -> openwepp_runner::HillslopeRunReport {
    let output_dir = run_dir.join("output");
    common::execute_with_adaptive_stage3_owner_seed(
        &HillslopeRunRequest {
            run_dir: run_dir.to_path_buf(),
            run_file: PathBuf::from("p102.run"),
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
            "p102.run".to_string(),
            "--output-dir".to_string(),
            output_dir.display().to_string(),
            "--direct-production-executor".to_string(),
        ],
    )
    .expect("p102 multi-OFE direct-production run should complete")
}

/// Coarsen the SECOND OFE's surface soil texture (sand 45.4 → 80.0,
/// clay 13.0 → 5.0) so its `prtcmp` particle classes diverge from
/// OFE-1's — the §4a per-OFE-sourcing probe. The p102 `.sol` carries the
/// texture columns as `... 45.4  13.0 ...` on layer rows of both OFEs;
/// only rows AFTER the second OFE header line are rewritten.
fn coarsen_second_ofe_soil(soil_path: &Path) {
    let contents = fs::read_to_string(soil_path).expect("read fixture soil");
    let mut in_second_ofe = false;
    let mut ofe_headers_seen = 0_usize;
    let updated = contents
        .split('\n')
        .map(|line| {
            if line.contains("sev fire") && line.contains('\'') {
                ofe_headers_seen += 1;
                in_second_ofe = ofe_headers_seen == 2;
            }
            if in_second_ofe && line.contains("45.4") && line.contains("13.0") {
                line.replace("45.4", "80.0").replace("13.0", "5.0")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_ne!(updated, contents, "the variant must modify OFE-2 layers");
    fs::write(soil_path, updated).expect("write variant soil");
}

struct OutletRow {
    sim_year_index: i64,
    julian: i64,
    runvol_m3: f64,
    peakro_m3_s: f64,
    tdet_kg: f64,
}

fn read_outlet_rows(path: &Path) -> Vec<OutletRow> {
    let file = File::open(path).expect("open pass parquet");
    let reader = ParquetRecordBatchReaderBuilder::try_new(file)
        .expect("parquet reader builder")
        .build()
        .expect("build parquet reader");
    let mut rows = Vec::new();
    for batch in reader {
        let batch = batch.expect("read record batch");
        let year = column_i16(&batch, "year");
        let julian = column_i16(&batch, "julian");
        let runvol = column_f64(&batch, "runvol");
        let peakro = column_f64(&batch, "peakro");
        let tdet = column_f64(&batch, "tdet");
        for i in 0..batch.num_rows() {
            rows.push(OutletRow {
                sim_year_index: i64::from(year.value(i)),
                julian: i64::from(julian.value(i)),
                runvol_m3: runvol.value(i),
                peakro_m3_s: peakro.value(i),
                tdet_kg: tdet.value(i),
            });
        }
    }
    rows
}

fn column_f64<'a>(batch: &'a arrow_array::RecordBatch, name: &str) -> &'a Float64Array {
    let index = batch
        .schema()
        .index_of(name)
        .unwrap_or_else(|_| panic!("pass parquet must carry `{name}`"));
    batch
        .column(index)
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap_or_else(|| panic!("`{name}` must be Float64"))
}

fn column_i16<'a>(batch: &'a arrow_array::RecordBatch, name: &str) -> &'a Int16Array {
    let index = batch
        .schema()
        .index_of(name)
        .unwrap_or_else(|_| panic!("pass parquet must carry `{name}`"));
    batch
        .column(index)
        .as_any()
        .downcast_ref::<Int16Array>()
        .unwrap_or_else(|| panic!("`{name}` must be Int16"))
}

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
        let array = column_f64(&batch, column);
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

fn climate_start_year(path: &Path) -> i32 {
    fs::read_to_string(path)
        .expect("read fixture climate")
        .lines()
        .find_map(|line| {
            let fields: Vec<_> = line.split_whitespace().take(3).collect();
            let [day, month, year] = fields.as_slice() else {
                return None;
            };
            let day = day.parse::<u8>().ok()?;
            let month = month.parse::<u8>().ok()?;
            let year = year.parse::<i32>().ok()?;
            ((1..=31).contains(&day) && (1..=12).contains(&month) && year >= 1).then_some(year)
        })
        .expect("fixture climate contains a daily calendar row")
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
