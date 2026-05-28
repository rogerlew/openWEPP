# 20260528-wshedimpl35-parser-runtime-icntrl-flgout-lineage-closure-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL35 immediate-next-action scope from WSHEDIMPL34 by closing
parser/runtime watershed-channel control-lineage ambiguity for `icntrl` and
`flgout` so parser authority projects unambiguously into WS10 runtime/kernel
consumption boundaries.

## Why This Package Exists
WSHEDIMPL34 closed `chnn >= chnnbr` parser/runtime lineage. The next immediate
seam from the WSHEDIMPL34 handoff is continued parser/runtime synchronization
for channel-routing control symbols as `chnero/chnrt/detach` migration proceeds.
`icntrl` and `flgout` are parser-authoritative controls but were not projected
or domain-guarded at WS10 runtime seed boundaries.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL35 scope:
  - `SC-ROUTE-001` (`GAP-ROUTE-009`)
  - `SC-SED-001` (`GAP-SED-006`)
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008`)
  - `docs/specifications/science-contracts/index.md`
- Runtime projection closure in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - add WS10 runtime seed projection of `ws10_channel_{id}_icntrl` and
    `ws10_channel_{id}_flgout`,
  - add explicit typed fail-closed domain guards for parser/runtime seam
    ingress (`icntrl in [0,4]`, `flgout in [0,1]`).
- Contract-derived parser/runtime tests and fixture updates:
  - parser strict vectors for `icntrl` and `flgout` out-of-domain rejection,
  - runtime seed vectors for out-of-domain fail-closed rejection on projected
    WS10 control symbols.
- Validation gate execution and evidence updates.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- Additional `chnero/chnrt/detach` process-family migration beyond this
  parser/runtime control-lineage closure.

## Deliverables
1. `artifacts/wshedimpl35-contract-implementation-evidence.md`
2. `artifacts/wshedimpl35-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl35-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl35-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl35-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl35-parser-runtime-icntrl-flgout-lineage-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl35_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL35 scope.
2. Implement contract-derived parser/runtime vectors for WSHEDIMPL35 behavior.
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
- Channel parser/runtime control-lineage authority for this package is sourced
  from canonical WS11/WS15+ routing contract authority plus watershed channel
  parser domain semantics.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl34-parser-runtime-chnn-chnnbr-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl34-parser-runtime-chnn-chnnbr-lineage-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/20260528-wshedimpl35-parser-runtime-icntrl-flgout-lineage-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/infile_watershed_channel_parser_contract.rs`
- `tests/fixtures/infile/watershed_channel/strict_icntrl_out_of_domain.chn`
- `tests/fixtures/infile/watershed_channel/strict_flgout_out_of_domain.chn`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL35 authorization from WSHEDIMPL34 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL35 scope.
- Add WSHEDIMPL35 contract-derived parser/runtime vectors.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime seam implementation
- Reconcile `icntrl/flgout` parser/runtime lineage at WS10 seed boundaries:
  - enforce explicit domain guards for `icntrl` and `flgout`,
  - project `ws10_channel_{id}_icntrl` and `ws10_channel_{id}_flgout`
    surfaces with typed fail-closed guard posture.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition with explicit residual blocker ownership.
- Hand off immediate next actions for remaining channel sediment parity
  families.

## Exit Criteria
- Parser and runtime agree on explicit WS10 channel control-lineage mapping for
  `icntrl` and `flgout`.
- Runtime seed fails closed on out-of-domain parser/runtime ingress values for
  `ws10_channel_{id}_icntrl` and `ws10_channel_{id}_flgout`.
- WSHEDIMPL35 contract-derived vectors pass.
- Required validation gates are executed and recorded.
- Residual blockers remain explicit with accurate GO/HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local parser/runtime/test/docs updates only; no network or
  credential surface changes.
