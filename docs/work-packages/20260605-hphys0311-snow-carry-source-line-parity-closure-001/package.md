# HPHYS0311 Snow Carry Source-Line Parity Closure

Status: executed-hold

## Objective

Execute the HPHYS0310 required continuation by comparing fixed-comparator
`snowd.for`/`winter.for` carry-state source lines against openWEPP snow runtime
projection and hourly snow-state update code for the seven HPHYS0310 first
divergence groups.

## Rationale

HPHYS0310 represented all `58` HPHYS0309 rows as seven carry-state groups and
kept production edits unauthorized. Six groups first diverged at day-1 hour-1
and one H1 2013 group diverged during early density/settling. HPHYS0311 must
determine whether those rows are source-line-owned openWEPP defects, inherited
prior-year terminal state, low-precision fixed-observe artifacts, or remaining
evidence gaps before any downstream water-balance edit.

## Included Scope

- Add canonical contract authority requiring source-line carry-state parity
  classification after HPHYS0310.
- Add contract-derived tests for HPHYS0311 artifacts, runner fail-closed
  posture, and no-compensation disposition.
- Build a diagnostic runner that consumes HPHYS0310, HPHYS0305 fixed-observe
  logs, HPHYS0305 openWEPP traces, and static source-line citations.
- For day-1 groups, compare fixed-comparator prior-year terminal `snodpt` and
  `densgt` against openWEPP prior-year terminal runtime depth/density, then
  compare the carried day-1 hour-1 state against the `winter.for:193` and
  `snowd.for:50-53` carry-copy source lines.
- For the H1 2013 density/settling group, compare previous-hour and current-hour
  depth/density states and flag whether fixed-observe precision is sufficient to
  authorize a production equation change.

## Excluded Scope

- No production Rust kernel edits unless source-line and paired-state evidence
  proves an openWEPP-owned defect.
- No baseline Fortran source edits or new fixed-comparator instrumentation.
- No branch-predicate, same-hour melt-term, WB13 publication, WB17 ET, WB18
  storage, WB19 lateral/percolation, or WB12 runoff compensation.
- No heuristic replacement for legacy snow physics.

## Deliverables

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-036`.
- `SC-WATBAL-001#INV-WATBAL-084`.
- `tests/integration/hphys0311_snow_carry_source_line_parity_contract.rs`.
- `artifacts/hphys0311_snow_carry_source_line_parity.py`.
- `artifacts/snow-carry-source-line-parity-ledger.json`.
- `artifacts/snow-carry-source-line-parity-summary.md`.
- `artifacts/snow-carry-source-line-parity-method.md`.
- `artifacts/snow-carry-source-line-parity-source-lineage.md`.
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
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/specifications/wepp-input-files/specs/snow.spec.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/work-packages/20260605-hphys0310-prior-day-snow-carry-divergence-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0310-prior-day-snow-carry-divergence-closure-001/artifacts/prior-day-snow-carry-divergence-ledger.json`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/baseline-observe-identity.json`
- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/openwepp-trace-field-audit.json`
- Fixed `wepp_260430` comparator commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`
- `/workdir/wepp-forest_260430_baseline/src/infile.for`
- `/workdir/wepp-forest_260430_baseline/src/inidat.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `Cargo.toml`
- `tests/integration/hphys0311_snow_carry_source_line_parity_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/**`

## Phase Plan

1. Contracts: amend canonical snow and water-balance contracts.
2. Contract tests: add HPHYS0311 artifact/authority tests.
3. Pre-implementation gate: run focused contract test before production edits.
4. Diagnostics: run the source-line parity diagnostic runner.
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

HPHYS0311 represented all `58` HPHYS0309 rows as `7` HPHYS0310 groups without
authorizing a production edit:

- `6` day-1 groups route to `prior-year-terminal-state-hold` because the
  day-1 hour-1 depth and density deltas exactly match the prior-year terminal
  snowpack deltas; the source-line carry-forward path itself is parity.
- `1` H1 2013 density/settling group routes to `fixed-observe-precision-hold`
  because previous-hour depth/density state is near-identical but the fixed
  observe lane is rounded and omits baseline `wdayct`, so a production settling
  equation defect is not proven.
- `0` production edits are authorized.

The package remains `HOLD` because the next source localization must move
earlier into the prior-year terminal snowpack trajectory or add full-precision
baseline settling/carry-state instrumentation.

## Exit Criteria

- Canonical contracts require HPHYS0311 source-line carry-state parity
  classification before any downstream compensation.
- The diagnostic ledger covers all seven HPHYS0310 groups and all `58`
  represented HPHYS0309 rows.
- Any production edit has direct baseline/source provenance; otherwise the
  package remains `HOLD`.
- Evidence artifacts label truthfulness (`Static:` vs `Ran:`).
- Dual review findings are dispositioned and dual verification is recorded.

## Security-Impact Gate

No security-sensitive runtime behavior, credentials, subprocess invocation, or
network operation is in scope. Package execution is local flat-file reads/edits
and local validation commands only.
