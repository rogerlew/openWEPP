# WSHEDIMPL26 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Runtime implementation updates:
  - File: `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - Added `WS22_DCAP_MAXE` and refactored `dcap` helper into
    `ws26_dcap(...)` with explicit `flagm` parameter.
  - Implemented `flagm != 1` max-detachment clipping behavior (`maxe`) in:
    - shallow-detachment branch (`timpot >= timsh`),
    - `difsh <= 0` branch after `shdist`,
    - erosion branch (`eros/dct`) with capped recomputation.
  - Updated call sites:
    - WS20 upper-boundary detachment lane calls `ws26_dcap(..., flagm=1, ...)`.
    - WS23 iterative detach closure initial call uses `flagm=1`.
    - WS23 iterative excess loop uses `flagm=2`.
- Contract-derived tests implemented:
  - WS11 integration stress vector.
  - Kernel unit test for `flagm=2` cap behavior.

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` -> pass
- `cargo test -p openwepp-watershed-orchestrator wshedimpl26_dcap_flagm2_caps_detachment_rate_at_maxe` -> pass
- `cargo test --workspace -q` -> pass
- `cargo deny check` -> pass (non-failing duplicate/license-not-encountered
  warnings only)
