Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0236-wb18-hourly-iterative-execution-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0235-wb18-dp-7x-legacy-root-cause-closure-001/artifacts/hphys0235_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0235-wb18-dp-7x-legacy-root-cause-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0236-wb18-hourly-iterative-execution-closure-001/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`

Task: execute HPHYS0236 objective end-to-end for declared scope: implement
WB18 hourly iterative substep percolation semantics in production code, enforce
contract-derived regression coverage against divisor-only single-pass behavior,
run required workspace gates, rerun `unpalatable-rind` `H1..H39`, and publish
readjudication disposition.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline`); typed guards; no silent
defaults for domain violations; no heuristic/proxy process-physics
substitutions in production paths.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
