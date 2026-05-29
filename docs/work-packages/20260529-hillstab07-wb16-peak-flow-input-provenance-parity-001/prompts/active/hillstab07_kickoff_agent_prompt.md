Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hillstab07-wb16-peak-flow-input-provenance-parity-001/package.md`
- `/workdir/openWEPP/docs/audits/20260529_peak_flow_implementation_audit.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/appmth.for`
- `/workdir/wepp-forest_260430_baseline/src/rdat.for`
- `/workdir/wepp-forest_260430_baseline/src/frcfac.for`
- `/workdir/wepp-forest_260430_baseline/src/irs.for`
- `/workdir/wepp-forest_260430_baseline/src/eplane.for`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hillstab07-wb16-peak-flow-input-provenance-parity-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`

Task: execute package objective end-to-end for declared scope to address WB16
peak-flow input-provenance parity gaps from the audit.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance anchor (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards;
no silent defaults; no heuristic/proxy process-physics substitutions in
production paths.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
