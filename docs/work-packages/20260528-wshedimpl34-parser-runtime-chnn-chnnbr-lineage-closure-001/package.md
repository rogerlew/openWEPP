# 20260528-wshedimpl34-parser-runtime-chnn-chnnbr-lineage-closure-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL34 immediate-next-action scope from WSHEDIMPL33 by closing
parser/runtime watershed-channel Manning relation lineage ambiguity so parser
authority (`chnn >= chnnbr`) projects unambiguously into WS10 runtime/kernel
consumption.

## Why This Package Exists
WSHEDIMPL33 reconciled parser/runtime `ienslp` lineage. The next immediate
parser/runtime closure is the watershed channel Manning relation invariant:
parser authority already enforces `chnn >= chnnbr`, while WS10 runtime seed
validation previously projected these symbols independently without an explicit
cross-field fail-closed guard. WSHEDIMPL34 aligns this boundary at projection
time.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL34 scope:
  - `SC-ROUTE-001` (`GAP-ROUTE-009`)
  - `SC-SED-001` (`GAP-SED-006`)
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008`)
  - `docs/specifications/science-contracts/index.md`
- Runtime projection closure in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - explicit `chnn >= chnnbr` guard at WS10 watershed channel seed boundary,
  - typed fail-closed rejection when the invariant is violated at parser/runtime
    seam ingress.
- Contract-derived parser/runtime tests and fixture updates for WS34 seam:
  - parser strict vector for `chnn < chnnbr` rejection,
  - runtime seed vector for `chnn < chnnbr` fail-closed rejection.
- Validation gate execution and evidence updates.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- Additional `chnero/chnrt/detach` process-family migrations beyond this
  parser/runtime seam.

## Deliverables
1. `artifacts/wshedimpl34-contract-implementation-evidence.md`
2. `artifacts/wshedimpl34-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl34-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl34-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl34-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl34-parser-runtime-chnn-chnnbr-lineage-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl34_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL34 scope.
2. Implement contract-derived parser/runtime vectors for WSHEDIMPL34 behavior.
3. Record pre-implementation contract-gate evidence.
4. Implement production runtime parser-seam edits.

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
- Channel parser/runtime invariant-lineage authority for this package is sourced
  from existing canonical WS15/WS30/WS32/WS33 contract amendments plus
  watershed channel parser contract semantics.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl33-parser-runtime-ienslp-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl33-parser-runtime-ienslp-lineage-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/20260528-wshedimpl34-parser-runtime-chnn-chnnbr-lineage-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/infile_watershed_channel_parser_contract.rs`
- `tests/fixtures/infile/watershed_channel/strict_chnn_less_than_chnnbr.chn`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL34 authorization from WSHEDIMPL33 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL34 scope.
- Add WSHEDIMPL34 contract-derived parser/runtime vectors.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime seam implementation
- Reconcile `chnn/chnnbr` lineage authority at parser/runtime boundaries:
  - enforce typed runtime projection guard for parser-seeded `chnn/chnnbr`
    symbols,
  - fail-closed `chnn < chnnbr` values at WS10 seed boundary.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition with explicit residual blocker ownership.
- Hand off immediate next actions for remaining channel sediment parity
  families.

## Exit Criteria
- Parser and runtime agree on explicit watershed-channel Manning relation
  mapping (`chnn >= chnnbr`) with no ambiguous parser/runtime seam behavior.
- Parser-derived WS10 runtime projection of `ws10_channel_{id}_chnn` and
  `ws10_channel_{id}_chnnbr` preserves typed guard posture for the relation.
- WSHEDIMPL34 contract-derived vectors pass.
- Required validation gates are executed and recorded.
- Residual blockers remain explicit with accurate GO/HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local parser/runtime/test/docs updates only; no network or
  credential surface changes.
