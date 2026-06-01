# 20260531-auth11-required-suite-obligation-and-antievasion-guards-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Add source-level, agent-executed guardrails that prevent required-suite
evidence-set manipulation, including case removal, threshold loosening, silent
lane posture changes, and untracked promotion sequencing.

## Why This Package Exists
AUTH10 review identified a structural governance failure: a known failing
anchor case (`valid_9002`) was removed while suite posture moved to
blocking semantics. Fixture integrity checks alone were insufficient because
they validated hashes/provenance but not fixture obligations and posture-change
semantics.

## Autonomous Execution Intent
This package is execution-ready and self-contained. It executes end-to-end
through disposition without additional user direction unless hard-blocked.

## Scope
### Included
- Add machine-readable required-suite obligation manifest.
- Add diff-based anti-evasion review guard script (local/source guard, not CI
  required).
- Add explicit promotion protocol (`red -> fix -> green`) for lane/failure
  posture changes.
- Add contract-derived anchor-binding tests.
- Restore required anchored discrepancy fixture coverage (`valid_9002`) in the
  direct-theta cohort with explicit threshold-status classification.

### Explicitly Out of Scope
- FC kernel physics remediation.
- CODEOWNERS/branch-protection ownership controls.
- External CI branch-protection policy changes.

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
1. Amend canonical authority/governance docs and guard protocol surfaces.
2. Amend contract-derived tests and fixtures.
3. Record pre-implementation contract gate evidence.
4. Run workspace gates and publish disposition.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth10-fc-authority-gate-and-suite-consistency-001/artifacts/claude-code-review-findings.md`

## Intended Write Set
- `AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth11-required-suite-obligation-and-antievasion-guards-001/**`
- `docs/specifications/external-authority/README.md`
- `docs/specifications/external-authority/suite-schema.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_soil_fc_direct_theta_minus33_cohort_001.md`
- `docs/specifications/external-authority/required-suite-obligations.json`
- `docs/specifications/external-authority/promotion-protocol.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/auth07_fc_authority_cohort_contract.rs`
- `tests/integration/auth11_required_suite_obligation_guards_contract.rs`
- `tests/fixtures/constitutive/cas_l4_soil_fc_direct_theta_minus33_cohort_001/*`
- `tools/release/check_authority_suite_antievasion.sh`
- `tools/release/README.md`

## Phase Plan
1. **Phase A - Governance and protocol surfaces**
   - Add obligation manifest and promotion protocol.
   - Re-anchor direct-theta suite posture and metadata.
2. **Phase B - Contract-derived test and fixture surfaces**
   - Restore anchored fixture coverage and threshold-status bindings.
   - Add anchor-guard test coverage and anti-evasion tool wiring.
3. **Phase C - Validation**
   - Run required gates and anti-evasion script.
4. **Phase D - Disposition**
   - Publish GO/HOLD with explicit follow-on physics closure requirements.

## Exit Criteria
- Required anchor case bindings are machine-checked and test-enforced.
- Diff-based anti-evasion script blocks case-removal / threshold-loosening /
  uncontrolled lane-change edits.
- Promotion protocol is canonicalized in repo docs and referenced by contract
  authority.
- Workspace gates pass and package artifacts are complete with truthful labels.

## Truthfulness Labeling Requirement
All artifacts must explicitly label evidence as `Static:` and/or `Ran:`.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: docs/tests/fixture/review-guard tooling only; no secret or network
  path changes.
