# 20260531-auth08a-solwpv-branch-gate-authority-retiering-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Re-tier the WB19 `solwpv` branch-law suite from blocking constitutive posture
to non-blocking legacy-conformance posture, consistent with
correctness-reanchoring governance.

## Why This Package Exists
The HPHYS0222 review identified a governance mismatch: the branch-law suite
`cas_l4_subhyd_solwpv_fcdep_branch_001` is legacy-anchored conformance
evidence, not independent constitutive physics authority. Required/hard-fail
classification overstated its authority role.

## Scope
### Included
- Update external-authority registry lane/failure-class and authority level.
- Update suite spec wording to legacy-conformance posture.
- Update `SC-SUBHYD-001` and `SC-WATBAL-001` addendum text to match retiering.
- Update contract-derived integration assertions (`auth08` test).
- Publish package evidence/disposition.

### Explicitly Out of Scope
- Production kernel physics changes.
- Fixture data rewrites.
- New constitutive physics suite authoring.

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
1. Amend registry/suite/SC governance text.
2. Amend contract-derived test assertions.
3. Run tests/gates.
4. Publish artifacts and disposition.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/specifications/external-authority/suites/cas_l4_subhyd_solwpv_fcdep_branch_001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth08a-solwpv-branch-gate-authority-retiering-001/**`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_solwpv_fcdep_branch_001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`

## Exit Criteria
- Branch suite is no longer `required`/`hard-fail`.
- Suite/contract text clearly states legacy-conformance posture.
- Contract-derived tests pass with updated expectations.

## Truthfulness Labeling Requirement
Artifacts must label evidence as `Static:` and/or `Ran:`.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: governance/docs/tests only.
