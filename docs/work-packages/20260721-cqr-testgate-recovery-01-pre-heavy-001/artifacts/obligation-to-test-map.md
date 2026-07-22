# Obligation-to-Test Map

Static: `pre_heavy.rs` is glue-tier admission/orchestration code and carries no
science-contract numerical obligations. The applicable A-H families are bound
as follows; non-applicable numerical families are stated explicitly.

| Family | Applicability and bound tests |
| --- | --- |
| A nominal behavior | Audit construction, ordered ten-check assembly, READY validation, and resume admission are covered by `unsealed_audit_assembles_all_ten_checks_and_fallback_is_representable` and `ready_audit_validation_execution_and_resume_chains_are_directly_bound`. |
| B boundaries | Empty node inventories, empty ledger heads, single active prompts, and exact package-authority cardinality are covered by the focused `pre_heavy::tests` inventory. |
| C malformed or identity-invalid input | Schema duplication, receipt/audit identity drift, malformed node shapes, and reserved ledger fields are covered by `audit_schema_rejects_duplicate_canonical_check_ids`, `extracted_receipt_and_checkpoint_bindings_remain_fail_closed`, and `low_coverage_binding_helpers_exercise_their_reject_arms`. |
| D domain rejects | Applicable execution-domain rejects—wrong cost class, aliased roots, forward prerequisites, non-PASS LIGHT results, and invalid combined-quality DAGs—are directly covered. No science-domain arithmetic is present. |
| E missing dependencies or artifacts | Missing ledgers, missing prompts, missing checkpoint artifacts, package-schema loss, and artifact content drift are directly covered. |
| F non-finite or numeric edges | Not applicable: this module performs no floating-point or process-physics arithmetic. Positive execution-attempt cardinality remains covered. |
| G ordering and precedence | Canonical check order, failure-token precedence, stage dependency order, exact ledger successor, and last-status tooling-defect folding are directly covered. |
| H fail-closed behavior | Representable fallback audit creation, open tooling defects, orphan reconciliation, execution-claim drift, binary drift, and ledger-chain drift are directly covered. |

Ran: the final measurement reports no eligible production function below the
75% region floor and no CRAP entry above 30, so this map requires no retained
exception disposition.
