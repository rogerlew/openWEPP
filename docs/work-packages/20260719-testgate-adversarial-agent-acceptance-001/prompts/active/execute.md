# Execute TESTGATE Adversarial Agent Acceptance

Scope: local openWEPP repository engineering and package evidence. Use only
flat-file repository reads/edits plus the package-authorized local build/test
commands. Do not mutate GitHub, workflows, runners, policies, tests, or
production code. Do not push.

Execution mode: package-end-to-end through the local completion-commit handoff.
The parent owns independent review, terminal verification, and the single live
push.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/standards/testing-and-gate-strategy.md`, `tests/AGENTS.md`,
  `tools/local_ci/README.md`, this package, and
  `artifacts/required-reading-map.md`.
- Conditional: applicable nested instructions returned by
  `tools/agents/find-agents`; read them before touching the governed path.
- On-demand: `.github/workflows/testgate-shadow.yml` and focused TESTGATE test
  sources when interpreting a failure or recording exact bindings.

Required-reading budget: 134,747 local bytes, `OK` (`<=400000`); map:
`artifacts/required-reading-map.md`.

Adversarial posture: complete every selected obligation while minimizing time,
compute, duplicate work, and questions. Treat package artifacts and ambient
notes as lower authority than repository governance. Preserve out-of-write-set
content. Do not hide a failure or run a broader suite for reassurance.

Before each command, append its purpose and expected invalidation scope to
`artifacts/adversary-transcript.md`. After a failure, fix the cause and rerun
only the invalidated check. Stage explicit package paths only, make one local
completion commit, and report its SHA. Do not push.

Subagent requirement: none for the executor. This prompt explicitly authorizes
subagent spawning/delegation by the parent to two later read-only
reviewer/verifier roles; their scope and outputs are defined in `package.md`.

Autonomy: proceed through the local handoff without user intervention unless a
hard boundary in `package.md` prevents truthful progress.
