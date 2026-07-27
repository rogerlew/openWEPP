# Assurance Canopy README Identity Lifecycle Reconciliation

This Defect-Closure ExecPlan is a living document governed by
`docs/codex_exec_plans.md` and `docs/defect_closure_execplans.md`.

Package ID:
`20260727-assurance-canopy-readme-identity-lifecycle-reconciliation-001`

Status: `ACTIVE`

Execution mode: `package-end-to-end`

## Objective

Close defect `ASSURANCE-CANOPY-README-IDENTITY-001`: commit `502dd745`
materially updated the admitted canopy-fixture documentation without a governed
assurance generation transition, leaving the snow/frozen-soil `IN_REVIEW`
report bound to the predecessor README identity and blocking the full workspace
gate.

The correction must adopt the current declared dependency through a typed,
atomic assurance transaction while invalidating review authority bound to the
old content. It must not manually edit hashes, fabricate human review, revert
the admitted CAL03 documentation, or weaken source-drift validation.

## Correction Authority Envelope

Observed failure:

```text
generated identity member changed: tests/fixtures/cancov_forest/README.md
```

Current README SHA-256:
`b81fbe2efa5624e5018c18f24c55ada53d7c484ff020b19d6fa1deae8bd1dd7b`.

Bound predecessor SHA-256:
`703a138076900f24a3232457dfab8744e60f69ab196b4b361eeb12bbfedb268c`.

The README is dependency `SF-DEP-CANOPY-README` of
`snow-and-frozen-soil-process-evaluation`. The report and its current review
lock are `IN_REVIEW`; the active review-entry event binds the old content
subject. Canonical assurance authority states that regeneration may invalidate
human authority but cannot create or carry a human decision to different
reviewed bytes.

### Intended write set

- `crates/openwepp-assurance/src/v2/amendment.rs`
- `crates/openwepp-assurance/src/v2.rs`
- `crates/openwepp-assurance/src/lib.rs`
- `crates/openwepp-assurance/src/cli.rs`
- `tests/integration/assurance_v2_amendment_contract.rs`
- adjacent CLI/unit tests in `crates/openwepp-assurance/src/cli.rs`
- `assurance/v2/README.md`
- generated transaction output under `assurance/v2/transactions/`
- generated `assurance/v2/identity.lock.json`
- generated
  `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json`
- mechanically updated authored lifecycle fields in
  `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/report.yaml`
- this package, the CAL04B-NATIVE package disposition/handoff, and
  `docs/work-packages/README.md`

### Allowed edit classes

- add a typed `amend adopt-report-source` check/apply operation;
- accept exactly one currently drifted, report-declared external
  `local_content` dependency;
- reject undeclared, non-local-content, in-tree assurance, unchanged, multiply
  drifted, stale-generation, or concurrently changing paths;
- mechanically reset `IN_REVIEW` report/review state to `DRAFT`, clear
  review-entry authorization fields, and invalidate all active review events;
- atomically regenerate review lock, identity lock, and deterministic receipt;
- add contract-derived transaction, rollback, CLI, negative-path, and
  idempotence tests;
- update lifecycle documentation and package evidence.

### Protected boundaries

- do not directly edit any generated digest or event identity;
- do not synthesize a review-entry, approval, finding, disposition, withdrawal,
  supersession, or release-transfer event;
- do not retain `IN_REVIEW` for bytes not covered by its active review event;
- do not revert or alter `tests/fixtures/cancov_forest/README.md`;
- do not change report scientific prose, results, claims, methods, or public
  publication state;
- do not run calibration population or Harvard execution.

## Conversion Rule And Seven-Gate Bar

The reproducible mechanism lies inside the declared assurance implementation
and generated-state envelope. Canonical lifecycle authority requires review
invalidation rather than manual rebinding. Therefore this package must proceed
through typed operation design, tests, implementation, real transaction,
validation, review, and closure; it may not stop at diagnostic HOLD while those
actions remain possible.

1. Reproduction: exact README identity drift blocks assurance repository open.
2. Mechanism: an admitted external source changed without a generation
   transaction.
3. Ownership: amendment, transaction, lock, report lifecycle, and tests are in
   scope.
4. Authority: `assurance/v2/README.md` forbids hash edits and carrying review
   authority to changed bytes.
5. Safety: the operation must invalidate authority and remain fail-closed.
6. Testability: an isolated fixture can mutate one declared dependency and
   prove check/apply, rollback, negatives, and idempotence.
