# CQR17 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`.

Static: protected boundaries are public API, runtime symbols, typed status
behavior, formulas, float expression order, parser compatibility, and
science-contract semantics.

Status: closed.

Ran: baseline LCOV and CRAP were captured before production refactor. Live
target was confirmed as
`Wb11HydrologyKernel::erod19_xcrit_classification`, CRAP
`465.5844995022966`, CC `37.0`, coverage `32.098765432098766`.

Ran: focused characterization was added before production refactor because the
target had low live coverage and many branch outcomes.

Static: production change is a behavior-preserving decomposition of the
classification decision tree into private helpers. It does not alter
orchestrator state, runtime publications, symbol aliases, units, typed errors,
or formula operands.

Ran: after LCOV and CRAP show target CRAP `2.0`; all extracted helpers are
CRAP `<= 12.666666666666664`.

Static: quality dimension stayed limited to CRAP/cyclomatic-complexity
burn-down for the scoped target.
