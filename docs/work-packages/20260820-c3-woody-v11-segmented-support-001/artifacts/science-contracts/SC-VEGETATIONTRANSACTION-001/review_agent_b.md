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

---

## Release re-review — candidate `c53adab0a91c0ecbe853c884bfe05591826441c5`

Status: `PASS`

Evidence mode: `Static + Ran + adversarial restart probes`

BEI and unit compliance pass for both contracts, the semantic model passes
36/36 poisons with actual restart-suffix equivalence, authority tests pass 5/5,
and diff hygiene passes.

### Transaction finding closure

- `V11-TXN-B-001`: CLOSED. Version 4 lifecycle, scope, maps, guard paths, BEI,
  and the restored normative `INV-VEGTRANSACTION-010` are internally complete.
- `V11-TXN-B-002`: CLOSED. Authenticated resource receipts retain separate
  water/NH4/NO3 staged beginnings, request/authorization/final use, exact
  ending bits, and reconstructed ending owners.
- `V11-TXN-B-003`: CLOSED. The exact seven aggregate owner envelopes are now a
  normative V1 boundary, with typed descriptors, independent live owner/clock
  checks, reconstructed endings, one-shot commit, and late-failure rollback.
- `V11-TXN-B-004`: CLOSED. The exact restart wire authenticates retained parent
  beginnings and full accepted receipt payloads, reconstructs cursor/
  participants/sequences/staged owners/resources, and executes the remaining
  suffix. All prior direct forgery probes reject.
- `V11-TXN-B-005`: CLOSED. The 36-poison executable population covers schema,
  custody, duration, event, material, owner, restart, commit, rollback, and
  publication aliases rather than relying on flags/counts alone.

### Verdict

`PASS / SC-VEGETATIONTRANSACTION-001 Version 4 may proceed to independent
verification at c53adab0a`.

Production Rust remains gated on dual authority verification and the recorded
promoted authority checkpoint.

---

## Restart V2 amendment review — candidate `ac8cb0eda4110d5b5fe8811d82da314b6d8bf25e`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + adversarial surface audit`

Mechanical evidence passes: BEI and unit-compliance for both contracts, the
restart V2 reference fixture with 15/15 declared poisons, schema/poison JSON
parsing, and the V11 authority contract test at 6/6. The following transaction
custody claims remain unproved.

### `V11-TXN-RESTART-V2-B-001` — BLOCKER — complete parent transaction is not restored

The oracle's embedded checkpoint is not the runtime
`V11ParentTransactionCheckpoint`. It omits complete beginning/staged V11
states, accepted segment projections, cumulative typed debits, staged owner
map, resource/material bodies, and the reconstructed suffix predecessor. The
reference returns three scalar facts after validation and never restores a
fresh transaction or executes its unaccepted suffix. This cannot establish
equivalent continuation, replay resistance, staged custody, or parent abort.

Required closure: serialize/admit the complete checkpoint projection, restore
fresh owners and transaction state, execute at least one unequal remaining
segment/event/resource/material suffix, and compare final candidate/owner/
receipt/publication bytes with uninterrupted execution.

### `V11-TXN-RESTART-V2-B-002` — BLOCKER — cross-authority joins and retained ledgers are incomplete

Only a subset of coupled-time facts is joined. Active segment/regime,
controller policy, configuration, current sequence, scheduled keys, and event
predecessor/support chains remain caller-controlled. Reduction sources and
fold order are not reconstructed; publication payload digests are unchecked;
outbox delivery semantics are not validated. Owner blobs are digest-checked
without typed `OwnerState::new` reconstruction or canonical nested-state
admission.

Required closure: field-by-field typed joins to admitted coupled-time V2 and
the complete vegetation checkpoint; reconstruction of every owner, receipt,
scheduled key, reduction, publication record, and outbox transition; and
one-field poisons across the entire retained surface.

### Verdict

`HOLD / SC-VEGETATIONTRANSACTION-001 Version 5 restart amendment is not yet a
complete authenticated continuation authority at ac8cb0eda`.

