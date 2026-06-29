# FROST STEP 3 Residue Parameterization Diagnostic

Status: complete — `EXECUTED-COMPLETE-DIAGNOSTIC-BRANCH-C`.

Package type: diagnostic-only work package.

Objective: test whether the Sleepers frost timing candidate defects from Step 2
are caused by inert versus seasonal residue parameterization.

Primary gap: `GAP-SNOWFREEZE-002`.

## Scope

Included sites:

- `site1_sleepers_south_field_vt`
- `site2_sleepers_w9_hardwood_vt`

Excluded sites:

- `site4_ggd498_morris_mn`: Step 1 `BLOCKED`.
- `site3_scan_mandan_nd`: Step 1 `INCONCLUSIVE-NO-PAIRED-SNOW`.
- `site5_reynolds_creek_us_rls_id`: Step 1 `INCONCLUSIVE-NO-PAIRED-SNOW`.

Included analysis:

- Entry-gate trace evidence that a `Dec_*` management produces seasonal
  `residue_depth_m` at the frost solver, or evidence that it remains flat.
- Package-local temporary Sleepers seasonal-management variants under `target/`
  only when the entry gate passes.
- A-versus-B frost timing comparison for current inert/baseline management and
  the seasonal `Dec_*` variant, using the Step 2 sign-coherence analyzer.
- Decision branch A/B/C routing and an updated `GAP-SNOWFREEZE-002`
  disposition input.

Excluded:

- No frost-model changes.
- No snow-model changes.
- No production fixture edits or fixture repointing.
- No contract-physics, default, output-schema, selector, or ratification change.
- No `Qwet`, frozen-K, SFCC, impedance, or residue-litter implementation.

## Required Reading

- `docs/planning/snow-frost-fidelity-strategy.md` section 11 step 3.
- `docs/work-packages/20260629-frost-step1-current-snow-control-rerun-001/`.
- `docs/work-packages/20260629-frost-step2-sleepers-attribution-001/`.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  `INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`, `INV-SNOWFREEZE-050`,
  `TOL-SNOWFREEZE-009`, and `GAP-SNOWFREEZE-002`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`.
- `tests/fixtures/snowfreeze_observed/site2_sleepers_w9_hardwood_vt/p3.man`.
- `tests/fixtures/snowfreeze_observed/site1_sleepers_south_field_vt/`.
- `tests/fixtures/cancov_forest/*_deciduous_*/` and
  `tests/fixtures/cancov_forest/*_mixed_*/`.
- `docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
  surface residue / litter cover section.

## Intended Write Set

- `docs/work-packages/20260629-frost-step3-residue-parameterization-001/**`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`

Generated run outputs and temporary fixture variants are confined to
`target/frost_step3_residue_parameterization/`.

## Execution Plan

1. Record required-reading evidence and static coupling evidence that
   `residue_depth_m` reaches `frost_surface_heat_path`.
2. Run an existing seasonal deciduous `Dec_*` fixture with
   `OPENWEPP_R7G_FROST_TRACE_PATH` and inspect the daily `residue_depth_m`
   trajectory at the frost solver.
3. If the entry gate is flat or physically unreasonable, close branch C without
   re-scoring Sleepers.
4. If the entry gate is seasonal, create package-local temporary Sleepers
   variants using the seasonal `Dec_*` management and run current default snow
   through `openwepp-cli-hill`.
5. Score baseline A and seasonal B with the Step 2 timing/sign-coherence
   analyzer and compare candidate-defect cells, onset residuals, thaw residuals,
   and frozen-duration residuals.
6. Route exactly one branch:
   - A: seasonal residue exists and shrinks timing defects.
   - B: seasonal residue exists but timing defects survive.
   - C: `Dec_*` does not drive seasonal residue or is physically unreasonable.
7. Record review, verification, line-count governance, and final disposition.

## Exit Criteria

- The entry-gate residue trajectory is captured with trace provenance.
- The core A-versus-B comparison is run only if the entry gate passes.
- The Step 2 sign-coherence analyzer is reused for timing attribution.
- The result routes to exactly one branch A/B/C with justification.
- `GAP-SNOWFREEZE-002` disposition is updated without implementing a fix.
- No production code, production fixture, default, schema, or contract-physics
  changes are made.
- Markdown validation passes for touched docs.

## Disposition

The entry gate ran `tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/p10`
(`Dec_4899`) through `openwepp-cli-hill` with
`OPENWEPP_R7G_FROST_TRACE_PATH` enabled. The frost solver received
`32874` trace rows, but `residue_depth_m` was constant at
`0.02302585092994045 m` for every row:

- minimum: `0.02302585092994045 m`
- maximum: `0.02302585092994045 m`
- rounded unique count at `1e-6 m`: `1`
- autumn mean: `0.02302585092994045 m`
- spring mean: `0.02302585092994045 m`

Because `Dec_*` did not drive a seasonal `residue_depth_m` trajectory to the
frost solver, the package followed decision branch C and stopped before the
Sleepers A-versus-B re-score. The flat entry-gate result is itself the finding:
the existing cropland-management seasonal deciduous fixture changes canopy-side
state but does not prove a seasonal forest litter/residue insulation path for
frost.

`GAP-SNOWFREEZE-002` remains open. The follow-on is not a production fixture
repoint. Promote the residue-cover dimension of
`docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md` to
implement or expose first-class forest litter/residue cover before rerunning
the Sleepers timing attribution. No frost-model, snow-model, production fixture,
contract-physics, default, output-schema, selector, or harness-default change was
made.

## Security / Production Impact

Diagnostic-only. Temporary fixture variants are generated under `target/` and
are not production fixtures. No runtime, physics, fixture, output-schema,
default, selector, or contract authority changes are authorized.
