# CQR26 Public API Surface Parity Report

Status: complete.

Static: no production Rust file was modified.

Static: public API parity is preserved by absence of code edits. The scoped
kernel file retains all existing function signatures, helper visibility,
runtime symbol names, aliases, units, typed errors, guard IDs, publication
fields, and writeback behavior.

Ran: `cargo clippy --workspace --all-targets -- -D warnings` passed.

Ran: `cargo test --workspace` passed.
