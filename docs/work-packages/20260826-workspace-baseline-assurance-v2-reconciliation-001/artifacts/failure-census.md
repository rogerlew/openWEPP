# Intake failure census and disposition

Evidence class: `Ran` for the historical exact-clean log and focused reruns;
`Static` for source/hash classification.

## Intake basis

Ran: the retained exact-clean workspace log from `9b1105d` records 101
failures. The exact identity partition is 81 Assurance V2 identity/source
failures, nine retained source/registry guards, and the historical eleven
orchestrator failures. The later `97accd99` comparator adds only the separately
classified V9 external-runtime failure.

Static: every Assurance failure below has the same bounded cause and
disposition: the production Assurance graph correctly remained bound to the
released canonical contract bytes at `43cc9bbe`, while four canonical files
and their registry rows had been replaced by rejected research-candidate text.
The identity lock and Assurance fixtures were therefore preserved. Restoring
the four released contract objects and four released registry rows exactly
disposed every listed failure without rebinding candidate bytes.

## 81 Assurance V2 failures

Each numbered row is individually retained from the exact-clean failure-name
census and receives the common source-drift classification above.

1. `openwepp-assurance cli::tests::amendment_recovery_command_reaches_typed_backend`
2. `openwepp-assurance cli::tests::inspect_and_lifecycle_commands_cover_human_json_and_request_paths`
3. `openwepp-assurance v2::assembly::tests::post_install_source_drift_restores_prior_bytes_without_timing`
4. `openwepp-assurance v2::transaction::tests::adoption_selected_source_race_preserves_the_active_generation`
5. `openwepp-assurance v2::transaction::tests::committed_cleanup_fault_leaves_new_generation_and_typed_recovery_state`
6. `openwepp-assurance v2::transaction::tests::every_precommit_fault_preserves_the_active_generation`
7. `openwepp-assurance v2::transaction::tests::recovery_tree_verifier_checks_the_selected_generation_members`
8. `openwepp::assurance_dossier_build_contract public_builder_stays_zero_report_while_validation_admits_internal_v2`
9. `openwepp::assurance_v2_amendment_contract attribution_check_is_read_only_and_apply_is_layer_proportional`
10. `openwepp::assurance_v2_amendment_contract implementation_rebind_adopts_only_the_finite_contract_surface`
11. `openwepp::assurance_v2_amendment_contract inspect_exposes_layered_identity_without_mutation`
12. `openwepp::assurance_v2_amendment_contract manifest_adoption_accepts_complete_owned_internal_source_set`
13. `openwepp::assurance_v2_amendment_contract manifest_adoption_admits_new_declared_external_local_content`
14. `openwepp::assurance_v2_amendment_contract manifest_adoption_rejects_drift_owned_by_another_report`
15. `openwepp::assurance_v2_amendment_contract manifest_adoption_rejects_invalid_new_external_sources_without_mutation`
16. `openwepp::assurance_v2_amendment_contract new_report_admission_checks_applies_and_repeats_as_no_op`
17. `openwepp::assurance_v2_amendment_contract production_generation_chain_and_recovery_inspection_are_current`
18. `openwepp::assurance_v2_amendment_contract report_source_adoption_rejects_noncanonical_unchanged_reset_repair`
19. `openwepp::assurance_v2_amendment_contract role_assignment_is_typed_receipted_and_idempotent`
20. `openwepp::assurance_v2_amendment_contract stale_optional_generation_rejects_check_and_apply_without_writes`
21. `openwepp::assurance_v2_assembly_contract canopy_named_and_all_builds_are_byte_equivalent_and_complete`
22. `openwepp::assurance_v2_assembly_contract malformed_duplicate_unsafe_link_and_inaccessible_figure_fail_closed`
23. `openwepp::assurance_v2_assembly_contract manifest_markdown_metadata_is_escaped_without_creating_external_links`
24. `openwepp::assurance_v2_assembly_contract mtime_changes_do_not_affect_assembly_bytes`
25. `openwepp::assurance_v2_assembly_contract real_cli_selects_v2_staging_without_weakening_zero_public_operations`
26. `openwepp::assurance_v2_assembly_contract real_named_and_all_builds_are_deterministic_equivalent_and_checkable`
27. `openwepp::assurance_v2_assembly_contract stale_missing_unit_precision_orphan_and_figure_drift_fail_closed`
28. `openwepp::assurance_v2_normalization_contract apply_rebinds_complete_graph_builds_and_is_idempotent`
29. `openwepp::assurance_v2_normalization_contract check_detects_converter_diff_without_writing`
30. `openwepp::assurance_v2_normalization_contract current_report_is_american_english_and_check_is_read_only`
31. `openwepp::assurance_v2_normalization_contract equivalent_inputs_emit_identical_receipts`
32. `openwepp::assurance_v2_normalization_contract lifecycle_review_and_packet_boundaries_fail_before_writing`
33. `openwepp::assurance_v2_planner_contract content_changes_select_expected_transitive_consumers`
34. `openwepp::assurance_v2_planner_contract current_one_and_all_plans_are_equivalent_stable_and_cli_consumed`
35. `openwepp::assurance_v2_planner_contract malformed_authority_remains_an_explicit_blocker`
36. `openwepp::assurance_v2_planner_contract manifest_method_figure_review_and_software_changes_select_the_report`
37. `openwepp::assurance_v2_planner_contract mtime_only_change_does_not_change_plan_bytes`
38. `openwepp::assurance_v2_planner_contract named_selection_isolated_and_all_plan_does_not_select_unrelated_report`
39. `openwepp::assurance_v2_planner_contract stale_consumer_cannot_mask_a_blocked_prerequisite`
40. `openwepp::assurance_v2_planner_contract unavailable_declared_content_blocks_consumers_with_relative_reasons`
41. `openwepp::assurance_v2_planner_contract unavailable_or_unparseable_selected_manifest_produces_a_bounded_blocked_plan`
42. `openwepp::assurance_v2_publication_contract aliased_usersum_root_fails_closed`
43. `openwepp::assurance_v2_publication_contract all_mode_extra_staging_and_overlapping_roots_fail_without_public_mutation`
44. `openwepp::assurance_v2_publication_contract approval_conflicts_and_release_mismatch_fail_before_publication`
45. `openwepp::assurance_v2_publication_contract authority_bound_byte_negative_matrix_is_fail_closed`
46. `openwepp::assurance_v2_publication_contract bootstrap_empty_directory_fails_closed`
47. `openwepp::assurance_v2_publication_contract bootstrap_unowned_readme_fails_closed`
48. `openwepp::assurance_v2_publication_contract canonical_public_path_and_real_markdown_narrative_link_are_mandatory`
49. `openwepp::assurance_v2_publication_contract concurrent_reader_observes_only_complete_old_or_new_report_bytes`
50. `openwepp::assurance_v2_publication_contract every_precommit_fault_boundary_preserves_the_prior_public_generation`
51. `openwepp::assurance_v2_publication_contract in_review_source_cannot_publish`
52. `openwepp::assurance_v2_publication_contract missing_competence_fails_closed`
53. `openwepp::assurance_v2_publication_contract missing_independence_fails_closed`
54. `openwepp::assurance_v2_publication_contract missing_release_transfer_fails_closed`
55. `openwepp::assurance_v2_publication_contract multi_report_production_snapshot_replays_complete_authority`
56. `openwepp::assurance_v2_publication_contract multiply_linked_staging_bytes_fail_before_publication`
57. `openwepp::assurance_v2_publication_contract named_publication_preserves_receipted_peer_and_all_mode_converges`
58. `openwepp::assurance_v2_publication_contract named_publication_rejects_unreceipted_prior_catalog_entries`
59. `openwepp::assurance_v2_publication_contract narrative_drift_fails_closed`
60. `openwepp::assurance_v2_publication_contract receipt_preparation_is_reused_only_when_bytes_match`
61. `openwepp::assurance_v2_publication_contract reconstructed_production_snapshot_passes_and_forged_roots_fail`
62. `openwepp::assurance_v2_publication_contract release_driver_persists_verified_v2_artifacts_and_discovery_sidecar`
63. `openwepp::assurance_v2_publication_contract simultaneous_publishers_serialize_and_converge_on_one_generation`
64. `openwepp::assurance_v2_publication_contract special_files_on_public_and_immutable_surfaces_fail_closed`
65. `openwepp::assurance_v2_publication_contract staging_fifo_fails_closed`
66. `openwepp::assurance_v2_publication_contract staging_symlink_fails_closed`
67. `openwepp::assurance_v2_publication_contract stale_roots_open_findings_conflicts_and_release_mismatch_fail_before_publication`
68. `openwepp::assurance_v2_publication_contract superseded_report_fails_closed`
69. `openwepp::assurance_v2_publication_contract synthetic_approved_fixture_publishes_idempotently_and_release_rejects_it`
70. `openwepp::assurance_v2_publication_contract withdrawn_report_fails_closed`
71. `openwepp::assurance_v2_publication_contract wrong_principal_kind_fails_closed`
72. `openwepp::assurance_v2_publication_contract wrong_principal_role_fails_closed`
73. `openwepp::assurance_v2_publication_contract wrong_principal_trust_domain_fails_closed`
74. `openwepp::assurance_v2_source_contract content_schema_contract_and_report_versions_are_enforced`
75. `openwepp::assurance_v2_source_contract every_record_family_has_executable_field_consumption`
76. `openwepp::assurance_v2_source_contract named_validation_isolated_from_an_unselected_broken_report`
77. `openwepp::assurance_v2_source_contract paths_symlinks_and_special_entries_fail_closed`
78. `openwepp::assurance_v2_source_contract real_source_and_cli_validate_named_and_all_deterministically`
79. `openwepp::assurance_v2_source_contract restricted_evidence_and_draft_lifecycle_contradictions_fail_closed`
80. `openwepp::assurance_v2_source_contract schema_required_nullable_fields_cannot_be_omitted`
81. `openwepp::assurance_v2_source_contract unknown_missing_duplicate_unresolved_and_unused_fields_fail_closed`

