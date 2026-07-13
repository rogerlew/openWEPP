# Medium Start Metrics

Evidence class: **Ran + Static**

Status: `PASS-SAME-SOURCE-RECOVERY`

## Source And Protocol

- Repository: `/home/workdir/openWEPP`.
- Clean source commit: `ec2f197e1f3248d14fb1232dc1ebb8766d5dc6b5`.
- Literal slug/phase: `medium` / `start`.
- LCOV command: `cargo llvm-cov --workspace --ignore-run-fail --lcov
  --output-path /tmp/openwepp-cqr-preint-medium-start.lcov`.
- Revised JSON command: `cargo llvm-cov report --json --output-path
  /tmp/openwepp-cqr-preint-medium-start.json`.
- CRAP command: `cargo crap --workspace --lcov
  /tmp/openwepp-cqr-preint-medium-start.lcov --min 0 --format json --output
  /tmp/openwepp-cqr-preint-medium-start-crap.json`.

## Results

| Phase | Exit | Elapsed | Max RSS |
| --- | ---: | ---: | ---: |
| LCOV | 0 | `35:41.85` | 833,232 KB |
| JSON report | 1 | `0:00.22` | 34,856 KB |
| CRAP | 0 | `0:01.12` | 208,784 KB |

The stale execution-contract text initially triggered a second instrumented
JSON run. The revised campaign cadence supersedes that command with a report
from the single LCOV profile. The duplicate was terminated promptly, but it
had already cleared `target/llvm-cov-target`; consequently the revised report
failed with `not found *.profraw`. LCOV and CRAP remain valid. The binding
start-recovery rule accepts the predecessor JSON because source commit and the
mechanically filtered census are identical. Repeating a 35-minute workspace
run solely for redundant start metadata is not required; Medium final must use
the corrected `--no-report` plus two-report sequence.

## Artifact Integrity

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| LCOV | 4,507,977 | `a2805c79cf00d50a9d1d0182abc4f10aeddc6c0d3e6480d00a768da94766516c` |
| CRAP JSON | 2,950,460 | `ed71152a1d0e630a93d2cc8591e95c038625ebda8f82930df8efd25951812e40` |
| Production over 30 | 6,872 | `292c40b7d9eb3dd757c6d4d8cf3e4656bb9bfea00f845e5d065ba37e3fa37118` |
| Substituted HB-final coverage JSON | 19,793,305 | `e7088349ed830f636f0ddffb45ac535de91968a76078b47b286c647bb90cec02` |

Durable primary artifacts and all available logs/times are under
`medium/start/`. The failed JSON log/time are retained as correction evidence;
the substituted JSON remains at `hb/final/final.json` to avoid duplicating a
19 MB same-source artifact in the documentation tree.

## Census And Same-Source Check

The production filter contains 32 rows across 25 modules. Its hash exactly
matches the High-B final production filter, proving no row/module drift between
the clean High-B transition and Medium start. No new identity, removed
identity, or same-source regression is observed.

## Ignored-Run Failure Attribution

LCOV records four failures, all in the known shared-environment H2637 selector
family:

- `active_trace_selector_requires_active_before_outputs`
- `h2637_active_fails_closed_without_routing_coefficients`
- `h2637_active_and_disable_are_mutually_exclusive`
- `h2637_active_and_shadow_are_mutually_exclusive`

These are the previously attributed coverage-only failure class, outside the
Medium target census. No target-related or unattributed LCOV failure appears.
The JSON report failure is fully attributed to the cleared profile and is not
a test failure. The substituted same-source JSON carries the previously
archived High-B attribution.
