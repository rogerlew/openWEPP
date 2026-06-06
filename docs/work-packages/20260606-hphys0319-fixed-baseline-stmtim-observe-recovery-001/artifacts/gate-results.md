# Gate Results

Status: complete

Evidence mode: Ran

Ran:

| Gate | Command | Result |
|---|---|---|
| Pre-implementation contract gate | `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract hphys0319_contract_authority_is_registered -- --nocapture` | Passed; exit status was `0`. |
| Recovery script syntax | `.venv/bin/python -m py_compile docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/artifacts/hphys0319_fixed_stmtim_observe.py` | Passed; exit status was `0`. |
| Recovery script scope test | `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract hphys0319_recovery_script_is_scoped_and_records_required_tags -- --nocapture` | Passed; exit status was `0`. |
| Fixed-baseline observe recovery and OpenWEPP trace regeneration | `.venv/bin/python docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/artifacts/hphys0319_fixed_stmtim_observe.py` | Passed after replacing the initial direct `cupdate.inc` include with observe-only `year`/`sdate` arguments; final exit status was `0`. |
| HPHYS0319 package test | `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Passed; exit status was `0`. |
| Formatting | `cargo fmt --check` | Initial check failed on new HPHYS0319 test formatting; `cargo fmt` applied formatting and rerun passed with exit status `0`. |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Passed; exit status was `0`. |
| Cargo deny | `cargo deny check` | Passed; exit status was `0`; existing duplicate/unmatched-license warnings only. |
| Authority anti-evasion | `bash tools/release/check_authority_suite_antievasion.sh` | Passed; exit status was `0`. |
| AUTH11 required-suite guards | `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Passed; exit status was `0`. |
| Markdown/package lint | `markdown-doc lint --path docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/index.md --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | Passed; `28` files validated; exit status was `0`. |
| Workspace tests | `cargo test --workspace` | Passed; exit status was `0`. |
| Final diff hygiene | `git diff --check` | Passed; exit status was `0`. |
