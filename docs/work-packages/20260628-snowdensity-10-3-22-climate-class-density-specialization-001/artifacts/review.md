# Local Review

Evidence mode: `Static`

## Scope Reviewed

- Contract amendment `SC-SNOWFREEZE-001` v107.
- Opt-in runtime density candidate `physics_bulk_climate_class_density_v1`.
- Direct-production class-normal computation and operand handoff.
- Cross-SNOTEL+cancov rubric wrapper and generated artifacts.
- Package disposition and protected-boundary claims.

## Findings

No unresolved implementation findings remain.

Accepted and fixed during verification:

- `cargo clippy` flagged precision-loss casts in the run-normal averaging code.
  Fixed by using bounded `u32` counters with `f64::from`.
- `cargo clippy` flagged `snow_liquid_partition` crossing the 100-line lint.
  Fixed by extracting the inactive-snow partition constructor.
- `cargo test --workspace` exposed stale SC-SNOWFREEZE version guards and a
  missing allowlist entry for the new package-bound diagnostic wrapper. Fixed
  affected snowdensity guard tests to v107 and added
  `tools/snowfreeze_observed/climate_class_density_specialization.py` to the
  authorized physics-bulk diagnostic/opt-in surface list.

## Protected Boundaries

- No production default changed.
- `legacy_wepp` and existing activated bundle rollback behavior remain
  available.
- No parser, runfile, user CLI, output schema, fixture, density-cap, frost,
  melt, phase, canopy, radiation, Qwet/frzftp, or compatibility-runtime change
  was made.
- Class assignment uses run-derived CDM/SPR/wind normals only; no site identity,
  geographic lookup, NSIDC raster lookup, observations, or residuals enter the
  runtime candidate.
- Sturm/Liston 2021 is recorded only as a cross-check and is not substituted for
  the Sturm 1995 names/thresholds paired with Sturm 2010 density parameters.

## Disposition Check

The package closes `HOLD-GATE-FAILURE-NON-PROMOTION`. Source authority and
conservation passed, but the candidate failed the primary observed-data rubric,
the bidirectional densification flip, and the persistence guardrail.
