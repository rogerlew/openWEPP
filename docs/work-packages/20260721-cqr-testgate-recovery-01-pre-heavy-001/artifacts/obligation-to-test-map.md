# Obligation-to-Test Map

Static: `pre_heavy.rs` is glue-tier admission/orchestration code and carries no
science-contract numerical obligations. The applicable A-H families are bound
as follows; non-applicable numerical families are stated explicitly.

| Family | Applicability and bound tests |
| --- | --- |
| A nominal behavior | Public `build_audit`, ordered ten-check assembly, READY validation, execution admission, resume admission, and exact plan reconstruction are covered by `ready_audit_validation_execution_and_resume_chains_are_directly_bound`, `unsealed_audit_assembles_all_ten_checks_and_fallback_is_representable`, and `exact_planner_output_reconstructs_through_the_public_audit_path`. |
| B boundaries | Empty node inventories and ledger heads: `cheap_file_shape_root_and_ledger_guards_cover_success_and_failure`; single active prompt: `active_package_prompt_requires_exactly_one_markdown_file`; exact package-authority cardinality: `package_admission_selects_exactly_one_independently_valid_authority`. |
| C malformed or identity-invalid input | Schema duplication, receipt/audit identity drift, malformed node shapes, and reserved ledger fields are covered by `audit_schema_rejects_duplicate_canonical_check_ids`, `extracted_receipt_and_checkpoint_bindings_remain_fail_closed`, and `low_coverage_binding_helpers_exercise_their_reject_arms`. |
| D domain rejects | Wrong cost class, aliased roots, forward prerequisites, and non-PASS LIGHT results: `cheap_file_shape_root_and_ledger_guards_cover_success_and_failure` and `light_stage_and_stage_order_reject_nonpass_or_forward_dependency`; invalid combined-quality DAG: `combined_decision_requires_its_exact_dag_shape`. No science-domain arithmetic is present. |
| E missing dependencies or artifacts | Missing ledger: `missing_ledger_is_reported_by_both_owning_checks_without_escape`; missing prompt: `active_package_prompt_requires_exactly_one_markdown_file`; missing/drifted checkpoint artifact: `light_checkpoint_artifacts_are_content_and_attempt_bound`; package-schema loss: `package_admission_reconstructs_real_candidate_authority_and_fails_closed`. |
| F non-finite or numeric edges | Not applicable: this module performs no floating-point or process-physics arithmetic. Positive execution-attempt cardinality remains covered. |
| G ordering and precedence | Canonical check order: `unsealed_audit_assembles_all_ten_checks_and_fallback_is_representable`; failure-token precedence: `failure_check_index_preserves_first_matching_token_precedence`; stage order: `light_stage_and_stage_order_reject_nonpass_or_forward_dependency`; exact ledger successor: `heavy_started_must_be_the_exact_successor_of_the_audited_ledger_head`; last-status folding: `tooling_defect_ledger_uses_the_last_status_for_each_defect`. |
| H fail-closed behavior | Fallback audit: `representable_early_failure_emits_ten_check_invalid_audit`; open defects: `tooling_defect_ledger_uses_the_last_status_for_each_defect`; orphan recovery: `orphaned_admission_is_closed_once_and_recurrence_opens_defect`; claim/binary/ledger drift: `low_coverage_binding_helpers_exercise_their_reject_arms` and `heavy_started_must_be_the_exact_successor_of_the_audited_ledger_head`. |

Ran: the corrected changed-head region measurement confirms all 111 production
functions at or above 75%, with an 80.00% minimum. No retained exception is
used.
