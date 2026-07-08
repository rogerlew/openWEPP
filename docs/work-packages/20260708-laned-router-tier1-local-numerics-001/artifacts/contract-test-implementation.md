# Contract Test Implementation

Status: `EXECUTED`

Added rev-47 unit coverage in
`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`:

- `rev47_depth_power_uses_sqrt_identity`
- `rev47_dust_residual_floor_matches_contract`
- `rev47_dry_and_zero_slope_cells_are_zero_local_hydraulics`
- `rev47_manning_celerity_uses_five_thirds_q_over_h`
- `rev47_laminar_skin_celerity_uses_three_q_over_h`
- `rev47_hirsch_skin_celerity_uses_exact_turbulent_pow`
- `rev47_pure_skin_branch_gap_uses_pre_step_branch_without_smoothing`
- `rev47_additive_menu_celerity_matches_small_finite_difference`
- `rev47_active_vegetation_nonfinite_local_numerics_fail_closed`

The pre-implementation gate failed as expected before the solver changes
because `depth_pow_3_2` and `alpha_q_celerity` did not exist yet. The final
post-implementation selector passes with `26` kinematic-wave tests, including
the rev-47 dry/zero-slope, pure-skin branch-gap, exact Hirsch pow,
active-vegetation non-finite, additive finite-difference, and dust-floor
vectors.
