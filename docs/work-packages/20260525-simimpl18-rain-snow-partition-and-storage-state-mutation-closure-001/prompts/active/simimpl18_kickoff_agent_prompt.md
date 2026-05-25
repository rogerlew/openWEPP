# SIMIMPL18 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-replay-parity-full-closure-criteria.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/replay-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/simimpl17_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/simimpl17-residual-classification-and-hold-lift-rationale.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T075424Z/suite_parquet/investigation/h5_wat_semantic_comparator.json`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T075424Z/suite_parquet/investigation/baseline_stdout.txt`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T075424Z/suite_dat/investigation/baseline_stdout.txt`

Files:
- `docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/package.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `tests/integration/pl14_tier_a_candidate_replay_contract.rs`
- `tests/integration/pl14r_tier_a_replay_rerun_contract.rs`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/integration/comparator_tier_routing_metadata.rs`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/artifacts/*.md`

Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified);
typed guards; no silent defaults or silent clamping; precipitation (`P`) parity
must be evaluated over full 1095 keyed rows for this fixture.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Do not edit production tooling/runtime code until all of the following are
  complete and documented:
  1. canonical contract amendments,
  2. contract-derived tests,
  3. pre-implementation contract gate evidence.
- Canonical `SC-*` files are authoritative for hydrologic partition/storage
  behavior and publication closure semantics; package-local notes are evidence
  only.
- Resolve the winter publication leak signal where
  `coupling_vectors.winter.ssd=250.0` and
  `coupling_vectors.hydout_equivalent.snow_water=250.0` co-vary as if a static
  parameter is being published as dynamic state.
- Resolve baseline replay span policy so legacy baseline does not remain clamped
  to one year for this parity fixture.
- Contract-derived tests must be authored from canonical authority, not backfit
  from current implementation behavior.
- Do not introduce fallback wrappers or silent default/clamp behavior for
  missing required inputs, invalid domains, or non-finite values; surface typed
  errors/guards.
- Complete dual review (`review_agent_a.md`, `review_agent_b.md`) and dual
  verification (`verification_agent_a.md`, `verification_agent_b.md`) before
  final disposition.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only: scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
