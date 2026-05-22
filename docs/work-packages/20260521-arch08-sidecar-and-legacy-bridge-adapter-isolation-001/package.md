# 20260521-arch08-sidecar-and-legacy-bridge-adapter-isolation-001

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Isolate legacy sidecar/HBP compatibility behavior into explicit edge adapters
for the greenfield openWEPP scientific hydrology simulation architecture.

## Why This Package Exists
ARCH02 established that parser and kernel internals must not absorb legacy I/O
compatibility behavior. ARCH08 creates a dedicated adapter boundary so core
simulation orchestration and kernels remain process-focused and typed.

## Parallel Execution Context
ARCH08 is intended to execute in parallel with:
- `20260521-arch09-unit-safe-boundary-types-001`
- `20260521-arch10-summary-accumulator-kernelization-001`

Concurrency governance:
1. Worker is not alone in the repository; do not revert or overwrite others.
2. Stay within ARCH08 write-set only.
3. Treat shared workspace files as quarantine-owned.
4. Record any shared-file need in `artifacts/worker-handoff.md` as a
   `shared-change-request`.

## Scope
### Included
- Implement dedicated legacy bridge crate for sidecar/HBP compatibility logic.
- Define typed adapter request/response/error surfaces.
- Keep sidecar normalization and compatibility policy at edge boundary only.
- Add crate-local tests for compatibility paths and typed failure outcomes.
- Document adapter boundary semantics and non-leakage rules.

### Explicitly Out of Scope
- Unit-safe boundary type rollout (`ARCH09`).
- Summary accumulator kernelization (`ARCH10`).
- Core kernel physics implementation changes.

## Worktree Execution Model
- Recommended worktree path: `/home/workdir/openWEPP/.worktrees/arch08-legacy-bridge`
- Recommended branch name: `arch08/legacy-bridge-adapter-isolation`
- Ownership rule: worker must stay within ARCH08 write-set and must not edit
  ARCH09/ARCH10-owned paths.

## Deliverables
1. Legacy bridge crate scaffold and implementation.
2. Typed sidecar/HBP adapter contract surfaces.
3. Crate-local test coverage for strict/compat behavior and error mapping.
4. Documentation for adapter boundary and policy.
5. Worker handoff notes:
   - `artifacts/worker-handoff.md`
6. Owned file manifest:
   - `artifacts/owned-file-manifest.md`
7. Gate evidence summary:
   - `artifacts/gate-results.md`
8. Closeout disposition:
   - `artifacts/arch08_disposition.md`
9. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch04-topology-graph-and-validation-gate-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/`

## Intended Write Set
- `crates/openwepp-legacy-bridge/**`
- `docs/architecture/legacy-sidecar-bridge-boundary.md`
- `docs/specifications/science-contracts/legacy-sidecar-bridge-contract.md`
- package-local artifacts under this work-package directory

## Shared-File Quarantine (Parallel Guard)
The following files are out of scope for direct edits during ARCH08 parallel
execution:
- `Cargo.toml`
- `Cargo.lock`
- `src/**`
- `tests/integration/**`
- `crates/openwepp-kernel-contract/**`

## Phase Plan
### Phase 0 - Boundary Freeze
- Freeze adapter ownership boundary and non-leakage constraints.
- Freeze typed edge error/status mapping expectations.

### Phase 1 - Crate and API Skeleton
- Scaffold `openwepp-legacy-bridge`.
- Define typed adapter input/output contracts.

### Phase 2 - Adapter Implementation
- Implement sidecar/HBP edge adapter behaviors with explicit strict/compat
  policy handling.

### Phase 3 - Tests and Documentation
- Add crate-local tests for nominal and failure paths.
- Publish adapter boundary architecture/spec docs.

### Phase 4 - Worker Gates and Closeout
- Run worker-local gates:
  1. `cargo fmt --manifest-path crates/openwepp-legacy-bridge/Cargo.toml --check`
  2. `cargo clippy --manifest-path crates/openwepp-legacy-bridge/Cargo.toml --all-targets -- -D warnings`
  3. `cargo test --manifest-path crates/openwepp-legacy-bridge/Cargo.toml`
- Record review/disposition/verification artifacts.

## Exit Criteria
- Legacy sidecar/HBP behaviors are isolated to edge adapter crate.
- Typed compatibility surfaces are explicit and tested.
- No unauthorized shared-file edits occurred.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal adapter and docs changes only.
