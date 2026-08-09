# Assurance Draft-Publication Defect Closure

Status: `complete / dual terminal verified`

Date: `2026-08-09`

Package ID: `20260809-assurance-draft-publication-defect-closure-001`

Plan class: `Critical assurance defect closure`

This ExecPlan is a living document maintained under `docs/codex_exec_plans.md`.

## Objective

Close the stable exact-head failure in
`assurance_v2_publication_contract::draft_subject_root_is_stable_but_cannot_publish`
without weakening publication confinement, trust-domain separation, review,
approval, release-transfer, or lifecycle gates. Restore a deterministic typed
rejection that identifies the selected report's `DRAFT` lifecycle before any
publication side effect.

## Authority And Trigger

The user explicitly granted assurance defect-closure authority on 2026-08-09.
The trigger is the isolated and Critical full-workspace failure discovered while
executing
`20260809-native-forest-ecohydrology-authority-reframe-001`. This package owns
the bounded closure and, after exact-head validation, may lift that package's
unrelated-gate hold.

## Included Scope

- Diagnose and reproduce the exact typed error returned for a canonical DRAFT
  report through the production publication entry point.
- Add or strengthen failing-first regression coverage for lifecycle rejection
  ordering and absence of public side effects.
- Make the smallest assurance implementation or fixture correction needed to
  restore the governed behavior.
- Run focused assurance tests and the Critical exact-head full-workspace gate.
- Complete primary Rust review, secondary Rust QA review, and two independent
  terminal verifications.
- Reconcile and lift the vegetation authority-reframe hold only if the exact-head
  full-workspace gate passes.

## Excluded Scope And Protected Invariants

- No scientific-assurance lifecycle transition, approval, publication, or
  release-transfer operation is authorized.
- No weakening of trust-domain separation, root confinement, exact staging,
  identity binding, approval locks, immutable snapshots, or fail-closed errors.
- No change to scientific report content or lifecycle state.
- No unrelated assurance refactor or generated-publication update.
- A DRAFT source must remain non-publishable and must create no public catalog,
  snapshot, or receipt.

## Intended Write Set

- This package tree.
- `tests/integration/assurance_v2_publication_contract.rs`.
- `crates/openwepp-assurance/src/v2.rs` and
  `crates/openwepp-assurance/src/v2/publication.rs` only if diagnosis proves a
  production defect.
- Assurance specification or README text only if the governed precedence was
  ambiguous and requires explicit clarification.
- Lifecycle artifacts for
  `20260809-native-forest-ecohydrology-authority-reframe-001` and repository
  package/backlog indexes solely to lift its recorded blocker after a passing
  exact-head full-workspace gate.

## Contract-First Sequence

1. Freeze the failure evidence, authority, protected invariants, and write set.
2. Expose the exact returned error and classify whether the defect is in the
   test fixture, gate invocation, entry-point preconditions, or publication
   validation order.
3. Strengthen regression diagnostics to name an unexpected typed rejection and
   prove no public/snapshot/receipt side effects.
4. Make the smallest fail-closed source correction if one is proven; otherwise
   correct the invalid gate invocation and do not perturb production code.
5. Reconcile the terminal diff and run exact-head Critical full-workspace
   validation.
6. Complete independent Rust correctness review, Rust QA review, and dual
   terminal verification; disposition every finding.
7. Archive the kickoff prompt and record terminal package disposition. Lift the
   vegetation package hold only on passing reusable exact-head evidence.

## Validation Plan

```bash
markdown-doc lint --path docs/work-packages/20260809-assurance-draft-publication-defect-closure-001 --format plain
cargo fmt --all -- --check
cargo clippy --test assurance_v2_publication_contract -- -D warnings
cargo nextest run --test assurance_v2_publication_contract draft_subject_root_is_stable_but_cannot_publish --profile quick
cargo nextest run --workspace --profile full
```

`TMPDIR=/home/workdir/openwepp-task-tmp` is the approved high-capacity scratch
root for these assurance tests because it is outside the repository and the
system `/tmp` volume was previously exhausted. An in-repository `TMPDIR` is
invalid for publication tests because their confinement contract correctly
requires repository, staging, public, and snapshot roots to be unrelated. The
full-workspace gate must be delegated to `comparator_suite_runner` and its exact
head recorded.

