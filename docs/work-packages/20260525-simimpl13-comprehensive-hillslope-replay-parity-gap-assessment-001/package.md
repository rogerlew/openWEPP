# 20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001

## Status
- state: completed-with-hold
- date: 2026-05-25
- timezone: UTC

## Objective
Produce a comprehensive assessment of all remaining gaps required to fully
implement promotable hillslope simulation replay/parity execution in openWEPP,
including timeseries execution span closure, replay comparability readiness,
and contract-test gate sufficiency.

## Why This Package Exists
SIMIMPL11 closed with `HOLD` while proving replay workflows execute end-to-end,
but parity remains non-promotable because candidate/baseline trajectory domains
are not yet comparable. Remaining gaps are now localized but spread across
runtime execution span, candidate publication semantics, and replay-tooling
surface alignment.

SIMIMPL13 converts that residual state into a complete, actionable assessment
that defines what must be true for hillslope replay/parity execution to be
considered fully implemented.

## Scope
### Included
- Consolidate residual evidence from SIMIMPL11 and predecessor SIMIMPL waves
  into one canonical replay/parity gap assessment.
- Audit the current hillslope production pipeline end-to-end:
  - `cli` input/mode ingestion,
  - `runner` runtime-surface seeding and scheduler lifecycle invocation,
  - simulation/orchestrator publication span behavior,
  - WB13/H.wat replay-candidate emission surfaces,
  - strict + semantic comparator readiness.
- Identify all remaining blockers to promotable parity replay closure,
  including but not limited to:
  - candidate timeseries span mismatch,
  - row-key domain mismatch,
  - comparator mapping drift (`.parquet`/`.dat` surface alignment),
  - contract-derived test coverage blind spots.
- Define explicit completion criteria for “fully implemented hillslope
  simulation replay/parity execution”.
- Produce an implementation-driving closure queue for follow-on packages.

### Explicitly Out of Scope
- Production physics/kernel code edits.
- Final hold-lift disposition decisions for downstream packages.
- Watershed replay/parity closure beyond hillslope scope.

## Deliverables
1. Contract/authority assessment evidence:
   - `artifacts/simimpl13-contract-implementation-evidence.md`
2. Replay/parity residual consolidation report:
   - `artifacts/simimpl13-replay-parity-residual-consolidation.md`
3. End-to-end pipeline timeseries-span audit:
   - `artifacts/simimpl13-pipeline-timeseries-span-audit.md`
4. Candidate-surface comparability gap register:
   - `artifacts/simimpl13-candidate-surface-comparability-gap-register.md`
5. Comparator tooling and schema/mapping gap audit:
   - `artifacts/simimpl13-comparator-tooling-gap-audit.md`
6. Contract-derived test blind-spot assessment:
   - `artifacts/simimpl13-contract-test-blind-spot-assessment.md`
7. Full closure criteria specification for replay/parity implementation:
   - `artifacts/simimpl13-replay-parity-full-closure-criteria.md`
8. Follow-on implementation queue (required):
   - `artifacts/replay-implementation-wp-queue.md`
9. Contract-test assessment evidence:
   - `artifacts/simimpl13-contract-test-implementation-evidence.md`
10. Pre-implementation contract gate:
   - `artifacts/simimpl13-preimplementation-contract-gate.md`
11. Implementation/test evidence:
   - `artifacts/simimpl13-implementation-and-test-evidence.md`
12. Kernel-profile compliance checklist:
   - `artifacts/simimpl13-kernel-profile-compliance-checklist.md`
13. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl13_disposition.md`
   - `artifacts/worker-handoff.md`
14. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
15. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For downstream code-authoring packages spawned from this assessment, sequence
must remain:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

No closure-wave item may be marked executable if this ordering is violated.

## Autonomous Execution Intent (Required)
This package must be execution-ready and self-contained. Assigned agents are
expected to progress phase-by-phase through disposition and artifact updates
without requesting additional user direction unless hard-blocked by missing
local authority, unreadable inputs, or contradictory canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections. Claims without explicit evidence labeling are
non-compliant.

## Physics and Authority Posture
- Legacy baseline provenance anchor remains
  `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Consolidated intake posture from `/workdir/wepp-forest` remains selective and
  contract-guarded; no wholesale migration assumptions are allowed.
- No physics invention is permitted; each proposed closure-wave item must cite
  canonical contract authority and source provenance.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/numerics/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-owner-surface-gap-closure-map.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-contract-invariant-crosswalk.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl06-simulation-owned-wb13-output-publication-001/artifacts/simimpl06_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/artifacts/simimpl07_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001/artifacts/simimpl09_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/artifacts/simimpl10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl11-tier-a-semantic-replay-recloseout-and-residual-classification-001/artifacts/simimpl11_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl11-tier-a-semantic-replay-recloseout-and-residual-classification-001/artifacts/simimpl11-residual-classification-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl11-tier-a-semantic-replay-recloseout-and-residual-classification-001/artifacts/replay-run-20260525T001432Z`
- `/workdir/wepp-forest_260430_baseline`
- `/workdir/wepp-forest`

## Intended Write Set
- `docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/**`
- `docs/work-packages/README.md`
- (assessment-dependent, if authority clarifications are required)
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `docs/specifications/science-contracts/index.md`

## Phase Plan
### Phase A - Intake and Evidence Baseline
- Confirm package authority and dependency readability.
- Consolidate SIMIMPL11 residual evidence anchors and terminology.

### Phase B - Pipeline and Span Assessment
- Audit actual execution span behavior across
  `cli -> runner -> simulation -> orchestration`.
- Identify timeseries-span and key-domain closure gaps.

### Phase C - Comparator and Contract-Test Coverage Assessment
- Assess strict/semantic comparator readiness and mapping drift.
- Assess contract-derived test blind spots against replay/parity closure needs.

### Phase D - Closure Criteria and Queue Authoring
- Define full closure criteria for promotable hillslope replay/parity
  implementation.
- Author dependency-aware closure-wave queue for follow-on packages.

### Phase E - Verification and Disposition
- Complete review/verification artifacts and gate/disposition updates.
- Keep disposition in `HOLD` if unresolved authority or closure blockers remain.

## Exit Criteria
- Residuals from SIMIMPL11 are consolidated into a deterministic gap register
  with explicit ownership and closure-wave targets.
- Pipeline audit explicitly states current vs required timeseries behavior for
  parity replay comparability.
- Comparator/tooling drift and contract-test blind spots are explicitly
  enumerated and linked to closure actions.
- `artifacts/simimpl13-replay-parity-full-closure-criteria.md` exists and
  defines measurable completion conditions.
- `artifacts/replay-implementation-wp-queue.md` exists and
  enumerates executable follow-on packages with contract-first sequencing.
- Required repository gates are run and recorded if non-doc code changes are
  introduced:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: assessment/planning package; no production execution-path mutation
  is introduced by this scaffold.
