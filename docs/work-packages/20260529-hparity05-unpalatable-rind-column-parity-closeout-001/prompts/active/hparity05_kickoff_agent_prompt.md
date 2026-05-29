Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity05-unpalatable-rind-column-parity-closeout-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity02-profile-capacity-storage-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity03-rainmelt-energy-snow-column-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity04-percolation-lateralflow-soilwater-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/wc1/runs/un/unpalatable-rind`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hparity05-unpalatable-rind-column-parity-closeout-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
- `tests/integration/**/hparity*_*.rs`
- `tools/legacy_comparison_suite/**`

Task: execute package objective end-to-end for declared scope: run full parity
closeout and hold-lift decision for the 12 previously always-fail hillslope
columns plus watershed integration closure checks.

Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults; baseline provenance anchored to
`wepp-forest_260430_baseline` commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`;
no heuristic/proxy physics substitutions in production code.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
