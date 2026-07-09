# Final Disposition

Status: `EXECUTED-COMPLETE`

## Decision

WSHED-W10 is `EXECUTED-COMPLETE` as of 2026-07-09 UTC.

## Closure Summary

The package ratified and implemented legacy-compatible `chan.inp` absent/open
error behavior as an explicit typed parser/default branch:

- `ichout=0`
- `dtchr_input_s=60`
- `dtchr_norm_s=60`
- `ntchr=1440`
- `cbase=0`
- `nchnum=0`
- empty `ichnum`
- `chan_output_enabled=false`

Watershed runtime now consumes `DefaultedCompat` and
`OpenErrorCollapsedCompat` parser outcomes directly when options are present.
Unconfigured missing `chan.inp` is routed through the parser at
`run_dir/chan.inp`, surfaces `CHN-W-001`, and no longer uses hidden
`dtchr=3600` / `ntchr=24` fallback globals. Explicit configured unreadable
`inputs.chaninp` remains an operator configuration error (`CLIWAT-E-029`).

## Review Disposition

Noether's lineage review and Kuhn's Rust review are accepted and dispositioned
in `review.md`. Kuhn's closure-blocking `NotApplicable` mismatch finding was
fixed with a frame context guard and regression test.

## Final Evidence

Ran:

- focused parser/frame tests: 31 passed
- standalone W10 CLI test: 1 passed
- `cargo check --workspace`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile quick`: 1399 passed
- `cargo nextest run --workspace --profile full`: 1474 passed
- `cargo deny check`
- scoped `markdown-doc lint`: 12 files, 0 errors, 0 warnings
- `git diff --check`
