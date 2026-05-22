# 20260522-sr02-slope-runtime-seam-contract-and-builder-001

## Status
- state: complete
- date: 2026-05-22
- timezone: UTC

## Objective
Implement the first-class slope runtime seam for hillslope orchestration by
projecting canonical slope parser outputs into typed runtime boundary symbols
with explicit guard/error behavior.

## Why This Package Exists
SR01 concluded that slope+soil cannot be closed as an isolated parser boundary
and explicitly queued `SR02` as the first implementation follow-on. Current
openWEPP runtime seam coverage is soil-only; a slope runtime seam is required
before downstream climate/hydrology/erosion coupling can be promoted safely.

## Scope
### Included
- Define authoritative slope runtime seam contract for hillslope execution.
- Implement slope parser -> hillslope runtime projection builder with typed
  errors (no silent defaulting).
- Add/update integration tests proving parser-to-runtime slope symbol closure.
- Document symbol continuity expectations for projected slope surfaces used by
  hillslope runtime phases.

### Explicitly Out of Scope
- Full soil runtime seam expansion (`SR03`).
- Global canonical alias registry closure across slope+soil (`SR04`).
- Watershed runtime surface redesign unrelated to slope seam ownership.

## Deliverables
1. Slope runtime seam contract artifact:
   - `artifacts/slope-runtime-seam-contract.md`
2. Slope runtime builder implementation evidence:
   - `artifacts/slope-runtime-builder-implementation-evidence.md`
3. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/sr02_disposition.md`
4. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr01-slope-soils-model-representation-discovery-001/artifacts/slope-soil-boundary-decision-record.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr01-slope-soils-model-representation-discovery-001/artifacts/openwepp-slope-soil-architecture-fit-analysis.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/slope.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`

## Intended Write Set
- `docs/work-packages/20260522-sr02-slope-runtime-seam-contract-and-builder-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `tests/integration/parser_runtime_seam_integration.rs`
- `crates/openwepp-input-contract/src/parsers/slope.rs` (if seam constraints require)

## Phase Plan
### Phase 0 - Intake
- Confirm SR01 boundary decision constraints and current slope parser/runtime
  seam gap surface.

### Phase 1 - Contract and Builder Design
- Author slope runtime seam contract and explicit symbol projection rules.
- Define typed error taxonomy for slope seam guard failures.

### Phase 2 - Implementation and Tests
- Implement slope runtime projection builder.
- Add integration coverage for successful projection and representative guard
  failure.

### Phase 3 - Verification and Disposition
- Execute required gates.
- Produce dual review + verification artifacts and final disposition.

## Exit Criteria
- Slope runtime seam contract is explicit and source-backed.
- A callable slope runtime builder is implemented in hillslope orchestration.
- Parser-to-runtime integration tests cover slope seam happy path and at least
  one typed failure path.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: runtime seam projection and tests only; no credential/network
  surface changes.
