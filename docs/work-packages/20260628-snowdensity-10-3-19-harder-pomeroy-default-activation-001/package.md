# SNOWDENSITY-10.3.19 Harder-Pomeroy Default Activation

Status: complete.

Objective: adopt `harder_pomeroy_hourly` as the direct-production no-env phase
partition default, composed with the activated
`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1` bundle,
while preserving explicit `legacy_rst` rollback/test selection.

## Authority

- `docs/planning/snow-frost-fidelity-strategy.md` §10.3 step 8.
- `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-075`,
  `OBL-SNOWFREEZE-P-050`, `INV-SNOWFREEZE-069`,
  `INV-SNOWFREEZE-072`, `INV-SNOWFREEZE-065`, and
  `INV-SNOWFREEZE-050`.
- SNOWDENSITY-10.3.18 cross-SNOTEL mechanism rubric artifacts.
- SNOWDENSITY-10.3.5b/10.3.5c Harder-Pomeroy partition authority and
  validation lineage.
- ADR-0017: legacy and PySnobal profiles are diagnostic flags, never targets.

## Scope

In scope:

- Contract-first amendment for the direct-production phase default and Policy-B
  cross-SNOTEL rubric gate.
- Change absent/empty `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL` to select
  `harder_pomeroy_hourly` in the direct-production snow path.
- Preserve explicit `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=legacy_rst`
  rollback/test behavior and fail-closed unknown values.
- Add internal trace evidence for the selected phase model.
- Re-run the coupled direct-production cross-SNOTEL rubric comparing the prior
  activated bundle with explicit `legacy_rst` phase against the new no-env
  default.
- Prove direct trace selector reachability and precipitation partition
  conservation.

Out of scope:

- No fixture, public output-schema, density-cap, frost, Qwet/frzftp,
  compatibility-runtime, parser/runfile/user selector, or `.run` disable change.
- No site calibration, fixture fitting, phase-coefficient tuning, or
  humid-New-England-specific promotion criterion.
- No activation of 10.3.16 sublimation or 10.3.17 shallow-pack guard.

## Gates

- Cross-SNOTEL forcing-robust rubric: new no-env default must be at least as good
  as the prior activated bundle with explicit `legacy_rst` phase.
- Workspace-suite no-regression under the new no-env default.
- Partition conservation: active-hour source guard remains enforced and real
  direct-production trace rows close the selected precipitation partition within
  tolerance.
- Release notes carry forward the humid-New-England depth regression as a
  non-representative roadmap item and the cross-SNOTEL density bias rise
  (`+23.6 kg m^-3`) as separately tracked recovery work.

## Execution Log

- [x] Required reading: strategy §10.3 step 8, `SC-SNOWFREEZE-001`
  `INV-SNOWFREEZE-069/072/075`, `INV-SNOWFREEZE-050`,
  SNOWDENSITY-10.3.18 artifacts, and 10.3.5 Harder-Pomeroy authority.
- [x] Contract-first amendment.
- [x] Implemented default selector and trace proof.
- [x] Ran cross-SNOTEL activation gate.
- [x] Ran workspace validation.
- [x] Recorded reviews, gate results, and disposition.

## Disposition

`ACTIVATED`.

The no-env direct-production default now composes the activated melt+density
bundle with `harder_pomeroy_hourly` phase partitioning. The real cross-SNOTEL
direct-production gate reconfirmed `15` robust fails / `179` score for the new
default versus `17` / `172` for the prior activated bundle with explicit
`legacy_rst` phase. Explicit `legacy_rst` rollback remains available, selector
trace proof closes, and partition conservation closes with max trace residual
`5.55e-17 m`.
