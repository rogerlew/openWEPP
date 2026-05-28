# WSHEDIMPL28 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Runtime implementation updates:
  - File: `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - `ws20_route_case12_segment_family` now ingests boundary-width arrays as:
    - `wida` symbols -> lower-boundary width vector,
    - `widb` symbols -> upper-boundary width vector.
  - Segment-loop hydraulic calls now preserve baseline `chnrt.for` semantics:
    - upper boundary width: `widb(i-1)`
    - lower boundary width: `wida(i)`
  - zero-flow hydraulic fallbacks now use the same boundary-correct widths.
- Contract-derived test update:
  - File: `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  - Added `wshedimpl28_contract_ws20_routing_responds_to_wida_lower_boundary_widths`.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl28_contract_ws20_routing_responds_to_wida_lower_boundary_widths -- --exact` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` -> pass (28/28)
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (duplicate/license-not-encountered warnings only)
