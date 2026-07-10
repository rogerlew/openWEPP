# Gate Results

Evidence label: Static/Ran.

Status: `EXECUTED-PASS-WITH-TARGETED-COVERAGE-SUBSTITUTION`

## Local Gate Table

| Gate | Command | Result | Evidence |
|---|---|---|---|
| Formatting | `cargo fmt --check` | PASS, exit `0` | parent shell, 2026-07-09 |
| Focused runner tests | `cargo nextest run --test cli01_runner_contract_derived_tests` | PASS, exit `0`; `13` passed | parent shell, 2026-07-09 |
| Target coverage after | `cargo llvm-cov --workspace --test cli01_runner_contract_derived_tests --json --output-path /tmp/openwepp-cqr-nightly-08-runner-errors-targeted-llvmcov.json --no-clean` | PASS, exit `0`; line `99.625468164794%`, region `98.73417721518987%` | `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-llvmcov.json` |
| Target CRAP after | `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-08-runner-errors-targeted.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-08-runner-errors-targeted-crap.json` | PASS, exit `0`; `0` target rows above `30` | `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-crap.json` |
| Focused runner clippy | `cargo clippy -p openwepp-runner --all-targets -- -D warnings` | PASS, exit `0` | parent shell, 2026-07-09 |
| Focused integration clippy | `cargo clippy --test cli01_runner_contract_derived_tests -- -D warnings` | PASS, exit `0` | parent shell, 2026-07-09 |
| Whitespace check | `git diff --check` | PASS, exit `0` | parent shell, 2026-07-09 |
| Package doc lint | `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-08-runner-errors-001 --path docs/work-packages/README.md` | PASS, exit `0`; `22` files validated | parent shell, 2026-07-09 |

Metric file provenance:

| Path | Bytes | SHA-256 |
|---|---:|---|
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted.lcov` | `2065951` | `cb7ae88ba17dcca138c89872ed74749911f902d347be7d0d144d953ce83baa72` |
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-llvmcov.json` | `8909904` | `7b09382aa2c85bd5c980b4518eb48da86fec75f7e997cf73237f871add562ed4` |
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-crap.json` | `2671921` | `2163d74f2e21dd8cd94ca04b8e59e9a0e0894543422480b1393e49e242a5473b` |

## Delegated Closure Runner

Runner:

- `comparator_suite_runner` subagent
  `019f496b-f3e6-70c3-ad3e-aa98a991d867`.
- Log directory:
  `artifacts/cqr-20260709-cqr-nightly-08-runner-errors-001-postsplit-final/`.

Ran:

| Gate | Command | Result | Evidence |
|---|---|---|---|
| Full coverage clean | `cargo llvm-cov clean --workspace` | PASS, exit `0` | `artifacts/cqr-20260709-cqr-nightly-08-runner-errors-001-postsplit-final/1-01- .log` |
| Full workspace coverage attempt | `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-08-runner-errors-full.lcov` | BLOCKED/FAIL, exit `130` | `artifacts/cqr-20260709-cqr-nightly-08-runner-errors-001-postsplit-final/2-02- .log`; marker `02-02- .log` |
| Full workspace CRAP from full LCOV | `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-08-runner-errors-full.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-08-runner-errors-full-crap.json` | BLOCKED/SKIPPED; no full LCOV produced | `artifacts/cqr-20260709-cqr-nightly-08-runner-errors-001-postsplit-final/03-cargo-crap.log` |
| Workspace clippy | `cargo clippy --workspace --all-targets -- -D warnings` | PASS, exit `0` | `artifacts/cqr-20260709-cqr-nightly-08-runner-errors-001-postsplit-final/04-cargo-clippy.log` |
| Full nextest | `cargo nextest run --workspace --profile full` | PASS, exit `0`; `1573` passed, `3` skipped | `artifacts/cqr-20260709-cqr-nightly-08-runner-errors-001-postsplit-final/05-cargo-nextest-full.log` |
| Cargo deny | `cargo deny check` | PASS, exit `0`; advisories/bans/licenses/sources ok | `artifacts/cqr-20260709-cqr-nightly-08-runner-errors-001-postsplit-final/06-cargo-deny-check.log` |

Full coverage blocker:

- `laned_shadow_h2637` failed under coverage instrumentation before LCOV
  emission with three failed tests:
  `h2637_active_fails_closed_without_routing_coefficients`,
  `h2637_active_and_shadow_are_mutually_exclusive`, and
  `h2637_active_and_disable_are_mutually_exclusive`.
- The log records `5` passed, `3` failed, and `2` ignored for that integration
  test binary.
- The log records `error: test failed, to rerun pass -p openwepp --test
  laned_shadow_h2637`.
- This blocker is outside this package write set and was not introduced by the
  runner error characterization change. The same full nextest suite passed
  outside coverage instrumentation in this closure run.

Metric substitution:

- Package Phase D permits a documented targeted equivalent when full-workspace
  coverage is blocked by unrelated coverage-instrumented tests.
- Targeted coverage/CRAP evidence above is the accepted module metric evidence
  for `crates/openwepp-runner/src/errors.rs`: target line coverage
  `266/267`, region coverage `390/395`, deduplicated CRAP rows above `30`:
  `0`, max target CRAP `20.0`.

Closure disposition:

- Required current-scope gates are satisfied.
- Full workspace coverage/CRAP is blocked by unrelated coverage-instrumented
  `laned_shadow_h2637` failures before LCOV emission; package Phase D permits
  the documented targeted coverage/CRAP equivalent used here.
- Dual verification passed after the full-coverage blocker summary was
  corrected.
