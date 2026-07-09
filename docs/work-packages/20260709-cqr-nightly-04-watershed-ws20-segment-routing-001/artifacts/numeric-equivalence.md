# Numeric Equivalence

Evidence label: Static/Ran.

Status: `PROVISIONAL-ROLLED-BACK-NOT-CLOSURE`

Provisional evidence:

- `cargo nextest run -p openwepp-watershed-orchestrator` - exit `0`,
  `37 tests run: 37 passed, 0 skipped`.
- `cargo clippy -p openwepp-watershed-orchestrator -- -D warnings` - exit `0`.

Numeric/API/output identity statement for the provisional diff:

- Decomposition moved whole statement blocks into private helpers without
  changing floating-point expression grouping, arithmetic order, loop order,
  state writeback order, public API, runtime symbols, diagnostics meaning, or
  serialization.
- Characterization tests assert exact synthetic outputs for the newly covered
  helper paths and guard-class identity for invalid domains.

Hold disposition:

- No production/test implementation diff remains in the current tree, so there
  is no landed numeric/API/output behavior change to accept.
- The provisional evidence is retained only to explain the attempted CQR route
  and the subsequent hold.
