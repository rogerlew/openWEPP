# 20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Implement CLIM04 by porting breakpoint (`ibrkpt=1`) legacy climate runtime
behavior into openWEPP runtime kernel surfaces and reconciling ratified
breakpoint policies (`1500` cardinality target and strict `dtime>0` interval
timing guards) with explicit compatibility controls.

## Why This Package Exists
CLIM01 authored the breakpoint behavior contract and ratified the unresolved
policy decisions. CLIM02 established parser-to-runtime seam ownership and
CLIM03 closed the non-breakpoint branch. CLIM04 is the bounded implementation
package that closes breakpoint runtime semantics and policy enforcement.

## Scope
### Included
- Port breakpoint (`ibrkpt=1`) runtime behavior from baseline authority into
  openWEPP runtime surfaces:
  - capture `stmstr` from the first breakpoint time,
  - convert breakpoint times from absolute hour to elapsed storm seconds,
  - build interval intensity/shape arrays from cumulative breakpoint depth,
  - preserve breakpoint-driven storm/event summary semantics consumed by
    downstream runtime modules.
- Implement ratified breakpoint policy closure:
  - parser/runtime capacity target alignment for `1500` breakpoints,
  - strict duplicate/decreasing-time rejection via `dtime>0` for all
    breakpoint intervals (no zero-drain exception).
- Add explicit compatibility controls for any non-default legacy behavior that
  must remain discoverable without weakening strict default execution policy.
- Add fixture-backed integration tests covering breakpoint success/failure
  paths using curated `.cli` fixtures from `/wc1/runs/**/wepp/runs/*.cli`.
- Produce dual review/disposition/verification artifacts.

### Explicitly Out of Scope
- Winter hourly coupling closure and rain/snow partition behavior (`CLIM05`).
- ET/water-balance/irrigation consumer closure (`CLIM06`) beyond directly
  required breakpoint forcing interfaces.
- Single-storm climate/modeling support.

## Deliverables
1. Breakpoint kernel port contract artifact:
   - `artifacts/breakpoint-kernel-port-contract.md`
2. Breakpoint fixture corpus provenance/curation artifact:
   - `artifacts/clim04-cli-fixture-corpus-manifest.md`
3. Breakpoint runtime parity and closure evidence artifact:
   - `artifacts/breakpoint-runtime-parity-evidence.md`
4. Breakpoint policy compatibility-control artifact:
   - `artifacts/breakpoint-policy-compat-controls.md`
5. Runtime implementation + integration tests across in-scope crates.
6. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim04_disposition.md`
7. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/openwepp-climate-model-detailed-specification.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/wepp-forest-climate-model-behavior-map.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/climate-implementation-wp-queue.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim02-climate-parser-to-runtime-seam-adapters-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim03-continuous-daily-climate-runtime-kernel-port-001/`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for`
- `/workdir/wepp-forest_260430_baseline/src/idat.for`
- `/wc1/runs/` (breakpoint fixture corpus source)

## Intended Write Set
- `crates/openwepp-input-contract/src/parsers/climate.rs`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `crates/openwepp-kernel-contract/**` (if runtime forcing types require)
- `tests/integration/**`
- `tests/fixtures/infile/climate/**`
- `Cargo.toml`
- `Cargo.lock`
- `docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake and Breakpoint Fixture Selection
- Reconcile CLIM01 breakpoint behavior contracts with current parser/runtime
  seam behavior.
- Curate representative breakpoint `.cli` fixtures from
  `/wc1/runs/**/wepp/runs/*.cli` and document provenance.

### Phase 1 - Breakpoint Runtime Port
- Implement breakpoint time normalization and interval-intensity projection
  consistent with baseline behavior and openWEPP typed seam rules.
- Implement explicit `1500` capacity alignment and strict `dtime>0` guards for
  all intervals.

### Phase 2 - Integration Tests and Parity Evidence
- Add integration tests covering:
  - supported breakpoint event conversion and forcing projection,
  - strict duplicate/decreasing-time rejection behavior,
  - cardinality-boundary behavior at/over `1500`,
  - compatibility-control behavior and defaults.
- Capture baseline-comparator/parity evidence and confidence notes.

### Phase 3 - Gates and Disposition
- Run required workspace gates and capture evidence.
- Complete dual review + dual verification artifacts and final disposition.

## Exit Criteria
- Breakpoint runtime forcing path is implemented and consumed via typed runtime
  seams.
- Breakpoint `1500` cardinality target is explicitly enforced/aligned across
  parser/runtime surfaces.
- Strict `dtime>0` interval timing guard is enforced for all breakpoint
  intervals (duplicate/decreasing times hard-fail).
- Compatibility controls are explicit, documented, and do not weaken strict
  defaults.
- Curated `/wc1/runs` breakpoint fixture provenance is documented and
  reproducible.
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
