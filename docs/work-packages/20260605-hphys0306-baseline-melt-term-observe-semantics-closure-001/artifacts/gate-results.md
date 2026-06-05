# Gate Results

Status: complete

Evidence mode: ran

Static:

- HPHYS0306 is diagnostic-only and does not change production physics.
- Review findings were dispositioned before final broad gates.

Ran:

- `python -m py_compile docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/artifacts/hphys0306_branch_active_observe_semantics.py` passed.
- `cargo fmt --check` initially failed on the new test; `cargo fmt` was run; final `cargo fmt --check` passed.
- `cargo test --test hphys0306_baseline_melt_observe_semantics_contract -- --nocapture` initially failed on an over-specific line-break-sensitive phrase; assertion was corrected; final command passed.
- `cargo test --test hphys0304_fixed_comparator_semantic_rerun_contract -- --nocapture` passed after accepting executed-HOLD HPHYS0305 follow-on status.
- `bash tools/release/check_authority_suite_antievasion.sh` passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with warnings only for duplicate crates
  (`getrandom`, `hashbrown`, `twox-hash`) and unmatched license allowances
  (`ISC`, `Unicode-DFS-2016`).
