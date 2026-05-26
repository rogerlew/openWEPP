# SIMIMPL36 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl36-frost-hold-blocker-closure-and-rerun-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/artifacts/frost-energy-solver-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001/artifacts/simimpl35-hold-lift-decision-report.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001/artifacts/worker-handoff.md`

Files:
- `docs/work-packages/20260526-simimpl36-frost-hold-blocker-closure-and-rerun-001/package.md`
- `docs/work-packages/20260526-simimpl36-frost-hold-blocker-closure-and-rerun-001/artifacts/*.md`
- `docs/work-packages/20260526-simimpl36-frost-hold-blocker-closure-and-rerun-001/prompts/active/simimpl36_kickoff_agent_prompt.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-input-contract/src/parsers/soil.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/infile_soil_parser_contract.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/fixtures/infile/soil/compat_quoted_header_9002_policy_first.sol`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `tools/legacy_comparison_suite/README.md`

Task: execute SIMIMPL36 objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified);
typed guards; no silent defaults/clamping for material domain violations.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Do not modify kernel/runtime code before contract amendments, contract-derived
  tests, and pre-implementation gate evidence are recorded.
- Canonical `SC-*` files remain authority; package artifacts are evidence.
- Preserve typed-failure posture for domain-invalid states; no fallback wrappers.
- Complete dual review (`review_agent_a.md`, `review_agent_b.md`) and dual
  verification (`verification_agent_a.md`, `verification_agent_b.md`) before
  final disposition.

False-positive block fallback (required):
- If policy false-positive blocks full kickoff prompt, retry with a shorter
  prompt containing only scope sentence, single-phase objective, and explicit
  file list.
- If blocked again, split into micro-prompts by file group.
- Record each block event and resumed prompt shape in package artifacts.
