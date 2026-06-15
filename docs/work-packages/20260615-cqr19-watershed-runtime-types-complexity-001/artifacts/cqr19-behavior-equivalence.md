# CQR19 Behavior Equivalence

Status: complete.

Static: production behavior protected by this package:

- public enum names, variants, fields, and derives in `types.rs`
- stable `WS-RUNTIME-E-*` and `CLIM-RUNTIME-E-*` code strings
- exact `Display` strings for watershed and climate runtime input errors
- parser/runtime compatibility and runtime symbol publication semantics
- units, formulas, float expression order, and science-contract behavior

Static: production refactor moved existing `Display` match-arm bodies into
private helper methods:

- `WatershedRuntimeInputError::fmt_basic`
- `WatershedRuntimeInputError::fmt_channel`
- `WatershedRuntimeInputError::fmt_impoundment`
- `WatershedClimateRuntimeInputError::fmt_daily_record`
- `WatershedClimateRuntimeInputError::fmt_breakpoint`
- `WatershedClimateRuntimeInputError::fmt_disaggregation`
- `WatershedClimateRuntimeInputError::fmt_runtime_context`

Static: the public `fmt` implementations now dispatch to those private helpers.
The formatted text in each preserved branch remains byte-for-byte identical to
the pre-refactor strings captured by characterization tests.

Ran: `cargo test -p openwepp-watershed-orchestrator runtime_input_error_characterizes`
passed before production refactor after characterization was added.

Ran: `cargo test -p openwepp-watershed-orchestrator runtime_input_error_characterizes`
passed after production refactor and after final test cleanup.