The additive V2 direction correctly avoids mutating the nonimplementable V1,
but production restart work must remain paused pending correction and another
independent review.

---

## Restart V2 amendment re-review — candidate `a38e2cfa12705a6692ced186b5dc4e51d97ab3f3`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent mutation probes`

The corrected reference passes its one-slab suffix equality and 24/24 declared
poisons; BEI, unit-compliance, and JSON gates pass. The Rust authority test was
blocked by unrelated concurrent orchestrator compilation failures.

### `V11-TXN-RESTART-V2-B-001` — OPEN — accepted receipt custody does not determine restored state

The complete checkpoint-shaped fixture is now present, but restore does not
fold its accepted resource receipts or material transfers into cumulative
debits, staged owners, or staged V11 state. Independently reframed changes to a
water final-use amount, cumulative water debit, and material amount all pass.
The suffix begins from a caller-retained scalar staged value and its equality
is therefore independent of the contradictory retained resource/material
chronology.

Required closure: derive cumulative debits and staged owner/state candidates
from authenticated accepted receipts, prove every checkpoint duplicate joins,
and execute the suffix from only the reconstructed continuation.

### `V11-TXN-RESTART-V2-B-002` — OPEN — regime and publication/reduction commit identities are not closed

Forged active regime, reduction operand ID, and publication record ID all pass
admission. The event transition does not derive the regime, reduction IDs are
not reconstructed in ordered folds, and publication/outbox IDs are not
reconstructed with uniqueness and delivery-state authority. These are retained
transaction facts, not optional diagnostics.

Required closure: bind regime to the accepted event transition and rebuild all
reduction, publication, and outbox identities before continuation exists; add
operand-level duplicate/reorder/identity and outbox transition poisons.

### Superseding verdict

`HOLD / Version 5 restart V2 remains non-releasable at a38e2cfa1 because
accepted custody does not reconstruct staged continuation and retained commit
identities remain forgeable`.

This supersedes the prior amendment review at `ac8cb0eda`. Production restart
implementation must remain paused.

---

## Restart V2 final amendment re-review — candidate `887d92ec557f22682cc5e4df048a20aa249d2cbf`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent mutation probes`

The reference passes real one-slab suffix equality, all 34 declared poisons,
and all six probes retained from `a38e2cfa1`. BEI, unit-compliance, and JSON
gates pass. Resource/material-to-staged-owner reconstruction and derived
reduction/publication IDs are materially closed for the fixture.

### `V11-TXN-RESTART-V2-B-003` — BLOCKER — parent/event chronology can contradict staged custody

Changing current and next parent sequence together is accepted without joining
the checkpoint states' last-parent sequence. Unknown fields in beginning state
and accepted event bodies are accepted. A digest-valid event re-framed with BGC
as source instead of snow is accepted even though staged owner envelopes still
encode snow liquid transfer to surface liquid. Thus the event receipt and
owner transition can contradict each other while suffix continuation succeeds.

Required closure: close nested state/event bodies, bind current sequence to the
checkpoint predecessor, and reconstruct event source/receiver/amount custody
against beginning/staged owner envelopes before returning continuation.

### `V11-TXN-RESTART-V2-B-004` — BLOCKER — durable outbox remains forgeable

An all-zero outbox ID and duplicate identical outbox rows both pass when their
record ID exists and delivery count is zero. The transaction therefore lacks a
reconstructed unique durable publication identity despite otherwise valid
publication records.

Required closure: derive outbox identity from parent/record authority, enforce
closed row shape and unique one-record binding, validate the full state/count
transition table, and poison forged, duplicate, unknown, and conflicting rows.

### Superseding verdict

`HOLD / Version 5 restart V2 is close but not releasable at 887d92ec5 while
parent/event custody and durable outbox identity remain independently
forgeable`.

This supersedes the re-review at `a38e2cfa1`; production restart must remain
paused pending bounded correction and confirmation.

---

## Restart V2 independent final Review B — candidate `937aadb329ced16f050c676f89769fec2d8f5efe`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent reframed probes`

