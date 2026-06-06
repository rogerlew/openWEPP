# HPHYS0310 Prior-Day Snow Carry Divergence Closure

Status: executed-hold

## Objective

Reconstruct the first prior-day/day-start snowpack carry-state divergence for
HPHYS0309 snow carry holds before any snow-producer, branch-predicate, or
downstream WB13/WB17/WB18/WB19/WB12 production edit.

## Rationale

HPHYS0309 classified all `58` HPHYS0308 baseline-extra melt-call keys as prior
carry-state lineage: openWEPP entered the key day with materially less carried
snow depth, or with no snow while the fixed comparator still carried snow. The
next correctness step is to reconstruct the earlier day/hour where the paired
fixed-comparator and openWEPP snowpack states first materially diverge across
the affected H1/H7/H39 target windows.

## Included Scope

- Add canonical contract authority requiring episode-level prior carry-state
  divergence reconstruction before producer or downstream edits.
- Add contract-derived tests for HPHYS0310 artifacts and no-production-edit
  posture.
- Build a fail-closed diagnostic runner that consumes HPHYS0309 rows, HPHYS0305
  fixed-comparator observe logs, and HPHYS0305 openWEPP traces.
- Compare fixed-comparator `snodpt`/`densgt` against openWEPP hourly
  depth/density and daily runtime snow state.
- For each affected hillslope/window/year group, identify the first material
  depth divergence before the HPHYS0309 key day and classify candidate source
  lanes: accumulation/settling, corrected negative-melt state loss,
  retained/released rain, raw/routed melt magnitude, or incomplete evidence.
- Preserve the single H7 first-2013 openWEPP-extra key as baseline branch
  instrumentation scope unless carry-state evidence explains it.

## Excluded Scope

- No production Rust kernel edits.
- No baseline Fortran source edits or new fixed-comparator instrumentation.
- No WB13/WB17/WB18/WB19/WB12 compensation, storage retuning, ET retuning, or
  publication proxy changes.
- No reproduction of the archived original negative-melt sign/scale bug rejected
  by ADR-0016/HPHYS0303.

## Deliverables

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-035`.
- `SC-WATBAL-001#INV-WATBAL-083`.
- `tests/integration/hphys0310_prior_day_snow_carry_divergence_contract.rs`.
- `artifacts/hphys0310_prior_day_snow_carry_divergence.py`.
- `artifacts/prior-day-snow-carry-divergence-ledger.json`.
- `artifacts/prior-day-snow-carry-divergence-summary.md`.
- `artifacts/prior-day-snow-carry-divergence-method.md`.
- `artifacts/prior-day-snow-carry-divergence-source-lineage.md`.
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
- `docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001/artifacts/snow-carry-depletion-lineage-ledger.json`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/baseline-observe-identity.json`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/openwepp-trace-field-audit.json`
- `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-comparator-source-delta.patch`
- Fixed `wepp_260430` comparator commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `Cargo.toml`
- `tests/integration/hphys0310_prior_day_snow_carry_divergence_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0310-prior-day-snow-carry-divergence-closure-001/**`

## Phase Plan

1. Contracts: amend canonical snow and water-balance contracts.
2. Contract tests: add HPHYS0310 artifact/authority tests.
3. Pre-implementation gate: run focused contract test before production edits.
4. Diagnostics: run the prior-day snow carry divergence runner.
5. Production checkpoint: do not edit production code unless source-line proof
   identifies a source-owned openWEPP defect.
6. Validation: run focused tests, anti-evasion gates, and workspace gates.
7. Review: complete dual review, disposition findings, and complete dual
   verification.
8. Closeout: update disposition, worker handoff, artifact statuses, and README.

## Execution Outcome

HPHYS0310 represented all `58` HPHYS0309 carry/depletion rows as `7`
hillslope/window/year groups without authorizing a production edit:

- `6` groups route to `initial-carry-state-projection-hold`;
- `1` group routes to `density-settling-carry-state-hold`;
- `0` production edits are authorized.

The package remains `HOLD` because the first material paired divergences are
snow-episode initial/carry-state producer lanes, not branch-predicate, same-hour
melt-term, WB13 publication, WB17 ET, WB18 storage, WB19 lateral/percolation,
or WB12 runoff authority.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits only when source-line provenance authorizes them.

## Exit Criteria

- Canonical contracts require episode-level carry-state divergence evidence for
  HPHYS0309 carry holds.
- The diagnostic ledger covers every affected HPHYS0309 hillslope/window/year
  group.
- Any production edit has direct baseline/source provenance; otherwise the
  package remains `HOLD`.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.
