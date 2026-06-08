use std::fs;
use std::path::Path;

const SC_CLIMATE: &str = "docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md";
const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const PACKAGE: &str = "docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/prompts/active/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001_kickoff_agent_prompt.md";
const RUNNER: &str = "docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/hphys0299_corrected_partition.py";
const LEDGER: &str = "docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/corrected-partition-ledger.json";
const OPENWEPP_FORCING: &str =
    "crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs";

fn read_runner_hillslope_sources() -> String {
    let runner_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/openwepp-runner/src/hillslope");
    let mut files: Vec<_> = fs::read_dir(&runner_dir)
        .expect("runner hillslope source directory should be readable")
        .map(|entry| {
            entry
                .expect("runner hillslope source entry should be readable")
                .path()
        })
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("rs"))
        .collect();
    files.sort();

    files
        .into_iter()
        .map(|path| {
            fs::read_to_string(&path).unwrap_or_else(|error| {
                panic!(
                    "runner source {} should be readable: {error}",
                    path.display()
                )
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn hphys0299_contracts_distinguish_hrsnow_depth_from_water_equivalent() {
    let climate = fs::read_to_string(SC_CLIMATE).expect("climate contract should be readable");
    let snow = fs::read_to_string(SC_SNOWFREEZE).expect("snow contract should be readable");
    let watbal = fs::read_to_string(SC_WATBAL).expect("watbal contract should be readable");

    assert!(
        climate.contains("INV-CLIMATE-014")
            && climate.contains("stmtim.for")
            && climate.contains("hrsnow(hour) = rain / wntdur * 10.0")
            && climate.contains("snow_hourly_snowfall_depth_sum_m")
            && climate.contains("snow_hourly_snowfall_water_equiv_sum_m")
            && climate.contains("cannot justify production precipitation-partition migration"),
        "SC-CLIMATE must define HPHYS0299 snowfall-depth vs water-equivalent authority"
    );
    assert!(
        snow.contains("INV-SNOWFREEZE-030")
            && snow.contains("winter.for:296-300")
            && snow.contains("stmtim.for:43-95")
            && snow.contains("observe cut-point, not the partition equation")
            && snow.contains("snow.hourly.snowfall_m_####")
            && snow.contains("snow_hourly_snowfall_depth_sum_m")
            && snow.contains("snow_hourly_snowfall_water_equiv_sum_m")
            && snow.contains("depth-vs-water-equivalent comparison"),
        "SC-SNOWFREEZE must suspend HPHYS0298 migration authority until corrected depth evidence"
    );
    assert!(
        watbal.contains("INV-WATBAL-074")
            && watbal.contains("corrected HPHYS0299 unit/provenance ledger")
            && watbal.contains("baseline `stmtim.for` `hrsnow`")
            && watbal.contains("snow_hourly_snowfall_depth_sum_m")
            && watbal.contains("snow_hourly_snowfall_water_equiv_sum_m")
            && watbal.contains("continuation routing must be recalculated")
            && watbal.contains("HPHYS0299 hourly snow partition unit/provenance gate")
            && watbal.contains("corrected depth-vs-depth evidence is absent"),
        "SC-WATBAL must require corrected unit/provenance evidence before routing continuation"
    );
}

#[test]
fn hphys0299_package_and_prompt_prohibit_depth_water_equivalent_migration() {
    let package = fs::read_to_string(PACKAGE).expect("package should be readable");
    let prompt = fs::read_to_string(PROMPT).expect("prompt should be readable");

    assert!(
        package.contains("snow_hourly_snowfall_depth_sum_m")
            && package.contains("snow_hourly_snowfall_water_equiv_sum_m")
            && package.contains("Correctness requires resolving that diagnostic/provenance seam")
            && package.contains(
                "Production migration of hourly precipitation-phase physics unless corrected"
            )
            && package.contains("Corrected HPHYS0299 partition ledger is published"),
        "HPHYS0299 package must encode corrected unit/provenance objective and exit criteria"
    );
    assert!(
        prompt.contains("Execution mode: package-end-to-end (default)")
            && prompt.contains("contract-first sequencing")
            && prompt.contains("/workdir/wepp-forest_260430_baseline")
            && prompt.contains("no production physics migration based on a")
            && prompt.contains("depth-vs-water-equivalent diagnostic mismatch")
            && prompt.contains("Autonomy: execute package phases end-to-end"),
        "HPHYS0299 prompt must be autonomous and prohibit migration from the old mismatch"
    );
}

#[test]
fn hphys0299_runner_uses_depth_field_for_canonical_hrsnow() {
    let runner = fs::read_to_string(RUNNER).expect("HPHYS0299 runner should be readable");

    assert!(
        runner.contains("\"snow_hourly_snowfall_depth_sum_m\"")
            && runner.contains("canonical_hrsnow_openwepp_field")
            && runner.contains("validate_trace_fields")
            && runner.contains("\"missing\"")
            && runner.contains("not-map")
            && runner.contains("non-numeric")
            && runner.contains("non-finite")
            && runner.contains("stmtim.for:43-95")
            && runner.contains("winter.for:296-300")
            && runner.contains("depth-vs-depth"),
        "HPHYS0299 runner must make the corrected depth mapping explicit"
    );
    assert!(
        runner.contains("partition_row.get(\"verdict\") == \"LEGACY-DEFECTIVE\"")
            && runner.contains("baseline_negative_raw_melt_sum_mm")
            && runner.contains("post-raw routed-melt/negative-melt handling"),
        "negative-melt continuation routing must not accept OPENWEPP-DEFECTIVE rows as legacy-defective authority"
    );
    let compute_window = runner
        .split("def compute_window_partition")
        .nth(1)
        .and_then(|tail| tail.split("def ").next())
        .expect("compute_window_partition should be extractable");
    assert!(
        compute_window.contains("\"snow_hourly_snowfall_depth_sum_m\""),
        "canonical hrsnow parity must use snowfall depth in window partitioning"
    );
    assert!(
        !compute_window.contains("\"snow_hourly_snowfall_water_equiv_sum_m\""),
        "canonical hrsnow parity must not use snowfall water equivalent"
    );
}

#[test]
fn hphys0299_ledger_routes_negative_melt_openwepp_defect_to_follow_on() {
    let ledger = fs::read_to_string(LEDGER).expect("corrected ledger should be readable");

    assert!(
        ledger.contains("\"first_divergent_cut_point\": \"negative-melt-correction\"")
            && ledger.contains("\"verdict\": \"OPENWEPP-DEFECTIVE\"")
            && ledger.contains("\"baseline_negative_raw_melt_sum_mm\": 0.0")
            && ledger
                .contains("Open follow-on package for post-raw routed-melt/negative-melt handling")
            && ledger
                .contains("not accepted as corrected negative-melt legacy-defective authority"),
        "OPENWEPP-DEFECTIVE negative-melt rows must route to follow-on, not legacy-defective acceptance"
    );
}

#[test]
fn hphys0299_static_openwepp_sources_publish_depth_and_water_equiv_separately() {
    let forcing =
        fs::read_to_string(OPENWEPP_FORCING).expect("openWEPP forcing source should be readable");
    let runner = read_runner_hillslope_sources();

    assert!(
        forcing.contains("simimpl28_stmtim_hourly_partition")
            && forcing.contains("rain_m / wntdur * 10.0")
            && forcing.contains("snow.hourly.snowfall_m"),
        "SIMIMPL28 forcing source should expose the baseline stmtim cold-branch depth equation"
    );
    assert!(
        runner.contains("snow_hourly_snowfall_depth_sum_m")
            && runner.contains("snow_hourly_snowfall_water_equiv_sum_m")
            && runner.contains("hphys0245_sum_runtime_prefix")
            && runner.contains("\"snow.hourly.snowfall_m_\"")
            && runner.contains("new_snow_density_kg_m3")
            && runner.contains("/ 1_000.0"),
        "runner source must keep snowfall depth and derived water-equivalent summaries separate"
    );
}
