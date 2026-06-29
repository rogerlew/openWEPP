# FROST Snow-Persistence Decomposition

Status: complete — `EXECUTED-COMPLETE-DIAGNOSTIC-SPARSE-OBS-NO-UNDER-MELT`.

Package type: diagnostic-only work package.

Objective: determine whether the snow-buried Sleepers thaw-late cells over-
persist from spring under-melt or over-accumulation by comparing modeled versus
observed ablation rate, not by SWE-delta sign alone.

Primary gap: `GAP-SNOWFREEZE-002`.

## Scope

Included:

- The `7` snow-buried thaw-late cells from
  `20260629-frost-thaw-residual-diagnostic-001`.
- The buried portions of the `2` mixed snow-control thaw-late cells.
- Paired Sleepers observed snow-depth rows and the post-residue Step 3 modeled
  WAT/trace outputs already on disk.
- Reuse of existing `tools/snowfreeze_observed/` loaders and snow-program
  diagnostic patterns.
- Like-for-like comparison to the SNOWDENSITY-10.3.8/10.3.9/10.3.10 spring
  melt residual lineage.

Excluded:

- The `2` snow-free persistent cells; they remain the deferred `Qwet` subset.
- No melt-model, snow-model, frost-model, contract, default, fixture, or schema
  change.
- No `Qwet` work.
- No ratification of `INV-SNOWFREEZE-047/048/050`.

## Required Reading

- `docs/work-packages/20260629-frost-thaw-residual-diagnostic-001/`
- `docs/work-packages/20260629-frost-thaw-residual-diagnostic-001/artifacts/claude-review.md`
- `docs/work-packages/20260629-frost-thaw-residual-diagnostic-001/artifacts/gap-snowfreeze-002-thaw-residual-disposition.md`
- `docs/planning/snow-frost-fidelity-strategy.md` sections 10.2 and 11.
- `tools/snowfreeze_observed/march_april_residual_attribution.py`
- `tools/snowfreeze_observed/spring_pack_depletion_compaction_adjudication.py`
- `tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py`
- `tools/snowfreeze_observed/winter_thaw_melt_response.py`
- `tests/fixtures/snowfreeze_observed/observations/sites/`

## Intended Write Set

- `docs/work-packages/20260629-frost-snow-persistence-decomposition-001/**`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`

## Execution Plan

1. Consume the prior thaw-residual bucket JSON and select only snow-buried cells
   plus the buried portions of mixed cells.
2. Load post-residue Step 3 WAT rows and paired Sleepers observed snow-depth
   rows through the existing snow/frost observation tooling.
3. For each scoped cell, build paired observed/model windows over the carried-
   frost interval and over its buried warm/wet subset.
4. Compute modeled and observed peak depth and ablation rate from paired rows.
   Verdicts require enough paired ablation-season observations to estimate a
   rate; sparse rows route to `INCONCLUSIVE-SPARSE-OBS`.
5. Route each cell to:
   - `OVER-ACCUMULATION-FORCING-LIMITED`,
   - `SPRING-UNDER-MELT-FIXABLE`, or
   - `INCONCLUSIVE-SPARSE-OBS`.
6. Compare the Sleepers route to the SNOWDENSITY-10.3.8 spring-melt residual
   lineage and determine whether the frost thaw-late residual unifies with the
   snow spring-melt residual.
7. Emit JSON/CSV/Markdown artifacts and update the GAP/strategy disposition.

## Exit Criteria

- Per-cell route table includes modeled-vs-observed peak depth and ablation-rate
  evidence.
- Sparse paired-observation cells are explicitly marked inconclusive, not forced
  into a route.
- The determination states whether the frost thaw-late residual unifies with
  the snow spring-melt residual.
- The `2` snow-free cells remain out of scope and deferred to `Qwet`.
- No production code, fixture, contract, default, or schema change.
- Python, JSON, Markdown, and diff checks pass.

## Disposition

Executed complete. The package consumed the prior thaw-residual bucket artifact,
the post-residue Step 3 seasonal Sleepers WAT outputs, paired Sleepers observed
snow-depth rows, and the SNOWDENSITY-10.3.10 spring-pack lineage. It changed no
production code, fixtures, contracts, defaults, or schemas.

Result:

- `9` scoped cells were analyzed: `7` snow-buried thaw-late cells plus the
  buried portions of `2` mixed cells.
- `8/9` route `INCONCLUSIVE-SPARSE-OBS`: paired observations are too sparse, or
  the available paired ablation interval shows modeled loss comparable to or
  faster than observed.
- `1/9` routes `OVER-ACCUMULATION-FORCING-LIMITED`: W9 1997 has modeled peak
  depth above observed tolerance and no modeled loss-rate deficit.
- `0/9` route `SPRING-UNDER-MELT-FIXABLE`: no scoped cell has an aggregate
  modeled-vs-observed ablation-rate deficit under the reused snow-program rate
  discriminator.

Determination: the Sleepers frost thaw-late residual does **not** establish a
shared spring-under-melt defect with the SNOWDENSITY-10.3.8/10.3.10 spring-melt
lineage. `GAP-SNOWFREEZE-002` remains open, with snow-buried cells carried as
snow-persistence uncertainty / forcing-limited evidence; the `2` snow-free
persistent cells remain the deferred `Qwet` subset.
