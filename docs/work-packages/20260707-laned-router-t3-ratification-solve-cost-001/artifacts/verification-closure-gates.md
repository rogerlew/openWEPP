# Closure Gate Results
Package: docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/
Ran on: 2026-07-06 (local)
One-line verdict: BLOCKED: closure is blocked by `cargo clippy` (1 error) and repository-wide `markdown-doc lint` failures (10 errors).

| Command | Status | Exit | Compact metric | Log path |
|---|---|---:|---|---|
| `git diff --check` | PASS | 0 | whitespace/eol checks clean | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/git-diff-check.log` |
| `cargo fmt --check` | PASS | 0 | formatting check completed with no diffs | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/cargo-fmt-check.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | FAIL | 101 | clippy::too_many_lines in `dval.rs` (`run_iwagaki_cells_hybrid`) — 1 error; no warnings suppressed | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/cargo-clippy-full.log` |
| `cargo nextest run --workspace --profile full` | PASS | 0 | 1428 tests run: 1428 passed, 0 failed, 5 skipped; 4 slow; wall time ~591.233s | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/cargo-nextest-full.log` |
| `cargo deny check` | PASS | 0 | advisories/bans/licenses/sources: OK | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/cargo-deny-check.log` |
| Line-count check (`git diff` touched `.rs`) | PASS | 0 | d10b_reconciliation_tests: 524; dval: 731; implicit_recession: 859; kinematic_wave: 1859; profile: 213; 05_runner_execution_and_outputs: 1335. No file hit WARN (>=2000) or BLOCK (>=3000) thresholds | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/rs-line-count-check.log` |
| Markdown/doc lint discovery | PASS | 0 | tooling discovered: `wctl` and `markdown-doc` available; `wctl doc-lint` exists (repo-wide), `markdown-doc lint` command exists | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/doc-lint-discovery.log` |
| Markdown/doc lint run | FAIL | 1 | repo-wide run failed: 10 errors (broken-links), 0 warnings; command was executed with available runner unchanged from normal suite settings | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/markdown-doc-lint.log` |

Notes:
- Case-4 hybrid ratification vector remained in its normal ignored posture; no unignore or inclusion flags were added for this closure-gate run.
- This did not modify source or contract files.
