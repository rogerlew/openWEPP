# 20260525-mofe06-single-hillslope-semantic-parity-carved-letter-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC

## Objective
Prepare and execute one MOFE hillslope semantic parity test on
`/wc1/runs/ca/carved-letter` by selecting a reasonable-closure hillslope using
`/workdir/wepppy` MOFE closure audit tooling, then generating an openWEPP
candidate and running semantic WAT comparison.

## Why This Package Exists
The carved-letter run is known to have MOFE closure defects at scale. A
single-hillslope lane is needed to isolate candidate parity behavior on a
low-closure-residual MOFE case before broader MOFE closure work.

## Scope
### Included
- Candidate hillslope selection from carved-letter MOFE rollup using objective
  closure metrics.
- Single-hillslope MOFE closure audit execution in `/workdir/wepppy`.
- Single-hillslope openWEPP candidate execution attempt from carved-letter
  hillslope inputs.
- Semantic comparator execution attempt and evidence/disposition.

### Explicitly Out of Scope
- Production kernel/parser code changes.
- Broad multi-hillslope parity campaigns.
- Watershed routing remediation.

## Deliverables
1. Hillslope selection report:
   - `artifacts/mofe06-single-hillslope-selection-report.md`
2. Semantic parity execution report:
   - `artifacts/mofe06-semantic-parity-execution-report.md`
3. Contract implementation evidence:
   - `artifacts/mofe06-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/mofe06-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/mofe06-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/mofe06-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/mofe06-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe06_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
    - `artifacts/verification_agent_a.md`
    - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end without user
intervention unless hard-blocked by contradictory canonical requirements or
unresolvable environment failures.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Contract-First Sequence (Required)
1. Evaluate whether canonical contract amendments are required for this lane.
2. Evaluate whether contract-derived tests are required for this lane.
3. Record pre-implementation contract gate evidence.
4. Execute production candidate/comparator commands for the scoped lane.

No production code edits are permitted before steps 1-3 are complete.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`
- `/workdir/wepppy/tools/hillslope_mofe_daily_closure_audit.py`

## Intended Write Set
- `docs/work-packages/20260525-mofe06-single-hillslope-semantic-parity-carved-letter-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Candidate Selection
- Rank carved-letter MOFE hillslopes by closure metrics and verify MOFE OFE
  cardinality from `H*.wat.dat`.

### Phase B - Closure Audit Execution
- Run `/workdir/wepppy` MOFE closure audit for selected hillslope and record
  closure diagnostics.

### Phase C - Contract/Pre-Implementation Gate
- Confirm whether contract amendments/tests are required for this evidence lane
  (no production code edit intent).

### Phase D - Candidate + Semantic Comparator Execution
- Attempt openWEPP single-hillslope candidate generation for the selected
  hillslope.
- Run semantic comparator if candidate generation succeeds; otherwise record
  blocker evidence with typed errors.

### Phase E - Disposition
- Record gate posture and publish GO/HOLD disposition with worker handoff.

## Exit Criteria
- A MOFE hillslope is selected using objective closure metrics.
- Closure audit evidence is captured for that hillslope.
- Candidate/comparator execution is either completed or blocked with typed,
  reproducible evidence.
- Disposition and governance artifacts are complete.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: read/execute evidence lane only; no security boundary changes.
