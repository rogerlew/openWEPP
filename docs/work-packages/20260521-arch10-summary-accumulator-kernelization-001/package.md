# 20260521-arch10-summary-accumulator-kernelization-001

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Implement kernelized summary accumulation (daily/monthly/yearly/EOS) with typed
status outcomes for the greenfield openWEPP scientific hydrology simulation.

## Why This Package Exists
ARCH02 requires summary accumulation to be an explicit kernelized phase,
separate from scheduling/orchestration side effects. ARCH10 establishes this
surface so reporting/comparator metadata routing can build on stable contracts.

## Parallel Execution Context
ARCH10 is intended to execute in parallel with:
- `20260521-arch08-sidecar-and-legacy-bridge-adapter-isolation-001`
- `20260521-arch09-unit-safe-boundary-types-001`

Concurrency governance:
1. Worker is not alone in the repository; do not revert or overwrite others.
2. Stay within ARCH10 write-set only.
3. Treat shared workspace files as quarantine-owned.
4. Record any shared-file need in `artifacts/worker-handoff.md` as a
   `shared-change-request`.

## Scope
### Included
- Implement dedicated summary-accumulator kernel crate.
- Define typed accumulator inputs/outputs and status behavior for
  daily/monthly/yearly/EOS windows.
- Add crate-local tests for accumulation correctness and boundary transitions.
- Document summary accumulator contract and phase semantics.

### Explicitly Out of Scope
- Legacy sidecar/HBP adapter implementation (`ARCH08`).
- Unit-safe boundary type rollout (`ARCH09`).
- Comparator-tier routing metadata integration (`ARCH11`).

## Worktree Execution Model
- Recommended worktree path: `/home/workdir/openWEPP/.worktrees/arch10-summary-accumulator`
- Recommended branch name: `arch10/summary-accumulator-kernelization`
- Ownership rule: worker must stay within ARCH10 write-set and must not edit
  ARCH08/ARCH09-owned paths.

## Deliverables
1. Summary accumulator crate scaffold and implementation.
2. Typed accumulation contract surfaces for daily/monthly/yearly/EOS.
3. Crate-local tests for windowing and accumulation correctness.
4. Documentation for accumulator semantics and status behavior.
5. Worker handoff notes:
   - `artifacts/worker-handoff.md`
6. Owned file manifest:
   - `artifacts/owned-file-manifest.md`
7. Gate evidence summary:
   - `artifacts/gate-results.md`
8. Closeout disposition:
   - `artifacts/arch10_disposition.md`
9. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch06-watershed-dispatch-scheduler-graph-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/status-taxonomy.md`

## Intended Write Set
- `crates/openwepp-summary-accumulator/**`
- `docs/architecture/summary-accumulator-kernelization.md`
- `docs/specifications/science-contracts/summary-accumulator-contract.md`
- package-local artifacts under this work-package directory

## Shared-File Quarantine (Parallel Guard)
The following files are out of scope for direct edits during ARCH10 parallel
execution:
- `Cargo.toml`
- `Cargo.lock`
- `src/**`
- `tests/integration/**`
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`

## Phase Plan
### Phase 0 - Accumulator Contract Freeze
- Freeze accumulation window definitions and status semantics.
- Freeze typed input/output contract shape.

### Phase 1 - Crate and API Skeleton
- Scaffold `openwepp-summary-accumulator`.
- Define accumulator interfaces and summary state structures.

### Phase 2 - Kernelization Implementation
- Implement daily/monthly/yearly/EOS accumulator logic.
- Enforce typed status/reporting outcomes.

### Phase 3 - Tests and Documentation
- Add crate-local tests for window transitions and rollups.
- Publish accumulator architecture/spec docs.

### Phase 4 - Worker Gates and Closeout
- Run worker-local gates:
  1. `cargo fmt --manifest-path crates/openwepp-summary-accumulator/Cargo.toml --check`
  2. `cargo clippy --manifest-path crates/openwepp-summary-accumulator/Cargo.toml --all-targets -- -D warnings`
  3. `cargo test --manifest-path crates/openwepp-summary-accumulator/Cargo.toml`
- Record review/disposition/verification artifacts.

## Exit Criteria
- Summary accumulator kernel is implemented and tested.
- Typed status semantics for summary windows are explicit.
- No unauthorized shared-file edits occurred.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal accumulator and docs changes only.
