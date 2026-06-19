# R2A - Direct Runtime Skeleton

Status: queued.

Package type: implementation work package / array-native runtime R2A.

## Objective

Introduce the first implementation-capable R2+ direct-runtime slice after
PERFDEEP09: a distinct direct-frame type namespace, a no-op or shadow-only
direct executor skeleton selected once at run setup, and executable
no-compatibility proof hooks.

This package must not port hydrology math, claim runtime readiness, cut over
publication, or activate direct mode by default. Its purpose is to make the
direct-runtime boundary real enough that later R3/R4 packages can port complete
phase spans without building another compatibility island.

## Rationale

PERFDEEP09 closed the default-disabled R2 blocker with a final H2637 median of
`635.65 s`, under the `<= 676.67 s` gate. The roadmap now authorizes R2+
implementation from the completed R0/R1 planning envelope.

The R0/R1 package explicitly decided that existing `HillslopeDayFrame` and
`HillslopeLaneDenseState` are compatibility/transition types. R2A must therefore
start by creating a separate direct-runtime type boundary and proving that the
new direct skeleton does not call into compatibility execution surfaces.

## Scope

In scope:

- introduce a direct-runtime module namespace;
- introduce distinct direct-frame type shells such as `DirectRunFrame`,
  `DirectLaneFrame`, `DirectDayFrame`, `DirectPublicationFrame`, and
  `DirectPhaseView`;
- introduce a no-op or shadow-only direct executor entrypoint selected once at
  run setup behind an explicit opt-in or test-only selection path;
- add compile/static tests proving direct-frame storage does not contain
  `SymbolRegistry`, `BoundarySymbol`, `BoundaryValue`, writeback payloads,
  indexed surfaces, hot symbol tables, dense-refresh state, or dirty-flush
  state;
- add call-graph/static proof and runtime counters/audit hooks showing the
  direct skeleton does not enter `execute_with_kernel*`,
  `HillslopeKernelRequest`, `KernelWritebackPayload`,
  `HillslopeWritebackSurface`, `state_value_for_symbol`,
  `flux_value_for_symbol`, registry/hot-table, indexed-surface,
  dense-refresh, or dirty-flush paths;
- prove default-disabled compatibility execution remains zero-cost for the new
  direct skeleton;
- preserve PERFDEEP09's default-disabled H2637 median gate as a regression
  guard;
- update package artifacts, roadmap/catalog state, review, verification,
  disposition, and worker handoff.

Out of scope:

- direct hydrology, erosion, growth, decomposition, frost, snow, or publication
  math;
- porting a complete phase span;
- R3/R4 endpoint improvement claims;
- direct publication cutover;
- output schema, unit, metadata, or conservation-operand changes;
- canonical `SC-*` contract amendments unless execution discovers that the
  skeleton changes guard or output authority and the package is amended first;
- default activation or opt-in activation beyond a no-op/shadow skeleton;
- deleting compatibility runtime paths.

## Required Reading

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/package.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/r0-runtime-schema-planning.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/direct-frame-type-boundary-decision.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/r1-frame-constructor-projection-plan.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/publication-ledger-promotion-plan.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/no-compatibility-proof-plan.md`
- `docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001/artifacts/gate-results.md`

Required before Rust edits:

- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

Conditional:

- `tests/AGENTS.md` before editing root tests.
- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` if canonical contracts
  must change.
