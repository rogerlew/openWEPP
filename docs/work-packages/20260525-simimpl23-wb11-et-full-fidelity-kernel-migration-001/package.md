# 20260525-simimpl23-wb11-et-full-fidelity-kernel-migration-001

## Status
- state: queued
- date: 2026-05-25
- timezone: UTC

## Objective
Implement baseline-authoritative WB11 ET runtime migration (`evap` + `swu`
semantics) in openWEPP architecture, using SIMIMPL21 canonical contract
authority and SIMIMPL22 contract-derived test vectors.

## Why This Package Exists
SIMIMPL22 completed contract-derived tests and pre-implementation gate evidence
and retained `HOLD` with explicit failing vectors for stage-memory, uptake
lineage, ordering, and WB13 lineage symbols. This package executes the
production runtime migration wave required to close ET physics/runtime gaps
before downstream lineage/publication closure and replay disposition packages.

## Scope
### Included
- Production kernel/runtime edits required to implement baseline-authoritative
  ET behavior for WB11 scope:
  - stage-memory surfaces and transitions (`s1`, `s2`, `tu`, `tv`),
  - layer-aware uptake lineage (`UPi`, `Ui`) and stress coupling (`Ws`),
  - baseline-compatible ET coupling across WB11 hydrology execution surfaces,
  - typed guard/error posture for domain and non-finite violations.
- Contract-derived test updates/additions required to validate migrated ET
  runtime behavior against SIMIMPL21/SIMIMPL22 authority.
- Package governance artifacts, gate evidence, review, verification, and
  downstream SIMIMPL24 handoff updates.

### Explicitly Out of Scope
- Canonical contract-authority rewrites beyond minimal contradiction fixes.
- Tier-A replay reruns and hold-lift disposition (`SIMIMPL25` scope).
- Full WB13 publication-lineage closeout when closure requires broader
  soil-water/publication scope reserved to `SIMIMPL24`.

## Deliverables
1. ET runtime migration provenance map:
   - `artifacts/simimpl23-runtime-migration-provenance-map.md`
2. Stage-memory and uptake closure report:
   - `artifacts/simimpl23-stage-memory-and-uptake-closure-report.md`
3. Ordering and coupling closure report:
   - `artifacts/simimpl23-ordering-and-coupling-closure-report.md`
4. Contract implementation evidence:
   - `artifacts/simimpl23-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/simimpl23-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/simimpl23-preimplementation-contract-gate.md`
7. Implementation/test evidence:
   - `artifacts/simimpl23-implementation-and-test-evidence.md`
8. Kernel profile checklist:
   - `artifacts/simimpl23-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl23_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
Kernel-affecting sequencing remains mandatory:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

SIMIMPL21 and SIMIMPL22 provide steps 1-3 for this scope. SIMIMPL23 executes
step 4 and any required contract-derived closure tests tied to migrated code.

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
- No heuristic/proxy/placeholder ET substitutions are allowed in production
  paths.
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
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/artifacts/simimpl21_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001/artifacts/simimpl22-contract-derived-test-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001/artifacts/simimpl22-pre-migration-failure-baseline.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001/artifacts/simimpl22-preimplementation-contract-gate.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001/artifacts/simimpl22_disposition.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/work-packages/20260525-simimpl23-wb11-et-full-fidelity-kernel-migration-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/**`
- `crates/openwepp-hillslope-output/**`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- Additional contract-derived test files under `tests/**` required for scoped
  ET migration closure.

## Phase Plan
### Phase A - Intake and Preconditions
- Confirm SIMIMPL21 authority and SIMIMPL22 gate outputs remain authoritative.
- Confirm package write set and migration closure targets.

### Phase B - ET Runtime Migration Implementation
- Implement baseline-authoritative ET stage-memory and uptake behavior in
  production runtime paths with typed guards.
- Preserve required WB11 coupling semantics without surrogate heuristics.

### Phase C - Contract-Derived Closure and Gates
- Update/enable contract-derived vectors for migrated behavior.
- Run required package gates and capture truthful run evidence.

### Phase D - Governance and Handoff
- Complete required artifacts, dual reviews, and dual verifications.
- Prepare explicit handoff for SIMIMPL24 unresolved scope (if any).

### Phase E - Disposition
- Record final SIMIMPL23 disposition and hold posture.
- Keep disposition in `HOLD` when required closure vectors, gates, or
  governance evidence remain incomplete.

## Exit Criteria
- Baseline-authoritative ET runtime migration surfaces are implemented for
  scoped stage-memory and uptake-lineage behavior.
- Contract-derived vectors are updated/run with explicit pass/fail posture.
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
- Rationale: production kernel/runtime mutation with typed-guard policy and
  existing contract-governed boundaries.
