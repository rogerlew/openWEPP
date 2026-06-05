# HPHYS0296 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0295-cumulative-storage-budget-ownership-closure-001/artifacts/worker-handoff.md`

Files:
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs`
- `Cargo.toml`
- `docs/work-packages/20260605-hphys0296-snow-rm-producer-acceptance-authority-closure-001/**`

Task: execute HPHYS0296 end-to-end. Amend canonical contracts first, add
contract-derived tests, run the pre-implementation gate, then classify H1/H7/H39
snow/`RM` residuals using full H1..H39 metrics and targeted traces. Patch
production only when baseline-authoritative producer evidence proves a concrete
defect.

Constraints: contract-first sequencing; canonical `SC-*` authority;
baseline provenance from `/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified;
preserve corrected `/workdir/wepp-forest` negative-melt authority; typed
guards; no silent defaults; no downstream WB17/WB18/WB19/WB13 compensation.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
