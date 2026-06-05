# HPHYS0295 Cumulative Storage-Budget Ownership Closure

Status: executed-hold

## Objective

Diagnose and, only if baseline-authoritative ownership is proven, correct the
cumulative H1/H7/H39 storage residual by reconciling row-to-row
`Total-Soil`/`SoilWaterTotal` deltas against WB17 `Ep`/`Es`, WB18 `D`, WB19
`latqcc`, and HPHYS0293 excluded snow/`RM` residual masks.

## Rationale

HPHYS0294 closed local WB18 aggregate identity, `D=Pe`, and WB19
target/unrealized lineage on target rows, but storage residual direction remains
mixed across hillslopes. Comparator deltas alone therefore do not prove a
production WB18/WB19 defect. The next low-regret move is a cumulative budget
classifier that identifies whether the storage residual is carried by ET,
percolation, lateral flow, snow/`RM` producer masks, or unresolved cross-term
timing before any production change.

## Included Scope

- Amend canonical `SC-*` authority for cumulative storage-budget ownership.
- Add contract-derived tests for required budget trace and contract surfaces.
- Run full H1..H39 semantic metrics.
- Run H1/H7/H39 target cumulative budget diagnostics over first residual
  windows and spring 2014/2016 continuation rows.
- Patch production code only if a baseline-authoritative process owner is
  proven by the cumulative budget.

## Excluded Scope

- Do not compensate corrected negative-melt snow producer residuals in
  WB17/WB18/WB19.
- Do not change `Ep`, `Es`, `D`, `latqcc`, or storage publication from
  comparator deltas alone.
- Do not patch WB13 aggregate output as a budget-balancing shortcut.
- Do not promote MOFE carry/runon storage-ingress behavior.

## Deliverables

- Canonical contract amendments in `SC-WATBAL-001` and `SC-EVAP-001`.
- Contract-derived test `tests/integration/hphys0295_cumulative_storage_budget_contract.rs`.
- H1/H7/H39 cumulative budget artifacts.
- Full H1..H39 semantic metrics artifact.
- Gate evidence, owned-file manifest, review/verification artifacts,
  disposition, and worker handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260605-hphys0294-post-ingress-storage-percolation-lateral-retention-closure-001/artifacts/worker-handoff.md`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0295-cumulative-storage-budget-ownership-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `tests/integration/hphys0295_cumulative_storage_budget_contract.rs`
- `Cargo.toml`
- Production WB17/WB18/WB19 files only if diagnostics prove a
  baseline-authoritative defect.

## Phase Plan

1. Contracts: amend `SC-WATBAL-001` and `SC-EVAP-001` budget authority.
2. Contract tests: add static contract-derived guards for budget surfaces.
3. Pre-implementation gate: run HPHYS0295 contract test before production edits.
4. Diagnostics: run full H1..H39 suite and H1/H7/H39 cumulative budget traces.
5. Implementation: patch only a proven process owner and rerun focused/full
   gates.
6. Review/disposition: update dual review/verification artifacts, disposition,
   and handoff.

## Contract-First Sequence

1. Implement required contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Modify production code only after diagnostic ownership is proven.

## Exit Criteria

- Contract amendments and tests exist and pass.
- H1/H7/H39 cumulative budgets identify whether residuals are ET, D, lateral,
  snow/`RM` mask, or unresolved cross-term timing.
- Full H1..H39 metrics are recorded.
- Production edits, if any, are baseline-authoritative and validated.
- Package remains `executed-hold` unless semantic/contract/review gates close.

## Security-Impact Gate

No external systems, credentials, network calls, or shell interpolation are
required. Work is local flat-file reads/edits plus local test and diagnostic
commands.

## Autonomous Execution

This package is intended for end-to-end autonomous execution. If diagnostics do
not prove a production owner, leave the package in `executed-hold` with a
specific continuation recommendation.
