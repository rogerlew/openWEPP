# 20260526-frostplan01-frost-energy-solver-assessment-and-queue-001

## Status
- state: package-complete
- date: 2026-05-26
- timezone: UTC
- decision: GO

## Objective
Prepare and execute a baseline-authoritative frost-process review across
openWEPP and `wepp-forest_260430_baseline`, then publish a dependency-ordered
implementation queue (`frost-energy-solver-wp-queue.md`) that closes frost
process-parity gaps without surrogate physics.

## Why This Package Exists
SIMIMPL30 completed winter-hourly rerun/disposition work but retained `HOLD`
because canonical contract posture still carries unresolved `frost.hourly.*`
closure obligations and no admissible parity lane currently demonstrates
frost-process closure.

The current openWEPP frost path is still reductive relative to baseline
frost-energy routines. This package creates an execution-ready review + queue
path so follow-on work can migrate to baseline-authoritative frost process
parity under contract-first governance.

## Scope
### Included
- Review and document the current openWEPP frost coupling implementation and
  guards (`compute_active_frost_coupling` path) against baseline-authoritative
  frost/winter routine families.
- Capture baseline routine-chain authority for frost process parity from:
  `winter.for`, `frostn.for`, `frsoil.for`, `frwatc.for`, `frzng.for`,
  `frznw.for`, `winthd.for`, and `getfreezecond.for`.
- Publish dependency-ordered queue artifact:
  `artifacts/frost-energy-solver-wp-queue.md`.
- Encode mandatory contract-first sequencing constraints for every follow-on
  code-authoring package:
  1. contracts,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production edits.
- Pre-create required governance/review/verification artifacts for
  no-intervention execution through disposition.

### Explicitly Out of Scope
- Production kernel/runtime code edits in this preparation package.
- Silent fallback wrappers or heuristic/proxy frost physics substitutions.
- Declaring hold-lift closure inside this package.

## Deliverables
1. Frost implementation review artifact:
   - `artifacts/frostplan01-openwepp-vs-baseline-frost-implementation-review.md`
2. Frost parity queue artifact:
   - `artifacts/frost-energy-solver-wp-queue.md`
3. Contract implementation evidence placeholder:
   - `artifacts/frostplan01-contract-implementation-evidence.md`
4. Contract-test implementation evidence placeholder:
   - `artifacts/frostplan01-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate placeholder:
   - `artifacts/frostplan01-preimplementation-contract-gate.md`
6. Implementation/test evidence placeholder:
   - `artifacts/frostplan01-implementation-and-test-evidence.md`
7. Kernel profile compliance checklist placeholder:
   - `artifacts/frostplan01-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/frostplan01_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
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
- Canonical process authority is in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy migration provenance defaults to:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Work-package artifacts are evidence and do not replace canonical authority.
- No heuristic/proxy frost process substitutions are acceptable as closure.
- Variable naming continuity with legacy WEPP symbols is required; when runtime
  names differ, publish explicit alias mappings in canonical contracts.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/artifacts/snowplan01-snow-hourly-energy-balance-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl29-snowd-melt-energy-balance-kernel-port-and-coupling-001/artifacts/simimpl29_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/artifacts/simimpl30_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/artifacts/simimpl30-hold-lift-decision-report.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/frostn.for`
- `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
- `/workdir/wepp-forest_260430_baseline/src/frwatc.for`
- `/workdir/wepp-forest_260430_baseline/src/frzng.for`
- `/workdir/wepp-forest_260430_baseline/src/frznw.for`
- `/workdir/wepp-forest_260430_baseline/src/winthd.for`
- `/workdir/wepp-forest_260430_baseline/src/getfreezecond.for`

## Intended Write Set
- `docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Authority Freeze
- Confirm queue authorization from SIMIMPL30 `HOLD` rationale and freeze
  baseline authority inputs for frost process parity.

### Phase B - openWEPP vs Baseline Frost Review
- Produce implementation-shape review focused on current openWEPP frost
  reduction versus baseline frost-energy solver routine chain.

### Phase C - Queue Authoring
- Publish dependency-ordered frost process-parity queue with explicit
  contract-first constraints and exit signals.

### Phase D - Governance Placeholders and Prompt Readiness
- Ensure all required artifact placeholders and kickoff prompt constraints are
  present for autonomous execution.

### Phase E - Preparation Disposition
- Publish worker handoff and disposition for preparation completion.
- Keep preparation disposition in `HOLD` if queue preconditions are incomplete.

## Exit Criteria
- `artifacts/frostplan01-openwepp-vs-baseline-frost-implementation-review.md`
  documents review findings and provenance anchors.
- `artifacts/frost-energy-solver-wp-queue.md` exists with dependency-ordered
  follow-on packages and contract-first sequencing constraints.
- Required governance artifacts exist with truthful `Static:`/`Ran:` labeling.
- Package entry is registered in `docs/work-packages/README.md`.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: planning/governance package; no production code changes.
