# Context Budget Evidence

Evidence mode: Static

OWCMP03 reduces parent-agent context load by giving comparison agents one batch
command and compact handoff artifacts:

- Parent prompt: one command,
  `tools/owcmp/owcmp batch h1-h39-semantic ...`.
- Raw reports: `reports/semantic/H*.semantic.json` on disk.
- Raw logs: `logs/H*.stdout.txt` and `logs/H*.stderr.txt` on disk.
- Command evidence: `command-log.json` on disk.
- Parent handoff: `summary.json`, `summary.md`, and one compact stdout JSON line
  containing `execution_verdict`, `semantic_pass_count`, `summary_json`, and
  `summary_md`.

This avoids pasting 39 raw semantic reports or subprocess logs into premium
reasoning context. The runner config now explicitly tells
`comparator_suite_runner` to return compact metrics and paths, not raw per-H
reports.
