# Gate Results

Status: complete
Evidence mode: Ran

Ran:
- `cargo fmt --check` -> pass.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass (`/tmp/hphys0287_clippy_after_review2.log`).
- `cargo test --workspace` -> pass (`/tmp/hphys0287_cargo_test_workspace_after_review2.log`).
- `cargo deny check` -> pass with existing warnings (`/tmp/hphys0287_cargo_deny_after_review2.log`).
- `bash tools/release/check_sc_unit_compliance.sh` -> fail, 219 findings (`/tmp/hphys0287_sc_unit_compliance_after_review.log`).
- Full H1..H39 release semantic suite -> runtime `39/39`, semantic reports `39/39`, semantic pass `0/39`; root `{run_root}`.

Known-open gate:
- The SC unit compliance failure is broad and pre-existing across `SC-CLIMATE`, `SC-INFILE-*`, `SC-PERC`, `SC-SNOWFREEZE`, `SC-SOIL`, `SC-SUBHYD`, and `SC-WATBAL`.
- HPHYS0287 does not own the global SC unit registry/alias backlog. This is explicitly deferred under `executed-hold`, not accepted as complete contract-profile closure.

Disposition:
- Required Rust gates pass after reviewer-driven corrections.
- Global SC unit compliance remains a governance continuation item.
