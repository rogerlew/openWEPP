# HPHYS0319 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/work-packages/20260606-hphys0318-stmtim-control-surface-instrumentation-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/wepp_observe.for`

Files:
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `Cargo.toml`
- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/**`

Task: execute package objective end-to-end for declared scope. Recover
fixed-baseline `stmtim.for` observe values for H1/H7/H39 at 2013 day 11 hour
11, pair them with regenerated OpenWEPP `snow.hourly.stmtim.*_0011` traces,
and classify the combined `57` carried rows.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults;
no canonicalize-and-proceed for domain violations; no heuristic or proxy
process-physics substitutions; no permanent fixed-baseline edits; no production
precipitation-phase, snow-producer, branch-predicate, melt-term, WB13, WB17,
WB18, WB19, or WB12 edit unless canonical contracts and paired evidence
explicitly authorize one.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including fixed-baseline observe patch evidence, paired observe ledger, command
logs, dual reviews, dual verification, and worker handoff.
