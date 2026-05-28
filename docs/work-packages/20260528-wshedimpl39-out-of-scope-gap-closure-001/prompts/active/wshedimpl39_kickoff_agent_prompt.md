# WSHEDIMPL39 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl39-out-of-scope-gap-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl38-channel-sediment-symbol-burndown-hold-lift-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl38-channel-sediment-symbol-burndown-hold-lift-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/contracts/openwepp-watershed-runfile-contract.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

Files:
- `docs/work-packages/20260528-wshedimpl39-out-of-scope-gap-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` (if required)
- `docs/specifications/science-contracts/index.md`
- `docs/contracts/openwepp-watershed-runfile-contract.md`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

Task: execute WSHEDIMPL39 objective end-to-end for declared scope by closing
or evidence-dispositioning residual out-of-scope blockers from WSHEDIMPL38
(`GAP-ROUTE-005`, `GAP-SYSTEM-001`, `GAP-SYSTEM-002`) with contract-first
authority, contract-derived tests, and typed runtime validator closure.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance where applicable; typed guards; no silent defaults/clamping.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
