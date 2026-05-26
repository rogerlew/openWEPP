# 20260525-simimpl29-snowd-melt-energy-balance-kernel-port-and-coupling-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC
- decision: HOLD

## Objective
Port baseline-authoritative snow kernel hourly state and melt coupling lineage
(`snowd` + `melt`) into openWEPP hydrology runtime execution, emitting required
SIMIMPL29 `snow.hourly.*` kernel-state families and preserving typed guard
posture across active winter coupling.

## Why This Package Exists
SIMIMPL28 closed hourly winter forcing synthesis publication but left hourly
snow kernel-state families open. SIMIMPL29 executes the kernel migration wave
for snow depth/density/melt state publication and runtime carry-state closure.

## Scope
### Included
- Canonical contract amendments for SIMIMPL29 snow-kernel closure authority.
- Contract-derived tests for hourly snow kernel-state publication and typed
  failure posture.
- Runtime seeding updates for snow carry-state symbols.
- Hydrology kernel implementation for hourly snowd/melt lineage with typed
  branch guards.
- Governance artifacts, reviews, verification, and disposition.

### Explicitly Out of Scope
- Full baseline frost energy-balance process-family migration (`frostN` and
  related hourly heat-flow solvers).
- Winter semantic parity rerun and hold-lift disposition (SIMIMPL30 scope).

## Deliverables
1. `artifacts/simimpl29-contract-implementation-evidence.md`
2. `artifacts/simimpl29-contract-test-implementation-evidence.md`
3. `artifacts/simimpl29-preimplementation-contract-gate.md`
4. `artifacts/simimpl29-implementation-and-test-evidence.md`
5. `artifacts/simimpl29-kernel-profile-compliance-checklist.md`
6. `artifacts/simimpl29-snowd-melt-port-mapping.md`
7. `artifacts/simimpl29-hourly-snow-state-alias-coverage.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/simimpl29_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

## Autonomous Execution Intent (Required)
This package is execution-ready and intended for autonomous end-to-end
completion through disposition without additional user intervention unless
hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/artifacts/snowplan01-snow-hourly-energy-balance-wp-queue.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`

## Intended Write Set
- `docs/work-packages/20260525-simimpl29-snowd-melt-energy-balance-kernel-port-and-coupling-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `tests/integration/parser_runtime_seam_integration.rs`

## Phase Plan
### Phase A - Intake and Authority Freeze
### Phase B - Canonical Contract Authority Amendments
### Phase C - Contract-Derived Tests
### Phase D - Pre-Implementation Contract Gate
### Phase E - Runtime Implementation and Validation
### Phase F - Governance, Review, Verification, Disposition

## Exit Criteria
- SIMIMPL29 contract authority is explicit in canonical `SC-*` locations.
- Active snow coupling emits required hourly snow kernel-state families.
- Runtime snow carry-state surfaces are published and remain non-negative.
- Active hourly symbol omissions/non-finite/domain failures remain typed hard
  errors.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: local kernel/runtime and contract/test updates; no credential,
  network, or privileged-surface changes.
