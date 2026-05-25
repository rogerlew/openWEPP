# 20260525-mofe05-watershed-contributor-metadata-and-intake-validation-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC

## Objective
Implement MOFE05 by adding watershed contributor MOFE metadata intake surfaces
and typed validation guards so malformed contributor metadata hard-fails at the
watershed boundary.

## Why This Package Exists
MOFE01 queued MOFE05 as the watershed-side metadata and intake validation wave.
MOFE03 and MOFE04 closed hillslope-side routing activation and publication
policy provenance; watershed intake still lacked contributor MOFE metadata
validation.

## Scope
### Included
- Contract-authority amendments (only where required) for watershed contributor
  MOFE metadata intake/validation semantics.
- Contract-derived tests covering malformed/missing/mismatched contributor
  metadata behavior.
- Watershed CLI runfile/intake implementation updates for contributor metadata
  surfaces and typed hard-fail validation.
- Validation gates, governance artifacts, and disposition.

### Explicitly Out of Scope
- New watershed routing process-physics equations.
- Broad redesign of watershed output serialization.
- Non-MOFE sidecar policy redesign.

## Deliverables
1. Watershed contributor metadata implementation report:
   - `artifacts/mofe05-watershed-contributor-metadata-implementation-report.md`
2. Contract-derived MOFE05 test matrix:
   - `artifacts/mofe05-watershed-contributor-metadata-test-matrix.md`
3. Contract implementation evidence:
   - `artifacts/mofe05-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/mofe05-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/mofe05-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/mofe05-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/mofe05-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe05_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
    - `artifacts/verification_agent_a.md`
    - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end without user
intervention unless hard-blocked by contradictory canonical requirements or
unresolvable environment failures.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Contract-First Sequence (Required)
1. Amend canonical contracts as needed for MOFE05 watershed metadata authority.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production watershed intake code.

No production watershed-intake behavior edits are permitted before steps 1-3 are
complete.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-assessment-report.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe04-output-publication-closure-001/artifacts/mofe04_disposition.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `/workdir/openWEPP/crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `/workdir/openWEPP/docs/contracts/openwepp-watershed-runfile-contract.md`

## Intended Write Set
- `docs/work-packages/20260525-mofe05-watershed-contributor-metadata-and-intake-validation-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (if required)
- `docs/contracts/openwepp-watershed-runfile-contract.md` (if required)
- `Cargo.toml` (if required for new integration contract target)
- `tests/integration/**` (MOFE05 contract-authority closure tests if required)
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

## Phase Plan
### Phase A - Intake and Contract Authority Alignment
- Confirm MOFE05 entry signal from MOFE01 queue and MOFE04 disposition.
- Confirm canonical authority for watershed contributor metadata intake policy.
- Amend contracts only if contributor metadata/validation authority is not
  explicit.

### Phase B - Contract-Derived Test Implementation
- Add/adjust tests proving malformed contributor metadata hard-fails with typed
  CLI guard codes and valid metadata passes intake gate.

### Phase C - Pre-Implementation Contract Gate
- Capture contract-gate evidence proving sequence integrity and
  implementation-ready authority/test baseline.

### Phase D - Production Watershed Intake Implementation
- Add contributor metadata intake surfaces for watershed `hillslopes_block`
  records.
- Implement typed validation for missing/malformed/mismatched MOFE metadata
  where required.

### Phase E - Validation, Review, and Disposition
- Run required gates.
- Complete dual review and dual verification artifacts.
- Publish final `GO`/`HOLD` disposition and worker handoff.

## Exit Criteria
- Watershed contributor metadata contract surfaces are explicit and validated.
- Malformed contributor metadata hard-fails with typed guard codes.
- Contract-derived tests cover acceptance and rejection vectors.
- Required artifacts and gate evidence are complete with truthful labels.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: validation-surface hardening and typed error closure only.
