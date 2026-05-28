# WSHEDIMPL41 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Runtime parity edits completed in
  `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - split WS11 branch selector so `ipeak=5` routes to
    `Ws11IpeakBranch::MuskingumCungeVariable`,
  - added MVPMC3 helper lineage for geometry/discharge/depth solve:
    `require_ws11_channel_length_from_scaffold`,
    `ws11_muskingum_geometry_from_depth`,
    `ws11_manning_discharge_for_depth`,
    `ws11_solve_depth_for_discharge`,
  - added `compute_variable_muskingum_cunge_state` implementing dynamic
    reference-flow lineage (`qref = (qin + qin_previous + q1_previous) / 3`)
    and per-step dynamic `c0..c4` refresh under typed fail-closed guards,
  - wired `run_channel_node` to use scaffold-derived channel length and
    `ishape` when executing `ipeak=5` dynamic branch.
- Contract and index updates are consistent with runtime/test outcomes.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl41_ -- --nocapture` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` -> pass
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (`advisories ok, bans ok, licenses ok, sources ok`;
  warnings-only duplicate crate-version and unmatched-license-allowance notices)
