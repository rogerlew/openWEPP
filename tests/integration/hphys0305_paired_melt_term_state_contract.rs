use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

const PACKAGE: &str =
    "docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/prompts/active/20260605-hphys0305-paired-melt-term-state-instrumentation-001_kickoff_agent_prompt.md";
const RUNNER: &str = "docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/hphys0305_paired_melt_term_state.py";
const LEDGER: &str = "docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/paired-melt-term-state-ledger.json";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";

fn collect_runner_source_files(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("runner hillslope source entry should be readable") {
        let entry = entry.expect("runner hillslope source entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            collect_runner_source_files(&path, files);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }
}

fn read_runner_hillslope_sources() -> String {
    let runner_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/openwepp-runner/src/hillslope");
    let mut files: Vec<PathBuf> = Vec::new();

    collect_runner_source_files(&runner_dir, &mut files);
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
fn hphys0305_package_requires_paired_term_state_no_compensation_gate() {
    let package = fs::read_to_string(PACKAGE).unwrap();
    let prompt = fs::read_to_string(PROMPT).unwrap();

    for required in [
        "contract-first",
        "amelt",
        "bmelt",
        "cmelt",
        "dmelt",
        "hrrain",
        "hrtemp",
        "tdpt",
        "hrad",
        "cloudC",
        "vwind",
        "snodpt",
        "densgt",
        "No downstream compensation",
    ] {
        assert!(
            package.contains(required) || prompt.contains(required),
            "missing package/prompt requirement {required}"
        );
    }
    assert!(
        prompt.contains("Execution mode: package-end-to-end (default)")
            && prompt.contains("Autonomy: execute package phases end-to-end")
            && prompt.contains("canonicalize-and-proceed")
            && prompt.contains("no external")
            && prompt.contains("connectivity")
    );
}

#[test]
fn hphys0305_contract_and_trace_aliases_are_registered() {
    let sc_watbal = fs::read_to_string(SC_WATBAL).unwrap();
    let runner_mod = read_runner_hillslope_sources();

    assert!(
        sc_watbal.contains("INV-WATBAL-078")
            && sc_watbal.contains("HPHYS0305 paired")
            && sc_watbal.contains("snow_hourly_rain_m")
            && sc_watbal.contains("snow_hourly_depth_after_m")
            && sc_watbal.contains("snow_hourly_density_after_kg_m3")
    );
    for field in [
        "snow_hourly_rain_m",
        "snow_hourly_snowfall_depth_m",
        "snow_hourly_depth_before_m",
        "snow_hourly_depth_available_m",
        "snow_hourly_depth_after_m",
        "snow_hourly_density_before_kg_m3",
        "snow_hourly_density_after_kg_m3",
        "snow.hourly.rain_m_",
        "snow.hourly.depth_after_m_",
        "snow.hourly.density_after_kg_m3_",
    ] {
        assert!(runner_mod.contains(field), "missing trace field {field}");
    }
}

#[test]
fn hphys0305_runner_uses_fixed_comparator_observe_without_remote_actions() {
    let runner = fs::read_to_string(RUNNER).unwrap();

    assert!(
        runner.contains("47ac4c32faeea81bb99081f955a14c38b815ef4d")
            && runner.contains("/workdir/wepp-forest_260430_baseline")
            && runner.contains("H305_T_AB")
            && runner.contains("H305_T_CD")
            && runner.contains("H305_F_HT")
            && runner.contains("H305_F_RR")
            && runner.contains("H305_F_CV")
            && runner.contains("H305_S_OUT")
            && runner.contains("git_rev_parse_fixed_source_worktree")
            && runner.contains("source_reuse_verified")
            && runner.contains("fixed_observe_binary_sha256")
            && runner.contains("production_edit_authorized")
            && !runner.contains(" git push ")
            && !runner.contains("\"push\"")
    );
}

#[test]
fn hphys0305_executed_ledger_is_status_coherent() {
    let text = fs::read_to_string(LEDGER).unwrap();
    let ledger: Vec<Value> = serde_json::from_str(&text).unwrap();
    assert_eq!(ledger.len(), 9);
    for row in ledger {
        assert_eq!(row["production_edit_authorized"].as_bool(), Some(false));
        assert!(row["first_divergent_source"].is_string());
        let comparisons = row["comparisons"].as_object().unwrap();
        let has_missing = comparisons.values().any(|comparison| {
            comparison["missing_baseline_count"].as_u64().unwrap_or(0) > 0
                || comparison["missing_openwepp_count"].as_u64().unwrap_or(0) > 0
        });
        if has_missing {
            assert_eq!(
                row["paired_surface_status"].as_str(),
                Some("paired-surface-gap")
            );
            assert!(
                row["first_divergent_source"]
                    .as_str()
                    .unwrap()
                    .starts_with("paired-surface-gap:"),
                "missing paired surfaces must remain HOLD"
            );
            assert_eq!(row["hphys0305_route"].as_str(), Some("surface-gap-hold"));
        }
    }
}
