# Gate Results

Evidence mode: Ran.

| Gate | Result | Evidence |
|---|---|---|
| Build snowbench binary | PASS | `cargo build -p openwepp-runner --bin openwepp-snowbench` |
| Build coupled WAT binary | PASS | `cargo build -p openwepp-runner --bin openwepp-cli-hill` |
| Diagnostic rerun | PASS | `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_correction.py` |
| Diagnostic report regeneration | PASS | `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_correction.py --skip-runs` after summarizer correction |
| Coupled WAT rerun | PASS | `.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py` |
| Focused package test | PASS | `cargo test --test snowdensity10_3_7_winter_thaw_melt_response_correction -- --nocapture` |
| Format check | PASS | `cargo fmt --check` |
| Clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings` |
| Workspace tests | PASS | `cargo test --workspace` |
| Dependency/license/advisory gate | PASS | `cargo deny check` |
| Work-package docs lint | PASS | `wctl doc-lint --path docs/work-packages` (`971 files validated, 0 errors, 0 warnings`) |

Package diagnostic summary:

```json
{
  "aggregate_depth_loss_deficit_delta_m": -6.476198898410999,
  "disposition": "WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES",
  "schema": "snowdensity10-3-7-winter-thaw-melt-response-correction-v1",
  "under_ablation_delta": -24
}
```

Conservation/coupled summary:

```json
{
  "candidate_conservation_passed": true,
  "coupled_disposition": "WINTER-THAW-COUPLED-WAT-IMPROVES",
  "coupled_fail_delta_default_minus_candidate": 169,
  "coupled_no_worse_gate_passed": true,
  "remaining_coupled_blocker": "SNOW-CONTROL-NOT-CLEARED"
}
```
