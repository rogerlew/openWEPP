Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0208-fc-threshold-coupled-residual-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/artifacts/hphys0204_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/artifacts/claude-code-review-findings.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0207-fcwp-depth-authority-tail-closure-001/artifacts/hphys0207_disposition.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_semantic_summary.json`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260530-hphys0208-fc-threshold-coupled-residual-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0208_fc_threshold_coupled_residual_contract.rs`

Task: execute package objective end-to-end for declared scope: close semantic
residuals for coupled FC-threshold families:
`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, and `SoilWaterTotal` on the
39-hillslope cohort.

Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults; baseline provenance anchored to
`wepp-forest_260430_baseline` commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`;
no heuristic/proxy physics substitutions in production code.
Do not modify production kernel/publication code before completing:
1) contract amendments, 2) contract-derived tests, and
3) pre-implementation contract gate evidence.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
