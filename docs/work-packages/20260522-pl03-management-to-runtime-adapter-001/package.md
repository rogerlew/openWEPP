# 20260522-pl03-management-to-runtime-adapter-001

## Status
- state: complete
- date: 2026-05-22
- timezone: America/Los_Angeles

## Objective
Implement the strict typed management-to-runtime adapter for PL surfaces
(`PL-MAN-SEAM-001`) so parsed `.man` outputs are projected into scheduler-facing
plant/growth/decomposition runtime surfaces with typed errors and no silent
defaults.

## Why This Package Exists
PL02 closed runtime boundary contract authoring and identified PL03 as the
next required implementation dependency. Without PL03, PL runtime surfaces
remain unspecified in orchestrator execution code and PL05/PL06 kernel packages
cannot wire state safely.

## Scope
### Included
- Implement typed parser-to-runtime projection from `ManagementParseOutput`.
- Project required PL runtime surface families (`pl_schedule`, `pl_growth`,
  `pl_decomp`) in orchestrator-owned surfaces.
- Implement strict typed error taxonomy for missing/invalid/non-finite required
  PL projection inputs.
- Add integration tests for positive projection and typed negative paths.
- Produce implementation evidence and disposition artifacts.

### Explicitly Out of Scope
- Canonical alias registry expansion (PL04).
- Growth/decomposition kernel behavior implementation (PL05/PL06).
- Comparator campaign execution (PL08).

## Deliverables
1. Runtime adapter contract notes:
   - `artifacts/pl03-runtime-adapter-contract.md`
2. Runtime projection map:
   - `artifacts/pl03-runtime-surface-projection-map.md`
3. Typed error taxonomy:
   - `artifacts/pl03-typed-error-taxonomy.md`
4. Parser-to-runtime integration evidence:
   - `artifacts/pl03-parser-to-runtime-integration-evidence.md`
5. Scheduler ordering compliance notes:
   - `artifacts/pl03-scheduler-ordering-compliance.md`
6. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl03_disposition.md`
7. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-boundary-contract.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-seam-requirements.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl02-follow-on-implementation-handoff.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`

## Intended Write Set
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-kernel-contract/**` (if seam types move here)
- `tests/integration/**`
- `docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/**`

## Phase Plan
### Phase 0 - Intake
- Confirm PL02 seam requirements and boundary contract constraints.

### Phase 1 - Adapter Implementation
- Implement strict typed PL projection and typed error boundaries.

### Phase 2 - Verification
- Add parser-to-runtime integration tests and negative path error checks.

### Phase 3 - Disposition
- Run required gates and complete review/verification/disposition artifacts.

## Exit Criteria
- PL management parser output projects into required runtime surfaces without
  silent defaults.
- Typed error taxonomy is explicit and tested.
- Integration evidence demonstrates positive and negative seam behavior.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: no
- Rationale: runtime seam code changes with typed-error enforcement only.
