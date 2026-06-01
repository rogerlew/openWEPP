# 20260601-hphys0224-cam-wb19-soilwater-authority-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Resume HPHYS remediation under the Correctness Authority Model by closing
remaining WB19/soil-water authority and gate gaps for open residual families
(`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`) using
contract-first sequencing and post-change cohort readjudication.

## Why This Package Exists
HPHYS0223 closed the required post-HPHYS0222 measurement rerun and confirmed no
residual-family movement. The unresolved families remain open and require
process-authority remediation under the now-formalized correctness authority
stack (`A0/A1/A3` mandatory, legacy as investigation signal only).

## Scope
### Included
- Canonical `SC-*` authority amendments for unresolved WB19/soil-water laws.
- Contract-derived test and external-authority suite updates required to align
  with the Correctness Authority Model.
- Targeted production remediation for contract-defined symbol paths.
- Full `unpalatable-rind` 39-hillslope rerun and residual readjudication
  versus HPHYS0223.

### Explicitly Out of Scope
- Watershed channel routing or sediment migration scopes.
- Unrelated parser/input-file producer-contract remediations.
- Legacy-only parity tuning without `SC-*` and external-authority basis.

## Closure Measures (Required)
1. `MEASURE-HP224-001`: required canonical contract authority amendments for
   touched laws are implemented in `SC-*` contracts and indexed.
2. `MEASURE-HP224-002`: required A3 constitutive suite coverage for touched
   process families is implemented/updated and registered with lane/failure
   posture aligned to the Correctness Authority Model.
3. `MEASURE-HP224-003`: contract-derived tests and pre-implementation gate
   evidence are recorded before production edits.
4. `MEASURE-HP224-004`: production remediation is implemented with typed
   guards; no silent defaults or heuristic/proxy substitutions.
5. `MEASURE-HP224-005`: 39-hillslope rerun evidence and residual deltas versus
   HPHYS0223 are published with explicit HOLD/GO adjudication.
6. `MEASURE-HP224-006`: required workspace gates pass and are truthfully
   recorded (`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`).

## Deliverables
1. `artifacts/hphys0224-residual-authority-gap-matrix.md`
2. `artifacts/hphys0224-contract-implementation-evidence.md`
3. `artifacts/hphys0224-contract-test-implementation-evidence.md`
4. `artifacts/hphys0224-preimplementation-contract-gate.md`
5. `artifacts/hphys0224-implementation-and-test-evidence.md`
6. `artifacts/hphys0224-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0224_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract amendments in `SC-*` authority.
2. Implement contract-derived tests and external-authority suite updates.
3. Record pre-implementation contract gate evidence.
4. Apply scoped production edits and run rerun/readjudication evidence.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority is in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Correctness authority ranking and gate posture are governed by
  `docs/specifications/correctness-authority-model.md`.
- External-authority suite governance is in
  `docs/specifications/external-authority/`.
- Legacy comparator outputs are investigation signal only.
- Legacy migration provenance defaults to
  `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/promotion-protocol.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/specifications/external-authority/required-suite-obligations.json`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0223-post-0222-cohort-rerun-readjudication-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0223-post-0222-cohort-rerun-readjudication-001/artifacts/hphys0223_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0223-post-0222-cohort-rerun-readjudication-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth08a-solwpv-branch-gate-authority-retiering-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/wc1/runs/un/unpalatable-rind`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0224-cam-wb19-soilwater-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/required-suite-obligations.json`
- `docs/specifications/external-authority/suites/*.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_soil_slope.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/**/*hphys0224*.rs`
- `tests/integration/**/*auth*.rs`
- `tools/legacy_comparison_suite/**`

## Phase Plan
### Phase A - Intake freeze and authority backlog
- Freeze unresolved residual families and symbol-path ownership from HPHYS0223.
- Map open gaps to `A0/A1/A3` obligations and touched contract invariants.

### Phase B - Contract and suite authority updates
- Amend canonical `SC-*` authority for touched WB19/soil-water laws.
- Implement external-authority suite/schema/registry amendments required by
  the Correctness Authority Model.

### Phase C - Contract-derived tests and pre-implementation gate
- Implement/adjust contract-derived tests for updated authority surfaces.
- Record pre-implementation gate evidence before any production edits.

### Phase D - Production remediation
- Apply scoped WB19/soil-water runtime corrections derived from contract
  authority and test vectors.
- Preserve typed fail-closed guards and invariant reporting.

### Phase E - Validation and readjudication rerun
- Run required workspace gates.
- Execute `unpalatable-rind` 39-hillslope rerun and semantic comparison.
- Publish residual-family delta matrix versus HPHYS0223 baseline.

### Phase F - Dual review, dual verification, disposition
- Complete dual review and dual verification artifacts.
- Publish explicit HOLD/GO disposition and immediate next-action handoff.

## Exit Criteria
- Closure measures `MEASURE-HP224-001..006` are satisfied and evidenced.
- If residual families remain open, disposition stays HOLD with a scoped
  follow-on package queue and explicit symbol-path ownership.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/doc updates only; no credentials/network changes.
