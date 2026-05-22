# 20260522-arch16-scheduler-hot-path-surface-optimization-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Implement `CRF-003` remediation by reducing scheduler hot-path allocation and
clone pressure while preserving deterministic ordering, typed seam contracts,
and writeback accept/reject/apply semantics.

## Why This Package Exists
ARCH14 identified structural hot-loop cost from cloning full state/flux
surfaces per phase/step invocation. ARCH15 closed typed seam migration
(`CRF-001`/`CRF-002`) and now enables targeted optimization without returning
to stringly boundaries.

## Scope
### Included
- Profile/scope current clone/allocation hotspots in hillslope and watershed
  scheduler kernel execution paths.
- Refactor kernel request surfaces and/or scheduler call protocol to reduce
  repeated full-map clone operations.
- Preserve deterministic ordering and existing status/writeback invariants.
- Add/adjust tests to prove behavioral parity and invariant closure.
- Add benchmark or repeatable timing evidence focused on hot-path deltas.
- Produce dual review/disposition/verification artifacts.

### Explicitly Out of Scope
- Parser-to-simulation seam ownership (`CRF-005`/`CRF-010`).
- HBP authority convergence (`CRF-006`).
- Top-level `.run` or parquet boundary contracts (`CRF-007`).

## Deliverables
1. Hot-path optimization implementation in:
   - `crates/openwepp-hillslope-orchestrator/**`
   - `crates/openwepp-watershed-orchestrator/**`
   - `crates/openwepp-kernel-contract/**` (only if required by call-surface redesign)
2. Performance evidence artifact:
   - `artifacts/hot-path-benchmark-and-allocation-evidence.md`
3. Contract impact artifact:
   - `artifacts/seam-impact-and-compatibility-notes.md`
4. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/arch16_disposition.md`
5. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`

## Intended Write Set
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `crates/openwepp-kernel-contract/**` (if seam protocol changes required)
- `tests/integration/kernel_writeback_contract.rs` (if affected)
- `Cargo.lock` (if dependency graph changes)
- `docs/work-packages/20260522-arch16-scheduler-hot-path-surface-optimization-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md`

## Phase Plan
### Phase 0 - Baseline
- Identify exact clone/allocation points and define baseline evidence capture.

### Phase 1 - Optimization Design
- Choose deterministic optimization approach (borrowed views, reduced copy
  surfaces, or phased immutable snapshots) with contract-safety analysis.

### Phase 2 - Implementation and Tests
- Implement hot-path reduction and update tests.
- Validate closure/status semantics remain correct.

### Phase 3 - Gates and Closeout
- Run required gates and benchmark evidence capture.
- Complete dual review/disposition/verification artifacts.

## Exit Criteria
- Clone/allocation pressure in scheduler hot-path is reduced with explicit
  before/after evidence.
- Deterministic scheduler order and failure/advisory semantics unchanged.
- Typed seam correctness and invariant guards remain intact.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Dual review and verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: local performance/architecture refactor only.
