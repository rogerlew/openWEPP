# HPHYS0289 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260604-hphys0288-winter-rain-on-snow-melt-partition-magnitude-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260604-hphys0288-winter-rain-on-snow-melt-partition-magnitude-closure-001/artifacts/disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/contin.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/outfil.for`

Files:
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0289_wb13_rm_snowwater_publication_contract.rs`
- `Cargo.toml`
- `docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/**`

Task: execute HPHYS0289 end-to-end for baseline-authoritative WB13 `RM`/`Snow-Water` publication lineage.

Constraints: contract-first sequencing; canonical SC authority; pinned baseline provenance; typed guards; no silent defaults; no heuristic/proxy process-physics substitutions; preserve HPHYS0287 fail-closed snow-state validation and HPHYS0288 partition centralization; dual review and dual verification required.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts, final disposition, and worker handoff for all completed phases.
