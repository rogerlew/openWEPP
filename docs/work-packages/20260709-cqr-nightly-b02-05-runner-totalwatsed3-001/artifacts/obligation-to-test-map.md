# Obligation-to-Test Map

| Obligation / eligible surface | Test binding | Status |
|---|---|---|
| Public CLI help and typed option grammar | `totalwatsed3_cli_help_and_argument_errors_preserve_cli_contract` | PASS |
| Required PASS consumer must fail closed when absent | `totalwatsed3_cli_fails_closed_when_required_pass_input_is_missing` | PASS |
| Explicit input selection overrides default discovery | `totalwatsed3_cli_explicit_relative_and_absolute_inputs_override_default_discovery` | PASS |
| Explicit optional input must fail closed when absent | `totalwatsed3_cli_rejects_missing_explicit_optional_inputs` | PASS |
| Aggregate pass/WAT consumer and output value identity | `totalwatsed3_cli_uses_pass_runvol_and_outlet_lateral_flow` | PASS |
| Per-hillslope pass/WAT discovery and output aggregation | `totalwatsed3_cli_reads_openwepp_per_hillslope_pass_and_wat_surfaces` | PASS |

Direct `SC-SYSTEM-001` applicability: the CLI is a watershed consumer/publication
boundary, so pass/WAT presence, no-silent-repair errors, and output identity are
bound above. Routing/impoundment physics, HBP event payload schema, groundwater
branching, channel execution, and their WS11/WS12-style authority obligations
are not directly changed by this totalwatsed3 CLI control-flow refactor; no
claim about those surfaces is made and no exclusion is used for target coverage.
