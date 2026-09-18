# QA review — feature compatibility completion

Reviewer: `/root/feature_qa` (independent QA/evidence review).  Scope: final
terminal source `d354ccc6ba947c6b0a9e3eed1cdedd0417ae1d5c2896288efbc828beca2f873b`,
the four final feature listings, bounded execution receipts, targeted formatting,
quality attribution, and source custody.  This is a HOLD, not a feature-acceptance
approval.

## Findings

- **Blocking — required capability control fails.** `completion-terminal-none-controls-03.json`
  and `completion-terminal-both-controls-03.json` each exit 100 because
  `accepted_publication_support_capability_is_private_move_only_non_wire` rejects
  `serde_json` in the trusted installer.  `completion-frozen-baseline-capability-control.json`
  reproduces the same assertion with the frozen `e93175…` executable (exit 101),
  so this is inherited rather than attributable to this feature increment.  It
  nevertheless prevents acceptance of the required control set.  The guard must
  neither be weakened nor worked around here.

- **Blocking — strict quality remains unmet.** Both terminal clippy receipts
  (`completion-terminal-quality-nodeps-03.json` and
  `completion-terminal-quality-strict-03.json`) exit 101.  The matched
  `completion-quality-attribution.json` reports 2,849 candidate diagnostics versus
  2,851 baseline diagnostics, with zero new relevant diagnostics; this is bounded
  attribution, not strict-quality acceptance.

## Final feature-membership reconciliation

Ran final listings are source-matched to `d354…`: no features 1475, evidence-only
1480, persisted-only 1512, and both features 1519.  The final full inventory is
the 1518-name baseline plus only
`snow_stage3_v11_current_context_capture::constructor_encoder_matches_independent_serde_and_literal_enum_names_v1`.
Every final inventory contains all fourteen retained promotion selectors.  Thus the
44 names absent from the no-feature listing are exactly source-declared feature
gates, as follows.

### Evidence-only — `restart-authority-evidence` (5)

- `direct_runtime::surface_liquid_owner_v4_projection::tests::projection_v4_refuses_outer_resealed_beginning_lse_parent_substitution`
- `direct_runtime::surface_liquid_owner_v4_projection::tests::projection_v4_refuses_outer_resealed_noncanonical_nested_receipts_atomically`
- `land_surface_energy_shadow::v4_restart_evidence::tests::accepted_evidence_preserves_negative_zero_and_replays_projection`
- `land_surface_energy_shadow::v4_restart_evidence::tests::accepted_nonzero_carry_support_advances_with_exact_continuity`
- `v9_real_consumer_shadow::restart_bgc_scope_tests::actual_b01_parent_restores_with_configured_bgc_scope`

### Persisted-only — `persisted-restart-v1` (37)

