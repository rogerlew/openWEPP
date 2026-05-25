# SIMIMPL19 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl19-rm-snow-water-baseline-authoritative-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/artifacts/simimpl18_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/artifacts/simimpl18-first-day-rain-snow-partition-diagnostic.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/artifacts/simimpl18-winter-publication-leak-diagnostic.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/artifacts/simimpl18-closure-criteria-evaluation-matrix.md`

Files:
- `docs/work-packages/20260525-simimpl19-rm-snow-water-baseline-authoritative-closure-001/package.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` (if required by closure semantics)
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-runner/src/lib.rs`
- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
- `tests/integration/pl14r_tier_a_replay_rerun_contract.rs`
- `tests/integration/pl14_tier_a_candidate_replay_contract.rs`
- `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py` (only if required)
- `docs/work-packages/20260525-simimpl19-rm-snow-water-baseline-authoritative-closure-001/artifacts/*.md`

Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified);
typed guards; no silent defaults/clamping; no heuristic/proxy process-physics
substitutions in production pathways.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Do not edit production kernel/runtime/publication code until all of the
  following are complete and documented:
  1. canonical contract amendments,
  2. contract-derived tests,
  3. pre-implementation contract gate evidence.
- Canonical `SC-*` files are authoritative. Package notes are evidence only.
- For RM/Snow-Water closure, migrate baseline-authoritative behavior from
  `/workdir/wepp-forest_260430_baseline`; do not invent replacement formulas.
- If ET-related publication behavior is touched, migrate baseline-authoritative
  evap routines rather than introducing placeholders.
- Preserve typed errors/guards for non-finite or invalid-domain winter inputs;
  do not add fallback wrappers.
- Complete dual review (`review_agent_a.md`, `review_agent_b.md`) and dual
  verification (`verification_agent_a.md`, `verification_agent_b.md`) before
  final disposition.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only: scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
