# Exact Diff Reconciliation

Status: `complete`

Evidence mode: `Static + Ran`

The EB-03B executable diff is limited to:

- `tools/local_ci/cqr_quality_evidence.py`: 82 inserted and 56 deleted lines.
  The CLI `intake(args)` continues to load canonical modules. Only private
  `_intake` is injectable, and only `self_test()` uses it with a controlled
  adjudication loader restored through `finally`.
- `tests/integration/assurance_v2_publication_contract.rs`: 96 inserted and 58
  deleted lines. Two test functions became 14 named tests; all original fixture
  mutations and fail-closed/public-nonmutation assertions remain.

There is no EB-03B production Rust diff and no edit to assurance authority,
snow physics, quality thresholds, CRAP registry, nextest scheduling, or timeout
configuration.

The complete quick, frost, and full profiles ran after the executable tree was
frozen. Post-run edits are limited to truthful package, EB-03A, roadmap,
catalog, and contract-index disposition prose.
