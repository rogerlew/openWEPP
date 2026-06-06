# HPHYS0309 Snow Carry/Depletion Lineage Closure

Status: executed-hold

## Objective

Classify the immediate snow-state carry/depletion cause for the HPHYS0308
`snow-state-carry-depletion-hold` rows before any snow-producer,
branch-predicate, or downstream WB13/WB17/WB18/WB19/WB12 production edit.

## Rationale

HPHYS0308 proved that `58` branch-extra fixed-comparator melt-call keys are not
openWEPP branch-predicate evidence: openWEPP has already depleted snow depth at
those keys while the fixed comparator still reaches `melt.for`. The next
correctness step is to compare the carry state immediately before those keys:
fixed-comparator after-hour `snodpt`/`densgt` from the HPHYS0305 observe lane
against openWEPP `snow_runtime_depth_before_m` and hourly before/after snow
state.

## Included Scope

- Add canonical contract authority requiring day-start, prior-hour, and
  same-day depletion-lead evidence for HPHYS0308 snow-state carry holds.
- Add contract-derived tests that gate package artifacts and prohibit
  production edits without source-line proof.
- Generate a lineage ledger for every HPHYS0308
  `snow-state-carry-depletion-hold` row.
- Classify whether each row is explained by pre-day carry deficit, prior-day
  openWEPP meltout, same-day depletion lead, or incomplete baseline/openWEPP
  state evidence.
- Preserve the HPHYS0308 single H7 first-2013 openWEPP-extra row as baseline
  branch-instrumentation scope unless this package proves otherwise.

## Excluded Scope

- No production Rust kernel edits.
- Baseline Fortran source edits or new fixed-comparator instrumentation.
- WB13/WB17/WB18/WB19/WB12 compensation, storage retuning, ET retuning, or
  publication proxy changes.
- Reproducing the archived original negative-melt sign/scale bug rejected by
  ADR-0016/HPHYS0303.

## Deliverables

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-034`.
- `SC-WATBAL-001#INV-WATBAL-082`.
- `tests/integration/hphys0309_snow_carry_depletion_lineage_contract.rs`.
- `artifacts/hphys0309_snow_carry_depletion_lineage.py`.
- `artifacts/snow-carry-depletion-lineage-ledger.json`.
- `artifacts/snow-carry-depletion-lineage-summary.md`.
- `artifacts/snow-carry-depletion-lineage-method.md`.
- `artifacts/snow-carry-depletion-lineage-source-lineage.md`.
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
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/artifacts/snowd-branch-state-ordering-ledger.json`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/baseline-observe-identity.json`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/openwepp-trace-field-audit.json`
- `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-comparator-source-delta.patch`
- Fixed `wepp_260430` comparator commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `Cargo.toml`
- `tests/integration/hphys0309_snow_carry_depletion_lineage_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001/**`

## Phase Plan

1. Contracts: amend canonical snow and water-balance contracts.
2. Contract tests: add HPHYS0309 artifact/authority tests.
3. Pre-implementation gate: run focused contract test before production edits.
4. Diagnostics: run the HPHYS0309 carry/depletion lineage runner.
5. Production checkpoint: do not edit production code unless source-line proof
   identifies a source-owned openWEPP defect.
6. Validation: run focused tests, anti-evasion gates, and workspace gates.
7. Review: complete dual review, disposition findings, and complete dual
   verification.
8. Closeout: update disposition, worker handoff, artifact statuses, and README.

## Execution Outcome

HPHYS0309 classified all `58` HPHYS0308 snow-state carry/depletion rows without
authorizing a production edit:

- `45` rows route to `pre-day-carry-deficit-hold`;
- `13` rows route to `prior-day-openwepp-meltout-hold`;
- `0` production edits are authorized.

The package remains `HOLD` because the baseline-extra melt-call keys are
explained by openWEPP entering the key day with materially less carried snow
depth, or with no carried snow while the fixed comparator still carries snow.
That is prior-day/day-start snowpack carry-state lineage, not branch-predicate,
same-hour melt-term, WB13 publication, WB17 ET, WB18 storage, WB19 lateral, or
WB12 runoff authority.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits only when source-line provenance authorizes them.

## Exit Criteria

- Canonical contracts require day-start, prior-hour, and same-day snow-state
  carry/depletion evidence for HPHYS0308 baseline-extra rows.
- The diagnostic ledger covers all `58` HPHYS0308
  `snow-state-carry-depletion-hold` rows.
- Any production edit has direct baseline/source provenance; otherwise the
  package remains `HOLD`.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.
