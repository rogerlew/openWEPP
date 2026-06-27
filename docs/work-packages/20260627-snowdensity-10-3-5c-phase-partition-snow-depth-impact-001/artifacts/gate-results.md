# Gate Results

Evidence mode: Static/Ran.

| Gate | Status | Evidence |
|---|---:|---|
| Real direct-production WAT path, not snowbench | PASS | Tool uses `openwepp-cli-hill --direct-production-executor`; guard test asserts no `coe-melt`. |
| Default selector absent / opt-in selector set | PASS | Report records absent default and `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=harder_pomeroy_hourly` opt-in commands. |
| Paired residual summaries for all paired surfaces | PASS | `phase-partition-snowdepth-impact.json` and `.md`. |
| Observation-blocked surfaces excluded from defect verdicts | PASS | HJ Andrews and Hubbard Brook surfaces are `OBSERVATION-BLOCKED-DIAGNOSTIC-ONLY`. |
| Valid-input solver robustness blocker resolved | PASS | Bracketing fallback added; focused meteorology test passes; full WAT batch completes. |
| No protected-boundary creep | PASS | `no-scope-creep-scan.md`. |
| `.venv/bin/python tools/snowfreeze_observed/phase_partition_snowdepth_adjudication.py` | PASS | Completed all WAT runs. |
| `cargo test --test snowdensity10_3_5c_phase_partition_snowdepth_impact` | PASS | `4 passed`. |
| `cargo fmt --check` | PASS | Ran before focused tests and before clippy. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Initial run caught `manual-let-else`; code fixed and rerun passed. |
| `cargo test --workspace` | PASS | Full workspace test suite passed. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `wctl doc-lint --path docs/work-packages` | PASS | `971 files validated, 0 errors, 0 warnings`. |

## Candidate Disposition

`harder_pomeroy_hourly` remains opt-in only and is not a snow-depth promotion
candidate. The next route is the 10.3.4 rank-2 winter-thaw melt response.