- `tools/owcmp/AGENTS.md` before editing comparator tooling.

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler/**`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/**`

## Dependencies

- PERFDEEP09 is `READY-FOR-R2` and removes the prior default-disabled blocker.
- The R0/R1 planning package defines the schema envelope, direct-frame
  type-boundary decision, constructor/projection plan, publication-ledger
  promotion plan, and no-compatibility proof method.
- `docs/architecture/array-native-runtime-specification.md` is the binding
  architecture authority.

## Intended Write Set

- `docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs` only if required for
  typed executor status or phase enumeration integration
- `crates/openwepp-runner/src/hillslope/**` only for one-time direct skeleton
  selection and default-disabled zero-cost proof
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` only if opt-in flag/env
  plumbing is required; no default behavior change
- `tests/integration/**` only for R2A direct-skeleton contract tests

Files outside this set require package amendment before edits.

## Phase Plan

1. Populate required-reading, owned-file, and pre-implementation artifacts.
2. Inventory current compatibility entrypoints and forbidden APIs.
3. Implement the direct-runtime module namespace and direct-frame type shells.
4. Implement a no-op or shadow-only direct executor entrypoint selected once at
   setup behind explicit opt-in/test selection.
5. Add static/compile tests for direct-frame type prohibitions and call-graph
   separation.
6. Add runtime counters/audit evidence for direct skeleton execution and
   default-disabled compatibility execution.
7. Run focused tests and prove no default-disabled construction/tax.
8. Run the PERFDEEP09 default-disabled H2637 regression gate.
9. Run full Rust closure gates, scoped docs lint, and `git diff --check`.
10. Complete line-count governance, dual review, finding disposition, dual
    verification, roadmap/catalog updates, disposition, and worker handoff.

## Acceptance Criteria

- Direct-runtime module namespace exists and is separate from compatibility
  `day_frame.rs` types.
- Direct-frame storage contains none of the prohibited compatibility types or
  mechanisms named in the R0/R1 type-boundary decision.
- Direct executor skeleton is selected once at setup behind explicit
  opt-in/test selection and is inactive by default.
- Static call-graph proof shows direct skeleton execution does not enter
  `execute_with_kernel*`, `HillslopeKernelRequest`, writeback payload/surface,
  symbol lookup, indexed surface, hot table, dense refresh, or dirty-flush
  paths.
- Runtime counter/audit evidence confirms zero forbidden direct-skeleton calls
  and zero direct-skeleton construction on the default-disabled compatibility
  path.
- Default-disabled H2637 final median remains `<= 676.67 s` with protected
  identity under the PERFDEEP09 policy.
- No phase math, output publication cutover, direct publication operand change,
  output schema/unit/metadata change, or default activation occurs.
- Full Rust closure gates pass:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`.
- Scoped markdown lint and `git diff --check` pass.
- Review and verification artifacts explicitly check Gate Evidence
  Non-Deferral, no-compatibility proof adequacy, default-disabled regression,
  protected-boundary integrity, and line-count governance.

## Conservation / Output Acceptance

This package must not change publication operands, units, metadata meaning,
output schema, conservation math, HBP/WAT/PASS/loss construction, or manifest
publication. If execution discovers that the skeleton requires any publication
operand or output authority change, stop and amend the package before
implementation.

## Contract-First Rule

No `SC-*` contract change is intended. If execution discovers a required change
to guard semantics, diagnostic attribution, output meaning, units, conservation
authority, or process physics, stop and re-scope before production edits.

## Security Impact Gate

No secrets, credentials, external network dependencies, user data, or production
host actions are in scope. New opt-in or test selection must fail closed and
must not silently mask missing dependencies or invalid direct-frame inputs.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only static-audit, benchmark runner, reviewer, and verifier subagents
for no-compatibility proof review, default-disabled H2637 regression runs,
closure-gate review, line-count-governance review, package artifact review, and
gate-legitimacy verification. Expected outputs are compact metrics, log paths,
call-graph findings, and review findings recorded in package artifacts. Write
access is limited to package artifacts unless this package is explicitly
amended.

## Deliverables

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/r2a-direct-runtime-skeleton-contract.md`
- `artifacts/direct-type-namespace-plan.md`
- `artifacts/direct-executor-selection-proof.md`
- `artifacts/no-compatibility-proof-checklist.md`
- `artifacts/default-disabled-regression-gate.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`

## Required Reading Budget

Local required-reading bytes total, excluding this new package file:
`212304`.

Disposition: `OK` (`<=400000` bytes).

See `artifacts/required-reading-map.md`.

## Autonomy

When authorized for execution, run this package end to end. Do not stop after
introducing type shells without proving the no-compatibility and default-disabled
gates. Do not proceed into R3 phase-span implementation.
