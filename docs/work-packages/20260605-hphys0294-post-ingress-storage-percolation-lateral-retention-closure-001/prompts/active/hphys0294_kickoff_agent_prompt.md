# HPHYS0294 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0294-post-ingress-storage-percolation-lateral-retention-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260605-hphys0293-winter-melt-magnitude-timing-snowpack-depletion-closure-001/artifacts/worker-handoff.md`

Files:

- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/hphys0294_post_ingress_storage_retention_contract.rs`
- `Cargo.toml`
- `docs/work-packages/20260605-hphys0294-post-ingress-storage-percolation-lateral-retention-closure-001/artifacts/*`
- Production WB18/WB19 files only if diagnostics prove a baseline-authoritative
  storage/percolation/lateral defect.

Task: execute package objective end-to-end for declared scope.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults;
no heuristic/proxy process-physics substitutions; do not compensate excluded
snow producer residuals in WB18/WB19/WB17; run dual review and dual verification
when tooling authorization is available, otherwise record the block truthfully.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
