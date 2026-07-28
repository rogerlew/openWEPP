# Assurance V2 Amendment Contract Clippy Line Disposition

Package ID:
`20260727-assurance-v2-amendment-contract-clippy-line-disposition-001`

Queue ID: `ASSURANCE-V2-CLIPPY-LINE-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to complete CAL-04B and resolve
its assurance/TESTGATE blockers through reviewed work-package corrections.

This defect-closure ExecPlan is maintained under
`docs/defect_closure_execplans.md`.

## Correction Authority Envelope

- Defect: canonical receipt
  `30054b51863488b85d23c95a68b8d5ebc8f5d2d9be5b94959dfec4dab194b54f`
  fails workspace Clippy only because one cohesive assurance integration test
  is 156 lines.
- Observed violation: `clippy::too_many_lines` under exact command
  `cargo clippy --workspace --all-targets -- -D warnings`.
- Allowed edit class: one function-scoped Clippy attribute and one adjacent
  explanatory comment on the named test.
- Validation surfaces: exact source diff, focused assurance target, workspace
  Clippy, full Nextest, doc tests, authority anti-evasion, formatting,
  documentation lint, and canonical TESTGATE receipt.
- Protected boundaries: every test statement and assertion, assurance
  implementation/fixture/identity/review authority, gate policy/inventory,
  ledger implementation, CAL data, and Harvard state.

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
   scaffold reviews recorded with separate attribution in
   `artifacts/scaffold-reviews.md`.
2. Add the single function-scoped disposition and rationale.
3. Run every command in `Validation Commands`.
4. Commit the exact implementation and obtain two independent implementation
   reviews. Record every finding as `accepted`, `rejected`, `deferred`, or
   `follow-up` in `artifacts/review-findings.md`, with both full independent
   reports retained in `artifacts/implementation-reviews.md`.
5. Record exact gate evidence, worker handoff, Rust line-count review, exact
   declared-base-to-terminal-head diff reconciliation, and final disposition
   artifacts; obtain two terminal verifiers recorded separately in
   `artifacts/terminal-verifications.md`.
6. Delegate one comparator-owned exact-head canonical transaction on a fresh
   root/ledger and record it in `artifacts/canonical-execution.md`. Require
   LIGHT PASS, READY audit, and every HEAVY node PASS.
7. Obtain two independent receipt verifiers, close this successor, and resume
   the ledger/CAL closeout chain. Retain both reports in
   `artifacts/receipt-verifications.md`.

## Acceptance

- The only Rust diff is a function-scoped Clippy disposition and adjacent
  rationale on the exact failing integration test.
- No assertion, setup, call, fixture, path, identity, or behavior changes.
- No broader lint suppression.
- The focused assurance integration target passes unchanged.
- Workspace Clippy passes with `-D warnings`.
- Full workspace regression and authority anti-evasion gates pass.
- The canonical successor receipt is PASS with no unavailable required item.
- The exact terminal diff contains only declared paths; the Rust diff contains
  only the scoped attribute and rationale.
- Required package artifacts exist: `review-findings.md`,
  `implementation-gates.md`, `worker-handoff.md`,
  `line-count-disposition.md`, `final-disposition.md`,
  `scaffold-reviews.md`, `implementation-reviews.md`,
  `terminal-verifications.md`, `canonical-execution.md`, and
  `receipt-verifications.md`. Consolidated review files identify reviewer A
  and reviewer B separately, preserve each evidence class/findings/verdict, and
  demonstrate independence.

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
markdown-doc lint --path docs/work-packages/README.md
markdown-doc lint --path docs/planning/canopy-phenology-assurance-roadmap.md
markdown-doc lint --path docs/work-packages/20260727-testgate-first-attempt-ledger-bootstrap-001/artifacts/canonical-execution.md
git diff --check
git diff --name-only 388432b8b8ee595c1f4433df49903ab34809f039..HEAD
git diff 388432b8b8ee595c1f4433df49903ab34809f039..HEAD -- tests/integration/assurance_v2_amendment_contract.rs
```

## Line-Count Governance

The touched integration file is 1,044 lines, below the 2,000-line Rust WARN
threshold. Both implementation reviewers and both terminal verifiers must
recount it and explicitly evaluate the 2,000-line WARN and 3,000-line mandatory
refactor thresholds. This package does not authorize a source-file line-count
exception.

## Progress

- [x] Canonical failure reproduced and retained.
- [x] Prospective scaffold committed.
- [x] Corrected scaffold received dual `GO`.
- [x] Scoped implementation and focused/supporting gates complete.
- [x] Dual implementation review and finding disposition complete.
- [x] Full-regression follow-up closed with 2,361/2,361 PASS.
- [ ] Dual terminal verification complete.
- [ ] Canonical PASS receipt and dual receipt verification complete.

## Surprises & Discoveries

- The ledger bootstrap and inherited-FD consumer path passed through LIGHT and
  the ten-check READY audit; the successor is isolated to a pre-existing
  integration-test lint disposition.
- The first full profile after the lint correction exposed a stale executor
  source assertion rather than an assurance or lint defect. Its bounded
  successor closed with full and canonical PASS.

## Decision Log

- Decision: preserve the cohesive lifecycle test and use the narrowest
  function-scoped Clippy disposition.
  Rationale: extracting helpers solely to satisfy a line-count heuristic would
  fragment one audit scenario and enlarge the behavioral diff.

## Outcomes & Retrospective

Pending implementation, canonical PASS, and closeout.

Harvard remains sealed and CAL population remains prohibited.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes one bounded
implementation worker, two independent scaffold/implementation reviewers, two
independent terminal/receipt verifiers, and the `comparator_suite_runner`;
writes are limited to the declared write set. Each reviewer/verifier must return
its evidence class, exact subject commit, findings, gate results, and GO/HOLD or
PASS/HOLD verdict for separately attributed retention in the named artifacts.
Heavy/comparator work must use `comparator_suite_runner`; if unavailable,
disposition `HOLD` without parent fallback.
