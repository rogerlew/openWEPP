# 20260522-sr03-soil-runtime-seam-expansion-001

## Status
- state: complete
- date: 2026-05-22
- timezone: UTC

## Objective
Expand the hillslope soil runtime seam from minimal seed symbols to full
layer/profile runtime surfaces required by `SC-SOIL-001`, `SC-WATBAL-001`, and
`SC-SUBHYD-001` consumer contracts.

## Why This Package Exists
SR01 identified that current openWEPP soil runtime projection exports only a
minimal subset (`solthk`, `dg`, `thetdr`, `thetfc`) and is insufficient for
full consumer-boundary closure. SR03 implements the soil-side seam expansion
queued by SR01 after SR02 slope seam completion.

## Scope
### Included
- Define authoritative expanded soil runtime seam contract for hillslope
  execution.
- Implement expanded soil parser -> runtime projection builder with typed
  guard/error behavior and no silent defaults.
- Project required layer/profile symbols for contracted downstream consumers.
- Add/update integration tests proving expanded parser-to-runtime soil closure.
- Produce explicit consumer-coverage matrix mapping projected symbols to
  `SC-SOIL-001`, `SC-WATBAL-001`, and `SC-SUBHYD-001` obligations.

### Explicitly Out of Scope
- Full cross-domain slope+soil alias registry closure (`SR04`).
- Global parser-to-runtime closure across all surfaces (`SR05`).
- Hillslope kernel consumer rewiring beyond seam projection ownership (`SR06`).

## Deliverables
1. Expanded soil runtime seam contract artifact:
   - `artifacts/soil-runtime-seam-contract.md`
2. Soil runtime builder implementation evidence:
   - `artifacts/soil-runtime-builder-implementation-evidence.md`
3. Consumer coverage matrix artifact:
   - `artifacts/soil-runtime-consumer-coverage-matrix.md`
4. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/sr03_disposition.md`
5. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr01-slope-soils-model-representation-discovery-001/artifacts/slope-soil-follow-on-wp-queue.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr01-slope-soils-model-representation-discovery-001/artifacts/openwepp-slope-soil-architecture-fit-analysis.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-sr02-slope-runtime-seam-contract-and-builder-001/package.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/soil.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`

## Intended Write Set
- `docs/work-packages/20260522-sr03-soil-runtime-seam-expansion-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `tests/integration/parser_runtime_seam_integration.rs`
- `crates/openwepp-input-contract/src/parsers/soil.rs` (if seam constraints require)
- `crates/openwepp-sim-contract/src/symbols.rs` (if local soil-symbol alias additions are required for seam validation)

## Phase Plan
### Phase 0 - Intake
- Confirm SR01/SR02 seam-boundary decisions and current soil runtime export
  gaps.

### Phase 1 - Contract and Projection Design
- Author expanded soil runtime seam contract.
- Define required symbol set and typed guard/error taxonomy.

### Phase 2 - Implementation and Tests
- Implement expanded soil runtime projection builder.
- Add integration coverage for happy-path projection and representative typed
  failures.

### Phase 3 - Verification and Disposition
- Execute required gates.
- Complete dual review + verification and final disposition artifacts.

## Exit Criteria
- Expanded soil runtime seam contract is explicit and source-backed.
- Runtime builder projects required layer/profile symbols beyond the minimal
  4-symbol seed.
- Integration tests prove parser-to-runtime soil seam closure and typed
  failure behavior.
- Consumer coverage matrix maps exported symbols to contracted soil/hydrology
  obligations.
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
