# 20260525-mofe13-ksatadj-three-regime-kernel-alignment-001

## Status
- state: complete
- date: 2026-05-26
- timezone: UTC

## Objective
Implement baseline-authoritative `ksatadj` conductivity-adjustment behavior in
openWEPP runoff reconciliation, covering the three legacy regimes
(`solwpv`/`datver` 9001, 9002, 9003) with typed guard semantics and carved-letter
MOFE parity rerun evidence.

## Why This Package Exists
Current openWEPP parses disturbed-soil policy fields (`ksatadj`, `ksatfac`,
`ksatrec`, `lkeff`) but does not execute the legacy conductivity-adjustment
routine from `/workdir/wepp-forest_260430_baseline/src/infpar.for`.
This leaves a known parity gap in runoff/infiltration behavior for
`ksatadj(iplane)=1` soils.

## Scope
### Included
- Canonical contract amendments for WB14 runoff/infiltration conductivity
  selection when `ksatadj=1`.
- Contract-derived tests for 9001/9002/9003 regime behavior.
- Pre-implementation contract gate evidence.
- Runtime soil-policy projection needed by WB14 runoff reconciliation.
- WB14 implementation of dynamic conductivity selection with typed hard-fail
  guards and no silent defaults.
- Carved-letter `H324` rerun and semantic comparator execution.

### Explicitly Out of Scope
- Non-WB14 infiltration/routing redesign beyond scoped `ksatadj` lane.
- Approximate/proxy process equations outside documented baseline authority.
- Watershed/channel kernel changes.

## Deliverables
1. `ksatadj` regime alignment report:
   - `artifacts/mofe13-ksatadj-regime-alignment-report.md`
2. H324 parity rerun report:
   - `artifacts/mofe13-h324-parity-rerun-report.md`
3. Contract implementation evidence:
   - `artifacts/mofe13-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/mofe13-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/mofe13-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/mofe13-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/mofe13-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe13_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
    - `artifacts/verification_agent_a.md`
    - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end without user
intervention unless hard-blocked by contradictory canonical authority or
unrecoverable environment failure.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Contract-First Sequence (Required)
1. Amend canonical contracts for `ksatadj` regime authority.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production runtime/kernel code and execute rerun evidence.

No production kernel/runtime edits are permitted before steps 1-3 complete.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe12-h2637-closure-spike-replication-diagnostic-001/artifacts/mofe12_disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/input.for`
- `/workdir/wepp-forest_260430_baseline/src/infpar.for`
- `/workdir/wepp-forest_260430_baseline/src/cvgpar.inc`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `/workdir/openWEPP/tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`

## Intended Write Set
- `docs/work-packages/20260525-mofe13-ksatadj-three-regime-kernel-alignment-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`

## Phase Plan
### Phase A - Contract Authority Alignment
- Amend canonical contract authority for WB14 `Ke` selection when
  `ksatadj(iplane)=1` using baseline source provenance.
- Record explicit 9001/9002/9003 deterministic equations and domain guards.

### Phase B - Contract-Derived Tests
- Add tests proving regime behavior and typed guard failures for invalid active
  domain inputs.
- Add runtime-seam tests proving soil-policy projection symbols are published.

### Phase C - Pre-Implementation Contract Gate
- Run newly added tests before production edits and record expected failing
  posture.

### Phase D - Runtime and Kernel Implementation
- Project required soil policy/runtime regime symbols.
- Implement WB14 dynamic conductivity routine with strict typed guards.

### Phase E - Validation and Parity Rerun
- Execute required repo gates and targeted tests.
- Rerun carved-letter `H324` lane and semantic comparator.
- Record parity outcome or next typed blocker.

### Phase F - Closeout
- Complete all artifacts, gate matrix, dual review/verification, and
  disposition.

## Exit Criteria
- WB14 runtime branch executes baseline-authoritative `ksatadj` regime logic
  for 9001/9002/9003 when active.
- Contract-derived tests for regime equations/guards pass.
- H324 rerun evidence is recorded with comparator outcome or typed blocker.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: local parser/runtime/kernel logic + tests/docs only; no credential
  or network boundary changes.
