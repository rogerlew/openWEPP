# HPHYS0319 Fixed-Baseline Stmtim Observe Recovery

Status: executed-hold

## Objective

Recover fixed-baseline `stmtim.for` observe values for the H1/H7/H39 2013 day
11 hour 11 positive-`hrsnow` route, pair them with regenerated OpenWEPP
`snow.hourly.stmtim.*_0011` diagnostics, and classify the combined `57`
carried rows without production physics or downstream water-balance edits.

This is the HPHYS0319 fixed-baseline `stmtim` observe recovery package.

## Rationale

HPHYS0318 added OpenWEPP-side SIMIMPL28 `stmtim` control-surface trace
instrumentation, but fixed-baseline paired values for `rain`, `stmdur`,
rounded `wntdur`, adjusted `wnttim`, `hrtemp`, `rst`, `hrrain`, `hrsnow`,
active interval membership, and branch choice remained unavailable. The route
therefore stayed ADR0017 `UNRESOLVED` under
`paired-fixed-baseline-stmtim-observe-hold`.

HPHYS0319 must recover those fixed-baseline observe values at the same key and
same lineage before any producer, branch, or downstream water-balance ownership
claim is made.

## Included Scope

- Add canonical climate, snow/freeze, and water-balance contract authority for
  HPHYS0319 fixed-baseline `stmtim` observe recovery and paired classification.
- Add contract-derived tests for the HPHYS0319 contract authority, package
  autonomy, temporary baseline instrumentation scope, artifact completion, and
  no-production-edit posture.
- Use a temporary `/tmp` worktree of the pinned fixed baseline
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` to instrument observe-only
  `stmtim.for`/`winter.for` surfaces.
- Run H1/H7/H39 fixed-baseline observe recovery for the 2013 day 11 hour 11
  key using existing paired run inputs.
- Regenerate H1/H7/H39 OpenWEPP HPHYS0245 traces after the HPHYS0318
  instrumentation and pair the `snow.hourly.stmtim.*_0011` values with the
  fixed-baseline observe values.
- Preserve the combined `57` carried rows and classify the route as an evidence
  ledger result without authorizing production code changes.

## Excluded Scope

- No production Rust precipitation-phase physics edit.
- No permanent edit to `/workdir/wepp-forest_260430_baseline`.
- No snow producer, drift, melt-term, branch-predicate, WB13, WB17, WB18,
  WB19, or WB12 compensation.
- No source-ownership claim from source-code resemblance alone.
- No fallback defaults, surrogate physics, silent clamping, or
  canonicalize-and-proceed behavior for missing paired control surfaces.
- No expansion of the HPHYS route beyond the combined `57` carried rows unless
  the package evidence proves a narrower owner and opens a separate follow-on.

## Deliverables

- `SC-CLIMATE-001` HPHYS0319 fixed-baseline `stmtim` observe recovery
  invariant.
- `SC-SNOWFREEZE-001` HPHYS0319 fixed-baseline snow/freeze observe recovery
  invariant.
- `SC-WATBAL-001` HPHYS0319 paired observe water-balance classification gate.
- Contract-derived HPHYS0319 integration test.
- Package-local fixed-baseline observe recovery script and generated
  instrumented patch evidence.
- Fixed-baseline/OpenWEPP paired `stmtim` observe ledger for H1/H7/H39.
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
- `docs/work-packages/20260606-hphys0318-stmtim-control-surface-instrumentation-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/wepp_observe.for`
- `/tmp/hphys0305_paired_melt_terms_20260605T000000Z/`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `Cargo.toml`
- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/**`

Temporary execution writes are limited to `/tmp/hphys0319_fixed_stmtim_observe_20260606T000000Z/**`.

## Phase Plan

1. Contracts: amend canonical climate, snow/freeze, and water-balance
   contracts.
2. Contract tests: add HPHYS0319 authority, package, script, artifact, and
   no-production-edit tests.
3. Pre-implementation gate: run focused contract authority tests before
   temporary baseline instrumentation or trace execution.
4. Implementation/evidence: run temporary fixed-baseline observe
   instrumentation and regenerate OpenWEPP H1/H7/H39 traces.
5. Classification: publish paired observe ledger, route classification, and
   full H1..H39 carry-forward posture for the `57` rows.
6. Review: complete dual review, disposition findings, and dual verification.
7. Closeout: update disposition, handoff, artifact statuses, and README.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Temporary observe instrumentation and trace execution.
5. Production code edits only if paired evidence and canonical contracts
   explicitly authorize them.

HPHYS0319 starts with production edit authorization set to `false`.

## Exit Criteria

- Canonical contracts require fixed-baseline observe recovery before source
  ownership or downstream water-balance edits.
- Focused HPHYS0319 tests pass before execution and after artifact closeout.
- Fixed-baseline observe values exist for H1/H7/H39 at 2013 day 11 hour 11, or
  any failure to recover them is recorded as a hard blocker with command logs.
- Regenerated OpenWEPP traces include `snow_hourly_stmtim_*` values at hour
  `0011` for the same key.
- Paired ledger records same-unit deltas, classification, and no-production-edit
  authorization unless the evidence explicitly proves a bounded next owner.
- Dual review findings are dispositioned as `accepted`, `rejected`,
  `deferred`, or `follow-up`; accepted findings are fixed and verified.
- Dual verification confirms no undispositioned findings remain.

## Security-Impact Gate

This package is local repository engineering work. It uses only flat-file reads
and edits in the worktree plus temporary `/tmp` baseline instrumentation for
local evidence generation. It does not require external systems, credentials,
network access, or shell-interpolated subprocess construction.

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
