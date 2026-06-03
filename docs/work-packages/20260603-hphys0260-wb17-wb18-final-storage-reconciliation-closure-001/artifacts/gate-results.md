# Gate Results

Status: completed

Evidence mode: ran

## Commands

- Ran: `cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings`
  - Result: passed after fixing clippy `similar_names` and `too_many_lines`
    diagnostics in the additive trace code.
- Ran: `cargo test --workspace`
  - Result: passed.
- Ran: `cargo deny check`
  - Result: passed with existing duplicate-crate and unmatched-license-allowance
    warnings; advisories, bans, licenses, and sources were ok.
- Ran: `wctl doc-lint --path docs/work-packages/20260603-hphys0260-wb17-wb18-final-storage-reconciliation-closure-001`
  - Result: passed; `0` files validated, `0` errors, `0` warnings.
- Ran: `wctl doc-lint --path docs/work-packages/README.md`
  - Result: passed; `1` file validated, `0` errors, `0` warnings.
- Ran: `bash tools/release/check_authority_suite_antievasion.sh`
  - Result: `PASS: authority suite anti-evasion checks passed.`
- Ran: `cargo test --test auth11_required_suite_obligation_guards_contract`
  - Result: passed.
- Ran: `/workdir/wepppy/.venv/bin/python -m py_compile docs/work-packages/20260603-hphys0260-wb17-wb18-final-storage-reconciliation-closure-001/artifacts/hphys0260_diagnostics.py`
  - Result: passed.
- Ran: `git diff --check`
  - Result: passed.
