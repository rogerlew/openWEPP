# HPHYS0285 Spring Soil Storage Retention Closure

Status: executed-hold

## Objective

Diagnose, correct, and validate the post-HPHYS0284 H1/H7/H39 spring soil-storage/retention residual, focusing on infiltration capacity, same-pass liquid ingress, WB18 percolation routing, and profile storage not holding water after corrected snowmelt timing.

## Rationale

HPHYS0284 corrected snowpack state lineage and improved `Snow-Water`, `RM`, and `Q`, but full H1..H39 semantic parity remained open. `Total-Soil` and `SoilWaterTotal` mean absolute residuals worsened from `83.841688` to `89.531529`, indicating that corrected meltout exposed downstream liquid/storage-retention defects. The lowest-regret package was therefore a baseline-authoritative storage-retention package, not WB17 `Ep` compensation.

## Included Scope

- Amend canonical `SC-*` contracts for newly proven liquid-ingress, infiltration-capacity, WB18 percolation, aggregate-storage, and narrow snowpack-state guard-seam invariants.
- Add contract-derived tests before production code edits.
- Diagnose whether storage loss is owned by WB12/WB14 infiltration capacity, WB18 same-pass profile ingress, WB18 percolation over-drainage, WB13 aggregate publication, or stale snowpack guard behavior.
- Correct one baseline-authoritative storage-retention defect if diagnosis proves one.
- Correct narrow stale/negative inactive snowpack guard behavior only where it blocks non-snow liquid ingress or dry no-event percolation.
- Run focused tests, Rust gates, and full H1..H39 runtime/semantic suite after implementation.
- Complete dual review, review disposition, dual verification, disposition, and worker handoff.

## Excluded Scope

- WB17 `Ep` tuning or plant-growth compensation unless storage tracing proves WB17 ownership.
- Broad snowpack timing/melt reformulation beyond the narrow pack-exhaustion canonicalization required by HPHYS0285 validation.
- Heuristic infiltration-capacity multipliers, empirical residual fitting, or profile-storage clamps.
- Frost migration for non-agricultural parity unless baseline evidence proves active frost-owned infiltration capacity for the H1/H7/H39 spring window.
- MOFE carry/runon storage-ingress promotion; review disposition deferred that scope to a dedicated follow-up package.
- Multi-OFE routing semantics beyond evidence needed for the 39 hillslope single-OFE parity lane.

## Deliverables

- Canonical contract amendments in `SC-PERC-001`, `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and `SC-SNOWFREEZE-001`.
- Contract-derived regression tests for direct-rain same-pass storage ingress and inactive stale-snow non-gating.
- Production code change only after contract and test gates are recorded.
- Full H1..H39 runtime and semantic metrics with pre/post-HPHYS0284 comparison.
- Dual review, review disposition, dual verification, final disposition, and worker handoff artifacts.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260604-hphys0284-spring-snowpack-retention-timing-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260604-hphys0284-spring-snowpack-retention-timing-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/grna.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/perc.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/purk.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/hphys0285_spring_soil_storage_retention_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260604-hphys0285-spring-soil-storage-retention-closure-001/**`

## Phase Plan

1. Scaffold package and placeholders.
2. Read required authority and local WB12/WB14/WB18 runtime code.
3. Localize storage-retention residual with H1/H7/H39 semantic reports and source inspection.
4. Add canonical contract amendments for the localized invariant.
5. Add contract-derived failing test before production code edits.
6. Record pre-implementation contract gate evidence.
7. Implement the minimal baseline-authoritative storage-retention correction.
8. Run focused tests, adjacent hydrology tests, Rust gates, and full H1..H39 semantic suite.
9. Complete dual review, disposition accepted findings, dual verification, final disposition, and handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests.
3. Pre-implementation contract gate.
4. Production code edits.

Production kernel edits are prohibited before steps 1-3 are complete and recorded.

## Exit Criteria

- Canonical `SC-*` contracts explicitly authorize any corrected storage-retention behavior.
- Contract-derived test fails before and passes after implementation, or the package records why no production defect was proven and remains `HOLD`.
- Full H1..H39 runtime completes and semantic metrics are recorded.
- Dual reviews and dual verification are complete with no undispositioned accepted findings.
- Evidence artifacts label claims truthfully with `Static:` vs `Ran:`.

## Security-Impact Gate

No external service, credential, network, shell-interpolated subprocess, or unsafe Rust change was made. Implementation did not change subprocess orchestration or sidecar discovery.
