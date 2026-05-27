# 20260527-wshedimpl16-channel-sediment-payload-seam-closure-and-vector-promotion-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL16 immediate next actions from WSHEDIMPL15 by closing the
missing contributor sediment payload seam (`particle_diameter_m`) at WS10
ingress with fail-closed typed guards, promoting WS11 sediment vectors to
equation checks, and rerunning watershed comparator-lane evidence.

## Why This Package Exists
WSHEDIMPL15 closed runtime control/scaffold prerequisites
(`crsh/depmid/depsid`) but left full channel sediment process parity
(`GAP-SYSTEM-008`, `GAP-ROUTE-009`, `GAP-SED-006`) unresolved. Immediate next
actions explicitly required additional missing seam projection and vector
promotion before the remaining full `chnero/chnrt/detach` migration wave.

## Scope
### Included
- HBP metadata ingestion and runtime projection for contributor
  `particle_diameter_m[npart]` payloads.
- WS10 channel ingress guard expansion requiring finite positive
  `hs{ID}_particle_diameter_m_{class:04}` symbols.
- Contract updates for `SC-ROUTE-001`, `SC-SED-001`, `SC-SYSTEM-001`, and
  registry `index.md` to record WSHEDIMPL16 seam closure posture.
- Contract-derived vector promotion:
  - WS11 sediment vector asserts equation closure for current production branch
    (`qsed`, `tc`) in addition to publication presence.
  - WS11 fail-closed vector for missing particle-diameter payload.
- Watershed comparator-lane rerun evidence capture in package artifacts.

### Explicitly Out of Scope
- Full baseline-authoritative migration of channel segment process families
  (`chnero/chnrt/detach`, `dcap`, `trncap`, `hydchn`, `case12/case34`).
- Parser/runtime closure for full channel segment geometry/state families
  required for literal `chnrt` segment-loop parity.
- Disposition of `GAP-SYSTEM-008`, `GAP-ROUTE-009`, `GAP-SED-006` as closed.

## Deliverables
1. `artifacts/wshedimpl16-contract-implementation-evidence.md`
2. `artifacts/wshedimpl16-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl16-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl16-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl16-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl16-channel-sediment-seam-and-vector-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl16_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract language for WSHEDIMPL16 seam closure scope and
   residual blocker posture.
2. Implement contract-derived tests for new payload ingress guard/equation
   vectors.
3. Record pre-implementation contract gate evidence.
4. Implement production runtime/kernel edits.

## Autonomous Execution Intent (Required)
This package is execution-ready and must execute end-to-end through disposition
without requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No surrogate/proxy process-physics substitutions are claimed as parity.
- WSHEDIMPL16 is seam-and-vector closure only; full channel sediment process
  parity remains open and non-promotable.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl15-watershed-channel-sediment-process-parity-migration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl15-watershed-channel-sediment-process-parity-migration-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`
- `/workdir/wepp-forest_260430_baseline/src/chnero.for`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl16-channel-sediment-payload-seam-closure-and-vector-promotion-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-input-contract/src/parsers/hbp.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-sim-contract/src/symbols.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/erod15_wave3_contract_authority_closure_contract.rs`
- `tests/integration/ws10_watershed_kernel_contract.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL16 authorization from WSHEDIMPL15 worker handoff immediate
  next actions.

### Phase B - Contract/test preparation
- Update canonical gap/index language for WS16 seam closure posture.
- Add contract-derived payload guard/equation vectors.
- Record pre-implementation contract gate.

### Phase C - Runtime and kernel implementation
- Project contributor `particle_diameter_m` payload into runtime state surface.
- Add WS10 fail-closed guard requirements for particle-diameter payloads.

### Phase D - Validation and evidence
- Run formatter and targeted integration/runner vectors.
- Run required repository gates and record outcomes.

### Phase E - Disposition and handoff
- Publish disposition with explicit residual HOLD ownership.
- Hand off immediate next package for full `chnero/chnrt/detach` migration.

## Exit Criteria
- HBP parse/runtime surfaces carry `particle_diameter_m` contributor payload
  symbols.
- WS10 channel ingress fails closed when required particle-diameter payloads are
  absent or invalid.
- WS11 channel sediment vectors assert publication and current branch-equation
  closure.
- Comparator-lane rerun evidence is recorded.
- Residual full-process blockers remain explicitly non-promotable and
  documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local runtime/kernel/test/docs changes only; no network or
  credential surfaces.

## Execution Outcome Summary
- Implemented WSHEDIMPL16 seam closure wave:
  - HBP metadata `particle_diameter_m[npart]` is now preserved by parser
    surfaces and projected by watershed CLI ingress as contributor-scoped
    state symbols (`hs{ID}_particle_diameter_m_{class:04}`).
  - WS10 channel contributor sediment intake now enforces fail-closed required
    particle-diameter payload guards.
- Promoted WS11 sediment vector coverage from symbol-presence-only assertions to
  explicit equation checks for current production branch (`qsed`, `tc`) and
  added fail-closed missing-payload vector for particle diameter.
- Reran watershed comparator-lane contract test and full workspace gates.
- Full channel sediment process parity remains unresolved and correctly
  retained as program blocker (`GAP-SYSTEM-008`, `GAP-ROUTE-009`,
  `GAP-SED-006`).
