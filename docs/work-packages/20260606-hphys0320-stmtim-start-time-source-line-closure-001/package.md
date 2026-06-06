# HPHYS0320 Stmtim Start-Time Source-Line Closure

Status: complete

## Objective

Close the HPHYS0319 `stmtim-active-interval-divergence-hold` by source-line
classifying pinned-baseline `winter.for` storm-start normalization against
OpenWEPP SIMIMPL28 timing projection, then implementing and validating the
baseline-authoritative timing path if the proof holds.

This is the HPHYS0320 `stmtim` start-time source-line closure package.

## Rationale

HPHYS0319 recovered the missing fixed-baseline observe lane for H1/H7/H39 at
2013 day 11 hour 11. The paired values show matching precipitation amount,
rounded storm duration, and near-identical hourly temperature surfaces, but a
control-flow divergence:

- Fixed baseline records `wntdur = 11`, adjusted `wnttim = 1`, active interval
  `1`, snow branch `1`, and `hrsnow = 0.00074545 m`.
- OpenWEPP records `wntdur = 11`, `wnttim = 0`, active interval `0`, snow
  branch `0`, and `snow.hourly.stmtim.hrsnow_m_0011 = 0`.

The next package must not be another narrow diagnostic ledger. Per root
`AGENTS.md` right-sizing guidance, HPHYS0320 must carry the coherent timing seam
through contract authority, source-line classification, implementation when
authorized, evidence generation, and disposition.

## Included Scope

- Add canonical climate, snow/freeze, and water-balance contract authority for
  HPHYS0320 storm-start timing closure.
- Add contract-derived tests covering HPHYS0320 authority, package scope,
  source-line proof artifacts, implementation behavior, paired trace evidence,
  gate results, review disposition, verification, and handoff.
- Source-line classify pinned-baseline `/workdir/wepp-forest_260430_baseline`
  `winter.for:206-235` and `stmtim.for:43-64` against OpenWEPP
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`.
- If source-line proof authorizes it, implement baseline-authoritative
  storm-start normalization for OpenWEPP SIMIMPL28 before `stmtim`
  active-interval evaluation.
- Add focused runtime tests proving breakpoint `stmstr = 0` with the HPHYS0319
  event duration normalizes to `wnttim = 1`, activates hour 11, selects the snow
  branch, and emits `hrsnow ~= 0.00074545 m`.
- Regenerate H1/H7/H39 HPHYS0245 traces and compare against the HPHYS0319
  fixed-baseline observe lane.
- Run or truthfully carry forward H1..H39 metrics according to whether
  production behavior changed.
- Disposition the combined `57` carried rows for this timing seam or reroute
  them to a newly proven next source lane.

## Excluded Scope

- No snow producer, drift, melt-term, branch-predicate, WB13, WB17, WB18, WB19,
  or WB12 compensation.
- No heuristic storm-start normalization; implementation must trace to
  canonical contract text plus pinned-baseline source lines.
- No climate parser redesign beyond the timing projection seam needed for
  `winter.for` parity.
- No permanent edits to `/workdir/wepp-forest_260430_baseline`.
- No closure claim from comparator improvement alone; source-line ownership and
  independent correctness authority remain required for `OPENWEPP-DEFECTIVE`.

## Deliverables

- `SC-CLIMATE-001` HPHYS0320 storm-start timing invariant.
- `SC-SNOWFREEZE-001` HPHYS0320 snow/freeze timing closure invariant.
- `SC-WATBAL-001` HPHYS0320 water-balance consumer gate.
- Contract-derived HPHYS0320 integration test.
- Source-line classification artifact for baseline `winter.for`/`stmtim.for`
  and OpenWEPP SIMIMPL28 timing projection.
- Production implementation, if authorized by source-line proof.
- Focused runtime tests for the HPHYS0319 key and timing normalization.
- H1/H7/H39 regenerated trace ledger paired to HPHYS0319 fixed-baseline observe
  values.
- Full carried-row disposition and, if behavior changes, H1..H39 metrics.
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
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/artifacts/hphys0319_fixed_stmtim_observe.json`
- `docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/artifacts/paired-stmtim-observe-classification.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `/tmp/hphys0305_paired_melt_terms_20260605T000000Z/`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260606-hphys0320-stmtim-start-time-source-line-closure-001/**`

