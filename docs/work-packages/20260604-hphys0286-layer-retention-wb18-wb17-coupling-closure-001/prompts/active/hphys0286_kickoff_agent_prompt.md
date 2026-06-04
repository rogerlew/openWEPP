Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0286-layer-retention-wb18-wb17-coupling-closure-001/package.md
- /workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md
- /workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md
- /workdir/openWEPP/docs/specifications/science-contracts/index.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md
- /workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md
- /workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0285-spring-soil-storage-retention-closure-001/artifacts/worker-handoff.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0285-spring-soil-storage-retention-closure-001/artifacts/full-39-suite-metrics.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0285-spring-soil-storage-retention-closure-001/artifacts/review-disposition.md
- /workdir/wepp-forest_260430_baseline/src/watbal_hourly.for
- /workdir/wepp-forest_260430_baseline/src/perc.for
- /workdir/wepp-forest_260430_baseline/src/purk.for
- /workdir/wepp-forest_260430_baseline/src/evap.for
- /workdir/wepp-forest_260430_baseline/src/swu.for
- /workdir/wepp-forest_260430_baseline/src/watbal.for

Files:
- docs/specifications/science-contracts/contracts/SC-PERC-001.md
- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs
- crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs
- tests/integration/hphys0286_layer_retention_wb18_wb17_contract.rs
- Cargo.toml
- docs/work-packages/README.md
- docs/work-packages/20260604-hphys0286-layer-retention-wb18-wb17-coupling-closure-001/**

Task: execute HPHYS0286 end-to-end for the declared scope: localize and correct the post-HPHYS0285 post-ingress layer capacity/retention and WB18/WB17 coupling residual.

Constraints: contract-first sequencing; canonical SC authority; pinned baseline provenance; typed guards; no silent defaults; no heuristic/proxy process-physics substitutions; no WB17 `Ep` compensation before layer-state lineage is proven; dual reviews and dual verification required.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases, including H1/H7/H39 layer-retention evidence, snow-column mass trace evidence where relevant, and full H1..H39 metrics.
