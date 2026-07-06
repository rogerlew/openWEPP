# Implementation Test Evidence

Status: **COMPLETE**.

Ran:

- `cargo check -q -p openwepp-hillslope-orchestrator -p openwepp-runner`:
  PASS.
- `cargo test -q -p openwepp-hillslope-orchestrator active_snow_hourly_routed_melt_preserves_shape_and_closes_daily_scalar`:
  PASS, 1 test.
- `cargo test -q -p openwepp-hillslope-orchestrator dc01_surface_shape`:
  PASS, 4 tests.
- `cargo test -q -p openwepp-hillslope-orchestrator r4g_rejects_hourly_routed_melt_daily_nonclosure`:
  PASS, 1 test.
- `cargo test -q -p openwepp-runner laned_shadow_dynamic_operand_tests`:
  PASS, 3 tests.
- `cargo test -q --test snowdensity05d_opt_in_coe_melt snowdensity05d_opt_in_changes_only_shortwave_amelt_operand`:
  PASS, 1 test.
- `cargo test -q --test laned_shadow_h2637 h2637_legacy_shadow_fails_closed_without_routing_coefficients`:
  PASS, 1 test.
- `cargo test --test laned_shadow_h2637 h2637_native_shadow_classifies_uniform_shape_after_d12 -- --ignored --nocapture`:
  PASS, 1 ignored evidence test, 324.83 s
  (`/tmp/laned_shadow_h2637_native_on_10668/manifest.json`).

Layout/governance follow-up:

- `cargo test -q -p openwepp-hillslope-orchestrator r7b_constructor_type_size_layout_is_bounded -- --nocapture`:
  PASS after removing duplicate hourly routed-melt vector storage from
  `DirectSnowCouplingState`/`DirectSnowCouplingShadowProjection` and boxing the
  downstream operand vector; `DirectDayFrame=15328`, bound `15456`.
- `cargo test -q -p openwepp-hillslope-orchestrator dc01_surface_shape`:
  PASS after moving DC01 tests to `direct_runtime_dc01.rs`.
- `cargo test -q -p openwepp-hillslope-orchestrator r4g_rejects_hourly_routed_melt_daily_nonclosure`:
  PASS after moving DC01/R4G tests to `direct_runtime_dc01.rs`.
