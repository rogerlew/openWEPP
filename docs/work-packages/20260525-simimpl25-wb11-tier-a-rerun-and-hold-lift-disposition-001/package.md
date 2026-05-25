# 20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001

## Status
- state: queued
- date: 2026-05-25
- timezone: UTC

## Objective
Execute post-SIMIMPL24 Tier-A replay rerun, publish closure evidence for WB11
soil-water/publication lineage surfaces, and produce explicit hold-lift
recommendation disposition.

## Why This Package Exists
SIMIMPL24 closed WB11 lineage/publication implementation and restored workspace
contract-gate pass posture. The remaining queue step is rerun-based evidence and
final hold-lift decisioning, carried by SIMIMPL25.

## Scope
### Included
- Tier-A rerun/replay evidence capture for SIMIMPL24-affected publication
  surfaces (`RM`, `Snow-Water`, `Total-Soil`, `SoilWaterTotal`, `Ep`, `Es`,
  `Er`, `Q`, `Dp`, `latqcc`).
- Comparator/semantic replay evidence updates and pass/fail posture recording
  for PL14/PL14R/PL14S/PL15/PL15R lanes relevant to SIMIMPL24 closure.
- Governance artifacts, gate evidence, dual review/verification, and explicit
  hold-lift recommendation.

### Explicitly Out of Scope
- New production process-physics feature work outside rerun-discovered
  correctness blockers.
- Contract-authority rewrites unless rerun evidence proves contradiction.
- Unrelated observability/docs/planning modifications outside SIMIMPL25 write
  scope.

## Deliverables
1. Tier-A rerun evidence report:
   - `artifacts/simimpl25-tier-a-rerun-evidence-report.md`
2. Hold-lift decision report:
   - `artifacts/simimpl25-hold-lift-decision-report.md`
3. Contract implementation evidence:
   - `artifacts/simimpl25-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/simimpl25-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/simimpl25-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/simimpl25-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/simimpl25-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl25_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
Kernel-affecting sequencing remains mandatory:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

SIMIMPL25 is expected to be evidence/disposition-heavy; if rerun findings
require code changes, this sequence is mandatory before production edits.

## Autonomous Execution Intent (Required)
This package must be executable end-to-end without user intervention. Assigned
agents must execute all phases through disposition and update required artifacts
without requesting additional direction unless hard-blocked by contradictory
canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.
Claims without evidence-mode labeling are non-compliant.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Legacy migration provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- No heuristic/proxy/placeholder publication or soil-water lineage
  substitutions are allowed in production paths.
- Variable naming continuity with legacy WEPP symbols is required; when
  boundary names differ, explicit alias mapping must be preserved.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/soil-water-et-baseline-auth-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/artifacts/simimpl21_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001/artifacts/simimpl22_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl23-wb11-et-full-fidelity-kernel-migration-001/artifacts/simimpl23_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl24-wb11-soil-water-lineage-and-publication-closure-001/artifacts/simimpl24_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl24-wb11-soil-water-lineage-and-publication-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/work-packages/20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001/**`
- `docs/work-packages/README.md`
- `tools/legacy_comparison_suite/**` (evidence updates only, if required)
- `tests/integration/pl14*_tier_a_*`
- `tests/integration/pl15*_tier_a_*`

## Phase Plan
### Phase A - Intake and Preconditions
- Confirm SIMIMPL24 disposition/handoff authorizes SIMIMPL25 rerun scope.
- Confirm prerequisite SIMIMPL21/22/23/24 artifacts remain authoritative.

### Phase B - Tier-A Rerun Execution
- Execute rerun/comparator workflow for affected Tier-A lanes.
- Capture semantic deltas and publication-surface closure outcomes.

### Phase C - Contract-Derived Closure and Gates
- Update/add contract-derived replay vectors only if rerun findings require it.
- Run required package gates and capture truthful evidence.

### Phase D - Governance and Hold-Lift Recommendation
- Complete required artifacts, dual reviews, and dual verifications.
- Publish explicit hold-lift recommendation with residual risk statement.

### Phase E - Disposition
- Record final SIMIMPL25 disposition.
- Keep disposition in `HOLD` when rerun closure vectors, gates, or governance
  evidence remain incomplete.

## Exit Criteria
- Tier-A rerun evidence is captured for SIMIMPL24-affected surfaces and
  contract vectors.
- Hold-lift recommendation is explicit, evidence-backed, and traceable to
  canonical contract posture.
- Required governance artifacts are complete with truthful `Static:`/`Ran:`
  labeling.
- Required non-doc gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: no
- Rationale: rerun/disposition package that may trigger bounded production
  corrections under typed-guard and contract-governed boundaries.
