Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/full-39-suite-metrics.md`

Files:
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `crates/openwepp-sim-contract/src/units.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/sim_contract_boundary_unit_registry.rs`
- `tests/integration/hphys0290_post_winter_rain_publication_contract.rs`
- `Cargo.toml`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/**`

Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline provenance from `/workdir/wepp-forest_260430_baseline`; typed guards; no silent defaults; no heuristic process-physics substitutions.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.
