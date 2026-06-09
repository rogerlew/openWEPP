# Context-Efficiency Evidence

Evidence mode: Static

Implemented context controls:

- Suite discovery no longer depends on chat history:
  `tools/owcmp/owcmp manifest list --json`.
- Suite preflight no longer requires loading raw comparator output:
  `tools/owcmp/owcmp env --manifest <path> --json`.
- Work-package prompts can reuse
  `docs/prompt_templates/owcmp-comparator-runner-guidance.md` to delegate
  context-heavy comparisons to `comparator_suite_runner`.
- `tools/owcmp/artifact-retention.md` defines the default commit set as
  `summary.json`, `summary.md`, `command-log.json`, and concise review or
  disposition evidence.
- Raw logs, raw per-hillslope reports, per-row dumps, and converted surfaces are
  local-only by default unless audit needs require promotion.
- `.codex/agents/comparator_suite_runner.toml` includes manifest/env examples
  and repeats the compact artifact contract.

Expected effect: parent agents should carry suite identity, env status, compact
verdicts, pass counts, first divergent keys, focus metrics, and artifact paths,
not raw per-row or per-hillslope reports.

