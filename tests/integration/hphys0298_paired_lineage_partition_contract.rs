use std::fs;
use std::process::Command;

const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_RUNOFFPART: &str = "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/prompts/active/hphys0298_kickoff_agent_prompt.md";
const HARNESS: &str = "docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py";

#[test]
fn hphys0298_contracts_require_paired_lineage_partition() {
    let snow = fs::read_to_string(SC_SNOWFREEZE).expect("snow contract should be readable");
    let runoff = fs::read_to_string(SC_RUNOFFPART).expect("runoff contract should be readable");
    let watbal = fs::read_to_string(SC_WATBAL).expect("watbal contract should be readable");

    assert!(
        snow.contains("INV-SNOWFREEZE-029")
            && snow.contains("paired snow/`RM` lineage partition invariant")
            && snow.contains("/workdir/wepp-forest_260430_baseline")
            && snow.contains("baseline observe identity")
            && snow.contains("winter-gate -> hourly-forcing -> raw-hourly-melt")
            && snow.contains("post-winter-wmelt-rain-flags")
            && snow.contains("WB17-WB18-WB19-storage-consumers")
            && snow.contains("`snodpy`, `frdp`, `rain`, `wmelt`, `hrmlt`")
            && snow.contains("`hrrain`, `hrsnow`, `pstvML`, `ngtvML`, `pstvhr`")
            && snow.contains("`LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`, or `UNRESOLVED`"),
        "SC-SNOWFREEZE must encode HPHYS0298 paired first-divergence authority"
    );
    assert!(
        snow.contains("HPHYS0299 supersedes the direct migration inference")
            && snow.contains("snow_hourly_snowfall_water_equiv_sum_m")
            && snow.contains("not canonical `hrsnow` parity evidence")
            && snow.contains("winter.for:410-412")
            && snow.contains("wepp_260430_hill")
            && snow.contains("supersedes any stale assertion that no reference binary exists"),
        "SC-SNOWFREEZE must record HPHYS0299 supersession and corrected hrsnow authority"
    );
    for window in [
        "H1 2013 112-127",
        "H1 2014 120-146",
        "H1 2016 104-111",
        "H7 2013 112-127",
        "H7 2014 120-146",
        "H7 2016 104-111",
        "H39 2013 97-112",
        "H39 2014 120-146",
        "H39 2016 104-111",
    ] {
        assert!(
            snow.contains(window),
            "canonical SC-SNOWFREEZE authority must name target window {window}"
        );
    }
    assert!(
        runoff.contains("INV-RUNOFFPART-026")
            && runoff.contains("runoff-consumer paired-partition invariant")
            && runoff.contains("closed `Q`")
            && runoff.contains("cannot convert downstream closure into snow/`RM` acceptance"),
        "SC-RUNOFFPART must keep closed runoff identity as context, not compensation"
    );
    assert!(
        watbal.contains("INV-WATBAL-073")
            && watbal.contains("paired snow/`RM` source-partition invariant")
            && watbal.contains("full H1..H39 same-HEAD metrics")
            && watbal.contains("Observe-identity failure")
            && watbal.contains("canonical symbol values and units")
            && watbal.contains("baseline and openWEPP source-line provenance")
            && watbal.contains("prevents downstream WB17/WB18/WB19/WB13 compensation")
            && (watbal.contains("producer-side hourly precipitation-phase partition")
                || watbal.contains("producer-side porting-fidelity defect"))
            && (watbal.contains("not as downstream storage")
                || watbal.contains("not downstream WB17/WB18/WB19/WB13 compensation authority"))
            && watbal.contains("winter.for:410-412"),
        "SC-WATBAL must require same-HEAD metrics, observe identity, and no downstream compensation"
    );
}

