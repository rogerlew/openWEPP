# ARCH23 Schedule Export and Introspection

Status: queued

## Objective

Execute `docs/architecture/schedule-export-and-introspection.md` by adding a deterministic, code-derived export and introspection surface for the live hillslope phase schedule DAG, then reconciling the drifted hand-maintained architecture and scheduler-contract documentation with the generated canonical artifacts.

## Rationale

The hillslope scheduler graph already exists as code in `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`, but the reviewable documentation is hand-maintained and has drifted. The current scheduler has 14 canonical phases, while `docs/architecture/hillslope-phase-scheduler-graph.md` and `docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md` still describe the stale 9-phase surface. This package removes that failure mode by making the schedule exportable, diffable, and gate-checked from the code source of truth.

This package is intentionally scoped as tooling, docs, and tests. It must not change runtime scheduler execution semantics. If execution reveals that runtime scheduler behavior or kernel-controlling branch projection must change, stop the implementation, record the issue in `artifacts/kernel-scope-screen.md`, move final disposition to `HOLD`, and queue a new kernel-affecting package using the root `AGENTS.md` kernel work-package procedure.

## Included Scope

- Implement a deterministic exporter for `HillslopePhaseGraph::canonical()` with Mermaid and JSON output, and Graphviz DOT output if it is low-risk.
- Add a local generator command that can refresh committed canonical artifacts without tests writing repository files by default.
- Add validation diagnostics for cycles, unreachable phases, and mismatches between `canonical_order()` and the dependency map, using the hillslope graph's own topological-order logic rather than watershed-specific `openwepp-topology` cycle detection.
- Add schedule diff support for two JSON exports, reporting added and removed nodes and edges deterministically.
- Add phase metadata where cheaply available: rank, phase name, mapped consumer adapter via `hillslope_consumer_adapter_for_phase()`, and concise precondition notes if already represented in code/docs.
- Commit generated canonical schedule artifacts and wire a `tools/release/` congruence gate that fails on drift.
- Reconcile the drifted architecture doc and the stale scheduler contract by replacing hand-maintained phase lists/edges with links to, or embedded excerpts from, generated artifacts while preserving prose for preconditions and halt semantics.
- Complete package artifacts, dual independent review, explicit finding disposition, dual verification, and final disposition.

## Excluded Scope

- Runtime scheduler execution changes.
- New scheduler graph definitions independent of `HillslopePhaseGraph::canonical()`.
- `SC-*` science-kernel contract changes.
- Watershed dispatch scheduler export; record it as follow-on only.
- Introducing `cargo xtask` unless a maintainer explicitly re-scopes the package.
- Network access, telemetry, or external service integration.

## Deliverables

- Export/introspection implementation in the hillslope orchestrator crate.
- A generator command invocable locally and in CI.
- Committed canonical schedule artifacts:
  - `docs/architecture/generated/hillslope-phase-schedule.mmd`
  - `docs/architecture/generated/hillslope-phase-schedule.json`
  - `docs/architecture/generated/hillslope-phase-schedule.dot` if implemented.
- A release gate script:
  - `tools/release/check_hillslope_schedule_export.sh`
- Tests for format output, validation diagnostics, congruence behavior, and synthetic schedule diff added/removed edge reporting.
- Reconciled docs:
  - `docs/architecture/hillslope-phase-scheduler-graph.md`
  - `docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md`
- Package evidence and disposition artifacts, including dual review and dual verification.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/architecture/schedule-export-and-introspection.md`
- `docs/architecture/hillslope-phase-scheduler-graph.md`
- `docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/package.md`
- `docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/artifacts/worker-handoff.md`
- `crates/openwepp-hillslope-orchestrator/Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `crates/openwepp-topology/src/lib.rs`
- `tools/release/README.md`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` only for minimal public/test surface required by the exporter.
- `crates/openwepp-hillslope-orchestrator/src/schedule_export.rs`
- `crates/openwepp-hillslope-orchestrator/src/bin/openwepp_hillslope_schedule_export.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `docs/architecture/generated/hillslope-phase-schedule.mmd`
- `docs/architecture/generated/hillslope-phase-schedule.json`
- `docs/architecture/generated/hillslope-phase-schedule.dot` if DOT is implemented.
- `docs/architecture/hillslope-phase-scheduler-graph.md`
- `docs/architecture/schedule-export-and-introspection.md` only for final disposition/status update.
- `docs/specifications/science-contracts/hillslope-phase-scheduler-contract.md`
- `tools/release/check_hillslope_schedule_export.sh`
- `tools/release/README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260604-arch23-schedule-export-and-introspection-001/**`

