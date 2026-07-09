# Gate Results

Evidence label: Static/Ran.

Status: `COMPLETE`

Focused local gates:

| Command | Status | Exit | Evidence |
|---|---|---:|---|
| `cargo fmt -- crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` | `PASS` | `0` | formatting applied/check-clean for target file |
| `cargo nextest run -p openwepp-watershed-orchestrator` | `PASS` | `0` | `39 tests run: 39 passed, 0 skipped` |
| `cargo clippy -p openwepp-watershed-orchestrator -- -D warnings` | `PASS` | `0` | finished clean |
| `cargo llvm-cov -p openwepp-watershed-orchestrator --lcov --output-path /tmp/openwepp-cqr-nightly-05-helpers-focused.lcov` | `PASS` | `0` | production LCOV `LF:532`, `LH:487`, `91.54135338345864%`; whole file `LF:856`, `LH:810`, `94.62616822429906%` |
| `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-05-helpers-focused.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-05-helpers-focused-crap.json` | `PASS` | `0` | target max CRAP `19.023147604437927`, rows over `30`: `0` |
| `cargo llvm-cov -p openwepp-watershed-orchestrator --json --summary-only --output-path /tmp/openwepp-cqr-nightly-05-helpers-focused-summary.json` | `PASS` | `0` | whole-file regions `922 / 964`, `95.64315352697096%` |
| `cargo llvm-cov -p openwepp-watershed-orchestrator --json --output-path /tmp/openwepp-cqr-nightly-05-helpers-focused-full.json` | `PASS` | `0` | weakest production function region floor: `79 / 94`, `84.04255319148936%` |
| `git diff --check` | `PASS` | `0` | no whitespace findings |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260709-cqr-nightly-05-watershed-kernel-helpers-001 --format json` | `PASS` | `0` | `22` files scanned, `0` errors, `0` warnings |
| `cargo fmt --check` | `PASS` | `0` | workspace format check clean |

Heavy workspace gates:

- Delegated to `comparator_suite_runner` agent `019f4825-b6ef-7822-b240-23c89bf52121`.

| Command | Status | Exit | Evidence |
|---|---|---:|---|
| `cargo fmt --check` | `PASS` | `0` | `artifacts/cargo-fmt-check.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | `PASS` | `0` | `artifacts/cargo-clippy-workspace-all-targets.log`; finished clean |
| `cargo nextest run --workspace --profile full` | `PASS` | `0` | `artifacts/cargo-nextest-full.log`; `1529 tests run: 1529 passed`, `3 skipped` |
| `cargo deny check` | `PASS` | `0` | `artifacts/cargo-deny-check.log`; advisories, bans, licenses, and sources ok |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-05-full.lcov` | `PASS` | `0` | `artifacts/cargo-llvm-cov-full-clean.log`; report saved to `/tmp/openwepp-cqr-nightly-05-full.lcov` |
| `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-05-full.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-05-full-crap.json` | `PASS` | `0` | `artifacts/cargo-crap-full-clean.log` |

Heavy-run target summary from the clean full LCOV/CRAP artifacts:

- Target LCOV line coverage: `LF:856`, `LH:810`, `94.62616822429906%`.
- Target CRAP rows: `21` unique functions, max CRAP
  `19.023147604437927`, rows over `30`: `0`.

Coverage caveat:

- The clean full coverage command intentionally used `--ignore-run-fail` as the
  nightly measurement command does. It produced the LCOV report and exited `0`,
  but the unrelated `-p openwepp --test laned_shadow_h2637` target failed under
  coverage instrumentation. The required full workflow gate
  `cargo nextest run --workspace --profile full` passed separately with `1529`
  tests passed. No closure claim depends on `laned_shadow_h2637` coverage
  success.

Superseded evidence:

- The interrupted duplicate `cargo-llvm-cov-full.log` was removed and is not
  used as closure evidence. The clean parent-run artifact is
  `artifacts/cargo-llvm-cov-full-clean.log`.
