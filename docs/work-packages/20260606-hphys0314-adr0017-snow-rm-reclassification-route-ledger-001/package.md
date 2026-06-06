# HPHYS0314 ADR0017 Snow/RM Reclassification Route Ledger

Status: executed-hold

## Objective

Reclassify the open HPHYS0298-HPHYS0313 H1/H7/H39 snow/`RM` and
water-balance findings under ADR0017, consolidate the current route ledger, and
produce the authoritative continuation order before any snow producer or
downstream water-balance implementation package proceeds.

## Rationale

ADR0017 ratified that fixed-baseline comparator agreement is a flag, not a
target. HPHYS0313 then split the remaining route evidence into three
`hourly-snowfall-input-lineage-hold` rows and three
`recursive-year-start-inherited-state-hold` rows, representing all `57` carried
HPHYS0309 rows. The next step should not add production physics. It should
normalize the route taxonomy, retract/supersede stale HPHYS0298-era
`OPENWEPP-DEFECTIVE` labels, and create a single owned continuation ledger that
distinguishes `HARNESS-SURFACE-MISMATCH`, `UNRESOLVED`,
`LEGACY-DEFECTIVE`, and `OPENWEPP-DEFECTIVE`.

## Included Scope

- Add canonical contract authority for ADR0017 HPHYS snow/`RM`
  reclassification and route-ledger closure.
- Add contract-derived tests for the reclassification route ledger, required
  taxonomy, no-production-edit posture, and artifact completeness.
- Consume HPHYS0298-HPHYS0313 dispositions, review artifacts, and ledgers.
- Reclassify stale snow/`RM` and water-balance rows under the ADR0017 taxonomy.
- Produce a route-ledger summary that explicitly owns the HPHYS0315 and
  HPHYS0316 continuation branches.
- Record full H1..H39 continuation metrics; for no-production-runtime-edit
  route-ledger packages, static carry-forward from the latest same-runtime full
  suite is valid when truthfully labeled.

## Excluded Scope

- No production Rust kernel edits.
- No new snow physics or heuristic process formulas.
- No WB13, WB17, WB18, WB19, WB12, melt-term, branch-predicate, or
  infiltration/storage compensation.
- No attempt to resolve the hourly snowfall source or recursive carry chain in
  this package beyond scoping and route ownership.

## Deliverables

- `SC-SNOWFREEZE-001` ADR0017 HPHYS reclassification invariant.
- `SC-WATBAL-001` ADR0017 HPHYS reclassification invariant.
- `tests/integration/hphys0314_adr0017_snow_rm_reclassification_contract.rs`.
- `artifacts/adr0017-snow-rm-reclassification-route-ledger.md`.
- `artifacts/route-consolidation-summary.md`.
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
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/disposition.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/snowpack-settling-carry-recursion-ledger.json`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `Cargo.toml`
- `tests/integration/hphys0297_snow_rm_defect_ledger_contract.rs`
- `tests/integration/hphys0313_snowpack_settling_carry_recursion_contract.rs`
- `tests/integration/hphys0314_adr0017_snow_rm_reclassification_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260606-hphys0314-adr0017-snow-rm-reclassification-route-ledger-001/**`

## Phase Plan

1. Contracts: amend canonical snow and water-balance contracts.
2. Contract tests: add HPHYS0314 taxonomy/ledger tests.
3. Pre-implementation gate: run focused contract test before diagnostics.
4. Diagnostics: build route ledger from HPHYS0298-HPHYS0313 evidence.
5. Metrics: record full H1..H39 metrics, using a current rerun or truthfully
   labeled same-runtime carry-forward when production runtime code is unchanged.
6. Production checkpoint: authorize no production code in this package.
7. Review: complete dual review, disposition findings, and dual verification.
8. Closeout: publish route ledger, metrics, disposition, and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Diagnostics only; no production code edits in this package.

## Exit Criteria

- All stale HPHYS0298-HPHYS0313 snow/`RM` route verdicts are reclassified under
  ADR0017.
- The route ledger explicitly accounts for the `3` hourly-snowfall-input rows
  and `3` recursive year-start rows from HPHYS0313.
- Full H1..H39 metrics are recorded for continuation, with execution or
  carry-forward truthfulness explicitly labeled.
- No production physics edits are made.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.

## Execution Notes

- HPHYS0314 made no production runtime edits.
- Full H1..H39 metrics are carried forward from the latest fixed-baseline
  same-runtime suite and labeled `Static` in `artifacts/full-39-suite-metrics.md`.
- Package remains `executed-hold` because HPHYS0315 and HPHYS0316 own the
  unresolved source-line continuations.
