# 20260531-auth01-correctness-authority-model-formalization-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Formalize repository-level correctness authority ranking and adjudication rules
for kernel acceptance, including explicit treatment of external constitutive
authority suites and legacy-comparator demotion.

## Why This Package Exists
Current governance is strong at `SC-*` contract authority but does not yet
publish a single canonical correctness-authority model that defines how
external constitutive suites are authored, ranked, and used for disposition.

## Scope
### Included
- Author canonical correctness-authority model documentation.
- Define external-authority suite taxonomy, authority ranking, and required
  metadata fields.
- Amend science-contract index/governance docs to point to the new authority
  model.
- Record migration/retirement criteria for legacy parity as investigation-only.

### Explicitly Out of Scope
- Production kernel code edits.
- CI/workflow wiring changes.
- New constitutive test implementation.

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
1. Amend canonical authority/governance documentation (`SC-*` adjacent docs).
2. Define contract-derived suite requirements and acceptance obligations.
3. Record pre-implementation contract gate evidence.
4. Land follow-on planning constraints for implementation packages.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical process authority remains in `SC-*` contracts.
- External constitutive suite authority must cite primary references with
  explicit version/commit provenance.
- Legacy comparator parity remains investigation signal, not acceptance oracle.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth01-correctness-authority-model-formalization-001/**`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/external-authority/README.md`

## Phase Plan
### Phase A - Scope freeze and authority intake
- Confirm queue authorization and freeze AUTH01 boundaries.

### Phase B - Canonical authority-model authoring
- Publish correctness-authority ranking, adjudication, and legacy-demotion
  policy in canonical docs.

### Phase C - Contract-derived suite requirement authoring
- Define required external-suite schema, metadata, tolerances, and gate classes
  for follow-on implementation packages.

### Phase D - Validation and disposition
- Run docs lint/check commands as available.
- Publish HOLD/GO disposition and AUTH02 handoff constraints.

## Exit Criteria
- Canonical correctness-authority model exists and is indexed.
- External-authority suite structure and minimum schema are normative.
- Legacy parity demotion policy is explicit and testable by workflow policy.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: documentation/governance updates only; no runtime/network/auth
  surface changes.
