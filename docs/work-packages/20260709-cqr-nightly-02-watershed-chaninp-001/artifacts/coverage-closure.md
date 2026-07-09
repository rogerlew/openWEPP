# Coverage Closure

Status: `COMPLETE`

ADR-0021 tier: `science`.

Target:
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs`

Closure summary:

- Full-workspace LCOV target line coverage: `1891 / 1975 =
  95.746835443038%`.
- Full-workspace JSON target source-region coverage:
  `2431 / 2536 = 95.8596214511041%` including module-local tests.
- Production/source-helper source-region coverage:
  `1517 / 1610 = 94.22360248447205%`.
- Unique target CRAP rows after implementation: `33`.
- Target rows with CRAP above `30`: `0`.
- Lowest cargo-crap function coverage row:
  `sample_riser_unsubmerged_curve`, line `578`, coverage
  `76.36363636363637%`, CRAP `20.816276483846725`.
- No target function is below the ADR-0021 75% function-coverage floor.
- No production/source-helper function is below the ADR-0021 75%
  source-region floor. The lowest production/source-helper source-region row is
  `sample_riser_unsubmerged_curve` at `69 / 92 = 75.0%`.
- Module-local test helper functions are not production floor subjects; their
  aggregate module-local test/helper region coverage is
  `914 / 926 = 98.70410367170626%`.

`SC-IMPOUND-001` obligation binding:

| Vector | Evidence |
|---|---|
| Nominal continuity/stage-discharge vector | Existing `wshedw5_typed_watershed_runtime_contract.rs` runtime impoundment coverage; not modified by this CQR package. |
| Active-projection vector | New module-local tests `active_impoundment_projection_covers_all_function_families`, `drop_spillway_ids2_and_ids3_project_function_families`, and `emergency_open_channel_and_filter_modes_are_projected`; existing integration test `typed_frame_active_impoundment_matches_drop_spillway_min_controller_composition`. |
| Missing/non-finite/domain guard vectors | New package-local guard tests plus existing integration tests `typed_frame_impoundment_projection_preserves_non_finite_guard_class` and `typed_frame_impoundment_projection_preserves_domain_guard_class`. |
| Surrogate-deauthorization vector | Not changed; existing WS12 runtime integration evidence remains the authority. |

Disposition:

- Science-tier closure is satisfied for this package’s CQR scope.
- The `laned_shadow_h2637` and `openwepp-hillslope-orchestrator --lib`
  failures observed inside `cargo llvm-cov --ignore-run-fail` are not
  target-module coverage closure failures; the same full nextest workflow
  passed in the required gate.
