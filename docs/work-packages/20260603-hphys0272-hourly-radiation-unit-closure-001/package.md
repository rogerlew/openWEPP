# HPHYS0272 Hourly Radiation Unit Closure

Status: completed/HOLD

## Objective

Correct and validate the HPHYS0271 day-36 radiation forcing defect where
`winter.hourly.rad_mj_m2_####` is emitted with Langley-scale magnitude under an
MJ-labeled surface. The package targets baseline-authoritative `radly` ->
`radmj` conversion and `sunmap`/`radcur`/`hr_tmp` hourly radiation lineage before
any WB13, WB17, storage, or snowmelt compensation edits.

## Rationale

HPHYS0271 proved H1 sim-day 36 raw melt reconstructs exactly from the
`melt.for` term sum, and the highest-magnitude hour reports
`59.258047 MJ m^-2 h^-1`. That value is physically impossible and is close to a
Langley-per-hour magnitude. Static baseline inspection shows `stmget.for` reads
`radly` in Langleys/day and `winter.for` converts `radmj = radly * 0.04184`
before `hr_tmp.for`; openWEPP currently treats the parser `rad` value as MJ/day
and converts it back to Langleys. HPHYS0272 must fix the unit seam with contract
and test authority first.

## Included Scope

- Contract-first amendments to `SC-CLIMATE-001`, plus snow/waterbalance cross
  references as needed, for explicit daily `radly` to `radmj` conversion and
  hourly radiation bounds.
- Static baseline provenance for `stmget.for`, `winter.for`, `sunmap.for`,
  `radcur.for`, and `hr_tmp.for` radiation units.
- Contract-derived tests proving hourly radiation sums/magnitudes use
  `radly * 0.04184`, not raw Langleys.
- Production correction in SIMIMPL28 hourly winter forcing only.
- Targeted H1/H7/H39 HPHYS0271-style melt-forcing diagnostics after the fix.
- Full H1..H39 semantic suite metrics after the fix.
- Disposition and continuation handoff.

## Excluded Scope

- WB13 `RM`/`Snow-Water` publication compensation.
- WB17 `Ep` changes.
- Aggregate storage compensation.
- Snowmelt equation changes.
- Negative-melt redistribution changes.
- Empirical calibration or heuristic radiation clipping in production paths.

## Deliverables

- Baseline radiation-unit provenance map.
- Canonical contract amendments authored before production edits.
- Contract-derived tests for daily-to-hourly radiation unit closure.
- Production patch to SIMIMPL28 radiation conversion.
- Targeted H1/H7/H39 and full H1..H39 metrics.
- Final `GO`/`HOLD` disposition and HPHYS0273 handoff if residuals remain.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/sunmap.for`
- `/workdir/wepp-forest_260430_baseline/src/radcur.for`
- `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for`
- `docs/work-packages/20260603-hphys0271-day36-melt-forcing-lineage-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260603-hphys0271-day36-melt-forcing-lineage-closure-001/artifacts/h1-day36-melt-forcing-diagnostics.md`
- `docs/work-packages/20260603-hphys0271-day36-melt-forcing-lineage-closure-001/artifacts/worker-handoff.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0272-hourly-radiation-unit-closure-001/**`

## Phase Plan

1. Record baseline radiation-unit provenance.
2. Amend canonical climate/snow/waterbalance contracts.
3. Add contract-derived radiation unit tests.
4. Record pre-implementation contract gate evidence.
5. Patch SIMIMPL28 daily `radly` to `radmj` conversion.
6. Run focused tests, targeted H1/H7/H39 diagnostics, and full H1..H39 metrics.
7. Complete reviews, verification, disposition, and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

## Exit Criteria

- Evidence artifacts truthfully distinguish `Static:` from `Ran:`.
- H1 sim-day 36 hourly radiation no longer exceeds physically plausible
  MJ-scale magnitude because of Langley/MJ unit confusion.
- Hourly radiation reconstruction uses `radly * 0.04184` and preserves
  baseline `sunmap`/`radcur`/`hr_tmp` lineage.
- Targeted H1/H7/H39 and full H1..H39 metrics are recorded.
- Disposition remains `HOLD` if downstream semantic parity still has unresolved
  residuals after the radiation-unit seam is corrected.

## Security-Impact Gate

No external systems or network actions are required. This package is local
repository engineering work limited to flat-file reads/edits and local command
execution in the worktree.

## Execution Result

- HPHYS0272 corrected the scoped radiation-unit seam and validated the fix with
  contract-derived tests plus targeted/full-suite diagnostics.
- H1 day-36 max hourly radiation changed from `59.258047` to
  `2.388678 MJ m^-2 h^-1`; H7 and H39 day-36 targeted maxima are also
  MJ-scale with no hour `>=10 MJ m^-2 h^-1`.
- Full H1..H39 runtime completed `39/39 rc=0`; semantic parity remains `0/39`.
- Decision: `completed/HOLD` because radiation-unit closure is complete but
  broader snowpack/ET/storage semantic parity residuals remain.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must work
through the phase plan, update artifacts through disposition, and only ask for
user direction when hard-blocked by missing local authority or unavailable
validation substrate.
