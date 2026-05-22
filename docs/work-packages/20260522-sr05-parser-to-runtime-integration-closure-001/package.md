# 20260522-sr05-parser-to-runtime-integration-closure-001

## Status
- state: complete
- date: 2026-05-22
- timezone: UTC

## Objective
Add integration-closure coverage proving slope and expanded soil parser outputs
reach hillslope runtime scheduler surfaces with typed guard failures and no
silent defaults.

## Why This Package Exists
SR01 queued `SR05` after SR02/SR03 seam delivery and SR04 alias continuity
closure. Before consumer rewiring (`SR06`), openWEPP needs explicit
automation-backed proof that slope+soil parser surfaces propagate into runtime
scheduler state surfaces without fallback behavior.

## Scope
### Included
- Author parser-to-runtime integration closure matrix for slope+soil seams.
- Add/update integration tests validating parser outputs are consumed by the
  hillslope scheduler runtime surface.
- Add/update typed-failure integration cases proving required slope/soil seam
  fields fail explicitly instead of defaulting.
- Capture coverage evidence mapping integration assertions to SR02/SR03/SR04
  seam and symbol-continuity obligations.

### Explicitly Out of Scope
- Hillslope consumer ownership rewiring (`SR06`).
- Legacy comparator confidence-tier review (`SR07`).
- New parser schema design beyond integration-closure test scope.

## Deliverables
1. Parser-to-runtime integration closure matrix:
   - `artifacts/parser-runtime-integration-closure-matrix.md`
2. Integration implementation and test evidence artifact:
   - `artifacts/parser-runtime-integration-implementation-evidence.md`
3. Runtime scheduler symbol coverage matrix:
   - `artifacts/runtime-scheduler-symbol-coverage-matrix.md`
4. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/sr05_disposition.md`
5. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr01-slope-soils-model-representation-discovery-001/artifacts/slope-soil-follow-on-wp-queue.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr02-slope-runtime-seam-contract-and-builder-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr03-soil-runtime-seam-expansion-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr04-symbol-alias-continuity-completion-001/package.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`

## Intended Write Set
- `docs/work-packages/20260522-sr05-parser-to-runtime-integration-closure-001/**`
- `docs/work-packages/README.md`
- `tests/integration/parser_runtime_seam_integration.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs` (if integration coverage requires seam guard refinements)
- `crates/openwepp-hillslope-orchestrator/src/lib.rs` (if scheduler-surface assertion hooks require updates)

## Phase Plan
### Phase 0 - Intake
- Confirm SR02/SR03 seam projections and SR04 alias continuity outputs targeted
  by integration closure.

### Phase 1 - Integration Closure Design
- Define integration matrix for happy-path and typed-failure closure paths.

### Phase 2 - Test Implementation
- Add/update integration tests for slope+soil runtime scheduler-surface
  propagation and no-silent-default guard behavior.

### Phase 3 - Verification and Disposition
- Execute required gates.
- Produce dual review/verification artifacts and final disposition.

## Exit Criteria
- Integration tests demonstrate slope and expanded soil parser outputs reach
  runtime scheduler surfaces used by hillslope execution.
- Integration tests include representative typed-failure paths for missing or
  invalid required seam fields and confirm no silent defaults.
- Coverage matrix traces integration assertions to SR02/SR03/SR04 closure
  obligations.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: parser/runtime integration tests and seam-surface assertions only;
  no network/credential surface changes.
