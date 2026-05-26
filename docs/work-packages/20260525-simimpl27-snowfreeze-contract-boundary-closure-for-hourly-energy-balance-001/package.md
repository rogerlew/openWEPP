# 20260525-simimpl27-snowfreeze-contract-boundary-closure-for-hourly-energy-balance-001

## Status
- state: package-complete-with-hold
- date: 2026-05-25
- timezone: UTC
- decision: HOLD

## Objective
Close canonical `SC-SNOWFREEZE-001` boundary/API authority ambiguity for hourly
snow energy-balance migration scope, ratify concrete alias mappings, and define
downstream contract-derived test requirements for SIMIMPL28/SIMIMPL29.

## Why This Package Exists
SNOWPLAN01 queued SIMIMPL27 as the required contract-first entry package for
hourly snow migration. `SC-SNOWFREEZE-001` still carried a non-promotable
boundary-name ambiguity (`GAP-SNOWFREEZE-002`) and provisional cross-contract
boundary posture for migration scope.

SIMIMPL27 executes contract-first step 1 so implementation packages can proceed
without authority drift or boundary naming contradictions.

## Scope
### Included
- Canonical contract amendments for:
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - `docs/specifications/science-contracts/index.md`
- Boundary/API alias ratification for hourly winter migration surfaces,
  including typed runtime aliases and reserved hourly namespaces.
- Explicit downstream contract-derived test requirements for SIMIMPL28/29.
- Truthful reclassification of snowfreeze gap posture for migration scope.
- Governance artifacts and handoff package for downstream sequence.

### Explicitly Out of Scope
- Contract-derived test implementation in code (SIMIMPL28/SIMIMPL29 scope).
- Production runtime/kernel code edits.
- Tier-A/MOFE reruns and closure disposition (SIMIMPL30 scope).

## Deliverables
1. Contract amendment log:
   - `artifacts/simimpl27-contract-authority-amendment-log.md`
2. Boundary alias finalization map:
   - `artifacts/simimpl27-snow-boundary-alias-finalization-map.md`
3. Cross-contract gap disposition:
   - `artifacts/simimpl27-cross-contract-gap-disposition.md`
4. Contract implementation evidence:
   - `artifacts/simimpl27-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/simimpl27-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/simimpl27-preimplementation-contract-gate.md`
7. Implementation/test evidence:
   - `artifacts/simimpl27-implementation-and-test-evidence.md`
8. Kernel profile checklist:
   - `artifacts/simimpl27-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl27_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For downstream code-authoring packages in this wave:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

SIMIMPL27 executes step 1 for hourly snow/freeze migration scope.

## Autonomous Execution Intent (Required)
This package is execution-ready and intended for end-to-end autonomous
completion through disposition without additional user direction unless
hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.

## Provenance and Authority Posture
- Canonical authority remains in `SC-*` contract files.
- Legacy baseline migration authority defaults to:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are permitted.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/artifacts/snowplan01-snow-hourly-energy-balance-wp-queue.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`

## Intended Write Set
- `docs/work-packages/20260525-simimpl27-snowfreeze-contract-boundary-closure-for-hourly-energy-balance-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`

## Phase Plan
### Phase A - Intake and Authority Freeze
- Confirm SNOWPLAN01 queue objective and migration boundary scope.

### Phase B - Canonical Contract Authority Amendments
- Ratify concrete alias mappings and downstream contract-test requirements in
  `SC-SNOWFREEZE-001`.

### Phase C - Cross-Contract Gap Reclassification
- Reclassify gap posture for boundary/API scope with explicit companion
  contract ownership references.

### Phase D - Governance and Handoff
- Produce evidence, review, verification, and handoff artifacts.

### Phase E - Disposition
- Publish package disposition and queued follow-on requirements.

## Exit Criteria
- `SC-SNOWFREEZE-001` no longer carries non-promotable boundary/API naming
  ambiguity for SIMIMPL27 migration scope.
- Downstream contract-derived test requirements are explicit for SIMIMPL28/29.
- Required governance artifacts are complete with truthful evidence labels.
- If non-doc files are changed, required gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: contract-authoring package; no production runtime mutation.
