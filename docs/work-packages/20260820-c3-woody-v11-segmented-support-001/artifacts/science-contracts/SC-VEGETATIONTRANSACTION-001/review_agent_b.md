# Authority Review B — Parent Transaction And Owner Custody

Status: `HOLD`

Evidence mode: `Static + Ran`

Reviewed exact commit:
`5f4d3902065c316309785cc47ac63f766884bbd1`.

## Ran evidence

- strict Binding Exposure Index check => PASS, 4 rows consolidated;
- science-contract unit-compliance check => PASS, no findings;
- V11 authority contract test in the Nix shell => PASS, 3/3.

## Findings

### `V11-TXN-B-001` — BLOCKER — V11 is appended, not integrated into normative surfaces

Frontmatter declares Version 4 `in_review/draft`, while the body still declares
`approved/active` and defines Purpose, scope, producer/consumer surfaces,
algorithm, branch table, test obligations, and symbols only for V8/LSE. The V11
amendment is after the Change Log. New invariants 009--013 have no Invariant
Guard Map entries; the branch table has no `VEGTXN-E-011..014`; and producer/
consumer obligations omit staged V11 custody/restart/finalization. A consumer
can legitimately miss Version 4.

Required disposition: reconcile lifecycle and integrate Version 4 into Purpose/
scope, state surfaces, algorithm, branch/guard table, guard map, obligations,
symbols/units, test-vector obligations, gap register, and change-log placement.

### `V11-TXN-B-002` — BLOCKER — typed resource identity and arithmetic are incomplete

Version 4 extends “the existing tuple,” but that tuple is water-specific and
does not canonically define NH4/NO3, energy, thermal, or material identity and
amount bases. It binds neither amount encoding, receipt order, reconstruction
arithmetic, staged authorization/final-use bounds, nor exact beginning-to-final
owner joins. One anonymous debit vector cannot detect owner/species/layer/basis
aliases.

Required disposition: define closed typed tuples and amount bases for every
resource; preserve NH4 and NO3 separately; bind deterministic order/arithmetic;
require independent reconstruction of authorization, final use, cumulative
debit, and ending inventory.

### `V11-TXN-B-003` — BLOCKER — complete-owner finalization is prose-only

The amendment does not define its closed candidate/API, fixed complete-owner
membership, active/inactive rules, canonical order/cardinality, digest framing,
parent receipt identity, exact live preconditions, or how consuming
finalization prevents a second call. `VEGTXN-E-013` merges several failures
without precedence or guard binding.

Required disposition: freeze candidate/receipt types, owner manifest/order,
begin/end joins, one-shot semantics, error precedence, and rollback. Poison
reordered/duplicate/missing owners, stale clock, second finalize, early
increment, and late-owner rejection.

### `V11-TXN-B-004` — BLOCKER — restart cannot authenticate equivalent continuation

Version 4 does not bind the V11 restart schema/semantic validator, identify
which owner bytes versus digests persist, or define joins between embedded
coupled-time V2 and duplicated V11 state. The schema uses opaque blobs and
omits reductions/publication, explicit events, and next sequence. It cannot
prove fresh-object, event-boundary, abort, or no-republication equivalence.

Required disposition: import a closed V11 restart wire and mandatory semantic
validator; require canonical complete owner bytes/receipts and cross-wire joins;
execute event, mid-parent, abort, scheduled-once, reduction/publication, and
replay poisons.

### `V11-TXN-B-005` — BLOCKER — transaction/reference poisons are absent

The 14 segmented vectors encode no staged owner snapshots or transaction
receipts. They cannot test wrong identity, stale segment beginning,
per-segment commit, separate water/N overbooking, material reorder, event
custody, restore, partial commit, or publication rollback. Passing tests assert
strings and a count rather than reconstructing custody.

Required disposition: add typed independent transaction vectors/validator for
the complete positive and poison population, asserting exact owner bytes/
digests, receipt order, one increment, and zero publication on rejection.

## Verdict

`HOLD / Version 4 is directionally correct but not releasable transaction authority`.

Structural lint passes do not supersede these blockers. Correct and disposition
them before verification or production Rust.
