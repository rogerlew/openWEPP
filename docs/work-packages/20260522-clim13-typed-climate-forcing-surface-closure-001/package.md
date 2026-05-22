# 20260522-clim13-typed-climate-forcing-surface-closure-001

## Status
- state: queued
- date: 2026-05-22
- timezone: UTC

## Objective
Replace string-key synthesized breakpoint forcing writeback with a typed climate
forcing surface while preserving canonical symbol alias mapping at boundaries.

## Why This Package Exists
CLIM04 accepted-in-part review finding `CLIM04-RVW-004` identified that
`BoundarySymbol` newtype usage does not close typed-surface intent in runtime
hot paths. This package closes typed forcing representation drift.

## Scope
### Included
- Define typed breakpoint forcing payload structures and conversion boundaries.
- Remove hot-path runtime string synthesis for per-breakpoint forcing keys.
- Preserve canonical WEPP symbol continuity via explicit alias mappings.

### Explicitly Out of Scope
- Shared extraction mechanics handled in CLIM12.
- Governance normalization handled in CLIM16.

## Deliverables
1. Typed forcing contract:
   - `artifacts/typed-climate-forcing-surface-contract.md`
2. Migration evidence artifact:
   - `artifacts/typed-forcing-migration-evidence.md`
3. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim13_disposition.md`
4. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim11-climate-ownership-boundary-reconciliation-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim12-shared-climate-runtime-adapter-extraction-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/`

## Intended Write Set
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `tests/integration/**`
- `docs/work-packages/20260522-clim13-typed-climate-forcing-surface-closure-001/**`

## Phase Plan
### Phase 0 - Intake
- Confirm extraction baseline from CLIM12 and typed-surface requirements.

### Phase 1 - Typed Surface Implementation
- Implement typed climate forcing structures and adapter boundary mappings.

### Phase 2 - Verification
- Add tests proving typed representation and boundary alias continuity.

### Phase 3 - Disposition
- Run required gates and finalize disposition artifacts.

## Exit Criteria
- Runtime climate forcing uses typed structures instead of dynamic
  per-breakpoint string key synthesis in hot paths.
- Boundary mappings preserve canonical symbol continuity where needed.
- Artifacts include clear evidence-mode labeling (`Static:` vs `Ran:`).
