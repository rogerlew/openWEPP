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

---

## Re-review — corrected candidate `675296fdb262efd052be40d32d6730b3d895220a`

Status: `HOLD`

Evidence mode: `Static + Ran`

Mechanical evidence: strict BEI PASS, unit-compliance PASS, authority tests
4/4 PASS, and calculator 46/46. Semantic closure remains incomplete.

### `V11-TXN-B-001` — PARTIAL, remains BLOCKER

Version 4 is now integrated into purpose, scope, state surfaces, algorithm,
branch table, obligations, and test obligations, and the Change Log is terminal.
However the body still says `approved/active` while frontmatter says
`in_review/draft`; producer scope still names V8 only; the Symbol and Unit maps
remain water/energy-only; and the Invariant Guard Map still ends at
`INV-VEGTRANSACTION-008`, omitting enforcement/evidence rows for 009--013.
Reconcile every lifecycle/profile surface before approval.

### `V11-TXN-B-002` — PARTIAL, remains BLOCKER

The ordered binary64 fold policy and separate water/NH4/NO3 keys are now
normative. The reference still uses anonymous decimal strings and aggregate
totals rather than typed parent/segment/slab/owner/OFE/tile/occupancy/layer/
species/basis receipts, staged authorization/final use, and bit-exact ending
owners. Energy, thermal, and material identities remain unexecuted. Add typed
wire cases, staged owner advancement, exact subtraction reconstruction, and
identity/order aliases.

### `V11-TXN-B-003` — PARTIAL, remains BLOCKER

The amendment now enumerates candidate fields and consuming semantics, but no
closed candidate/parent-receipt schema, complete-owner manifest/cardinality,
typed field/framing definition, or executable consume transition exists. The
oracle hardcodes one increment/commit and cannot detect a forged/partial/
reordered candidate or late owner failure. Freeze and execute this wire/API.

### `V11-TXN-B-004` — PARTIAL, remains BLOCKER

Restart fields were expanded, but the generic receipt shape is invalid for
non-slab event/scheduled/publication chronology, embedded JSON remains open,
and the promised semantic validator/cross-wire authentication is not
implemented. Restart vectors compare claimed digest strings without parsing or
continuation. Define phase/type unions and execute canonical restore,
reconstruction, continuation, abort, reduction/publication, and replay cases.

### `V11-TXN-B-005` — OPEN, remains BLOCKER

Forty-six results do not constitute the transaction population when events are
only counted, participants only length-checked, restart is string equality,
consecutive parents are independent, publication is an input-list hash,
commit counts are constants, and materials/typed owners are absent. Replace
flags with constructed authenticated transaction objects and operand-level
poisons.

### Re-review verdict

`HOLD / Version 4 remains non-releasable at 675296fdb`.

The correction improves normative direction but does not yet supply the closed,
independently reconstructed owner transaction required before Rust.

---

## Final re-review — candidate `c7ec8e73096f9816ffbe812ac15deeba1d2b8574`

Status: `HOLD`

Evidence mode: `Static + Ran + adversarial probe`

Mechanical/independent gates pass: BEI, unit compliance, semantic validator
with 21/21 poisons, and authority tests 5/5. The typed receipt/candidate model is
a substantive improvement, but the transaction is not yet releasable.

### `V11-TXN-B-001` — OPEN — profile integration contains a missing invariant

Lifecycle status, producer/consumer scope, symbols/units, and guard-map rows
were corrected. The edit accidentally removed the normative invariant-table
statement for `INV-VEGTRANSACTION-010`; the ID now appears only in its guard-map
row and BEI. The canonical staged-inventory/cumulative-fold invariant therefore
has enforcement references but no normative statement. Restore that invariant
and rerun BEI/profile gates.

### `V11-TXN-B-003` — OPEN — candidate can commit from the wrong live owners

The new candidate schema and `AtomicStore` prove fixed class order, one-shot
consumption, stale-clock rejection, publication-after-install, and rollback.
They do not join candidate beginning owners to live store owners. A read-only
probe replaced the vegetation beginning digest with zeros, recomputed the
parent receipt, and the validator and store both accepted it. Commit must reject
unless every live owner digest and typed manifest entry exactly equals the
candidate beginning.

Further, `V11CompleteOwnerManifestV1` prose requires owner IDs, expected count,
schema/model/configuration IDs, active disposition, and within-class ordering;
the schema contains only seven class strings plus seven digest arrays. Encode
the declared manifest rather than a one-owner-per-class surrogate.

### `V11-TXN-B-004` — OPEN — checkpoint proof does not use the additive restart

