# HPHYS0263 WB11/WB17 EVAPPM Demand Migration Closure

Status: hold

## Objective

Port the pinned legacy WEPP `evappm.for` Penman-Monteith demand equations into
the openWEPP WB11 ET-demand seed seam so H1/H7/H39 PMET-mode runs no longer
seed WB17 `Ep` from the Priestley-Taylor branch when `pmetpara.iflget != 1`.

## Rationale

HPHYS0262 proved that the 39 hillslope suite discovers `pmetpara.txt` and that
H1/H7/H39 select PMET mode with `iflget=2`, `kcb=0.95`, and `rawp=0.80`, but
openWEPP still seeded `wb11_et_demand` from the legacy `evap` Priestley-Taylor
branch. Pinned legacy `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
calls `evappm` whenever `iflget != 1`; therefore parity closure requires
baseline-authoritative `evappm.for` migration, not coefficient tuning or proxy
PMET formulas.

## Progress

- [x] (2026-06-03 Codex) Scaffolded package structure, kickoff prompt, and
  required evidence artifacts.
- [x] (2026-06-03 Codex) Amended canonical `SC-*` authority before
  production code edits.
- [x] (2026-06-03 Codex) Added contract-derived tests and recorded the red
  gate.
- [x] (2026-06-03 Codex) Ported the baseline-authoritative WB11 PMET demand
  seed subset.
- [x] (2026-06-03 Codex) Ran focused tests, diagnostics, and full H1..H39
  metrics.
- [x] (2026-06-03 Codex) Completed disposition and handoff.

## Surprises & Discoveries

- Runfile-sidecar mode still needed default-file discoverability for
  `pmetpara.txt`; without it the first HPHYS0263 full-suite run selected
  `iflget=1` and could not exercise the migrated branch.
- The management projection computed baseline-derived `canhgt` but did not
  publish it to the scalar runtime surface consumed by `evappm`; publishing it
  was required to avoid a typed `canhgt` missing-symbol failure.
- Adding `deglat`/`elevm` to hillslope climate projection required the
  watershed climate adapter to publish matching per-hillslope symbols to keep
  parser-runtime parity tests green.
- Post-commit Claude Code review accepted as
  `artifacts/review_claude_code_disposition.md` found that HPHYS0263 migrated
  the EVAPPM equations but left a WB11/WB17 seam ambiguity: `pmet.ep_m` is an
  actual/stressed PMET transpiration value while downstream WB17 still treats
  `wb11_et_demand` as a potential demand and applies partition/SWU stress.

## Decision Log

- Decision: HPHYS0263 will target WB11 demand seeding, not full post-ET soil
  evaporation redistribution.
  Rationale: HPHYS0262 isolated the immediate divergence to `wb11_et_demand`
  branch selection; full `evappm.for:391-454` redistribution mutates `st` after
  `Es` and belongs in a follow-on only if diagnostics prove it is the next
  residual owner.
  Date/Author: 2026-06-03 Codex.
- Decision: Keep package disposition `HOLD` for full `evappm.for` routine
  closure even though the WB11 demand-seed subset is implemented and validated.
  Rationale: pinned `evappm.for:391-454` post-ET soil evaporation
  redistribution remains intentionally excluded from this package, so full
  routine migration closure must continue under a follow-on package.
  Date/Author: 2026-06-03 Codex.

## Outcomes & Retrospective

- WB11 demand seed now selects `evappm_pmet` when `pmetpara.mode.iflget != 1`
  and publishes migrated PMET intermediates.
- H1/H7/H39 day-1 `Ep` residual tightened from `+0.235294 mm` to
  `+0.001823 mm`.
- Full 39-hillslope semantic pass remains `0/39`; selected residual means
  improved for `Ep`, storage, and `Dp`, while snow/runoff/lateral families
  remain continuation scope.
- Package remains `HOLD` for full `evappm.for` routine closure because
  post-ET redistribution lines `391-454` remain unported.
- Package also remains `HOLD` because the PMET-mode WB11/WB17 seam must be
  corrected before building the deferred redistribution path on top of it.

## Context and Orientation

The openWEPP runner seeds daily WB11 water-balance inputs in
`crates/openwepp-runner/src/hillslope/mod.rs`. The current seed path can see
`pmetpara` sidecar mode and selected crop coefficients, but HPHYS0262 showed it
still uses a Priestley-Taylor demand formula even when legacy WEPP would call
`evappm`. The legacy authority is pinned in
`/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` and
`/workdir/wepp-forest_260430_baseline/src/evappm.for`. In legacy code,
`iflget.eq.1` selects `evap`; any other value selects `evappm`.

For this package, "PMET demand seed" means the scalar openWEPP publishes as
`wb11_et_demand` before the root-uptake phase. The seed must be computed from
the baseline-authoritative `evappm` equations when `pmetpara.mode.iflget != 1`.

## Included Scope

- Canonical `SC-EVAP-001` and `SC-WATBAL-001` contract amendments for the
  migrated `evappm.for:181-297` equation subset and required input aliases.
- Contract-derived tests that fail before the production migration and pass
  after the migrated PMET seed path is active.
- Production WB11 ET-demand seed migration for the baseline-authoritative
  `evappm` reference ET, crop coefficient, soil evaporation reduction, and
  plant transpiration stress equations needed to publish `wb11_et_demand`.
- Trace additions for PMET branch intermediate diagnostics needed to explain
  H1/H7/H39 residuals.
- H1/H7/H39 targeted PMET branch classification and full H1..H39 semantic
  metric rerun.

## Excluded Scope

- Heuristic, empirical, or proxy Penman-Monteith substitutes not traced to
  pinned `evappm.for`.
- Complete migration of legacy post-ET soil evaporation redistribution in
  `evappm.for:391-454` unless required to seed WB11 demand.
- Watershed routing closure beyond reported H1..H39 hillslope metrics.
- Unrelated WB18/WB19 percolation, lateral-flow, or final storage repairs.

## Deliverables

- Updated canonical `SC-*` authority before production code edits.
- Contract-derived tests and red pre-implementation gate evidence.
- Baseline-authoritative PMET demand seeding with actual branch trace
  `evappm_pmet`.
- H1/H7/H39 PMET branch metrics and full H1..H39 semantic metrics.
- Kernel-profile checklist, gate results, review/verification artifacts,
  disposition, and continuation handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/evappm.for`
