# 20260525-mofe02-cross-file-ofe-parity-hard-gate-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC

## Objective
Implement a hard cross-file OFE parity gate for hillslope execution intake so
MOFE runs enforce:
- `slope.ofe_count == management.topology_count == soil.ntemp`
before runtime surface merge and scheduler execution.

## Why This Package Exists
MOFE01 confirmed canonical contract authority for cross-file OFE parity but
identified a blocking runtime gap: production hillslope intake currently parses
and merges slope/management/soil surfaces without enforcing triad parity.

This package closes that production gap with contract-first sequencing,
contract-derived tests, typed hard-fail behavior, and no silent defaults.

## Scope
### Included
- Contract-authority audit and amendments (only if needed) for explicit runner
  intake parity semantics and error ownership.
- Contract-derived tests for all mismatch classes:
  - slope vs management,
  - slope vs soil,
  - management vs soil,
  - full triad mismatch.
- Production hillslope intake implementation:
  - wire soil parser `expected_topology_count` for hillslope scope,
  - add explicit triad parity validator,
  - emit typed hard failures on mismatch.
- Validation gates, governance artifacts, and disposition.

### Explicitly Out of Scope
- EROD14 Wave-2 routing symbol synthesis/activation (`MOFE03`).
- MOFE publication/output closure (`MOFE04`).
- Watershed contributor MOFE metadata closure (`MOFE05`).
- Non-MOFE runner/orchestrator refactors.

## Deliverables
1. Cross-file OFE parity implementation report:
   - `artifacts/mofe02-cross-file-parity-implementation-report.md`
2. Contract-derived parity test matrix:
   - `artifacts/mofe02-cross-file-parity-test-matrix.md`
3. Contract implementation evidence:
   - `artifacts/mofe02-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/mofe02-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/mofe02-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/mofe02-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/mofe02-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe02_disposition.md`
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
1. Amend canonical contracts as needed for intake parity authority.
2. Implement contract-derived tests for parity mismatch classes.
3. Record pre-implementation contract gate evidence.
4. Modify production runner/orchestrator intake code.

No kernel/runtime behavior edits are permitted before steps 1-3 are complete.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-assessment-report.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe01_disposition.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/soil.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/tests/integration/`

## Intended Write Set
- `docs/work-packages/20260525-mofe02-cross-file-ofe-parity-hard-gate-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md` (if required)
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-runner/src/errors.rs` (if required)
- `tests/integration/**` (contract-derived MOFE02 tests)

## Phase Plan
### Phase A - Intake and Contract Authority Alignment
- Confirm MOFE02 entry signal from MOFE01 artifacts.
- Confirm canonical parity invariants and authority owners.
- Amend canonical contracts only if implementation ownership/guard IDs are
  insufficiently explicit.

### Phase B - Contract-Derived Test Implementation
- Add/adjust tests that assert hard-fail mismatch behavior for all cross-file
  OFE parity mismatch classes.
- Ensure tests fail before production code changes (where practical).

### Phase C - Pre-Implementation Contract Gate
- Capture contract-gate evidence proving sequence integrity and
  implementation-ready authority/test baseline.

### Phase D - Production Intake Implementation
- Wire soil parser `expected_topology_count` in hillslope runner intake.
- Add explicit triad OFE parity validator for slope/management/soil counts.
- Ensure mismatch paths return typed hard-fail errors (no silent fallback).

### Phase E - Validation, Review, and Disposition
- Run required gates.
- Complete dual review and dual verification artifacts.
- Publish final `GO`/`HOLD` disposition and worker handoff.

## Exit Criteria
- Cross-file OFE parity is enforced before runtime surface merge.
- Parity mismatch classes are covered by contract-derived tests.
- No silent defaults or clamping on domain mismatch conditions.
- Required artifacts and gate evidence are complete with truthful labels.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: intake validation and typed error-path hardening only.
