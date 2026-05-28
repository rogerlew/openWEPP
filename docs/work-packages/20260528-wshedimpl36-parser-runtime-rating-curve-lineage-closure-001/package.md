# 20260528-wshedimpl36-parser-runtime-rating-curve-lineage-closure-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL36 immediate-next-action scope from WSHEDIMPL35 by closing
parser/runtime watershed-channel rating-curve control lineage so parser
authority for `icntrl==4` payloads projects explicitly into WS10 runtime
consumption surfaces.

## Why This Package Exists
WSHEDIMPL35 closed parser/runtime lineage for `icntrl` and `flgout`. Remaining
channel-routing control seams include rating-curve payload controls
(`rccoef`, `rcexp`, `rcoset`) that are parser-authoritative but not yet
consumed via explicit runtime symbol projection and fail-closed guards.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL36 traceability:
  - `SC-ROUTE-001` (`GAP-ROUTE-009`)
  - `SC-SED-001` (`GAP-SED-006`)
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008`)
  - `docs/specifications/science-contracts/index.md`
- Runtime projection/guard closure in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - project `ws10_channel_{id}_{rccoef,rcexp,rcoset}` when `icntrl==4`,
  - fail closed when `icntrl==4` lacks rating-curve payload,
  - fail closed when `icntrl!=4` carries rating-curve payload,
  - enforce explicit domains (`rccoef>0`, `rcexp>0`, `rcoset>=0`).
- Contract-derived parser/runtime vectors:
  - parser strict fixtures for out-of-domain rating-curve fields,
  - runtime seed vectors for payload-shape and domain fail-closed behavior.
- Validation gate execution and evidence update.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- WS11 hydrology routine-chain migration (`wshcqi/wshirs/wshrun`) owned by
  follow-on WSHEDIMPL37.

## Deliverables
1. `artifacts/wshedimpl36-contract-implementation-evidence.md`
2. `artifacts/wshedimpl36-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl36-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl36-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl36-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl36-parser-runtime-rating-curve-lineage-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl36_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL36 scope.
2. Implement contract-derived parser/runtime vectors.
3. Record pre-implementation contract-gate evidence.
4. Implement production runtime seam edits.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl35-parser-runtime-icntrl-flgout-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl35-parser-runtime-icntrl-flgout-lineage-closure-001/artifacts/worker-handoff.md`

## Intended Write Set
- `docs/work-packages/20260528-wshedimpl36-parser-runtime-rating-curve-lineage-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/infile_watershed_channel_parser_contract.rs`
- `tests/fixtures/infile/watershed_channel/strict_rating_curve_rccoef_non_positive.chn`
- `tests/fixtures/infile/watershed_channel/strict_rating_curve_rcoset_negative.chn`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL36 authorization from WSHEDIMPL35 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL36 scope.
- Add WSHEDIMPL36 contract-derived parser/runtime vectors.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime seam implementation
- Reconcile parser/runtime rating-curve lineage at WS10 seed boundaries.

### Phase D - Validation and evidence
- Run required validation gates and record outcomes with `Static`/`Ran` labels.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition and immediate next actions for WSHEDIMPL37.

## Exit Criteria
- Parser and runtime agree on explicit rating-curve payload lineage for
  `icntrl==4` channel lanes.
- Runtime seed fails closed on payload-presence and domain violations for
  `ws10_channel_{id}_{rccoef,rcexp,rcoset}` controls.
- WSHEDIMPL36 contract-derived vectors pass.
- Required validation gates are executed and recorded.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local parser/runtime/test/docs updates only; no network or
  credential surface changes.
