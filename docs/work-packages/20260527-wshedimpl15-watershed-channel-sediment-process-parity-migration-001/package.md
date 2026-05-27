# 20260527-wshedimpl15-watershed-channel-sediment-process-parity-migration-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL15 by implementing required runtime channel-sediment control
projection and WS10 kernel scaffold surfaces for baseline `chnero/chnrt/detach`
migration while preserving fail-closed typed guard posture.

## Why This Package Exists
WSHEDIMPL14 closed the watershed end-to-end comparator-lane blocker and left
`GAP-SYSTEM-008` / `GAP-ROUTE-009` / `GAP-SED-006` as the remaining watershed
program blocker: full channel sediment process parity. WSHEDIMPL06 closed
publication-family symbols (`qsed`, `tc`), but the runtime still lacked
channel-sediment control projection and baseline variable scaffolding needed
for full routine migration.

## Scope
### Included
- Parser-to-runtime projection of channel sediment control symbols from
  `WatershedChannelFile` into watershed runtime state surface:
  - `ws10_channel_{id}_{ishape,ienslp,chnz,chnnbr,chntcr,chnedm,chneds,ctlz,ctln}`.
- WS10 channel kernel typed guard enforcement requiring those projected symbols
  on execution path.
- WS10 channel writeback publication of baseline-authoritative conversion
  scaffolds used by `chnero/chnrt/dcap/detach` lineage:
  - `ws10_channel_{id}_crsh` (`chntcr * 0.021`)
  - `ws10_channel_{id}_depmid` (`chnedm * 3.281`)
  - `ws10_channel_{id}_depsid` (`chneds * 3.281`)
  - plus retained channel scaffold carries (`chz`, `nbarch`).
- Contract-derived integration vectors validating WS15 scaffold publication and
  fail-closed missing-control behavior.
- Canonical contract/index updates documenting this scaffold closure wave and
  remaining blockers for full process migration.

### Explicitly Out of Scope
- Full baseline-authoritative port of complete `chnero/chnrt/detach` routine
  families (segment loop, `hydchn`, `dcap`, `trncap`, per-class continuity).
- Closure of `GAP-SYSTEM-008` / `GAP-ROUTE-009` / `GAP-SED-006`.
- Introduction of surrogate process-physics formulas in production paths.

## Deliverables
1. `artifacts/wshedimpl15-contract-implementation-evidence.md`
2. `artifacts/wshedimpl15-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl15-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl15-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl15-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl15-channel-sediment-process-parity-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl15_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract language for WSHEDIMPL15 scope and residual blocker
   posture.
2. Implement contract-derived tests for WS15 scaffolds/guards.
3. Record pre-implementation contract gate evidence.
4. Implement production runtime/kernel edits.

## Autonomous Execution Intent (Required)
This package is execution-ready and was executed end-to-end through
disposition without requesting additional user direction.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority remains in `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No surrogate/proxy process-physics substitutions are claimed as parity.
- WSHEDIMPL15 closure is scaffold/guard closure only; full process parity
  remains open and non-promotable.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl12-worker-handoff-immediate-next-actions-closure-001/artifacts/wshedimpl12-follow-on-package-specs.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl14-watershed-baseline-authoritative-end-to-end-comparator-lane-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/chnero.for`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`
- `/workdir/wepp-forest_260430_baseline/src/chncon.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl15-watershed-channel-sediment-process-parity-migration-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws10_watershed_kernel_contract.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL15 authorization from WSHEDIMPL12 follow-on specs and
  WSHEDIMPL14 worker handoff.

### Phase B - Contract/test preparation
- Update canonical gap language for WS15 scaffold closure posture.
- Add contract-derived WS15 scaffold vectors.
- Record pre-implementation contract gate.

### Phase C - Runtime and kernel implementation
- Project channel sediment controls to runtime state.
- Add WS10 channel typed guard requirements for projected controls.
- Publish baseline conversion scaffolds in WS10 writeback.

### Phase D - Validation and evidence
- Run formatter and focused integration tests.
- Run required repository gates and record outcomes.

### Phase E - Disposition and handoff
- Publish disposition with explicit HOLD ownership.
- Hand off immediate next package for full process parity closure.

## Exit Criteria
- WS15 runtime seam projects required channel sediment control symbols.
- WS10 channel kernel fails closed when required projected controls are absent.
- WS10 publishes baseline conversion scaffold symbols (`crsh`, `depmid`,
  `depsid`) at channel state surface.
- Contract-derived WS15 vectors pass.
- Residual parity blockers remain explicitly non-promotable and documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local runtime/kernel/test/docs changes only; no network or
  credential surfaces.

## Execution Outcome Summary
- Implemented WSHEDIMPL15 scaffold wave:
  - parser-to-runtime channel sediment control projection expanded,
  - WS10 channel guard posture now requires projected sediment controls,
  - WS10 writeback now emits baseline conversion scaffold symbols for
    `chnero/chnrt/detach` migration lineage.
- Added WS15 contract-derived vectors for scaffold publication and fail-closed
  missing-control behavior.
- Full process parity remains unresolved and correctly retained as program
  blocker (`GAP-SYSTEM-008`, `GAP-ROUTE-009`, `GAP-SED-006`).
