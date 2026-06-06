# HPHYS0316 2013 Terminal Carry Recursion Closure

Status: queued

## Objective

Recurse the H1/H7/H39 spring-2016 year-start inherited snowpack rows into the
2013 terminal carry chain feeding 2014 day 1 hour 1, classify the first material
paired divergence, and determine the next source-owned route without downstream
compensation.

## Rationale

HPHYS0313 proved that the 2016-target rows are already materially divergent at
2014 day 1 hour 1, with openWEPP carrying approximately `0.013-0.015 m` more
snow depth than the fixed comparator. That means the 2015 day 1 route remains
inherited and must recurse into the 2013 terminal snowpack state that feeds
2014 day 1. This package should locate the first material divergence in the
2013 chain before any branch-predicate, melt-term, WB13, WB17, WB18, WB19, or
WB12 edit is considered.

## Included Scope

- Add canonical contract authority for 2013 terminal carry recursion after
  HPHYS0313.
- Add contract-derived tests for recursive carry ledger completeness,
  source-line provenance, route counts, and no-compensation posture.
- Compare fixed-baseline and openWEPP snowpack carry state through the 2013
  terminal chain feeding 2014 day 1 hour 1 for H1/H7/H39 rows.
- Classify first material divergence by source lane: initial projection,
  snowfall input, density settling, retained liquid, negative-melt state loss,
  raw/routed melt, or unresolved paired-evidence gap.
- Run full H1..H39 semantic metrics for continuation.

## Excluded Scope

- No production Rust kernel edits unless source-line and paired-state evidence
  proves an openWEPP-owned defect.
- No WB13, WB17, WB18, WB19, WB12, branch-predicate, or melt-term
  compensation.
- No heuristic snowpack carry correction.
- No snow science-review equation adjudication unless diagnostics hit a flagged
  `snowd.for` uncertainty; if so, open/route to the science-review backlog.

## Deliverables

- `SC-SNOWFREEZE-001` 2013 terminal carry recursion invariant.
- `SC-WATBAL-001` water-balance consumer gate for 2013 terminal carry
  recursion.
- `tests/integration/hphys0316_2013_terminal_carry_recursion_contract.rs`.
- `artifacts/2013-terminal-carry-recursion-ledger.md`.
- `artifacts/2013-terminal-carry-source-lineage.md`.
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
- `docs/backlog/20260605-snow-code-deferred-science-review.md`
- `docs/work-packages/20260606-hphys0314-adr0017-snow-rm-reclassification-route-ledger-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/snowpack-settling-carry-recursion-ledger.json`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `Cargo.toml`
- `tests/integration/hphys0316_2013_terminal_carry_recursion_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260606-hphys0316-2013-terminal-carry-recursion-closure-001/**`

## Phase Plan

1. Contracts: amend canonical snow and water-balance contracts.
2. Contract tests: add HPHYS0316 recursive-carry tests.
3. Pre-implementation gate: run focused contract test before diagnostics.
4. Diagnostics: recurse through 2013 terminal carry lineage.
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

- All H1/H7/H39 spring-2016 recursive year-start rows are traced into the 2013
  terminal chain or remain owned `HOLD` with a concrete blocker.
- First material paired divergence is classified by source lane with source-line
  provenance.
- Any production edit has direct contract and source-line authority; otherwise
  no production edits are made.
- Full H1..H39 metrics are recorded.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.