#[test]
fn hphys0298_package_names_all_target_windows_and_outputs() {
    let package = fs::read_to_string(PACKAGE).expect("package should be readable");

    for window in [
        "`H1`, first-2013, year `2013`, days `112-127`",
        "`H1`, spring-2014, year `2014`, days `120-146`",
        "`H1`, spring-2016, year `2016`, days `104-111`",
        "`H7`, first-2013, year `2013`, days `112-127`",
        "`H7`, spring-2014, year `2014`, days `120-146`",
        "`H7`, spring-2016, year `2016`, days `104-111`",
        "`H39`, first-2013, year `2013`, days `97-112`",
        "`H39`, spring-2014, year `2014`, days `120-146`",
        "`H39`, spring-2016, year `2016`, days `104-111`",
    ] {
        assert!(package.contains(window), "missing target window {window}");
    }

    assert!(
        package.contains("artifacts/paired-observe-identity-evidence.md")
            && package.contains("artifacts/partition-ledger.md")
            && package.contains("artifacts/full-39-suite-metrics.md")
            && package.contains("HPHYS0299 supersedes the HPHYS0298")
            && package.contains("review_claude_hrsnow_unit_artifact.md")
            && package.contains("Do not reproduce the pinned-baseline negative-melt")
            && package.contains("Do not reintroduce parser compatibility for `wepp_observe*`")
            && package
                .contains("Dual review, disposition, and verification artifacts are complete"),
        "HPHYS0298 package must define required evidence outputs and safety constraints"
    );
}

#[test]
fn hphys0298_prompt_is_end_to_end_and_baseline_scoped() {
    let prompt = fs::read_to_string(PROMPT).expect("kickoff prompt should be readable");

    assert!(
        prompt.contains("Execution mode: package-end-to-end (default)")
            && prompt.contains("Autonomy: execute package phases end-to-end")
            && prompt.contains("/workdir/wepp-forest_260430_baseline")
            && prompt.contains("dac3c950d8b16cc73774bf5ce2e7e11f80baac70")
            && prompt.contains("baseline")
            && prompt.contains("observe identity")
            && prompt.contains("no downstream WB17/WB18/WB19/WB13")
            && prompt.contains("dual review artifacts")
            && prompt.contains("verification artifacts"),
        "HPHYS0298 prompt must be autonomous, baseline-scoped, and review-gated"
    );
}

#[test]
fn hphys0298_harness_checks_forcing_before_raw_melt_and_fails_closed_on_missing_trace() {
    let harness = fs::read_to_string(HARNESS).expect("harness should be readable");
    let first_divergence = harness
        .split("def first_divergence_for")
        .nth(1)
        .and_then(|tail| tail.split("def first_divergent_symbols").next())
        .expect("first-divergence function should be extractable");
    let forcing_pos = first_divergence
        .find("baseline_raw_snow_minus_openwepp_raw_snow_mm")
        .expect("forcing snow delta check should exist");
    let raw_melt_pos = first_divergence
        .find("baseline_raw_melt_minus_openwepp_raw_melt_mm")
        .expect("raw melt delta check should exist");
    assert!(
        forcing_pos < raw_melt_pos,
        "hourly forcing checks must precede raw hourly melt checks"
    );
    assert!(
        harness.contains("REQUIRED_OPENWEPP_TRACE_FIELDS")
            && harness.contains("openwepp_trace_missing_field_count")
            && harness.contains("trace-gap"),
        "harness must fail closed when required paired openWEPP trace fields are missing"
    );
    assert!(
        harness.contains("UnitPairingEvidenceError")
            && harness.contains("snow_hourly_snowfall_depth_sum_m")
            && harness.contains("snow_hourly_snowfall_water_equiv_sum_m")
            && harness.contains("HPHYS0299 corrected depth-vs-depth"),
        "harness must fail closed on the historical HPHYS0298 depth-vs-water-equivalent pairing"
    );
}

#[test]
fn hphys0298_harness_rejects_historical_hrsnow_water_equiv_pairing() {
    let output = Command::new(".venv/bin/python")
        .arg(HARNESS)
        .arg("--run-root")
        .arg("/tmp/hphys0298_unit_guard")
        .arg("--skip-full-suite")
        .arg("--skip-targeted-traces")
        .arg("--skip-baseline-observe")
        .output()
        .expect("harness should execute far enough to reject unit pairing");

    assert!(
        !output.status.success(),
        "historical HPHYS0298 harness must fail closed instead of regenerating a depth-vs-water-equivalent verdict"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("snow_hourly_snowfall_water_equiv_sum_m")
            && stderr.contains("snow_hourly_snowfall_depth_sum_m")
            && stderr.contains("HPHYS0299 corrected depth-vs-depth"),
        "unit guard stderr must point to the bad water-equivalent field and corrected HPHYS0299 depth surface; stderr: {stderr}"
    );
}
