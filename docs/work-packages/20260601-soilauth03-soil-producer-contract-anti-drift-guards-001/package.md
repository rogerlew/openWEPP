# 20260601-soilauth03-soil-producer-contract-anti-drift-guards-001

## Status
- state: queued
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute SOILAUTH03 to implement machine-checkable anti-drift guards that block
future divergence between:
1. openWEPP `.sol` producer contract text,
2. openWEPP parser/runtime expectations, and
3. canonical `wepppy` producer behavior.

## Why This Package Exists
SOILAUTH02 can close current gaps, but without guardrails the same contract
drift can recur. SOILAUTH03 creates repeatable release-time enforcement:
required symbol/arity obligations, fixture provenance/hash checks, and
drift-detection tests with hard-fail behavior for required lanes.

## Scope
### Included
- Encode machine-readable `.sol` datver obligations for required fields/order.
- Add enforcement tests/guards that fail on:
  - missing required symbols,
  - row-arity/order drift,
  - fixture-provenance/hash drift.
- Wire guards into openWEPP release-gate lanes and document triage protocol.

### Explicitly Out of Scope
- New physics/process-branch changes.
- Non-soil input-file families.
- Manual-only review controls (this package is machine-guard focused).

## Closure Measures (Required)
1. `MEASURE-SA03-001`: required `.sol` obligation guard fails on injected
   symbol/arity drift and passes on current authoritative fixtures.
2. `MEASURE-SA03-002`: fixture provenance/hash guard fails on tampered fixture
   state and passes on locked state.
3. `MEASURE-SA03-003`: release-gate documentation and lane configuration include
   explicit hard-fail policy for required soil producer-contract checks.

## Deliverables
1. `artifacts/soilauth03-guard-obligation-map.md`
2. `artifacts/soilauth03-contract-implementation-evidence.md`
3. `artifacts/soilauth03-contract-test-implementation-evidence.md`
4. `artifacts/soilauth03-preimplementation-contract-gate.md`
5. `artifacts/soilauth03-implementation-and-test-evidence.md`
6. `artifacts/soilauth03-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/soilauth03_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Ratify machine-readable obligation authority surfaces.
2. Implement contract-derived guard tests and fixture-integrity checks.
3. Record pre-implementation gate evidence before guard wiring changes.
4. Implement release-gate integration and update runbook/triage docs.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Producer-contract authority for `.sol`:
  `docs/specifications/wepp-input-files/specs/soil-file.spec.md`.
- Parser/runtime acceptance authority:
  `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`.
- Canonical producer reference:
  `/workdir/wepppy/wepppy/wepp/soils/utils/wepp_soil_util.py`,
  `/workdir/wepppy/wepppy/soils/ssurgo/ssurgo.py`,
  `/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-soilauth02-soil-producer-contract-correctness-reconciliation-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-soilauth02-soil-producer-contract-correctness-reconciliation-001/artifacts/soilauth02-reconciliation-gap-ledger.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/external-authority/required-suite-obligations.json`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/tests/fixtures/constitutive/`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-soilauth03-soil-producer-contract-anti-drift-guards-001/**`
- `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/required-suite-obligations.json`
- `docs/specifications/external-authority/registry.yaml`
- `tests/integration/soilauth03_soil_contract_drift_guards_contract.rs`
- `tools/legacy_comparison_suite/**` (if guard scripts are housed there)
- `scripts/release/**` (if guard wiring requires release scripts)

## Phase Plan
### Phase A - Intake and scope freeze
- Freeze guard requirements from SOILAUTH02 residual-risk handoff.

### Phase B - Contract/spec authority updates
- Ratify machine-readable obligation schema and required symbol sets.

### Phase C - Contract-derived tests
- Implement failing drift tests for symbol/arity/order and fixture hash drift.

### Phase D - Pre-implementation contract gate
- Record red-state contract/test gate evidence.

### Phase E - Production implementation
- Implement guard wiring and release-lane integration.

### Phase F - Validation and parity rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Run injected-drift negative tests and locked-fixture positive tests.

### Phase G - Dual review, dual verification, disposition
- Complete dual review/verification and publish anti-drift closure disposition.

## Exit Criteria
- Closure measures `MEASURE-SA03-001..003` are satisfied and evidenced.
- Required soil producer-contract drift checks are release-gated with
  explicit hard-fail behavior.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: guard/test/release-script changes only; no credential surface
  modifications.
