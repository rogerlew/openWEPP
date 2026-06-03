# Gate Results

Status: completed

Evidence mode: ran

## Gates

- Ran: `cargo fmt --check` failed initially with rustfmt-only changes needed.
- Ran: `cargo fmt` applied formatting.
- Ran: `cargo fmt --check` passed.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Ran: `cargo test --workspace` passed.
- Ran: `cargo deny check` passed with existing duplicate and unmatched-license
  warnings.
- Ran: `bash tools/release/check_authority_suite_antievasion.sh` passed.
- Ran: `cargo test --test auth11_required_suite_obligation_guards_contract`
  passed.
- Ran: `/workdir/wepppy/.venv/bin/python -m py_compile
  docs/work-packages/20260603-hphys0259-wb19-trace-ep-dp-storage-localization-closure-001/artifacts/hphys0259_diagnostics.py`
  passed.
- Ran: `git diff --check` passed.
