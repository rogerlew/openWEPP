Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260525-simimpl29-snowd-melt-energy-balance-kernel-port-and-coupling-001/package.md
- /workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md
- /workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md
- /workdir/openWEPP/docs/specifications/science-contracts/index.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md
- /workdir/wepp-forest_260430_baseline/src/winter.for
- /workdir/wepp-forest_260430_baseline/src/snowd.for
- /workdir/wepp-forest_260430_baseline/src/melt.for
Files:
- docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
- docs/specifications/science-contracts/index.md
- crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs
- crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs
- tests/integration/clim05_snow_runtime_kernel_contract.rs
- tests/integration/parser_runtime_seam_integration.rs
Task: execute SIMIMPL29 objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline provenance (/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70); typed guards; no silent defaults.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.
