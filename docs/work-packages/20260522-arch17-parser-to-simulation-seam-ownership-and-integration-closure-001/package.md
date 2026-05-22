# 20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Implement `CRF-005` and `CRF-010` remediation by defining and enforcing the
parser-to-simulation seam ownership boundary, then wiring parser outputs into
runtime-consumed simulation surfaces with explicit integration acceptance
evidence.

## Why This Package Exists
ARCH14 identified that parser/bridge crates and simulation/orchestrator
surfaces were not fully integrated via an explicit ownership seam contract.
ARCH17 closes that gap by codifying the seam and proving active consumption
paths.

## Scope
### Included
- Define canonical ownership boundary from parser outputs to simulation runtime
  state surfaces.
- Specify which parser surfaces are authoritative runtime inputs vs
  compatibility/control-only surfaces.
- Implement integration wiring so runtime/orchestrator surfaces consume parser
  outputs through explicit typed adapters/contracts.
- Add integration tests that prove end-to-end parser-to-runtime ingestion for
  selected representative inputs.
- Add acceptance checks that prevent root-package-only dependency masking from
  being treated as integration closure.
- Produce dual review/disposition/verification artifacts.

### Explicitly Out of Scope
- Scheduler hot-path optimization (`CRF-003`) beyond required integration-safe
  edits.
- HBP authority split/convergence remediation (`CRF-006`) except dependency
  annotations for ARCH18.
- Top-level `.run` or parquet boundary contract completion (`CRF-007`).

## Deliverables
1. Seam contract and ownership docs:
   - `artifacts/parser-to-simulation-seam-ownership-contract.md`
   - `artifacts/runtime-input-surface-classification.md`
2. Integration implementation and tests in relevant crates/workspace tests.
3. Integration acceptance evidence:
   - `artifacts/parser-to-runtime-integration-evidence.md`
4. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/arch17_disposition.md`
5. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-legacy-bridge/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`

## Intended Write Set
- `crates/openwepp-input-contract/**`
- `crates/openwepp-legacy-bridge/**`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `crates/openwepp-kernel-contract/**` (if seam adapters require)
- `tests/integration/**` (targeted seam integration suites)
- `Cargo.toml`
- `Cargo.lock`
- `docs/work-packages/20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md`

## Phase Plan
### Phase 0 - Seam Inventory
- Inventory parser outputs, bridge outputs, and runtime-consumed surfaces.

### Phase 1 - Ownership Contract
- Define authoritative seam ownership and classification.
- Resolve derived/compatibility vs runtime input ownership boundaries.

### Phase 2 - Integration Implementation
- Implement parser-to-runtime ingestion wiring.
- Add integration tests and acceptance guards.

### Phase 3 - Gates and Closeout
- Run required gates and capture evidence.
- Complete dual review/disposition/verification artifacts.

## Exit Criteria
- Parser-to-simulation seam ownership is explicitly documented and enforced.
- Runtime consumption path exists for targeted parser surfaces with
  integration-test evidence.
- No unresolved ownership ambiguity remains for in-scope surfaces.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Dual review and verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: architecture/integration wiring only.
