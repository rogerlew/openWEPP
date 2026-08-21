# Stage 3 cadence and terminal API audit

Status: `PASS FOR EXTRACTION / COVERED-CONSUMER CLOSURE BLOCKED`.

`Static:` `SC-COUPLEDTIME-001` v3 and `SC-SNOWFREEFORCING-001` v1 already admit
the required common 1,800-second support and 48-support day. The existing
transition is in
`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`,
and the terminal solve is in `stage3_solver/terminal_event.rs`.

`Static:` The implementation adds
`DirectSnowStage3SupportInput { forcing, duration_seconds }` and public
support evaluators. It preserves the existing constants, operation order,
terminal solver, closure checks, and typed errors. It does not run a whole-day
result, duplicate an hourly row, halve a result, interpolate a daily state, or
construct `TerminalStateRates`.

`Ran:` the one-support event test and focused five-test orchestrator slice
passed. The remaining contract-gated closure requirement is the actual
snow-covered V11/shared-carrier consumer, which is not present in the released
consumer stack and is therefore fail-closed by the new attachment.
