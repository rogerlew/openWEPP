# 20260525-simimpl24-wb11-soil-water-lineage-and-publication-closure-001

## Status
- state: queued
- date: 2026-05-25
- timezone: UTC

## Objective
Close WB11 aggregate soil-water lineage and WB13 publication semantics for
`wb11_soil_water` and ET-related WB13 surfaces (`Ep`, `Es`, `Er`,
`Total-Soil`, `SoilWaterTotal`) using simulation-owned runtime lineage only.

## Why This Package Exists
SIMIMPL23 closed baseline-authoritative ET runtime migration and enabled the
SIMIMPL22 contract-derived closure vectors. Queue step 4 from SIMIMPL20 now
requires explicit lineage/publication closure for WB13-facing surfaces before
Tier-A rerun/disposition hold-lift work can proceed.

## Scope
### Included
- Production kernel/runtime/output edits needed to complete WB11 aggregate
  lineage closure from runtime layer/state surfaces to WB13 publication fields.
- Contract-derived output closure tests for WB13 publication semantics tied to
  canonical `SC-WATBAL-001` and `SC-SYSTEM-001` authority.
- Typed guard/error posture for missing/non-finite/domain-invalid publication
  lineage inputs at runtime boundaries.
- Governance artifacts, gate evidence, dual review/verification, and
  SIMIMPL25 handoff updates.

### Explicitly Out of Scope
- New process-physics substitutions unrelated to WB11 lineage/publication.
- Tier-A replay rerun and hold-lift disposition (`SIMIMPL25` scope).
- Canonical contract rewrites beyond minimal contradiction fixes required for
  scoped implementation coherence.

## Deliverables
1. Soil-water lineage provenance map:
   - `artifacts/simimpl24-soil-water-lineage-provenance-map.md`
2. WB13 publication surface closure report:
   - `artifacts/simimpl24-publication-surface-closure-report.md`
3. Contract implementation evidence:
   - `artifacts/simimpl24-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/simimpl24-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/simimpl24-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/simimpl24-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/simimpl24-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl24_disposition.md`
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

SIMIMPL21/SIMIMPL22/SIMIMPL23 provide prerequisite authority/test/migration
context; SIMIMPL24 must preserve this sequence for any additional scoped
contract/test closure work.

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
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl23-wb11-et-full-fidelity-kernel-migration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl23-wb11-et-full-fidelity-kernel-migration-001/artifacts/simimpl23_disposition.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/work-packages/20260525-simimpl24-wb11-soil-water-lineage-and-publication-closure-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-output/**`
- `crates/openwepp-summary-accumulator/**`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- `tests/integration/wb13_daily_water_balance_output_surface_contract.rs`
- Additional contract-derived tests under `tests/**` required for scoped
  lineage/publication closure.

## Phase Plan
### Phase A - Intake and Preconditions
- Confirm SIMIMPL20 queue authorization and SIMIMPL21/22/23 prerequisite
  artifacts remain authoritative.
- Confirm package write set and lineage/publication closure targets.

### Phase B - Lineage and Publication Implementation
- Implement runtime/publication closure for WB11 aggregate soil-water lineage
  and WB13 ET/soil-water publication surfaces.
- Preserve typed guard/error behavior; prohibit silent defaults.

### Phase C - Contract-Derived Closure and Gates
- Update or add contract-derived output tests for lineage/publication closure.
- Run required package gates and capture truthful evidence.

### Phase D - Governance and Handoff
- Complete required artifacts, dual reviews, and dual verifications.
- Prepare explicit handoff for SIMIMPL25 rerun/disposition scope.

### Phase E - Disposition
- Record final SIMIMPL24 disposition and hold posture.
- Keep disposition in `HOLD` when required closure vectors, gates, or
  governance evidence remain incomplete.

## Exit Criteria
- WB11 aggregate soil-water lineage and scoped WB13 publication semantics are
  implemented with simulation-owned runtime provenance.
- Contract-derived lineage/publication closure vectors are updated/run with
  explicit pass/fail posture.
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
- Rationale: production kernel/runtime publication mutation with typed-guard
  posture and contract-governed boundaries.
