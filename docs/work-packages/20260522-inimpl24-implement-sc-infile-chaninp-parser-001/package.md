# 20260522-inimpl24-implement-sc-infile-chaninp-parser-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Implement Wave 4 parser surface 'SC-INFILE-CHANINP-001' for 'infile-chaninp (chan.inp)' in a
dedicated worktree-owned worker stream.

## Why This Package Exists
Wave 4 requires parallel, isolated implementation streams so
watershed-sidecar surfaces can proceed with explicit ownership and
deterministic integration.

## Scope
### Included
- Implement strict/compat parser behavior for 'infile-chaninp (chan.inp)' per 'SC-INFILE-CHANINP-001'.
- Implement guard/invariant-linked validation errors for required fields.
- Add surface-specific parser tests and fixtures.
- Emit worker handoff artifacts for integration package 'INIMPL30'.
- Capture required Wave 4 decision evidence for: W4DR-003, W4DR-004, W4DR-005, W4DR-006, W4DR-008.

### Explicitly Out of Scope
- Other Wave 4 parser surfaces.
- Non-parser kernel/orchestrator implementation.
- Final integration/cherry-pick onto mainline.

## Worktree Execution Model
- Assigned worktree path: '/home/workdir/openWEPP/.worktrees/inimpl24-chaninp'
- Assigned branch name: 'inimpl24/chaninp-parser'
- Ownership rule: worker must stay within assigned write-set and must not
  refactor outside the package boundary without explicit coordination update.

## Deliverables
1. Parser implementation for 'infile-chaninp (chan.inp)' under assigned module ownership.
2. Surface test suite and fixtures for strict/compat mode behavior.
3. Worker handoff notes:
   - artifacts/worker-handoff.md
4. Change manifest:
   - artifacts/owned-file-manifest.md
5. Closeout disposition:
   - artifacts/inimpl24_disposition.md
6. Review and verification artifacts:
   - artifacts/review_agent_a.md
   - artifacts/review_agent_b.md
   - artifacts/verification_agent_a.md
   - artifacts/verification_agent_b.md

## Dependencies
- /home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md
- /home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md
- /home/workdir/openWEPP/docs/planning/wave4-parser-worktree-execution-plan.md
- /home/workdir/openWEPP/docs/work-packages/20260522-inimpl23-wave4-worktree-orchestration-001/
- /home/workdir/openWEPP/docs/work-packages/20260522-arch13-wave4-hold-ratification-checklist-001/artifacts/wave4-hold-ratification-checklist.md

## Intended Write Set
- crates/openwepp-input-contract/src/parsers/chaninp.rs
- tests/integration/infile_chaninp_parser_contract.rs
- surface-local test fixtures under tests/fixtures/infile/chaninp/**
- package artifacts under this work-package directory

## Phase Plan
### Phase 0 - Contract-to-Code Mapping
- Map contract sections to parser data model and typed errors.
- Map applicable ratified Wave 4 decisions: W4DR-003, W4DR-004, W4DR-005, W4DR-006, W4DR-008.

### Phase 1 - Parser Implementation
- Implement parser read/parse/validate logic for 'infile-chaninp (chan.inp)'.
- Ensure invariant guard behavior is explicit and typed.

### Phase 2 - Surface Tests
- Add strict/compat fixture tests.
- Add negative tests for malformed/missing/open-failure cases.
- Add tests for applicable `W4DR` decision branches.

### Phase 3 - Worker Closeout
- Produce handoff notes and owned-file manifest.
- Run review/disposition/verification gates.

## Exit Criteria
- Contract-mandated parser behaviors for 'infile-chaninp (chan.inp)' are implemented.
- Surface tests pass and cover strict/compat branches.
- Applicable `W4DR` decision checks are evidenced.
- Owned-file manifest is complete and integration-ready.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: parser implementation and tests only; no network/service exposure.
