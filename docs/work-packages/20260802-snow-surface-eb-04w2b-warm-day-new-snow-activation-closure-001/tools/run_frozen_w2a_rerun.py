#!/usr/bin/env python3
"""Rerun the frozen EB-04W2A contrast under the EB-04W2B corrected consumer."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import subprocess
import sys
from pathlib import Path


PACKAGE = Path(__file__).resolve().parents[1]
REPO = PACKAGE.parents[2]
W2A_TOOL = REPO / (
    "docs/work-packages/20260802-snow-surface-eb-04w2a-"
    "residual-melt-chronology-attribution-001/tools/run_melt_chronology_diagnostic.py"
)
OUTPUT = REPO / "target/snow_surface_eb04w2b_frozen_w2a_rerun"
SOURCE_PATHS = (
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs",
    "crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs",
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs",
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs",
    "crates/openwepp-runner/src/hillslope/snowbench.rs",
    "crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs",
    "crates/openwepp-runner/src/hillslope/03_tests.rs",
    "crates/openwepp-runner/src/hillslope/tests03/eb04w2b_warm_snow.rs",
    "tests/integration/snow_surface_eb04w_accumulation_melt_diagnostics_contract.rs",
    "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md",
)


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def load_w2a():
    spec = importlib.util.spec_from_file_location("eb04w2b_frozen_w2a", W2A_TOOL)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {W2A_TOOL}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def main() -> int:
    w2a = load_w2a()
    w2a.PACKAGE = PACKAGE
    w2a.ARTIFACTS = PACKAGE / "artifacts"
    w2a.FIGURES = w2a.ARTIFACTS / "figures"
    w2a.OUTPUT = OUTPUT
    w2a.RUNS = OUTPUT / "runs"
    w2a.FREEZE = w2a.ARTIFACTS / "frozen-w2a-rerun-freeze.json"
    w2a.RECEIPT = w2a.ARTIFACTS / "frozen-w2a-rerun-receipt.json"
    w2a.RESULTS = w2a.ARTIFACTS / "frozen-w2a-rerun-results.json"
    w2a.SUMMARY = w2a.ARTIFACTS / "frozen-w2a-rerun-summary.csv"

    w2a.self_check()
    freeze = w2a.freeze()
    diff = subprocess.check_output(["git", "diff", "--binary", "HEAD"], cwd=REPO)
    source_file_sha256 = {
        path: sha256_bytes((REPO / path).read_bytes()) for path in SOURCE_PATHS
    }
    source_manifest = json.dumps(
        source_file_sha256, sort_keys=True, separators=(",", ":")
    ).encode("utf-8")
    freeze.update(
        {
            "w2b_package": str(PACKAGE.relative_to(REPO)),
            "w2b_wrapper_sha256": sha256_bytes(Path(__file__).read_bytes()),
            "source_dirty_diff_sha256": sha256_bytes(diff),
            "source_file_sha256": source_file_sha256,
            "source_file_manifest_sha256": sha256_bytes(source_manifest),
            "source_identity_note": (
                "HEAD plus the dirty tracked diff and an explicit hash manifest "
                "covering every result-affecting EB-04W2B source, including untracked files"
            ),
            "inherited_rules": "exact EB-04W2A cells, models, operators, and thresholds",
        }
    )
    w2a.write_json(w2a.FREEZE, freeze)
    w2a.execute(4)
    result = w2a.analyze()
    summary = {
        "schema": "snow-surface-eb04w2b-rerun-summary-v1",
        "freeze_sha256": w2a.sha256(w2a.FREEZE),
        "receipt_sha256": w2a.sha256(w2a.RECEIPT),
        "results_sha256": w2a.sha256(w2a.RESULTS),
        "maximum_mass_closure_m": result["maximum_mass_closure_m"],
        "maximum_energy_closure_j_m2": result["maximum_energy_closure_j_m2"],
        "result": result,
    }
    (PACKAGE / "artifacts/frozen-w2a-rerun-adjudication.json").write_text(
        json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(
        "EB-04W2B frozen W2A rerun: PASS "
        f"mass_closure={result['maximum_mass_closure_m']:.3e} m"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
