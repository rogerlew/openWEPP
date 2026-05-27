# WSHEDIMPL21 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented in `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - Added WS21 opt-in control symbol handling:
    - `ws10_channel_{id}_ws21_case34_enable`
  - Extended routing diagnostics/publication models with WS21 fields:
    - `ws21_case3_segment_count`
    - `ws21_case4_segment_count`
    - `ws21_enddet_segment_count`
    - `ws21_detach_unmigrated_segment_count`
  - Added typed opt-in toggle reader used for both WS20 and WS21 toggles.
  - Extended WS20 segment-loop routing path so the detachment-unmigrated branch:
    - classifies WS21 opt-in segments as case3/case4 candidates,
    - publishes explicit WS21 unresolved detach/dcap visibility, and
    - preserves fail-visible fallback behavior (no silent surrogate detach/dcap).
  - Preserved existing WS20 diagnostics publication:
    - `ws20_case1_segment_count`
    - `ws20_case2_segment_count`
    - `ws20_detachment_unmigrated_segment_count`
- Added WS11 integration vectors for WS21 default-off and WS20+WS21 opt-in
  diagnostics.
- Full baseline-authoritative `detach/dcap` production math is intentionally not
  closed in this package and remains explicit HOLD scope.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` passed.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed (warnings-only duplicate/license-allowance notices).
