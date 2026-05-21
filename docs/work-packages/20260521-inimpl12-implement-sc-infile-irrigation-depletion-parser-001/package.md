# 20260521-inimpl12-implement-sc-infile-irrigation-depletion-parser-001

## Status
- `state`: active
- `date`: 2026-05-21
- `timezone`: UTC

## Objective
Implement Wave 2 parser surface `SC-INFILE-IRRIGATION-DEPLETION-001` for `infile-irrigation-depletion (irrigation-depletion sidecar)` in a dedicated
worktree-owned worker stream.

## Why This Package Exists
Wave 2 requires parallel, isolated implementation streams so sidecar-surface
work can proceed concurrently without hidden path overlap.

## Scope
### Included
- Implement strict/compat parser behavior for `infile-irrigation-depletion (irrigation-depletion sidecar)` per `SC-INFILE-IRRIGATION-DEPLETION-001`.
- Implement guard/invariant-linked validation errors for required fields.
- Add surface-specific parser tests and fixtures.
- Emit worker handoff artifacts for integration package `INIMPL17`.

### Explicitly Out of Scope
- Other Wave 2 parser surfaces.
- Watershed parser surfaces.
- Final integration/cherry-pick onto mainline.

## Worktree Execution Model
- Assigned worktree path: `/home/workdir/openWEPP/.worktrees/inimpl12-irrigation-depletion`
- Assigned branch name: `inimpl12/irrigation-depletion-parser`
- Ownership rule: worker must stay within assigned write-set and must not
  refactor outside the package boundary without explicit coordination update.

## Deliverables
1. Parser implementation for `infile-irrigation-depletion (irrigation-depletion sidecar)` under assigned module ownership.
2. Surface test suite and fixtures for strict/compat mode behavior.
3. Worker handoff notes:
   - `artifacts/worker-handoff.md`
4. Change manifest:
   - `artifacts/owned-file-manifest.md`
5. Closeout disposition:
   - `artifacts/inimpl12_disposition.md`
6. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md`
- `/home/workdir/openWEPP/docs/planning/wave2-parser-worktree-execution-plan.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl10-wave2-worktree-orchestration-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl09-management-full-typed-datamodel-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/`

## Intended Write Set
- `crates/openwepp-input-contract/src/parsers/irrigation_depletion.rs`
- `crates/openwepp-input-contract/src/parsers/mod.rs`
- `tests/integration/infile_irrigation_depletion_parser_contract.rs`
- surface-local test fixtures under `tests/fixtures/infile/irrigation_depletion/**`
- package artifacts under this work-package directory

## Phase Plan
### Phase 0 - Contract-to-Code Mapping
- Map contract sections to parser data model and typed errors.

### Phase 1 - Parser Implementation
- Implement parser read/parse/validate logic for `infile-irrigation-depletion (irrigation-depletion sidecar)`.
- Ensure invariant guard behavior is explicit and typed.

### Phase 2 - Surface Tests
- Add strict/compat fixture tests.
- Add negative tests for malformed/missing/open-failure cases.

### Phase 3 - Worker Closeout
- Produce handoff notes and owned-file manifest.
- Run review/disposition/verification gates.

## Exit Criteria
- Contract-mandated parser behaviors for `infile-irrigation-depletion (irrigation-depletion sidecar)` are implemented.
- Surface tests pass and cover strict/compat branches.
- Owned-file manifest is complete and integration-ready.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: parser implementation and tests only; no network/service exposure.
