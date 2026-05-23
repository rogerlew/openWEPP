# 20260522-pl07-parser-to-runtime-integration-tests-001

## Status
- state: complete
- date: 2026-05-22
- timezone: UTC

## Objective
Add integration tests asserting full PL runtime surface projection from `.man`
fixtures, including typed reject paths.

## Why This Package Exists
PL01 follow-on sequencing identifies PL07 as the integration-closure package
after PL03 adapter and PL04 alias continuity work. PL07 provides fixture-backed
evidence that parser outputs project into expected PL runtime surfaces without
silent defaults.

## Scope
### Included
- Add parser-to-runtime integration tests covering PL schedule, growth, and
  decomposition/resup runtime surfaces.
- Add typed reject-path tests for missing/invalid/non-finite required PL
  projection inputs.
- Assert runtime surface coverage from representative `.man` fixture classes
  and document the coverage matrix.
- Record integration evidence, review/verification outputs, and package
  disposition.

### Explicitly Out of Scope
- New kernel process behavior changes for growth or decomposition beyond test
  scaffolding assertion updates.
- Comparator confidence-tier campaign work (`PL08`).
- Broad parser feature expansion outside PL projection seam requirements.

## Deliverables
1. Fixture/runtime projection coverage matrix:
   - `artifacts/pl07-fixture-runtime-projection-coverage-matrix.md`
2. Typed reject-path catalog:
   - `artifacts/pl07-typed-reject-path-catalog.md`
3. Runtime surface assertion map:
   - `artifacts/pl07-runtime-surface-assertion-map.md`
4. Parser-to-runtime integration evidence:
   - `artifacts/pl07-parser-to-runtime-integration-evidence.md`
5. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl07_disposition.md`
6. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/artifacts/pl03-runtime-surface-projection-map.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/artifacts/pl03-typed-error-taxonomy.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl04-pl-symbol-alias-completion-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl04-pl-symbol-alias-completion-001/artifacts/pl04-canonical-symbol-alias-table.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl05-growth-kernel-surface-scaffolding-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl06-decomposition-resup-kernel-surface-scaffolding-001/package.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/tests/integration/`

## Intended Write Set
- `tests/integration/**`
- `crates/openwepp-hillslope-orchestrator/**` (only if seam/test hooks require)
- `docs/work-packages/20260522-pl07-parser-to-runtime-integration-tests-001/**`

## Phase Plan
### Phase 0 - Intake
- Confirm PL03 seam contracts, PL04 alias expectations, and fixture inventory.

### Phase 1 - Coverage Matrix
- Define fixture classes and expected runtime projection coverage targets.

### Phase 2 - Integration Test Closure
- Add positive and typed-negative parser-to-runtime integration assertions.

### Phase 3 - Verification
- Run targeted and required gates; capture evidence and risk notes.

### Phase 4 - Disposition
- Finalize review/verification artifacts and package disposition.

## Exit Criteria
- Fixture-backed tests assert full PL runtime surface projection coverage in
  scope.
- Typed reject paths are explicit and tested with no silent defaults.
- Integration evidence and assertion maps are complete and internally
  consistent.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: integration-test and docs closure package.
