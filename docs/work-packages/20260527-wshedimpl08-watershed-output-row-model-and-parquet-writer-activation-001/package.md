# 20260527-wshedimpl08-watershed-output-row-model-and-parquet-writer-activation-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHED08 by replacing watershed output placeholder refusal
(`OWSOUT-E-004`) with real row-model-backed parquet emission for all required
watershed interchange outputs, promote the WSHED03 parquet expected-failure
vector to active conformance, and preserve typed fail-closed writer behavior.

## Why This Package Exists
WSHEDIMPL07 closed WS12 continuity migration but watershed publication remained
blocked on placeholder writer behavior. WSHED08 is required to unblock
end-to-end watershed publication closure and WSHED09 hold-lift validation.

## Scope
### Included
- Watershed parquet writer activation in
  `crates/openwepp-watershed-output/src/writers.rs`:
  - replace placeholder guard return with schema-backed parquet emission,
  - write all 14 required watershed parquet outputs,
  - emit non-empty row batches with dataset metadata.
- Watershed output row-seed builder in
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` sourced from
  watershed dispatch/kernel execution report surfaces.
- Watershed CLI contract vector promotion in
  `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`:
  - remove ignored WSHED03 vector,
  - update legacy/output-guard expected-failure tests to active output-emission
    assertions.
- Canonical system contract/index synchronization for `GAP-SYSTEM-006` closure.
- Package artifacts, gates, review, verification, disposition, and handoff.

### Explicitly Out of Scope
- Full all-structure impoundment parser projection closure
  (`GAP-SYSTEM-007` / `GAP-IMPOUND-006`).
- Full channel sediment process parity closure (`GAP-SYSTEM-008`).
- End-to-end watershed hold-lift disposition and comparator rerun package
  (WSHED09).

## Deliverables
1. `artifacts/wshedimpl08-watershed-output-row-model-and-parquet-writer-activation-report.md`
2. `artifacts/wshedimpl08-contract-implementation-evidence.md`
3. `artifacts/wshedimpl08-contract-test-implementation-evidence.md`
4. `artifacts/wshedimpl08-preimplementation-contract-gate.md`
5. `artifacts/wshedimpl08-implementation-and-test-evidence.md`
6. `artifacts/wshedimpl08-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl08_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
Sequencing applied for WSHED08 authority scope:
1. amend canonical `SC-SYSTEM-001`/index gap posture for WSHED08 closure,
2. promote/update contract-derived watershed CLI vectors,
3. record pre-implementation gate evidence, then
4. activate production writer + row-seed implementation.

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through disposition
without requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts include explicit `Static:` and/or `Ran:` sections.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Writer activation is implementation scaffolding for required publication
  surfaces and does not claim watershed process-physics parity closure.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/package.md`
- `/workdir/openWEPP/crates/openwepp-watershed-output/src/writers.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `/workdir/openWEPP/crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl08-watershed-output-row-model-and-parquet-writer-activation-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-output/src/writers.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHED08 queue authority and current placeholder writer blocker.

### Phase B - Contract and contract-test updates
- Synchronize `SC-SYSTEM-001`/index gap posture.
- Promote/update WSHED03 watershed parquet vector expectations.

### Phase C - Runtime implementation
- Activate watershed writer with row-model-backed parquet emission.
- Implement watershed row-seed builder from execution report surfaces.

### Phase D - Validation and governance evidence
- Run required gates and scoped watershed validations.
- Update evidence, review, verification, and compliance artifacts.

### Phase E - Disposition and handoff
- Record WSHED08 closure posture and residual blockers for WSHED09.

## Exit Criteria
- `OWSOUT-E-004` placeholder block is removed from valid execution lanes.
- All 14 required watershed parquet files emit non-empty schema-compatible
  outputs.
- WSHED03 watershed parquet vector is active (not ignored) and passing.
- Typed fail-closed writer/orchestrator behavior remains intact for real
  failures.
- Required evidence artifacts are complete with truthful labeling.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local parquet writer/runtime wiring and tests only.

## Execution Outcome Summary
- WSHED08 objective is complete:
  - watershed writer now emits all required parquet outputs using concrete row
    batches and schema metadata,
  - watershed CLI builds row-model seed data from execution-report surfaces and
    no longer fails valid runs with `CLIWAT-E-034`/`OWSOUT-E-004`,
  - WSHED03 watershed parquet vector is active and passing.
- Canonical system contract/index posture is synchronized for WSHED08 closure
  (`GAP-SYSTEM-006` set to `closed`).
- Program-level watershed closure remains `HOLD` pending WSHED09 and residual
  non-WSHED08 blockers (`GAP-SYSTEM-005`, `GAP-SYSTEM-007`,
  `GAP-SYSTEM-008`).
