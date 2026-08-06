# Owned File Manifest

Evidence class: `Static`.

Baseline: `b966d5d86316d15ef409f933ecd7ee011d53514a`.

The closure-candidate diff owns exactly 94 paths: 1 workspace manifest, 4
assurance custody/transaction paths, 16 Rust implementation paths, 5
non-package documentation paths, 30 package paths, and 38 integration-test
paths. The two additive wrapper/export files and publication-parity test are the
review-driven write-set amendment recorded prospectively in `package.md`.

Exact paths:

```text
Cargo.toml
assurance/v2/identity.lock.json
assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json
assurance/v2/transactions/a9084c0d79287206ae62cd1a122b78009f8618f4cf206c4dabce5b957e07da23.json
assurance/v2/transactions/cd73c2bbbb618c4dde76b28d4b607919f183dbae4f4e552f9faf8bcd8b271171.json
crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/snow_mass_transition.rs
crates/openwepp-hillslope-orchestrator/src/lib.rs
crates/openwepp-runner/src/hillslope/03_tests.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00f_snow_accumulation_melt_trace.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00g_snow_diagnostic_capture.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00h_snow_stage3_evaluation_trace.rs
crates/openwepp-runner/src/hillslope/tests03/stage3_evaluation_publication_parity.rs
docs/ROADMAP.md
docs/planning/snow-surface-energy-balance-roadmap.md
docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
docs/specifications/science-contracts/index.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/README.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/assurance-source-adoption.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/authority-freeze.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/contract-implementation-evidence.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/contract-test-implementation-evidence.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/disposition.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/exact-diff-reconciliation.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/gate-results.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/implementation-test-evidence.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/kernel-profile-compliance.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/line-count-governance.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/modularization-plan.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/operand-lineage.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/owned-file-manifest.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/pre-implementation-contract-gate.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/public-api-parity.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/required-reading-map.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/review-disposition.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/review_agent_a.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/review_agent_b.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/review_agent_qa.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/security-impact.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/verification_agent_a.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/verification_agent_b.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/artifacts/worker-handoff.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/package.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/prompts/README.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/prompts/active/README.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/prompts/archived/20260806-snow-stage3-shadow-solver-extraction-and-observability-001_kickoff_agent_prompt.md
docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/prompts/archived/README.md
docs/work-packages/README.md
tests/integration/paradigm2_multilayer_promotion.rs
tests/integration/paradigm2_stage1_layered_snow_density.rs
tests/integration/paradigm2_stage2_snow_frost_insulation_profile.rs
tests/integration/paradigm2_stage3_decouple_water_temperature.rs
tests/integration/paradigm2_stage3_liquid_routing_meltwater_temperature.rs
tests/integration/snow_mass_transition_ledger_persistence_contract.rs
tests/integration/snow_stage3_evaluation_shadow_authority_contract.rs
tests/integration/snow_stage3_shadow_observability_contract.rs
tests/integration/snow_surface_eb03_contract.rs
tests/integration/snow_surface_eb03_runtime.rs
tests/integration/snow_surface_eb04w_accumulation_melt_diagnostics_contract.rs
tests/integration/snowdensity02_contract_adr_guard.rs
tests/integration/snowdensity05a_melt_contract_guard.rs
tests/integration/snowdensity05b_shortwave_source_contract.rs
tests/integration/snowdensity05c_albedo_state_core.rs
tests/integration/snowdensity05d_opt_in_coe_melt.rs
tests/integration/snowdensity05f_melt_closure_handoff.rs
tests/integration/snowdensity05g_harness_fidelity_rerun.rs
tests/integration/snowdensity06_density_compaction.rs
tests/integration/snowdensity06b_coe_bound_density_replay.rs
tests/integration/snowdensity07_runtime_opt_in.rs
tests/integration/snowdensity08_gate_rerun.rs
tests/integration/snowdensity09_coupled_wat_rerun.rs
tests/integration/snowdensity10_3_11_spring_compaction_densification.rs
tests/integration/snowdensity10_3_12_bundle_activation_adjudication.rs
tests/integration/snowdensity10_3_13_residual_policy_b_diagnostic.rs
tests/integration/snowdensity10_3_14_policy_b_no_regression_cap_authority.rs
tests/integration/snowdensity10_3_15_default_activation_active_cap.rs
tests/integration/snowdensity10_3_16_open_surface_ablation_stage_a.rs
tests/integration/snowdensity10_3_17_shallow_pack_compaction_guard.rs
tests/integration/snowdensity10_3_19_harder_pomeroy_default_activation.rs
tests/integration/snowdensity10_3_1a_per_day_cancov.rs
tests/integration/snowdensity10_3_20_sublimation_stage_b_unlock.rs
tests/integration/snowdensity10_3_22_climate_class_density_specialization.rs
tests/integration/snowdensity10_3_5a_meteorology_crate_contract.rs
tests/integration/snowdensity10_3_5b_hourly_partition_jennings_contract.rs
tests/integration/snowdensity10_3_7_winter_thaw_melt_response_correction.rs
tests/integration/snowdensity10_3_8_liquid_holding_capacity.rs
```

Terminal reconciliation command:

```bash
comm -3 \
  <(git diff --name-only b966d5d86316d15ef409f933ecd7ee011d53514a..HEAD | sort) \
  <(sed -n '/^```text$/,/^```$/p' artifacts/owned-file-manifest.md | sed '1d;$d' | sort)
```

Acceptance is empty output at the exact clean terminal commit.
