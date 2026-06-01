# 20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Identify, in one bulk discovery pass, every baseline-authoritative hourly
hydrology routine that still requires iterative substep migration in openWEPP,
and publish a dispatch-ready implementation queue for follow-on remediation
packages.

## Why This Package Exists
HPHYS0236 migrated WB18 hourly percolation to iterative substeps, but residual
families indicate additional hourly-shape gaps remain. The next action is not
piecemeal guessing; it is a complete inventory of all routines and coupling
paths that must migrate together.

## Scope
### Included
- Baseline authority scan of `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
  and dependent hourly routines (`purk.for`, `drain.for`).
- openWEPP routine inventory across:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - `crates/openwepp-hillslope-orchestrator/src/phase.rs`
  - `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs`
- Contract authority alignment audit for:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- Dispatch-ready routine queue with explicit write targets and sequencing.

### Explicitly Out of Scope
- Production kernel code changes.
- Comparator reruns or hold-lift adjudication.
- Contract-text amendments (identified only, not applied).

## Deliverables
1. `artifacts/hphys0237-hourly-routine-inventory.md`
2. `artifacts/hphys0237-contract-implementation-evidence.md`
3. `artifacts/hphys0237-contract-test-implementation-evidence.md`
4. `artifacts/hphys0237-preimplementation-contract-gate.md`
5. `artifacts/hphys0237-implementation-and-test-evidence.md`
6. `artifacts/hphys0237-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0237_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Confirm contract authority and baseline anchors.
2. Inventory baseline hourly execution routines and coupling order.
3. Inventory openWEPP routines currently implementing equivalent families.
4. Produce one consolidated “must-update” routine list with grouping and
   follow-on sequencing.
5. Publish dispatch handoff artifacts.

## Autonomous Execution Intent (Required)
Execute discovery and artifact publication end-to-end without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
Each artifact must label evidence class (`Static:` vs `Ran:`).

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0236-wb18-hourly-iterative-execution-closure-001/artifacts/hphys0236_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0236-wb18-hourly-iterative-execution-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/drain.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001/**`

## Phase Plan
### Phase A - Authority intake
- Confirm baseline hourly authority surfaces and SC coverage.

### Phase B - Bulk routine inventory
- Map baseline hourly routines to openWEPP routine ownership.
- Classify each routine as `migrated`, `partial`, or `not-migrated`.

### Phase C - Dispatch queue publication
- Publish follow-on queue grouped by coupled routine families.
- Publish handoff with explicit next package boundaries.

## Exit Criteria
- A single consolidated routine inventory exists with no uncovered hourly
  routine family in scope.
- Follow-on package queue is explicit and dispatch-ready.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: docs-only discovery and planning changes.
