# Numeric Equivalence

Evidence label: Static/Ran.

Status: `COMPLETE`

Numeric/API/output identity evidence:

- No public function signatures, exported types, runtime symbols,
  serialization paths, thresholds, tolerances, or contract files were changed.
- `impoundment_outflow_at_stage` now dispatches to private helpers in the same
  branch and accumulation order as the scaffold baseline.
- The culvert helper preserves the original index families (`3..5` and `6..8`)
  and original boundary-symbol strings (`f04_b`, `f05_bd`, `f07_b`, `f08_bd`).
- Focused tests assert closed-form outlet-family totals and exact stable-step
  integration values for deterministic fixtures.
- Focused tests assert typed fail-closed behavior for invalid domains and
  non-finite states instead of permitting fallback values.

Ran:

- `cargo nextest run -p openwepp-watershed-orchestrator` - exit `0`,
  `39 tests run: 39 passed, 0 skipped`.
- `cargo clippy -p openwepp-watershed-orchestrator -- -D warnings` -
  exit `0`.

No broad comparator parity claim is made by this package. Heavy workspace gates
are recorded separately in `gate-results.md`.
