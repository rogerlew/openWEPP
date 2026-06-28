# Verification Agent A

Evidence mode: Ran.

Verified package gates:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- Package diagnostic `default_activation_active_cap.py`: PASS.
- Anti-evasion guard and `auth11_required_suite_obligation_guards_contract`:
  PASS.

Gate non-deferral check: PASS. Current-scope activation evidence is present in
the package artifact set.
