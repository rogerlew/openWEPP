# CQR14 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in `crates/openwepp-runner/src/release.rs`.

Static: protected boundaries are public API, release sidecar schema, binary
role classification, release file-name policy, stable error variants, error
payloads, hash/timestamp behavior, and JSON field behavior.

Static: characterization was required because the live target
`lint_release_directory` had `0.0%` baseline coverage.

Static: production work is limited to private helper extraction and focused
release-lint tests in `crates/openwepp-runner/src/release.rs`.

Status: complete. The CQR14 target and all newly extracted helpers are below
the CRAP `<= 30` closure threshold.
