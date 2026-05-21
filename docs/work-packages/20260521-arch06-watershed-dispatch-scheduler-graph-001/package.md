# 20260521-arch06-watershed-dispatch-scheduler-graph-001

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Implement the deterministic watershed dispatch scheduler graph as an
orchestrator subsystem, using ARCH03 status contracts and ARCH04 topology
validation output as hard preconditions.

## Why This Package Exists
ARCH02 requires explicit watershed routing/impoundment dispatch ordering before
kernel-boundary implementation. ARCH04 established validated topology surfaces;
ARCH06 turns watershed execution ordering into typed, testable scheduling
logic.

## Concurrent Execution Context
ARCH06 is intended to run concurrently with:
- `20260521-arch05-hillslope-phase-scheduler-graph-001`

Concurrency governance:
1. Worker is not alone in the repository; do not revert or overwrite others.
2. Stay within ARCH06 write-set only.
3. Treat shared files as quarantine-owned and request changes via handoff notes.
4. Rebase/adapt to upstream changes rather than forcing local assumptions.

## Scope
### Included
- Create watershed orchestrator crate with deterministic dispatch scheduler
  graph.
- Encode routing/impoundment execution dependencies and dispatch order rules.
- Enforce topology-validation precondition surface from `openwepp-topology`.
- Emit typed dispatch outcomes/status via `openwepp-sim-contract`.
- Add crate-local tests for dispatch ordering, precondition enforcement, and
  failure classification.
- Document dispatch scheduler graph semantics.

### Explicitly Out of Scope
- Hillslope phase scheduler implementation (`ARCH05`).
- Cross-orchestrator kernel trait boundary design (`ARCH07`).
- Comparator/reporting integration.

## Worktree Execution Model
- Recommended worktree path: `/home/workdir/openWEPP/.worktrees/arch06-watershed-scheduler`
- Recommended branch name: `arch06/watershed-dispatch-scheduler-graph`
- Ownership rule: worker must stay within ARCH06 write-set and must not edit
  ARCH05-owned paths.

## Deliverables
1. Watershed orchestrator crate scaffold and implementation.
2. Deterministic watershed dispatch scheduler graph API and execution engine.
3. Topology-precondition validation boundary wiring.
4. Crate-local scheduler tests.
5. Scheduler architecture/spec documentation.
6. Worker handoff notes:
   - `artifacts/worker-handoff.md`
7. Owned file manifest:
   - `artifacts/owned-file-manifest.md`
8. Gate evidence summary:
   - `artifacts/gate-results.md`
9. Closeout disposition:
   - `artifacts/arch06_disposition.md`
10. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/architecture/topology-graph-model.md`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/`
- `/home/workdir/openWEPP/crates/openwepp-topology/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch04-topology-graph-and-validation-gate-001/`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/status-taxonomy.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/topology-validation-gate.md`

## Intended Write Set
- `crates/openwepp-watershed-orchestrator/**`
- `docs/architecture/watershed-dispatch-scheduler-graph.md`
- `docs/specifications/science-contracts/watershed-dispatch-scheduler-contract.md`
- package-local artifacts under this work-package directory

## Shared-File Quarantine (Concurrent Guard)
The following files are shared coupling points and are **out of scope** for
ARCH06 direct edits during concurrent execution:
- `Cargo.toml`
- `Cargo.lock`
- `/home/workdir/openWEPP/src/**`
- `/home/workdir/openWEPP/tests/integration/**`

Any required shared-file update must be recorded in
`artifacts/worker-handoff.md` as a `shared-change-request` for later
integration application.

## Phase Plan
### Phase 0 - Scheduler Contract Freeze
- Finalize watershed dispatch phases, ordering constraints, and preconditions.
- Map dispatch outcomes to ARCH03 status taxonomy.

### Phase 1 - Crate and API Skeleton
- Scaffold `openwepp-watershed-orchestrator`.
- Define dispatch scheduler graph API and execution context interfaces.

### Phase 2 - Deterministic Scheduler Implementation
- Implement dispatch dependency graph and deterministic execution ordering.
- Enforce topology-validation precondition checks.
- Emit typed outcomes/violations.

### Phase 3 - Tests and Documentation
- Add crate-local tests covering nominal path, dispatch dependency violations,
  and precondition failures.
- Publish dispatch scheduler architecture/spec docs.

### Phase 4 - Worker Gates and Closeout
- Run worker-local gates:
  1. `cargo fmt --check`
  2. `cargo clippy --manifest-path crates/openwepp-watershed-orchestrator/Cargo.toml --all-targets -- -D warnings`
  3. `cargo test --manifest-path crates/openwepp-watershed-orchestrator/Cargo.toml`
- Record review/disposition/verification artifacts.

## Exit Criteria
- Deterministic watershed dispatch scheduler graph is implemented and tested.
- Topology precondition enforcement is explicit and typed.
- No unauthorized shared-file edits occurred during concurrent execution.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal orchestration crate and docs only.