The complete-continuation reference, 52/52 declared poisons, all twelve prior
direct probes, schema/BEI/unit gates, and the 6/6 authority test pass. Current
sequence, closed owner/event bodies, event transfer custody, reduction/
publication ordering and identity, and unique reconstructed outbox identity
are materially closed for the fixture.

### `V11-TXN-RESTART-V2-B-005` — BLOCKER — segment custody does not join to restored staged custody

A digest-valid checkpoint with a forged accepted-segment
`beginning_state_sha256` is accepted. A checkpoint whose segment-ending
vegetation owner is replaced by the parent-beginning owner is also accepted
while checkpoint/outer staged owners remain unchanged. The suffix consumes the
outer staged set rather than proving that it is the exact ending set produced
by the accepted segment chain.

This leaves two contradictory predecessor/owner chronologies inside one
admitted checkpoint and violates the contract's exact sequential parent
custody requirement.

Required closure: reconstruct every segment beginning state digest, chain it
to the preceding ending, validate every segment ending owner envelope, and join
the terminal segment ending set byte-for-byte to both checkpoint and outer
staged owner sets. Add operand-level predecessor and ending-set poisons.

### Superseding verdict

`HOLD / Version 5 restart V2 cannot release at 937aadb32 while accepted-segment
predecessor and ending-owner custody are retained but unauthenticated`.

All previously reported Review B findings are otherwise closed. One bounded
correction and confirmation remain before PASS.

---

## Restart V2 final regression Review B — candidate `5918d4dbdfd0a7641d16b1f5f2040289c9893788`

Date: 2026-08-20

Status: `PASS`

Evidence mode: `Static + Ran + independent reframed probes`

The restart V2 reference passes its complete-continuation digest and 54/54
poisons. Every direct probe accumulated across Review B rejects, including the
exact forged segment predecessor and terminal-owner substitutions. Schema,
BEI, unit-compliance, and the 6/6 authority test pass.

### Transaction finding closure

- Complete typed parent/checkpoint and coupled-time identities join before a
  continuation exists.
- Accepted segment state digests form one predecessor chain and the terminal
  ending-owner set equals checkpoint and outer staged custody byte-for-byte.
- Water/NH4/NO3 and material receipts reconstruct cumulative debits and staged
  owners; event receipts reconstruct regime and snow-to-surface custody.
- Scheduled keys, reduction operands, publication records, and outbox rows are
  closed, ordered, uniquely identified, and replay resistant.
- Fresh suffix execution reconstructs all seven ending owners and the complete
  retained receipt/publication sequence identically to uninterrupted execution.

`V11-TXN-RESTART-V2-B-001..005` are CLOSED. No Review B waiver is required.

### Verdict

`PASS / SC-VEGETATIONTRANSACTION-001 Version 5 Restart V2 amendment may proceed
to independent verification at 5918d4dbd`.

This supersedes all earlier amendment HOLD verdicts. Production remains gated
on required verification and the exact promoted authority checkpoint.

---

## Sequential-debit amendment Review B — candidate `3065c209c7d5d203a2a06fca793dc8cbc340e26e`

Date: 2026-08-20

Status: `PASS`

Evidence mode: `Static + Ran + independent binary64 reframing`

Version 8 now gives the parent validator two non-interchangeable
reconstructions for each exact resource key:

1. accepted-order staged subtraction authenticates every segment ending and
   next-segment beginning and is authoritative for owner custody;
2. an independent accepted-order `+0.0` cumulative fold authenticates the
   diagnostic receipt identity.

The contract explicitly prohibits regrouping the second into
`parent_beginning - cumulative_debit` to validate or manufacture the first.
The three-segment water/NH4/NO3 fixture proves the distinction at the bit
level, and both direct regrouped aliases reject with `VEG-E-124`. Sorting,
reassociation, compensated arithmetic, tolerance closure, and aggregate
mineral-N substitution remain forbidden.

