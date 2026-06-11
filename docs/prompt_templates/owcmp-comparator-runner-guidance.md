# owcmp Comparator Runner Guidance

Use this snippet in work-package prompts that require openWEPP comparator,
metric, or cohort validation work.

```text
Comparator execution:
- Subagent authorization: this prompt explicitly authorizes subagent
  spawning/delegation to `comparator_suite_runner` for context-heavy `owcmp`
  comparisons only.
- Discover suites with `tools/owcmp/owcmp manifest list`.
- Prefer a manifest path under `tools/owcmp/suites/` plus `tools/owcmp/owcmp env --manifest <path>` before running a suite.
- The runner must return only compact metrics and artifact paths: command, exit
  code, execution verdict, pass count, structural failures, first divergent key,
  focus-column metrics, `summary.json`, `summary.md`, `command-log.json`, logs
  directory, and reports directory.
- Do not paste raw logs, raw per-hillslope reports, or per-row dumps into chat.
- Commit `summary.json`, `summary.md`, and `command-log.json` by default.
  Commit raw reports/logs only when the package explicitly needs them for audit.
```

This guidance keeps parent-agent context focused on outcomes and evidence paths
instead of large comparator artifacts.
