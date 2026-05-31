# 20260531-auth09-legacy-sanity-tier-normalization-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Close AUTH08A review findings F-2/F-3/F-4 by introducing a canonical
legacy/sanity authority tier below Level-4 and re-tiering the WB19 `solwpv`
branch-conformance suite so suite ID, authority level, and governance posture
are aligned.

## Why This Package Exists
AUTH08A correctly de-blocked the WB19 branch suite, but left a taxonomy
inconsistency: `cas_l4_*` ID, `authority_level: 5`, and legacy-conformance
semantics conflict. The external-authority model currently has no dedicated
legacy/sanity suite tier, so legacy-anchored checks have no canonical home.

## Autonomous Execution Intent
This package is execution-ready and self-contained. It provides all sequencing,
target files, and validation expectations needed for end-to-end execution
without additional user direction unless hard-blocked.

## Scope
### Included
- Add Level-3 legacy/sanity semantics to authority model and suite schema.
- Re-tier WB19 branch suite from `cas_l4_*`/`authority_level: 5` to
  `cas_l3_*`/`authority_level: 3`.
- Align registry, suite doc, fixture pathing/provenance, and SC addendum
  references to the Level-3 suite ID.
- Update contract-derived integration tests to assert the new canonical tier.
- Publish full package artifacts through disposition.

### Explicitly Out of Scope
- Production hydrology/kernel algorithm changes.
- New constitutive physics equations or tolerances.
- Re-adjudication reruns for HPHYS residual families.

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
1. Amend canonical authority/governance text (`correctness-authority-model`,
   `suite-schema`, external-authority suite/registry, SC addendum references).
2. Amend contract-derived tests and fixture metadata/path assertions.
3. Record pre-implementation contract gate evidence.
4. Modify production code only if required by contract/test deltas (expected:
   none) and run workspace validation gates.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth08a-solwpv-branch-gate-authority-retiering-001/artifacts/claude-code-review-findings.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth09-legacy-sanity-tier-normalization-001/**`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/external-authority/README.md`
- `docs/specifications/external-authority/suite-schema.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l3_subhyd_solwpv_fcdep_branch_001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/fixtures/constitutive/cas_l3_subhyd_solwpv_fcdep_branch_001/*`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`

## Phase Plan
1. **Phase A — Contract/authority amendments**
   - Update authority model, suite schema, registry, suite spec, and SC
     addendum references to introduce/use Level-3 legacy/sanity tier.
2. **Phase B — Contract-derived test amendments**
   - Update AUTH06/AUTH08 integration assertions and fixture references to new
     suite ID/path/tier semantics.
3. **Phase C — Validation gates and evidence**
   - Run required workspace gates and collect outputs in artifacts.
4. **Phase D — Disposition and handoff**
   - Publish HOLD/GO disposition and follow-on recommendations.

## Exit Criteria
- Authority docs canonically define a Level-3 legacy/sanity tier.
- WB19 branch suite is named/routed consistently as
  `cas_l3_subhyd_solwpv_fcdep_branch_001` with `authority_level: 3`,
  `gate_lane: periodic`, and `failure_class: investigation`.
- Suite, registry, fixture metadata, SC references, and tests are coherent and
  passing.
- Package artifacts are complete with truthful `Static:`/`Ran:` labeling.

## Truthfulness Labeling Requirement
All artifact claims must be prefixed with `Static:` or `Ran:` and only report
commands actually executed in this package.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: governance/docs/tests/fixture-metadata retiering only; no network
  or secret handling changes.
