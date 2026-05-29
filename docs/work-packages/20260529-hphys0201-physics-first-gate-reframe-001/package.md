# 20260529-hphys0201-physics-first-gate-reframe-001

## Status
- state: completed
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: GO

## Objective
Reframe HPARITY02+ follow-on execution to a physics-correctness posture where
contract-authoritative process lineage is the primary gate and semantic parity
is a diagnostic signal.

## Why This Package Exists
Recent HPARITY execution showed that parity-only closure objectives can drive
rework without proving process authority closure. We need a documented gate
reframe before additional kernel implementation waves.

## Scope
### Included
- Define physics-first closure measures for HPHYS0202–HPHYS0204.
- Encode comparator/parity as diagnostic evidence tier, not primary
  promotability gate.
- Update queue/readme wording so package intent is explicit and deterministic.
- Publish immediate-next-action handoff for the implementation packages.

### Explicitly Out of Scope
- Production Rust kernel code changes.
- Science-contract equation changes.
- Cohort reruns or benchmark execution.

## Closure Measures (Required)
1. `MEASURE-HP201-001`: HPHYS follow-on package objectives and exit criteria
   explicitly prioritize contract-authoritative process closure.
2. `MEASURE-HP201-002`: parity/comparator evidence is explicitly labeled as
   diagnostic/investigation in package closure criteria.
3. `MEASURE-HP201-003`: package queue entries and dependencies reflect the new
   execution sequence (`HPHYS0201 -> HPHYS0202 -> HPHYS0203 -> HPHYS0204`).
4. `MEASURE-HP201-004`: all required HPHYS0201 artifacts are populated through
   disposition with truthfulness labels.

## Deliverables
1. `artifacts/hphys0201-physics-gap-matrix.md`
2. `artifacts/hphys0201-contract-implementation-evidence.md`
3. `artifacts/hphys0201-contract-test-implementation-evidence.md`
4. `artifacts/hphys0201-preimplementation-contract-gate.md`
5. `artifacts/hphys0201-implementation-and-test-evidence.md`
6. `artifacts/hphys0201-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0201_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend package governance wording and dependencies first.
2. Add/adjust contract-derived test expectations for follow-on package scopes.
3. Record pre-implementation gate evidence for follow-on execution posture.
4. Update queue/roadmap documents and publish disposition.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy migration provenance remains anchored to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No surrogate/proxy process-physics substitutions are allowed in production
  execution packages.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity02-profile-capacity-storage-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hphys0201-physics-first-gate-reframe-001/**`
- `docs/work-packages/20260529-hphys0202-profile-fc-wp-lineage-closure-001/package.md`
- `docs/work-packages/20260529-hphys0203-physics-robustness-test-suite-001/package.md`
- `docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/package.md`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm user authorization for physics-first gate reframe.
- Freeze scope to package/roadmap scaffolding only.

### Phase B - Governance mapping
- Define explicit physics-first closure criteria and parity-diagnostic
  classification for follow-on packages.

### Phase C - Contract-derived expectation updates
- Update follow-on package expectations so tests/validation prove contract
  process closure before parity interpretation.

### Phase D - Pre-implementation gate
- Record readiness evidence that follow-on scopes are contract-first and
  autonomy-ready.

### Phase E - Queue and scaffold finalization
- Update package queue entries and cross-package dependencies.

### Phase F - Validation
- Run markdown/doc hygiene checks needed for package-only updates.

### Phase G - Dual review, dual verification, disposition
- Complete review/verification artifacts and disposition.

## Exit Criteria
- Closure measures `MEASURE-HP201-001..004` are satisfied and evidenced.
- Follow-on implementation packages are execution-ready under physics-first
  closure posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: docs/work-package governance updates only; no auth/network/runtime
  surface changes.
