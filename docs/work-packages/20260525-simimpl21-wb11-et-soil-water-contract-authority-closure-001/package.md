# 20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001

## Status
- state: package-complete-with-hold
- date: 2026-05-25
- timezone: UTC
- decision: HOLD

## Objective
Amend canonical `SC-*` authority for baseline-authoritative WB11 ET +
soil-water closure, including ET stage-memory surfaces, root-uptake semantics,
execution ordering, and alias lineage requirements identified by SIMIMPL20.

## Why This Package Exists
SIMIMPL20 completed baseline-authority assessment/planning and retained `HOLD`.
Its queue requires SIMIMPL21 to complete contract-authority closure before any
contract-derived test or production-code migration package can proceed.

This package performs the required contract-authority step so downstream
packages can execute contract-first sequencing without authority drift.

## Scope
### Included
- Amend canonical contracts:
  - `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `docs/specifications/science-contracts/index.md` (if cross-links or status
    references change)
- Encode baseline-authoritative ET state and sequencing authority from
  `/workdir/wepp-forest_260430_baseline` (commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) for:
  - stage-memory semantics (`s1`, `s2`, `tu`, `tv`),
  - root-zone uptake distribution/deficit adjustment (`UPi`, `Ui`),
  - coupled ET/perc/lateral/drain/root-uptake ordering authority,
  - soil-water aggregate lineage and alias continuity obligations.
- Update gap/register posture where authority becomes explicit, while preserving
  truthful non-promotable flags where companion closures remain incomplete.
- Produce SIMIMPL21 contract amendment evidence and downstream handoff details
  for SIMIMPL22 test-authoring scope.
- Complete required governance artifacts for this contract-authoring package.

### Explicitly Out of Scope
- Contract-derived test implementation (SIMIMPL22 scope).
- Production kernel/runtime/output code edits (SIMIMPL23+ scope).
- Tier-A replay reruns or hold-lift disposition waves (SIMIMPL25 scope).

## Deliverables
1. Contract authority amendment log:
   - `artifacts/simimpl21-contract-authority-amendment-log.md`
2. Legacy provenance citation map:
   - `artifacts/simimpl21-legacy-provenance-citation-map.md`
3. Cross-contract gap disposition update:
   - `artifacts/simimpl21-cross-contract-gap-disposition.md`
4. Contract implementation evidence:
   - `artifacts/simimpl21-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/simimpl21-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/simimpl21-preimplementation-contract-gate.md`
7. Implementation/test evidence:
   - `artifacts/simimpl21-implementation-and-test-evidence.md`
8. Kernel profile checklist:
   - `artifacts/simimpl21-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl21_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For downstream code-authoring packages produced from this authority wave,
sequencing must remain:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

SIMIMPL21 executes step 1 for the declared WB11 ET/soil-water scope.

## Autonomous Execution Intent (Required)
This package must be executable end-to-end without user intervention. Assigned
agents must execute all phases through disposition and update required artifacts
without requesting additional direction unless hard-blocked by contradictory
canonical requirements.

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
- No heuristic/proxy/placeholder ET substitutions are allowed in authority
  text.
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
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/simimpl20_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/simimpl20-wb11-soil-water-baseline-authority-path-assessment.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/simimpl20-ep-es-er-full-fidelity-migration-risk-register.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/simimpl20-contract-impact-crosswalk.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/artifacts/soil-water-et-baseline-auth-queue.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/work-packages/20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`

## Phase Plan
### Phase A - Intake and Authority Freeze
- Confirm dependency readability and SIMIMPL20 carry-forward scope.
- Freeze baseline authority boundaries for ET stage-memory, root uptake,
  ordering, and soil-water lineage symbols.

### Phase B - Contract Authority Amendments
- Implement canonical contract amendments across declared `SC-*` files.
- Add explicit provenance citations and alias mappings.
- Update invariants/guard-map obligations where required.

### Phase C - Cross-Contract Consistency and Gap Disposition
- Validate consistency of amended authority across ET/watbal/plant/soil/system
  surfaces.
- Update gap posture truthfully (closed, promotable-with-risk, non-promotable)
  with rationale.

### Phase D - Governance and Handoff
- Complete required evidence artifacts, dual reviews, and dual verifications.
- Prepare downstream handoff for SIMIMPL22 test-authoring scope.

### Phase E - Disposition
- Record final SIMIMPL21 disposition and gate posture.
- Keep disposition in `HOLD` when unresolved authority contradictions or
  required review/verification closures remain.

## Exit Criteria
- Canonical contract amendments for SIMIMPL20-identified ET/soil-water
  authority gaps are authored with explicit baseline provenance citations.
- Cross-contract consistency is documented and unresolved gaps are explicitly
  dispositioned.
- Downstream handoff requirements for SIMIMPL22 are explicit.
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
- Rationale: contract-authoring package; no production runtime mutation.
