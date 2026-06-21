# Verification B

Evidence mode: Static + Ran.

Ran:

- `cargo test -p openwepp-runner r6_direct_publication_cutover_cli_flag_fails_closed_before_outputs --test r6_direct_publication_cutover_cli_contract -- --nocapture` -> PASS.
- no-compatibility source scans recorded in
  `artifacts/no-compatibility-proof-checklist.md` -> PASS.
- `wc -l crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs` -> PASS for tests, FAIL/HOLD for monolithic runner line-count threshold.
- `cargo clippy --workspace --all-targets -- -D warnings` -> PASS.
- `cargo test --workspace` -> PASS.
- `cargo deny check` -> PASS.
- `wctl doc-lint --path docs/work-packages` -> PASS.
- `git diff --check` -> PASS.

Verified:

- CLI cutover flag fails closed with the R6D hold marker;
- cutover branch consumes retained frame and contains no skeleton capture calls;
- retained producer functions contain no forbidden compatibility-source reads;
- line-count governance remains a documented closure blocker.
