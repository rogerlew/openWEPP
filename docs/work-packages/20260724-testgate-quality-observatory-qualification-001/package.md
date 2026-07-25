# TESTGATE Changed-Head Qualification After Quality Separation

Package ID: `20260724-testgate-quality-observatory-qualification-001`

Status: `ACTIVE`

## Pre-Execution Intent

Risk: `CRITICAL`.

This package performs the roadmap's one changed-head functional TESTGATE
qualification. Before publication or dispatch it will run the repository-owned
cheap source/schema/focused checks, reconcile the exact base-to-head authority
chain, record the current provider queue state, and retain a dispatch-intent
artifact binding:

- active authority/base commit
  `086244c889c20de823fd1fa5b02d3527ecffa236`;
- this exact intent package path;
- the final pushed `main` head; and
- one TESTGATE workflow dispatch per corrected stable head.

The workflow itself owns LIGHT execution, the canonical ten-check pre-heavy
audit, forest1 HEAVY admission, receipt production, hosted independent
verification, and retained recovery evidence. The parent does not execute
heavy gates locally. The authorized comparator runner monitors provider state
and retained artifacts read-only.

If the attempt exposes an in-scope repository tooling defect, evidence is
retained and the defect is corrected before a changed-head retry. An unchanged
failure is never rerun. No QA, coverage, CRAP, or CQR workflow is selected.

Terminal acceptance requires the exact receipt and plan to prove
`DEFERRED_TO_QUALITY_CI`, no quality execution node, truthful
`LOCAL_UNTRUSTED`, valid recovery/archive/ledger evidence, and explicit
rejection of incompatible pre-split evidence.

## Objective

Qualify changed-head TESTGATE on forest1 after quality separation, iteratively
correcting in-scope tooling defects until the blocking correctness workflow has
direct functional evidence.

## Included Scope

- Exact-head TESTGATE dispatch and retained receipt verification.
- Incompatible pre-split receipt recovery treatment.
- Proof that quality execution is absent and typed deferral is present.
- Iterative diagnosis and correction of in-scope TESTGATE tooling defects.

## Excluded Scope

- QA dispatch, coverage collection, CRAP execution, or CQR intake.
- Science implementation or CQR module refactors.
- Relaxing policy to convert execution corruption into success.
- Canceling, waiting for, or otherwise mutating defunct Omarchy queue records.
- Repeated reassurance dispatches.

## Declared Write Set

- `.github/workflows/testgate-shadow.yml`
- `gate-policy/v1/**`
- `crates/openwepp-gate-planner/**`
- `tools/local_ci/**`
- `tests/integration/testgate_*`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/work-packages/20260724-testgate-quality-observatory-qualification-001/**`
- `docs/work-packages/20260724-cqr-testgate-coverage-reconstruction-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/testgate-quality-observatory-roadmap.md` (Order-6
  closeout routing only)
- `docs/ROADMAP.md` (Order-6/Order-7 routing only)

## Dependencies

- Orders 1 through 5 complete and pushed.
- Current forest1 runner available.
- No live current TESTGATE run at dispatch. Permanently queued Omarchy records
  are ignored.

## Execution Protocol

1. Run cheap source, schema, focused, inventory, and pre-heavy checks first.
2. Commit and push one stable qualification head.
3. Prove current forest1 TESTGATE idle, ignoring defunct Omarchy records.
4. Dispatch exactly one TESTGATE against the current head and inspect all
   retained evidence.
5. If a repository-owned tooling defect appears, retain evidence, correct it
   within the declared write set, amend the head, and requalify. Do not repeat
   an unchanged failing attempt.
6. Verify recovery archive/ledger, incompatible old-receipt treatment, and the
   absence of quality execution nodes.
7. Complete reviews, verifications, disposition, commit, and push.

## Exit Criteria

- Current-head TESTGATE passes without a coverage/CRAP execution node and its
  receipt records `DEFERRED_TO_QUALITY_CI`.
- Receipt remains valid with its truthful `LOCAL_UNTRUSTED` forest1 trust class.
- Exact base/head/package, selected blocking nodes, receipt, recovery archive,
  and ledger independently verify.
- A retained incompatible pre-split receipt is rejected without blocking the
  fresh execution.
- No defunct Omarchy record is awaited, canceled, or counted as forest1
  occupancy.
- Every failed attempt has a typed cause and disposition; no known in-scope
  invariant remains.
- Dual result review, dual terminal verification, documentation/path checks,
  and security review pass.

## Security Impact

External receipts and provider state are untrusted until verified. Heavy
candidate code remains forest1-only. Hosted verification may validate compact
evidence but does not execute heavy suites.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to a `comparator_suite_runner` for all heavy workflow monitoring/evidence
collection, two read-only result reviewers, and two read-only terminal
verifiers; expected outputs are compact metrics, run/log/artifact paths, and
package artifacts; write access is read-only. Only the parent may commit, push,
or dispatch workflows.
