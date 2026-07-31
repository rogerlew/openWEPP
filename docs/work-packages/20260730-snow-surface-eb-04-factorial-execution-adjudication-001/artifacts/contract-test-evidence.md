# Contract-Test Evidence

Status: `complete / pass`

Evidence mode: `Ran`

The new source-level test first failed before the producer edit, then passed as
part of `snow_surface_eb03_contract` 9/9. The Stage 3 runtime suite passed 6/6.
See [`pre-implementation-contract-gate.md`](pre-implementation-contract-gate.md).

The 6/6 runtime evidence explicitly includes
`polar_night_and_double_sublimation_fail_closed` and the additive-cell
assertions that sublimation does not alias routed melt, snowpack SWE loss, or
incoming/routed/retained/refrozen liquid. Those unchanged focused tests bind
the package acceptance checks for exact double-sublimation rejection and
liquid non-aliasing.
