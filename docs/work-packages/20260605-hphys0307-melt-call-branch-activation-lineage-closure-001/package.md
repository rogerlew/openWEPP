# HPHYS0307 Melt-Call Branch Activation Lineage Closure

Status: executed-hold

## Objective

Execute the HPHYS0306 required continuation by localizing the eight-window
`melt-call-mask` divergence between fixed-baseline `winter.for`/`snowd.for`
`melt.for` call semantics and openWEPP `snow_hourly_melt_branch_active`
publication semantics before any numeric melt-term correction or downstream
hydrology compensation.

## Rationale

HPHYS0306 proved that fixed-baseline inactive melt hours are not zero-valued
term observations and that eight H1/H7/H39 target windows still have
branch-active mask gaps. Branch activation is a control-flow predicate, so the
next package must compare the baseline source predicates against openWEPP
branch publication before authorizing a kernel edit.

## Included Scope

- Amend canonical `SC-*` authority for melt-call branch activation lineage.
- Add contract-derived tests for package scope, source-line provenance, ledger
  classification, and no downstream compensation.
- Compare fixed-baseline `winter.for` unconditional `snowd` calls,
  `snowd.for` `melt.for` call predicates, and openWEPP
  `snow_hourly_melt_branch_active` publication logic.
- Reclassify the HPHYS0306 nine-row ledger into baseline-extra/openWEPP-extra
  branch activation lanes.
- Authorize production code edits only if source-line provenance identifies an
  openWEPP branch-activation defect in scope.
- Preserve H39 first-2013 same-hour `cmelt`/`snodpt` as a separate
  source-ordering lane if branch activation does not explain it.

## Excluded Scope

- Numeric `amelt`/`bmelt`/`cmelt`/`dmelt` magnitude correction.
- WB13/WB17/WB18/WB19/WB12 compensation or residual redistribution.
- Fixed-baseline source changes, comparator branch/tag mutation, or remote ref
  actions.
- Accepting defective negative melt behavior as an openWEPP production
  requirement.

## Deliverables

- Canonical contract amendment for branch activation lineage.
- Contract-derived integration test registered in `Cargo.toml`.
- Package-local diagnostic runner and ledger/summary/method artifacts.
- Static source-lineage artifact comparing baseline and openWEPP predicates.
- Updated package evidence artifacts, gate results, review disposition,
  verification artifacts, final disposition, and worker handoff.

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
- `docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/artifacts/branch-active-melt-term-ledger.json`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`

## Intended Write Set

- `Cargo.toml`
- `tests/integration/hphys0307_melt_call_branch_activation_contract.rs`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/**`

## Phase Plan

1. Contracts: amend canonical `SC-WATBAL-001` with branch-activation lineage.
   Complete.
2. Contract tests: add package and ledger coherence tests. Complete.
3. Pre-implementation gate: run focused contract tests before production edits.
   Complete.
4. Diagnostics: generate branch-activation source-lineage and ledger artifacts.
   Complete.
5. Production edits: edit kernel code only if the diagnostic identifies a
   source-line-owned openWEPP defect.
   Not authorized; no production kernel edit made.
6. Validation: run focused tests, anti-evasion guards, and broad Rust gates.
   Complete.
7. Review: dispatch dual review, disposition findings, and run dual
   verification. Complete.
8. Closeout: update disposition and worker handoff. Complete.

## Execution Outcome

HPHYS0307 classified all nine HPHYS0306 rows without authorizing a production
edit:

- seven rows route to `baseline-extra-melt-call-hold`;
- one H7 first-2013 row routes to `openwepp-extra-melt-call-hold`;
- one H39 first-2013 row remains `same-hour-multi-source-hold`.

The package remains `HOLD` because classification/source-lineage evidence did
not prove a source-line-owned openWEPP branch-predicate defect. The required
continuation is a baseline-authoritative `snowd.for` branch-predicate/state
ordering package focused on the H1/H7/H39 branch-extra keys.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits only when source-line provenance authorizes them.

## Exit Criteria

- Contract and tests encode branch activation lineage and compensation
  prohibitions.
- Diagnostic ledger classifies all nine HPHYS0306 target windows.
- Any production edit has direct baseline-source provenance; otherwise the
  package remains `HOLD`.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.
