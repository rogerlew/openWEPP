# 20260526-simimpl34-frost-energy-solver-kernel-migration-and-coupling-001

## Status
- state: package-complete-with-hold
- date: 2026-05-26
- timezone: UTC
- decision: HOLD

## Objective
Execute SIMIMPL34 by replacing reductive active-frost coupling behavior with
baseline-authoritative frost solver process migration (`frostN` family with
`frwatc`/`frzng`/`frznw`/`frsoil` + `getFreezeCond`) and coupling outputs into
runoff/infiltration/water-balance paths.

## Why This Package Exists
SIMIMPL33 established runtime topology and seam surfaces required for frost
solver migration. SIMIMPL34 is the production physics migration package that
must close known routine-chain and conductivity-lineage execution gaps before
SIMIMPL35 parity rerun/disposition.

## Scope
### Included
- Production migration in hydrology kernel active-frost coupling path from
  reductive closure to baseline-authoritative process shape.
- Runtime enforcement of routine-chain handoff semantics equivalent to
  `frwatc(1)` ingress / `frwatc(0)` egress effects in published runtime state.
- Runtime freeze-lineage sensitivity (`frzng`/`frznw` authority shape) and
  land-use-dependent frozen-soil conductivity lineage (`frsoil` +
  `getFreezeCond`).
- Contract-derived test activation/update for SIMIMPL32 frost vectors that were
  intentionally ignored pre-migration.
- Full governance artifacts, gate evidence, dual review, dual verification,
  handoff, and disposition.

### Explicitly Out of Scope
- Winter-hourly comparator rerun and hold-lift GO/HOLD decision publication
  (SIMIMPL35 ownership).
- New empirical regressions or surrogate proxy equations not present in
  baseline-authoritative routine lineage.

## Deliverables
1. Routine-lineage migration map:
   - `artifacts/simimpl34-routine-lineage-migration-map.md`
2. Contract implementation evidence:
   - `artifacts/simimpl34-contract-implementation-evidence.md`
3. Contract-test implementation evidence:
   - `artifacts/simimpl34-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate:
   - `artifacts/simimpl34-preimplementation-contract-gate.md`
5. Implementation/test evidence:
   - `artifacts/simimpl34-implementation-and-test-evidence.md`
6. Kernel profile checklist:
   - `artifacts/simimpl34-kernel-profile-compliance-checklist.md`
7. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl34_disposition.md`
   - `artifacts/worker-handoff.md`
8. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
9. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
SIMIMPL34 executes post-SIMIMPL31/SIMIMPL32 contract-first prerequisites:
1. canonical contract authority established in SIMIMPL31,
2. contract-derived tests + pre-implementation gate established in SIMIMPL32,
3. production code migration in this package.

## Autonomous Execution Intent (Required)
This package was executed end-to-end without user intervention.

## Truthfulness Labeling Requirement
All evidence artifacts include explicit `Static:` and/or `Ran:` sections.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/artifacts/frost-energy-solver-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001/artifacts/simimpl31_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001/artifacts/simimpl32_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl33-frost-energy-runtime-state-topology-and-seam-closure-001/artifacts/simimpl33_disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/frostn.for`
- `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
- `/workdir/wepp-forest_260430_baseline/src/frwatc.for`
- `/workdir/wepp-forest_260430_baseline/src/frzng.for`
- `/workdir/wepp-forest_260430_baseline/src/frznw.for`
- `/workdir/wepp-forest_260430_baseline/src/getfreezecond.for`

## Intended Write Set
- `docs/work-packages/20260526-simimpl34-frost-energy-solver-kernel-migration-and-coupling-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`

## Phase Plan
### Phase A - Intake and Prerequisite Confirmation
- Confirm SIMIMPL31/32/33 prerequisite authority and queue sequencing.

### Phase B - Frost Solver Migration
- Implement routine-chain-authoritative active-frost kernel behavior in
  `compute_active_frost_coupling` including freeze-lineage sensitivity,
  frwatc-style water handoff effect, and conductivity lineage selection.

### Phase C - Contract-Derived Validation
- Activate/update SIMIMPL32 frost vectors and ensure typed guard posture
  remains explicit.

### Phase D - Gates and Governance
- Run required gates, record evidence, and complete review/verification/handoff.

### Phase E - Disposition
- Publish SIMIMPL34 disposition and downstream ownership for SIMIMPL35.

## Exit Criteria
- Active-frost runtime behavior follows baseline-authoritative process shape
  without reductive proxy closure in production path.
- Previously ignored SIMIMPL32 frost vectors for handoff/freeze/conductivity
  lineage are enabled and pass.
- Required gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Governance artifacts are complete with truthful `Static:`/`Ran:` labeling.

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: no
- Rationale: production kernel runtime-process migration and cross-contract
  frost coupling mutation.
