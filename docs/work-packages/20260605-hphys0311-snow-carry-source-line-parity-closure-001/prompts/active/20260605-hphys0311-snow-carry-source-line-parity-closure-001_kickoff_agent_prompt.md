# HPHYS0311 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading:

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/specifications/wepp-input-files/specs/snow.spec.md`
- `docs/work-packages/20260605-hphys0310-prior-day-snow-carry-divergence-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0310-prior-day-snow-carry-divergence-closure-001/artifacts/prior-day-snow-carry-divergence-ledger.json`

Files:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `Cargo.toml`
- `tests/integration/hphys0311_snow_carry_source_line_parity_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/**`

Task: execute package objective end-to-end for the declared scope.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline`; typed guards; no silent
defaults; no downstream compensation; no production edit without source-line
proof.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