### Normalized intake signatures

Normalization removes elapsed time, ordinal, thread ID, source line/column,
and backtrace-note noise. Every numbered Assurance row maps to exactly one
signature below:

| Key | Normalized signature | Assigned Assurance rows |
|---|---|---|
| A | `called Result::unwrap() on an Err value: Drift("generated identity member changed: docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md")` | All rows except the rows explicitly assigned B--K below (68 rows). |
| B | `calculate implementation rebind: Drift("generated identity member changed: docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md")` | 1 |
| C | `copy exact v2 fixture: Drift("generated identity member changed: docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md")` | 3 |
| D | `copy fixture: Drift("generated identity member changed: docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md")` | 4, 5, 6, 7 |
| E | `run public and internal-source validate CLI: Invalid("SHA-256 mismatch for identified source 'docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md'")` | 8 |
| F | `build complete catalog: Invalid("report 'snow-and-frozen-soil-process-evaluation' is not current and cannot be assembled")` | 21 |
| G | `build all reports through real CLI: Invalid("report 'snow-and-frozen-soil-process-evaluation' is not current and cannot be assembled")` | 25 |
| H | `build all reports: Invalid("report 'snow-and-frozen-soil-process-evaluation' is not current and cannot be assembled")` | 26 |
| I | `current production DRAFT is normalized: Drift("generated identity member changed: docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md")` | 30 |
| J | `assertion left == right failed; left: Selected; right: Current` | 34 |
| K | `validate all sources: Invalid("SHA-256 mismatch for identified source 'docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md'")` | 78 |

