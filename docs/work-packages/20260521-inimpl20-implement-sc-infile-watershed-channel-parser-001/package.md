# 20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Implement Wave 3 parser surface 'SC-INFILE-WATERSHED-CHANNEL-001' for 'infile-watershed-channel (watershed channel (.chn))' in a dedicated
worktree-owned worker stream.

## Why This Package Exists
Wave 3 requires parallel, isolated implementation streams so watershed-core
surface work can proceed with explicit ownership and deterministic integration.

## Scope
### Included
- Implement strict/compat parser behavior for 'infile-watershed-channel (watershed channel (.chn))' per 'SC-INFILE-WATERSHED-CHANNEL-001'.
- Implement guard/invariant-linked validation errors for required fields.
- Add surface-specific parser tests and fixtures.
- Emit worker handoff artifacts for integration package 'INIMPL22'.

### Explicitly Out of Scope
- Other Wave 3 parser surfaces.
- Wave 4 watershed sidecar parser surfaces.
- Final integration/cherry-pick onto mainline.

## Worktree Execution Model
- Assigned worktree path: '/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel'
- Assigned branch name: 'inimpl20/watershed-channel-parser'
- Ownership rule: worker must stay within assigned write-set and must not
  refactor outside the package boundary without explicit coordination update.

## Deliverables
1. Parser implementation for 'infile-watershed-channel (watershed channel (.chn))' under assigned module ownership.
2. Surface test suite and fixtures for strict/compat mode behavior.
3. Worker handoff notes:
   - artifacts/worker-handoff.md
4. Change manifest:
   - artifacts/owned-file-manifest.md
5. Closeout disposition:
   - artifacts/inimpl20_disposition.md
6. Review and verification artifacts:
   - artifacts/review_agent_a.md
   - artifacts/review_agent_b.md
   - artifacts/verification_agent_a.md
   - artifacts/verification_agent_b.md

## Dependencies
- /home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md
- /home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md
- /home/workdir/openWEPP/docs/planning/wave3-parser-worktree-execution-plan.md
- /home/workdir/openWEPP/docs/work-packages/20260521-inimpl18-wave3-worktree-orchestration-001/
- /home/workdir/openWEPP/docs/work-packages/20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/

## Intended Write Set
- crates/openwepp-input-contract/src/parsers/watershed_channel.rs
- crates/openwepp-input-contract/src/parsers/mod.rs
- tests/integration/infile_watershed_channel_parser_contract.rs
- surface-local test fixtures under tests/fixtures/infile/watershed_channel/**
- package artifacts under this work-package directory

## Phase Plan
### Phase 0 - Contract-to-Code Mapping
- Map contract sections to parser data model and typed errors.

### Phase 1 - Parser Implementation
- Implement parser read/parse/validate logic for 'infile-watershed-channel (watershed channel (.chn))'.
- Ensure invariant guard behavior is explicit and typed.

### Phase 2 - Surface Tests
- Add strict/compat fixture tests.
- Add negative tests for malformed/missing/open-failure cases.

### Phase 3 - Worker Closeout
- Produce handoff notes and owned-file manifest.
- Run review/disposition/verification gates.

## Exit Criteria
- Contract-mandated parser behaviors for 'infile-watershed-channel (watershed channel (.chn))' are implemented.
- Surface tests pass and cover strict/compat branches.
- Owned-file manifest is complete and integration-ready.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: parser implementation and tests only; no network/service exposure.
