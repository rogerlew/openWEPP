# 20260531-auth06-fixture-provenance-hash-enforcement-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Implement deterministic fixture reproducibility controls for active
external-authority suites by enforcing fixture hash locks and provenance
sidecars in release gates.

## Why This Package Exists
AUTH05 closed Level-4 constitutive authority posture, but active suites still
lacked mandatory fixture lock/provenance enforcement. Reproducibility incidents
against wepp-forest workflows require explicit, machine-enforced fixture
integrity with source lineage.

## Scope
### Included
- Update external-authority schema/model/template to require per-fixture hash
  and source provenance metadata.
- Backfill active Level-4 suites with fixture hash/provenance metadata.
- Add fixture root sidecars:
  - `fixtures.sha256`
  - `fixtures.provenance.yaml`
- Enforce fixture integrity in release-gate automation before lane execution.
- Add AUTH06 contract-derived integration tests for schema/registry/gate
  enforcement and tamper detection.
- Publish complete AUTH06 evidence/disposition artifacts.

### Explicitly Out of Scope
- New process-physics implementation work.
- New Level-5/Level-6 suite authoring.
- Non-authority CI refactors outside fixture-integrity enforcement.

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
1. Amend external-authority normative schema/model/template requirements.
2. Add/strengthen contract-derived tests for fixture-integrity enforcement.
3. Record pre-implementation contract gate evidence.
4. Implement release-gate enforcement and fixture sidecars.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without additional user
direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical process authority remains in `SC-*` contracts.
- External-authority fixtures are executable evidence and must be reproducible.
- Legacy parity remains non-authoritative for acceptance.

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
- `/workdir/openWEPP/docs/specifications/external-authority/suite-template.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry-template.yaml`
- `/workdir/openWEPP/docs/governance/openwepp-release-procedure-draft.md`
- `/workdir/openWEPP/tools/release/README.md`
- `/workdir/openWEPP/tools/release/run_release_candidate_gates.sh`
- `/workdir/openWEPP/docs/work-packages/20260531-auth05-level4-constitutive-authority-hardening-001/package.md`

## Intended Write Set
- `Cargo.toml`
- `.gitattributes`
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth06-fixture-provenance-hash-enforcement-001/**`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/external-authority/README.md`
- `docs/specifications/external-authority/suite-schema.md`
- `docs/specifications/external-authority/suite-template.md`
- `docs/specifications/external-authority/registry-template.yaml`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_soil_fc_minus33_001.md`
- `docs/specifications/external-authority/suites/cas_l4_soil_wp_minus1500_001.md`
- `docs/specifications/external-authority/suites/cas_l4_watbal_relax_to_fc_001.md`
- `docs/governance/openwepp-release-procedure-draft.md`
- `tools/release/README.md`
- `tools/release/run_release_candidate_gates.sh`
- `tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/fixtures.sha256`
- `tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/fixtures.provenance.yaml`
- `tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/fixtures.sha256`
- `tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/fixtures.provenance.yaml`
- `tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/fixtures.sha256`
- `tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/fixtures.provenance.yaml`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`

## Phase Plan
### Phase A - Scope freeze and reproducibility objective ratification
- Freeze AUTH06 scope around fixture-integrity enforcement and backfill.

### Phase B - Contract/schema amendments
- Make fixture hash + source provenance required in canonical docs/templates.

### Phase C - Contract-derived test implementation
- Add AUTH06 integration tests for schema/registry sidecar/tamper enforcement.

### Phase D - Release gate and fixture sidecar implementation
- Implement blocking fixture-integrity checks in release automation.
- Backfill active Level-4 suite fixtures with lock/provenance files.

### Phase E - Validation and disposition publication
- Run scoped tests/lints and publish complete AUTH06 evidence artifacts.

## Exit Criteria
- External-authority schema requires fixture hash and source provenance fields.
- Active Level-4 suites publish lock/provenance sidecars and suite fixture
  metadata includes hashes/provenance.
- Release gate blocks on missing/mismatched fixture locks/provenance.
- AUTH06 contract-derived tests pass, including tamper detection.
- AUTH06 disposition is published with explicit `Static`/`Ran` evidence.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: fixture-integrity enforcement and documentation/test updates only;
  no new auth/network/runtime execution surfaces.
