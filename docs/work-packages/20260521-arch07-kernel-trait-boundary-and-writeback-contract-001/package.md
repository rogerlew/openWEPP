# 20260521-arch07-kernel-trait-boundary-and-writeback-contract-001

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Implement explicit kernel trait boundaries and orchestrator-controlled writeback
contracts across hillslope and watershed execution surfaces.

This package establishes the typed seam where pure process kernels return
proposed state/flux updates while orchestrators remain the sole owner of
state-surface mutation and commit policy.

Domain context:
- openWEPP is a greenfield scientific hydrology simulation engine.
- This package is strictly simulation architecture and numerical contract work
  for hillslope/watershed modeling behavior.

## Why This Package Exists
ARCH02 identified kernel boundary ambiguity as a high rework risk. ARCH05/ARCH06
now provide deterministic scheduler infrastructure; ARCH07 must define the
contracted kernel invocation and writeback model before additional process
kernel implementations proceed.

## Scope
### Included
- Define a dedicated kernel contract crate for shared trait signatures and
  writeback state-update record types.
- Define typed kernel input/output contract surfaces for hillslope and
  watershed domains.
- Define orchestrator-owned writeback protocol (accept/reject/apply) with typed
  status and diagnostic outcomes.
- Wire orchestrators to consume kernel trait contracts without embedding kernel
  mutation ownership in kernel implementations.
- Add integration tests covering:
  - successful writeback application
  - rejected writeback on closure/invariant violations
  - deterministic failure classification and propagation
- Document kernel boundary and writeback ownership semantics.

### Explicitly Out of Scope
- Sidecar/legacy adapter isolation (`ARCH08`).
- Unit-safe boundary wrapper rollout (`ARCH09`).
- Comparator metadata routing (`ARCH11`).

## Worktree Execution Model
- Recommended worktree path: `/home/workdir/openWEPP/.worktrees/arch07-kernel-contract`
- Recommended branch name: `arch07/kernel-trait-writeback-contract`
- Ownership rule: stay within ARCH07 write-set unless scope amendment is
  explicitly recorded in package artifacts.

## Deliverables
1. Kernel contract crate scaffold and workspace integration.
2. Kernel trait signatures and typed writeback payload model.
3. Orchestrator writeback policy integration (hillslope + watershed).
4. Integration tests for writeback acceptance/rejection/error propagation.
5. Architecture/spec docs for kernel boundary + writeback contract.
6. Worker handoff notes:
   - `artifacts/worker-handoff.md`
7. Owned file manifest:
   - `artifacts/owned-file-manifest.md`
8. Gate evidence summary:
   - `artifacts/gate-results.md`
9. Closeout disposition:
   - `artifacts/arch07_disposition.md`
10. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/architecture/hillslope-phase-scheduler-graph.md`
- `/home/workdir/openWEPP/docs/architecture/watershed-dispatch-scheduler-graph.md`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/`
- `/home/workdir/openWEPP/crates/openwepp-topology/`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch05-hillslope-phase-scheduler-graph-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch06-watershed-dispatch-scheduler-graph-001/`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/status-taxonomy.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/closure-check-primitives.md`

## Intended Write Set
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `Cargo.toml`
- `Cargo.lock`
- `tests/integration/kernel_writeback_contract.rs`
- `docs/architecture/kernel-trait-boundary-and-writeback.md`
- `docs/specifications/science-contracts/kernel-writeback-contract.md`
- package-local artifacts under this work-package directory

## Phase Plan
### Phase 0 - Contract Freeze
- Freeze kernel invocation and writeback ownership invariants.
- Map contract semantics to ARCH03 status + closure primitives.

### Phase 1 - Kernel Contract Crate
- Add `openwepp-kernel-contract` crate and workspace wiring.
- Define trait signatures and typed request/response/writeback record structs.

### Phase 2 - Orchestrator Integration
- Integrate contract surfaces into hillslope and watershed orchestrators.
- Enforce orchestrator-controlled writeback commit/reject semantics.

### Phase 3 - Tests and Documentation
- Add integration tests for nominal and failure writeback paths.
- Publish architecture/spec docs for boundary and ownership semantics.

### Phase 4 - Quality Gates and Closeout
- Run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Record review/disposition/verification artifacts.

## Exit Criteria
- Kernel trait and writeback boundary are explicit, typed, and orchestrator-owned.
- Integration tests cover writeback acceptance and rejection behavior.
- All required gates pass and are recorded.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal contract/orchestration crate and docs changes only.
