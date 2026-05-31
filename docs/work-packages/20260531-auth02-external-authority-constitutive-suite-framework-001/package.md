# 20260531-auth02-external-authority-constitutive-suite-framework-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Define and scaffold the normative structure for external-authority constitutive
suites (layout, schema, fixture conventions, naming rules, and contract linkage)
so follow-on physics gates can be implemented deterministically.

## Why This Package Exists
AUTH01 defines the correctness-authority model. AUTH02 operationalizes it by
creating repository-local suite structure and standards that package execution
and CI can enforce.

## Scope
### Included
- Create canonical external-authority suite docs and templates.
- Define suite-ID naming, metadata schema, fixture conventions, and citation
  requirements.
- Define suite classes (`required`, `periodic`, `manual`) and failure classes
  (`hard-fail`, `investigation`) for CI wiring.
- Add governance references from science-contract index/profile docs.

### Explicitly Out of Scope
- Production constitutive equation changes.
- Release workflow/job implementation.
- Numerical-threshold tuning in kernel code.

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
1. Amend canonical authority/governance docs to require constitutive suites.
2. Publish suite schema and contract-derived suite obligations.
3. Record pre-implementation contract gate evidence.
4. Prepare AUTH03-ready implementation scaffolding constraints.

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
- `/workdir/openWEPP/docs/work-packages/20260531-auth01-correctness-authority-model-formalization-001/package.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth02-external-authority-constitutive-suite-framework-001/**`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/external-authority/README.md`
- `docs/specifications/external-authority/suite-schema.md`
- `docs/specifications/external-authority/suite-template.md`
- `docs/specifications/external-authority/registry-template.yaml`
- `tests/fixtures/constitutive/README.md`

## Phase Plan
### Phase A - Scope freeze and AUTH01 intake
- Confirm queue authorization and freeze AUTH02 boundaries.

### Phase B - Suite framework authoring
- Publish constitutive-suite structure, naming, and schema authority.

### Phase C - Contract linkage and template publication
- Bind suite schema to `SC-*` invariant references and publish authoring
  templates for AUTH03+ packages.

### Phase D - Validation and disposition
- Run docs lint/check commands as available.
- Publish HOLD/GO disposition and AUTH03 handoff constraints.

## Exit Criteria
- Canonical external-authority suite framework exists and is indexed.
- Suite schema/naming/template requirements are normative and reusable.
- Fixture location and citation requirements are explicit for implementation
  packages.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: documentation/governance updates only; no runtime/network/auth
  surface changes.
