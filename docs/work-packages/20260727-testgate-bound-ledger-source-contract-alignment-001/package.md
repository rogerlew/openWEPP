# TESTGATE Bound-Ledger Source Contract Alignment

Package ID: `20260727-testgate-bound-ledger-source-contract-alignment-001`

Queue ID: `TESTGATE-BOUND-LEDGER-CONTRACT-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to complete CAL-04B and resolve
its assurance/TESTGATE blockers through reviewed work-package corrections.

This defect-closure ExecPlan is maintained under
`docs/defect_closure_execplans.md`.

## Correction Authority Envelope

- Defect: full-profile run `dd04b429-27d3-494d-96be-1d3a7a80423f`
  passes 2,360/2,361 tests but one source-contract assertion expects the
  superseded pathname resume call.
- Observed violation: `blocking_executor_and_quality_deferral_preserve_manual_rollback`
  asserts `load_candidate_after_ready_audit(...)`, while the reviewed secure
  transition requires `load_candidate_after_ready_audit_text(...)` and
  `ledger.read_text()` from `BoundAttemptLedger`.
- Allowed edit class: replace the one stale positive source assertion with
  exact positive assertions for the bound-text API and retained-handle read.
- Validation surfaces: exact diff, focused source-contract target, full
  workspace profile, workspace Clippy/doc tests, authority anti-evasion,
  formatting, documentation, and canonical TESTGATE receipt.
- Protected boundaries: production code, gate inventory/policy, assurance
  behavior, fixtures, all unrelated assertions, CAL data, and Harvard state.

## Objective

Align one stale source-level executor contract with the already reviewed
bound-ledger transition without changing runtime behavior.

## Included Scope

- `tests/integration/testgate_ci_executor_contract.rs`;
- exact assertion replacement only;
- complete reviewed/verified/canonical closure.

## Excluded Scope

- production Rust changes;
- weakening or deleting the resume-consumer assertion;
- fixture, executor, gate, assurance, CAL, or Harvard changes.

## Declared Write Set

- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260727-assurance-v2-amendment-contract-clippy-line-disposition-001/artifacts/implementation-gates.md`
- `docs/work-packages/20260727-testgate-bound-ledger-source-contract-alignment-001/**`

No other path is writable. This write set must not widen.

## Execution Plan

1. Commit this scaffold, bind its literal authority base, and obtain two
   independent scaffold reviews retained in `artifacts/scaffold-reviews.md`.
2. Assign one bounded worker to replace only the stale assertion.
3. Run every `Validation Commands` command.
4. Obtain two implementation reviews and classify findings in
   `artifacts/review-findings.md`; retain full A/B reports in
   `artifacts/implementation-reviews.md`.
5. Record implementation gates, worker handoff, line-count disposition, exact
   diff reconciliation, and final disposition; obtain two terminal verifiers
   in `artifacts/terminal-verifications.md`.
6. Delegate one fresh no-retry comparator-owned canonical TESTGATE transaction
   and retain `canonical-execution.md`.
7. Obtain two receipt verifiers in `receipt-verifications.md`, close this
   package and its predecessors, and resume CAL-04B.

## Acceptance

- The only Rust diff replaces one obsolete positive source string with exact
  positive coverage of `load_candidate_after_ready_audit_text(` and
  `&ledger.read_text()?`.
- The source contract continues to require trusted transition, receipt
  verification, current context, and all existing executor guards.
- No production or behavioral change.
- Focused test, 2,361-test full profile, workspace Clippy, doc tests, and
  authority gates pass.
- Exact terminal diff is within the declared write set.
- Both reviewers/verifiers are separately attributable with evidence class,
  exact subject, findings, gates, and GO/HOLD or PASS/HOLD.
- Fresh canonical receipt is PASS with exact inventory/count reconciliation.

## Validation Commands

```text
cargo nextest run --test testgate_ci_executor_contract
cargo nextest run --workspace --profile full
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --doc --locked --offline
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
cargo fmt --all -- --check
markdown-doc lint --path docs/work-packages/20260727-testgate-bound-ledger-source-contract-alignment-001
markdown-doc lint --path docs/work-packages/README.md
markdown-doc lint --path docs/planning/canopy-phenology-assurance-roadmap.md
markdown-doc lint --path docs/work-packages/20260727-assurance-v2-amendment-contract-clippy-line-disposition-001/artifacts/implementation-gates.md
git diff --check
```

## Line-Count Governance

Both implementation reviewers and terminal verifiers must record the exact
touched-file count and evaluate the 2,000-line WARN and 3,000-line mandatory
refactor thresholds.

## Progress

- [x] Full-profile defect reproduced exactly.
- [ ] Scaffold base bound and dual-reviewed.
- [ ] Implementation and exact gates complete.
- [ ] Dual implementation review and terminal verification complete.
- [ ] Canonical PASS and dual receipt verification complete.

## Surprises & Discoveries

- Strict Clippy passed after the assurance disposition; the full profile then
  exposed the only stale source-contract consumer of the bound-ledger API.

## Decision Log

- Decision: update the positive source assertion rather than retain a legacy
  API alias.
  Rationale: the transition must prove it consumes retained ledger bytes, and a
  compatibility alias would weaken that security claim.

## Outcomes & Retrospective

Pending implementation and closure.

Harvard remains sealed and CAL population remains prohibited.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes one bounded worker,
two independent scaffold/implementation reviewers, two independent
terminal/receipt verifiers, and `comparator_suite_runner`. Writes are limited
to the declared write set. Every report must include evidence class, exact
subject, findings, gates, and verdict. Heavy work must use
`comparator_suite_runner`; unavailability is `HOLD`, with no parent fallback.
