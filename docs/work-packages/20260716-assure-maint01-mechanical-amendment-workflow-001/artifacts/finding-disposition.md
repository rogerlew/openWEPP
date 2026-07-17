# Implementation Finding Disposition

Evidence class: Static and Ran

All implementation-review findings were accepted and corrected in current
scope.

## Identity And Lifecycle

1. Accepted: finite event-type/decision mappings now reject negative or unknown
   decisions before authority or state transition. Generated event schema and
   production validation use the same admitted types.
2. Accepted with architecture correction: approval realization now binds the
   complete deterministic input projection and an embedded digest of identity,
   assembly, lifecycle, planning, and publication implementation. Build/check
   and publication independently bind exact observed staged bytes. The
   specification now states this noncircular two-part contract explicitly.
3. Accepted: role and principal-governance mutation is limited to `DRAFT` and
   `IN_REVIEW`; `APPROVED` requires a current approval lock. Withdrawal and
   supersession now set terminal report/review states, reset publication
   authority, invalidate active events, and retain the terminal event.
4. Accepted: steward events bind all scientific, communication, governance,
   ledger, realization, attribution, and predecessor inputs. Release transfer
   must name the principal on its exact steward predecessor.
5. Accepted: a validation-enforced top-level classification must exactly equal
   the typed report field set. Science, communication, and review-governance
   projections bind their full stable inputs; lifecycle and publication are
   explicitly event-governed so legitimate transitions do not change the
   content subject.
6. Accepted: generation validation checks canonical receipt bytes,
   content-addressed names, predecessor equality, and the archived chain. The
   focused runner requires the selected receipt to be the unique direct archive
   transition to the active generation.
7. Accepted: schemas admit and tests validate `role_assignment` and
   `principal_version` events.

## Workflow And Mechanization

1. Accepted: committed-cleanup failure returns the committed receipt, leaves
   the new generation active, and leaves typed recovery state.
2. Accepted: focused gate ID is derived exactly from impact class; gate argv is
   the pinned nextest command; generic, off-archive, symlinked, duplicate, or
   non-current receipts fail before execution.
3. Accepted: external files and directory trees are captured and copied through
   confined directory descriptors with mode and byte revalidation immediately
   before exchange.
4. Accepted: held generations are verified before restore, and inspection
   reports both active and held identities.
5. Accepted: every selected crate unit test is named explicitly in the profile
   filter; integration selections are also explicit, so future tests do not
   enter silently.
6. Accepted: ten current and ten scaled trials are recorded in
   `performance-evidence.md`. The scaled transaction limit is explicitly 10
   seconds because this filesystem rejects reflinks; current-corpus and
   end-to-end limits are unchanged.
7. Accepted: principal and lifecycle checks now prove repeated byte-identical
   candidate receipts; normalization and transaction determinism remain covered
   by their focused contracts.

## Closure Discoveries

1. Accepted: one-time intermediate and final migration commands and the
   old-algorithm loader were removed after the tracked generation was migrated.
   The production parser now accepts only the current identity algorithm.
2. Accepted: a permanent typed `amend rebind-implementation --all` operation
   now recalculates generated locks after assurance implementation changes,
   without editing authored sources or authority events. It fails closed on
   incompatible approval bindings and repeats as a no-op.
3. Accepted: high-complexity identity, approval, principal, generation-chain,
   and recovery functions were decomposed. Selected-generation recovery
   verification and implementation-rebind CLI coverage were added to the
   pinned focused profile.
4. Accepted from superseded terminal verification: implementation rebinding is
   classified `scientific-full`, names the implementation-package gate, and
   carries no focused argv. The focused runner therefore cannot authorize an
   implementation or schema change.
5. Accepted from superseded terminal verification: the performance campaign
   was repeated against the final release binary and final 45-test profile.
   Ten current and twenty scaled observations meet the transaction, p95,
   maximum, and hard-regression limits while retaining the scaled 60.197-second
   maximum.
6. Accepted from superseded terminal verification: the pinned amendment
   contract now rejects mismatched gates, mismatched argv, forbidden
   escalation tokens, schema paths, non-current generations, duplicate
   transitions, noncanonical receipts, symlinks, and off-archive receipts.

Ran: `cargo nextest run --workspace --profile assurance-amendment` passed 44 of
44 selected tests after implementation-review disposition. After terminal
finding disposition expanded the runner-negative matrix, the final profile
passed 45 of 45 selected tests.
