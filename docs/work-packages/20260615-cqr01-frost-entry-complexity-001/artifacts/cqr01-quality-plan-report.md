# CQR01 Quality Plan Report

Status: complete

Evidence mode: static-and-ran

## Static

Planned quality dimension: function-length / lint-debt burndown for
`compute_active_frost_coupling`.

Closure metric:

- Remove `#[allow(clippy::too_many_lines)]` from
  `compute_active_frost_coupling`.
- Keep workspace clippy green under `-D warnings`.

Supporting metrics:

- Before/after target line and function span.
- Before/after target coverage and CRAP.

## Ran

- Baseline metrics captured before production edits.
- Focused frost suite passed before production edits.
- Refactor completed as private helper extraction in the same file.
- After metrics captured after production edits.
- Required closure gates passed; see `gate-results.md`.
