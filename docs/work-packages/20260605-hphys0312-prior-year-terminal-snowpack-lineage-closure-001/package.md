# HPHYS0312 Prior-Year Terminal Snowpack Lineage Closure

Status: executed-hold

## Objective

Execute the HPHYS0311 required continuation by scanning the prior calendar year
for each inherited terminal snowpack delta and classifying the first material
fixed-comparator/openWEPP snowpack divergence that produced the prior-year
terminal carry-state residual.

## Rationale

HPHYS0311 proved that six day-1 H1/H7/H39 spring groups inherit their
snowpack depth/density deltas directly from prior-year terminal state. The
year-boundary carry path is source-line parity, so the next localization step
must move earlier within the prior calendar year before any producer,
publication, runoff, ET, storage, or lateral-flow edit is authorized.

## Included Scope

- Add canonical contract authority for HPHYS0312 prior-year terminal snowpack
  lineage classification.
- Add contract-derived tests for HPHYS0312 artifact completeness, fail-closed
  runner behavior, source-line evidence, route counts, and no-compensation
  disposition.
- Build a diagnostic runner that consumes HPHYS0311, HPHYS0305 fixed-observe
  logs, and HPHYS0305 openWEPP traces.
- For the six `prior-year-terminal-state-hold` groups, scan the full prior
  calendar year and locate the first material paired snowpack divergence using
  depth and density tolerances.
- Classify first-divergence lanes as within-year cold settling/depth update or
  year-start inherited state, preserving source-line citations and continuation
  needs.

## Excluded Scope

- No production Rust kernel edits unless source-line and paired-state evidence
  proves an openWEPP-owned defect.
- No baseline Fortran source edits or new fixed-comparator instrumentation.
- No WB13, WB17, WB18, WB19, WB12, branch-predicate, or melt-term compensation.
- No heuristic replacement for baseline snow physics.

## Deliverables

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-037`.
- `SC-WATBAL-001#INV-WATBAL-085`.
- `tests/integration/hphys0312_prior_year_terminal_snowpack_lineage_contract.rs`.
- `artifacts/hphys0312_prior_year_terminal_snowpack_lineage.py`.
- `artifacts/prior-year-terminal-snowpack-lineage-ledger.json`.
- `artifacts/prior-year-terminal-snowpack-lineage-summary.md`.
- `artifacts/prior-year-terminal-snowpack-lineage-method.md`.
- `artifacts/prior-year-terminal-snowpack-lineage-source-lineage.md`.
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
- `docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/snow-carry-source-line-parity-ledger.json`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/baseline-observe-identity.json`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/openwepp-trace-field-audit.json`
- Fixed `wepp_260430` comparator commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `Cargo.toml`
- `tests/integration/hphys0312_prior_year_terminal_snowpack_lineage_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/**`

## Phase Plan

1. Contracts: amend canonical snow and water-balance contracts.
2. Contract tests: add HPHYS0312 artifact/authority tests.
3. Pre-implementation gate: run focused contract test before production edits.
4. Diagnostics: run the prior-year terminal lineage diagnostic runner.
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

## Execution Outcome

HPHYS0312 represented all `57` HPHYS0309 rows carried by the six HPHYS0311
`prior-year-terminal-state-hold` groups without authorizing a production edit:

- `3` 2014-target rows route to `settling-depth-update-hold`; their first
  material prior-year divergence is 2013 day 11 hour 11 during cold existing
  snow, no-snowfall, no-melt settling/depth update.
- `3` 2016-target rows route to `year-start-inherited-state-hold`; their first
  material divergence is already present at 2015 day 1 hour 1.
- `0` production edits are authorized.

The package remains `HOLD` because the settling rows require full-precision
baseline `wdayct`/settling reconstruction before equation ownership can be
proven, and the 2016-target rows require another earlier-year carry-state scan.

## Exit Criteria

- Canonical contracts require HPHYS0312 prior-year terminal snowpack lineage
  classification before any downstream compensation.
- The diagnostic ledger covers all six HPHYS0311 inherited terminal groups and
  all `57` represented HPHYS0309 rows.
- Any production edit has direct baseline/source provenance; otherwise the
  package remains `HOLD`.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.
