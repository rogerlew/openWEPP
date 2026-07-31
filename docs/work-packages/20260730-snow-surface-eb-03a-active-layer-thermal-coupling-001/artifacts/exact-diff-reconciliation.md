# Exact-Diff Reconciliation

Status: `complete`

Evidence mode: `Static + Ran`

The terminal tree contains two consecutive operator-authorized increments:
the inherited uncommitted EB-03 implementation and EB-03A. The reconciliation
therefore uses the package write set and the inherited-state declaration,
rather than falsely attributing the complete `git diff` to EB-03A.

EB-03A edits are confined to:

- canonical snow-energy/snow-freeze authority and mechanically affected
  contract-version assertions;
- meteorology conductivity and typed validation;
- Stage 3 active/lower projection, substeps, diagnostics, and pressure
  construction;
- direct-runner trace diagnostics;
- focused tests and the Stage 0 opt-in source-boundary allowlist;
- this package, campaign roadmap/catalog entries, and directly required
  assurance adoption/render output.

No fixture, observed dataset, public schema, default selector, CoE melt
authority, phase algorithm, density algorithm, or frost equation changed.
No branch was created or switched. No secret, unsafe block, external mutation,
or new user coefficient was introduced.

Ran: `git diff --check` passed. Source scanning and both independent reviews
found no clamp, fitted limiter, air-temperature replacement, or unadmitted
physics.

`crates/openwepp-meteorology/src/error.rs` is an inherited EB-03 edit that adds
the typed surface-energy errors used by that package's longwave/sublimation
implementation. EB-03A did not expand or reclassify it as active-layer
physics; its own new meteorology mechanism is in `surface_energy.rs`.
