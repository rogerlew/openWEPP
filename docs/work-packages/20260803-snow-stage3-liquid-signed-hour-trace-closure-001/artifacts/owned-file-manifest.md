# Owned File Manifest

Status: `terminal / 73 paths / PASS`

Evidence mode: `Static`

The terminal diff is confined to the amended write set: five diagnostic
publication Rust modules; contract, index, roadmap, and catalog; three typed
assurance outputs; 36 integration tests comprising the substantive EB-04W
contract target plus mechanical v123 pin reconciliation; and 25 package-local
artifact, prompt-lifecycle, and tool paths. Disposable evidence under `target/`
is intentionally untracked.

Exact terminal path list relative to scaffold commit `48d89081`:

```text
assurance/v2/identity.lock.json
assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json
assurance/v2/transactions/ac9ae76f8a62d4563363d11442e68661f68bb847fc382be7594f821c50405e1f.json
crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00f_snow_accumulation_melt_trace.rs
docs/planning/snow-surface-energy-balance-roadmap.md
docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
docs/specifications/science-contracts/index.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/README.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/contract-implementation-evidence.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/contract-test-implementation-evidence.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/disposition.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/gate-results.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/implementation-test-evidence.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/kernel-profile-compliance.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/line-count-governance.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/operand-lineage.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/owned-file-manifest.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/pre-implementation-contract-gate.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/real-consumer-reconstruction.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/review-agent-a.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/review-agent-b.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/review-disposition.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/verification-agent-a.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/verification-agent-b.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/artifacts/worker-handoff.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/package.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/prompts/README.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/prompts/active/20260803_execute_trace_closure_kickoff_agent_prompt.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/prompts/active/README.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/prompts/archived/20260803_execute_trace_closure_kickoff_agent_prompt.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/prompts/archived/README.md
docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/tools/trace_closure.py
docs/work-packages/README.md
tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs
tests/integration/paradigm2_multilayer_promotion.rs
tests/integration/paradigm2_stage1_layered_snow_density.rs
tests/integration/paradigm2_stage2_snow_frost_insulation_profile.rs
tests/integration/paradigm2_stage3_decouple_water_temperature.rs
tests/integration/paradigm2_stage3_liquid_routing_meltwater_temperature.rs
tests/integration/snow_surface_eb03_contract.rs
tests/integration/snow_surface_eb04v_density_process_diagnostics_contract.rs
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

The active prompt path is a deletion and the archived prompt path is its
byte-identical addition. No fixture, observation, historical evidence, public
output, or other protected file is present.