- `direct_runtime::laned_active::native_routing_summary_wire_retains_nonzero_fields_and_refuses_foreign_mode`
- `direct_runtime::surface_liquid_owner::tests::restart_admission_lineage_clone_is_identity_only_and_rejects_poisons`
- `snow_stage3_v11_attachment::actual_b01_archived_finalization_event_bridge_refuses_foreign_endpoints`
- `snow_stage3_v11_attachment::actual_b01_archived_receipts_reconstruct_cold_snow_owner`
- `snow_stage3_v11_attachment::restart_v3_active_execution_beginning_tests::snow_free_predecessor_wins_and_omission_or_owner_substitution_rejects`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::archive_first_positive_beginning_rejects_pre_event_owner_substitution`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::archive_records_authenticate_sequentially_from_pinned_seed`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::composed_member_native_restoration_accepts_complete_deferred_admission`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::composed_member_native_restoration_from_pinned_seed`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::current_phase_members_validate_without_physics`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::current_phase_semantic_substitutions_refuse_without_physics`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::deferred_context_complete_bundle_private_diagnostic_restoration`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::deferred_context_raw_correspondence_refuses_semantic_poison`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::deferred_context_raw_primitives_authenticate_credit_correspondence_before_admission`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::deferred_credit_capture_represents_no_per_credit_clock_provenance`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::member_constructor_inverse_roundtrips_actual_operands`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::prepared_day_member_restores_provider_and_forcing_component`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::provider_authority_scalars_reject_semantic_substitutions_without_installation`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::recorded_archive_manifest_rejects_semantic_order_and_namespace_changes`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::snow_free_member_export_reconstructs_provisional_on_clone_from_pinned_seed`
- `snow_stage3_v11_current_context_capture::member_backed_restore_tests::snow_free_member_export_restores_clock_and_parent_without_native_decode`
- `snow_stage3_v11_current_context_capture::member_restoration::actual_member_frozen_litter_history_preflight_is_pure`
- `snow_stage3_v11_current_context_capture::member_restoration::authenticated_gsi_joins_refuse_semantic_substitutions_without_mutating_capture`
- `snow_stage3_v11_current_context_capture::member_restoration::deferred_context_late_actual_entry_rollback_and_receipt_precedence`
- `snow_stage3_v11_current_context_capture::member_restoration::deferred_context_lifecycle_controls_refuse_without_publication`
- `snow_stage3_v11_current_context_capture::member_restoration::inactive_counter_custody_refuses_actual_member_substitutions_without_installation`
- `snow_stage3_v11_current_context_capture::member_restoration::ordinary_native_soil_admission_refuses_actual_deferred_context_without_mutation`
- `snow_stage3_v11_current_context_capture::member_restoration::owner_comparison::owner_report_equal_missing_extra_and_byte_boundaries`
- `snow_stage3_v11_current_context_capture::member_restoration::owner_comparison::owner_report_io_failure_preserves_comparison_refusal`
- `snow_stage3_v11_current_context_capture::member_restoration::owner_comparison::owner_report_saves_exact_bytes_and_refuses_overwrite`
- `v9_real_consumer_shadow::accepted_publication_chronology_tests::archived_restorer_uses_replayed_terminal_event_tail_and_rejects_wire_poisons`
- `v9_real_consumer_shadow::deferred_composite_capability_traits::deferred_capabilities_are_non_wire_and_non_copyable`
- `v9_real_consumer_shadow::direct_v10_soil_thermal_v2_tests::native_soil_restart_proof_constructor_and_both_restore_hooks_preserve_exact_custody`
- `v9_real_consumer_shadow::native_provider_position_tests::native_provider_receipt_requires_exact_expected_position_without_mutation`
- `v9_real_consumer_shadow::v3_publication_retention_tests::native_clock_mode_restore_preserves_exact_owner_and_refuses_foreign_inputs_atomically`
- `v9_real_consumer_shadow::v3_publication_retention_tests::native_restart_full_exact_adoption_rejects_resealed_initial_carry`
- `v9_real_consumer_shadow::v3_publication_retention_tests::native_routing_projection_preserves_clock_and_rejects_foreign_install_atomically`

### Both features — `persisted-restart-v1` and `restart-authority-evidence` (2)

- `v9_real_consumer_shadow::tests::b01_current_context_importer_restores_fixture_without_physics`
- `v9_real_consumer_shadow::tests::native_recorder_file_owned_reader_restores_local_fixture`

The importer control executes in the both-feature receipt.  The file-owned reader
is listed and compiles in that mode, but is deliberately not run: it physically
replays archived days 0–3 before day 4, outside the owner-approved one-context,
zero-physics restoration boundary.  This is an explicit non-execution, not a PASS.

## Fix verification and evidence

The C6 parity repair covers every growth/decomposition action and active-context
variant, both erosion-shape authorities, and populated `Some(-1.25°C)` outcome
state.  The raw Serde oracle remains independent.  C7 uses a captured native
hydrology constructor pin; no test-only fallback was found.  The final targeted
format check passes with no introduced unformatted hunk.  Eight terminal feature
build checks pass.  Source custody reconstructs all 747 entries exactly from the
frozen integrated baseline plus the retained patch.

The final controls were selected by exact names with `--no-tests fail`: none and
both modes each ran the expected control population and exposed the inherited
capability failure; evidence-only (2) and persisted-only (5) pass.  Earlier actual
ordinary recorder controls passed, including covered and first-snow-free producer
paths.  No zero-selection result is used as evidence.

## Non-blocking follow-up

No broad cleanup is authorized for the inherited capability-guard or strict-lint
debt.  Keep both visible in the package disposition; scientific, campaign,
production-adoption, and cadence claims remain outside this QA scope.

## QA verdict

Membership, final-source custody, bounded feature compilation, targeted formatting,
and the accepted C6/C7/fix-verification evidence are satisfactory.  **HOLD:** the
actual required capability control and strict quality gate remain failing, so this
review does not approve feature completion.
