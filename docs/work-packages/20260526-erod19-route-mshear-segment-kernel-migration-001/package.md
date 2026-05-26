# 20260526-erod19-route-mshear-segment-kernel-migration-001

## Status
- state: package-complete-with-hold
- date: 2026-05-26
- timezone: UTC
- decision: HOLD

## Objective
Execute EROD19 by migrating baseline-authoritative hillslope `route.for`
segment-branch behavior into openWEPP closure diagnostics routing surfaces:
upper-boundary `dl` branch (`abs(qostar) < .0011`), `xcrit`-driven
`mshear (1..5)` dispatch classification, and `depc/depend`-driven `xdend`
publication with typed hard-fail guards.

## Why This Package Exists
EROD18 closed route topology ingress/publication seams, but branch-family
algorithm behavior remained unresolved (`GAP-SED-005`). EROD19 is the required
runtime migration step before EROD20 magic-number cleanup and EROD21 parity
closeout.

## Scope
### Included
- Production kernel migration in:
  - `crates/openwepp-hillslope-orchestrator/src/constants.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- Runner provenance status continuity update in:
  - `crates/openwepp-runner/src/hillslope/mod.rs`
- Route contract-vector updates in:
  - `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- Governance artifacts and EROD20 handoff evidence.

### Explicitly Out of Scope
- Sediment-routing magic-number elimination (`EROD20` scope).
- Route parity rerun and hold-lift disposition (`EROD21` scope).

## Deliverables
1. `artifacts/erod19-route-topology-ingress-matrix.md`
2. `artifacts/erod19-route-topology-implementation-report.md`
3. `artifacts/erod19-contract-implementation-evidence.md`
4. `artifacts/erod19-contract-test-implementation-evidence.md`
5. `artifacts/erod19-preimplementation-contract-gate.md`
6. `artifacts/erod19-implementation-and-test-evidence.md`
7. `artifacts/erod19-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/erod19_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Canonical contract amendments (`EROD16`) complete.
2. Contract-derived vectors (`EROD17`) complete.
3. Pre-implementation contract gate (`EROD17`) complete.
4. Production runtime migration (`EROD19`) execute.

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
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260526-erod19-route-mshear-segment-kernel-migration-001/**`

## Exit Criteria
- Route seam publishes branch-family symbols with baseline-derived behavior.
- EROD17 route vectors execute as active tests (no `#[ignore]`) and pass.
- MOFE03 runner Wave-2 continuity test still passes with route migration active.
- Package artifacts are complete with truthful `Static`/`Ran` labeling.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local runtime math and tests only; no external interface changes.

## Execution Outcome Summary
- Implemented EROD19 route migration helpers in kernel support:
  `shear`, quadratic roots, `xcrit` branch classification, `depc`, and
  `depend`-style `xdend` solver.
- Replaced EROD18 placeholder route topology publication path with EROD19
  branch migration behavior and success status `HKERNEL-EROD19-ROUTE-OK-001`.
- Updated runner provenance detection to accept `EROD19-ROUTE` status.
- Activated EROD17 route vectors (removed `#[ignore]`) and tuned vector inputs
  to exercise migrated branch behavior under current guard posture.
- HOLD remains until EROD20/EROD21 closure stages complete.
