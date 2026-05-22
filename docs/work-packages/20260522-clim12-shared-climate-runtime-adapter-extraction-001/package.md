# 20260522-clim12-shared-climate-runtime-adapter-extraction-001

## Status
- state: queued
- date: 2026-05-22
- timezone: UTC

## Objective
Eliminate duplicated climate runtime seam logic by extracting shared
adapters/types/tests into a single-owner module or crate consumed by both
orchestrators.

## Why This Package Exists
CLIM04 accepted review finding `CLIM04-RVW-002` (high-severity duplication)
across hillslope and watershed climate runtime adapters. This package
centralizes climate adaptation logic to prevent silent divergence.

## Scope
### Included
- Inventory duplicated constants, guard logic, and conversion paths.
- Extract shared climate runtime seam implementation and typed interfaces.
- Rewire hillslope/watershed orchestrators to consume shared implementation.

### Explicitly Out of Scope
- Typed forcing model redesign (`CLIM13`) beyond extraction boundary.
- Governance normalization (`CLIM16`).

## Deliverables
1. Shared extraction contract:
   - `artifacts/shared-climate-runtime-adapter-contract.md`
2. Parity evidence artifact:
   - `artifacts/shared-adapter-parity-evidence.md`
3. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim12_disposition.md`
4. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim11-climate-ownership-boundary-reconciliation-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/artifacts/clim04_disposition.md`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`

## Intended Write Set
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `crates/openwepp-kernel-contract/**` (if shared contract types move)
- `tests/integration/**`
- `docs/work-packages/20260522-clim12-shared-climate-runtime-adapter-extraction-001/**`

## Phase Plan
### Phase 0 - Intake
- Confirm ownership contract from CLIM11 and lock shared extraction boundary.

### Phase 1 - Extraction
- Move duplicated logic to a single shared owner with equivalent behavior.

### Phase 2 - Verification
- Add parity tests demonstrating equivalent outputs from both orchestrators.

### Phase 3 - Disposition
- Run required gates and finalize review/verification/disposition.

## Exit Criteria
- Duplicated climate runtime logic is removed from parallel orchestrator
  implementations and replaced by one shared owner.
- Behavior parity is demonstrated with tests/evidence.
- Artifacts include clear evidence-mode labeling (`Static:` vs `Ran:`).
