# Unit Remediation Plan

Status: completed
Evidence mode: static

Static: HPHYS0277 remediation is complete for the high hourly radiation guard
gap.

Ran: not-run; execution evidence is recorded in `implementation-test-evidence.md`.

## Remediation Completed

- Contract authority: `SC-CLIMATE-001#INV-CLIMATE-013` now includes the
  physical high-flux guard.
- Runtime guard: SIMIMPL28 hourly radiation publication fails closed on finite
  impossible values.
- Test coverage: red/green contract-derived test added.
- Valid-run compatibility: H1/H7/H39 and full H1..H39 diagnostics completed
  without guard trips.

## Recommended Continuation

- Keep HPHYS0278 as the next output-metadata unit registry alignment package.
- Keep HPHYS0279 as the next machine-checkable `SC-*` unit-governance lint
  package.
- Do not spend further radiation effort on clipping or empirical snowmelt/ET
  compensation; remaining semantic residuals belong to the winter snowpack,
  ET, and storage migration lineages.
