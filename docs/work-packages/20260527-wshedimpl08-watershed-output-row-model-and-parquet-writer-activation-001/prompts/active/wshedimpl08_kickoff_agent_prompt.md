# WSHEDIMPL08 Kickoff Agent Prompt

Scope: local repository science-contract/system-integration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl08-watershed-output-row-model-and-parquet-writer-activation-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`

Files:
- `docs/work-packages/20260527-wshedimpl08-watershed-output-row-model-and-parquet-writer-activation-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-output/src/writers.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

Task: execute WSHED08 objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults/clamping for domain violations; preserve fail-closed writer
behavior for real I/O/serialization failures.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
