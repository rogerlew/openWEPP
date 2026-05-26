# SIMIMPL28 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl28-hourly-winter-forcing-synthesis-port-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/artifacts/snowplan01-snow-hourly-energy-balance-wp-queue.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/wepp-forest_260430_baseline/src/aspect.for`
- `/workdir/wepp-forest_260430_baseline/src/psolr.for`
- `/workdir/wepp-forest_260430_baseline/src/sunmap.for`
- `/workdir/wepp-forest_260430_baseline/src/radcur.for`
- `/workdir/wepp-forest_260430_baseline/src/hrtmp.for`
- `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`

Files:
- `docs/work-packages/20260525-simimpl28-hourly-winter-forcing-synthesis-port-001/package.md`
- `docs/work-packages/20260525-simimpl28-hourly-winter-forcing-synthesis-port-001/artifacts/*.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-climate-runtime-adapter/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/parser_runtime_seam_integration.rs`
- `docs/work-packages/README.md`

Task: execute SIMIMPL28 end-to-end for hourly winter forcing synthesis port
scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`);
typed guards; no silent defaults; no heuristic/proxy physics substitutions as
final closure behavior.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.
