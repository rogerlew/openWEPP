# 20260522-sr06-consumer-ownership-wiring-hillslope-kernels-001

## Status
- state: complete
- date: 2026-05-22
- timezone: UTC

## Objective
Wire slope/soil runtime seam surfaces into hillslope consumer boundaries
(runoff, soil, water-balance, and percolation phase adapters) using typed error
propagation only.

## Why This Package Exists
SR01 queued `SR06` after parser-to-runtime integration closure (`SR05`) to
promote slope/soil seam surfaces into concrete hillslope consumer boundaries.
This package closes ownership wiring between runtime seam projection and
hillslope phase consumer execution surfaces without introducing fallback
wrappers.

## Scope
### Included
- Define explicit consumer-ownership wiring contract for hillslope phase
  consumers that ingest slope/soil runtime seam surfaces.
- Implement/adjust hillslope consumer boundary wiring for runoff, soil,
  water-balance, and percolation-related adapter surfaces.
- Enforce typed error propagation for missing/invalid required consumer inputs;
  no silent default substitution.
- Add/update integration coverage proving wired consumer boundaries receive
  required runtime symbols during hillslope scheduler execution.

### Explicitly Out of Scope
- Legacy comparator confidence-tier review (`SR07`).
- Watershed consumer-boundary rewiring outside hillslope scope.
- New parser schema expansion unrelated to consumer wiring.

## Deliverables
1. Hillslope consumer ownership wiring contract:
   - `artifacts/hillslope-consumer-ownership-wiring-contract.md`
2. Wiring implementation evidence artifact:
   - `artifacts/hillslope-consumer-wiring-implementation-evidence.md`
3. Consumer boundary coverage matrix:
   - `artifacts/hillslope-consumer-boundary-coverage-matrix.md`
4. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/sr06_disposition.md`
5. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr01-slope-soils-model-representation-discovery-001/artifacts/slope-soil-follow-on-wp-queue.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr05-parser-to-runtime-integration-closure-001/package.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/docs/architecture/hillslope-phase-scheduler-graph.md`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`

## Intended Write Set
- `docs/work-packages/20260522-sr06-consumer-ownership-wiring-hillslope-kernels-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-kernel-contract/src/lib.rs` (if consumer-boundary writeback typing requires extension)
- `tests/integration/parser_runtime_seam_integration.rs`
- `tests/integration/hillslope_consumer_boundary_integration.rs` (if dedicated consumer-boundary fixture coverage is added)

## Phase Plan
### Phase 0 - Intake
- Confirm SR05 integration closure outputs and current hillslope consumer
  boundary ownership gaps.

### Phase 1 - Wiring Contract
- Define canonical runtime-symbol to consumer-boundary ownership mapping for
  runoff/soil/watbal/perc hillslope phases.

### Phase 2 - Wiring Implementation and Tests
- Implement consumer-boundary wiring with typed error propagation only.
- Add/update integration tests for happy-path wiring and representative typed
  failures.

### Phase 3 - Verification and Disposition
- Execute required gates.
- Produce review/verification artifacts and final disposition.

## Exit Criteria
- Hillslope consumer boundaries ingest required slope/soil runtime surfaces for
  scoped phases with explicit ownership mapping.
- Missing/invalid required consumer inputs produce typed errors; no silent
  defaults are introduced.
- Integration evidence demonstrates wiring coverage across runoff/soil/watbal/
  percolation phase boundaries.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: hillslope consumer-boundary wiring and test closure only; no
  network/credential surface changes.
