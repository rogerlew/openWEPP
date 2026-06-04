Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0285-spring-soil-storage-retention-closure-001/package.md
- /workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md
- /workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md
- /workdir/openWEPP/docs/specifications/science-contracts/index.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md
- /workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md
- /workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0284-spring-snowpack-retention-timing-closure-001/artifacts/worker-handoff.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0284-spring-snowpack-retention-timing-closure-001/artifacts/full-39-suite-metrics.md
- /workdir/wepp-forest_260430_baseline/src/watbal_hourly.for
- /workdir/wepp-forest_260430_baseline/src/grna.for
- /workdir/wepp-forest_260430_baseline/src/perc.for
- /workdir/wepp-forest_260430_baseline/src/purk.for
- /workdir/wepp-forest_260430_baseline/src/watbal.for

Files:
- docs/specifications/science-contracts/contracts/SC-PERC-001.md
- docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs
- tests/integration/hphys0285_spring_soil_storage_retention_contract.rs
- Cargo.toml
- docs/work-packages/README.md
- docs/work-packages/20260604-hphys0285-spring-soil-storage-retention-closure-001/**

Task: execute HPHYS0285 end-to-end for the declared scope: localize and correct the post-HPHYS0284 spring soil-storage/retention residual.

Constraints: contract-first sequencing; canonical SC authority; pinned baseline provenance; typed guards; no silent defaults; no heuristic/proxy process-physics substitutions; no WB17 `Ep` compensation before liquid/storage lineage is proven; dual reviews and dual verification required.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases, including targeted H1/H7/H39 storage trace evidence and full H1..H39 metrics.