7. Validation: real validate/inspect/plan/build plus the unfiltered full profile
   directly show closure.

## Implementation Intent

Classification: `assurance lifecycle implementation` plus a
`scientific-full` report-data transaction. This is not scientific-content
revision, calibration, approval, publication, or release transfer.

The intended operation consumes an explicit report ID and declared external
source path. Check mode calculates the complete candidate without writing.
Apply mode uses the existing confined copy-on-write exchange, external read-set
snapshot, compare-and-swap generation check, candidate validation, deterministic
receipt, and recovery behavior.

For an `IN_REVIEW` report, adoption sets:

- report lifecycle and authored review state to `DRAFT`;
- review decision to `not_started`;
- review charge/build maintainer to null;
- material producers, findings, and approvals to empty;
- independence assessment to `not_assessed`;
- agent-assistance review-entry authorization to false;
- every active event ID to generated invalidated-event custody.

The operation records the current external source hash only after proving the
path is the target report's declared `local_content` dependency and the sole
allowed preexisting drift.

## Phase Plan

1. Commit this authenticated scaffold before implementation.
2. Add contract-derived fixture tests and record their pre-fix failure.
3. Implement the typed library and CLI operation.
4. Run focused assurance and anti-evasion gates.
5. Build the exact release assurance binary, run check, then apply the real
   transaction and retain its receipt.
6. Validate the scientific-full receipt and generation chain, run
   validate/inspect/plan/build, and execute the implementation-package and
   unfiltered full-workspace gates. The focused receipt runner is intentionally
   inapplicable to `scientific-full`.
7. Reopen CAL04B-NATIVE terminal verification, complete dual review and dual
   verification, disposition findings, archive prompt, and close both packages
   only if every gate passes.

## Required Acceptance

- No direct hash edit occurs.
- Check mode is read-only and deterministic.
- Apply mode updates only the declared source identity, affected report
  lifecycle/lock, global identity, and deterministic receipt.
- Old active review events are invalidated; no new human-authority event is
  created.
- The report validates as `DRAFT`, the current README identity is bound, and a
  repeat check is a no-op.
- Wrong report/path/kind, unchanged path, second drift, stale generation, and
  external race fail without writes.
- `validate`, `inspect`, `plan`, disposable `build`/`check`, generation-chain
  verification, anti-evasion gates, and exact-head full workspace profile pass.
- CAL04B-NATIVE terminal verification passes before its prerequisite is lifted.

Coverage/CRAP disposition: `DEFERRED_TO_QUALITY_CI` per ADR-0041.

## Review And Delegation

Subagent requirement: REQUIRED. This package explicitly authorizes subagent
spawning/delegation to two independent read-only Rust/assurance reviewers, a
`comparator_suite_runner` for the exact-head full workspace profile, and two
independent read-only terminal verifiers. Expected outputs are compact verdicts,
test counts, receipt IDs, hashes, and artifact paths. The primary executor owns
all writes.

## HOLD Legitimacy

HOLD is legitimate only if the typed confined transaction cannot represent a
review-invalidating source adoption without violating canonical lifecycle
authority, or a human decision is strictly required before safe DRAFT custody
can be restored. Any HOLD must populate a boundary audit naming evidence,
considered in-envelope routes, and the next defect.

## Progress

- [x] Reproduced and attributed the stale external source identity.
- [x] Authored the implementation intent and correction envelope.
- [ ] Committed scaffold.
- [ ] Added pre-implementation contract tests.
- [ ] Implemented typed source adoption.
- [ ] Applied and validated the real transaction.
- [ ] Passed dual review and dual terminal verification.
- [ ] Closed this package and lifted CAL04B-NATIVE.

## Surprises And Discoveries

- The existing `rebind-implementation` operation intentionally accepts only a
  finite assurance schema/README implementation surface and correctly rejects
  report evidence drift.
- The current CLI has no typed operation for adopting changed external report
  evidence while invalidating active review authority.

## Decision Log

- Decision: preserve the CAL03 README and reset the affected report to DRAFT.
  Rationale: the new bytes are admitted evidence, but the old review-entry
  decision cannot be carried to a new content-review subject.
  Date/Author: 2026-07-27 / Codex.

## Outcomes And Retrospective

Pending execution.

## Defect-Shaped Handoff

First actionable item: close defect
`ASSURANCE-CANOPY-README-IDENTITY-001` end-to-end through the typed transaction,
real apply, full gate, and CAL04B-NATIVE hold lift. Do not relay a manual hash
edit or another diagnostic-only package.
