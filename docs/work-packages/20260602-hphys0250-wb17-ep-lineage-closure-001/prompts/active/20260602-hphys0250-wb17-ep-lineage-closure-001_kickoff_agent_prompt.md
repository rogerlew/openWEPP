# HPHYS0250 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0250-wb17-ep-lineage-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260602-hphys0249-wb17-et-snow-runoff-storage-closure-001/artifacts/hphys0249_disposition.md`
- `docs/work-packages/20260602-hphys0249-wb17-et-snow-runoff-storage-closure-001/artifacts/worker-handoff.md`

Files:

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `tests/integration/wb17_et_physics_kernel_contract.rs`
- `docs/work-packages/20260602-hphys0250-wb17-ep-lineage-closure-001/**`

Task: execute package objective end-to-end for the declared WB17 `Ep` lineage
scope.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline` commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults;
no heuristic/proxy process-physics substitutions in production code.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including dual review, dual verification, gate logs, full `H1..H39` metrics,
residual ledger, and worker handoff.
