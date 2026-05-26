Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260525-mofe11-oratea-zero-domain-compatibility-and-h324-parity-rerun-001/package.md
- /workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md
- /workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md
- /workdir/openWEPP/docs/specifications/science-contracts/index.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
- /workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md
- /workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- /workdir/openWEPP/docs/work-packages/20260525-mofe10-legacy-gddmax-runtime-resolution-and-h324-parity-rerun-001/artifacts/mofe10_disposition.md
- /workdir/openWEPP/docs/work-packages/20260525-mofe10-legacy-gddmax-runtime-resolution-and-h324-parity-rerun-001/artifacts/worker-handoff.md
- /workdir/wepp-forest_260430_baseline/src/infile.for
- /workdir/wepp-forest_260430_baseline/src/decomp.for
- /workdir/wepp-forest_260430_baseline/src/cdecvar1.inc
Files:
- docs/specifications/science-contracts/contracts/SC-PLANT-001.md
- docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md
- crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs
- crates/openwepp-hillslope-orchestrator/src/hydrology.rs
- crates/openwepp-hillslope-orchestrator/src/tests.rs
- tests/integration/parser_runtime_seam_integration.rs
- docs/work-packages/20260525-mofe11-oratea-zero-domain-compatibility-and-h324-parity-rerun-001/**
Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline provenance; typed guards; no silent defaults.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.
