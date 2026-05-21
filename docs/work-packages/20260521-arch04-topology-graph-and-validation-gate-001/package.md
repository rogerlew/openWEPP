# 20260521-arch04-topology-graph-and-validation-gate-001

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Implement the topology subsystem baseline for openWEPP:
- typed watershed/hillslope topology graph model
- deterministic pre-execution validation gate
- typed topology closure diagnostics aligned with ARCH03 status/closure contracts

This package converts ARCH02 topology requirements into executable, testable
runtime gates that must pass before timestep execution.

## Why This Package Exists
ARCH02 and the architecture draft require explicit topology ownership and
closure checks before simulation scheduling. ARCH03 established shared
status/closure contracts; ARCH04 applies that substrate to watershed/hillslope
structure validation.

Without this package, downstream scheduler/kernel packages cannot rely on a
validated graph boundary and risk late-stage topology failures.

## Scope
### Included
- Create a dedicated topology crate for graph modeling and validation.
- Implement typed node/edge model for hillslope, channel, and impoundment
  topology.
- Implement validation gate enforcing closure/invariant rules before execution.
- Integrate with `openwepp-sim-contract` status/closure primitives.
- Add focused integration tests for valid and invalid topology scenarios.
- Document topology model and validation semantics.

### Explicitly Out of Scope
- Hillslope phase scheduler implementation (`ARCH05`).
- Watershed dispatch scheduler implementation (`ARCH06`).
- Kernel trait boundary and writeback contracts (`ARCH07`).
- Legacy sidecar adapter isolation (`ARCH08`).

## Worktree Execution Model
- Recommended worktree path: `/home/workdir/openWEPP/.worktrees/arch04-topology`
- Recommended branch name: `arch04/topology-graph-validation-gate`
- Ownership rule: stay within ARCH04 write-set unless an explicit scope
  amendment is recorded in package artifacts.

## Deliverables
1. Topology crate scaffold and workspace integration.
2. Typed graph model for watershed/hillslope topology entities.
3. Pre-execution validation gate with typed closure diagnostics.
4. Integration with `openwepp-sim-contract` status/closure surfaces.
5. Integration tests for canonical pass/fail topology scenarios.
6. Worker handoff notes:
   - `artifacts/worker-handoff.md`
7. Owned file manifest:
   - `artifacts/owned-file-manifest.md`
8. Gate evidence summary:
   - `artifacts/gate-results.md`
9. Closeout disposition:
   - `artifacts/arch04_disposition.md`
10. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch02-simulation-subsystem-kernel-architecture-discovery/artifacts/openwepp-simulation-architecture-requirements.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch02-simulation-subsystem-kernel-architecture-discovery/artifacts/openwepp-subsystem-and-kernel-ownership-proposal.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md`

## Intended Write Set
- `crates/openwepp-topology/**`
- `Cargo.toml`
- `tests/integration/topology_graph_validation_gate.rs`
- `tests/fixtures/topology/**`
- `docs/architecture/topology-graph-model.md`
- `docs/specifications/science-contracts/topology-validation-gate.md`
- package-local artifacts under this work-package directory

## Phase Plan
### Phase 0 - Topology Contract Freeze
- Derive canonical node/edge/state requirements from ARCH02 + active watershed
  input contracts.
- Freeze topology invariant list and validation status code mapping.

### Phase 1 - Crate and Model Skeleton
- Add new topology crate and workspace wiring.
- Define public graph data model and typed validation API.

### Phase 2 - Validation Gate Implementation
- Implement topology graph materialization and closure checks.
- Emit typed diagnostics via `openwepp-sim-contract` status/failure surfaces.

### Phase 3 - Tests and Documentation
- Add integration tests for canonical valid graph and failure classes.
- Add/refresh topology model + validation-gate docs.

### Phase 4 - Quality Gates and Closeout
- Run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Record review/disposition/verification artifacts.

## Exit Criteria
- Topology crate exists and is wired into workspace.
- Validation gate enforces declared topology invariants with typed diagnostics.
- Integration tests cover canonical pass/fail topology patterns.
- All required gates pass and are recorded.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal graph/validation logic; no external service or network
  exposure changes.
