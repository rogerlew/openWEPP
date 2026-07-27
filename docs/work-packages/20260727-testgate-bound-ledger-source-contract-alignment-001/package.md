# TESTGATE Bound-Ledger Source Contract Alignment

Package ID: `20260727-testgate-bound-ledger-source-contract-alignment-001`

Queue ID: `TESTGATE-BOUND-LEDGER-CONTRACT-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to complete CAL-04B and resolve
its assurance/TESTGATE blockers through reviewed work-package corrections.

This defect-closure ExecPlan is maintained under
`docs/defect_closure_execplans.md`.
This is a living ExecPlan. `Progress`, `Surprises & Discoveries`, `Decision
Log`, and `Outcomes & Retrospective` must be updated at every stopping point so
execution can resume from this file alone.

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

## Conversion Rule And Seven-Gate Bar

If the package confirms the reproducible root cause inside this envelope, it
must replace the stale assertion, validate the correction, and complete dual
review and disposition in this package. It may not close as `HOLD` merely
because more investigation or implementation remains possible.

The package-specific seven gates are: (1) reproduction is the retained
2,360/2,361 full-profile result; (2) the named mechanism is one obsolete source
string; (3) ownership is the declared integration-test path; (4) authority is
the reviewed bound-ledger consumer contract already implemented in
`crates/openwepp-gate-planner/src/main.rs`; (5) safety requires retaining all
existing fail-closed assertions and making no production change; (6)
testability is the focused source-contract test failing before and passing
after the replacement; and (7) validation is the focused test, exact diff,
full profile, strict Clippy, authority gates, and canonical PASS receipt.

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

## Milestones

Milestone 1 binds the scaffold and produces two independent reviews. Its
observable result is two `GO` verdicts at one exact clean scaffold head, with
every finding dispositioned.

Milestone 2 records an accepted pre-implementation intent plan before the Rust
edit, then assigns one bounded worker to replace only the stale assertion. Its
observable result is an exact two-assertion source-contract diff and a focused
PASS.

Milestone 3 reconciles the exact terminal diff into an authenticated terminal
plan and runs every command in `Validation Commands`. Its observable result is
a 2,361/2,361 full profile and all supporting gates PASS.

Milestone 4 obtains dual implementation reviews and dual terminal
verifications, including line-count and non-deferral checks. Its observable
result is separately attributable A/B `GO` and `PASS` evidence with no
undispositioned finding.

Milestone 5 delegates one fresh, no-retry comparator-owned canonical TESTGATE
transaction and obtains dual receipt verification. Its observable result is a
balanced PASS receipt with exact inventory/count reconciliation. This package
then updates only the named predecessor hold artifact; broader predecessor
closeout and CAL resumption remain separately governed by their declared write
sets.

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

## HOLD Legitimacy And Defect-Shaped Handoff

`HOLD` is legitimate only if the mechanism is proven outside this envelope,
governing authority is missing or contradictory, the retained failure is
invalid evidence, or a required gate cannot run in the available environment.
Before any `HOLD`, `artifacts/final-disposition.md` must name the boundary,
evidence, considered in-envelope correction route, why it cannot close now,
and the owner. Implementation effort, diagnostic uncertainty, or partial
success are not legitimate boundaries.

If a legitimate boundary is reached, the handoff's first actionable item must
be `close defect TESTGATE-BOUND-LEDGER-CONTRACT-01`, followed by observable
failure, mechanism, authority/write set, required reading, failing evidence,
correction authority, acceptance target, and HOLD conditions. It must not relay
only another diagnostic step.

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
git diff --name-only f8cba1c9f3e02d241a2bb7fccc3329a0a142ac57..HEAD
git diff f8cba1c9f3e02d241a2bb7fccc3329a0a142ac57..HEAD -- tests/integration/testgate_ci_executor_contract.rs
```

## Line-Count Governance

Both implementation reviewers and terminal verifiers must record the exact
touched-file count and evaluate the 2,000-line WARN and 3,000-line mandatory
refactor thresholds.

## Progress

- [x] (2026-07-27, Codex) Full-profile defect reproduced exactly.
- [x] (2026-07-27, Codex) Initial scaffold reviewed; both reviewers returned
  governance `HOLD`.
- [ ] (2026-07-27, Codex) Amended scaffold requires exact-head dual review.
- [ ] Scaffold base bound and dual-reviewed.
- [ ] Implementation and exact gates complete.
- [ ] Dual implementation review and terminal verification complete.
- [ ] Canonical PASS and dual receipt verification complete.

## Surprises & Discoveries

- Strict Clippy passed after the assurance disposition; the full profile then
  exposed the only stale source-contract consumer of the bound-ledger API.
- Both scaffold reviewers independently found missing autonomous ExecPlan and
  prompt scaffolding while confirming the proposed two-assertion correction.

## Decision Log

- Decision: update the positive source assertion rather than retain a legacy
  API alias.
  Rationale: the transition must prove it consumes retained ledger bytes, and a
  compatibility alias would weaken that security claim.
  Date/Author: 2026-07-27, Codex.
- Decision: limit predecessor mutation to the one declared assurance gate
  artifact and defer broader closeout to each predecessor's own write set.
  Rationale: a package may not promise writes outside its prospective scope.
  Date/Author: 2026-07-27, Codex.

## Outcomes & Retrospective

Pending implementation and closure.

Harvard remains sealed and CAL population remains prohibited.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one bounded implementation worker, two independent
scaffold/implementation reviewers, two independent terminal/receipt verifiers,
and `comparator_suite_runner`. The worker may write only the target integration
test; reviewers and verifiers are read-only; the comparator may write only its
fresh external artifact root and ledger. Expected outputs are compact reports
and the named package evidence artifacts. Every report must include evidence
class, exact subject, findings, gates, and verdict. Heavy work must use
`comparator_suite_runner`; unavailability is `HOLD`, with no parent fallback.

Revision note (2026-07-27, Codex): expanded the initially reviewed scaffold to
restore autonomous DC shape, prospective gate planning, exact delegated-role
authority, required evidence placeholders, and bounded predecessor claims.
