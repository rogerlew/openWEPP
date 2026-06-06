# HPHYS0315 Hourly Snowfall Input Lineage Closure

Status: queued

## Objective

Diagnose, correct if authorized, and validate the branch-gated hourly snowfall
input lineage feeding `snowd.for:166-172` for the H1/H7/H39 spring-2014
settling-route rows localized by HPHYS0313.

## Rationale

HPHYS0313 proved that the 2014-target settling rows do not originate from the
settling equation or snow-drift. The first material divergence is a same-hour
snowfall input mismatch: pinned baseline records `hrsnow =
0.0007454545120708644 m` at 2013 day 11 hour 11 while the homologous openWEPP
hourly snowfall depth is `0.0 m`. This package must trace that value back
through fixed-baseline precipitation phase/distribution source lines and
openWEPP homologous forcing publication before any snow producer edit is
authorized.

## Included Scope

- Add canonical contract authority for branch-gated hourly snowfall input
  lineage after HPHYS0313.
- Add contract-derived tests for same-unit `hrsnow`/openWEPP snowfall-depth
  pairing, source-line provenance, and no-compensation posture.
- Build diagnostics that compare fixed-baseline `winter.for -> stmtim.for ->
  snowd.for` precipitation phase/distribution surfaces with openWEPP hourly
  snowfall-depth traces for H1/H7/H39 spring-2014 rows.
- Identify whether the first source-owned divergence is parser forcing,
  precipitation phase partition, hourly distribution, trace mapping, or
  unresolved harness surface.
- Run full H1..H39 semantic metrics after any authorized correction or after
  recording `HOLD`.

## Excluded Scope

- No snow-drift migration from this route.
- No WB13, WB17, WB18, WB19, WB12, melt-term, or branch-predicate
  compensation.
- No production edit unless source-line, same-unit, same-lineage evidence
  proves an openWEPP-owned defect and contract authority is amended first.
- No reuse of stale depth-vs-SWE HPHYS0298 verdicts.

## Deliverables

- `SC-SNOWFREEZE-001` hourly snowfall input lineage invariant.
- `SC-WATBAL-001` water-balance consumer gate for hourly snowfall input
  lineage.
- `tests/integration/hphys0315_hourly_snowfall_input_lineage_contract.rs`.
- `artifacts/hourly-snowfall-input-lineage-ledger.md`.
- `artifacts/hourly-snowfall-source-lineage.md`.
- `artifacts/full-39-suite-metrics.md`.
- Gate, review, verification, disposition, and worker-handoff artifacts.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/unit-governance.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/work-packages/20260606-hphys0314-adr0017-snow-rm-reclassification-route-ledger-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/snowpack-settling-carry-recursion-ledger.json`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `Cargo.toml`
- `tests/integration/hphys0315_hourly_snowfall_input_lineage_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260606-hphys0315-hourly-snowfall-input-lineage-closure-001/**`

## Phase Plan

1. Contracts: amend canonical snow and water-balance contracts.
2. Contract tests: add HPHYS0315 source-line/unit-lineage tests.
3. Pre-implementation gate: run focused contract test before diagnostics.
4. Diagnostics: trace fixed-baseline and openWEPP hourly snowfall inputs.
5. Production checkpoint: edit production code only if source-owned defect is
   proven.
6. Metrics: run full H1..H39 semantic suite for continuation metrics.
7. Review: complete dual review, disposition findings, and dual verification.
8. Closeout: update disposition, handoff, artifact statuses, and README.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits only when source-line provenance authorizes them.

## Exit Criteria

- H1/H7/H39 spring-2014 hourly snowfall input rows are classified with
  same-unit/same-lineage evidence.
- Any `OPENWEPP-DEFECTIVE` verdict satisfies ADR0017 independent correctness
  authority; otherwise the route remains `HARNESS-SURFACE-MISMATCH` or
  `UNRESOLVED`.
- Full H1..H39 metrics are recorded.
- No downstream compensation is authorized.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.
