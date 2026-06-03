Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0255-mofe-storage-projection-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe04-output-publication-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/package.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/scon.for`

Files:
- `docs/work-packages/20260602-hphys0255-mofe-storage-projection-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `tests/integration/wb11_storage_projection_kernel_contract.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`

Task: execute package objective end-to-end for declared MOFE storage projection
scope.

Constraints: contract-first sequencing; canonical SC authority; pinned
baseline provenance at `/workdir/wepp-forest_260430_baseline` commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults;
no heuristic or static area-weighted storage synthesis in production when
dynamic per-OFE hydrology state is absent.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including contract evidence, review artifacts, verification artifacts, targeted
test evidence, and HOLD/GO disposition.
