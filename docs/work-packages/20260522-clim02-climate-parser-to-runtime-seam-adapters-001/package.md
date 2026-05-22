# 20260522-clim02-climate-parser-to-runtime-seam-adapters-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Implement CLIM02 by wiring climate parser outputs into first-class
runtime-consumed adapter seams for hillslope and watershed orchestrators,
with typed climate runtime errors and integration-test closure evidence.

## Why This Package Exists
CLIM01 established climate behavior/spec authority and identified
`CLIM-ARCH-GAP-001` as the blocking implementation gap: climate parser output
exists, but there is no parser-to-runtime adapter seam in orchestrator crates.
CLIM02 closes this seam gap and prepares the climate runtime surfaces needed by
CLIM03/CLIM04.

## Scope
### Included
- Implement `HS-CLIM-SEAM-001` and `WS-CLIM-SEAM-001` parser-to-runtime
  climate adapters in orchestrator runtime input surfaces.
- Define and implement typed climate runtime error taxonomy (`CLIM-RUNTIME-E-*`)
  for seam/adapter failures.
- Wire climate parser outputs into immutable runtime climate request surfaces
  consumed by simulation scheduling/execution boundaries.
- Enforce ratified CLIM01 policy at seam boundaries:
  - support explicit `datver=0.0` override (`iclig=0`)
  - support CLIGEN `4.0+` branch (`iclig=1`)
  - reject pre-4 nonzero branch (`0.0<datver<4.0`, `iclig=2`) via explicit typed guard.
  - enforce strict breakpoint `dtime>0` guard for all intervals (duplicate or
    decreasing `timem` hard-fail, regardless of `drain`).
- Add targeted integration tests proving parser-to-runtime climate seam closure.
- Produce dual review/disposition/verification artifacts.

### Explicitly Out of Scope
- Porting continuous-daily climate kernel computations (`CLIM03`).
- Porting breakpoint kernel computations and interval-intensity reconciliation
  (`CLIM04`).
- Winter/ET/water-balance/irrigation consumer closure beyond seam-level wiring
  (`CLIM05`/`CLIM06`).
- Single-storm climate/modeling support.

## Deliverables
1. Climate seam ownership + adapter contract artifact:
   - `artifacts/climate-seam-adapter-ownership-contract.md`
2. Climate runtime error taxonomy artifact:
   - `artifacts/climate-runtime-error-taxonomy.md`
3. Climate parser-to-runtime seam integration evidence artifact:
   - `artifacts/climate-parser-runtime-seam-integration-evidence.md`
4. Runtime adapter implementation + tests across in-scope crates.
5. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim02_disposition.md`
6. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/climate-parser-architecture-integration-map.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/openwepp-climate-model-detailed-specification.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/clim01_disposition.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/parser-contract-requirements.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/wepp-forest_260430_baseline/`

## Intended Write Set
- `crates/openwepp-input-contract/**`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `crates/openwepp-kernel-contract/**` (if seam request types require)
- `tests/integration/**`
- `Cargo.toml`
- `Cargo.lock`
- `docs/work-packages/20260522-clim02-climate-parser-to-runtime-seam-adapters-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake and Seam Inventory
- Reconcile CLIM01 seam requirements against current parser + orchestrator
  runtime input surfaces.

### Phase 1 - Adapter and Error Taxonomy Implementation
- Implement hillslope/watershed climate runtime adapters.
- Implement typed `CLIM-RUNTIME-E-*` seam failure taxonomy.
- Enforce climate-version guard semantics (`datver=0.0` override + `datver>=4.0`; pre-4 nonzero rejected).

### Phase 2 - Integration Tests and Closure Evidence
- Add targeted parser-to-runtime seam integration tests for climate surfaces.
- Verify immutable runtime request view consumption and typed failure behavior.

### Phase 3 - Gates and Disposition
- Run required workspace gates and capture evidence.
- Complete dual review + dual verification artifacts and final disposition.

## Exit Criteria
- `HS-CLIM-SEAM-001` and `WS-CLIM-SEAM-001` are implemented and consumed via
  explicit runtime input seams.
- Climate-version guard behavior is implemented with typed failures and tests:
  `datver=0.0` + `datver>=4.0` supported, pre-4 nonzero rejected.
- Duplicate/decreasing breakpoint time guard behavior (`dtime>0`) is
  implemented with typed failures and tests.
- Climate parser-to-runtime integration evidence is published with `Ran`
  command traces.
- No unresolved high-severity seam ownership findings remain.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Dual review and verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: parser/orchestrator seam integration and typed error handling
  changes only.
