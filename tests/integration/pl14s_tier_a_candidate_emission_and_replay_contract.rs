use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use openwepp_comparator_metadata::{
    COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID, ComparatorConfidenceTier,
    ComparatorSurfaceClass, ComparatorTierRoutingRequest, route_comparator_tier_metadata,
};

const PL14S_SEMANTIC_COMPARATOR_SCRIPT: &str =
    include_str!("../../tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py");
const PL14S_REPLAY_SUITE_SCRIPT: &str =
    include_str!("../../tools/legacy_comparison_suite/run_pl14s_legacy_suite.py");
const PL14S_SUITE_README: &str = include_str!("../../tools/legacy_comparison_suite/README.md");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StrictLaneMode {
    Required,
    StrictEquivalentRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CandidateSourceClass {
    NativeRuntimeDat,
    ConversionDerivedDat,
    NativeRuntimeParquet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConversionSourceRowConsistencyVerdict {
    Ready,
    Hold,
}

fn strict_lane_mode(candidate_extension: &str) -> StrictLaneMode {
    if candidate_extension.eq_ignore_ascii_case(".dat") {
        StrictLaneMode::Required
    } else {
        StrictLaneMode::StrictEquivalentRequired
    }
}

fn candidate_source_class(
    candidate_extension: &str,
    conversion_derived_dat: bool,
) -> CandidateSourceClass {
    if candidate_extension.eq_ignore_ascii_case(".dat") {
        if conversion_derived_dat {
            CandidateSourceClass::ConversionDerivedDat
        } else {
            CandidateSourceClass::NativeRuntimeDat
        }
    } else {
        CandidateSourceClass::NativeRuntimeParquet
    }
}

fn conversion_derived_dat_row_consistency_verdict(
    common_row_count: usize,
    only_baseline_count: usize,
    only_candidate_count: usize,
) -> ConversionSourceRowConsistencyVerdict {
    if common_row_count == 0 || only_baseline_count > 0 || only_candidate_count > 0 {
        return ConversionSourceRowConsistencyVerdict::Hold;
    }
    ConversionSourceRowConsistencyVerdict::Ready
}

fn contains_all(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().all(|needle| haystack.contains(needle))
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn fixture_temp_dir(prefix: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    env::temp_dir().join(format!("{prefix}_{stamp}"))
}

fn dat_row(ofe: i32, julian: i32, year: i32, seed: i32) -> String {
    let tail = (0_i32..17_i32)
        .map(|offset| (seed + offset).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    format!("{ofe} {julian} {year} {tail}")
}

#[test]
fn pl14s_contract_conformance_routes_single_ofe_daily_lane_to_higher_confidence() {
    let metadata = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
        Some(1),
    ))
    .expect("single OFE replay lane should route");

    assert_eq!(
        metadata.surface_class,
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance
    );
    assert_eq!(
        metadata.confidence_tier,
        ComparatorConfidenceTier::HigherConfidence
    );
    assert_eq!(
        metadata.message_id,
        COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID
    );
}

#[test]
fn pl14s_contract_conformance_declares_semantic_report_and_provenance_schema_markers() {
    assert!(contains_all(
        PL14S_SEMANTIC_COMPARATOR_SCRIPT,
        &[
            "REPORT_SCHEMA_VERSION = \"pl14s-semantic-wat-v2\"",
            "duplicate row key",
            "\"Total-Soil\": \"Total-Soil\"",
            "\"Total-Soil Water\": \"Total-Soil\"",
            "baseline_only_columns",
            "candidate_only_columns",
            "investigation_columns_used",
            "candidate_column_alias_sources",
            "row_key_fields",
        ]
    ));
    assert!(contains_all(
        PL14S_REPLAY_SUITE_SCRIPT,
        &[
            "\"suite_schema_version\": \"pl14s-legacy-suite-v2\"",
            "--candidate-surface-source-class",
            "\"strict_lane_policy\"",
            "\"strict-equivalent-required\"",
            "\"native-runtime-dat\"",
            "\"conversion-derived-dat\"",
            "\"native-runtime-parquet\"",
            "\"common_row_count\"",
            "\"conversion_source_row_consistency_ready\"",
            "\"conversion_source_row_consistency_blockers\"",
            "semantic_summary = load_semantic_summary",
        ]
    ));
    assert!(contains_all(
        PL14S_SUITE_README,
        &[
            "Strict comparator is required when candidate input is `.dat`",
            "`--candidate-surface-source-class`",
            "strict-equivalent-required",
            "conversion-derived-dat",
            "row-presence deltas",
            "top divergent rows",
        ]
    ));
}

#[test]
fn pl14s_contract_conformance_enforces_strict_lane_required_vs_strict_equivalent_modes() {
    assert_eq!(strict_lane_mode(".dat"), StrictLaneMode::Required);
    assert_eq!(strict_lane_mode(".DAT"), StrictLaneMode::Required);
    assert_eq!(
        strict_lane_mode(".parquet"),
        StrictLaneMode::StrictEquivalentRequired
    );
}

#[test]
fn pl14s_contract_conformance_classifies_candidate_source_provenance() {
    assert_eq!(
        candidate_source_class(".dat", false),
        CandidateSourceClass::NativeRuntimeDat
    );
    assert_eq!(
        candidate_source_class(".dat", true),
        CandidateSourceClass::ConversionDerivedDat
    );
    assert_eq!(
        candidate_source_class(".parquet", false),
        CandidateSourceClass::NativeRuntimeParquet
    );
}

#[test]
fn pl14s_contract_conformance_requires_conversion_dat_row_consistency_for_evidence_readiness() {
    let collapsed = conversion_derived_dat_row_consistency_verdict(1, 1, 0);
    assert_eq!(collapsed, ConversionSourceRowConsistencyVerdict::Hold);

    let aligned = conversion_derived_dat_row_consistency_verdict(2, 0, 0);
    assert_eq!(aligned, ConversionSourceRowConsistencyVerdict::Ready);
}

#[test]
fn pl14s_contract_conformance_rejects_duplicate_row_keys_in_semantic_lane_inputs() {
    let temp_dir = fixture_temp_dir("pl14s_duplicate_keys");
    fs::create_dir_all(&temp_dir).expect("temporary directory should be creatable");

    let baseline_wat = temp_dir.join("baseline.wat.dat");
    let candidate_wat = temp_dir.join("candidate.wat.dat");
    let report_path = temp_dir.join("semantic_report.json");

    let baseline_payload = format!("{}\n{}\n", dat_row(1, 1, 2008, 10), dat_row(1, 1, 2008, 20));
    let candidate_payload = format!("{}\n", dat_row(1, 1, 2008, 30));

    fs::write(&baseline_wat, baseline_payload).expect("baseline fixture should be writable");
    fs::write(&candidate_wat, candidate_payload).expect("candidate fixture should be writable");

    let script_path = repo_root()
        .join("tools")
        .join("legacy_comparison_suite")
        .join("semantic_hillslope_wat_compare.py");

    let output = Command::new("python3")
        .current_dir(repo_root())
        .arg(script_path)
        .arg("--baseline-wat")
        .arg(&baseline_wat)
        .arg("--candidate-wat")
        .arg(&candidate_wat)
        .arg("--report-json")
        .arg(&report_path)
        .output()
        .expect("python3 should run semantic comparator");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "duplicate row keys must hard-fail semantic comparator"
    );
    assert!(
        stderr.contains("duplicate row key"),
        "error stream should mention duplicate row key; stderr={stderr}"
    );

    let _ = fs::remove_dir_all(&temp_dir);
}
