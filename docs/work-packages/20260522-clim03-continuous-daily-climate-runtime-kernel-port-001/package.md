# 20260522-clim03-continuous-daily-climate-runtime-kernel-port-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Implement CLIM03 by porting continuous-daily (non-breakpoint) legacy climate
runtime behavior into openWEPP runtime kernel surfaces, including `iclig`
branch handling, unit conversion, event-shape/disaggregation behavior, and
fixture-backed parity evidence.

## Why This Package Exists
CLIM01 authored the authoritative continuous-daily behavior specification and
CLIM02 established parser-to-runtime seam ownership. CLIM03 is the next
implementation package to port baseline non-breakpoint runtime behavior before
breakpoint-specific CLIM04 work.

## Scope
### Included
- Port non-breakpoint (`ibrkpt=0`) climate runtime behavior from baseline
  authority into openWEPP runtime surfaces.
- Implement ratified version policy in runtime climate processing:
  - support explicit `datver=0.0` override branch (`iclig=0`)
  - support `datver>=4.0` branch (`iclig=1`, `ip *= 0.70`)
  - reject pre-4 nonzero branch (`0.0<datver<4.0`, legacy `iclig=2`).
- Implement continuous-daily normalization and guards:
  - `stmdur` cap at `23.999 h` pre-conversion,
  - unit conversion (`mm->m`, `hr->s`),
  - `prcp>0` with non-positive duration handling,
  - deterministic typed failure/warn policy with no silent fallback.
- Port event-shape/disaggregation behavior required by continuous-daily path
  (`disag` / `const` / `dblex` lineage) into openWEPP runtime consumable
  forcing surfaces.
- Add fixture-backed integration tests for continuous-daily runtime closure
  using curated `.cli` fixtures sourced from `/wc1/runs/**/wepp/runs/*.cli`.
- Produce dual review/disposition/verification artifacts.

### Explicitly Out of Scope
- Breakpoint runtime port/policy reconciliation (`CLIM04`).
- Winter coupling closure (`CLIM05`) and ET/water-balance/irrigation consumer
  closure (`CLIM06`) beyond interfaces directly needed for continuous-daily
  runtime forcing.
- Single-storm climate/modeling support.

## Deliverables
1. Continuous-daily kernel port contract artifact:
   - `artifacts/continuous-daily-kernel-port-contract.md`
2. Fixture corpus provenance/curation artifact:
   - `artifacts/clim03-cli-fixture-corpus-manifest.md`
3. Continuous-daily parity and closure evidence artifact:
   - `artifacts/continuous-daily-runtime-parity-evidence.md`
4. Runtime implementation + integration tests across in-scope crates.
5. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim03_disposition.md`
6. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/openwepp-climate-model-detailed-specification.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/wepp-forest-climate-model-behavior-map.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim02-climate-parser-to-runtime-seam-adapters-001/`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for`
- `/workdir/wepp-forest_260430_baseline/src/idat.for`
- `/workdir/wepp-forest_260430_baseline/src/disag.for`
- `/workdir/wepp-forest_260430_baseline/src/dblex.for`
- `/workdir/wepp-forest_260430_baseline/src/const.for`
- `/wc1/runs/` (continuous-daily fixture corpus source)

## Intended Write Set
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `crates/openwepp-kernel-contract/**` (if runtime forcing types require)
- `tests/integration/**`
- `tests/fixtures/infile/climate/**`
- `Cargo.toml`
- `Cargo.lock`
- `docs/work-packages/20260522-clim03-continuous-daily-climate-runtime-kernel-port-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake and Fixture Selection
- Reconcile CLIM01 continuous-daily behavior contract with current runtime
  surfaces.
- Curate representative `.cli` fixtures from `/wc1/runs/**/wepp/runs/*.cli`
  into `tests/fixtures/infile/climate/**` with provenance notes.

### Phase 1 - Continuous-Daily Runtime Port
- Implement non-breakpoint normalization, policy guards, and unit conversion.
- Implement runtime event-shape/disaggregation projection consistent with
  baseline behavior and openWEPP typed seam rules.

### Phase 2 - Integration Tests and Parity Evidence
- Add integration tests covering:
  - supported `datver=0.0` and `datver>=4.0` branches,
  - rejected pre-4 nonzero branch,
  - invalid-duration handling,
  - event-shape closure invariants.
- Capture baseline-comparator/parity evidence and confidence notes.

### Phase 3 - Gates and Disposition
- Run required workspace gates and capture evidence.
- Complete dual review + dual verification artifacts and final disposition.

## Exit Criteria
- Continuous-daily runtime forcing path is implemented and consumed via typed
  runtime seams.
- `datver=0.0` override and `datver>=4.0` branches are supported with typed
  runtime behavior; pre-4 nonzero branch is rejected.
- Continuous-daily disaggregation/event-shape behavior is implemented with
  deterministic test coverage.
- Curated `/wc1/runs` fixture provenance is documented and reproducible.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Dual review and verification artifacts are complete.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: runtime forcing implementation and tests only; no new service
  exposure.
