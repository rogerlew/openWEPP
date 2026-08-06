# Exact Diff Reconciliation

Evidence class: `Static + Ran`

Baseline: `71521161afcc85f06098bb76205302bf54badf9b`.
Heavy-validation head: `e601f0f966c1531fb95bb81e304a23bd3044a1ab`.
Terminal path count: `84`.

The final baseline-to-terminal diff contains exactly one Cargo registration,
nine assurance paths, five non-package documentation paths, 33 package paths,
and 36 integration-test paths. No production crate source, fixture, reference,
configuration, public schema, selector, default, observation, consumer, or
runtime path changed.

## Cargo Registration

```text
Cargo.toml
```

The only change registers the new static integration-test target.

## Assurance Paths

```text
assurance/v2/identity.lock.json
assurance/v2/reports/snow-and-frozen-soil-process-evaluation/evidence/agent-assistance-packet.json
assurance/v2/reports/snow-and-frozen-soil-process-evaluation/manuscript.md
assurance/v2/reports/snow-and-frozen-soil-process-evaluation/report.yaml
assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json
assurance/v2/reports/snow-and-frozen-soil-process-evaluation/supplement.md
assurance/v2/transactions/095ab87aeff87aac7cd966b2f3c066a140d3d6ef2d67ebcf032c0376bdef1e68.json
assurance/v2/transactions/b2e9d32d40779a135e7891ee955b03e1c25103ad759b07dc8e8ce677fdbcd8c4.json
assurance/v2/transactions/e30ab158aebb6ad1109442cd43ca8cbc3550b3c09260fe9bb19f5c77fa948c32.json
```

Typed adoption tooling included the linear-groundwater and forest report review
locks in its tool-affected set, but their baseline and terminal Git blobs and
science/communication roots are identical. They are not actual byte changes
and therefore do not appear in this terminal path inventory. No immutable
review-event byte changed.

## Contract, Roadmap, And Catalog Paths

```text
docs/ROADMAP.md
docs/planning/snow-surface-energy-balance-roadmap.md
docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
docs/specifications/science-contracts/index.md
docs/work-packages/README.md
```

## Work-Package Paths

```text
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/README.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/assurance-source-adoption.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/authority-freeze.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/contract-implementation-evidence.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/contract-test-implementation-evidence.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/disposition.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/exact-diff-reconciliation.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/gate-results.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/implementation-test-evidence.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/kernel-profile-compliance.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/line-count-governance.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/owned-file-manifest.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/pre-implementation-contract-gate.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/required-reading-map.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/review-disposition.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/review_agent_a.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/review_agent_b.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/review_agent_qa.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/science-contracts/SC-SNOWFREEZE-001/contract_ref.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/science-contracts/SC-SNOWFREEZE-001/disposition.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/science-contracts/SC-SNOWFREEZE-001/review_agent_a.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/science-contracts/SC-SNOWFREEZE-001/review_agent_b.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/science-contracts/SC-SNOWFREEZE-001/verification_agent_a.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/science-contracts/SC-SNOWFREEZE-001/verification_agent_b.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/security-impact.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/verification_agent_a.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/verification_agent_b.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/artifacts/worker-handoff.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/package.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/prompts/README.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/prompts/active/README.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/prompts/archived/20260806-snow-stage3-evaluation-shadow-authority-001_kickoff_agent_prompt.md
docs/work-packages/20260806-snow-stage3-evaluation-shadow-authority-001/prompts/archived/README.md
```

The kickoff prompt is added only at its final archived path relative to the
package baseline. Its SHA-256 is
`bf2f05c09b7e70ce6a2f52e64b6e4c3e035e082e4af46de08d12917d7dc16e79`,
identical to the scaffolded active prompt.

## Integration-Test Paths

```text
tests/integration/paradigm2_multilayer_promotion.rs
tests/integration/paradigm2_stage1_layered_snow_density.rs
tests/integration/paradigm2_stage2_snow_frost_insulation_profile.rs
tests/integration/paradigm2_stage3_decouple_water_temperature.rs
tests/integration/paradigm2_stage3_liquid_routing_meltwater_temperature.rs
tests/integration/snow_mass_transition_ledger_persistence_contract.rs
tests/integration/snow_stage3_evaluation_shadow_authority_contract.rs
tests/integration/snow_surface_eb03_contract.rs
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

The 35 pre-existing test files contain only mechanical v126-to-v127 exact
version substitutions. The new 130-line test is static contract/assurance
authority enforcement. No test changes production behavior.

## Terminal Reconciliation

The 84-path inventory matches the declared and review-amended write set. The
only tool-affected but byte-identical peer-report locks are excluded explicitly.
No unknown or unauthorized path remains. Q3 is closed. All changes after clean
heavy-validation head `e601f0f9` are documentation and byte-identical prompt
archival; no heavy-gate input changed.