Restart V2 retains ordered segment resource bodies, cumulative rows, and the
staged seven-owner envelopes, so fresh admission can replay the sequential
chain and cumulative fold independently. No field, canonical ordering, schema,
or digest rule changed: schema/poison/reference hashes remain
`af9314c3...2441`, `fa5ae93f...ad34`, and `13f3d009...f7c`. Thus the amendment
changes semantic validation, not the released V2 serialization authority.

### Gate evidence

- segmented-support reference => PASS, 49/49;
- restart V2 reference => PASS, 54/54 and complete suffix digest
  `512c259b...f0e`;
- authority contract => PASS, 7/7;
- strict BEI, unit-compliance, and amendment diff hygiene => PASS;
- independent binary64 probe => PASS: all three sequential terminal owner
  values are bit-distinct from their regrouped aliases and all cumulative
  receipt totals reproduce exactly.

### Verdict

`PASS / SC-VEGETATIONTRANSACTION-001 Version 8 closes the sequential-owner
versus cumulative-receipt ambiguity without changing Restart V2 wire authority
at 3065c209c`.

No Review B finding or waiver remains. Production remains gated on independent
verification and promotion of this exact amendment authority.

---

## Resource-custody amendment Review B — candidate `1302b60b9c4d07f28e58c92a30dce6f39cd70c8e`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + adversarial transaction/restart probes`

The debit-versus-shared-owner separation is scientifically and
transactionally correct in intent. Nominal evidence passes: segmented reference
54/54, Restart V2 54/54 with digest `512c259b...f0e`, authority tests 7/7,
JSON Schema 2020-12 meta-validation, strict BEI, unit compliance, and diff
hygiene. The following transaction findings block release.

### Findings

- `V11-TXN-RESOURCE-B-001`: the closed Restart V2 checkpoint has no
  `shared_resource_transition` collection or IDs. It cannot independently
  restore accepted transition bodies, linked occupancy debits, terminal shared
  owner custody, or the transition predecessor for the unaccepted suffix.
- `V11-TXN-RESOURCE-B-002`: the debit DTO omits inherited owner/OFE/tile/basis
  identity required by `INV-VEGTRANSACTION-009`, while the reference link join
  compares only slab/resource/layer/source. This leaves cross-owner, cross-OFE,
  and cross-basis aliases constructible and leaves water versus BGC admission
  underspecified.
- `V11-TXN-RESOURCE-B-003`: transition admission does not reconstruct its
  candidate digest or transition ID and does not enforce canonical transition
  or debit-link order, global transition-ID uniqueness, admitted slab
  membership, or exact per-key/slab cardinality. Direct reframing proved that
  an arbitrary valid 64-hex candidate digest and reversed link order are
  accepted. An additional same-key transition with a duplicate ID,
  out-of-parent slab, empty links, and altered terminal ending is also accepted.
- `V11-TXN-RESOURCE-B-004`: the staging artifact retains the superseded
  occupancy-final-use subtraction as authoritative owner custody, contradicting
  the new transition-predecessor rule. It must be reconciled before a fresh
  implementer can determine which chronology is binding.

Required correction: close and canonically frame debit and transition bodies;
derive every receipt/transition/candidate identity; freeze exact inherited
water and NH4/NO3 owner keys; enforce deterministic order, uniqueness,
cardinality, link coverage, and support membership; carry the accepted
transition chain through an amended/successor Restart V2 wire and real suffix
comparison; add digest-valid, ordering, duplicate, cross-OFE/basis, and NH4/NO3
poisons. A malformed short digest is not an adequate forged-candidate poison.

### Verdict

`HOLD / SC-VEGETATIONTRANSACTION-001 Version 9 is not transaction- or
restart-complete at 1302b60b9; V11-TXN-RESOURCE-B-001..004 require correction
and independent re-review`.

No waiver is recommended and production remains gated.

---

## Resource-custody final re-review B — candidate `9020f3dcb4cabfde3517f3ee5e23142c8517ce50`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + coordinated transaction/restart forgeries`

The V3 correction closes prior identity-field and staging contradictions and
rejects the original malformed/valid candidate, reversed/duplicate/missing
link, extra/out-of-support transition, and cross-owner/OFE/tile/basis probes.
Required nominal evidence passes: segmented 54/54; protected V2 54/54; V3
12/12; authority 8/8; schema, strict BEI, unit, and diff gates PASS. V2 remains
byte-identical and explicitly nonproduction for resource custody.