Temporary execution writes may use
`/tmp/hphys0320_stmtim_start_time_source_line_20260606T000000Z/**`.

## Phase Plan

1. Contracts: amend canonical climate, snow/freeze, and water-balance
   contracts for HPHYS0320 timing authority and closure gates.
2. Contract tests: add HPHYS0320 authority, package-scope, implementation, and
   artifact tests.
3. Pre-implementation gate: run focused contract authority tests before
   production code edits.
4. Source-line classification: prove whether baseline `winter.for:206-235`
   authorizes OpenWEPP breakpoint `stmstr` minimum-hour normalization before
   `stmtim` active-interval evaluation.
5. Implementation: if authorized, modify OpenWEPP SIMIMPL28 timing projection
   and add focused runtime tests; if not authorized, record why and keep
   production authorization false.
6. Evidence: regenerate H1/H7/H39 traces, compare against HPHYS0319
   fixed-baseline observe values, and run or truthfully carry forward H1..H39
   metrics.
7. Review: complete dual review, disposition findings, and dual verification.
8. Closeout: update disposition, handoff, artifact statuses, and README.

## Contract-First Sequence

The package followed contract-first sequencing:

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits only after source-line classification authorizes them.

HPHYS0320 starts with production edit authorization set to `false`.

## Exit Criteria

Complete closure requires:

- Canonical contracts explicitly authorize the storm-start timing behavior used
  by OpenWEPP SIMIMPL28.
- Source-line classification cites baseline `winter.for:206-235` and
  `stmtim.for:43-64` plus the homologous OpenWEPP SIMIMPL28 path.
- If source-line proof holds, focused runtime tests prove HPHYS0319 key
  behavior: `wntdur = 11`, `wnttim = 1`, active interval `1`, snow branch `1`,
  and `hrsnow ~= 0.00074545 m`.
- Regenerated H1/H7/H39 traces match the HPHYS0319 fixed-baseline observe lane
  for timing membership and hourly snowfall at 2013 day 11 hour 11, or any
  remaining divergence is rerouted with source evidence.
- The combined `57` carried rows are either closed for this timing seam or
  assigned to a newly proven next source lane.
- Dual review findings are dispositioned as `accepted`, `rejected`,
  `deferred`, or `follow-up`; accepted findings are fixed and verified.
- Dual verification confirms no review findings remain undispositioned.

Executed-hold closure is valid only when:

- Source-line classification disproves OpenWEPP timing ownership, or
- The timing fix is valid and implemented but residual paired evidence proves a
  different upstream lane that must be handled by a named follow-on package.

The package must remain `HOLD` if contract authority, source-line proof,
paired traces, or required validation cannot be completed.

## Security-Impact Gate

This package is local repository engineering work. It uses flat-file reads and
edits in the worktree plus local trace execution. It does not require external
systems, credentials, network access, or shell-interpolated subprocess
construction.

## Review and Verification Requirements

- Dual independent reviews are mandatory before final disposition.
- Every finding must be dispositioned as `accepted`, `rejected`, `deferred`, or
  `follow-up` with rationale.
- Accepted findings must be fixed and verified before closure.
- Rejected findings must explain why no change is required.
- Deferred or follow-up findings must be linked from disposition and worker
  handoff artifacts.
- Dual verification must confirm no review findings remain undispositioned.

## Truthfulness Labeling

Evidence artifacts must label claims with `Static:` when based on file/source
inspection and `Ran:` when based on executed commands. A validator such as
`cargo check` or focused contract tests is not a substitute for broader gates.
