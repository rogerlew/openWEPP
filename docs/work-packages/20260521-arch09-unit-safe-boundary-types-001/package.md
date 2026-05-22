# 20260521-arch09-unit-safe-boundary-types-001

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Introduce unit-safe boundary types for critical hydrologic state/flux surfaces
in the greenfield openWEPP scientific hydrology simulation architecture.

## Why This Package Exists
ARCH02 and ARCH07 identify boundary-type ambiguity as a correctness risk.
ARCH09 applies dimensional/type safety at kernel and orchestrator boundaries to
reduce class errors and improve contract clarity.

## Parallel Execution Context
ARCH09 is intended to execute in parallel with:
- `20260521-arch08-sidecar-and-legacy-bridge-adapter-isolation-001`
- `20260521-arch10-summary-accumulator-kernelization-001`

Concurrency governance:
1. Worker is not alone in the repository; do not revert or overwrite others.
2. Stay within ARCH09 write-set only.
3. Treat shared workspace files as quarantine-owned.
4. Record any shared-file need in `artifacts/worker-handoff.md` as a
   `shared-change-request`.

## Scope
### Included
- Implement dedicated units/boundary-types crate for simulation interfaces.
- Define typed wrappers for selected runoff/flow/storage/rate boundaries.
- Provide typed conversion helpers and guarded construction paths.
- Add crate-local tests for unit/domain safety behavior.
- Document boundary-type policy and conversion constraints.

### Explicitly Out of Scope
- Legacy sidecar/HBP adapter implementation (`ARCH08`).
- Summary accumulator kernelization (`ARCH10`).
- Broad replacement of all internal scalar arithmetic.

## Worktree Execution Model
- Recommended worktree path: `/home/workdir/openWEPP/.worktrees/arch09-unit-boundary`
- Recommended branch name: `arch09/unit-safe-boundary-types`
- Ownership rule: worker must stay within ARCH09 write-set and must not edit
  ARCH08/ARCH10-owned paths.

## Deliverables
1. Unit-safe boundary crate scaffold and implementation.
2. Typed boundary wrappers + conversion helpers.
3. Crate-local tests for valid/invalid construction and conversion behavior.
4. Documentation for boundary type policy.
5. Worker handoff notes:
   - `artifacts/worker-handoff.md`
6. Owned file manifest:
   - `artifacts/owned-file-manifest.md`
7. Gate evidence summary:
   - `artifacts/gate-results.md`
8. Closeout disposition:
   - `artifacts/arch09_disposition.md`
9. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch03-sim-contract-crate-and-status-taxonomy-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/status-taxonomy.md`

## Intended Write Set
- `crates/openwepp-unit-boundary/**`
- `docs/architecture/unit-safe-boundary-types.md`
- `docs/specifications/science-contracts/unit-safe-boundary-types-contract.md`
- package-local artifacts under this work-package directory

## Shared-File Quarantine (Parallel Guard)
The following files are out of scope for direct edits during ARCH09 parallel
execution:
- `Cargo.toml`
- `Cargo.lock`
- `src/**`
- `tests/integration/**`
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`

## Phase Plan
### Phase 0 - Type Surface Freeze
- Freeze initial boundary variable set and unit semantics.
- Freeze conversion and guard policy expectations.

### Phase 1 - Crate and API Skeleton
- Scaffold `openwepp-unit-boundary`.
- Define typed wrappers and constructor/validation interfaces.

### Phase 2 - Type Implementation
- Implement boundary wrappers and conversion paths.
- Enforce non-finite/domain validation at construction boundaries.

### Phase 3 - Tests and Documentation
- Add crate-local tests for safety invariants.
- Publish boundary-type architecture/spec docs.

### Phase 4 - Worker Gates and Closeout
- Run worker-local gates:
  1. `cargo fmt --manifest-path crates/openwepp-unit-boundary/Cargo.toml --check`
  2. `cargo clippy --manifest-path crates/openwepp-unit-boundary/Cargo.toml --all-targets -- -D warnings`
  3. `cargo test --manifest-path crates/openwepp-unit-boundary/Cargo.toml`
- Record review/disposition/verification artifacts.

## Exit Criteria
- Unit-safe boundary types exist and are tested.
- Conversion and construction guards are explicit.
- No unauthorized shared-file edits occurred.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal type-system and docs changes only.
