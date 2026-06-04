# HPHYS0283 Spring Snowmelt Runoff/Infiltration Partition

Status: complete

## Objective

Diagnose, correct, and validate the spring 2014 `Total-Soil` collapse in the H1..H39 semantic suite by localizing the snowmelt runoff/infiltration partition and implementing the baseline-authoritative meltwater infiltration coupling required before committing to a downstream `Ep` or aggregate-storage physics direction.

## Rationale

The post-0281 H1..H39 rebaseline on `/tmp/hphys0281_rebaseline_20260604T143411Z` shows no movement in `Ep`, `Total-Soil`, `SoilWaterTotal`, `Dp`, `latqcc`, `Q`, `RM`, or `Snow-Water` relative to HPHYS0272/HPHYS0277. Day-1 H1/H7/H39 storage projection is essentially closed, while the largest residuals cluster around spring 2014 Julian 143-147 where candidate `Total-Soil` collapses to roughly 30-45 mm, baseline remains roughly 580-646 mm, candidate `Q` is large, and candidate `Snow-Water` remains material. Static inspection shows current runoff reconciliation adds redistributed snowmelt to runoff closure but computes Green-Ampt infiltration only from liquid rainfall and irrigation. Baseline WEPP includes `wmelt(iplane)` in both water-balance infiltration (`watbal_hourly.for`) and Green-Ampt snowmelt forcing (`grna.for`). This package resolves that authoritative coupling gap rather than compensating through `Ep`, WB13 publication, or storage reconciliation.

## Included Scope

- Preserve the post-0281 and post-0282 governance fixes and do not reopen condensation or SC-EVAP unit compliance.
- Add canonical contract authority for snowmelt as infiltration/runoff liquid forcing in `SC-SNOWFREEZE-001`, `SC-RUNOFFPART-001`, and `SC-WATBAL-001` only where needed.
- Add contract-derived tests proving redistributed snowmelt contributes to infiltration capacity and that runoff closure does not force all meltwater to `Q` when infiltration capacity is available.
- Implement baseline-authoritative snowmelt forcing in the openWEPP WB12 runoff/infiltration phase without heuristic storage compensation.
- Run targeted H1/H7/H39 spring 2014 traces and the full H1..H39 semantic suite to measure `Ep`, `Total-Soil`, `SoilWaterTotal`, `Q`, `RM`, and `Snow-Water` movement.
- Complete dual independent review, explicit finding disposition, dual verification, and final package disposition.

## Excluded Scope

- Changing WB17 `Ep` production, EVAPPM PMET lineage, or plant-growth logic.
- Changing WB13 publication formulas except as required to reflect existing writeback surfaces.
- Reproducing the pinned baseline negative-melt bug; preserve the openWEPP/wepp-forest corrected negative-melt authority.
- Broad snowpack accumulation/melt timing rewrites not required for the meltwater infiltration partition.
- Multi-OFE routing behavior except to preserve existing single-OFE behavior and avoid regressions.

## Deliverables

- Localization evidence for H1/H7/H39 spring 2014 showing the pre-fix partition of melt, infiltration, runoff, `S`, `Snow-Water`, and `Total-Soil`.
- Canonical contract amendments with baseline provenance to `/workdir/wepp-forest_260430_baseline` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Contract-derived tests that fail before production changes and pass after implementation.
- Production code that includes redistributed meltwater in WB12 infiltration forcing and caps infiltration against rainfall plus melt plus irrigation liquid supply.
- Targeted and full H1..H39 semantic metrics for continuation routing.
- Fully dispositioned dual reviews and dual verification artifacts.

## Dependencies

- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- docs/work-packages/README.md
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
- docs/specifications/science-contracts/index.md
- docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- docs/specifications/science-contracts/contracts/SC-PERC-001.md
- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- docs/decisions/0011-architecture-first-top-down-science-contracts.md
- docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- /workdir/wepp-forest_260430_baseline/src/watbal_hourly.for
- /workdir/wepp-forest_260430_baseline/src/watbal.for
- /workdir/wepp-forest_260430_baseline/src/grna.for
- /workdir/wepp-forest_260430_baseline/src/winter.for
- /workdir/wepp-forest_260430_baseline/src/snowd.for
- /workdir/wepp-forest_260430_baseline/src/melt.for
- docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/artifacts/worker-handoff.md
- docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/artifacts/worker-handoff.md
- docs/work-packages/20260603-hphys0271-day36-melt-forcing-lineage-closure-001/artifacts/worker-handoff.md
- docs/work-packages/20260603-hphys0272-hourly-radiation-unit-closure-001/artifacts/worker-handoff.md
- /tmp/hphys0281_rebaseline_20260604T143411Z/reports/hillslope_semantic_summary.md

## Intended Write Set

- docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- docs/specifications/science-contracts/contracts/SC-PERC-001.md
- Cargo.toml
- crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs
- crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs
- crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs
- crates/openwepp-sim-contract/src/units.rs
- tests/integration/hphys0283_snowmelt_infiltration_partition_contract.rs
- docs/work-packages/README.md
- docs/work-packages/20260604-hphys0283-spring-snowmelt-runoff-infiltration-partition-001/**

## Phase Plan

1. Scaffold and record localization evidence.
2. Amend canonical contracts for meltwater infiltration/runoff partition authority.
3. Add contract-derived tests and record the pre-implementation gate.
4. Implement production snowmelt infiltration forcing.
5. Run focused tests, targeted traces, full H1..H39 metrics, dual review, dual verification, and disposition.

## Dual Review and Finding Disposition Requirement

Before final package disposition, run two independent review passes and record them in `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`. Each finding must be dispositioned as `accepted`, `rejected`, `deferred`, or `follow-up` with rationale and evidence. Accepted findings must be fixed and verified. Rejected findings must explain why no change is required. Deferred or follow-up findings must be linked from the disposition and worker-handoff artifacts. Package closure is blocked while any review finding is undispositioned.

Dual verification artifacts (`artifacts/verification_agent_a.md` and `artifacts/verification_agent_b.md`) must verify both technical gates and review finding disposition.

## Contract-First Sequence

1. Amend canonical contract authority when required.
2. Implement contract-derived tests and diagnostic gates.
3. Record pre-implementation contract gate evidence.
4. Modify production code.

## Exit Criteria

- Canonical contracts explicitly require redistributed snowmelt to participate in WB12 runoff/infiltration liquid forcing with baseline provenance.
- Contract tests prove meltwater can infiltrate and does not automatically become runoff when capacity/storage permits.
- H1/H7/H39 spring 2014 traces localize post-fix movement in `Total-Soil`, `Q`, `RM`, `Snow-Water`, and `Ep`.
- The full H1..H39 semantic suite runs on post-fix HEAD and records current metrics.
- No known contract/profile, review, or validation finding remains undispositioned.

## Security-Impact Gate

No external systems or network actions are required. This package is local repository engineering work limited to flat-file reads/edits and local command execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must execute all phases through disposition, update required artifacts with truthfulness labels, and only ask for user direction when hard-blocked by missing local authority or unavailable validation substrate.
