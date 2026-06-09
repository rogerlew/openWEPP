# Execute OWCMP04

Local repository scope: `/workdir/openWEPP`.

Execution mode: package-end-to-end.

Autonomy: execute all OWCMP04 phases through disposition without additional
user intervention unless a declared hard blocker prevents progress.

Required reading:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/20260609-owcmp04-context-efficiency-suite-manifests-001/package.md`
- `tools/owcmp/AGENTS.md`
- `tools/owcmp/specification.md`
- `tools/owcmp/README.md`
- `.codex/agents/comparator_suite_runner.toml`

Conditional:

- `tests/AGENTS.md` if editing integration tests.
- `docs/standards/AGENTS.md` if changing reusable prompt or standard guidance.

Task:

1. Add declarative `owcmp` suite manifests for the three user-named validation
   cohorts.
2. Add `owcmp manifest list/show` and `owcmp env --manifest` so agents can
   discover and preflight suites cheaply.
3. Update `tools/owcmp`, `.codex`, and reusable prompt guidance so
   `comparator_suite_runner` returns compact metrics and artifact paths.
4. Add focused contract tests for discovery, env preflight, and preflight-only
   inventory manifests.
5. Run focused gates, external-authority anti-evasion gates, real `/wc1`
   manifest preflights, review, verification, line-count governance, worker
   handoff, and final package disposition.

