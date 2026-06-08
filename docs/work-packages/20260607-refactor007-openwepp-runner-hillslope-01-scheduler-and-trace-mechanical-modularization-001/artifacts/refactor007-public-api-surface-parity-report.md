# REFACTOR007 refactor007 public api surface parity report

Status: complete  
Evidence mode: static+ran  
Date: 2026-06-08

## Static
- No public-facing Rust API surface was intentionally removed or renamed.
- The split is mechanically internal to `hillslope` module internals.
- `crates/openwepp-runner/src/hillslope/scheduler_trace::mod.rs` re-exports all
  extracted items via include-backed module wiring, preserving the same symbols and
  visibility (`pub`/`pub(super)` bindings unchanged).
- `01_scheduler_and_trace.rs` is intentionally reduced to a wrapper/facade and no
  callsite signature changes.

## Ran
- `cargo test -p openwepp-runner --tests`: pass.
- `cargo test --workspace`: pass.

## Scope
Parity for scheduler/trace execution surface and layout-coupled assertions.
