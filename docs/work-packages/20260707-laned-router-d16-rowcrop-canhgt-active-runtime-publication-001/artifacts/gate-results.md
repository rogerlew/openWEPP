# Gate Results

Status: COMPLETE

| Gate | Status | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran after package closure edits; no whitespace errors. |
| Markdown/doc lint for touched docs | PASS | `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-PLANT-001.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/work-packages/README.md --path docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001` passed. |
| Contract/profile/BEI checks for touched `SC-*` contracts | PASS | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-PLANT-001.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` returned `PASS: SC unit compliance lint found no findings`. |
| Focused growth/canopy-height tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator active_route_uses_post_growth_canhgt_not_static_lane_config r5d_annual_growth_phase_computes_mutates_downstream_shadow_and_r4n_context r5d_perennial_growth_phase_supports_grazing_after_annual_phase_identity` passed. |
| Focused Lane D active operand-source tests | PASS | `cargo nextest run -p openwepp-runner laned_shadow_dynamic_operands_reject_missing_canhgt_when_lai_positive cqr_row7_growth_crop_and_surface_litter_projection_cover_schedule_paths laned_shadow_consumes_live_dynamic_friction_operands` passed. |
| P61 erosion guard after scope correction | PASS | `cargo nextest run --test erosion_single_ofe_p61_sediment` passed after reverting the out-of-scope erosion consumer change. |
| Selected-cohort active plain rerun through former blocker | PASS | `mn_corn_h4` active plain completed with exit `0` and `0:00.58` wall time. |
| Selected-cohort active plain/hybrid suite | PASS | Eight selected runs passed: four active plain and four active explicit hybrid. See `active-suite-run-summary.md`. |
| Protected-output/default-off isolation evidence | N/A | Runtime selector defaults were not touched. The fix is behind active Lane D operand consumption and preserves fail-closed active guards. |
| `cargo fmt --check` | PASS | Ran and passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran and passed. |
| `cargo nextest run --workspace --profile full` | PASS | Ran after code fixes; `1440 tests run: 1440 passed, 4 skipped`. |
| `cargo deny check` | PASS | Ran and passed: advisories, bans, licenses, and sources ok. |
| `.rs` line-count governance | PASS-WARN | Touched Rust files are all below the 3000-line hard threshold. WARN files: `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` at 2996 lines, `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs` at 2619 lines, and `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` at 2746 lines. The 2996-line test module is the first follow-on split candidate for future edits. |
