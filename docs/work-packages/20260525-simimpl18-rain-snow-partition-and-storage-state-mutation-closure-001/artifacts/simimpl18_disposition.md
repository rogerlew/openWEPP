# simimpl18_disposition

Status: package-complete-with-hold
Evidence mode: static+ran
Date: 2026-05-25
Decision: HOLD

## Static
- Phase A complete: intake and SIMIMPL17 signal freeze executed.
- Phase B complete: canonical contract amendments landed.
- Phase C complete: contract-derived tests added and pre-implementation gate
  captured.
- Phase D complete (partial-closure outcome):
  - replay-tooling baseline-year/full-span policy implemented;
  - Tier-A reruns executed under identical shared input fixtures;
  - production runner physics closure not completed.
- Phase E complete: closure criteria evaluated and final decision recorded.

## Ran
- Final evidence bundle:
  - `artifacts/replay-run-20260525T132822Z/`
- Full-span policy outcomes:
  - `common_row_count=1095`, `only_baseline_count=0`,
    `only_candidate_count=0` (policy materialized baseline rows).
- Remaining blockers:
  - day-1 mismatch persists (`RM`, `Snow-Water`, `Total-Soil`, `frozwt`,
    `SoilWaterTotal`),
  - candidate storage tuple remains invariant across span,
  - publication leak signal persists (`winter.ssd` mirrored in
    `hydout_equivalent.snow_water`),
  - workspace test gate fails on SIMIMPL18 contract assertions.

## Final disposition
- Package is complete from execution/governance perspective.
- Hold-lift is not approved.
- Residual closure requires baseline-authoritative process-physics migration
  and rerun evidence refresh in a follow-on package.
