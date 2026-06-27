# SNOWDENSITY-10.3.11 Spring Compaction/Densification Candidate

Status: complete
Owner: Codex
Date: 2026-06-27

## Objective

Implement and adjudicate an opt-in spring compaction/densification candidate for
the remaining March/April snow-depth failures after `coe_liquid_holding_capacity_v1`.
The candidate must preserve SWE and the existing `522 kg m^-3` density cap, then
prove whether physically realized wet-snow compaction improves the coupled WAT
snow-control gate.

## Context

SNOWDENSITY-10.3.10 resolved the 10.3.9 depth-only attribution gap with a
cap-as-mass test. Of `282` March/April failures, `190` are compaction-only
feasible within the existing `SC-SNOWFREEZE-001` `522 kg m^-3` cap, `33` are
cap-limited depletion, `16` are patchy meltout/depletion, and `43` are
under-persistence. This makes compaction the first safe lever: densification
changes depth and density at fixed SWE, while depletion/export requires separate
mass-routing proof.

Two cautions are binding:

- The `522 kg m^-3` cap is a current contract/Ch. 3 authority, but cap
  sensitivity remains real. This package must not raise or fit the cap; it may
  only report cap-limited residuals as follow-on evidence.
- Densification must be a physical wet-snow/melt-freeze compaction process, not
  "densify until observed depth passes." The implementation must not consume
  observed depth, observed density, fixture identity, or row class when computing
  the candidate.

## Required Reading

- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3
- `docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/package.md`
- `docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/artifacts/liquid-holding-capacity-coupled-wat.json`
- `docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001/package.md`
- `docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001/artifacts/spring-pack-depletion-compaction-adjudication.json`
- `references/copyrighted/noaa_6392_DS1.md` Anderson snow compaction and liquid-water settling passages
- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`
- `tools/snowfreeze_observed/snowdensity09_coupled_wat_rerun.py`

## Scope

In scope:

- Amend `SC-SNOWFREEZE-001` before runtime code to authorize exactly one new
  density selector member: `physics_bulk_spring_densification_v1`.
- Implement the candidate behind the existing package-bound diagnostic density
  selector `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL`.
- Keep `legacy_wepp` as the default density model and
  `physics_bulk_density_compaction_v1` behavior unchanged.
- Preserve `coe_liquid_holding_capacity_v1` as the fixed melt boundary for the
  coupled adjudication.
- Preserve CoE SWE/liquid authority: the density candidate may mutate only
  runtime physical depth and density, then force-normalize mass back to the CoE
  runtime SWE.
- Preserve the current `522 kg m^-3` density cap and publish cap sensitivity as
  evidence only.
- Generate coupled WAT evidence comparing `coe_liquid_holding_capacity_v1` with
  `physics_bulk_density_compaction_v1` against
  `coe_liquid_holding_capacity_v1` with `physics_bulk_spring_densification_v1`,
  while retaining the SNOWDENSITY-10.3.8 holding-capacity-only report as prior
  baseline context.
- Report March/April compaction-feasible row clearance, cap-limited residuals,
  under-persistence guardrails, per-surface snow-control deltas, and the
  remaining blocker.

Out of scope:

- Default activation.
- Parser, runfile, user CLI, public output schema, or fixture changes.
- Site-specific constants, observed-depth fitting, density-cap changes, or
  calibration against the Sleepers/Harvard rows.
- Melt coefficient, radiation, canopy, phase partition, rain heat, sub-canopy
  longwave, frost, Qwet/frzftp, or compatibility-runtime changes.
- Treating observation-blocked surfaces as verdict-bearing.
- Declaring snow-control or frost attribution cleared unless the coupled WAT
  gate actually passes.

## Candidate Definition

`physics_bulk_spring_densification_v1` reuses the SNOWDENSITY-06/07
Anderson/SNOBAL-lineage constants and the existing CoE-bound density seam. Its
only algorithmic delta is wet-snow compaction realization: when the fixed CoE
boundary supplies positive liquid for compaction, apply the same total liquid
once to the Anderson/SNOBAL liquid-compaction term and let wet conditions
accelerate the daily 24 time-compaction substeps. This is a process-timing
change, not a new fitted multiplier.

The candidate must:

- Use no observed snow-depth, observed density, fixture identity, site metadata,
  row class, or tolerance in the runtime calculation.
- Use the same total same-day liquid input already supplied by the selected CoE
  melt/liquid boundary.
- Preserve total SWE identity with the CoE runtime boundary after compaction.
- Keep the final runtime density at or below `522 kg m^-3`.
- Preserve the separate CoE boundary depth/density/settle-count carry so future
  CoE melt calculations do not alias to the opt-in runtime density surface.

## Closure Gates

Closure may be `complete` only if:

- The contract is amended before runtime code and includes selector, cap,
  no-fitting, conservation, and coupled-WAT gates.
- Focused runtime tests prove default identity, selector fail-closed behavior,
  SWE conservation, density cap enforcement, and no observed-depth coupling.
- The coupled WAT report exercises the real direct-production publication path
  and trace evidence proves the selected density model reached the snow
  partition.
- The report compares against the 10.3.8 holding-capacity baseline and not an
  older default/melt path.
- No paired surface worsens on snow-control failures. If any paired surface
  worsens, close `HOLD`/non-promotion.
- Under-persistence rows are reported and must not be silently hidden by a
  positive-only summary.
- The package does not claim activation or frost-attribution clearance unless
  the observed-snow-depth coupled WAT gate passes.
- Focused gates pass:
  - `.venv/bin/python tools/snowfreeze_observed/spring_compaction_densification_candidate.py`
  - `cargo fmt --check`
  - `cargo test --test snowdensity10_3_11_spring_compaction_densification`
  - `cargo clippy --test snowdensity10_3_11_spring_compaction_densification -- -D warnings`

## Status Log

- 2026-06-27: Scaffolded the opt-in spring compaction/densification candidate
  package with cap-sensitivity and no-depth-fitting gates.
- 2026-06-27: Added `SC-SNOWFREEZE-001` v96 contract amendment for
  `physics_bulk_spring_densification_v1`.
- 2026-06-27: Implemented the opt-in runtime and snowbench density selector,
  preserving default `legacy_wepp`, existing `physics_bulk_density_compaction_v1`,
  CoE SWE identity, and final `522 kg m^-3` runtime cap.
- 2026-06-27: Ran the coupled WAT gate across all seven surfaces with
  `coe_liquid_holding_capacity_v1` plus both density arms. Closed complete as
  `SPRING-DENSIFICATION-NON-PROMOTION`: existing
  `physics_bulk_density_compaction_v1` under holding-capacity improves prior
  10.3.8 failures `761 -> 498`, but the new spring densification candidate
  worsens that baseline `498 -> 502` with three paired surfaces worse.
