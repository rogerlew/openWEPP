# Disposition

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

Final disposition: validated non-defect / invalid upstream input.

Static:

- WBVAL02 did not identify a valid-radiation SIMIMPL28 physics defect.
- The six observed hillslopes share an invalid DRIGGS daily climate record:
  `1990-02-18 radly=486 Ly d^-1` exceeds baseline `sunmap` horizontal daily
  potential `r3=453.068716 Ly d^-1`.
- `SC-CLIMATE-001` now explicitly requires active SIMIMPL28 synthesis to fail
  closed at source symbol `radly` for this condition.
- The HPHYS0277 hourly guard remains active for source-valid daily radiation.
- No snowmelt, percolation, WAT ledger, or comparator compensation was changed.

Ran:

- Before-state six-wrapper validation reproduced the hourly
  `CLIM-RUNTIME-E-017` failures.
- Red contract tests failed before production edit and passed after.
- After-state six-wrapper validation returned typed `radly=486` evidence for
  all six WBVAL02 hillslopes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test -p openwepp-hillslope-orchestrator`, and `cargo deny check`
  passed.
- `cargo test --workspace` failed outside WBVAL02 at an ADR0017 decisions README
  assertion; this is recorded as a residual gate result, not a WBVAL02 closure
  claim.

Closure:

- WBVAL02 acceptance is met through invalid-upstream typed evidence.
- No WBVAL02 `HOLD` remains.
- Follow-on upstream input-boundary work is named in `worker-handoff.md`.
