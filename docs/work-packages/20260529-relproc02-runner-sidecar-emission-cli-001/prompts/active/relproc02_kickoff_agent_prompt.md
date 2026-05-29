Scope: local repository runner/release contract and CLI implementation task;
flat-file reads/edits and local command execution only; no external
connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-relproc02-runner-sidecar-emission-cli-001/package.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-binary-release-contract.md`
- `/workdir/openWEPP/docs/governance/openwepp-release-procedure-draft.md`
- `/workdir/openWEPP/docs/work-packages/20260529-relproc01-openwepp-release-procedure-draft-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/errors.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/release.rs`

Files:
- `docs/contracts/openwepp-runner-contract.md`
- `docs/contracts/openwepp-binary-release-contract.md`
- `docs/governance/openwepp-release-procedure-draft.md`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- `crates/openwepp-runner/src/errors.rs`
- `docs/work-packages/20260529-relproc02-runner-sidecar-emission-cli-001/**`

Task: execute RELPROC02 end-to-end by implementing
`open_wepp_runner release sidecar --binary <path> --role <role>`, aligning
contracts/runbook to the command, and dispositioning validation evidence.

Constraints:
- Update contract text before production CLI code edits.
- Add/adjust tests before or alongside implementation so contract behavior is
  asserted.
- Use typed errors for invalid/missing arguments and metadata write failures.
- Do not add silent fallbacks.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
