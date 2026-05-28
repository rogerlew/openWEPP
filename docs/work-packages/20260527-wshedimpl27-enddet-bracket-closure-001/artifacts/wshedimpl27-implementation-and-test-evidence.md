# WSHEDIMPL27 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Runtime implementation updates:
  - File: `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - Added `ws27_case4_enddet_bracket_closure(...)` helper to preserve
    baseline-authoritative enddet bracket progression sequencing.
  - WS27 helper now executes both closure branches:
    - midpoint rebracket (`nt < class_count`):
      `xdsmal = xdbmin`, `xdbmin = (xdsmal + xdbig) / 2`
    - upper-bracket rebracket (`nt == class_count`):
      `xdbig = xdbmin` followed by recomputed `xdbeg` path.
  - Updated WS20 case4 enddet lane call site to invoke the WS27 helper and
    reuse existing transport-capacity evaluation (`ws18_trncap`) without
    changing typed guard posture.
- Contract-derived tests implemented:
  - WS11 case4 resolved-bracket migration vector.
  - Kernel unit vector proving both WS27 rebracket branches execute.

## Ran
- `cargo fmt --check` -> fail, then pass after `cargo fmt`
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl27_contract_ws21_case4_bracket_migration_vector_remains_resolved -- --exact` -> pass
- `cargo test -p openwepp-watershed-orchestrator wshedimpl27_enddet_helper_exercises_xdbig_and_midpoint_rebracketing` -> pass
- `cargo test --workspace -q` -> pass
- `cargo deny check` -> pass (non-failing duplicate/license-not-encountered
  warnings only)
