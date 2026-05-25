# 20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001

## Status
- state: queued
- date: 2026-05-25
- timezone: UTC

## Objective
Produce a baseline-authoritative assessment and execution plan for the
`wb11_soil_water` pathway and full-fidelity `Ep`/`Es`/`Er` migration, including
consumer/dependency landmine analysis and contract-first follow-on sequencing.

## Why This Package Exists
SIMIMPL19 closed `RM`/`Snow-Water` authority gaps but left a broader
baseline-alignment wave open for `wb11_soil_water` and evap surfaces. Recent
discussion also identified process-risk from shortcut ET substitutions and
potential hidden dependencies on plant/runtime state consumers.

This package establishes a baseline-authoritative, contract-governed migration
plan before additional production edits so future implementation waves do not
rework physics, contracts, and dependencies repeatedly.

## Scope
### Included
- Map the baseline-authoritative `wb11_soil_water` routine path and state
  surfaces from `/workdir/wepp-forest_260430_baseline` into openWEPP naming
  and ownership boundaries.
- Map full-fidelity `Ep`/`Es`/`Er` authority (equations, guards, call order,
  state dependencies) from baseline routines into a migration-ready plan.
- Identify dependency/consumer landmines for ET migration, including plant,
  canopy/residue/root state surfaces, and scheduler ordering constraints.
- Identify required canonical `SC-*` amendments and contract-derived test
  additions needed before any production implementation package.
- Produce a dependency-ordered follow-on queue artifact for baseline-authority
  soil-water + ET closure work:
  - `artifacts/soil-water-et-baseline-auth-queue.md`.
- Complete required governance artifacts for a docs-only planning package.

### Explicitly Out of Scope
- Production kernel/runtime code edits for soil-water or ET physics.
- Introducing placeholder/proxy equations for `Ep`/`Es`/`Er`.
- Re-running full replay campaigns beyond scoped evidence needed for planning.

## Deliverables
1. Baseline authority path assessment:
   - `artifacts/simimpl20-wb11-soil-water-baseline-authority-path-assessment.md`
2. ET migration dependency/landmine register:
   - `artifacts/simimpl20-ep-es-er-full-fidelity-migration-risk-register.md`
3. Contract impact crosswalk:
   - `artifacts/simimpl20-contract-impact-crosswalk.md`
4. Follow-on implementation queue (required):
   - `artifacts/soil-water-et-baseline-auth-queue.md`
5. Contract implementation evidence:
   - `artifacts/simimpl20-contract-implementation-evidence.md`
6. Contract-test implementation evidence:
   - `artifacts/simimpl20-contract-test-implementation-evidence.md`
7. Pre-implementation contract gate:
   - `artifacts/simimpl20-preimplementation-contract-gate.md`
8. Implementation/test evidence:
   - `artifacts/simimpl20-implementation-and-test-evidence.md`
9. Kernel profile checklist:
   - `artifacts/simimpl20-kernel-profile-compliance-checklist.md`
10. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl20_disposition.md`
   - `artifacts/worker-handoff.md`
11. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
12. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For follow-on code-authoring packages produced by this plan, sequence must
remain:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

Any queue item that violates this order is non-compliant.

## Autonomous Execution Intent (Required)
This package must be executable end-to-end without user intervention. Assigned
agents must execute all phases through disposition and update required artifacts
without requesting additional direction unless hard-blocked by missing local
authority or contradictory canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.
Claims without evidence-mode labeling are non-compliant.

## Provenance and Authority Posture
- Canonical authority is in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Legacy migration provenance defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- No heuristic/proxy/placeholder ET substitutions are allowed in migration
  plans or follow-on execution specs.
- Variable naming continuity with legacy WEPP symbols is required; alias maps
  must be explicit when openWEPP boundary names differ.

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
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/artifacts/simimpl18_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl19-rm-snow-water-baseline-authoritative-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl19-rm-snow-water-baseline-authoritative-closure-001/artifacts/simimpl19_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl19-rm-snow-water-baseline-authoritative-closure-001/artifacts/simimpl19-storage-state-mutation-diagnostic.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl19-rm-snow-water-baseline-authoritative-closure-001/artifacts/simimpl19-runtime-swe-publication-diagnostic.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/**`
- `docs/work-packages/README.md`
- If authority clarifications are required during assessment:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `docs/specifications/science-contracts/index.md`

## Phase Plan
### Phase A - Intake and Authority Freeze
- Confirm dependency readability and prior-package residual anchors.
- Freeze baseline authority scope for `wb11_soil_water` and `Ep`/`Es`/`Er`.

### Phase B - Baseline Path Assessment (`wb11_soil_water`)
- Build routine/state path map with canonical symbol continuity and openWEPP
  alias/ownership mapping requirements.

### Phase C - ET Full-Fidelity Dependency/Landmine Assessment
- Map `Ep`/`Es`/`Er` routine graph, guards, and state dependencies.
- Enumerate consumer landmines (plant/canopy/residue/root/scheduler coupling,
  output projections, comparator/test implications).

### Phase D - Contract/Test Impact and Queue Authoring
- Define required contract amendments and contract-derived test prerequisites.
- Author dependency-ordered follow-on queue with explicit contract-first
  sequencing and gate expectations.

### Phase E - Verification and Disposition
- Complete governance, review, verification, and disposition artifacts.
- Keep disposition in `HOLD` if baseline authority or sequencing gaps remain.

## Exit Criteria
- `wb11_soil_water` baseline-authority path assessment artifact is complete and
  traceable.
- `Ep`/`Es`/`Er` dependency/landmine register is complete with concrete
  ownership and sequencing implications.
- `artifacts/soil-water-et-baseline-auth-queue.md` exists and contains
  executable follow-on package proposals with contract-first ordering.
- Required governance artifacts are complete with truthful `Static:`/`Ran:`
  labeling.
- If any non-doc files are changed, required gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: assessment/planning package; no production physics mutation.
