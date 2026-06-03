# HPHYS0264 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0264-wb11-wb17-pmet-seam-correction-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/review_claude_code_disposition.md`
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/evappm.for`
- `/workdir/wepp-forest_260430_baseline/src/swu.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`

Files:

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `docs/work-packages/20260603-hphys0264-wb11-wb17-pmet-seam-correction-closure-001/**`

Task: execute the HPHYS0264 objective end-to-end for the declared scope.

Constraints: contract-first sequencing; canonical SC authority; pinned baseline
provenance; typed guards; no silent defaults; no heuristic/proxy ET formulas;
do not edit production code before contracts, tests, and red gate evidence are
recorded.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
