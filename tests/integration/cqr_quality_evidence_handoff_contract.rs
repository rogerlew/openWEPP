use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn scratch(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "openwepp-cqr-quality-{label}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir(&path).expect("create scratch");
    path
}

#[test]
fn cqr_quality_evidence_self_test_passes() {
    let output = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/cqr_quality_evidence.py"))
        .arg("self-test")
        .current_dir(root())
        .output()
        .expect("run CQR quality evidence self-test");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn missing_evidence_is_typed_invalid_without_collection() {
    let temp = scratch("missing");
    let output_path = temp.join("intake.json");
    let status = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/cqr_quality_evidence.py"))
        .args(["inspect", "--repo", "."])
        .arg("--published-dir")
        .arg(temp.join("absent-published"))
        .arg("--control-receipt")
        .arg(temp.join("absent-control/quality-control-receipt.json"))
        .args([
            "--expected-id",
            "0000000000000000000000000000000000000000000000000000000000000000",
        ])
        .arg("--output")
        .arg(&output_path)
        .current_dir(root())
        .status()
        .expect("run missing evidence intake");
    assert!(!status.success());
    let receipt = fs::read_to_string(output_path).expect("read invalid receipt");
    assert!(receipt.contains("\"disposition\":\"INVALID\""));
    assert!(receipt.contains("\"collection_launched\":false"));
    assert!(receipt.contains("\"selection\":null"));
    fs::remove_dir_all(temp).expect("remove scratch");
}

#[test]
fn recollection_requires_typed_noncurrent_receipt_and_explicit_directive() {
    let temp = scratch("recollect");
    let receipt = temp.join("intake.json");
    let inspected = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/cqr_quality_evidence.py"))
        .args(["inspect", "--repo", "."])
        .arg("--published-dir")
        .arg(temp.join("missing-published"))
        .arg("--control-receipt")
        .arg(temp.join("missing-control/quality-control-receipt.json"))
        .args([
            "--expected-id",
            "0000000000000000000000000000000000000000000000000000000000000000",
        ])
        .arg("--output")
        .arg(&receipt)
        .current_dir(root())
        .status()
        .expect("create retained invalid intake");
    assert!(!inspected.success());
    let authorization = temp.join("authorization.json");
    let accepted = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/cqr_quality_evidence.py"))
        .arg("authorize-recollection")
        .arg("--intake-receipt")
        .arg(&receipt)
        .args(["--operator-directive", "execute cqr nightly for 3 modules"])
        .arg("--output")
        .arg(&authorization)
        .current_dir(root())
        .status()
        .expect("authorize explicit recollection");
    assert!(accepted.success());
    let authorization_text = fs::read_to_string(&authorization).expect("read authorization");
    assert!(authorization_text.contains("\"status\":\"AUTHORIZED\""));

    let alias_rejected = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/cqr_quality_evidence.py"))
        .arg("authorize-recollection")
        .arg("--intake-receipt")
        .arg(&receipt)
        .args(["--operator-directive", "execute cqr nightly"])
        .arg("--output")
        .arg(&receipt)
        .current_dir(root())
        .status()
        .expect("reject intake receipt overwrite");
    assert!(!alias_rejected.success());

    let mut forged: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&receipt).expect("read receipt"))
            .expect("parse receipt");
    forged["reasons"] = serde_json::json!(["forged reason"]);
    let forged_receipt = temp.join("forged-intake.json");
    let mut forged_bytes = serde_json::to_vec(&forged).expect("encode forged receipt");
    forged_bytes.push(b'\n');
    fs::write(&forged_receipt, forged_bytes).expect("write forged receipt");
    let forgery_rejected = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/cqr_quality_evidence.py"))
        .arg("authorize-recollection")
        .arg("--intake-receipt")
        .arg(&forged_receipt)
        .args(["--operator-directive", "execute cqr nightly"])
        .arg("--output")
        .arg(temp.join("forged-authorization.json"))
        .current_dir(root())
        .status()
        .expect("reject forged receipt");
    assert!(!forgery_rejected.success());

    let rejected = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/cqr_quality_evidence.py"))
        .arg("authorize-recollection")
        .arg("--intake-receipt")
        .arg(&receipt)
        .args(["--operator-directive", "please refresh metrics"])
        .arg("--output")
        .arg(temp.join("rejected.json"))
        .current_dir(root())
        .status()
        .expect("reject vague recollection");
    assert!(!rejected.success());
    fs::remove_dir_all(temp).expect("remove scratch");
}

#[test]
fn source_and_execplan_forbid_silent_recollection() {
    let source = fs::read_to_string(root().join("tools/local_ci/cqr_quality_evidence.py"))
        .expect("read intake source");
    for required in [
        "quality.verify_published(",
        "reconstruct_selection(",
        "\"collection_launched\": False",
        "receipt.get(\"disposition\") not in {\"STALE\", \"INVALID\"}",
        "current registry does not reconstruct report partitions",
        "expected evidence ID differs from canonical payload",
        "build_intake_fixture(",
        "selection_review_status",
        "retained intake receipt did not reproduce exactly",
    ] {
        assert!(
            source.contains(required),
            "missing intake guard: {required}"
        );
    }
    for forbidden in [
        "cargo llvm-cov",
        "cargo crap",
        "quality_observatory.py transition",
    ] {
        assert!(
            !source.contains(forbidden),
            "intake source contains collection command: {forbidden}"
        );
    }
    let plan =
        fs::read_to_string(root().join("docs/work-packages/cqr-nightly-burndown-execplan.md"))
            .expect("read CQR plan");
    assert!(plan.contains("cqr_quality_evidence.py"));
    assert!(plan.contains("typed `STALE` or `INVALID`"));
}
