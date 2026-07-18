# TESTGATE-CI-01 Shadow Executor And CI Observation

This package implements and launches the nonblocking executor, affected-quality
measurement, and CI observation phase authorized by `TESTGATE-CI-01`.

The package is deliberately not a blocking cutover. The conservative release
runner remains authoritative until the fixed 14-day/20-increment scorecard,
protected-context evidence, and provider-side migration gates pass.

- Execution specification: [package.md](package.md)
- Active kickoff: [prompts/active/20260718-codex-execute-testgate-ci-01.md](prompts/active/20260718-codex-execute-testgate-ci-01.md)
- Evidence: [artifacts/](artifacts/)
