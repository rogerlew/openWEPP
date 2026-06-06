# HPHYS0320 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260606-hphys0320-stmtim-start-time-source-line-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/artifacts/hphys0319_fixed_stmtim_observe.json`
- `docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/artifacts/paired-stmtim-observe-classification.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`

Files:
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260606-hphys0320-stmtim-start-time-source-line-closure-001/**`

Task: execute HPHYS0320 end-to-end. source-line classify the fixed-baseline
`winter.for` storm-start timing path against OpenWEPP SIMIMPL28, implement the
baseline-authoritative timing closure if authorized, regenerate paired
H1/H7/H39 traces, and disposition the combined `57` carried rows.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults;
no canonicalize-and-proceed for domain violations; no heuristic or proxy
process-physics substitutions; no downstream WB13/WB17/WB18/WB19/WB12
compensation; no production edit until contracts and source-line proof
authorize it.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including source-line classification, implementation/test evidence, paired
trace ledger, full carried-row disposition, dual reviews, dual verification,
and worker handoff.
