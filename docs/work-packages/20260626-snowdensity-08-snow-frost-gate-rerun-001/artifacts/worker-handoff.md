# Worker Handoff

Evidence class: Static.

SNOWDENSITY-08 is closed
`COMPLETE-08-SNOTEL-CLEARED-FROST-ATTRIBUTION-BLOCKED`.

Next recommended package: SNOWDENSITY-09 diagnostic coupled opt-in WAT path.

Recommended scope:

- Build an authorized diagnostic-only coupled WAT/publication path for the
  non-SNOTEL frost fixtures that applies
  `snow_density_model = physics_bulk_density_compaction_v1` to the actual
  runtime snow-depth state consumed by frost and WAT `Snow-Depth`.
- Keep `legacy_wepp` default and preserve rollback.
- Do not add production parser/runfile/CLI activation unless a new contract
  package explicitly authorizes it.
- Rerun the non-SNOTEL snow-control/frost rubric with the coupled opt-in WAT
  path and compare against the SNOWDENSITY-08 default-path baseline.
- Preserve CoE SWE/liquid/melt anti-alias evidence and no-site-constants
  constraints.
- Do not tune coefficients, canopy, radiation, albedo, melt, density, or frost
  physics.
