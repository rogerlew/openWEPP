# 20260527-wshedimpl17-channel-segment-geometry-hydraulic-seam-intake-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL17 immediate next actions from WSHEDIMPL16 by implementing the
missing WS10 channel segment geometry/hydraulic seam intake required before
baseline-authoritative `chnero/chnrt/detach` segment-loop migration.

## Why This Package Exists
WSHEDIMPL16 closed contributor payload and vector coverage but explicitly handed
off unresolved full channel sediment process parity (`GAP-SYSTEM-008`,
`GAP-ROUTE-009`, `GAP-SED-006`). The next mandatory precursor is fail-closed
projection/consumption of channel segment families (`nslpts`, segment `x/slope`,
hydraulic width/depth scaffolds) at watershed runtime boundaries.

## Scope
### Included
- Runtime seeding of WS10 channel segment/hydraulic scaffold symbols from
  parsed watershed slope + channel payloads.
- WS10 kernel fail-closed guard enforcement for required WS17 segment/hydraulic
  scaffold families.
- Runner wiring to parse slope payload and seed WS17 runtime segment symbols in
  watershed CLI execution path.
- Contract updates for `SC-ROUTE-001`, `SC-SED-001`, `SC-SYSTEM-001`, and
  registry `index.md` for WSHEDIMPL17 seam-closure posture.
- Contract-derived tests for WS17 projection and fail-closed kernel guard
  behavior.

### Explicitly Out of Scope
- Full baseline-authoritative migration of channel sediment process families
  (`chnero/chnrt/detach`, `dcap`, `trncap`, `hydchn`, `case12/case34`).
- Disposition of `GAP-SYSTEM-008`, `GAP-ROUTE-009`, or `GAP-SED-006` as closed.
- Comparator promotion from seam/scaffold closure to full process parity.

## Deliverables
1. `artifacts/wshedimpl17-contract-implementation-evidence.md`
2. `artifacts/wshedimpl17-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl17-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl17-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl17-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl17-channel-segment-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl17_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract language for WSHEDIMPL17 seam-closure scope and
   residual blocker posture.
2. Implement contract-derived tests for WS17 projection + fail-closed guard
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
- WSHEDIMPL17 is segment/hydraulic seam closure only; full channel sediment
  process parity remains open and non-promotable.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl16-channel-sediment-payload-seam-closure-and-vector-promotion-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl16-channel-sediment-payload-seam-closure-and-vector-promotion-001/artifacts/worker-handoff.md`
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
- `docs/work-packages/20260527-wshedimpl17-channel-segment-geometry-hydraulic-seam-intake-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/ws10_watershed_kernel_contract.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL17 authorization from WSHEDIMPL16 worker handoff immediate
  next actions.

### Phase B - Contract/test preparation
- Update canonical gap/index language for WS17 seam-closure posture.
- Add contract-derived WS17 projection and fail-closed guard vectors.
- Record pre-implementation contract gate.

### Phase C - Runtime and kernel implementation
- Seed WS17 channel segment/hydraulic symbols from slope/channel parser
  payloads at watershed runtime boundaries.
- Require WS17 segment/hydraulic scaffold symbols in WS10 channel kernel path
  with typed fail-closed guards.
- Wire watershed CLI to parse slope input and invoke WS17 seeding.

### Phase D - Validation and evidence
- Run formatter and required gates.
- Record outcomes in package evidence artifacts with explicit `Static`/`Ran`
  labeling.

### Phase E - Disposition and handoff
- Publish disposition with explicit residual HOLD ownership.
- Hand off immediate next package for full baseline-authoritative
  `chnero/chnrt/detach` process-family migration using WS17 seam intake.

## Exit Criteria
- WS10 runtime surface includes required WS17 segment/hydraulic scaffold symbol
  families from parser input.
- WS10 channel kernel fails closed on missing/non-finite/out-of-domain WS17
  segment scaffold inputs.
- Watershed CLI parse/seeding path includes slope-driven WS17 projection.
- Contract/index posture records WSHEDIMPL17 closure and residual blockers.
- Required repository validation gates are executed and recorded.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local runtime/kernel/test/docs updates only; no network or
  credential surface changes.

## Execution Outcome Summary
- Implemented WS17 seam closure wave:
  - watershed CLI now parses slope input and seeds WS10 channel
    segment/hydraulic scaffold symbols from slope+channel payloads
    (`nslpts`, `x/slope/depa/depb/wida/widb`).
  - WS10 channel kernel now requires WS17 scaffold families with typed
    fail-closed guard enforcement before channel execution proceeds.
  - runtime input test coverage and WS11 contract-derived guard vectors were
    expanded for WS17 seam behavior.
- Canonical contract/index posture updated for WSHEDIMPL17 (`SC-ROUTE-001`,
  `SC-SED-001`, `SC-SYSTEM-001`, registry index).
- Full channel sediment process-family parity (`chnero/chnrt/detach`) remains
  unresolved and correctly retained as non-promotable HOLD blockers:
  `GAP-SYSTEM-008`, `GAP-ROUTE-009`, `GAP-SED-006`.
