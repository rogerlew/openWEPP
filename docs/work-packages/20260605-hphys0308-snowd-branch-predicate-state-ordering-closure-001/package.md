# HPHYS0308 Snowd Branch Predicate State-Ordering Closure

Status: executed-hold

## Objective

Execute the HPHYS0307 required continuation by diagnosing the fixed-baseline
`snowd.for` branch-predicate/state-ordering cause of the H1/H7/H39
baseline-extra and openWEPP-extra melt-call keys before any numeric melt-term
correction, snow-state carry patch, or downstream water-balance compensation.

## Rationale

HPHYS0307 showed that seven target rows have fixed-baseline melt-call keys
absent from openWEPP `snow_hourly_melt_branch_active`, one H7 first-2013 row
has an openWEPP-only active key, and H39 first-2013 remains a matched-mask
same-hour `cmelt`/`snodpt` lane. The next diagnostic must inspect exact
branch-extra timestamps and state surfaces so the continuation can distinguish
branch-predicate defects from upstream snowpack carry/depletion defects.

## Included Scope

- Amend canonical `SC-WATBAL-001` for branch-extra key state-ordering evidence.
- Add contract-derived tests for package scope, ledger shape, gate posture, and
  no downstream compensation.
- Extract branch-extra keys from the HPHYS0307 ledger.
- Load fixed-baseline HPHYS0305 observe data and openWEPP final
  `post_wb13` trace rows for those keys.
- Record per-key baseline melt-call state, openWEPP branch state, snow-depth /
  density / forcing surfaces (`snow_hourly_depth_before_m`,
  `snow_hourly_depth_available_m`, `snow_hourly_depth_after_m`), and inferred
  branch-predicate outcome.
- Route production edits only if a source-line-owned openWEPP predicate defect
  is proven; otherwise keep `HOLD`.

## Excluded Scope

- Numeric `amelt`/`bmelt`/`cmelt`/`dmelt` magnitude correction.
- Snowpack carry/depletion production edits unless source-line proof emerges in
  this package.
- WB13/WB17/WB18/WB19/WB12 compensation or residual redistribution.
- Fixed-baseline source changes, comparator branch/tag mutation, or remote ref
  actions.

## Deliverables

- Canonical `SC-WATBAL-001` branch-extra state-ordering amendment.
- Contract-derived integration test registered in `Cargo.toml`.
- Package-local diagnostic runner and generated key-level ledger/summary.
- Source-lineage/method artifacts and final `HOLD` disposition.
- Dual review, review disposition, and dual verification artifacts.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/artifacts/melt-call-branch-activation-ledger.json`
- `docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/artifacts/hphys0306_branch_active_observe_semantics.py`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/hphys0305_paired_melt_term_state.py`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`

## Intended Write Set

- `Cargo.toml`
- `tests/integration/hphys0308_snowd_branch_state_ordering_contract.rs`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/**`

## Phase Plan

1. Contracts: amend canonical `SC-WATBAL-001` with HPHYS0308 state-ordering
   evidence requirements. Complete.
2. Contract tests: add package/ledger/source-lineage/no-compensation tests.
   Complete.
3. Pre-implementation gate: run focused contract tests before production edits.
   Complete.
4. Diagnostics: generate key-level branch-extra state-ordering ledger.
   Complete.
5. Production edits: edit kernel code only if the diagnostic identifies a
   source-line-owned openWEPP defect.
   Not authorized; no production kernel edit made.
6. Validation: run focused tests, anti-evasion guards, and applicable Rust
   gates. Complete.
7. Review: dispatch dual review, disposition findings, and run dual
   verification. Complete.
8. Closeout: update disposition and worker handoff. Complete.

## Execution Outcome

HPHYS0308 classified `59` branch-extra keys without authorizing a production
edit:

- `58` baseline-extra keys route to `snow-state-carry-depletion-hold`;
- `1` H7 first-2013 openWEPP-extra key routes to
  `baseline-branch-instrumentation-hold`;
- `0` production edits are authorized.

The package remains `HOLD` because the baseline-extra keys are explained by
openWEPP already having zero snow-depth surfaces at those timestamps, while
fixed baseline still observes `melt.for`; that is a snow-state carry/depletion
continuation, not branch-predicate edit authority.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits only when source-line provenance authorizes them.

## Exit Criteria

- Contract and tests encode branch-extra state-ordering evidence requirements.
- Diagnostic ledger covers all HPHYS0307 branch-extra keys.
- Any production edit has direct baseline-source provenance; otherwise the
  package remains `HOLD`.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.
