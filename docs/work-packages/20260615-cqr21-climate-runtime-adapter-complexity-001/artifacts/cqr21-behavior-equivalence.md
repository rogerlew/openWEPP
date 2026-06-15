# CQR21 Behavior Equivalence

Status: complete.

Static: production edits are limited to private display-message helper
extraction below `SharedClimateRuntimeInputError`.

Static: preserved behavior:

- public API unchanged; no new `pub` items or exported type changes.
- stable `SharedClimateRuntimeInputError::code()` mappings preserved, including
  retired `E-010` gap and shared `E-011` mapping for both breakpoint
  cardinality variants.
- display strings preserved byte-for-byte for all error variants.
- runtime projection symbols, aliases, units, parser compatibility, formulas,
  float expression order, and climate disaggregation behavior unchanged.

Ran: focused characterization passed before production refactor:

```bash
cargo test -p openwepp-climate-runtime-adapter cqr21_shared_climate_runtime_input_error_characterizes
```

Ran: the same focused characterization passed after production refactor:

```bash
cargo test -p openwepp-climate-runtime-adapter cqr21_shared_climate_runtime_input_error_characterizes
```
