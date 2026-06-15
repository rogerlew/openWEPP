# CQR14 Public API Surface Parity Report

Ran: `cargo clippy --workspace --all-targets -- -D warnings`, passed after
adjusting a private helper to borrow `ReleaseHbpPair`.

Static: public exports in `crates/openwepp-runner/src/lib.rs` were not edited.

Static: CQR14 changed only private implementation details and unit tests inside
`crates/openwepp-runner/src/release.rs`.

Status: no public API delta.

Static: planned production edits are private helper extraction in runner
release code. No public API change is authorized.
