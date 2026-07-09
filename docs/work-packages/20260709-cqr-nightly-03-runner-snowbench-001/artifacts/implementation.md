# Implementation

Evidence label: Static/Ran.

Status: `EXECUTED`

Implementation summary:

- Replaced the monolithic `run` command parser with `run_with_args`,
  `classify_top_level_command`, `run_snowbench_command`, and common
  parse-result helpers.
- Added `CommonSnowbenchArgs`, `JenningsPhaseArgs`, `ParsedCliArgs`, and
  `CliParseAction` to make help short-circuiting explicit without changing
  error precedence.
- Extracted common snowbench flag handling and Jennings flag handling into
  private helpers.
- Preserved diagnostic runner dispatch, request field mapping, help text,
  command-specific guard text, and missing/unknown argument precedence.
- Added 9 module-local tests in
  `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`.

Non-goals preserved:

- no snow/frost formulas changed;
- no snowbench report schemas or stdout strings changed;
- no default activation or contract authority changed.
