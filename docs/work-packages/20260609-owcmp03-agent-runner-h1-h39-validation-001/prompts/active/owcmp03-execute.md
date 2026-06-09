# Execute OWCMP03

Local repository scope: `/workdir/openWEPP`.

Execution mode: package-end-to-end.

Autonomy: execute all OWCMP03 phases through disposition without additional user
intervention unless a declared hard blocker prevents progress.

Required reading:

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/package.md`
- `tools/owcmp/specification.md`
- `tools/owcmp/README.md`
- `.codex/config.toml`
- `.codex/agents/comparator_suite_runner.toml`

Conditional:
- `tests/AGENTS.md` if editing integration tests.
- `docs/standards/AGENTS.md` if changing reusable prompt or package guidance.

On-demand:
- Historical HPHYS H1-H39 artifacts only when reconstructing expected compact
  metrics or fixture paths.

Task:

1. Add a first-class `owcmp batch h1-h39-semantic` command that composes the
   existing semantic WAT comparator, writes per-H reports/logs, and emits compact
   `summary.json` and `summary.md`.
2. Update `.codex` runner config and `tools/owcmp` docs/spec so agents discover
   the new path and no active runner examples point to `tools/legacy_comparison_suite`.
3. Add focused tests for CLI contract and runner config compliance.
4. Run focused local gates and a delegated H1-H39 validation using
   `.codex/agents/comparator_suite_runner.toml`.
5. Complete reviews, disposition, verification, line-count governance, worker
   handoff, and final package disposition.
