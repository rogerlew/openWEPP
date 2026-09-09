# Command and dependency receipt

Ran: paths under `/tmp` and the target cache are external dependencies, not a
claim of repository-contained rebuildability.

| Operation | CWD | Invocation | Exit |
| --- | --- | --- | ---: |
| Final release build | `/tmp/openwepp-attribution-Q-3r3eP8` | `nix develop /workdir/openWEPP --command cargo test --release --no-run -p openwepp-land-surface-energy -p openwepp-runner` | 0 |
| Focused compact tests | `/tmp/openwepp-attribution-Q-3r3eP8` | `nix develop /workdir/openWEPP --command cargo test --release -p openwepp-land-surface-energy solver_mechanism_audit::compact_cost::tests` | 0 |
| Rustfmt | `/tmp/openwepp-attribution-Q-3r3eP8` | `nix develop /workdir/openWEPP --command cargo fmt --all -- --check` | 0 |
| P scoped Clippy | `/tmp/openwepp-compact-cost-P-vKI6gb` | `nix develop /workdir/openWEPP --command cargo clippy --release -p openwepp-land-surface-energy -p openwepp-hillslope-orchestrator -p openwepp-runner --lib -- -D warnings -A dead_code -A clippy::too_many_lines` | 101 |
| Q scoped Clippy | `/tmp/openwepp-attribution-Q-3r3eP8` | same invocation as P | 101 |

The build, test, and fmt invocations are reconstructed from the executed shell
record and their logs, which did not print argv. The frozen Clippy invocation was
executed identically in P and Q. Corresponding output is in `raw/gates/`.

The exact final series invocation was:

```text
.venv/bin/python docs/work-packages/20260909-stage3-attribution-recomputation-001/artifacts/reproduction/run_series.py --binary /workdir/.cache/openwepp/targets/openwepp-attribution-Q-3r3eP8-a0d08c9a4ce0/release/deps/openwepp_runner-682944f41aa5eb67 --environment /tmp/openwepp-compact-cost-P-vKI6gb/custody-stage/fixture/runtime-environment.json --identity docs/work-packages/20260909-stage3-attribution-recomputation-001/artifacts/raw/q-final-admission-identity.json --out docs/work-packages/20260909-stage3-attribution-recomputation-001/artifacts/raw/series-05 --cwd /tmp/openwepp-attribution-Q-3r3eP8
```

It exited 0 from `/workdir/openWEPP`. The per-process exact runner argv, cwd,
timeout, CPU, schedule, exit codes, log hashes, and semantic records are in
`raw/series-05/protocol.json` and its sixteen receipts. The external runtime
environment file SHA-256 is
`463cd045d5413611e2785f80d12f58803f36e1db5ac39533cb7af03011074a84`.

The detailed capture invoked the same runner argv recorded by the series on CPU0
with that environment plus `OPENWEPP_EXPERIMENT_TRACE_DIR` targeting
`raw/recompute-trace-cut3`, `OPENWEPP_COMPACT_COST=off`, one OFE, one repeat,
memory observation off, `RUST_MIN_STACK=67108864`, and release LTO disabled. It
exited 0; `run.log` and the trace files are retained.

`series-01` stopped after its first process because provenance admission rejected
`/binary_path`; its receipt records that error. `series-02` completed with binary
`2d86a3e1...8b857`; `series-03` with `7d2c2750...c761`; and `series-04` with
`27a3dd78...ff1b`. Those are retained development cuts produced before later
source corrections and the terminal source/custody freeze; they were not
candidates selected by timing outcome. `series-05`, binary `3e684ac3...ce53`, is
the sole accepted series because it follows the final source correction and exact
final-binary admission. Likewise `recompute-trace-cut3` supersedes earlier
retained trace cuts solely by exact-source chronology, not numerical result.

Final Q custody used the repository evidence-bundle capture and verify helpers
with source `/tmp/openwepp-attribution-Q-3r3eP8` and destination
`/tmp/openwepp-attribution-Q-final-bundle`. Successful capture and verify exits
are 0 in `raw/gates/custody-capture-cut2.log` and
`raw/gates/custody-verify-cut2.log`. An earlier capture incorrectly nested its
destination below the source and failed; that log is retained but is not final
custody evidence.
