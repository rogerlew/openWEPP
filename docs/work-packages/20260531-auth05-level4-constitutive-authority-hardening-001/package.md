# 20260531-auth05-level4-constitutive-authority-hardening-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Harden AUTH03 Level-4 constitutive gate authority so FC/WP and relax-to-FC
checks are model-to-authority checks on real soil inputs, not fixture
self-consistency checks.

## Why This Package Exists
AUTH03 established the first Level-4 suite structure and blocking lane posture.
Review findings in
`docs/work-packages/20260531-auth03-level4-constitutive-gate-bootstrap-001/artifacts/claude-code-review-findings.md`
identified remaining authority gaps:
- FC/WP fixture self-consistency without runtime-authority comparison.
- Optional relax-to-FC positive-branch assertions.
- Legacy baseline citation mixed into Level-4 external authority suite docs.

## Scope
### Included
- Add AUTH05 contract-derived checks in the AUTH03 Level-4 integration test
  target for runtime FC/WP model-to-authority comparison on real soil fixtures.
- Remove legacy-baseline entries from Level-4 suite `external_citations`
  sections where those entries are acting as constitutive authority.
- Tighten relax-to-FC fixture assertion schema so positive-branch checks cannot
  be silently skipped.
- Publish complete AUTH05 artifacts with evidence labels and disposition.

### Explicitly Out of Scope
- Production kernel/process-physics rewrites for over-drainage remediation.
- New Level-5/6 suites.
- Comparator-threshold policy changes outside AUTH03/AUTH05 gate scope.

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

## Mandatory Contract-First Sequence (Required)
1. Amend constitutive suite authority docs (`external_citations`,
   lane/failure semantics continuity).
2. Add/strengthen contract-derived tests for runtime-authority comparison and
   relax-branch assertions.
3. Record pre-implementation contract gate evidence.
4. Publish implementation evidence and disposition.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical process authority remains in `SC-*` contracts.
- Level-4 constitutive authority must not use legacy parity as acceptance
  authority.
- Legacy references may remain as implementation provenance only, not as
  constitutive authority criteria.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth03-level4-constitutive-gate-bootstrap-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth03-level4-constitutive-gate-bootstrap-001/artifacts/claude-code-review-findings.md`

## Intended Write Set
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth05-level4-constitutive-authority-hardening-001/**`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md`
- `docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md`
- `docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md`
- `tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/*.json`
- `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`

## Phase Plan
### Phase A - Scope freeze and AUTH03 findings intake
- Freeze AUTH05 scope from AUTH03 review findings.

### Phase B - Contract authority amendments
- Remove legacy-as-authority citations from Level-4 suite docs.

### Phase C - Contract-derived test hardening
- Add runtime FC/WP model-to-authority checks on real soil fixtures.
- Make relax-to-FC positive-branch assertions mandatory.

### Phase D - Validation and artifact publication
- Run scoped tests/lints and complete AUTH05 evidence/disposition artifacts.

## Exit Criteria
- Level-4 suite docs no longer present legacy baseline as constitutive
  authority citation.
- AUTH03 Level-4 integration target includes runtime FC/WP model-to-authority
  checks on real soil fixtures.
- Relax-to-FC branch assertions are non-optional in fixtures/tests.
- AUTH05 disposition published with `Static`/`Ran` evidence.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: test/spec/documentation hardening only; no new network/auth/runtime
  surface.
