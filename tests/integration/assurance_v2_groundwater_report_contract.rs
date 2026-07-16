use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

const REPORT_ROOT: &str = "assurance/v2/reports/linear-groundwater-reservoir-recurrence";

#[test]
fn independent_analytical_procedure_reproduces_retained_result_exactly() {
    let root = repository_root();
    let output = Command::new("python3")
        .arg("-B")
        .arg(
            root.join(REPORT_ROOT)
                .join("procedures/reproduce_groundwater_report.py"),
        )
        .arg("analytical")
        .arg("--input")
        .arg(
            root.join(REPORT_ROOT)
                .join("inputs/two-day-recurrence-input.json"),
        )
        .output()
        .expect("python3 should execute the public-safe analytical procedure");
    assert!(
        output.status.success(),
        "analytical procedure failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let actual: Value = serde_json::from_slice(&output.stdout).expect("procedure JSON");
    let expected = json_value(
        &root
            .join(REPORT_ROOT)
            .join("results/two-day-recurrence.json"),
    );
    assert_eq!(
        actual, expected,
        "retained result must be procedure-derived"
    );
}

#[test]
fn retained_h2637_values_close_both_groundwater_and_surface_ledgers() {
    let root = repository_root();
    let result = json_value(&root.join(REPORT_ROOT).join("results/h2637-ledger.json"));
    let output = Command::new("python3")
        .arg("-B")
        .arg(
            root.join(REPORT_ROOT)
                .join("procedures/reproduce_groundwater_report.py"),
        )
        .arg("h2637")
        .arg("--manifest")
        .arg(root.join(REPORT_ROOT).join("evidence/h2637/manifest.json"))
        .arg("--hbp")
        .arg(root.join(REPORT_ROOT).join("evidence/h2637/H2637.hbp"))
        .arg("--parquet")
        .arg(
            root.join(REPORT_ROOT)
                .join("evidence/h2637/H2637.pass.parquet"),
        )
        .output()
        .expect("python3 should execute the retained H2637 procedure");
    assert!(
        output.status.success(),
        "H2637 procedure failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reproduced: Value = serde_json::from_slice(&output.stdout).expect("H2637 procedure JSON");
    assert_eq!(
        reproduced, result,
        "retained raw evidence must reproduce result"
    );
    let values = numeric_values(&result);

    let initial = values["initial_storage"];
    let recharge = values["cumulative_recharge"];
    let baseflow = values["cumulative_baseflow"];
    let seepage = values["cumulative_deep_seepage"];
    let terminal = values["terminal_pre_export_storage"];
    let terminal_baseflow = values["terminal_day_baseflow"];
    let terminal_seepage = values["terminal_day_deep_seepage"];

    let recurrence =
        initial + recharge - (baseflow - terminal_baseflow) - (seepage - terminal_seepage);
    let recurrence_residual = terminal - recurrence;
    assert_close(
        recurrence,
        values["recurrence_reconstructed_storage"],
        "recurrence reconstruction",
    );
    assert_close(
        recurrence_residual,
        values["recurrence_residual"],
        "recurrence residual",
    );
    assert!(recurrence_residual.abs() <= values["recurrence_allowance"]);

    let post_export = terminal - terminal_baseflow - terminal_seepage;
    let full_run = initial + recharge - baseflow - seepage;
    let post_residual = post_export - full_run;
    assert_close(
        post_export,
        values["post_export_storage"],
        "post-export storage",
    );
    assert_close(
        full_run,
        values["full_run_reconstructed_storage"],
        "full-run reconstruction",
    );
    assert_close(
        post_residual,
        values["post_export_residual"],
        "post-export residual",
    );
    assert!(post_residual.abs() <= values["post_export_allowance"]);

    let surface = values["surface_source"]
        - values["surface_routed_outlet"]
        - values["surface_end_window_storage"]
        - values["surface_clamp"];
    assert!(surface.abs() <= values["surface_allowance"]);
    assert!(values["surface_residual"].abs() <= values["surface_allowance"]);
    assert!(
        surface.abs() / values["surface_source"]
            <= values["surface_allowance"] / values["surface_source"]
    );
    assert!(
        values["surface_relative_residual"]
            <= values["surface_allowance"] / values["surface_source"]
    );

    assert_close(values["duration_days"], 731.0, "H2637 duration");
    assert_close(values["ofe_count"], 19.0, "H2637 OFE count");
    assert_close(seepage, 0.0, "H2637's zero ks is a disclosed limitation");
}

#[test]
fn manuscript_is_science_first_and_preserves_the_claim_boundary() {
    let root = repository_root();
    let manuscript = fs::read_to_string(root.join(REPORT_ROOT).join("manuscript.md"))
        .expect("manuscript readable");

    let required_headings = [
        "## Key Findings",
        "## Plain-Language Summary",
        "## Abstract",
        "## 1. Introduction",
        "## 2. Model Formulation",
        "## 3. Materials and Methods",
        "## 4. Results",
        "## 5. Discussion",
        "## 6. Limitations",
        "## 7. Conclusions",
        "## 8. Open Research and Reproduction",
        "## References",
        "## About This Report",
    ];
    let mut previous = 0;
    for heading in required_headings {
        let position = manuscript
            .find(heading)
            .unwrap_or_else(|| panic!("missing manuscript heading {heading}"));
        assert!(position >= previous, "manuscript headings out of order");
        previous = position;
    }

    for prohibited in [
        "# CANDIDATE",
        "# INSUFFICIENT_EVIDENCE",
        "## Authorship and review disclosure",
        "nonpublic ASSURE-04A source-contract fixture",
    ] {
        assert!(
            !manuscript.contains(prohibited),
            "science-first manuscript retains prohibited status-first text: {prohibited}"
        );
    }
    for required in [
        "not performance statistics for",
        "No streamflow, groundwater-level",
        "Internal coding-agent review is not",
        "latest runoff-event HBP baseflow was not used as `Qb_N`",
    ] {
        assert!(
            manuscript.contains(required),
            "missing claim boundary: {required}"
        );
    }
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn json_value(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("JSON readable")).expect("valid JSON")
}

fn numeric_values(result: &Value) -> BTreeMap<String, f64> {
    result["values"]
        .as_array()
        .expect("result values")
        .iter()
        .map(|entry| {
            (
                entry["id"].as_str().expect("value id").to_owned(),
                entry["value"].as_f64().expect("numeric value"),
            )
        })
        .collect()
}

fn assert_close(actual: f64, expected: f64, label: &str) {
    let allowance = 1.0e-14 * actual.abs().max(expected.abs()).max(1.0);
    assert!(
        (actual - expected).abs() <= allowance,
        "{label}: actual={actual:.17e}, expected={expected:.17e}, allowance={allowance:.17e}"
    );
}
