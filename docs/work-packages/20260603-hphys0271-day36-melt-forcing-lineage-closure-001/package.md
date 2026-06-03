# HPHYS0271 Day-36 Melt Forcing Lineage Closure

Status: completed/HOLD

## Objective

Continue from HPHYS0270 by diagnosing, correcting, and validating the H1
sim-day 36 spurious early-February melt event that first opens the large spring
snowpack residual. The package targets term-level `melt.for` energy-balance
lineage and hourly forcing inputs (`amelt`, `bmelt`, `cmelt`, `dmelt`, hourly
temperature, radiation, dewpoint/vapor-pressure, wind, canopy/albedo/cover,
precipitation phase, and `winter -> snowd -> melt` branch flags) before any
WB17 `Ep`, aggregate-storage, or WB13 publication compensation is considered.

## Rationale

HPHYS0270 added daily pre/post snowpack carry-state observability and stayed
`completed/HOLD`. Post-execution Claude Code review bisected H1 against the
HPHYS0270 run root and showed days 1-35 track baseline, then sim-day 36 releases
approximately `27-28 mm` redistributed melt from approximately `54 mm` raw melt
while baseline reports `RM=0` and continues accumulating. This proves the next
high-value seam is the melt trigger/magnitude or its hourly forcing, not broad
seasonal accumulation or negative-melt redistribution.

## Included Scope

- Contract-first amendments to canonical `SC-*` contracts for HPHYS0271
  term-level melt/forcing evidence obligations.
- Baseline/static diagnosis of `/workdir/wepp-forest_260430_baseline/src/melt.for`,
  `winter.for`, `snowd.for`, `stmtim.for`, `hr_tmp.for`, and `radcur.for` for
  H1 sim-day 36.
- Runtime trace additions for hourly melt terms and forcing inputs required to
  classify the H1 day-36 spurious melt event.
- Deterministic focused tests for HPHYS trace term publication.
- Production implementation only for proven baseline-authoritative or
  corrected-authority defects; otherwise explicit no-patch disposition.
- Targeted H1 day-36 diagnostics plus H1/H7/H39 and full H1..H39 metrics.
- Review, verification, disposition, and continuation handoff artifacts.

## Excluded Scope

- WB17 `Ep` tuning or repartition.
- WB13 `RM`/`Snow-Water` publication compensation.
- Aggregate storage compensation.
- Further negative-melt redistribution work unless a mixed thaw/refreeze day is
  proven material to this residual.
- Reproducing the pinned baseline negative-melt sign/branch bug.
- Empirical calibration, surrogate formulas, or process approximations.

## Deliverables

- Baseline provenance map for H1 day-36 melt terms and hourly forcing lineage.
- Canonical contract amendments authored before production edits.
- Contract-derived trace tests for melt-term/hourly-forcing evidence.
- H1 day-36 diagnosis report with term-level candidate values.
- Production trace implementation or explicit no-patch disposition for each
  diagnosed seam.
- Targeted H1/H7/H39 and full H1..H39 metrics.
- Final `GO`/`HOLD` disposition and HPHYS0272 handoff if closure remains open.

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
- `docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/artifacts/review_claude_code.md`
- `docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/artifacts/review_disposition.md`
- `docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/artifacts/worker-handoff.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0271-day36-melt-forcing-lineage-closure-001/**`

## Phase Plan

1. Read HPHYS0270 review/disposition artifacts and baseline melt authority.
2. Build a baseline provenance map for H1 day-36 melt terms and hourly forcing.
3. Amend canonical contracts for HPHYS0271 term-level melt/forcing evidence.
4. Add contract-derived tests and pre-implementation gate evidence.
5. Implement trace publication for hourly melt terms and forcing inputs.
6. Run targeted H1 day-36/H1-H7-H39 diagnostics and full H1..H39 metrics.
7. Complete review, verification, disposition, and continuation handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests and trace assertions.
3. Pre-implementation contract gate.
4. Production code edits.

## Exit Criteria

- Evidence artifacts truthfully distinguish `Static:` from `Ran:`.
- H1 sim-day 36 melt-trigger/magnitude lineage is materially narrowed or
  corrected with baseline-authoritative proof.
- Corrected `/workdir/wepp-forest@03fee455...` negative-melt authority remains
  preserved and off the critical path unless evidence proves otherwise.
- Targeted H1/H7/H39 and full H1..H39 metrics are recorded.
- Disposition is `GO` only if the declared day-36 melt/forcing slice is complete
  and no known contract violations remain; otherwise `HOLD` with narrowed
  handoff.

## Security-Impact Gate

No external systems or network actions are required. This package is local
repository engineering work limited to flat-file reads/edits and local command
execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must work
through the phase plan, update artifacts through disposition, and only ask for
user direction when hard-blocked by missing local authority or unavailable
validation substrate.
