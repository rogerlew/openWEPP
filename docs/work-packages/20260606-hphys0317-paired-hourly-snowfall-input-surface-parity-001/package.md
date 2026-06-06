# HPHYS0317 Paired Hourly Snowfall Input Surface Parity

Status: executed-hold

## Objective

Close the HPHYS0315 and HPHYS0316 follow-on route by capturing or
reconstructing paired fixed-baseline/openWEPP controlling input surfaces for
the 2013 day 11 hour 11 positive-`hrsnow` key, then classify source ownership
without downstream compensation.

This is the HPHYS0317 paired hourly snowfall input-surface parity package.

## Rationale

HPHYS0315 routed `24` H1/H7/H39 spring-2014 rows to an hourly snowfall
input-surface blocker. HPHYS0316 routed another `33` spring-2016 inherited rows
through the same 2013 terminal carry chain to that blocker. The known material
key is 2013 day 11 hour 11, where fixed baseline records
`hrsnow = 0.0007454545120708644 m` and openWEPP records homologous
`snow.hourly.snowfall_m_0011 = 0.0 m`. The source-code structure is
homologous, but ADR0017 requires paired same-unit/same-lineage input-surface
evidence before assigning `OPENWEPP-DEFECTIVE` ownership or authorizing a
production edit.

## Included Scope

- Add canonical climate, snow/freeze, and water-balance contract authority for
  paired hourly snowfall input-surface closure.
- Add contract-derived tests for HPHYS0317 contract authority, row totals,
  artifact status, source-line citations, and no-compensation posture.
- Preserve the combined `57` carried-row route from HPHYS0315 and HPHYS0316.
- Inspect available source, package, and fixture evidence for paired
  `rain`, `stmdur`, `wntdur`, `wnttim`, `hrtemp`, `rst`, `hrsnow`, `hrrain`,
  active-interval, and branch-choice values at 2013 day 11 hour 11.
- Classify parser forcing, daily climate input, phase partition, hourly
  distribution, trace/instrumentation, or unresolved ownership under ADR0017.
- Record full H1..H39 metrics as static carry-forward if no production runtime
  code changes are authorized.

## Excluded Scope

- No production Rust kernel edits unless paired source-line evidence proves an
  openWEPP-owned defect.
- No production edit from source-code resemblance alone.
- No WB13, WB17, WB18, WB19, WB12, branch-predicate, melt-term, snow-drift, or
  snowpack-state compensation.
- No heuristic phase-partition or snowfall correction.
- No fallback defaults for missing paired input-surface values.

## Deliverables

- `SC-CLIMATE-001` paired hourly snowfall input-surface invariant.
- `SC-SNOWFREEZE-001` combined 57-row snow/freeze route invariant.
- `SC-WATBAL-001` water-balance consumer gate.
- `tests/integration/hphys0317_hourly_snowfall_input_surface_parity_contract.rs`.
- `artifacts/paired-input-surface-ledger.md`.
- `artifacts/paired-input-surface-source-lineage.md`.
- `artifacts/full-39-suite-metrics.md`.
- Gate, review, verification, disposition, and worker-handoff artifacts.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/unit-governance.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/work-packages/20260606-hphys0315-hourly-snowfall-input-lineage-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260606-hphys0316-2013-terminal-carry-recursion-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/snowpack-settling-carry-recursion-ledger.json`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `Cargo.toml`
- `tests/integration/hphys0317_hourly_snowfall_input_surface_parity_contract.rs`
- `tests/integration/hphys0316_2013_terminal_carry_recursion_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260606-hphys0317-paired-hourly-snowfall-input-surface-parity-001/**`

## Phase Plan

1. Contracts: amend canonical climate, snow, and water-balance contracts.
2. Contract tests: add HPHYS0317 paired input-surface tests.
3. Pre-implementation gate: run focused contract authority test.
4. Diagnostics: inspect paired source lanes and available trace/fixture evidence.
5. Production checkpoint: edit production code only if source-owned defect is
   proven.
6. Metrics: record full H1..H39 semantic metrics for continuation.
7. Review: complete dual review, disposition findings, and dual verification.
8. Closeout: update disposition, handoff, artifact statuses, and README.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits only when source-line provenance authorizes them.

## Exit Criteria

- The combined `57` carried rows are represented without stale
  `OPENWEPP-DEFECTIVE` labels.
- The 2013 day 11 hour 11 paired input-surface status is classified with
  source-line provenance.
- Any production edit has direct contract and source-line authority; otherwise
  no production edits are made.
- Full H1..H39 metrics are recorded.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Execution Notes

HPHYS0317 amended canonical climate, snow/freeze, and water-balance contracts
before adding the contract-derived test. Static source and artifact inspection
preserved the combined `57` carried rows from HPHYS0315 and HPHYS0316 at the
2013 day 11 hour 11 positive-`hrsnow` key.

The available fixed-baseline/openWEPP evidence publishes the same-unit
snowfall-depth mismatch (`hrsnow = 0.0007454545120708644 m` versus
`snow.hourly.snowfall_m_0011 = 0.0 m`) but does not publish paired
controlling input surfaces for `rain`, `stmdur`, `wntdur`, `wnttim`,
`hrtemp`, `rst`, `hrrain`, active interval membership, or branch choice.
Source-code resemblance is not parity proof under ADR0017 and
`SC-CLIMATE-001#INV-CLIMATE-015`.

Final classification: `executed-hold`. Owner: `HPHYS0318`. No production Rust
kernel edits were made or authorized. No downstream WB13, WB17, WB18, WB19,
WB12, producer, or water-balance compensation is authorized from HPHYS0317.
The HPHYS0316 regression guard was refreshed to avoid pinning obsolete exact
contract-version numbers after HPHYS0317 advanced the same canonical
contracts, while still asserting the HPHYS0316 invariant and obligation text.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.
