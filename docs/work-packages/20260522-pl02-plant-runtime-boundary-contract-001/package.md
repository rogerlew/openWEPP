# 20260522-pl02-plant-runtime-boundary-contract-001

## Status
- state: complete
- date: 2026-05-22
- timezone: UTC

## Objective
Define the openWEPP typed runtime boundary contract for plant/landuse/
growth/decomposition (PL) surfaces, including canonical symbol continuity,
mutable-state ownership splits, and strict parser-to-runtime seam
requirements for downstream implementation packages.

## Why This Package Exists
PL01 established baseline representation semantics and concluded with
`BOUNDARY_EXTEND_SERIES_REQUIRED`. The next required step is to define the
runtime boundary contract before implementing PL adapters/kernels so ownership,
phase ordering, and alias continuity are explicit and testable.

## Scope
### Included
- Author typed runtime boundary contract for PL surfaces.
- Define mutable-state ownership matrix across management schedule, growth,
  and decomposition/residue state families.
- Define canonical WEPP symbol continuity and alias requirements for PL
  boundary exports.
- Define strict parser-to-runtime seam requirements and typed-failure policy.
- Publish PL02 disposition and handoff requirements for PL03+ follow-ons.

### Explicitly Out of Scope
- Implementing runtime adapters (PL03) or kernel wiring (PL05/PL06).
- Executing comparator campaigns (PL08).
- Reworking PL01 discovery conclusions.

## Deliverables
1. PL runtime boundary contract:
   - `artifacts/pl-runtime-boundary-contract.md`
2. PL runtime state-surface map:
   - `artifacts/pl-runtime-state-surface-map.md`
3. PL ownership matrix:
   - `artifacts/pl-runtime-ownership-matrix.md`
4. Canonical symbol alias requirements:
   - `artifacts/pl-runtime-canonical-symbol-alias-requirements.md`
5. Parser-to-runtime seam requirements:
   - `artifacts/pl-runtime-seam-requirements.md`
6. PL02 follow-on implementation handoff:
   - `artifacts/pl02-follow-on-implementation-handoff.md`
7. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl02_disposition.md`
8. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl01-plant-landuse-growth-decomposition-model-representation-discovery-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl01-plant-landuse-growth-decomposition-model-representation-discovery-001/artifacts/plant-landuse-growth-decomposition-boundary-decision-record.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl01-plant-landuse-growth-decomposition-model-representation-discovery-001/artifacts/openwepp-plant-landuse-growth-decomposition-architecture-fit-analysis.md`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs`

## Intended Write Set
- `docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL01 decision constraints and architecture governance requirements.

### Phase 1 - Boundary Contract Authoring
- Author typed runtime boundary contract and explicit ownership/state splits.

### Phase 2 - Seam and Alias Requirements
- Author parser-to-runtime seam strictness requirements and canonical symbol
  alias requirement tables.

### Phase 3 - Disposition
- Complete review/verification artifacts and docs-only gate checks.

## Exit Criteria
- Runtime boundary contract is explicit for PL management/growth/decomp
  surfaces with no ownership ambiguity.
- Canonical symbol continuity requirements are concrete and implementable.
- Parser-to-runtime seam requirements are strict (typed failures, no silent
  defaults).
- Dual review and verification artifacts are complete.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: contract/specification package only.

## Execution Result

- PL runtime boundary contract was authored with explicit ownership surfaces for
  schedule controls, growth state, and decomposition/transition state.
- Canonical symbol alias requirements and strict seam requirements were
  documented as implementation-ready constraints for `PL03` and `PL04`.
- Docs-only governance artifacts, reviews, and verifications are complete.