The full-workspace run is the reusable terminal evidence for the complete
`assurance_v2_publication_contract` target and `openwepp-assurance` crate. Their
standalone quick-profile attempts were superseded after campaign-scale cases
exceeded the quick timeout under concurrent load; they are recorded, not
misrepresented as required failing correctness gates.

## Review And Delegation Requirements

The executing agent is explicitly authorized to spawn subagents for this
package. Required roles are:

- one `rust_code_reviewer` for the primary correctness gate;
- one `rust_qa_reviewer` for the secondary tests/maintainability gate;
- two independent terminal verifiers after finding disposition; and
- one `comparator_suite_runner` for the exact-head full-workspace gate.

Reviewers and verifiers inspect the exact current diff and write independent
artifacts. Author self-review does not satisfy these gates.

## Exit Criteria

- The isolated test's original exact error is recorded.
- The invalid in-repository scratch invocation fails for the correct confinement
  reason and the corrected external-scratch invocation passes.
- DRAFT publication rejection is deterministic, typed, explicit, and has no
  public, snapshot, or receipt side effect.
- All focused assurance gates and the exact-head full workspace pass.
- Rust reviews and dual terminal verifications have no unresolved material
  finding.
- Package lifecycle, prompt archive, terminal diff, and downstream hold lift are
  truthful and complete.

## Progress

- [x] 2026-08-09: User granted assurance defect-closure authority.
- [x] 2026-08-09: Stable isolated and full-workspace failure identified.
- [x] 2026-08-09: Applicable repository, test, crate, documentation, and
  work-package instructions inspected.
- [x] 2026-08-09: Exact returned error and defect ownership diagnosed.
- [x] 2026-08-09: Diagnostic regression strengthened; no production correction
  is warranted because the confinement behavior is correct.
- [x] 2026-08-09: Focused formatting, Clippy, isolated regression, Markdown,
  and exact-head full-workspace gates pass; full workspace ran 2,325/2,325.
- [x] 2026-08-09: Required Rust reviews are unconditionally approved and the
  vegetation package's unrelated-gate hold is lifted.
- [x] 2026-08-09: Kickoff prompt archived byte-for-byte with digest
  `a68817d4fc21bad2ad2f55fd9532109172646374da6de123831e19bf328c9a5b`.
- [x] 2026-08-09: Dual terminal verification passed after two accepted
  lifecycle-wording findings were fixed and independently rechecked; final
  lifecycle reconciliation is complete.

## Decision Log

- 2026-08-09: Treat the failure as an assurance defect, not as permission to
  narrow or defer the vegetation package's Critical gate.
- 2026-08-09: Preserve lifecycle rejection before approval validation for a
  selected DRAFT report after safe publication-root validation.
- 2026-08-09: The failing command was invalid because
  `TMPDIR=/home/workdir/openWEPP/target/task-tmp` made staging a repository
  descendant. Use `/home/workdir/openwepp-task-tmp`; never weaken root
  confinement to accommodate an in-repository test scratch path.

## Surprises And Discoveries

- The production validator already checks report lifecycle before approval
  locks once a report context is loaded; an earlier operation is masking that
  intended decision in the failing path.
- The masking operation was the intentionally prior root-confinement gate, and
  it fired only because the disk-remediation invocation selected an
  in-repository temporary root.
- The canonical `markdown-doc mv` again remained CPU-bound and timed out after
  60 seconds without a filesystem change; a direct same-filesystem rename
  preserved the kickoff prompt digest.

## Outcomes And Retrospective

The reported assurance assertion was an invocation-topology defect, not a
production publication defect. An in-repository `TMPDIR` made the staging root
a repository descendant, so the deliberately prior root-confinement gate
correctly rejected publication. External high-capacity scratch reached the
intended DRAFT lifecycle rejection.

The regression now requires the exact report-specific lifecycle error and
proves complete public-tree non-mutation plus absence of snapshots/receipts.
No production assurance Rust changed. Formatting, warnings-denied Clippy, the
isolated test, documentation gates, and the exact full workspace pass; primary
Rust review and secondary QA review are unconditionally approved. The
vegetation package's exact hold condition is satisfied without weakening
assurance or reopening site-value selection.

Two independent terminal verifiers replayed the isolated external-scratch pass,
checked the expected in-repository confinement failure, verified the reviewed
test blob and full-run log identity, and passed the terminal diff. Their two
lifecycle-wording findings were accepted and fixed before final PASS.
