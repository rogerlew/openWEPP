# Gate Results

Evidence class: Ran.

## Focused Checks

- `cargo check -q -p openwepp-hillslope-orchestrator`: passed.
- `cargo check -q -p openwepp-runner`: passed.
- `cargo test --test snowdensity05d_opt_in_coe_melt -- --nocapture`: passed.
- SNOWDENSITY focused regression set: passed.

## Required Gates

- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed.
- `bash tools/release/check_authority_suite_antievasion.sh`: passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract`: passed.

## Honest Intermediate Failure

The first full `cargo test --workspace` run exposed a stale
`SC-SNOWFREEZE-001` version assertion in
`tests/integration/snowdensity02_contract_adr_guard.rs`. The test was updated
to v79, and the full workspace suite was rerun successfully.