### Residual transaction findings

- `V11-TXN-RESOURCE-B-001` remains open. V3 is a standalone resource envelope,
  not a closed successor of the complete V2 restart. It omits coupled time,
  vegetation checkpoint/staged state, beginning owners, sequence/cursor/regime,
  event/scheduled/material receipts, reductions, publication, and outbox. Its
  oracle validates no fresh-object suffix or complete uninterrupted
  continuation. Persist or canonically embed the immutable V2 payload, join all
  duplicated parent/support/owner/receipt facts, and execute the unaccepted
  suffix before admitting V3.
- `V11-TXN-RESOURCE-B-003` remains partially open. Debit `receipt_id` and
  `transition_id` are checked for order/uniqueness but never derived from their
  bodies; coordinated ID-and-link reframes are accepted. Admitted other-flux
  IDs are likewise caller-reframable because no authenticated receipt bodies or
  V2 joins exist. Candidate bytes contain only owner plus scalar ending bits.
  Separate NH4 and NO3 transitions therefore create two alleged complete BGC
  candidates, while terminal joining retains only the last candidate for owner
  `bgc`. Freeze domain-separated ID derivations and bind every scalar component
  to a single complete hydrology/BGC owner candidate whose canonical bytes and
  digest join the terminal complete-owner set.

`V11-TXN-RESOURCE-B-002` and `B-004` are closed. Direct reframing at this
checkpoint still accepts coordinated debit-ID, transition-ID, and other-flux-ID
forgeries, so passing the declared 12 poisons is insufficient for release.

### Verdict

`HOLD / SC-VEGETATIONTRANSACTION-001 Version 10 cannot release at 9020f3dcb;
complete Restart V3 continuation and derived transaction/candidate identities
remain required`.

Production remains gated; no waiver is justified.

---

## Resource-custody final bounded Review B — candidate `bf2c288c4e1010c47042078c362925db747d46b1`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + exact cross-wire support probes`

The candidate now derives debit, transition, and typed-flux IDs; rejects the
prior coordinated reframes; binds NH4/NO3 components into one BGC candidate;
and closes the original ordering, link, candidate, and terminal-owner probes.
Segmented 54/54, V2 54/54, V3 14/14, authority 8/8, schemas, BEI, unit, and diff
gates pass. Direct cross-owner/OFE/tile/basis, extra-transition, duplicate-link,
and missing-link reruns reject.

`V11-TXN-RESOURCE-B-001` nevertheless remains open. The admitted V3 object
uses `[0,1800)` with two 900-tick slabs, but its embedded coupled-time V2 object
uses `[0,1800000000000)` and `accepted_until_ns=600000000000`. Validation
accepts the mismatch because only parent transaction ID is cross-joined. It
does not bind outer support, debit/flux/transition supports, segment/slab IDs,
or V3 candidate ordinals to the embedded V2 cursor and accepted receipt
chronology.

The reported `complete_suffix_sha256` is therefore the unchanged V2 reference
suffix. No V3 custody operand enters suffix execution or uninterrupted
comparison. The schema's fixed 14 candidates and ordinal maximum 1 additionally
make it a two-slab test shape rather than a restart wire for arbitrary accepted
chronology.

Required correction: join exact V2 support/cursor/slab/segment chronology;
derive V3 collections from the accepted prefix; persist them in the staged
continuation consumed by restore; execute the remaining slabs with custody;
and compare all complete owner candidates, transitions, receipts, and terminal
owners against uninterrupted V3 execution. Poison nanosecond/second support
aliases, cursor disagreement, missing/extra accepted prefix, variable slab
counts, and a V3 custody mutation that would alter the suffix.

### Verdict

`HOLD / SC-VEGETATIONTRANSACTION-001 Version 11 remains non-releasable at
bf2c288c4; full V2 composition is present structurally but its time chronology
and actual V3 suffix continuation are not joined`.

