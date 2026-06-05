# Gate Results

Status: complete

Evidence mode: ran

Static:

- HPHYS0305 is instrumentation-only and routes missing paired surfaces to HOLD.
- Anti-evasion guards are required because the package touches external
  authority suite posture and comparator evidence.

Ran:

- `python -m py_compile docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/hphys0305_paired_melt_term_state.py` passed.
- `cargo test --test hphys0305_paired_melt_term_state_contract -- --nocapture` passed.
- `bash tools/release/check_authority_suite_antievasion.sh` passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` passed.
- `cargo fmt --check` initially failed after edits; `cargo fmt` was run; final `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` initially failed on a stale `map_unwrap_or` and the expanded trace test line count; both were fixed; final command passed.
- `cargo test --test hphys0293_winter_melt_timing_contract -- --nocapture` passed after updating stale comparator-authority wording.
- `cargo test --workspace` initially exposed the same stale HPHYS0293 wording; final workspace test run passed.
- `cargo deny check` passed with warnings only for duplicate crates
  (`getrandom`, `hashbrown`, `twox-hash`) and unmatched license allowances
  (`ISC`, `Unicode-DFS-2016`).
