# 20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001

## Status
- state: queued
- date: 2026-05-25
- timezone: UTC

## Objective
Assess closure scope for baseline-authoritative hourly energy-balance snow
migration and publish a dependency-ordered four-package queue that can be
executed with contract-first sequencing.

## Why This Package Exists
The kernel audit documents that openWEPP still uses reduced snow coupling
instead of the legacy hourly energy-balance winter path. Implementing full
closure in one package is high-risk because it spans contract authority,
hourly forcing synthesis, snow kernel routines, and parity/disposition gates.

This planning package prepares a bounded four-package queue so execution can
proceed with explicit dependencies and observable exit signals.

## Scope
### Included
- Confirm one-package feasibility posture for hourly snow energy-balance
  closure against baseline authority and current openWEPP runtime surfaces.
- Author a dependency-ordered 4-package queue covering:
  1. contract/boundary closure,
  2. hourly forcing synthesis port,
  3. snow energy-balance kernel migration,
  4. semantic parity rerun/disposition.
- Encode mandatory contract sequencing pattern in queue constraints:
  1. contract amendments,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.
- Produce required governance placeholders for later package execution.

### Explicitly Out of Scope
- Production kernel/runtime code edits.
- Running replay/comparator lanes.
- Modifying canonical `SC-*` content in this preparation package.

## Deliverables
1. Four-package queue artifact:
   - `artifacts/snowplan01-snow-hourly-energy-balance-wp-queue.md`
2. Contract implementation evidence placeholder:
   - `artifacts/snowplan01-contract-implementation-evidence.md`
3. Contract-test implementation evidence placeholder:
   - `artifacts/snowplan01-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate placeholder:
   - `artifacts/snowplan01-preimplementation-contract-gate.md`
5. Implementation/test evidence placeholder:
   - `artifacts/snowplan01-implementation-and-test-evidence.md`
6. Kernel profile checklist placeholder:
   - `artifacts/snowplan01-kernel-profile-compliance-checklist.md`
7. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/snowplan01_disposition.md`
   - `artifacts/worker-handoff.md`
8. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
9. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For each queued code-authoring package:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

## Autonomous Execution Intent (Required)
This package is authored for autonomous execution through disposition without
additional user direction unless hard-blocked by contradictory canonical
authority.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.

## Provenance and Authority Posture
- Canonical authority remains `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy migration provenance defaults to:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed in queued
  implementation packages.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/audits/20260525_water_erosion_kernel_audit.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`
- `/workdir/wepp-forest_260430_baseline/src/radcur.for`
- `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`

## Intended Write Set
- `docs/work-packages/20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Authority Freeze
- Confirm audit gap posture and baseline hourly winter dependency chain.

### Phase B - Feasibility and Boundary Assessment
- Confirm whether full closure is feasible in one package; identify coupling
  seams requiring staged execution.

### Phase C - Queue Authoring
- Author dependency-ordered four-package queue with objectives, dependencies,
  and exit criteria.

### Phase D - Governance Placeholders
- Pre-create required evidence/review/verification artifacts in queued state.

### Phase E - Preparation Disposition
- Publish preparation disposition and worker handoff.

## Exit Criteria
- A dependency-ordered 4-package queue exists with explicit contract-first
  constraints and baseline provenance posture.
- Required preparation artifacts exist and are queued/traceable.
- Package entry is registered in `docs/work-packages/README.md`.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: planning-only documentation package; no runtime code changes.