Ran: after exact authority restoration, the complete `openwepp-assurance`
selection passed 32/32 (nextest run ID
`10823b8e-37ab-4133-a02a-c18a09109dba`) and the seven Assurance integration
binaries passed 109/109 with two configured skips (nextest run ID
`36ac01d2-f52f-4b16-a3dd-0920f6398036`). This covers all 81 intake failures
and their passing peers. No Assurance lock, fixture, report source, or expected
byte was changed.

## Nine retained guards

| # | Exact failing guard | Classification and disposition |
|---:|---|---|
| 1 | `paradigm2_stage0_surface_energy_balance_contract production_runtime_sources_only_wire_stage0_flux_primitives_through_stage3_opt_in` | Stale exact-path allowlist. Added only two proven Stage-3-only source paths; retained the global scan and every forbidden token. |
| 2 | `snow_stage3_legacy_predecessor_bridge_contract v130_protocol_and_registry_retain_claim_limits` | Stale unreleased SnowFreeze v137 assertion. Rebound to released v136 while retaining every asserted claim limit. |
| 3 | `snow_stage3_persistent_accumulation_shadow_contract v134_admits_exactly_one_mechanics_only_persistent_operator` | Stale unreleased SnowFreeze v137 assertion. Rebound to released v136; mechanics-only invariant unchanged. |
| 4 | `snow_stage3_shared_carrier_authority_contract snow_energy_v18_physical_custody_is_approved_and_preserves_v17_precipitation` | Canonical SnowEnergy source/registry drift. Exact released v18 restoration; test unchanged. |
| 5 | `snow_stage3_terminal_receiver_authority_contract all_owner_failure_rolls_back_and_index_records_lifecycle` | Canonical lifecycle registry drift. Exact released registry restoration; test unchanged. |
| 6 | `snow_stage3_turbulent_operator_reconciliation_contract v130_retains_production_and_claim_holds` | Stale unreleased SnowFreeze v137 assertion. Rebound to released v136; production and claim holds unchanged. |
| 7 | `snow_stage3_turbulent_operator_reconciliation_contract v131_retains_fail_closed_authority_gaps_and_protected_boundaries` | Stale unreleased SnowFreeze v137 assertion. Rebound to released v136; fail-closed gaps and boundaries unchanged. |
| 8 | `snow_stage3_v11_constitutive_boundary_contract typed_attachment_excludes_rejected_live_carrier_and_rate_surfaces` | Stale decomposed-source/current-token binding. Now scans both host and included terminal module, retains every negative token, and requires the current exact parcel-construction token. |
| 9 | `snow_terminal_enthalpy_event_numerics_contract package_and_index_preserve_receiving_surface_and_production_boundaries` | Stale unreleased SnowFreeze v137 assertion. Rebound to released v136; receiving/production boundaries unchanged. |

