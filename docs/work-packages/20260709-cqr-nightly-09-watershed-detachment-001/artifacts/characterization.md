# Characterization

Evidence label: Static/Ran.

Status: `EXECUTED`

Scaffold boundary:

- Scaffold commit: `2e6d3a5a Scaffold CQR nightly watershed detachment package`
- Characterization edits occurred after the scaffold commit.

Characterization added fourteen `#[cfg(test)]` tests inside
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs`:

- `wshedimpl20_fall_velocity_and_shdist_cover_boundaries`
- `wshedimpl22_table_lookup_characterizes_direction_and_bounds`
- `wshedimpl23_detach_rejects_invalid_validation_inputs`
- `wshedimpl23_detach_case4_closure_path_is_finite`
- `wshedimpl23_detach_case4_iterative_loop_low_shear_is_characterized`
- `wshedimpl23_detach_leaf_helpers_characterize_sums_and_flux_guards`
- `wshedimpl23_detach_start_and_iteration_helpers_cover_terminals`
- `wshedimpl24_transition_rejects_nonpositive_length`
- `wshedimpl26_dcap_characterizes_expanding_width_path`
- `wshedimpl26_dcap_midlayer_step_characterizes_terminals_and_caps`
- `wshedimpl26_dcap_expanding_width_characterizes_terminal_and_cap_paths`
- `wshedimpl26_dcap_low_width_shear_outcome_characterizes_terminals`
- `wshedimpl27_enddet_bracket_characterizes_iteration_cap`
- `wshedimpl30_shape_and_rectangular_fallback_characterize_guards`

Existing focused tests retained:

- `wshedimpl26_dcap_flagm2_caps_detachment_rate_at_maxe`
- `wshedimpl27_enddet_helper_exercises_xdbig_and_midpoint_rebracketing`

Behavior oracle:

- WS20 fall-velocity and WS22 shear-distribution helpers preserve exact zero,
  Stokes, interpolated-table, tail-table, low-ratio, threshold, and upper-ratio
  values;
- table lookup returns `None` for invalid cardinality and out-of-range values,
  and interpolates increasing/decreasing column-2 tables without changing
  interpolation math;
- WS30 shape parsing preserves exact accepted flags and fail-closed
  domain-violation identity for fractional/out-of-range shape values;
- WS24 transition closure fails closed for nonpositive remaining length with
  channel domain-violation identity;
- WS23 validation fails closed for class-cardinality mismatch and nonpositive
  `dx_ft`, with channel domain-violation message and boundary class preserved;
- WS23 case-4 closure returns exact characterized class fluxes
  `[7.315_907_199_757_94, 10.898_860_799_636_909,
  18.084_767_999_394_845]` and erodible width
  `8.256_153_333_081_187`;
- WS23 low-shear case-4 iterative loop returns exact characterized class fluxes
  `[0.010_271_239_667_832_036, 0.007_376_358_644_761_355,
  0.006_762_460_067_254_847]` and erodible width `1.0`;
- WS23 leaf helpers preserve exact potential-load vector `[0.16, 0.195]`,
  transport sums (`sumtcl=0`, `sumpld=0.355`, `sumdf=0.03`,
  `sumexd=-0.041`), final-flux vector `[0.03, 0.04]`, and guard behavior;
- WS23 initial/iteration helpers preserve complete and iterative starts plus
  finite low-shear transport/detachment sums;
- WS26 expanding-width path returns exact class detachment
  `[2.633_526_687_031_146_5, 3.950_290_030_546_719,
  6.583_816_717_577_865]`, depth `0.25`, and erodible width
  `14.466_284_828_287_218`;
- WS26 midlayer and expanding-width helpers preserve exact cap allocation,
  empty/zero/low-shear terminals, depleted-layer clamping, low-ad terminal
  return, and already-wide terminal return behavior;
- WS26 low-width-shear terminal branches preserve no-detachment and
  class-fraction allocation behavior with exact `[0.004, 0.006]`
  allocation for `[0.4, 0.6]`.
- WS27 end-detachment bracketing preserves the iteration-cap terminal path.

Commands:

- `cargo test -p openwepp-watershed-orchestrator --lib wshedimpl -- --nocapture`
  - Result: PASS, `16` passed, `0` failed, `66` filtered.
- `cargo fmt --check`
  - Result: PASS.
- `cargo clippy -p openwepp-watershed-orchestrator --lib --tests -- -D warnings`
  - Result: PASS.
