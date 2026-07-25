# Reproduction And Root Cause

Evidence class: Ran / Static.

## Clean-Scaffold Reproduction

At clean scaffold head `81599ed1`, the exact seven-test coverage-configured run
completed in `715.903s`: 3 passed and 4 failed.

The three `GATE-COMMITTED-CHECKOUT-NOT-EXACT` failures from the Order-3 dirty
attempt passed on the committed scaffold head:

- `pre_heavy::coverage_tests::exact_planner_output_reconstructs_through_the_public_audit_path`
- `verifier::tests::receipt_verification_reconstructs_identity_dag_inventory_and_artifacts`
- `verifier::tests::verifier_accepts_truthful_fail_and_blocked_receipts`

Those were not independent gate-planner defects. They correctly rejected the
uncommitted execution identity used by the earlier observatory attempt. A fresh
Order-3 run must start from the committed correction state.

## Four Owned Mechanisms

1. `executor::coverage_tests::ready_audited_heavy_preserves_import_and_final_receipt_bindings`
   and
   `verifier::tests::coverage_tests::ready_audit_verification_preserves_order_and_exact_verdict`
   construct a synthetic HEAVY node named `adjudicated-crap-v1`. Order 2
   intentionally made that identity schema-invalid. These tests exercise
   staged execution and verification, not CRAP, so they require an ordinary
   fixture-only HEAVY identity.
2. `executor::tests::terminal_plan_detects_out_of_manifest_source_mutation_and_verifies_invalid_receipt`
   used an independent gate identity that sorted before the primary mutator.
   The marker therefore invalidated reconstruction first and blocked the
   intended `.github` mutation. Give the prerequisite-free secondary gate a
   later deterministic identity so the intended mutator runs first. The
   monitored execution checkout must contain `.github/probe.yml`, its source
   digest must change, and the later marker must remain absent.
3. `planner::tests::terminal_reconciliation_reports_added_paths_and_escalation`
   mutates `quality_disposition.owner` but `reconcile_semantics` currently
   accepts the drift and returns ordinary path reconciliation. This is a real
   terminal trust gap: intent and terminal quality dispositions must be exact.

All mechanisms lie inside the declared correction envelope. Production
correction is limited to the missing exact quality-disposition comparison;
the other changes repair stale fixtures without weakening their assertions.
