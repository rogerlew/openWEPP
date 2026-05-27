# WSHEDIMPL18 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Production/runtime edits:
  - `crates/openwepp-watershed-orchestrator/src/lib.rs`
    - Expanded `Ws15ChannelSedimentControls` to include `ishape` and `ctlz`
      for sediment-capacity branch inputs.
    - Changed `require_ws17_channel_segment_scaffold(...)` to return `nslpts`
      so terminal segment slope/width symbols can be consumed directly.
    - Migrated hillslope sediment ingress from scalar-only mass return to
      structured payload (`mass_kg`, class fractions, class diameters).
    - Added WS18 helper families for baseline-lineage sediment-capacity
      computation:
      - `ws18_shield_parameter(...)`
      - `ws18_hydchn(...)`
      - `ws18_trncap(...)`
    - Updated `assemble_incoming_sediment_load_and_capacity(...)` to compute
      class-aware transport capacity `tc` and remove surrogate `tc = qsed`
      coupling.
    - Added typed guard/lint-safe closure adjustments for strict
      `clippy -D warnings` compliance.
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
    - Updated/added WS18 process-equivalence vectors for transport-capacity
      behavior.

## Ran
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with non-failing duplicate/license-not-encountered
  warnings already present in policy output.
