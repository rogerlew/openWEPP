# HPHYS0269 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/audits/20260525_water_erosion_kernel_audit.md`
- `/workdir/openWEPP/docs/audits/20260603_wepp_forest_nonag_frost_disable_audit.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/artifacts/spring-snowpack-lineage-diagnosis.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for`
- `/workdir/wepp-forest_260430_baseline/src/radcur.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`

Files:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/**`
- Production kernel files outside this list only if contract-first diagnosis
  proves they own a declared snowpack seam.

Task: execute package objective end-to-end for the declared scope. Diagnose and
baseline-authoritatively migrate or wire `winter.for`/`snowd.for`/`melt.for`
snowpack behavior so openWEPP snow-water retention, hourly melt release, `RM`,
and WB13 `Snow-Water` publication match pinned legacy semantics as far as the
declared source scope allows.

Constraints: contract-first sequencing; canonical `SC-*` authority; pinned
baseline provenance at
`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, except daily negative-melt redistribution uses corrected `/workdir/wepp-forest@03fee4558456535138592630b5dedc4d81ce8d06` authority and must not reproduce the pinned bug;
typed guards; no silent defaults; no heuristic/proxy process-physics
substitutions; no WB17 `Ep` tuning before the snowpack process path is wired;
keep non-ag frost disabled for HPHYS baseline parity unless a separate
correctness decision changes the target.

Autonomy: execute package phases end-to-end and update required
artifacts/disposition without requesting additional user direction unless
hard-blocked.

Outputs: update package artifacts/disposition for all completed phases, record
baseline provenance and iterative diagnosis evidence, and record targeted plus
full H1..H39 semantic metrics.
