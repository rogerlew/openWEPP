# Initial Combined-Path Benchmark

Evidence class: `Ran`

The focused subject was `openwepp-gate-planner`. The previously recorded plain
focused run executed 26/26 tests in 238.088 seconds. One new instrumented
Nextest run used profile `affected`, executed 26/26 tests, emitted fresh JUnit
and LCOV, and finished in 249.87 seconds with 493,860 KiB maximum RSS.

- Before/after Nextest inventory: byte-identical, 26 tests.
- JUnit: 26 test cases, SHA-256
  `03659652d382963a3edef47e1ef01c2c06042151ed12aae6225e16a044107a1d`.
- LCOV: 321,827 bytes, SHA-256
  `70d6285cd87cc1f2dff636f78056b08fdc50f5c45d2c2ff4ab4378d7c394f516`.
- Workspace CRAP projection from that LCOV: 11,868 entries, SHA-256
  `8a0a00cdfb142c3db9c43c2f97e6a1d21f56466a44f8d755d97c8f851f4bee7f`.
- Candidate saving if the instrumented run replaced separate plain plus
  coverage execution: 48.8%, below the required 50% median reduction.

Disposition: `HOLD`. Inventory and artifacts are promising, but the threshold
is missed and terminal-plan covering-test contribution closure is not proven.
No deduplication is authorized.
