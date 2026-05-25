# 20260525-mofe03-wave2-routing-activation-and-input-synthesis-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC

## Objective
Implement MOFE03 by making EROD14 Wave-2 routing executable from production
hillslope runfile intake without manual test-only symbol injection.

## Why This Package Exists
MOFE01 identified that Wave-2 runtime logic exists with typed guards but is not
production-reachable because hillslope runner intake does not seed
`erod14_wave2_enabled` or required `erod14_*` producer symbols.

MOFE02 closed cross-file OFE parity gating and is a prerequisite for MOFE03
activation policy based on aligned multi-OFE topology.

## Scope
### Included
- Contract-authority amendments (only where needed) to define production
  activation/seeding policy for `erod14_wave2_enabled` and required Wave-2
  producer symbols.
- Contract-derived tests covering:
  - multi-OFE runfile execution reaches Wave-2 path without manual symbol
    injection,
  - single-OFE policy keeps Wave-2 disabled,
  - typed guard-family continuity remains explicit.
- Production runner/runtime seeding updates to derive/seed required
  `erod14_*` symbols from parsed/runtime surfaces under explicit policy.
- Validation gates, governance artifacts, and disposition.

### Explicitly Out of Scope
- MOFE publication/output closure (`MOFE04`).
- Watershed contributor MOFE metadata closure (`MOFE05`).
- New erosion physics equations beyond existing canonical EROD14 authority.

## Deliverables
1. Wave-2 activation implementation report:
   - `artifacts/mofe03-wave2-activation-implementation-report.md`
2. Contract-derived Wave-2 test matrix:
   - `artifacts/mofe03-wave2-test-matrix.md`
3. Contract implementation evidence:
   - `artifacts/mofe03-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/mofe03-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/mofe03-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/mofe03-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/mofe03-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe03_disposition.md`
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
1. Amend canonical contracts as needed for MOFE03 activation/seeding authority.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production runner/runtime code.

No production behavior edits are permitted before steps 1-3 are complete.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-assessment-report.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/artifacts/mofe-readiness-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe02-cross-file-ofe-parity-hard-gate-001/artifacts/mofe02_disposition.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/openWEPP/tests/integration/cli03_runner_contract_derived_tests.rs`

## Intended Write Set
- `docs/work-packages/20260525-mofe03-wave2-routing-activation-and-input-synthesis-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (if required)
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- `tests/integration/erod14_contract_authority_closure_contract.rs` (if required)

## Phase Plan
### Phase A - Intake and Contract Authority Alignment
- Confirm MOFE03 entry signal from MOFE01 queue and MOFE02 disposition.
- Confirm canonical authority for Wave-2 activation and required producer
  symbol ownership.
- Amend contracts only if activation/seeding policy is not explicit.

### Phase B - Contract-Derived Test Implementation
- Add/adjust tests to prove production runfile execution reaches Wave-2 without
  manual symbol injection for multi-OFE aligned inputs.
- Add/adjust tests to prove single-OFE policy disables Wave-2.

### Phase C - Pre-Implementation Contract Gate
- Capture contract-gate evidence proving sequence integrity and
  implementation-ready authority/test baseline.

### Phase D - Production Activation and Symbol Seeding Implementation
- Implement explicit `erod14_wave2_enabled` production policy from runtime
  topology surfaces.
- Derive/seed required `erod14_*` symbols from parsed/runtime surfaces with
  typed hard-fail behavior on invalid domains.

### Phase E - Validation, Review, and Disposition
- Run required gates.
- Complete dual review and dual verification artifacts.
- Publish final `GO`/`HOLD` disposition and worker handoff.

## Exit Criteria
- Multi-OFE runfile execution reaches Wave-2 without manual symbol injection.
- Single-OFE policy deterministically disables Wave-2.
- Guard-family behavior remains typed and explicit on domain violations.
- No silent defaults or clamping are introduced for domain violations.
- Required artifacts and gate evidence are complete with truthful labels.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: runtime surface activation/seeding and typed guard-path closure.
