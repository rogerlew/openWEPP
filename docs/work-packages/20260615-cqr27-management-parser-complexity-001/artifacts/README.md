# CQR27 Artifacts

Artifact set for CQR27. Evidence entries label `Static:` versus `Ran:`.

Status: complete-with-warnings.

Ran: package evidence includes before/after LCOV and CRAP JSON:

- `lcov_before.info`
- `crap_before.json`
- `lcov_after.info`
- `crap_after.json`

Static: companion Markdown artifacts record target identity, line counts,
behavior equivalence, parser-surface parity, implementation and test evidence,
gate results, reviews, verification, disposition, and handoff.

Warnings: `cargo crap` reported LCOV source-map warnings for 126 workspace
test/support source files. The CQR27 target file was present in LCOV, and the
target plus extracted helpers are below CRAP `30`.