The prior ID and complete-owner candidate findings are closed. Production
remains gated without waiver.

---

## Resource-custody cross-wire Review B — candidate `e97f1683b5de8615e5c45b62aae2e346d3ca8d1c`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + coordinated transaction/flux probes`

The exact V2 parent support/cursor/accepted-slab projection, prefix-only wire,
dynamic seven-owner candidate cardinality, terminal V11 state, and terminal
staged owners now join for the control. Required gates pass: V2 54/54, V3
10/10, authority 8/8, schemas, BEI, unit compliance, and diff hygiene. The
declared cross-wire and suffix-consumption poisons reject.

`V11-TXN-RESOURCE-B-001` nevertheless remains open in two related forms:

- a debit and transition can be moved together to an arbitrary parent and
  segment, have both IDs and component links rederived, and still pass because
  validation joins only accepted slab/support and local debit-transition
  fields—not their parent/segment to V2;
- a typed other-flux receipt with owner `snow` can be linked to a hydrology
  transition and accepted. Flux validation does not join parent, segment,
  support, owner, OFE, layer, source, or basis to the consuming transition, nor
  apply an admitted flux-class direction mapping.

The runtime-only suffix comparison is also tautological for prefix authority:
both restored and `full` executions consume the same supplied checkpoint.
Coordinated parent/segment reframing changes the reported suffix digest but
passes both sides. An uninterrupted oracle must instead begin from independent
parent-beginning state/forcing and reconstruct the accepted prefix plus suffix.

Required correction: enforce exact V2 parent/segment identity for every custody
body, full typed flux-to-transition lineage, and independent uninterrupted
construction. Add coordinated parent/segment, cross-owner/OFE/source/basis flux,
and same-forged-origin suffix poisons.

### Verdict

`HOLD / SC-VEGETATIONTRANSACTION-001 Version 12 remains non-releasable at
e97f1683b; accepted chronology is time-aligned but not fully identity-joined or
independently reconstructed`.

Production remains gated with no waiver.

---

## Resource-custody terminal Review B — candidate `e11b6c15e3daf5daaf9d4143e7ca361a4fde1a87`

Date: 2026-08-20

Status: `PASS`

Evidence mode: `Static + Ran + independent transaction/suffix regression`

Version 13 closes the cross-wire transaction boundary. Every V3 custody body
now joins the exact decoded V2 parent, active segment, accepted slab receipt,
support, and cursor projection. Collections are prefix-only, canonically
ordered, exact-cardinality and identity-unique; candidate count derives as
seven owners per accepted slab. Terminal V11 and complete-owner bytes join the
embedded staged checkpoint exactly.

Typed flux receipts have closed body-derived IDs and admit only the frozen
class/direction/source-owner/receiver-owner/resource/spatial/basis mapping.
Each is linked exactly once. Debit and transition IDs are likewise derived;
NH4/NO3 components bind one complete BGC owner candidate rather than scalar
owner aliases.

The uninterrupted comparison is independently rooted in frozen parent
beginnings and frozen operations. It does not read the checkpoint under test.
Restored-only prefix mutation therefore rejects rather than changing both sides
of a self-consistency comparison.

Direct regressions reject coordinated parent/segment reframing (`V3-V2`),
snow-to-hydrology flux mapping (`V3-FLUX-MAPPING`), prefix substitution
(`V3-DEBIT`), and restored-only prefix forgery (`V3-SUFFIX`). V2 54/54, V3
13/13, authority 8/8, schema, strict BEI, unit, and diff gates all pass. The
complete V3 suffix digest is `0b2ff7b0...1096`; V2 schema/reference hashes stay
`af9314c3...2441` and `13f3d009...f7c`.

### Verdict

`V11-TXN-RESOURCE-B-001..004` are CLOSED without waiver. This supersedes all
earlier transaction resource-custody Review B HOLDs.

`PASS / SC-VEGETATIONTRANSACTION-001 Version 13 may proceed to disposition and
independent verification at e11b6c15e`.

Production remains gated on completion of the contract cycle and exact
authority promotion.
