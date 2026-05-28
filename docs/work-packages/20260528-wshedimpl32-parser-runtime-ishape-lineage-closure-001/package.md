# 20260528-wshedimpl32-parser-runtime-ishape-lineage-closure-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL32 immediate-next-action scope from WSHEDIMPL31 by closing
parser/runtime `ishape` lineage ambiguity for naturally eroded channel class
mapping so watershed channel input authority projects unambiguously into WS10
runtime/kernel consumption.

## Why This Package Exists
WSHEDIMPL30/31 activated erodible-lane channel routing behavior (`ishape=3`)
in WS10 sediment paths. The remaining immediate next action from WSHEDIMPL31 is
to reconcile upstream watershed channel parser semantics with that runtime
expectation so naturally eroded class handling is explicit, typed, and
consistent end-to-end.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL32 scope:
  - `SC-ROUTE-001` (`GAP-ROUTE-009`)
  - `SC-SED-001` (`GAP-SED-006`)
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008`)
  - `docs/specifications/science-contracts/index.md`
- Parser authority closure in
  `crates/openwepp-input-contract/src/parsers/watershed_channel.rs`:
  - strict-mode `ishape` domain alignment with runtime semantics (`1..=3`),
  - compatibility normalization of legacy out-of-range `ishape` values into
    naturally eroded class (`3`) with explicit warning continuity.
- Runtime projection closure in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - explicit `ishape` domain guard at WS10 watershed channel seed boundary,
  - parser-to-runtime symbol projection continuity for naturally eroded class.
- Contract-derived parser/runtime tests and fixture updates for WS32 seam.
- Validation gate execution and evidence updates.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- Additional `chnero/chnrt/detach` process-family migrations beyond this
  parser/runtime lineage seam.

## Deliverables
1. `artifacts/wshedimpl32-contract-implementation-evidence.md`
2. `artifacts/wshedimpl32-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl32-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl32-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl32-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl32-parser-runtime-ishape-lineage-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl32_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL32 scope.
2. Implement contract-derived parser/runtime vectors for WSHEDIMPL32 behavior.
3. Record pre-implementation contract-gate evidence.
4. Implement production parser/runtime edits.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Shape-class lineage authority for this package is sourced from:
  - `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
  - `/workdir/wepp-forest_260430_baseline/src/detach.for`
  - existing canonical WS30/WS31 contract amendments.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl31-detach-lower-boundary-width-mutation-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl31-detach-lower-boundary-width-mutation-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`

## Intended Write Set
- `docs/work-packages/20260528-wshedimpl32-parser-runtime-ishape-lineage-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-input-contract/src/parsers/watershed_channel.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/infile_watershed_channel_parser_contract.rs`
- `tests/fixtures/infile/watershed_channel/compat_ishape_normalized.chn`
- `tests/fixtures/infile/watershed_channel/strict_ishape_naturally_eroded.chn`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL32 authorization from WSHEDIMPL31 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL32 scope.
- Add WSHEDIMPL32 contract-derived parser/runtime vectors.
- Record pre-implementation contract-gate evidence.

### Phase C - Parser/runtime implementation
- Reconcile `ishape` lineage authority at parser/runtime boundaries:
  - align strict parser `ishape` domain to runtime-consumable classes,
  - normalize legacy out-of-range parser classes to naturally eroded class with
    explicit warning continuity,
  - enforce typed runtime projection guard for parser-seeded `ishape` symbols.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition with explicit residual blocker ownership.
- Hand off immediate next actions for remaining channel sediment parity
  families.

## Exit Criteria
- Parser and runtime agree on explicit naturally eroded class mapping
  (`ishape=3`) with no ambiguous parser normalization path.
- Parser-derived WS10 runtime projection of `ws10_channel_{id}_ishape` preserves
  class semantics and typed guard posture.
- WSHEDIMPL32 contract-derived vectors pass.
- Required validation gates are executed and recorded.
- Residual blockers remain explicit with accurate GO/HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local parser/runtime/test/docs updates only; no network or
  credential surface changes.
