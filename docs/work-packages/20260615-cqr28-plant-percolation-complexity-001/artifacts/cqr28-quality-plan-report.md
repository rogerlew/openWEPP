# CQR28 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`.

Static: protected boundaries are WB17/WB18 formulas, runtime symbols, aliases,
units, typed guards, writeback order, public/crate-visible signatures, and
science-contract behavior.

Ran: baseline and final metrics were captured in package artifacts.

Final status: complete-with-warnings.

Closure:

- Target `Wb11HydrologyKernel::run_percolation`: CRAP
  `281.82979375564685` to `17.19373252009578`.
- Maximum newly extracted helper CRAP: `22.896222121074196`.
- Target-file line coverage: `68.76%` to `72.95%`.
- Target-file function coverage: `71.43%` to `85.19%`.

Warnings:

- Same-file out-of-scope rows above CRAP `30` remain:
  `resolve_effective_wb18_frozen_depth` and `run_plant_root_uptake`.
- `cargo crap` reports 126 LCOV source-map warnings unrelated to the target
  file's LCOV mapping.
