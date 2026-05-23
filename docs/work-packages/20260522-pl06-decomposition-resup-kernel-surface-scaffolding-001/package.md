# 20260522-pl06-decomposition-resup-kernel-surface-scaffolding-001

## Status
- state: hold
- date: 2026-05-22
- timezone: UTC

## Objective
Add typed interfaces and scheduler integration for decomposition + residue
partition transitions preserving baseline phase order.

## Why This Package Exists
PL01 follow-on sequencing identifies PL06 as the decomposition/residue
counterpart to PL05 growth scaffolding. PL06 closes the kernel-boundary path
for decomposition and resup transitions after PL02/PL03/PL04 boundary and seam
closure.

## Scope
### Included
- Define typed decomposition/resup kernel interfaces for scheduler-facing PL
  state transitions.
- Add placeholder scheduler phase surfaces for decomposition and residue
  partition transitions while preserving baseline phase ordering constraints.
- Wire orchestrator-to-kernel boundary integration for decomposition/resup
  surfaces with typed error propagation only.
- Add integration coverage for decomposition/resup phase scaffolding,
  transition ordering guards, and boundary shape checks.
- Publish PL06 implementation evidence and disposition artifacts.

### Explicitly Out of Scope
- Full process-level decomposition kinetics and calibration tuning beyond
  scaffolding.
- Growth-kernel interface/scaffold work already scoped to PL05.
- Comparator confidence-tier campaign execution (`PL08`).

## Deliverables
1. Decomposition/resup kernel interface contract notes:
   - `artifacts/pl06-decomposition-resup-kernel-surface-contract.md`
2. Scheduler phase scaffolding notes:
   - `artifacts/pl06-residue-partition-scheduler-phase-scaffold.md`
3. Decomposition/resup transition surface map:
   - `artifacts/pl06-decomposition-resup-transition-state-map.md`
4. PL06 decomposition boundary test evidence:
   - `artifacts/pl06-decomposition-surface-test-evidence.md`
5. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl06_disposition.md`
6. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-boundary-contract.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl02-follow-on-implementation-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/artifacts/pl03-runtime-surface-projection-map.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/artifacts/pl03-scheduler-ordering-compliance.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl04-pl-symbol-alias-completion-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl04-pl-symbol-alias-completion-001/artifacts/pl04-canonical-symbol-alias-table.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl05-growth-kernel-surface-scaffolding-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl05-growth-kernel-surface-scaffolding-001/artifacts/pl05-growth-scheduler-phase-scaffold.md`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/`

## Intended Write Set
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-hillslope-orchestrator/**`
- `tests/integration/**`
- `docs/work-packages/20260522-pl06-decomposition-resup-kernel-surface-scaffolding-001/**`

## Phase Plan
### Phase 0 - Intake
- Confirm PL02 boundary requirements and PL03/PL04 closure assumptions for
  decomposition/resup scheduling.
- Reconcile scheduler ordering assumptions with PL05 scaffold outputs.

### Phase 1 - Interface Scaffolding
- Add typed decomposition/resup boundary interfaces and placeholder phase
  structures.

### Phase 2 - Scheduler Integration
- Integrate placeholder decomposition/resup phase sequencing into
  scheduler-facing orchestration paths with typed failures only.

### Phase 3 - Verification
- Add and run targeted tests for boundary shape, ordering guards, and typed
  reject paths.

### Phase 4 - Disposition
- Run required gates and finalize review/verification/disposition artifacts.

## Exit Criteria
- Decomposition/resup kernel boundary interfaces exist and are typed.
- Placeholder decomposition/residue partition phase sequencing is explicit and
  preserves baseline ordering constraints.
- Integration/test evidence demonstrates boundary behavior and ordering guards.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: typed interface/scheduler scaffolding and tests only.
