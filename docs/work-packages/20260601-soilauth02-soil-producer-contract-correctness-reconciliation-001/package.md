# 20260601-soilauth02-soil-producer-contract-correctness-reconciliation-001

## Status
- state: queued
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute SOILAUTH02 to reconcile producer-contract mismatches identified by
SOILAUTH01 by applying correctness fixes to:
1. openWEPP producer/parser contract artifacts, and/or
2. canonical `wepppy` producer behavior,
with explicit provenance and regenerated fixture-hash evidence.

## Why This Package Exists
SOILAUTH01 provides the mismatch inventory but does not remediate defects.
SOILAUTH02 is the implementation package that closes P0/P1 mismatches and
aligns contract text, parser acceptance, and producer outputs.

## Scope
### Included
- Implement approved corrections for `.sol` datver `7778/9002/9003/9005`
  mismatches from SOILAUTH01.
- Update canonical contract/spec text and parser conformance tests.
- Apply producer-side corrections in `wepppy` when defects are producer-owned.
- Regenerate canonical fixtures and provenance hashes used by authority suites.

### Explicitly Out of Scope
- New anti-drift automation/obligation guards (SOILAUTH03 scope).
- Non-soil input-file families.
- Process-physics remediation outside producer/parser contract scope.

## Closure Measures (Required)
1. `MEASURE-SA02-001`: all SOILAUTH01 P0/P1 mismatches are resolved or carry
   explicit `HOLD` disposition with owner, rationale, and unblock condition.
2. `MEASURE-SA02-002`: openWEPP parser tests pass on canonical producer fixtures
   for datver `7778/9002/9003/9005`.
3. `MEASURE-SA02-003`: fixture provenance hashes and lock artifacts are updated
   for modified authoritative fixtures.

## Deliverables
1. `artifacts/soilauth02-reconciliation-gap-ledger.md`
2. `artifacts/soilauth02-contract-implementation-evidence.md`
3. `artifacts/soilauth02-contract-test-implementation-evidence.md`
4. `artifacts/soilauth02-preimplementation-contract-gate.md`
5. `artifacts/soilauth02-implementation-and-test-evidence.md`
6. `artifacts/soilauth02-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/soilauth02_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/spec authority surfaces for resolved mismatch policy.
2. Implement contract-derived tests and fixture obligations.
3. Record pre-implementation contract gate before remediation edits.
4. Apply production parser/producer remediation and rerun validation.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Producer-contract authority for `.sol` remains:
  `docs/specifications/wepp-input-files/specs/soil-file.spec.md`.
- Parser/runtime acceptance authority remains:
  `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`.
- `wepppy` is canonical producer implementation reference and must be corrected
  when mismatch ownership is producer-side.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-soilauth01-soil-producer-contract-conformance-audit-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-soilauth01-soil-producer-contract-conformance-audit-001/artifacts/soilauth01-producer-conformance-matrix.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/soil.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `/workdir/openWEPP/tests/fixtures/infile/soil/`
- `/workdir/wepppy/wepppy/weppcloud/routes/usersum/input-file-specifications/soil-file.spec.md`
- `/workdir/wepppy/wepppy/wepp/soils/utils/wepp_soil_util.py`
- `/workdir/wepppy/wepppy/soils/ssurgo/ssurgo.py`
- `/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-soilauth02-soil-producer-contract-correctness-reconciliation-001/**`
- `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-input-contract/src/parsers/soil.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `tests/fixtures/infile/soil/**`
- `tests/integration/soilauth02_soil_producer_reconciliation_contract.rs`
- `/workdir/wepppy/wepppy/wepp/soils/utils/wepp_soil_util.py` (if producer-owned mismatch closure is required)
- `/workdir/wepppy/wepppy/soils/ssurgo/ssurgo.py` (if producer-owned mismatch closure is required)
- `/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py` (if producer-owned mismatch closure is required)

## Phase Plan
### Phase A - Intake and scope freeze
- Import and freeze SOILAUTH01 mismatch ledger and ownership classification.

### Phase B - Contract/spec authority updates
- Amend producer/parser contract surfaces for approved closure policy.

### Phase C - Contract-derived tests
- Implement contract-derived tests that fail on unresolved P0/P1 mismatch
  patterns.

### Phase D - Pre-implementation contract gate
- Record pre-implementation red-state evidence before remediation edits.

### Phase E - Production implementation
- Apply parser/producer fixes and regenerate canonical fixtures/hashes.

### Phase F - Validation and parity rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Run targeted reconciliation tests and fixture-integrity checks.

### Phase G - Dual review, dual verification, disposition
- Complete dual review/verification and publish residual mismatch disposition.

## Exit Criteria
- Closure measures `MEASURE-SA02-001..003` are satisfied and evidenced.
- SOILAUTH03 receives a concrete anti-drift guard requirements handoff.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: parser/producer contract and fixture closure only; no credential
  surface changes.