- Pinned baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `docs/work-packages/20260603-hphys0262-wb17-pmet-demand-seeding-lineage-closure-001/artifacts/worker-handoff.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/**`

## Phase Plan

1. Amend canonical contracts with exact `evappm.for` equation authority,
   variable aliases, and guards.
2. Add contract-derived tests and record the red pre-implementation gate.
3. Port the baseline-authoritative PMET demand path into WB11 seeding.
4. Run focused tests, H1/H7/H39 diagnostics, and full H1..H39 metrics.
5. Complete review, verification, disposition, and continuation handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

## Exit Criteria

- Truthfulness-labeled artifacts distinguish `Static:` from `Ran:`.
- When `pmetpara.mode.iflget != 1`, `wb11_et_demand` is produced by the
  baseline-authoritative PMET branch and trace branch is `evappm_pmet`.
- H1/H7/H39 targeted metrics show whether day-1 `Ep` residuals improve,
  regress, or move to a downstream process after PMET demand migration.
- Full H1..H39 semantic metrics are recorded.
- If only a subset of `evappm.for` is migrated, disposition explicitly states
  whether that subset is sufficient for WB11 demand closure and what remains.

## Security-Impact Gate

No external systems or network actions are required. This package is local
repository engineering work limited to flat-file reads/edits and local command
execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must work
through the phase plan, update artifacts through disposition, and only ask for
user direction when hard-blocked by missing local authority or unavailable
validation substrate.
