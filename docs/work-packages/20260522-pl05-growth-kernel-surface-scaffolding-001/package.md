# 20260522-pl05-growth-kernel-surface-scaffolding-001

## Status
- state: hold
- date: 2026-05-22
- timezone: UTC

## Objective
Add kernel-facing typed interfaces and placeholder scheduler phases for
annual/perennial growth state transitions.

## Why This Package Exists
PL01 follow-on sequencing identifies PL05 as the next dependency after PL02,
PL03, and PL04 to open a controlled path from management-derived runtime
surfaces into growth-kernel execution boundaries.

## Scope
### Included
- Define typed growth kernel interfaces for scheduler-facing state transitions.
- Add placeholder scheduler phase surfaces for annual/perennial growth
  transitions, preserving deterministic ordering.
- Wire orchestrator-to-kernel boundary integration for PL growth surfaces with
  typed error propagation only.
- Add integration coverage for scheduler phase scaffolding and boundary
  interface shape/guard behavior.
- Publish PL05 implementation evidence and disposition artifacts.

### Explicitly Out of Scope
- Full process-level growth kernel behavior implementation beyond placeholder
  scaffolding.
- Decomposition/residue kernel surface scaffolding (`PL06`).
- Comparator confidence-tier review execution (`PL08`).

## Deliverables
1. Growth kernel interface contract notes:
   - `artifacts/pl05-growth-kernel-surface-contract.md`
2. Scheduler phase scaffolding notes:
   - `artifacts/pl05-growth-scheduler-phase-scaffold.md`
3. Annual/perennial transition surface map:
   - `artifacts/pl05-annual-perennial-transition-state-map.md`
4. PL05 growth boundary test evidence:
   - `artifacts/pl05-growth-surface-test-evidence.md`
5. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl05_disposition.md`
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
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/`

## Intended Write Set
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-hillslope-orchestrator/**`
- `tests/integration/**`
- `docs/work-packages/20260522-pl05-growth-kernel-surface-scaffolding-001/**`

## Phase Plan
### Phase 0 - Intake
- Confirm PL02 boundary requirements and PL03/PL04 closure assumptions for
  growth-surface scheduling.

### Phase 1 - Interface Scaffolding
- Add typed growth-kernel boundary interfaces and placeholder phase structures.

### Phase 2 - Scheduler Integration
- Integrate placeholder phase sequencing into scheduler-facing orchestration
  paths with typed failures only.

### Phase 3 - Verification
- Add and run targeted tests for interface shape, phase ordering, and guard
  behavior.

### Phase 4 - Disposition
- Run required gates and finalize review/verification/disposition artifacts.

## Exit Criteria
- Growth-kernel boundary interfaces exist and are typed.
- Placeholder annual/perennial growth phase sequencing is explicit and
  deterministic.
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
