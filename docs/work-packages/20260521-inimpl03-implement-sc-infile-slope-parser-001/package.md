# 20260521-inimpl03-implement-sc-infile-slope-parser-001

## Status
- `state`: active
- `date`: 2026-05-21
- `timezone`: UTC

## Objective
Implement Wave 1 parser surface `SC-INFILE-SLOPE-001` for `infile-slope-slp (.slp)` in a dedicated
worktree-owned worker stream.

## Why This Package Exists
Wave 1 requires parallel, isolated implementation streams to accelerate delivery
without path-overlap merge conflicts.

## Scope
### Included
- Implement strict/compat parser behavior for `infile-slope-slp (.slp)` per `SC-INFILE-SLOPE-001`.
- Implement guard/invariant-linked validation errors for required fields.
- Add surface-specific parser tests and fixtures.
- Emit worker handoff artifacts for integration package `INIMPL07`.

### Explicitly Out of Scope
- Other Wave 1 parser surfaces.
- Watershed parser surfaces.
- Final integration/cherry-pick onto mainline.

## Worktree Execution Model
- Assigned worktree path: `/home/workdir/openWEPP/.worktrees/inimpl03-slope`
- Assigned branch name: `inimpl03/slope-parser`
- Ownership rule: worker must stay within assigned write-set and must not
  refactor outside the package boundary without explicit coordination update.

## Deliverables
1. Parser implementation for `infile-slope-slp (.slp)` under assigned module ownership.
2. Surface test suite and fixtures for strict/compat mode behavior.
3. Worker handoff notes:
   - `artifacts/worker-handoff.md`
4. Change manifest:
   - `artifacts/owned-file-manifest.md`
5. Closeout disposition:
   - `artifacts/inimpl03_disposition.md`
6. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001`
- `/home/workdir/openWEPP/docs/planning/wave1-parser-worktree-execution-plan.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl02-wave1-worktree-orchestration-001/`

## Intended Write Set
- `crates/openwepp-input-contract/src/parsers/slope.rs`
- `tests/integration/infile_slope_parser_contract.rs`
- surface-local test fixtures under `tests/fixtures/infile/`
- package artifacts under this work-package directory

## Phase Plan
### Phase 0 - Contract-to-Code Mapping
- Map contract sections to parser data model and typed errors.

### Phase 1 - Parser Implementation
- Implement parser read/parse/validate logic for `infile-slope-slp (.slp)`.
- Ensure invariant guard behavior is explicit and typed.

### Phase 2 - Surface Tests
- Add strict/compat fixture tests.
- Add negative tests for malformed/missing/open-failure cases.

### Phase 3 - Worker Closeout
- Produce handoff notes and owned-file manifest.
- Run review/disposition/verification gates.

## Exit Criteria
- Contract-mandated parser behaviors for `infile-slope-slp (.slp)` are implemented.
- Surface tests pass and cover strict/compat branches.
- Owned-file manifest is complete and integration-ready.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: parser implementation and tests only; no network/service exposure.
