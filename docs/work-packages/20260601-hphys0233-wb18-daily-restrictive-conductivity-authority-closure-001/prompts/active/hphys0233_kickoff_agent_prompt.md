Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0233-wb18-daily-restrictive-conductivity-authority-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `/workdir/wepp-forest_260430_baseline/src/perc.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0232-wb18-hourly-lane-percolation-alignment-001/artifacts/hphys0232_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0232-wb18-hourly-lane-percolation-alignment-001/artifacts/worker-handoff.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0233-wb18-daily-restrictive-conductivity-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`

Task: execute HPHYS0233 objective end-to-end for declared scope: migrate
baseline-authoritative daily restrictive-layer conductivity branch
(`slflag`/`kslast`) into WB18 percolation production execution, harden WB13
`D`/`Pe` publication lineage against stale state shadowing, rerun
`unpalatable-rind` (`H1..H39`), and publish readjudication/disposition.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline`); typed guards; no silent
defaults for domain violations; no heuristic/proxy process-physics
substitutions in production paths.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
