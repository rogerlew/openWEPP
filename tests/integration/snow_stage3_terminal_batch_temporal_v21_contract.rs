use std::fs;
use std::process::Command;

const REJECTED_V21_CHECKPOINT: &str = "e3b9e20eebbf5ecd319c372c3d31b1a05a2479d7";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn read_rejected_contract(path: &str) -> String {
    let object = format!("{REJECTED_V21_CHECKPOINT}:{path}");
    let output = Command::new("git")
        .args(["show", &object])
        .output()
        .unwrap_or_else(|error| panic!("failed to run git show {object}: {error}"));
    assert!(
        output.status.success(),
        "failed to resolve preserved candidate object {object}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap_or_else(|error| panic!("preserved candidate object {object} is not UTF-8: {error}"))
}

#[test]
fn corrected_successors_bind_estimator_scc_forcing_and_solver_authority() {
    let snow = read_rejected_contract(
        "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md",
    );
    let lse = read_rejected_contract(
        "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md",
    );
    let freeze = read_rejected_contract(
        "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    );
    let graph = read(
        "docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/artifacts/terminal-same-support-scc-and-forcing-inventory-v2.md",
    );

    for required in [
        "candidate_contract_version: 21",
        "(X_F-X_C)/3",
        "Install only `X_F`",
        "0.6<=h<1.2 s",
        "J_H e_H=-d_H",
        "gamma=2",
        "Acceptance is exactly `E<=1`",
        "clamp(0.5,2.0,0.8*E^(-1/3))",
        "quantize by floor to an integer nanosecond",
        "Maximum rejected attempts per\nabsolute prefix is `16`",
        "Damped Newton uses the exact AD\nJacobian",
        "Armijo constant `1e-4`",
        "backtracking multiplier `0.5`",
        "exactly one equivalence class may pass",
        "INV-SNOWENERGY-061",
        "INV-SNOWENERGY-062",
        "INV-SNOWENERGY-063",
        "INV-SNOWENERGY-064",
        "INV-SNOWENERGY-065",
    ] {
        assert!(snow.contains(required), "SnowEnergy v21 missing {required}");
    }

    for required in [
        "candidate_contract_version: 11",
        "PrescribedAmount",
        "ArmGeneratedAmount",
        "StateDependentRate",
        "Incident provider radiation",
        "Absorbed snow/canopy shortwave",
        "throughfall, drainage and stemflow",
        "CoveredLseRateEvaluationReceiptV2",
        "CoveredLseAmountSetReceiptV2",
        "INV-LANDSURFACEENERGY-134",
        "INV-LANDSURFACEENERGY-137",
    ] {
        assert!(lse.contains(required), "LSE v11 missing {required}");
    }

    for required in [
        "candidate_contract_version: 139",
        "implicit SCC is\nsnow, vegetation/LSE/shared carrier",
        "BGC\nis an exact follower only",
        "excluded by exact zero cardinality",
        "surface-liquid ingress set",
        "INV-SNOWFREEZE-111",
        "INV-SNOWFREEZE-114",
    ] {
        assert!(
            freeze.contains(required),
            "SnowFreeze v139 missing {required}"
        );
    }

    for required in [
        "TBTV20-NUM-006",
        "Strongly connected\ncomponents are solved together",
        "implicit physical SCC",
        "prescribed upstream amount",
        "arm-generated amount",
        "state-dependent endpoint/collocation rate",
        "Terminal liquid remains a\nsnow-owned discrete output",
    ] {
        assert!(graph.contains(required), "SCC inventory missing {required}");
    }
}

#[test]
fn corrected_coupled_time_defines_every_wire_node_and_split_owner_join() {
    let time = read_rejected_contract(
        "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    );

    for required in [
        "candidate_contract_version: 6",
        "covered-terminal-batch-request-v3",
        "covered-terminal-prescribed-amount-set-v1",
        "covered-terminal-rate-evaluation-set-v1",
        "covered-terminal-generated-amount-set-v1",
        "covered-terminal-temporal-arm-v1",
        "covered-terminal-high-estimator-v1",
        "covered-terminal-batch-result-v3",
        "covered-terminal-ending-joint-v3",
        "covered-terminal-lane-event-v2",
        "covered-terminal-group-topology-v3",
        "covered-terminal-positive-prefix-owner-v1",
        "covered-terminal-cursor-event-intent-v1",
        "covered-terminal-zero-prefix-receipt-v2",
        "stage3-v11-terminal-event-proposal-core-v2",
        "stage3-v11-terminal-group-preaccept-v2",
        "stage3-v11-terminal-group-accepted-v2",
        "openwepp-stage3-terminal-parcel-set-v2",
        "covered-terminal-parent-chain-v2",
        "O_begin -> O_prefix",
        "O_prefix -> O_event",
        "no-terminal-ingress receipt",
        "INV-COUPLEDTIME-032",
        "INV-COUPLEDTIME-036",
    ] {
        assert!(time.contains(required), "CoupledTime v6 missing {required}");
    }
}

#[test]
fn rejected_trial_diagnostic_gate_is_read_only_and_preimplementation() {
    let gate = read(
        "docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/artifacts/terminal-rejected-trial-diagnostic-authority-mini-gate.md",
    );
    for required in [
        "EVIDENCE-ONLY / NO IMPLEMENTATION AUTHORITY",
        "cfg(test)`-only observer",
        "returns `()`",
        "cannot return failure/control information",
        "cannot suppress, replace or translate `BelowCarrierDomain`",
        "terminal-rejected-trial-evidence-v1",
        "zero calls occur below the floor",
        "27.2131278332233 J m^-2",
        "Two independent reviews are required",
        "Either HOLD stops before diagnostic implementation",
    ] {
        assert!(
            gate.contains(required),
            "diagnostic gate missing {required}"
        );
    }
}
