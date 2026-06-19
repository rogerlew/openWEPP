# PERFDEEP08 - Disabled-Path Hard Isolation

Status: executed 2026-06-19. Disposition: HOLD.

HOLD reason: the only scoped candidate tested, disabled diagnostic-hook caching
for PERFDEEP02 roundtrip and indexed-shadow hooks, measured `691.93 s` /
`229444 KB` on H2637 default-disabled, slower than the PERFDEEP07 retained
`685.85 s` point and above the P0 `<= 676.67 s` threshold. The candidate was
reverted and no production Rust change was retained. R2+ direct-frame runtime
implementation remains blocked.

Package type: performance defect closure / disabled-path hard isolation.

## Objective

Close the PERFDEEP07 default-disabled hold before any R2+ direct-frame runtime
implementation.

When all PERFDEEP opt-ins are disabled, the H2637 default path must not pay for
dense/direct-frame compatibility plumbing. The package must identify, remove,
or hard-gate remaining always-on dense-first, indexed shadow, hot-table,
writeback, frame-shadow, or symbol-resolution setup that exists only for failed
PERFDEEP opt-in islands.

The package does not implement direct-frame hydrology, direct execution, output
publication cutover, or default activation.

## Rationale

PERFDEEP07 retained a partial disabled-path cleanup that improved H2637 from
PERFDEEP05's `701.95 s` to `685.85 s`, but it did not meet the P0 threshold
`<= 676.67 s`. Because PERFDEEP07 held before direct-frame implementation, R2+
runtime work remains blocked by the R0/R1 planning package and the revised
array-native runtime architecture.

This package is the narrow follow-up: prove the default-disabled path is flat
and zero-cost-when-disabled, or hold with a precise remaining blocker. It must
not use the work to start another compatibility island or a direct-frame
executor.

## Scope

In scope:

- reproduce or re-baseline the retained PERFDEEP07 disabled-path state;
- audit all default-disabled setup and hot paths for remaining dense-first,
  indexed shadow, direct-frame, hot-symbol, writeback payload, frame-shadow, or
  symbol-resolution work;
- patch hard isolation so failed PERFDEEP02/03/05/07 plumbing is not
  constructed, resolved, refreshed, or flushed when opt-ins are off;
- preserve explicit opt-in behavior behind the existing fail-closed flags;
- preserve default-disabled H2637 output identity for HBP, WAT, PASS, plot/loss,
  and manifest/provenance surfaces protected by earlier PERFDEEP packages;
- record rejected candidates and slower experiments so they are not repeated
  without new evidence;
- prove the disabled-path gate with at least three clean H2637 no-UI endpoint
  runs and a median `<= 676.67 s`;
- update artifacts, roadmap/catalog state, line-count governance, review,
  verification, and final disposition.

Out of scope:

- direct-frame hydrology implementation;
- direct executor skeletons;
- R2+ runtime schema implementation;
- output schema or publication cutover;
- physics or numerical formula changes;
- canonical `SC-*` contract changes unless the package is amended first;
- default activation of any PERFDEEP opt-in;
- accepting an opt-in speedup while the default-disabled gate remains failed.

