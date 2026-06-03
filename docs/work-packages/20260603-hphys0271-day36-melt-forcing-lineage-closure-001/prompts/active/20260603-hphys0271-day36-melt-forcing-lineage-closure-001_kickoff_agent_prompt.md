# HPHYS0271 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0271-day36-melt-forcing-lineage-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/artifacts/review_claude_code.md`
- `docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/artifacts/review_disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for`
- `/workdir/wepp-forest_260430_baseline/src/radcur.for`

Files:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `docs/work-packages/20260603-hphys0271-day36-melt-forcing-lineage-closure-001/**`

Task: execute package objective end-to-end for H1 sim-day 36 melt
energy-balance/hourly-forcing lineage closure.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance; typed guards; no silent defaults; no WB17 `Ep`, aggregate storage,
WB13 publication, or empirical melt compensation edits.

Autonomy: execute package phases end-to-end and update required artifacts without
requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
