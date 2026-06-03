# HPHYS0262 WB17 PMET Demand Seeding Lineage Closure

Status: HOLD

## Objective

Diagnose and correct, where baseline authority is fully available, the H1/H7/H39
WB17 day-1 `Ep` magnitude residual by making the legacy `pmetpara`/`evappm`
lineage observable at the ET-demand seed seam and rerunning the full H1..H39
hillslope semantic suite.

## Rationale

HPHYS0261 showed H1/H7/H39 candidate day-1 `Ep = Etp = 0.385294 mm` while the
baseline WAT `Ep = 0.150000 mm`. The residual is full-demand uptake rather than
SWU stress clipping. The next correctness question is whether openWEPP is
seeding WB11 ET demand with the same `evap`/`evappm` branch and plant/PMET
state consumed by the pinned legacy baseline.

## Included Scope

- Canonical contract amendments for PMET sidecar discoverability, WB11 ET seed
  branch observability, and WB17 `Ep` lineage closure.
- Contract-derived tests for PMET lineage surfaces in HPHYS trace output.
- Runtime projection of parsed `pmetpara` mode and selected crop coefficients
  into numeric state surfaces.
- H1/H7/H39 targeted diagnosis against pinned baseline WAT partitions.
- Full H1..H39 hillslope semantic metric rerun for continuation evidence.

## Excluded Scope

- Heuristic or partial substitutes for legacy `evappm.for` physics.
- Watershed routing closure beyond reported H1..H39 hillslope metrics.
- Production hydrology equation changes unless traced directly to canonical
  `SC-*` text plus pinned baseline provenance.

## Deliverables

- Updated canonical `SC-*` contracts before production code edits.
- Contract-derived tests and pre-implementation gate evidence.
- Trace/runtime PMET lineage surfaces.
- Targeted H1/H7/H39 classification and full H1..H39 semantic metrics.
- Dual review and dual verification artifacts.
- Disposition and worker handoff with continuation recommendation if parity
  remains open.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `docs/work-packages/20260603-hphys0261-wb17-ep-magnitude-initialization-lineage-closure-001/artifacts/worker-handoff.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0262-wb17-pmet-demand-seeding-lineage-closure-001/**`

## Phase Plan

1. Amend contracts for PMET branch observability and WB17 closure authority.
2. Add contract-derived trace/projection tests and record the red gate.
3. Implement trace/runtime PMET lineage projection without proxy physics.
4. Run focused tests, H1/H7/H39 diagnostics, and full H1..H39 metrics.
5. Complete reviews, verification, disposition, and handoff artifacts.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

## Exit Criteria

- Truthfulness-labeled evidence artifacts distinguish `Static:` and `Ran:`.
- PMET sidecar mode, selected crop coefficients, fallback status, and actual
  ET seed branch are visible in HPHYS trace rows.
- H1/H7/H39 day-1 `Ep` residual is classified with baseline and candidate
  values.
- Full H1..H39 semantic metrics are recorded for continuation.
- If full baseline-authoritative `evappm.for` migration is not implemented,
  disposition remains `HOLD`.

## Security-Impact Gate

No external systems or network actions are required. The package is local
repository engineering work limited to flat-file reads/edits and local command
execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must work
through the phase plan, update artifacts through disposition, and only ask for
user direction when hard-blocked by missing local authority or unavailable
validation substrate.
