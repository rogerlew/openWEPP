# 20260522-clim14-runtime-breakpoint-cardinality-policy-closure-001

## Status
- state: queued
- date: 2026-05-22
- timezone: UTC

## Objective
Align runtime seam breakpoint cardinality enforcement with the `1500` policy
target and explicitly codify parser-override compatibility behavior.

## Why This Package Exists
CLIM04 accepted-in-part review finding `CLIM04-RVW-005` identified
parser/runtime policy alignment ambiguity when parser cardinality override is
enabled. This package closes runtime-side policy semantics.

## Scope
### Included
- Add explicit runtime seam cardinality policy behavior aligned with ratified
  target.
- Define compatibility behavior for parser override modes.
- Add tests for strict-policy and override-policy execution branches.

### Explicitly Out of Scope
- Shared extraction work in CLIM12.
- Typed surface migration in CLIM13 unless required for policy enforcement.

## Deliverables
1. Cardinality policy contract:
   - `artifacts/runtime-breakpoint-cardinality-policy.md`
2. Policy evidence artifact:
   - `artifacts/cardinality-policy-parity-evidence.md`
3. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim14_disposition.md`
4. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim12-shared-climate-runtime-adapter-extraction-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/artifacts/breakpoint-policy-compat-controls.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`

## Intended Write Set
- `crates/openwepp-input-contract/**`
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `tests/integration/**`
- `docs/work-packages/20260522-clim14-runtime-breakpoint-cardinality-policy-closure-001/**`

## Phase Plan
### Phase 0 - Intake
- Confirm cardinality behavior from parser and runtime seams.

### Phase 1 - Policy Closure
- Implement explicit runtime cardinality behavior for strict/override modes.

### Phase 2 - Verification
- Validate boundary and override behavior with targeted tests.

### Phase 3 - Disposition
- Run required gates and finalize disposition artifacts.

## Exit Criteria
- Runtime cardinality semantics are explicit and policy-consistent.
- Strict and override modes are documented and test-covered.
- Artifacts include clear evidence-mode labeling (`Static:` vs `Ran:`).
