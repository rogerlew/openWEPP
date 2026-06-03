# HPHYS0270 Winter Daily Snowpack State Closure

Status: completed/HOLD

## Objective

Continue from HPHYS0269 by diagnosing, correcting, and validating the daily
winter snowpack state lineage that remains after retained-rain accounting,
signed raw melt observability, and corrected daily negative-melt redistribution
were wired. The package targets the H1/H7 day-99 and H39 day-115 material
spring snowpack divergences by making pre-day and post-day snowpack carry state
observable and by migrating the next proven `winter -> snowd -> melt` daily
state slice.

## Rationale

HPHYS0269 closed important trace identities but full H1..H39 semantic parity
remained `0/39`. H1/H7/H39 first-material `Ep` divergences are still
snowpack/SWE/`RM` divergences with closed trace arithmetic. Per
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-014` and `SC-WATBAL-001#INV-WATBAL-054`, the
project must not reopen WB17 `Ep`, aggregate storage, or WB13 publication
compensation until snowpack lineage proves the residual belongs elsewhere.
HPHYS0270 therefore focuses on daily snowpack carry state, settlement, density,
pre-day state reproduction, and remaining `snowd.for`/`melt.for` lineage.

## Included Scope

- Contract-first amendments to canonical `SC-*` contracts when new daily-state
  obligations are proven.
- Baseline/static diagnosis of `winter.for`, `snowd.for`, `melt.for`, and
  required hourly forcing routines for H1/H7 day 99 and H39 day 115 context.
- Runtime trace additions for pre-day and post-day snowpack state where needed
  to classify the residual.
- Deterministic focused tests for snowpack daily state, including carry-state
  settlement and corrected daily negative-melt behavior.
- Production implementation only for a proven baseline-authoritative or
  corrected-authority daily snowpack state gap.
- Targeted H1/H7/H39 verification plus full H1..H39 hillslope metrics.
- Review, verification, disposition, and continuation handoff artifacts.

## Excluded Scope

- WB17 `Ep` tuning or repartition.
- WB13 `RM`/`Snow-Water` publication compensation.
- Aggregate storage compensation.
- Re-enabling non-agricultural frost for HPHYS parity.
- Reproducing the pinned baseline negative-melt sign/branch bug. Corrected
  `/workdir/wepp-forest@03fee4558456535138592630b5dedc4d81ce8d06` remains
  target authority for daily negative-melt redistribution.
- Empirical calibration, surrogate formulas, or process approximations.

## Deliverables

- Baseline and corrected-authority provenance map for the daily snowpack state
  slice.
- Canonical contract amendments, if required, authored before production code.
- Contract-derived tests for daily snowpack state and trace evidence.
- Iterative diagnosis log covering H1/H7/H39 first-divergence state lineage.
- Production implementation or explicit no-patch disposition for each diagnosed
  seam.
- Targeted H1/H7/H39 and full H1..H39 metrics.
- Final `GO`/`HOLD` disposition and HPHYS0271 handoff if closure remains open.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for`
- `/workdir/wepp-forest_260430_baseline/src/radcur.for`
- `/workdir/wepp-forest@03fee4558456535138592630b5dedc4d81ce8d06`
- `docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/artifacts/disposition.md`
- `docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/artifacts/full-39-suite-metrics.md`
- `docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/artifacts/worker-handoff.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/**`

## Phase Plan

1. Read authority and HPHYS0269 continuation artifacts.
2. Build a daily snowpack provenance map for H1/H7 day 99 and H39 day 115.
3. Amend canonical contracts if the daily-state seam requires additional
   authority beyond HPHYS0269.
4. Add contract-derived tests and pre-implementation gate evidence.
5. Diagnose targeted traces for pre-day/post-day runtime snowpack state.
6. Implement one proven daily-state slice at a time.
7. Run focused tests, targeted H1/H7/H39 diagnostics, and full H1..H39 metrics.
8. Complete review, verification, disposition, and continuation handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests and trace assertions.
3. Pre-implementation contract gate.
4. Production code edits.

## Exit Criteria

- Evidence artifacts truthfully distinguish `Static:` from `Ran:`.
- Corrected `/workdir/wepp-forest@03fee455...` negative-melt authority remains
  preserved; pinned bug compatibility is not implemented.
- H1/H7/H39 first-divergence snowpack state gap is materially reduced or
  assigned to a precise remaining source seam.
- Targeted H1/H7/H39 and full H1..H39 metrics are recorded.
- Disposition is `GO` only if the declared daily-state slice is complete and no
  known contract violations remain; otherwise `HOLD` with narrowed handoff.

## Security-Impact Gate

No external systems or network actions are required. This package is local
repository engineering work limited to flat-file reads/edits and local command
execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must work
through the phase plan, update artifacts through disposition, and only ask for
user direction when hard-blocked by missing local authority or unavailable
validation substrate.
