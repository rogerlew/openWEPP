# Schedule Export Generation Evidence

Status: complete
Evidence mode: Static + Ran

## Outputs

- `docs/architecture/generated/hillslope-phase-schedule.mmd`
- `docs/architecture/generated/hillslope-phase-schedule.json`
- `docs/architecture/generated/hillslope-phase-schedule.dot`

## Evidence

Static: `crates/openwepp-hillslope-orchestrator/src/schedule_export.rs` builds export data from `HillslopePhaseGraph::canonical()` and validates the graph before rendering.

Static: generated JSON contains 14 nodes, 13 edges, and a 14-item `topological_order`, all ordered by canonical phase rank.

Static: node metadata includes `phase`, `rank`, and `consumer_adapter` from `hillslope_consumer_adapter_for_phase()`.

Ran: `cargo run --quiet --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --bin openwepp_hillslope_schedule_export -- generate --output-dir docs/architecture/generated` completed with exit 0.

Ran: `cargo run --quiet --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --bin openwepp_hillslope_schedule_export -- topological-order` emitted the 14 canonical phases from `normalization` through `closure_diagnostics`.

Ran: `cargo run --quiet --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --bin openwepp_hillslope_schedule_export -- validate` printed `schedule validation: ok`.
