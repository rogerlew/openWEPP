# HPHYS0313 Snowpack Settling Carry Recursion Closure

Status: executed-hold

## Objective

Execute the HPHYS0312 required continuation by resolving both split routes for
inherited H1/H7/H39 terminal snowpack deltas: full-precision 2013 settling/depth
reconstruction for the `settling-depth-update-hold` route, and one-year-earlier
2014 terminal carry-state recursion for the `year-start-inherited-state-hold`
route.

## Rationale

HPHYS0312 proved that the six inherited terminal rows do not authorize a
producer or downstream water-balance edit yet. Three 2014-target rows first
materially diverge during cold existing-snow settling at 2013 day 11 hour 11,
but rounded fixed observe output is insufficient to prove equation ownership.
Three 2016-target rows are already divergent at 2015 day 1 hour 1, so the
lineage must recurse into the 2014 terminal carry state feeding that year-start
state.

## Included Scope

- Add canonical contract authority for HPHYS0313 split-route settling/carry
  recursion.
- Add contract-derived tests for artifact completeness, contract authority,
  fail-closed source-line checks, route counts, and no-compensation disposition.
- Build a diagnostic runner that consumes HPHYS0312 output and existing
  HPHYS0305 paired evidence, then emits a split-route ledger.
- For the three `settling-depth-update-hold` groups, reconstruct baseline and
  openWEPP cold-settling equation inputs/outputs for 2013 day 11 hour 11.
- For the three `year-start-inherited-state-hold` groups, scan 2014 carry-state
  lineage feeding 2015 day 1 hour 1 and classify the earliest available paired
  divergence.
- Preserve source-line proof requirements and no-compensation gates.

## Excluded Scope

- No production Rust kernel edits unless source-line and paired-state evidence
  proves an openWEPP-owned defect.
- No permanent baseline Fortran source edits.
- No WB13, WB17, WB18, WB19, WB12, branch-predicate, or melt-term compensation.
- No heuristic replacement for baseline snow physics.

## Deliverables

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-038`.
- `SC-WATBAL-001#INV-WATBAL-086`.
- `tests/integration/hphys0313_snowpack_settling_carry_recursion_contract.rs`.
- `artifacts/hphys0313_snowpack_settling_carry_recursion.py`.
- `artifacts/snowpack-settling-carry-recursion-ledger.json`.
- `artifacts/snowpack-settling-carry-recursion-summary.md`.
- `artifacts/snowpack-settling-carry-recursion-method.md`.
- `artifacts/snowpack-settling-carry-recursion-source-lineage.md`.
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
- `docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/artifacts/prior-year-terminal-snowpack-lineage-ledger.json`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/baseline-observe-identity.json`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/openwepp-trace-field-audit.json`
- Fixed `wepp_260430` comparator commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `Cargo.toml`
- `tests/integration/hphys0313_snowpack_settling_carry_recursion_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/**`

## Phase Plan

1. Contracts: amend canonical snow and water-balance contracts.
2. Contract tests: add HPHYS0313 artifact/authority tests.
3. Pre-implementation gate: run focused contract test before production edits.
4. Diagnostics: run the split-route settling/carry recursion diagnostic runner.
5. Production checkpoint: do not edit production code unless source-line proof
   identifies a source-owned openWEPP defect.
6. Validation: run focused tests, anti-evasion gates, and workspace gates.
7. Review: complete dual review, disposition findings, and complete dual
   verification.
8. Closeout: update disposition, worker handoff, artifact statuses, and README.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits only when source-line provenance authorizes them.

## Exit Criteria

- Canonical contracts require HPHYS0313 split-route settling/carry recursion
  before any downstream compensation.
- The diagnostic ledger covers all six HPHYS0312 inherited terminal groups and
  all `57` represented HPHYS0309 rows.
- Settling rows record full-precision reconstruction status for `wdayct`,
  `densgy`, `setf`, `densgt`, and depth update at 2013 day 11 hour 11.
- Year-start rows recurse into the 2014 terminal carry-state chain feeding 2015
  day 1 hour 1.
- Any production edit has direct baseline/source provenance; otherwise the
  package remains `HOLD`.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.
