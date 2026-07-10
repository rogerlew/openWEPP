# Implementation

`run` now coordinates private `parse_invocation`, `parse_path_option`,
`validate_required_options`, `collect_resolved_inputs`, and
`execute_totalwatsed3` stages. These retain the original option order, help
exit, exact `CLITW3-E-*` errors, required/optional discovery precedence,
`Totalwatsed3Config` construction, writer call, and success message.

The obsolete local `clippy::too_many_lines` suppression was removed. No output
formula, schema, CLI, unit, serialization, or fallback behavior changed.
