# Gate Results

| Gate | Result | Evidence |
|---|---|---|
| Python compile | PASS | `.venv/bin/python -m py_compile tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py` |
| Focused source guard | PASS | `cargo test --test snowdensity10_3_18_cross_snotel_mechanism_rubric -- --nocapture` (`2 passed`) |
| Formatting | PASS | `cargo fmt --check` |
| Real diagnostic run | PASS | `.venv/bin/python tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py` |
| Reuse/regeneration path | PASS | `.venv/bin/python tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py --skip-model-runs` |
| Matrix emitted | PASS | `1280` per-site x model x signature rows in `cross-snotel-mechanism-rubric.json` |
| No promotion/activation decision | PASS | Summary reports `activation_authorized=false`, `promotion_decision_made=false` |

Workspace-wide `cargo test --workspace` and `cargo deny check` were not rerun
for this diagnostic-only Python/package addition.
