# Assurance V2 Amendment Contract Clippy Line Disposition

Package ID:
`20260727-assurance-v2-amendment-contract-clippy-line-disposition-001`

Queue ID: `ASSURANCE-V2-CLIPPY-LINE-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to complete CAL-04B and resolve
its assurance/TESTGATE blockers through reviewed work-package corrections.

This defect-closure ExecPlan is maintained under
`docs/defect_closure_execplans.md`.

## Objective

Close the exact workspace-Clippy failure retained in the TESTGATE ledger
bootstrap canonical receipt by adding a narrow, justified line-length
disposition to one cohesive assurance integration test without changing test
behavior.

## Reproducer

At exact subject head
`2e3d51eca5945a8fb324cf0e23e6371cd04b05d9`, canonical transaction root
`/home/workdir/gate-testgate-ledger-bootstrap-canonical-002` reached LIGHT
`PASS` and a ten-check `READY` audit. HEAVY failed only at:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Clippy reported `clippy::too_many_lines` for the 156-line integration test
`report_source_adoption_is_read_only_deterministic_and_invalidates_review_authority`
in `tests/integration/assurance_v2_amendment_contract.rs`. The receipt then
dependency-blocked doc tests and full-workspace Nextest.

## Included Scope

- one function-scoped `#[allow(clippy::too_many_lines)]` disposition with an
  adjacent rationale that the test intentionally proves one atomic
  read-only/check/apply/invalidation/idempotence lifecycle;
- exact focused test, workspace Clippy, and full regression validation;
- dual scaffold review, dual implementation review, dual terminal
  verification, and comparator-owned canonical successor execution.

## Excluded Scope

- changing assurance behavior, fixtures, assertions, review authority,
  identities, lifecycle semantics, or science;
- splitting or weakening the integration test;
- broad file/crate/workspace lint allowances;
- changing gate definitions, inventory, executor behavior, ledger code, CAL
  data, or Harvard state.

## Declared Write Set

- `tests/integration/assurance_v2_amendment_contract.rs`
- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260727-testgate-first-attempt-ledger-bootstrap-001/artifacts/canonical-execution.md`
- `docs/work-packages/20260727-assurance-v2-amendment-contract-clippy-line-disposition-001/**`

No other path is writable. This write set must not widen.

## Execution Plan

1. Commit this prospective scaffold and obtain two independent read-only
   scaffold reviews.
2. Add the single function-scoped disposition and rationale.
3. Run every command in `Validation Commands`.
4. Commit the exact implementation and obtain two independent implementation
   reviews.
5. Record exact evidence and obtain two terminal verifiers.
6. Delegate one comparator-owned exact-head canonical transaction on a fresh
   root/ledger. Require LIGHT PASS, READY audit, and every HEAVY node PASS.
7. Obtain two independent receipt verifiers, close this successor, and resume
   the ledger/CAL closeout chain.

## Acceptance

- The only Rust diff is a function-scoped Clippy disposition and adjacent
  rationale on the exact failing integration test.
- No assertion, setup, call, fixture, path, identity, or behavior changes.
- No broader lint suppression.
- The focused assurance integration target passes unchanged.
- Workspace Clippy passes with `-D warnings`.
- Full workspace regression and authority anti-evasion gates pass.
- The canonical successor receipt is PASS with no unavailable required item.

## Validation Commands

```text
cargo nextest run --test assurance_v2_amendment_contract
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo test --workspace --doc --locked --offline
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
cargo fmt --all -- --check
markdown-doc lint --path docs/work-packages/20260727-assurance-v2-amendment-contract-clippy-line-disposition-001
git diff --check
```

## Line-Count Governance

The touched integration file is below the 2,000-line Rust WARN threshold.
This package does not authorize a source-file line-count exception.

Harvard remains sealed and CAL population remains prohibited.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes one bounded
implementation worker, two independent scaffold/implementation reviewers, two
independent terminal/receipt verifiers, and the `comparator_suite_runner`;
writes are limited to the declared write set. Heavy/comparator work must use
`comparator_suite_runner`; if unavailable, disposition `HOLD` without parent
fallback.
