# Gate Results

Evidence label: Static/Ran.

Status: `PASS`

Focused gates run:

| Command | Result |
|---|---|
| `cargo fmt --check` | pass |
| `git diff --check` | pass |
| `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-06-watershed-kernel-direct-001 --format plain` | pass, `23 files validated, 0 errors, 0 warnings` |
| `cargo nextest run -p openwepp-watershed-orchestrator` | pass, `68 tests run: 68 passed` |
| `cargo nextest run --test wshedw5_typed_watershed_runtime_contract` | pass, `18 tests run: 18 passed` |
| `cargo clippy -p openwepp-watershed-orchestrator --all-targets -- -D warnings` | pass |
| `cargo llvm-cov -p openwepp-watershed-orchestrator --lib --no-report` | pass |
| `cargo llvm-cov --workspace --test wshedw5_typed_watershed_runtime_contract --lcov --output-path /tmp/openwepp-cqr-nightly-06-direct-targeted-final7.lcov --no-clean` | pass |
| `cargo llvm-cov --json --output-path /tmp/openwepp-cqr-nightly-06-direct-targeted-final7.json --no-run` | pass |
| `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-06-direct-targeted-final7.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-06-direct-targeted-final7-crap.json` | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass, exit `0` |
| `cargo nextest run --workspace --profile full` | pass, `1558 tests run: 1558 passed (4 slow), 3 skipped` |
| `cargo deny check` | pass, `advisories ok, bans ok, licenses ok, sources ok` |

Interrupted/non-final gate:

- `cargo llvm-cov --workspace --lcov --output-path /tmp/openwepp-cqr-nightly-06-direct-workspace.lcov --ignore-run-fail`
  was attempted and interrupted after the known unrelated
  coverage-instrumented `laned_shadow_h2637` failure/hang. This is recorded as
  non-final evidence, not as a package regression.

Current-source final gate artifact root:

- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/summary.json`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/summary.md`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/command-log.json`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/cargo_fmt.log`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/git_diff_check.log`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/doc_lint.log`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/focused_nextest.log`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/wshedw5_nextest.log`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/focused_clippy.log`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/cargo_clippy.log`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/nextest.log`
- `artifacts/20260709-cqr-nightly-06-watershed-kernel-direct-001/final-current-3/deny.log`

Heavy-run delegation:

- `comparator_suite_runner` agent `019f48e0-3f49-74c3-9276-1f2a4f31773b`
  ran the current-source heavy closure gates and reported all requested
  commands completed with exit `0`; no command was skipped.

Closure note:

- Gate execution is complete. Review, verification, final disposition, handoff,
  and completion-commit evidence are recorded in their package artifacts.
