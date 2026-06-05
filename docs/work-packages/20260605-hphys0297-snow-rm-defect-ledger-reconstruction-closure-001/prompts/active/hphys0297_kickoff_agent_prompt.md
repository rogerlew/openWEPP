# HPHYS0297 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading:
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/artifacts/review-disposition.md`
- `docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest/src/winter.for`

Files:
- `Cargo.toml`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/work-packages/README.md`
- `tests/integration/hphys0297_snow_rm_defect_ledger_contract.rs`
- `docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/**`

Task: execute HPHYS0297 end-to-end for the declared defect-ledger scope.

Constraints: contract-first sequencing; canonical `SC-*` authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline`; corrected negative-melt
authority must not be replaced with baseline bug compatibility; typed guards; no
silent defaults; no downstream WB17/WB18/WB19/WB13 compensation.

Autonomy: execute package phases end-to-end and update required artifacts without
requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
