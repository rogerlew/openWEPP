# Strip Inventory

Status: EXECUTED-INVENTORY. Evidence mode: Static.

Source authority:

- `SC-OFEROUTE-002` rev 5, read before deletion.
- ADR-0037 decision item 4: tests retire with the abandoned code.
- D16 viability review CL-M3: deprecation is contract lifecycle,
  code removal/quarantine, and test retirement together.

## Contract Surfaces

Remove from main:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`.
- The `SC-OFEROUTE-001` live hybrid Branch/Guard row.
- The `SC-OFEROUTE-001` hybrid Test-Vector Obligation row.
- The `SC-OFEROUTE-001` `OFEROUTE-HYBRID-IMPLICIT-STEPPING` BEI row.
- The `SC-OFEROUTE-002` registry row's active lifecycle posture; set the
  registry status to `withdrawn` and point notes at ADR-0037 plus the
  archive branch.

Keep:

- `SC-OFEROUTE-001` historical revision-history rows 28-35.
- All hybrid work-package directories and execution-log history.
- Plain `INV-OFEROUTE-011` Case-4 oracle surface.

## Code Surfaces

Delete or remove:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`.
- `pub mod implicit_recession` from `ofe_routing.rs`.
- `ofe_routing::cascade::route_single_ofe_hybrid`.
- `ofe_routing::cascade::hybrid_implicit_eligibility_mask`.
- Source-memory cooldown, hour-partition, non-integral hybrid-window, and
  hybrid span-composition logic in `cascade.rs`.
- `absorb_deficit`, `dispose_terminal_carry`, and related hybrid
  cross-span carry helpers.
- `KinematicWaveSolver::run_with_options_deficit_carry` and returned carry
  plumbing from `kinematic_wave.rs`; keep only the fail-closed public
  `run_with_options`.
- Exact bare-skin direct-equilibrium evaluator and
  `CellParameters::is_bare_skin_only` unless a surviving plain path uses
  them.
- `ofe_routing::profile` implicit counters:
  `solver_steps_implicit`, `implicit_equilibrium_map_evaluations`,
  `implicit_branch_evaluations`, and their counter functions.
- Hybrid D-val harnesses in `dval.rs`:
  `run_iwagaki_manning_hybrid`,
  `run_iwagaki_hybrid_source_memory_explicit_end_s`,
  `run_iwagaki_cells_hybrid`, `run_iwagaki_hybrid_supply_phase`, and
  `append_iwagaki_hybrid_recession`.
- Direct-runtime selector plumbing:
  `LanedActiveConfig::hybrid_implicit`, request/selected/fallback counters,
  exact-bare-skin selection, and `route_single_ofe_hybrid` call sites.
- Runner selector plumbing:
  `OPENWEPP_LANED_ACTIVE_IMPLICIT`,
  `env_hybrid_implicit_enabled`,
  manifest `hybrid_implicit_stepping`, and requested/selected/fallback
  manifest fields.

Keep:

- Active plain owner path selected by `OPENWEPP_LANED_ACTIVE=1`.
- Default/off protected-output identity behavior.
- Shadow path and shadow profile behavior.
- Explicit-route profile counters:
  `solver_steps`, `solver_steps_homogeneous`,
  `solver_steps_source_free`, `alpha_evaluations`, and timing accumulators.
- Plain `case4_manning_solver_converges_to_iwagaki_oracle`.

## Test Surfaces To Retire

Remove:

- `implicit_step_ledger_is_exact_and_positive`.
- `implicit_step_books_upstream_inflow_exactly`.
- `dust_scale_steps_do_not_accumulate_a_material_leak`.
- `steady_state_is_a_fixed_point_of_the_implicit_step`.
- `branch_warm_seed_preserves_solution_and_reduces_or_matches_map_work`.
- `branch_warm_seed_acceptance_is_basin_locked`.
- `bare_skin_direct_equilibrium_matches_iterated_branch_values`.
- `bare_skin_direct_equilibrium_avoids_fixed_point_map_work`.
- `bare_skin_direct_equilibrium_composed_edge_cases_close_cell_residuals`.
- `implicit_step_rejects_invalid_inactive_raw_operands_before_direct_path`.
- `low_jump_recovers_high_branch_root_and_never_commits_filippov`.
- `hybrid_is_bit_identical_on_all_explicit_windows`.
- `hybrid_event_day_ledger_exact_and_fidelity_bounded`.
- `hybrid_rejects_non_integral_windows`.
- `hybrid_source_memory_cooldown_keeps_post_source_bins_explicit`.
- `hybrid_source_memory_allows_implicit_after_cooldown`.
- `hybrid_source_memory_resets_on_later_source_burst`.
- `hybrid_rejects_cadence_that_does_not_partition_the_seam_hour`.
- `hybrid_source_memory_routes_upstream_fed_zero_source_bins_implicitly`.
- `absorb_deficit_exact_total_and_non_negative`.
- `dispose_terminal_carry_material_deficit_fails_closed`.
- `dispose_terminal_carry_subnoise_absorbs_backward_exactly`.
- `dispose_terminal_carry_all_dry_subnoise_drop_is_bounded`.
- `bin_recorder_returns_material_terminal_deficit_exactly`.
- `recession_ladder_diagnostic`.
- `recession_converges_to_the_explicit_reference_under_dt_refinement`.
- `bare_skin_direct_equilibrium_tracks_effective_addends`.
- `bare_skin_direct_equilibrium_does_not_authorize_invalid_raw_operands`.
- `case4_hybrid_manning_ladder_meets_iwagaki_oracle`.
- Direct-runtime no-harm selector vectors:
  `hybrid_request_selects_exact_bare_skin_lane_day` and
  `hybrid_request_falls_back_to_plain_on_post_growth_vegetation`.

## Env-Var Posture

`OPENWEPP_LANED_ACTIVE_IMPLICIT` is no longer a selector. The removal
implementation must reject the variable when present with a typed runner
startup error that names ADR-0037. Silent ignore is not used because it
would hide an abandoned production request.
