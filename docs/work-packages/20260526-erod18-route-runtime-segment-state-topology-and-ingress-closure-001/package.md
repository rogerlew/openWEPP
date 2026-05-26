# 20260526-erod18-route-runtime-segment-state-topology-and-ingress-closure-001

## Status
- state: package-complete-with-hold
- date: 2026-05-26
- timezone: UTC
- decision: HOLD

## Objective
Execute EROD18 by implementing typed runtime segment-state topology and ingress
closure for hillslope route migration scope (`nslpts`, `xu/xl`,
`ainf/binf/cinf`, `ainftc/binftc/cinftc`, `qostar`, `xdetst`, `ldlast/lddend`,
`xdbeg/xdend`, `du/dl`, `mshear`, `xc1/xc2`, `ndep`) with typed hard-fail
guards and no silent ingress defaults.

## Why This Package Exists
ROUTEPLAN01 and EROD17 established that route-branch contract vectors were
blocked by missing route segment-state publication surfaces. EROD18 closes the
state-topology seam so EROD19 can focus on baseline-authoritative
`route.for` branch-family migration.

## Scope
### Included
- Production kernel/runtime topology seam updates in:
  - `crates/openwepp-hillslope-orchestrator/src/constants.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- Runner ingress projection updates in:
  - `crates/openwepp-runner/src/hillslope/mod.rs`
- Integration test updates in:
  - `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- Governance artifacts and EROD19 handoff evidence.

### Explicitly Out of Scope
- Full baseline `route.for` branch algorithm migration (`EROD19` scope).
- Sediment-routing magic-number elimination (`EROD20` scope).
- Route parity rerun/hold-lift disposition (`EROD21` scope).

## Deliverables
1. `artifacts/erod18-route-topology-ingress-matrix.md`
2. `artifacts/erod18-route-topology-implementation-report.md`
3. `artifacts/erod18-contract-implementation-evidence.md`
4. `artifacts/erod18-contract-test-implementation-evidence.md`
5. `artifacts/erod18-preimplementation-contract-gate.md`
6. `artifacts/erod18-implementation-and-test-evidence.md`
7. `artifacts/erod18-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/erod18_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For route code-authoring scope:
1. canonical contract amendments (`EROD16`) complete,
2. contract-derived vectors (`EROD17`) complete,
3. pre-implementation contract gate (`EROD17`) complete,
4. production/runtime topology edits (`EROD18`) execute.

## Autonomous Execution Intent (Required)
This package is execution-ready and complete for end-to-end autonomous
execution through disposition without additional user intervention unless
hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts include explicit `Static:` and/or `Ran:` labels.

## Provenance and Authority Posture
- Canonical authority remains in `SC-*` contracts.
- Baseline provenance anchor:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No surrogate route-physics closure claim; EROD19 remains required.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-routeplan01-hillslope-sediment-routing-assessment-and-queue-001/artifacts/sediment-routing-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260526-erod16-route-branch-contract-authority-and-routine-map-001/artifacts/erod16_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260526-erod17-route-branch-contract-derived-tests-and-preimplementation-gate-001/artifacts/erod17_disposition.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260526-erod18-route-runtime-segment-state-topology-and-ingress-closure-001/**`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm EROD16/EROD17 preconditions and EROD18-only closure targets.

### Phase B - Runtime topology and typed guards
- Implement route segment topology symbol family and EROD18 guard-code family.

### Phase C - Ingress projection and contract tests
- Add runner ingress projection for route-topology symbols.
- Add/adjust integration tests for EROD18 guards and topology publication seam.

### Phase D - Governance closeout
- Record gate/test evidence and package disposition with EROD19 handoff.

## Exit Criteria
- Required route segment topology symbol family is available in runtime output.
- Typed EROD18 guard failures exist for missing/non-finite/domain-invalid route
  topology ingress (`HKERNEL-EROD18-ROUTE-E-001..003`).
- Runner ingress path carries required topology symbols for Wave-2 enabled runs.
- EROD18 seam coverage is enforced by targeted integration tests.
- Package artifacts are complete with truthful `Static`/`Ran` labeling.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: runtime math/topology and test changes only; no external interfaces.

## Execution Outcome Summary
- Added EROD18 guard-code family and route-topology publication seam.
- Added runner ingress projection for route-topology symbols under Wave-2
  activation path.
- Added EROD18 integration tests for missing/non-finite/domain guard behavior
  and enabled route-topology seam publication vector.
- Maintained HOLD posture because full `route.for` branch-family migration
  remains open (`EROD19`).
