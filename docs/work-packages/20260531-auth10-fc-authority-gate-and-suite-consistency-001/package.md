# 20260531-auth10-fc-authority-gate-and-suite-consistency-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Execute the AUTH09 follow-on closure requested by the review findings:
1. close remaining Level-3 suite rename/provenance inconsistencies, and
2. remove inverted FC direct-theta cohort behavior by separating blocking
   constitutive gate behavior from non-blocking discrepancy monitoring.

## Why This Package Exists
AUTH09 correctly introduced the Level-3 legacy/sanity tier, but follow-on
review findings identified remaining inconsistencies:
- stale `cas_l4_*` provenance pathing in the renamed Level-3 WB19 suite,
- an AUTH07 cohort fixture label mismatch,
- and non-blocking/inverted discrepancy pinning in the FC direct-theta cohort
  test posture.

## Autonomous Execution Intent
This package is execution-ready and self-contained. It must run end-to-end
through disposition without additional user direction unless hard-blocked.

## Scope
### Included
- Fix AUTH09 Level-3 WB19 suite metadata/provenance coherence.
- Correct AUTH07 fixture label inconsistency (`h1_high_rock_authority` bucket).
- Convert AUTH07 cohort test posture to non-inverted monitoring semantics.
- Promote direct-theta FC cohort coverage to explicit Level-4
  required/hard-fail gate semantics in the AUTH07 contract-derived suite path.
- Update registry/suite/contract text so lane semantics and test intent are
  coherent.

### Explicitly Out of Scope
- Production kernel algorithm rewrites in `crates/**`.
- Full FC process-physics remediation for known high-error soil profiles.
- New external data acquisition.

## Deliverables
1. `artifacts/contract-implementation-evidence.md`
2. `artifacts/contract-test-implementation-evidence.md`
3. `artifacts/preimplementation-contract-gate.md`
4. `artifacts/implementation-and-test-evidence.md`
5. `artifacts/kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence
1. Amend canonical authority/spec/contract suite definitions.
2. Amend contract-derived tests and fixture metadata.
3. Record pre-implementation contract-gate evidence.
4. Apply code-level changes (tests/fixtures/docs expected; no kernel algorithm
   edits expected) and run workspace gates.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/specifications/external-authority/suites/cas_l3_subhyd_solwpv_fcdep_branch_001.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suites/cas_l4_soil_fc_direct_theta_minus33_cohort_001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth09-legacy-sanity-tier-normalization-001/artifacts/claude-code-review-findings.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth10-fc-authority-gate-and-suite-consistency-001/**`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l3_subhyd_solwpv_fcdep_branch_001.md`
- `docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md`
- `docs/specifications/external-authority/suites/cas_l4_soil_fc_direct_theta_minus33_cohort_001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `tests/integration/auth07_fc_authority_cohort_contract.rs`
- `tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/*`

## Phase Plan
1. **Phase A — Contract/spec amendments**
   - Normalize suite metadata/provenance and suite posture language.
2. **Phase B — Contract-derived test + fixture amendments**
  - Remove inversion-prone AUTH07 expectations and enforce Level-4 direct-theta
    FC gate assertions in AUTH07.
3. **Phase C — Validation**
   - Run workspace gates and record results.
4. **Phase D — Disposition**
   - Publish GO/HOLD decision with follow-on queue if gaps remain.

## Exit Criteria
- Level-3 WB19 suite metadata no longer references stale `cas_l4_*` pathing for
  active fixture provenance fields.
- AUTH07 cohort test no longer pins discrepancy as acceptance behavior.
- Level-4 required/hard-fail FC authority test path includes direct-theta
  comparator coverage with explicit threshold semantics.
- Workspace gates pass and artifacts are complete with truthful labels.

## Truthfulness Labeling Requirement
All artifacts must explicitly label evidence as `Static:` and/or `Ran:`.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: docs/tests/fixture metadata only; no secret/material access paths.
