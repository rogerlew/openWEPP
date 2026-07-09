# Coverage After

Status: `COMPLETE`

Target:
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs`

Final full-workspace LCOV:

- Command:
  `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr02-final-local-after.lcov`
- Log:
  `artifacts/logs/final-local-llvm-cov.log`
- SHA-256:
  `f6a7ddd4d5778af90d8336ae73e337903aedb2394d9bde908d242493cef93604`
- Exit code: `0`
- Artifact: `/tmp/openwepp-cqr02-final-local-after.lcov`

Target LCOV summary:

- `LF: 1975`
- `LH: 1891`
- Line coverage: `95.746835443038%`

Notes:

- `cargo llvm-cov` recorded internal failures for
  `-p openwepp --test laned_shadow_h2637` and
  `-p openwepp-hillslope-orchestrator --lib` under `--ignore-run-fail`, then
  wrote the LCOV report and exited `0`.
- The required workflow gate is the separate full nextest run, which passed
  after the LCOV run.

Final full-workspace JSON coverage export:

- Command:
  `cargo llvm-cov report --json --output-path /tmp/openwepp-cqr02-final-local-after-full.json`
- Log:
  `artifacts/logs/final-local-llvm-cov-report-json.log`
- SHA-256:
  `62037946ebf36652bd8fb1e50cfdd87bf3ac56bf79376b1a806ffad7d1eb407a`
- Exit code: `0`
- Artifact: `/tmp/openwepp-cqr02-final-local-after-full.json`

Target source-region summary from JSON:

- Unique target source regions including module tests:
  `2431 / 2536 = 95.8596214511041%`.
- Unique production/source-helper regions:
  `1517 / 1610 = 94.22360248447205%`.
- Module-local test/helper regions:
  `914 / 926 = 98.70410367170626%`.
- Metrics extraction log:
  `artifacts/logs/final-local-coverage-metrics.log`, SHA-256
  `99c28641abc5341db3adc1e52c6682460fc42a42c28d48eef56456b22207f450`,
  `__EXIT_CODE__:0`.