## Required Reading

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/package.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/package.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/disposition.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/gate-results.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/perfdeep07-disabled-path-audit.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/perfdeep07-zero-cost-disabled-proof.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/perfdeep07-hold-lift-disposition.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/no-compatibility-proof-plan.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`

Required before Rust edits:

- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `tests/AGENTS.md` before editing root tests

Source inventory:

- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/**`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/**`

## Dependencies

- PERFDEEP07 is the immediate HOLD this package must close or refine.
- The R0/R1 planning package blocks R2+ direct-frame implementation until this
  hold is closed or superseded.
- The revised array-native runtime specification remains the architecture
  authority.
- Existing H2637 identity and publication surfaces are protected boundaries.

## Intended Write Set

- `docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` only if unavoidable;
  if touched, the package must first add a split or explicit line-count closure
  plan that satisfies local governance before final closure.
- `crates/openwepp-hillslope-orchestrator/src/phase.rs` only for routing metadata
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/**`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/**`

Any additional production write set requires amending this package before the
edit.

## Phase Plan

1. Populate required-reading and owned-file artifacts. Confirm execution is
   hard isolation for the disabled path, not direct-frame implementation.
2. Reproduce the retained PERFDEEP07 baseline or record a same-machine
   replacement baseline before edits.
3. Audit default-disabled setup and hot paths for remaining dense/direct-frame
   compatibility construction, lookup, refresh, flush, or symbol resolution.
4. Patch hard isolation in the smallest production surface that prevents the
   disabled path from building or resolving opt-in-only machinery.
5. Run focused tests and default-disabled identity checks after each viable
   candidate. Record rejected/slower candidates.
6. Run the P0 endpoint gate: at least three clean H2637 no-UI default-disabled
   runs with all PERFDEEP opt-ins off, min/median/max seconds and RSS, and
   median `<= 676.67 s`.
7. Prove zero-cost-disabled statically and, where feasible, with runtime
   counters or trace evidence. The proof must show opt-in-only structures are
   not constructed on the disabled path.
8. Run full closure gates when the P0 gate passes:
   `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`, scoped docs lint, and
   `git diff --check`.
9. Complete line-count governance, dual review, finding disposition, dual
   verification, roadmap/catalog updates, and worker handoff.
10. Close as `READY-FOR-R2`, `HOLD`, or `NO-GO`. `READY-FOR-R2` requires the
    P0 gate and all closure gates to pass.

## Acceptance Criteria

- Default-disabled H2637 identity passes for protected output surfaces.
- At least three clean default-disabled H2637 no-UI endpoint runs are recorded
  with all PERFDEEP opt-ins off.
- The three-run median is `<= 676.67 s`; min/max seconds and RSS are recorded.
- Same-machine baseline/control evidence is recorded or a missing-control
  reason is stated.
- Static proof shows disabled execution does not construct, resolve, refresh,
  or flush dense/direct-frame compatibility machinery.
- Focused regression tests cover any new disabled-path guard or bypass.
- All previous opt-ins remain explicit, fail-closed, and non-default.
- No direct-frame hydrology, direct executor, R2+ runtime schema, or publication
  cutover is implemented.
- Full Rust closure gates pass when the package claims `READY-FOR-R2`.
- Markdown lint passes for the package and touched docs.
- Line-count governance is recorded before closure; any touched file over the
  threshold is split or otherwise closed per local governance before completion.
- Review findings are dispositioned as `accepted`, `rejected`, `deferred`, or
  `follow-up`; accepted findings are fixed and verified.
- Both reviews and both verifications explicitly check Gate Evidence
  Non-Deferral.

## Conservation / Output Acceptance

This package must preserve current output schemas and protected identity
surfaces. It may not change publication operands, metadata meaning,
normalization, conservation math, or output units.

If execution discovers a need to change output meaning, stop and amend the
package under the contract-first and conservation/publication rules before
production edits.

## Contract-First Rule

No physics or canonical `SC-*` contract change is intended. If execution
discovers that disabled-path isolation requires changing invariant authority,
guard semantics, diagnostic attribution policy, output meaning, units, or
process physics, stop and amend the package before implementation.

## Security Impact Gate

No secrets, credentials, external network dependencies, or user data are in
scope. Do not weaken fail-closed behavior, typed error handling, validation
gates, output schema contracts, or serialization safeguards.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only reviewer and verifier subagents for package artifact review,
disabled-path hard-isolation proof review, regression-gate review, line-count
governance review, and gate-legitimacy verification. It also explicitly
authorizes spawning/delegating to comparator or batch-runner subagents for H2637
endpoint/identity runs if the local tooling supports it. Expected outputs are
compact findings or metrics recorded in the package artifacts. Write access is
limited to artifact files unless this package is explicitly amended.

## Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/perfdeep08-disabled-path-baseline.md`
- `artifacts/perfdeep08-disabled-path-audit.md`
- `artifacts/perfdeep08-hard-isolation-plan.md`
- `artifacts/perfdeep08-zero-cost-disabled-proof.md`
- `artifacts/perfdeep08-rejected-candidates-ledger.md`
- `artifacts/perfdeep08-h2637-identity-timing-evidence.md`
- `artifacts/perfdeep08-r2-blocker-disposition.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
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

## Autonomy

Execute end-to-end when triggered. Do not proceed into R2 or direct-frame
runtime implementation from this package. Do not ask the user for next steps
unless a hard blocker prevents a truthful `READY-FOR-R2`, `HOLD`, or `NO-GO`
disposition.
