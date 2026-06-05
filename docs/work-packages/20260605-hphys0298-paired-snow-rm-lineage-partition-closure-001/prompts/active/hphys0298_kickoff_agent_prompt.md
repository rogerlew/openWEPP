# HPHYS0298 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/package.md`
sequentially through disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/subsystems/observability/legacy-observe-migration.md`
- `/workdir/openWEPP/docs/specifications/subsystems/observability/observability-subsystem-contract.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/artifacts/review-disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/artifacts/defect-ledger.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/artifacts/reconstruction-evidence.md`
- `/workdir/wepp-forest_260430_baseline/src/wepp_observe.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/contin.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/grna.for`
- `/workdir/wepp-forest_260430_baseline/src/idat.for`

Files:

- `Cargo.toml`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/work-packages/README.md`
- `tests/integration/hphys0298_paired_lineage_partition_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/**`
- `crates/openwepp-runner/src/**`
- `docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/**`
- Diagnostic-only temporary worktree or reversible patch derived from
  `/workdir/wepp-forest_260430_baseline/**`

Task: execute HPHYS0298 end-to-end. Build paired baseline/openWEPP lineage
observation for all nine H1/H7/H39 snow/`RM` target windows, prove baseline
observe identity, run the full H1..H39 semantic suite, and publish a first
divergent cut-point partition ledger.

Constraints: contract-first sequencing; canonical `SC-*` authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults;
no parser compatibility for `wepp_observe*`; no downstream WB17/WB18/WB19/WB13
compensation; do not reproduce the known pinned-baseline negative-melt bug as
target physics.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts, source-partition ledger, full-suite metrics,
dual review artifacts, verification artifacts, disposition, and worker handoff
for all completed phases.
