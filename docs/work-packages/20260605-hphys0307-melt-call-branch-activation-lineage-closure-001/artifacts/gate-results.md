# Gate Results

Status: complete

Evidence mode: ran

Ran:

- `cargo fmt --check`: initial run failed on a new-test formatting issue; the
  issue was patched and the gate was rerun.
- `cargo fmt --check`: pass after one formatting patch.
- `python -m py_compile docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/artifacts/hphys0307_melt_call_branch_activation.py`:
  pass.
- `cargo test --test hphys0307_melt_call_branch_activation_contract -- --nocapture`:
  pass, `5` tests.
- `bash tools/release/check_authority_suite_antievasion.sh`: pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`:
  pass, `2` tests.
- `cargo test --test hphys0306_baseline_melt_observe_semantics_contract -- --nocapture`:
  pass, `4` tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass with warnings for duplicate `getrandom`,
  `hashbrown`, `twox-hash` and unmatched license allowances already present in
  `deny.toml`.
- `git diff --check`: pass.
