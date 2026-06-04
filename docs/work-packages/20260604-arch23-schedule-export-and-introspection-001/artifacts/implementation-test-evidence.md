# Implementation and Test Evidence

Status: complete
Evidence mode: Static + Ran

## Implementation Evidence

Static: `crates/openwepp-hillslope-orchestrator/src/schedule_export.rs` implements deterministic export, validation diagnostics, JSON parsing for schedule diffs, and text diff rendering without adding dependencies.

Static: `crates/openwepp-hillslope-orchestrator/src/bin/openwepp_hillslope_schedule_export.rs` implements local commands: `export`, `generate`, `validate`, `topological-order`, and `diff`.

Static: `crates/openwepp-hillslope-orchestrator/src/lib.rs` exposes the `schedule_export` module.

Static: `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` adds only a `#[cfg(test)]` malformed-graph constructor.

Static: `tools/release/check_hillslope_schedule_export.sh` regenerates artifacts into a temp directory and compares JSON, Mermaid, and DOT with `diff -u`.

## Test Evidence

Static: `crates/openwepp-hillslope-orchestrator/src/tests.rs` includes tests for:

- canonical format output;
- cycle diagnostics;
- disconnected phase diagnostics;
- topological-order drift diagnostics;
- synthetic schedule diff added/removed nodes and edges.

Ran: `cargo test --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml` passed with 97 library tests, 0 binary tests, and 0 doctests.

Ran: `cargo clippy --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --all-targets -- -D warnings` passed.

Ran: `cargo test --workspace` passed, including the ARCH23 unit tests and all workspace integration/doc tests.

## Dependency Notes

Static: no new Cargo dependencies were added. `Cargo.toml` was not edited for ARCH23.
