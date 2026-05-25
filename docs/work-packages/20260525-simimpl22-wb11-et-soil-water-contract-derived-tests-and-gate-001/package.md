# 20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001

## Status
- state: queued
- date: 2026-05-25
- timezone: UTC

## Objective
Implement SIMIMPL22 contract-derived tests and pre-implementation contract-gate
closure for baseline-authoritative WB11 ET + soil-water migration scope
established by SIMIMPL21.

## Why This Package Exists
SIMIMPL21 completed canonical contract-authority closure and intentionally
retained `HOLD`. Contract-first sequencing now requires test-authoring and
pre-implementation gate evidence before any production ET/soil-water code
migration wave begins.

SIMIMPL22 executes those prerequisite steps so SIMIMPL23 can start runtime
migration under validated guard expectations.

## Scope
### Included
- Implement contract-derived tests for SIMIMPL21 authority surfaces covering:
  - ET stage-memory transitions (`s1`, `s2`, `tu`, `tv`),
  - root-zone uptake/stress lineage (`UPi`, `Ui`, `Ws`, `Rd`/`rtd`),
  - WB11 execution ordering (`purk -> evap/evappm -> drain/lateral -> swu -> watcon`),
  - WB13 publication lineage/alias continuity (`Ep`, `Es`, `Er`,
    `Total-Soil`, `SoilWaterTotal`).
- Record pre-migration test posture and pre-implementation contract gate
  evidence required before production edits.
- Update test/governance evidence artifacts and downstream handoff details for
  SIMIMPL23 runtime migration.
- Complete required governance artifacts for this test-and-gate package.

### Explicitly Out of Scope
- Production kernel/runtime/output code edits (SIMIMPL23+ scope).
- Contract-authority rewrites outside minimal corrective clarifications
  required to keep tests coherent with canonical SIMIMPL21 authority.
- Tier-A hold-lift replay disposition (SIMIMPL25 scope).

## Deliverables
1. Contract-derived test matrix:
   - `artifacts/simimpl22-contract-derived-test-matrix.md`
2. Pre-migration test posture baseline:
   - `artifacts/simimpl22-pre-migration-failure-baseline.md`
3. Contract implementation evidence:
   - `artifacts/simimpl22-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/simimpl22-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/simimpl22-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/simimpl22-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/simimpl22-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl22_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For downstream code-authoring packages produced from this wave, sequencing must
remain:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

SIMIMPL22 executes steps 2 and 3 for the declared WB11 ET/soil-water scope.

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
- No heuristic/proxy/placeholder ET substitutions are allowed in test vectors
  or acceptance claims.
- Variable naming continuity with legacy WEPP symbols is required; alias maps
  from SIMIMPL21 contracts remain authoritative.

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
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/artifacts/simimpl21_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/artifacts/simimpl21-contract-authority-amendment-log.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/artifacts/simimpl21-legacy-provenance-citation-map.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/artifacts/simimpl21-cross-contract-gap-disposition.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/work-packages/20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001/**`
- `docs/work-packages/README.md`
- Contract-derived test files under `tests/**` required for SIMIMPL22 scope.
- Minimal clarifying contract/index edits only when required to resolve
  authoritative contradiction discovered while authoring tests.

## Phase Plan
### Phase A - Intake and Test-Scope Freeze
- Confirm SIMIMPL21 authority outputs and required test families.
- Freeze declared write set for tests and governance artifacts.

### Phase B - Contract-Derived Test Authoring
- Implement SIMIMPL21-derived tests for ET stage-memory, uptake lineage,
  WB11 ordering, and WB13 publication lineage.
- Ensure tests encode typed hard-fail/no-surrogate authority posture.

### Phase C - Pre-Implementation Contract Gate
- Execute required test/gate evidence capture.
- Record whether pre-migration behavior fails as expected and whether gate
  posture is sufficient for SIMIMPL23.

### Phase D - Governance and Handoff
- Complete required evidence, dual reviews, and dual verifications.
- Prepare downstream handoff for SIMIMPL23 runtime migration scope.

### Phase E - Disposition
- Record final SIMIMPL22 disposition and gate posture.
- Keep disposition in `HOLD` when required test closure, gate evidence, or
  review/verification requirements are incomplete.

## Exit Criteria
- Contract-derived tests are authored for all SIMIMPL21 authority families in
  declared scope.
- Pre-implementation contract gate evidence is recorded and explicit.
- Downstream handoff requirements for SIMIMPL23 are explicit.
- Required governance artifacts are complete with truthful `Static:`/`Ran:`
  labeling.
- If non-doc files are changed, required gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: test-authoring + governance package; no production runtime
  mutation expected.
