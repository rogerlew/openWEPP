# HPHYS0309 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/artifacts/snowd-branch-state-ordering-ledger.json`

Files:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `Cargo.toml`
- `tests/integration/hphys0309_snow_carry_depletion_lineage_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001/**`

Task: execute package objective end-to-end for declared scope.

Constraints: contract-first sequencing; canonical SC authority; fixed
`wepp_260430` negative-melt provenance; typed guards; no silent defaults; no production edits before source-line proof; no WB13/WB17/WB18/WB19/WB12
compensation.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
