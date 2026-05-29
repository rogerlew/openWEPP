# 20260529-hillstab08-wb16-ealpha-producer-chain-runtime-migration-001

## Status
- state: complete
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: GO

## Objective
Execute immediate next actions from HILLSTAB07 by migrating WB16 runtime
`ealpha` producer-chain behavior into openWEPP runtime surfaces and kernel
ingress lanes with contract-first sequencing, then publish parity evidence for
single-OFE and multi-OFE fixtures.

## Why This Package Exists
HILLSTAB07 closed WB16 contract authority and compatibility-seed observability
gaps, but left full producer migration open (`GAP-RUNOFFPART-005`,
`GAP-WATBAL-005`). This package executes the next closure wave: production
symbol projection + runtime producer logic + parity vectors, without heuristic
substitution.

## Scope
### Included
- Canonical contract amendments for WB16 producer input/state ownership and
  producer-chain runtime requirements.
- Runtime input projection updates for required producer-chain symbols covering
  friction/canopy/residue/rill-geometry lineage needed by WB16 `ealpha`
  production.
- Production producer implementation for `frcfac -> rdat(alpha) -> alphay ->
  eplane` runtime lanes using authoritative equations and typed guards.
- Contract-derived tests asserting runtime-produced `ealpha` behavior for
  representative single-OFE and multi-OFE vectors.
- Validation gates and disposition artifacts.

### Explicitly Out of Scope
- Unrelated hydrology/erosion process migrations outside the WB16 `ealpha`
  producer chain.
- Heuristic/proxy substitutions for missing required producer symbols.
- Silent defaults for domain-invalid producer inputs.

## Deliverables
1. `artifacts/hillstab08-wb16-ealpha-gap-matrix.md`
2. `artifacts/hillstab08-contract-implementation-evidence.md`
3. `artifacts/hillstab08-contract-test-implementation-evidence.md`
4. `artifacts/hillstab08-preimplementation-contract-gate.md`
5. `artifacts/hillstab08-implementation-and-test-evidence.md`
6. `artifacts/hillstab08-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hillstab08_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement canonical contract amendments (`SC-*`) and index updates.
2. Implement contract-derived tests for producer-chain behavior.
3. Record pre-implementation contract gate evidence.
4. Apply production runtime/kernel edits.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority is `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy comparator/provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Every implemented equation/constant/guard must trace to authoritative
  contracts and baseline lineage.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hillstab07-wb16-peak-flow-input-provenance-parity-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hillstab07-wb16-peak-flow-input-provenance-parity-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/frcfac.for`
- `/workdir/wepp-forest_260430_baseline/src/rdat.for`
- `/workdir/wepp-forest_260430_baseline/src/irs.for`
- `/workdir/wepp-forest_260430_baseline/src/eplane.for`
- `/workdir/wepp-forest_260430_baseline/src/grow.for`
- `/workdir/wepp-forest_260430_baseline/src/infile.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hillstab08-wb16-ealpha-producer-chain-runtime-migration-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HILLSTAB07 handoff objectives and freeze scope to WB16 producer
  migration.

### Phase B - Gap inventory and authority map
- Materialize producer-chain gap matrix against baseline lineage.

### Phase C - Contract updates (required gate 1)
- Amend canonical contracts for required producer symbols, guards, and closure
  criteria.

### Phase D - Contract-derived tests (required gate 2)
- Add parity vectors for single-OFE and multi-OFE producer outputs.

### Phase E - Pre-implementation contract gate (required gate 3)
- Record contract/test readiness evidence.

### Phase F - Production implementation (required gate 4)
- Implement runtime projection and producer-chain computation/edit wiring.

### Phase G - Validation gates
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### Phase H - Dual review, dual verification, disposition
- Complete review/verification artifacts.
- Publish explicit GO/HOLD with residual gap ownership.

## Exit Criteria
- WB16 `ealpha` runtime producer chain is implemented for scoped lanes with
  typed guard behavior and no silent fallback.
- Contract-derived vectors for single-OFE and multi-OFE producer behavior pass.
- Required artifacts are complete with truthful evidence labels.
- Disposition explicitly closes or carries forward gap ownership.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test changes only; no auth, secret, or
  network-surface changes.
