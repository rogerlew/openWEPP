# CQR23 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`.

Static: protected boundaries are public API, module visibility, typed statuses,
runtime symbols, units, formulas, float expression order, dispatch order, and
science-contract behavior.

Ran: live before target was
`Wb11HydrologyKernel::run_erod19_route_segment_migration` at CRAP
`351.9234211799049`, CC `79.0`, and coverage `64.76868327402136`.

Ran: live after target is the same function at CRAP `9.00460855712335`, CC
`9.0`, and coverage `96.15384615384616`.

Ran: every newly extracted helper is CRAP `14.787398726851855` or lower.

Static: quality plan executed as scoped behavior-preserving decomposition.
Characterization tests froze route publication shape, EROD14 wave gating,
EROD13 update precedence, and legacy fallback inputs before the production
decomposition.

Status: complete-with-warning. Warning holds: target-file line coverage remains
below the ADR-0021 `90%` line threshold, and pre-existing out-of-scope
`erod19_depend` remains above CRAP `30`.
