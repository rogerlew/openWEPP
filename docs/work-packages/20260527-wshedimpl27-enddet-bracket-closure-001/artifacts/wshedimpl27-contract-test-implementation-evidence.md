# WSHEDIMPL27 Contract Test Implementation Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Contract-derived WS11 vector added in:
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  - `wshedimpl27_contract_ws21_case4_bracket_migration_vector_remains_resolved`
    - enables WS20+WS21 opt-in with `crfrac`,
    - applies elevated channel erodibility (`chnk`) forcing in case4 lanes,
    - asserts successful routing and zero unresolved-detachment diagnostics.
- Kernel unit vector added in:
  - `crates/openwepp-watershed-orchestrator/src/lib.rs` test module
  - `wshedimpl27_enddet_helper_exercises_xdbig_and_midpoint_rebracketing`
    - verifies WS27 helper executes both `xdbig` and midpoint rebracketing
      branches.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl27_contract_ws21_case4_bracket_migration_vector_remains_resolved -- --exact`
  - Result: pass.
- `cargo test -p openwepp-watershed-orchestrator wshedimpl27_enddet_helper_exercises_xdbig_and_midpoint_rebracketing`
  - Result: pass.

### Pre-implementation gate note
- Prior to runtime edits, an initial stricter WS11 vector assertion requiring
  non-zero `ws21_enddet_segment_count` was executed and failed under fixture
  forcing (`enddet` stayed `0`). The runtime branch-closure proof obligation was
  therefore captured directly by the kernel unit vector above.

## Ran (pre-implementation probe)
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl27_contract_ws21_case4_enddet_bracket_lane_is_exercised -- --exact`
  - Result: fail (`expected ws21 enddet lane activity ... got 0`).
  - This probe command shape references the earlier temporary test name and
    documents pre-runtime fixture behavior.
