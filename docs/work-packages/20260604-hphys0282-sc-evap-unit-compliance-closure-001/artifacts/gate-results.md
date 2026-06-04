# Gate Results

Status: completed
Evidence mode: ran

| Gate | Result | Notes |
|---|---:|---|
| Pre-fix SC-EVAP unit lint | fail | 11 findings; expected red gate before contract edits. |
| Post-fix SC-EVAP unit lint | pass | `tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-EVAP-001.md`. |
| HPHYS0279 lint tests | pass | `cargo test --test hphys0279_sc_unit_compliance_lint_contract -- --nocapture`: 9 passed. |
| Docs lint | pass | Scoped `markdown-doc lint`: 23 files, 0 errors, 0 warnings. |
| Diff hygiene | pass | `git diff --check`. |
| Placeholder scan | pass | No `queued`, `not-run`, placeholder, or pending markers remain in HPHYS0282 package artifacts. |

Disposition impact: HPHYS0282 closes the remaining SC-EVAP unit-compliance lint debt identified by HPHYS0281.
