# CQR29 Coverage Closure

Ran: before LCOV for
`crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`
reported `FNF:3`, `FNH:2`, `LF:139`, and `LH:35`; the target
`Wb11HydrologyKernelGuardError::fmt` had coverage `0%`.

Ran: final LCOV reports `FNF:9`, `FNH:9`, `LF:191`, and `LH:184`; the target
`Wb11HydrologyKernelGuardError::fmt` has coverage `100%`.

Ran: newly extracted helper coverage is sufficient for CRAP closure:

- `display_parts`: `100%`
- `phase_display_parts`: `97.72727272727273%`
- `erod13_display_parts`: `94.73684210526315%`
- `erod14_display_parts`: `94.73684210526315%`
- `erod18_display_parts`: `95.65217391304348%`
- `fmt_with_code`: `100%`

Static: uncovered helper lines are defensive `unreachable!` invariant guards
that cannot be reached through `display_parts()`.
