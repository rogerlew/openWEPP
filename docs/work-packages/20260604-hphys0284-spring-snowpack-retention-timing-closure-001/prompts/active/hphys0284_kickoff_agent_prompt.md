Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0284-spring-snowpack-retention-timing-closure-001/package.md
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
- /workdir/openWEPP/docs/work-packages/20260604-hphys0283-spring-snowmelt-runoff-infiltration-partition-001/artifacts/worker-handoff.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0283-spring-snowmelt-runoff-infiltration-partition-001/artifacts/full-39-suite-metrics.md
- /workdir/wepp-forest_260430_baseline/src/winter.for
- /workdir/wepp-forest_260430_baseline/src/snowd.for
- /workdir/wepp-forest_260430_baseline/src/melt.for
- /workdir/wepp-forest/src/winter.for

Files:
- docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs
- tests/integration/hphys0284_negative_melt_snowpack_state_contract.rs
- Cargo.toml
- docs/work-packages/README.md
- docs/work-packages/20260604-hphys0284-spring-snowpack-retention-timing-closure-001/**

Task: execute HPHYS0284 end-to-end for the declared scope: localize and correct the remaining spring 2014 snowpack retention/timing residual after HPHYS0283.

Constraints: contract-first sequencing; canonical SC authority; pinned baseline provenance; preserve corrected wepp-forest negative-melt authority; typed guards; no silent defaults; no heuristic/proxy process-physics substitutions; dual reviews and dual verification required.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases, including targeted H1/H7/H39 trace evidence and full H1..H39 metrics.
