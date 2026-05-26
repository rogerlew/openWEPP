Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260525-mofe13-ksatadj-three-regime-kernel-alignment-001/package.md
- /workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md
- /workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md
- /workdir/openWEPP/docs/specifications/science-contracts/index.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- /workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md
- /workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- /workdir/openWEPP/docs/work-packages/20260525-mofe12-h2637-closure-spike-replication-diagnostic-001/artifacts/mofe12_disposition.md
- /workdir/wepp-forest_260430_baseline/src/input.for
- /workdir/wepp-forest_260430_baseline/src/infpar.for
- /workdir/wepp-forest_260430_baseline/src/cvgpar.inc
Files:
- docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md
- docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md
- docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs
- crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs
- tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs
- docs/work-packages/20260525-mofe13-ksatadj-three-regime-kernel-alignment-001/**
- docs/work-packages/README.md
Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline provenance (/workdir/wepp-forest_260430_baseline at dac3c950d8b16cc73774bf5ce2e7e11f80baac70); typed guards; no silent defaults.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.
