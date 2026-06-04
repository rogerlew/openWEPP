# Implementation Test Evidence

Status: completed/HOLD
Evidence mode: ran

Static: implementation adds an executable SC contract unit compliance linter
without changing runtime behavior.

Implemented:

- `tools/release/check_sc_unit_compliance.py`
- `tools/release/check_sc_unit_compliance.sh`
- `tests/integration/hphys0279_sc_unit_compliance_lint_contract.rs`
- `Cargo.toml` integration-test registration
- `tools/release/README.md` command documentation
- `docs/specifications/unit-governance.md` lint authority text

Ran:

- `cargo test --test hphys0279_sc_unit_compliance_lint_contract -- --nocapture`:
  pass, 9 tests.
- `tools/release/check_sc_unit_compliance.sh --format json`: fail/HOLD with
  227 current findings.
- `tools/release/check_sc_unit_compliance.sh`: fail/HOLD with persisted text
  inventory in `sc-unit-compliance-findings.txt`.
- `markdown-doc lint --path docs/specifications/unit-governance.md --path tools/release/README.md --path docs/work-packages/20260603-hphys0279-sc-contract-unit-compliance-lint-001`:
  pass.
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo deny check`: pass with existing duplicate-crate and unmatched-license
  warnings.
- `cargo test --workspace`: HOLD; two pre-existing SIMIMPL18/PL14S tests fail
  with `HKERNEL-WB11-ET-E-003`.
