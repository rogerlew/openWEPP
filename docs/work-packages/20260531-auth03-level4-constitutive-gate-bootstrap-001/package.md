# 20260531-auth03-level4-constitutive-gate-bootstrap-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Implement the first external-authority Level-4 constitutive suites and
contract-derived tests for adjudicating soil-water over-drainage risks
independent of legacy parity.

## Why This Package Exists
AUTH02 defines framework and schema. AUTH03 instantiates the first executable
constitutive suites so correctness can be gated by physics laws, not parity
targets.

## Scope
### Included
- Amend canonical `SC-*` invariants and vectors for Level-4 constitutive gates.
- Implement constitutive fixtures and integration tests for:
  - FC at `-33 kPa`,
  - WP at `-1500 kPa`,
  - relax-to-FC and `Dp -> 0` near FC cutoff behavior.
- Add typed guard assertions for missing/invalid constitutive symbols.
- Record adjudication outputs and follow-on remediation queue.

### Explicitly Out of Scope
- Full CI workflow lane wiring (AUTH04 scope).
- Plot/lysimeter Level-5 validation.
- Independent-solver Level-6 validation.

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
1. Amend canonical `SC-*` constitutive authority and invariants.
2. Implement contract-derived Level-4 suite tests and fixtures.
3. Record pre-implementation contract gate evidence.
4. Modify production kernel paths only after gate prerequisites exist.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth02-external-authority-constitutive-suite-framework-001/package.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth03-level4-constitutive-gate-bootstrap-001/**`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/external-authority/registry.yaml`
- `tests/fixtures/constitutive/**`
- `tests/integration/auth03_level4_constitutive_gate_contract.rs`
- `Cargo.toml`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`

## Phase Plan
### Phase A - Scope freeze and AUTH02 intake
- Confirm queue authorization and freeze AUTH03 boundaries.

### Phase B - Contract authority amendments
- Publish Level-4 constitutive invariants and authoritative vectors.

### Phase C - Level-4 suite implementation
- Implement fixture-backed constitutive tests and typed guard checks.

### Phase D - Production adjustments (if required)
- Apply bounded kernel changes only after contract-gate evidence is recorded.

### Phase E - Validation and disposition
- Run workspace gates (`fmt`, `clippy`, `test`, `deny`) and publish disposition
  with residual/follow-on ownership.

## Exit Criteria
- Level-4 constitutive suites are executable and contract-linked.
- Hard-fail guard behavior is covered for invalid constitutive inputs.
- Residual adjudication no longer depends on parity-only acceptance logic.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: documentation/governance updates only; no runtime/network/auth
  surface changes.
