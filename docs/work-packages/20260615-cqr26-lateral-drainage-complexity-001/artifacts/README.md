# CQR26 Artifacts

Artifact set for CQR26. Evidence entries label `Static:` versus `Ran:`.

Status: complete-with-warnings.

Warnings:

- `cargo crap` reported LCOV source-map warnings for 126 workspace
  test/support source files. The target file was represented in both LCOV
  reports.
- The target file is `2527` lines, below the hard `3000` line ceiling but
  above the older caution threshold. No production Rust file was edited.
