# Pre-Implementation Contract Gate

Static: complete before production Rust edits.

## Required Authority

Static:

- Read `docs/specifications/science-contracts/AGENTS.md`.
- Read `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`.
- Inspected compatibility provenance in:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/05_pl_phase_dispatch.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`

## Contract-Derived Rules

Static:

- Decomposition seed pools must be finite and non-negative.
- `tmax`, `tmin`, precipitation, water-stress fraction, `oratea`, and `orater`
  must be finite and domain-valid.
- `oratea` and `orater` are non-negative; zero is valid and yields no decay.
- Event fractions are bounded to `[0, 1]`.
- PL17 decomposition update is equation-derived and must not be a pass-through
  fallback for covered branches.
- Missing R5B upstream state or missing/ambiguous active context in the current
  typed scope must fail closed.

## Contract Amendment Decision

Static: no `SC-*` amendment is required for R5C because the implementation
ports an already documented PL17 scalar/tracked-pool slice into direct typed
state. If implementation needs new equations, bounded tolerance, public
publication authority, or non-PL17 process behavior, package disposition must
switch to `HOLD` before extending scope.
