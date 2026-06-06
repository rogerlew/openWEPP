# Gate Results

Status: complete

Evidence mode: ran

Ran:

- `cargo fmt --check`: pass.
- `python -m py_compile docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/artifacts/hphys0308_snowd_branch_state_ordering.py`:
  pass.
- `cargo test --test hphys0308_snowd_branch_state_ordering_contract -- --nocapture`:
  initial run failed on missing package wording for `snow_hourly_depth_before_m`;
  patched and rerun.
- `cargo test --test hphys0308_snowd_branch_state_ordering_contract -- --nocapture`:
  pass, `5` tests.
- `bash tools/release/check_authority_suite_antievasion.sh`: pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`:
  pass, `2` tests.
- `cargo test --test hphys0307_melt_call_branch_activation_contract -- --nocapture`:
  pass, `5` tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass with warnings for duplicate `getrandom`,
  `hashbrown`, `twox-hash` and unmatched license allowances already present in
  `deny.toml`.
- `git diff --check`: pass.
