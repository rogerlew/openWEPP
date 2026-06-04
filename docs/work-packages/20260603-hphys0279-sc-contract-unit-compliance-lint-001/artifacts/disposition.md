# Disposition

Status: completed/HOLD
Evidence mode: mixed

Static: HPHYS0279 completed the SC contract unit compliance linting package.
The linter, wrapper, contract-derived tests, documentation authority, persisted
gap inventory, dual review, and dual verification are complete. Package remains
HOLD because the new full-contract lint intentionally exposes 227 current
`SC-*` unit-compliance findings, and full workspace testing remains blocked by
the known PL14S/SIMIMPL18 `HKERNEL-WB11-ET-E-003` failures.

## Closure Summary

- Added `tools/release/check_sc_unit_compliance.py` and shell wrapper.
- Registered the HPHYS0279 integration test target in `Cargo.toml`.
- Added nine fixture tests for compliant contracts, missing sections,
  placeholder alias unit checks, registry unit mismatches, missing/unparseable
  registry sources, missing registered aliases, alias-only variable coverage,
  and empty variable tables.
- Added unit-governance and release-tool documentation.
- Persisted full lint inventories as JSON and text artifacts.

## Review Disposition

| ID | Severity | Disposition | Evidence |
| --- | --- | --- | --- |
| A-1 | High | accepted/resolved | Registry loading fails closed with `SCUNIT-E-010` for missing and unparseable sources; regression tests cover both cases. |
| A-2 | High | accepted/resolved | Registered boundary/API and publication aliases must appear in the owning contract alias map; `SCUNIT-E-011` fixture coverage added. |
| A-3 | Medium | accepted/resolved | Canonical registered symbols are required in `Variables and Units`; alias-only coverage emits `SCUNIT-E-012`, and empty tables emit `SCUNIT-E-009`. |
| B-1 | High | accepted/resolved | Disposition and dual verification artifacts are completed in this final reconciliation. |
| B-2 | High | accepted/resolved | Same resolution as A-1. |
| B-3 | High | accepted/resolved | `cargo test --workspace` rerun and recorded as HOLD on known PL14S/SIMIMPL18 failures. |
| B-4 | Medium | accepted/resolved | Full JSON/text finding inventories are persisted and linked from gate/handoff artifacts. |

No review finding remains undispositioned. No HPHYS0279-specific implementation
blocker remains.

## Gate Disposition

Ran:

- `python3 -m py_compile tools/release/check_sc_unit_compliance.py`: pass.
- `cargo fmt --check`: pass.
- `cargo test --test hphys0279_sc_unit_compliance_lint_contract -- --nocapture`:
  pass, 9 tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo deny check`: pass with documented duplicate-crate and
  unmatched-license warnings.
- `markdown-doc lint --path docs/specifications/unit-governance.md --path tools/release/README.md --path docs/work-packages/20260603-hphys0279-sc-contract-unit-compliance-lint-001`:
  pass, 25 files.
- `tools/release/check_sc_unit_compliance.sh`: fail/HOLD with 227 current
  findings.
- `cargo test --workspace`: fail/HOLD only on the two known PL14S/SIMIMPL18
  `HKERNEL-WB11-ET-E-003` failures.

Final posture: completed/HOLD.