The independent model restores a package-local `CHECKPOINT_TEST_V2`, not
`OPENWEPP_C3_WOODY_V11_RESTART_V1`. It then validates and commits the already
finished candidate instead of continuing remaining slabs/events/resources from
fresh restored owners. No reduction/publication/outbox or coupled-time V2 join
is restored. Construct, admit, restore, and continue the actual closed restart
wire before claiming equivalent continuation.

### Other prior transaction findings

- `V11-TXN-B-002` is materially closed for the reference water/NH4/NO3 staged
  folds and authenticated receipt arithmetic.
- `V11-TXN-B-005` is materially closed for the parent-candidate/event/material/
  publication poison population, subject to the live-owner and actual-restart
  blockers above.

### Final verdict

`HOLD / INV-010 authority, live-owner authentication, complete manifest, and
actual restart continuation remain open at c7ec8e730`.

No production Rust may begin from this checkpoint.

---

## Terminal review — candidate `205e0ad4e628044093e42eb99388fbbac6942d2c`

Status: `HOLD`

Evidence mode: `Static + Ran + adversarial restart probes`

BEI, unit compliance, semantic validator 26/26, authority tests 5/5, and diff
hygiene pass. `INV-VEGTRANSACTION-010` is restored and the independent live-
owner/clock/ending-owner/consuming-commit paths now execute correctly.

### `V11-TXN-B-001` — CLOSED

Lifecycle surfaces, producer/consumer scope, symbol/unit maps, guard map, and
the normative staged-custody invariant are now internally present.

### `V11-TXN-B-003` — PARTIAL

The live beginning-owner and clock joins now reject independently forged state;
ending owners are reconstructed from authenticated receipts; one-shot commit,
publication ordering, and rollback execute. The typed manifest still fixes
exactly one owner ID per seven classes, while canonical prose permits multiple
owners within a class and requires within-class ID order/count. Either freeze
one aggregate owner per class normatively or encode the actual variable owner
set before promotion.

### `V11-TXN-B-004` — OPEN — restart wire fields are not fully authenticated

The exact V11 restart schema is now constructed and a suffix is executed, but
restore accepts independently forged reduction value, outer staged-state
digest, active participant set, and next parent transaction sequence. These
were direct probes, not inferred gaps. Scheduled/material prefixes,
publication/outbox, phase/cursor, authority/config identity, and current
sequence are likewise not reconstructed by the restore path. Add complete
field-by-field admission and poisons before claiming restart custody.

### Remaining transaction disposition

`V11-TXN-B-002` is closed for the reference water/NH4/NO3 fold and
`V11-TXN-B-005` is closed for the declared non-restart parent poison surface.
Neither supersedes the restart owner-chronology blocker.

### Terminal verdict

`HOLD / Version 4 cannot release while its restart continuation accepts forged
accepted chronology and publication/reduction state`.

Do not begin production Rust.

---

## Re-review — candidate `ab07b1cf62b3da4299baf4ce045ebecccd85911e`

Status: `HOLD`

Evidence mode: `Static + Ran + adversarial restart probes`

BEI, unit compliance, semantic validator 34/34, authority tests 5/5, and diff
hygiene pass. `INV-VEGTRANSACTION-010`, typed staged folds, owner descriptors,
live owner/clock authentication, and actual suffix execution remain present.

### `V11-TXN-B-004` — OPEN — retained restart custody is forgeable

The expanded validation closes all eight newly declared checkpoint poisons, but
two independent mutations still pass:

- a forged `parent_beginning_owner_sha256` entry;
- a forged accepted material receipt payload digest.

The restore path never joins the retained parent-beginning array to candidate/
live beginnings, and it checks material prefix identity IDs without decoding
and authenticating the receipt payload. Consequently equivalent continuation
is established from a trusted complete candidate while contradictory restart
custody remains admitted.

Required closure: authenticate every retained owner beginning and complete
receipt envelope from restart bytes, rebuild receipt IDs/order, and poison every
receipt category's payload, digest, identity, omission, duplicate, and reorder.

### Remaining status

- `V11-TXN-B-001/002/003/005`: materially closed for the aggregate-owner
  reference transaction, subject to reconciling the contract's multiple-owner-
  within-class language with the exactly-seven-descriptor schema.
- Complete V10 configuration/state migration remains an upstream
  `SC-VEGETATION-001` blocker and prevents transaction release even though
  `INV-010` is restored.

### Verdict

`HOLD / restart owner and material custody remain unauthenticated at ab07b1cf6`.

Do not promote or begin Rust implementation.
