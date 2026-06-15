# CQR19 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`.

Static: protected boundaries are public enum surface, error codes, display
strings, runtime seam behavior, parser compatibility, runtime symbols, units,
and science-contract behavior.

Ran: baseline metrics identified live target
`WatershedClimateRuntimeInputError::fmt` at CRAP `420.0`.

Ran: focused characterization was added because the target formatter had `0.0%`
coverage in the before report.

Ran: after metrics closed the target at CRAP `6.0`; all target-file rows and
new helpers are `<= 30`.

Status: complete.
