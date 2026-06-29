# FROST Thaw-Residual Diagnostic

Status: complete — `EXECUTED-COMPLETE-DIAGNOSTIC-SNOW-BURIED-DOMINANT`.

Package type: diagnostic-only work package.

Objective: bucket the 13 post-residue Sleepers candidate-defect timing cells
into thaw residual mechanisms, separating tiny-tail detector artifacts from
material frost persistence.

Primary gap: `GAP-SNOWFREEZE-002`.

## Scope

Included:

- The two Step 1 unblocked Sleepers sites:
  `site1_sleepers_south_field_vt` and `site2_sleepers_w9_hardwood_vt`.
- The post-residue seasonal runs from Step 3 / residue-cover implementation.
- Daily trajectories from observed thaw forward for the 11 remaining thaw-late
  cells.
- Separate characterization of the 2 remaining early-onset cells.
- Threshold-sensitivity sweep for tiny-tail classification.
- Snow-depth-controlled re-bucketing over warm/wet material-frost days, with
  paired observed snow-depth evidence where available.
- Snow-persistence decomposition into snow-buried accumulation/near-balance,
  snow-buried under-melt/linger, snow-free persistence, and mixed cells.

Excluded:

- No frost solver change.
- No detector-threshold change.
- No `Qwet`, wet-heat, state-machine, snow, residue, fixture, contract,
  default, output-schema, or validation-rubric change.
- No ratification of `INV-SNOWFREEZE-047/048/050`.

## Required Reading

- `docs/planning/snow-frost-fidelity-strategy.md` section 11.
- `docs/work-packages/20260629-frost-step2-sleepers-attribution-001/`.
- `docs/work-packages/20260629-frost-step3-residue-parameterization-001/`.
- `docs/work-packages/20260629-frost-residue-cover-implementation-001/`.
- `tools/snowfreeze_observed/observed_harness.py` thaw-date detector.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`.
- `crates/openwepp-runner/src/hillslope/frost_entry.rs` or the active R7G frost
  trace writer path.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`.

## Intended Write Set

- `docs/work-packages/20260629-frost-thaw-residual-diagnostic-001/**`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- Step 3 diagnostic artifact updates only if needed to cross-reference the
  narrowed `GAP-SNOWFREEZE-002` disposition.

## Execution Plan

1. Consume the current post-residue Step 3 seasonal Sleepers comparison reports
   and frost traces.
2. Align WAT daily rows with R7G frost trace rows.
3. Extract each remaining candidate-defect cell window:
   - thaw-late: observed thaw date through modeled thaw date;
   - early-onset: modeled onset date through observed onset date.
4. Bucket each thaw-late cell into `H2`, `H1a`, or `H1b` with evidence.
5. Re-bucket thaw-late cells by snow control over warm/wet material-frost days:
   snow-buried, snow-free persistent, or mixed, with a snow-depth sensitivity
   sweep at `0.05`, `0.10`, and `0.20 m`.
6. Decompose snow-buried cells into accumulation/near-balance versus
   under-melt/linger using modeled SWE gain/loss through the carried-frost
   window, and report paired observed snow depth where available.
7. Characterize early-onset cells separately.
8. Emit per-cell table, threshold-sensitivity sweep, snow-control split,
   aggregate split, routing recommendation, and `GAP-SNOWFREEZE-002`
   disposition.

## Exit Criteria

- Per-cell artifact names each candidate-defect cell, bucket, and evidence.
- Tiny-tail threshold sensitivity is reported over a range, without adopting a
  production detector threshold.
- H1a/H1b routing is based on material frost plus warm/rain/melt/surface-temp
  evidence from the trace/WAT surfaces.
- Snow-depth-controlled re-bucketing is reported separately from the H1a/H1b
  bucket and is not used to adopt a production threshold.
- Snow-persistence evidence reports modeled SWE gain/loss and paired observed
  snow-depth residuals where the comparison reports provide them.
- Early-onset cells are reported separately.
- No production code, contract, fixture, default, or output-schema change.
- Markdown/JSON/script checks pass.

## Disposition

Executed complete. The package consumed the post-residue Step 3 seasonal Sleepers
runs and frost traces without changing production code, fixtures, contracts,
defaults, or the observed harness detector.

Initial result:

- 11 thaw-late cells remain after residue-cover coupling.
- 9 of 11 thaw-late cells bucket `H1a` missing wet/advective thaw energy.
- 2 of 11 thaw-late cells bucket `H1b` state-machine thaw asymmetry.
- 0 of 11 thaw-late cells bucket `H2` under material thresholds through `0.05 m`.
- Only an aggressive `0.10 m` diagnostic threshold would classify 4 thaw-late
  cells as H2; that threshold is not adopted and would require observation-
  protocol authority before any future use.
- The two early-onset cells are material early-freeze cells, not tiny tails and
  not part of the thaw-late persistence mechanism.

Post-review snow-controlled re-bucketing accepted the Claude review finding that
the H1a bucket alone over-routed the residual to `Qwet`: at the diagnostic
`0.10 m` snow-control split, the `11` thaw-late cells route to `7`
snow-buried cells, `2` snow-free persistent cells, and `2` mixed cells. The
snow-buried cells further split to `5` under-melt/linger cells and `2`
accumulation/near-balance cells. The snow-depth sensitivity sweep preserves the
same `7` snow-buried cells across `0.05`, `0.10`, and `0.20 m`; the snow-free
count ranges from `1` to `3`.

`GAP-SNOWFREEZE-002` remains open and is narrowed: the thaw-late residual is
snow-buried-dominant, so the next package should decompose snow persistence
first: forcing-limited over-accumulation versus fixable spring under-melt. Only
the snow-free persistent subset justifies a `Qwet` / wet-advective thaw-energy
candidate. Mixed cells and the `H1b` minority should remain secondary follow-up
items after the snow-persistence route is resolved.
