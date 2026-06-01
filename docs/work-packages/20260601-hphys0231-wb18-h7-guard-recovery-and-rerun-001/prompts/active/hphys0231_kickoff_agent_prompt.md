Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0231-wb18-h7-guard-recovery-and-rerun-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0230-wb18-overdrainage-authority-closure-001/artifacts/hphys0230_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0230-wb18-overdrainage-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/perc.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0231-wb18-h7-guard-recovery-and-rerun-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`

Task: execute HPHYS0231 end-to-end by triaging the H7 WB18 guard failure with
symbol/value evidence capture, implementing contract-authoritative WB18
guard-placement behavior, and rerunning `unpalatable-rind` (`H1..H39`) to
refresh residual readjudication evidence.

Constraints: contract-first sequencing is mandatory:
1) contract amendments,
2) contract-derived tests,
3) pre-implementation contract gate evidence,
4) production code edits.
Use canonical SC authority. Preserve typed guards. No silent defaults/clamps
for domain violations.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
