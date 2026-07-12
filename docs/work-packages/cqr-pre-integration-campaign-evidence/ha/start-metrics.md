# High-A Start Metrics

Evidence class: **Ran**

## Source State

- Repository: `/home/workdir/openWEPP`
- Commit: `3b0976406b6d5d28c24cda0d075c5f2af5d7e871`
- Branch: `main`
- `git status --short` was empty before and after measurement.
- All ten fixed High-A modules and the one ignored-failure test are byte-identical
  to campaign metric-source commit
  `14dcb022a86aa2e8921ab1154a6b8335e9ef0c26`.

## Exact Commands And Results

The delegated `comparator_suite_runner` expanded the binding protocol with
`slug=ha` and `phase=start` and ran, in order:

```text
cargo llvm-cov clean --workspace
/usr/bin/time -v -o /tmp/openwepp-cqr-preint-ha-start-lcov.time cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-preint-ha-start.lcov
/usr/bin/time -v -o /tmp/openwepp-cqr-preint-ha-start-json.time cargo llvm-cov --workspace --ignore-run-fail --json --output-path /tmp/openwepp-cqr-preint-ha-start.json
/usr/bin/time -v -o /tmp/openwepp-cqr-preint-ha-start-crap.time cargo crap --workspace --lcov /tmp/openwepp-cqr-preint-ha-start.lcov --min 0 --format json --output /tmp/openwepp-cqr-preint-ha-start-crap.json
sha256sum /tmp/openwepp-cqr-preint-ha-start.lcov /tmp/openwepp-cqr-preint-ha-start.json /tmp/openwepp-cqr-preint-ha-start-crap.json
wc -c /tmp/openwepp-cqr-preint-ha-start.lcov /tmp/openwepp-cqr-preint-ha-start.json /tmp/openwepp-cqr-preint-ha-start-crap.json
jq '[.entries[] | select(.file | startswith("/home/workdir/openWEPP/crates/")) | select(.file | contains("/src/")) | select((.file | contains("/src/tests/")) | not) | select(.crap > 30) | {file:(.file | sub("^/home/workdir/openWEPP/"; "")), function, line, cyclomatic, coverage, crap}] | unique_by([.file,.function,.line,.cyclomatic,.coverage,.crap]) | sort_by(.file,.line,.function)' /tmp/openwepp-cqr-preint-ha-start-crap.json > /tmp/openwepp-cqr-preint-ha-start-production-over30.json
```

| Step | Exit | Elapsed | Max RSS |
| --- | ---: | ---: | ---: |
| Clean | 0 | 4.60 s observed | not recorded |
| LCOV | 0 | 34:28.70 | 825,776 KB |
| JSON | 0 | 34:19.14 | 826,952 KB |
| CRAP | 0 | 1.06 s | 203,128 KB |
| Hash, size, exact filter | 0 | negligible | not recorded |

## Artifact Integrity

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `/tmp/openwepp-cqr-preint-ha-start.lcov` | 4,376,310 | `d95f434a046a3f626d7d160e6fd535a3313f53e41d05ebd6332abdcd49f44b32` |
| `/tmp/openwepp-cqr-preint-ha-start.json` | 19,110,180 | `2f46f7fa1c7ce552fcdde7ca9d988849500457477923283dd1546792f5aa0e27` |
| `/tmp/openwepp-cqr-preint-ha-start-crap.json` | 2,869,930 | `3701decc856e2c5cb340f1e4de1944d47fbbd0aa338102d055618f4e3c9616cf` |
| `/tmp/openwepp-cqr-preint-ha-start-production-over30.json` | 15,524 | `ec86490d678acf8dfa7d3902392ce3ad93605dbafafdb13142470df50b9636a8` |

Logs and `/usr/bin/time -v` reports remain under
`/tmp/openwepp-cqr-preint-ha-start-{lcov,json,crap}.{log,time}`.

## Ignored Failure Attribution

Both coverage formats reported only
`-p openwepp --test laned_shadow_h2637` as failed: five passed, three failed,
two ignored. The failing cases were
`h2637_active_fails_closed_without_routing_coefficients`,
`h2637_active_and_disable_are_mutually_exclusive`, and
`h2637_active_and_shadow_are_mutually_exclusive`; each unexpectedly received a
successful `HillslopeRunReport`. This is the campaign-baseline parallel
environment-interference family. The target and test sources are byte-identical
to the prior measurement source. No other test target failed. Fixture text such
as `compat_quoted_header... FAILED` is intentional injected-drift subprocess
output inside a passing test, not a failed test target.

## Census

The exact production filter yields 67 deduplicated rows above 30 across 45
modules. The fixed High-A cohort contains 13 rows across all ten fixed modules.
The source-bound classification and actionable result are recorded in
`raw-to-actionable-ledger.md`.
