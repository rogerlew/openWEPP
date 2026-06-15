# CQR07 Disposition

Status: complete-with-warnings

Static: source objective closed. `read_batch_into` is a 14-line dispatcher over
private helpers and no longer carries `#[allow(clippy::too_many_lines)]`.

Static: behavior-preservation boundaries held. No public API, WAT output
formula, unit, operand source, alias mapping, optional default, or fail-closed
guard threshold was intentionally changed.

Ran: required gates passed:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

Ran: package quality gates passed:

- focused `watershed_wat::tests`
- before/after LCOV and CRAP capture
- `markdown-doc lint`
- `git diff --check`

WARN holds:

- Target LCOV is `665/877` lines and `46/59` functions after refactor, below the
  science-tier threshold.
- Pre-existing out-of-scope CRAP rows above `30` remain for
  `WatershedWatPublicationError::fmt`, `build_watershed_daily_rows_from_wat`,
  and `read_wat_file_into`.

Disposition: close package as complete-with-warnings. Future coverage/CRAP work
should use a separate package with explicit scope for the remaining public
entry/display functions.
