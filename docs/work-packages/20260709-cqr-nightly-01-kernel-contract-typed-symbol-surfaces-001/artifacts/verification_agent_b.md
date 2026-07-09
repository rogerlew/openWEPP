# Verification Agent B

Status: `PASS`

Static: read package docs/artifacts and relevant CQR/ADR closure requirements.

Ran: read-only verification only: `rg`, `nl`, `git log/status`, `sha256sum`,
`cmp`, `awk`, `jq`, `ls/stat`. No edits and no cargo gates executed.

Findings:

- None.

Verified:

- Scaffold commit exists: `232718f7 Scaffold CQR nightly 01 typed symbol
  surfaces`.
- Package-local current-run logs exist; recorded SHA-256 values match, and all
  contain `__EXIT_CODE__:0`.
- Full nextest log records `1490 tests run: 1490 passed (4 slow), 3 skipped`.
- Markdown/doc lint is current: `23` files scanned, `0` errors, `0` warnings,
  exit `0`.
- Final2 evidence is sufficient: LCOV `278 / 284`, unique source regions
  `332 / 338`, normalized per-function floor failures `0`, deduplicated CRAP
  rows `24`, rows over `30`: `0`, max CRAP `22.035011574074073`.
- Final2 local replay artifacts are byte-identical to delegated full JSON and
  CRAP JSON outputs.

Non-blocking:

- Worktree was still dirty and completion/final artifacts were not committed at
  verification time; completion commit is required before the next target
  package.
