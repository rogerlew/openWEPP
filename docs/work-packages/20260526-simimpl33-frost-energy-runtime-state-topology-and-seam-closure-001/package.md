# 20260526-simimpl33-frost-energy-runtime-state-topology-and-seam-closure-001

## Status
- state: package-complete-with-hold
- date: 2026-05-26
- timezone: UTC
- decision: HOLD

## Objective
Implement SIMIMPL33 runtime frost state topology and typed seam wiring required
for baseline-authoritative frost solver migration, including fine-layer
index/count lineage, layered conductivity lineage, and `frost.hourly.*`
payload-surface closure scaffolding.

## Why This Package Exists
SIMIMPL31 closed canonical frost routine authority and SIMIMPL32 completed
contract-derived tests plus pre-implementation gate evidence. SIMIMPL33 is the
first production-edit package in this wave and must deliver typed runtime state
surfaces required by canonical authority before SIMIMPL34 physics migration.

## Scope
### Included
- Runtime seam symbol-surface expansion for frost topology and bookkeeping
  families.
- Typed guard posture for SIMIMPL33 seam-required symbols in active frost path.
- Hydrology writeback emission for frost topology + hourly seam payloads.
- Contract-derived SIMIMPL33 tests validating:
  - topology symbol presence and boundedness,
  - typed missing-symbol failure for required frost seam symbols.
- Full governance artifacts, gate evidence, dual review, dual verification,
  handoff, and disposition.

### Explicitly Out of Scope
- Baseline-authoritative frost solver physics migration (`frzng`/`frznw`/
  `frwatc`/`frsoil`) reserved for SIMIMPL34.
- Winter-hourly hold-lift parity rerun/disposition reserved for SIMIMPL35.

## Deliverables
1. Runtime topology map:
   - `artifacts/simimpl33-runtime-state-topology-map.md`
2. Frost seam symbol catalog:
   - `artifacts/simimpl33-frost-seam-symbol-catalog.md`
3. Contract implementation evidence:
   - `artifacts/simimpl33-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/simimpl33-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/simimpl33-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/simimpl33-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/simimpl33-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl33_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
This package executes post-SIMIMPL31/SIMIMPL32 contract-first prerequisites and
implements production runtime code for the declared seam scope:
1. canonical contract authority (SIMIMPL31),
2. contract-derived tests + pre-implementation gate (SIMIMPL32),
3. production runtime/code edits (SIMIMPL33).

## Autonomous Execution Intent (Required)
This package is executed end-to-end without user intervention unless
hard-blocked by contradictory canonical requirements.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.

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
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/frostn.for`
- `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
- `/workdir/wepp-forest_260430_baseline/src/frwatc.for`
- `/workdir/wepp-forest_260430_baseline/src/frzng.for`
- `/workdir/wepp-forest_260430_baseline/src/frznw.for`
- `/workdir/wepp-forest_260430_baseline/src/getfreezecond.for`

## Intended Write Set
- `docs/work-packages/20260526-simimpl33-frost-energy-runtime-state-topology-and-seam-closure-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
- `tests/integration/parser_runtime_seam_integration.rs`

## Phase Plan
### Phase A - Intake and Prerequisite Confirmation
- Confirm SIMIMPL31 + SIMIMPL32 authority/gate preconditions.

### Phase B - Runtime Topology and Seam Implementation
- Add fine-layer topology symbols, conductivity lineage symbols, and hourly
  frost seam payload symbols under typed active-frost wiring.

### Phase C - Contract-Derived Validation
- Add/execute SIMIMPL33 seam tests for topology emission and typed failure
  posture.

### Phase D - Gates and Governance
- Run required gates, record evidence, complete review/verification/handoff.

### Phase E - Disposition
- Publish SIMIMPL33 disposition and hold posture for SIMIMPL34/SIMIMPL35.

## Exit Criteria
- Runtime frost topology and seam surfaces required by SIMIMPL33 are emitted
  under active frost coupling.
- Typed failures exist for missing required SIMIMPL33 seam symbols.
- Required gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Governance artifacts are complete with truthful `Static:`/`Ran:` labeling.

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: no
- Rationale: production kernel/runtime seam mutation with typed guards and
  contract-governed boundaries.
