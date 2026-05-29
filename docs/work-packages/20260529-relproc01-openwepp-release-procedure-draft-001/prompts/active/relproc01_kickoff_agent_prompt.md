Scope: local repository documentation task; flat-file reads/edits only; no
external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-relproc01-openwepp-release-procedure-draft-001/package.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-binary-release-contract.md`
- `/workdir/openWEPP/docs/decisions/0007-openwepp-runner-and-release-governance.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hillstab06-wb16-peak-closure-and-p24-climate-triage-001/artifacts/worker-handoff.md`

Files:
- `docs/governance/openwepp-release-procedure-draft.md`
- `docs/governance/README.md`
- `docs/README.md`
- `README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-relproc01-openwepp-release-procedure-draft-001/**`

Task: execute RELPROC01 end-to-end by drafting and dispositioning a canonical
release runbook with explicit commands for release gates, artifact assembly,
release lint, and stability evidence expectations.

Constraints:
- Keep guidance concrete and executable.
- Align command surfaces and invariants to current in-repo contract/code
  authority.
- Record known process/tooling gaps explicitly as follow-on items.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
