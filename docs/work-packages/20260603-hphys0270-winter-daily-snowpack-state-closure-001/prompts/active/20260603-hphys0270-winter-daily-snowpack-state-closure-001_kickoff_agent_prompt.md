# HPHYS0270 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`
- `/workdir/wepp-forest/src/winter.for` at commit `03fee4558456535138592630b5dedc4d81ce8d06`

Files:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/**`

Task: execute package objective end-to-end for the declared scope. Diagnose and
migrate or wire the next authoritative daily snowpack state slice needed to
close or narrow H1/H7/H39 snowpack/SWE/`RM` divergence.

Constraints: contract-first sequencing; canonical `SC-*` authority; corrected
`/workdir/wepp-forest@03fee4558456535138592630b5dedc4d81ce8d06` negative-melt
authority; typed guards; no silent defaults; no heuristic/proxy process-physics
substitutions; no WB17 `Ep` tuning; no WB13/aggregate-storage compensation;
keep non-ag frost disabled for HPHYS parity.

Autonomy: execute package phases end-to-end and update required
artifacts/disposition without requesting additional user direction unless
hard-blocked.

Outputs: update package artifacts/disposition for all completed phases, record
provenance and iterative diagnosis evidence, and record targeted plus full
H1..H39 semantic metrics.