Do not edit `Cargo.toml` unless implementation cannot satisfy the interface without new dependencies. Prefer deterministic standard-library formatting over adding dependencies. If `Cargo.toml` must change, record the reason in `artifacts/implementation-test-evidence.md` and `artifacts/worker-handoff.md`.

## Phase Plan

1. Record scope screen and spec disposition.
   - Confirm this remains a read-only projection/tooling package.
   - Record current ARCH05 doc/contract drift evidence from source and docs.
2. Implement exporter/introspection APIs and generator command.
   - Consume `HillslopePhaseGraph::canonical()`.
   - Emit deterministic Mermaid and JSON.
   - Emit DOT if low-risk.
   - Return typed exporter errors for cycles, missing phases, malformed input, and diff parse failures.
3. Implement tests and release gate.
   - Add unit tests for deterministic output and diagnostics.
   - Add diff tests using synthetic JSON exports.
   - Add a release script that regenerates to a temporary path and compares against committed artifacts.
4. Reconcile documentation.
   - Commit generated artifacts.
   - Update the architecture graph doc and scheduler contract so phase lists and edge lists cannot drift independently.
   - Update `tools/release/README.md` with the new gate.
5. Run validation, reviews, and disposition.
   - Run focused crate gates and workspace gates where feasible.
   - Record truthfully labeled evidence.
   - Complete dual independent reviews, disposition every finding, run dual verification, and publish final disposition.

## Required Validation

Run and record results as `Ran:` evidence when execution reaches validation:

1. `cargo fmt --check`
2. `cargo clippy --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --all-targets -- -D warnings`
3. `cargo test --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml`
4. `bash tools/release/check_hillslope_schedule_export.sh`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo test --workspace`
7. `cargo deny check`

If workspace-wide gates are blocked by unrelated dirty-worktree changes or pre-existing failures, record the exact blocker and the narrower gates that did run. Do not claim a skipped gate passed.

## Dual Review and Finding Disposition Requirement

Before final package disposition, run two independent review passes and record them in `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`. Each finding must be dispositioned as `accepted`, `rejected`, `deferred`, or `follow-up` with rationale and evidence. Accepted findings must be fixed and verified. Rejected findings must explain why no change is required. Deferred or follow-up findings must be linked from `artifacts/disposition.md` and `artifacts/worker-handoff.md`. Package closure is blocked while any review finding is undispositioned.

Dual verification artifacts (`artifacts/verification_agent_a.md` and `artifacts/verification_agent_b.md`) must verify both technical gates and review finding disposition.

## Exit Criteria

- Mermaid and JSON generated artifacts reflect the live 14-phase `HillslopePhaseGraph::canonical()` order and edges.
- The congruence gate fails for an intentionally drifted committed artifact and passes after regeneration.
- The diff path reports a synthetic added and removed edge deterministically.
- The stale architecture graph doc and scheduler contract no longer carry independently maintained incorrect phase/edge lists.
- No runtime scheduler execution behavior changed.
- Required validation is run or skipped with truthful blocker evidence.
- Dual review findings are fully dispositioned and dual verification confirms no undispositioned findings remain.

## Security-Impact Gate

Security impact: low. This package is local repository engineering work limited to flat-file reads/edits and local command execution in the worktree. No external systems, network actions, credentials, or telemetry are required.

## Kernel Scope Applicability

Default classification: non-kernel-affecting tooling/docs/tests package.

Reason: the source spec says this is a read-only projection of the existing graph, with no runtime execution change, no new graph definition, and no `SC-*` science-kernel contract changes. The package may edit a non-`SC-*` scheduler contract doc only to reconcile stale phase-list documentation with generated artifacts.

HOLD trigger: if the implementation must change runtime phase ordering, scheduler branch behavior, kernel writeback behavior, or canonical `SC-*` physics contracts, stop and record a HOLD in `artifacts/kernel-scope-screen.md`. Do not continue under this package until a kernel-affecting work package is prepared under root `AGENTS.md`.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must execute all phases through disposition, update required artifacts with truthfulness labels (`Static:` versus `Ran:`), and only ask for user direction when hard-blocked by missing local authority or unavailable validation substrate.
