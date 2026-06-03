# Gate Results

Status: completed/HOLD
Evidence mode: ran

Ran:

| Gate | Result | Notes |
|---|---:|---|
| `python3 -m py_compile docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/artifacts/hphys0270_diagnostics.py` | 0 | Diagnostics script syntax valid. |
| `cargo fmt --check` | 0 | Formatting clean after `cargo fmt`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | No clippy warnings. |
| `cargo test -p openwepp-runner hphys0270_trace_row_captures_pre_day_snowpack_state --lib -- --nocapture` | 0 | New regression passed. |
| `cargo test -p openwepp-runner hphys02 --lib -- --nocapture` | 0 | 38 HPHYS runner tests passed. |
| `bash tools/release/check_authority_suite_antievasion.sh` | 0 | Authority anti-evasion guard passed. |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | 0 | 2 tests passed. |
| `.venv/bin/python .../hphys0270_diagnostics.py --run-root /tmp/hphys0270_full_20260603T201051Z --trace-max-days 180` | 0 | Targeted traces and full 39-suite ran. |
| `cargo test --workspace` | 101 | Two existing SIMIMPL18 fixture tests fail with `HKERNEL-WB11-ET-E-003`; unrelated to HPHYS0270 trace-only changes. |
| `cargo deny check` | 0 | Passed with existing duplicate-crate and unmatched-license-allowance warnings. |
| `markdown-doc lint --path docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001 --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/specifications/science-contracts/index.md` | 0 | 26 files validated, 0 errors, 0 warnings. |

Static:

- The only red gate is the known workspace SIMIMPL18 fixture ET-domain failure. HPHYS0270-specific and contract guard gates passed.
