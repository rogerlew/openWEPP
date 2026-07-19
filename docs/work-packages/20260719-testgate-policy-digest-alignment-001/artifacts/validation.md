# Focused Validation

Evidence class: `Ran`

## Planner Library

The first exact planner-library invocation ran from the uncommitted correction
tree:

```text
cargo nextest run -p openwepp-gate-planner --lib
57 passed; 1 failed; 4 canceled; 782.617 seconds
```

The sole failure was
`verifier::tests::receipt_verification_reconstructs_identity_dag_inventory_and_artifacts`.
It correctly rejected the dirty checkout as
`GATE-COMMITTED-CHECKOUT-NOT-EXACT`; it did not report policy digest drift.

After committing the already-proven one-field correction, only that failed
case and the four fail-fast cancellations were rerun by exact test name:

```text
cargo nextest run -p openwepp-gate-planner --lib \
  verifier::tests::receipt_verification_reconstructs_identity_dag_inventory_and_artifacts \
  verifier::tests::reuse_fails_closed_for_nonreusable_failed_wrong_trust_and_stale_evidence \
  verifier::tests::verifier_accepts_truthful_fail_and_blocked_receipts \
  verifier::tests::verifier_derives_partial_failed_inventory_from_junit_bytes \
  verifier::tests::verifier_rejects_provider_without_reconstruction_root
```

```text
5 passed; 57 skipped; 933.146 seconds
```

The two source-identical runs therefore cover the complete 62-test planner
inventory: 57 retained passes plus 5 clean-commit passes, with no unresolved
failure.

## TESTGATE Contracts

```text
cargo nextest run \
  --test testgate_align_authority_contract \
  --test testgate_assure_campaign_currency_contract \
  --test testgate_ci_executor_contract
15 passed; 0 skipped; 75.578 seconds
```

No workspace/full Nextest, Clippy, coverage, CRAP, cargo-deny, release,
workflow, or runner command had run when this focused evidence was recorded.
Both reviewers subsequently found that the policy change is mechanically
`CRITICAL`; the focused results remain truthful but cannot replace the
planner-selected critical gate plan.