Ran: the corrected retained-guard selection passed 47/47, nextest run ID
`34bb0d72-b4ef-4fab-8032-89492f451075`. The source-level authority-suite
anti-evasion guard also passed.

## Historical eleven

The eleven names are retained as the exact expected baseline-failure set; none
is waived or modified by this package:

1. `openwepp-hillslope-orchestrator hydrology::support_helpers_mod::runoff_reconciliation::stage3_solver::stage3_evaluation_validation_tests::persistent_tests::persistent_support_evaluator_runs_one_admitted_parent_support`
2. `openwepp-hillslope-orchestrator hydrology::support_helpers_mod::runoff_reconciliation::stage3_solver::stage3_evaluation_validation_tests::persistent_tests::terminal_event_request_is_state_bound_and_censors_remaining_time`
3. `openwepp-hillslope-orchestrator hydrology::support_helpers_mod::runoff_reconciliation::stage3_solver::stage3_evaluation_validation_tests::persistent_tests::terminal_no_event_refreeze_closes_persistent_day`
4. `openwepp-hillslope-orchestrator v9_real_consumer_shadow::tests::coupled_hard_boundary_truncates_selected_900_second_child`
5. `openwepp-hillslope-orchestrator v9_real_consumer_shadow::tests::interior_terminal_event_runs_covered_event_and_snow_free_remainder`
6. `openwepp-hillslope-orchestrator v9_real_consumer_shadow::tests::latest_accepted_stage3_state_changes_next_wb14_proposal`
7. `openwepp-hillslope-orchestrator v9_real_consumer_shadow::tests::one_1800_second_child_matches_complete_historical_candidate`
8. `openwepp-hillslope-orchestrator v9_real_consumer_shadow::tests::resolved_snow_and_snow_free_lanes_publish_one_atomic_parent`
9. `openwepp-hillslope-orchestrator v9_real_consumer_shadow::tests::two_900_second_complete_owner_children_publish_one_parent`
10. `openwepp-hillslope-orchestrator v9_real_consumer_shadow::tests::two_resolved_snow_lanes_choose_common_earliest_cadence`
11. `openwepp-hillslope-orchestrator v9_real_consumer_shadow::tests::v10_midnight_failure_rolls_back_every_shadow_owner_exactly`

Their normalized intake signatures, in the same numbered order, are:

1. `one parent support must use the actual terminal solver: TerminalNumerics(BelowCarrierDomain)`
2. `called Result::unwrap() on an Err value: TerminalNumerics(BelowCarrierDomain)`
3. `called Result::unwrap() on an Err value: TerminalNumerics(BelowCarrierDomain)`
4. `synchronized covered parent cadence: Stage3(Kernel(StateSymbolOutOfRange { phase_class: HydrologyRunoffReconciliation, symbol: BoundarySymbol("snow.stage3_terminal_persistent_identity_or_model"), value: 1.0, minimum: Some(0.0), maximum: Some(0.0) }))`
5. `synchronized covered parent cadence: Stage3(TerminalNumerics(BelowCarrierDomain))`
6. `synchronized covered parent cadence: Stage3(Kernel(StateSymbolOutOfRange { phase_class: HydrologyRunoffReconciliation, symbol: BoundarySymbol("snow.stage3_terminal_persistent_identity_or_model"), value: 1.0, minimum: Some(0.0), maximum: Some(0.0) }))`
7. `synchronized covered parent cadence: Stage3(Kernel(StateSymbolOutOfRange { phase_class: HydrologyRunoffReconciliation, symbol: BoundarySymbol("snow.stage3_terminal_persistent_identity_or_model"), value: 1.0, minimum: Some(0.0), maximum: Some(0.0) }))`
8. `real mixed covered/open OFE execution: Executor(Identity("covered carrier lane/OFE set"))`
9. `synchronized covered parent cadence: Stage3(Kernel(StateSymbolOutOfRange { phase_class: HydrologyRunoffReconciliation, symbol: BoundarySymbol("snow.stage3_terminal_persistent_identity_or_model"), value: 1.0, minimum: Some(0.0), maximum: Some(0.0) }))`
10. `real mixed covered/open OFE execution: Executor(Identity("covered carrier lane/OFE set"))`
11. `assertion failed: matches!(shadow.execute_first_interval_for_test(&input), Err(DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Unsupported("forcing transaction, cadence, or snow domain"))))`
