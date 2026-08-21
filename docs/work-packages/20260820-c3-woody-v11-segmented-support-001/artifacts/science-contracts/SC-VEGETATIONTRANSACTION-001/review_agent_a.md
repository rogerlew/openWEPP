# Contract Review Agent A — Time, Numerics, And Chronology

Status: complete / `HOLD`

Date: 2026-08-20

Reviewed exact commit: `5f4d3902065c316309785cc47ac63f766884bbd1`

Evidence class: `Static + Ran`

Scope: segmented parent chronology, event/restart ordering, exact staged
resource arithmetic, atomic finalization, and the independent acceptance
population. Production Rust was not reviewed or edited.

## Findings

### A-001 — Critical — Transaction vectors do not execute the admitted hierarchy

The amendment requires segment/slab/participant/support/duration and owner-set
digests plus typed receipts (`SC-VEGETATIONTRANSACTION-001.md:271-284`), but the
vectors/reference model only support intervals and scalar debit totals. They do
not model any transaction identity suffix, participant set, owner digest,
receipt, current staged owner, event, restart checkpoint, or atomic publication
state. The positive cases cannot distinguish sequential execution from a
whole-parent aggregate, and most package-required poisons are absent.

Required disposition: implement the complete transaction hierarchy in frozen
independent vectors/reference logic, including wrong slab/participant,
stale-next-beginning, per-segment commit, event start/end, restart before/after,
consecutive parents, publication/rollback, and typed water/N aliases.

### A-002 — Critical — Event receipt and custody transition have no transaction shape

The parent candidate says it contains ordered event receipts
(`SC-VEGETATIONTRANSACTION-001.md:292-297`), but the contract never defines the
receipt fields, beginning/ending owner joins, event ordinal/replay identity,
transfer ledger, active-participant transition, deterministic same-tick order,
or typed event failure. Therefore the claimed ordered event chain and
event-boundary restore cannot be validated or serialized.

Required disposition: add a closed V11 event transaction/receipt binding or a
normative import/profile mapping to the exact coupled-time fields plus V11
custody preconditions and postconditions; add start/end/same-tick/failure/replay
vectors.

### A-003 — High — Restart schema cannot reconstruct the stated parent transaction

The amendment requires parent beginning/current staged complete owner sets,
accepted receipt chronology, and scheduled/material state
(`SC-VEGETATIONTRANSACTION-001.md:299-302`). The candidate restart schema omits
explicit segment/slab/event/participant cursor, reduction state, and publication
buffer, while its generic base64 blobs have no canonical encoding/hash relation.
This does not prove byte-equivalent mid-parent continuation or prevent loss and
replay of accepted transaction facts.

Required disposition: close the wire over every transaction continuation fact,
define canonical encodings, and independently test fresh restore, mid-parent,
pre/post-event, rollback, rejected-attempt absence, and replay.

### A-004 — High — Cumulative debit arithmetic is not specified or independently reconstructed

The contract requires current staged authorization and independent cumulative
reconstruction (`SC-VEGETATIONTRANSACTION-001.md:278-284`). The reference instead
computes `math.fsum(all_debits)` and compares only against one scalar inventory.
No exact operation order/amount representation is bound; NH4/NO3, layers,
owners, and parent-beginning-to-ending identity are absent.

Required disposition: specify canonical amount arithmetic and receipt order,
advance typed owner candidates per segment, reconstruct ending inventories
independently, and add non-associative and identity-swap poisons.

### A-005 — High — Scheduled-once and phenology/material boundary remains ambiguous

The transaction contract accumulates per-segment material receipts, but the
vegetation amendment does not fully separate per-segment phenology edge/state
updates from once-per-calendar GSI/management/initialization operations. A
multi-slab parent can therefore create different receipt/material chronology
under two plausible implementations while satisfying the current prose.

Required disposition: bind an operation-level scheduled/sequential ledger and
test retries plus multiple slabs at one calendar boundary.

### A-006 — Medium — Atomic finalization lacks an exhaustive identity allowlist for V10 compatibility

The compatibility gate requires exact non-identity V10 payload equivalence,
while V11 necessarily changes parent/segment/receipt identities. The current
prose ledger does not freeze an exhaustive field projection distinguishing
physical transaction fields from successor-only identity fields. That leaves
the implementation free to classify a mismatch after observing it.

Required disposition: generate and freeze the exhaustive comparison inventory
and allowed identity projection before production edits; populate its values
during implementation.

## Gate evidence

- Ran: strict Binding Exposure Index lint — PASS for both amended contracts.
- Ran: science-contract unit compliance lint — PASS for both contracts.
- Ran: `cargo test --test c3_woody_v11_authority_contract` — 3/3 PASS.
- Ran: independent Python calculator — 22/22 reported PASS, but it does not
  exercise the admitted transaction hierarchy (A-001/A-004).
- Ran: `git diff --check` — PASS.

## Recommendation

`HOLD`. The one-parent/one-commit design is correct, but its event, restart,
resource-arithmetic, and anti-alias transaction authority is incomplete. No
production Rust may begin before correction and dual verification.

## Re-review at corrected commit

Reviewed exact commit: `675296fdb262efd052be40d32d6730b3d895220a`

Evidence class: `Static + Ran`

Verdict: `HOLD`.

The corrected contract now integrates V11 into purpose, scope, state,
algorithm, guards, obligations, tests, BEI, and change-log surfaces. It also
freezes event capability semantics, ordered typed resource arithmetic,
restart/publication continuation fields, and a parent-candidate field list.
Those changes close original `A-002`, `A-004`, `A-006`, and `A-007` at the
canonical-authority level. The following executable/closed-wire blockers
remain and also cover the overlapping Review B findings.

### RA-TXN-001 — Blocker — Event and restart vectors are receipt-free shortcuts

The oracle does not construct an event receipt, transition participants or
custody, join beginning/ending owners, advance ordinals, or test same-tick
ordering/failure/no-progress. Restart “equivalence” is equality of two input
strings rather than serialization, restore, continuation, and byte comparison.
It therefore cannot prove `INV-VEGTRANSACTION-013` or reject lost/replayed
scheduled, reduction, material, publication, and outbox facts.

Required disposition: independently execute the admitted event/restart
transaction structures and their complete positive/poison population.

### RA-TXN-002 — Blocker — The semantic wire validator remains prose-only

The model definition names a semantic validator, but no independent executable
validator exists. Configuration/state permit arbitrary embedded JSON objects;
restart receipts permit arbitrary payload objects; claimed hashes are not
reconstructed by schema or tests. Owner/receipt type, cardinality, canonical
order, uniqueness, imported schema identity, and cross-wire joins are therefore
not demonstrated fail-closed. This leaves overlapping `V11-AUTH-B-002` and
`V11-TXN-B-004` open.

Required disposition: provide typed closed envelopes plus an executable
semantic validator and canonical/digest/schema/order/cardinality poisons.

### RA-TXN-003 — Blocker — Complete-owner candidate membership remains unspecified

The added parent wire lists digests and ordered receipt IDs, but it does not
freeze the complete owner manifest, owner class/order/cardinality, typed ending
candidate envelopes, or stable material proposal framing. The vectors contain
no missing/duplicate/reordered owner, stale clock, late-owner rejection,
material reorder, or second consuming-finalize structure. A scalar
`atomic_commits: 1` result cannot prove absence of a vegetation-only/partial
commit path. Overlapping `V11-AUTH-B-005` and `V11-TXN-B-003` remain open.

Required disposition: freeze the owner manifest and typed candidate/proposal/
receipt framing and execute all atomic-finalization poisons.

### RA-TXN-004 — High — Resource custody is still aggregated rather than staged

Although the arithmetic policy is now canonical, the oracle folds anonymous
amounts once, never authorizes against each current staged owner, never checks
final use against authorization, and never reconstructs exact final owner bits.
Typed owner/species/layer/basis swaps and non-associative order aliases are
absent. `A-005`, `V11-AUTH-B-003`, and `V11-TXN-B-002` remain open at the
executable-evidence level.

Required disposition: model sequential typed authorization/final-use/ending
owner recurrence and independent parent reconstruction.

### RA-TXN-005 — High — Required transaction poisons are still missing

Scaled V10, shortened V10 cadence, material reorder/final-state recomputation,
rejected staged-owner leakage, wrong active participant structure, complete
restart replay, partial/reordered owners, stale clock, late failure, and
publication/reduction continuation are not executable fixtures. Boolean flags
and result-count checks are insufficient anti-alias evidence.

Re-review gates run at the corrected commit:

- strict BEI lint: PASS for both contracts;
- science-contract unit lint: PASS for both contracts;
- authority contract test: 4/4 PASS;
- independent calculator: 46/46 reported PASS, with the semantic limitations
  above;
- `git diff --check`: PASS.

## Regression release re-review

Reviewed exact commit: `c53adab0a91c0ecbe853c884bfe05591826441c5`

Verdict: `PASS` — prior Review A transaction-authority PASS is confirmed with
no regression or residual finding. The final delta freezes exactly one aggregate
envelope for each of the seven ordered owner classes, authenticates restart
parent beginnings and retained receipt bodies, and adds forged-beginning and
forged-material checkpoint poisons. Strict BEI/unit lint, the 5/5 authority
test, 46/46 chronology oracle, 36/36 semantic poisons, restore equivalence, and
diff hygiene all PASS.

## Authority release re-review

Reviewed exact commit: `ab07b1cf62b3da4299baf4ce045ebecccd85911e`

Evidence class: `Static + Ran + adversarial execution`

Verdict: `PASS`.

`TA-TXN-001` is closed. The restart validator now joins every retained
transaction family required by `INV-VEGTRANSACTION-013`, including the outer and
embedded clock cursor/ordinals, participant regime, owner manifest and staged
owner set, staged V11 state, resource/material prefixes, scheduled state,
reduction, publication/outbox, and successor sequence. Continuation advances
from the reconstructed staged owner/resource set and proves byte-identical
complete-owner commit against uninterrupted execution.

The 34-poison population now includes and rejects the exact former residuals:
bad participants, missing material prefix, forged reduction, cursor mismatch,
forged staged state, bad successor sequence, scheduled replay, and outbox
forgery. Direct reruns of all earlier adversarial probes, including forged
staged vegetation owner, also reject.

No remaining Review A transaction-authority finding was identified. This PASS
is limited to preimplementation authority release; actual V11 Rust custody,
restart, consuming atomicity, and exact full-support V10 compatibility remain
implementation gates.

Release re-review gates:

- strict BEI/unit lint: PASS for both contracts;
- authority contract test: 5/5 PASS;
- chronology oracle: 46/46 PASS;
- semantic validator: 34/34 poisons and both restart positions PASS;
- adversarial restart/custody probes: all reject as required;
- `git diff --check`: PASS.

## Terminal authority re-review

Reviewed exact commit: `205e0ad4e628044093e42eb99388fbbac6942d2c`

Evidence class: `Static + Ran + adversarial execution`

Verdict: `HOLD`.

All exact prior transaction probes now reject: receipt bodies are closed,
hydrology ending is joined to its ledger, and the live store is independently
supplied. The manifest, owner-ending recurrence, consuming commit, rollback,
and five added semantic poisons close `FA-TXN-001`, `FA-TXN-002`, and
`FA-TXN-003`.

### TA-TXN-001 — Blocker — Checkpoint suffix does not consume the complete persisted transaction

The restored suffix now replays slab 1, the event when required, and the final
three resource receipts. It does not authenticate or consume the complete
restart transaction. Restore accepted independently corrupted:

- active participant set;
- accepted material-receipt prefix;
- reduction operands/value;
- V11 cursor inconsistent with embedded coupled-time cursor;
- staged V11 physical state (with only the inner physical digest updated).

This proves that the result is still partly reconstructed from the prebuilt
uninterrupted candidate rather than from all persisted owner/receipt/reduction/
publication state. The material, scheduled-once, reduction, publication/outbox,
sequence, active-participant, staged-state, and cross-wire joins required by
`INV-VEGTRANSACTION-013` remain unenforced. `checkpoint_rejected_leakage` and
`checkpoint_event_replay` are useful but cover only two members of that larger
restart surface.

Required disposition: require exact prefix equivalence and independent
reconstruction for every retained receipt and buffer, validate all duplicated
coupled-time joins, use authenticated staged vegetation/owner state to execute
the suffix, and add one semantic poison for every retained field family.

Terminal gates:

- strict BEI/unit lint: PASS both contracts;
- authority contract test: 5/5 PASS;
- chronology oracle: 46/46 PASS;
- semantic validator: 26/26 listed poisons PASS;
- former unknown-body/forged-ending/forged-live-beginning probes: reject;
- five checkpoint-field corruption probes: accepted;
- `git diff --check`: PASS.

## Final re-review at typed-validator candidate

Reviewed exact commit: `c7ec8e73096f9816ffbe812ac15deeba1d2b8574`

Evidence class: `Static + Ran + adversarial execution`

Verdict: `HOLD`.

The integrated Version 4 authority now defines the seven owner classes, owner
order/cardinality, typed public symbols, material proposal framing, mandatory
semantic admission, closed outer parent candidate, and consuming commit. The
remaining blockers are exact transaction contradictions, not missing document
structure.

### FA-TXN-001 — Blocker — Authenticated receipt envelopes still contain open bodies

The executable validator authenticates receipt envelopes but permits arbitrary
extra fields in every decoded payload. A resource receipt with a newly inserted
unknown field was re-framed and accepted. Consequently producer/consumer
agreement is not closed for resource, material, event, scheduled, slab, or
publication receipts, and the contract's unknown-object rejection is not
implemented by its cited independent authority.

Required disposition: bind and enforce exact per-kind payload schemas/field
sets and poison unknown/missing/wrong-type fields for every receipt class.

### FA-TXN-002 — Blocker — Complete-owner replacement is not joined to resource/event ledgers

Resource receipt endings are sequentially checked, but the resulting final
water/N values are not authenticated into the hydrology/BGC owner ending
digests. Event transfer validates the snow ending only, not the receiving
surface-liquid ending. The validator accepted a forged hydrology ending digest
after parent-receipt reconstruction. Material receipts similarly do not join
vegetation and receiving BGC/residue owner endings.

Required disposition: require exact donor/receiver candidate reconstruction for
every accepted transfer before the parent receipt or atomic install is valid.

### FA-TXN-003 — Blocker — Atomic live preconditions are candidate-selected

`AtomicStore(c)` initializes its live owner set and clock from `c` itself.
Commit therefore cannot authenticate the candidate against an independently
accepted live beginning. A forged beginning owner digest was accepted and
committed. The stale-clock poison mutates this self-created store only after
construction and does not close the fundamental caller-selected beginning.

Required disposition: construct the store from separately supplied accepted
clock/owner authority and require exact joins to the candidate. Add forged
beginning-owner and beginning-clock poisons.

### FA-TXN-004 — Blocker — Checkpoint continuation validates a prebuilt final candidate

The checkpoint preserves a receipt prefix, but restore validates that prefix
and then commits the original complete candidate through a fresh self-seeded
store. It never executes the suffix from restored cursor/event ordinal/staged
owners, so `restore_equivalent=true` does not prove equivalent continuation.
Replay/leakage poisons raise directly in the mutation helper rather than
testing restart admission.

Required disposition: resume suffix execution from both before-event and
after-event checkpoint objects, independently reconstruct final receipts/
owners/publication, and compare with uninterrupted bytes; feed replay and
rejected-state mutations through restore.

Final re-review gates:

- strict BEI lint and unit lint: PASS for both contracts;
- authority contract test: 5/5 PASS;
- chronology oracle: 46/46 PASS;
- semantic oracle: valid transaction and 21/21 listed poisons PASS;
- adversarial probes: open payload, forged ending, and forged live beginning
  were all accepted;
- `git diff --check`: PASS.
## Restart V2 transaction amendment review at `ac8cb0eda4110d5b5fe8811d82da314b6d8bf25e`

Evidence class: `Static + Ran`

Verdict: `HOLD`.

Scope: read-only Review A of restart suffix chronology, coupled-time joins,
accepted reduction/publication retention, and the 15 V2 poisons.

### RVA-TXN-001 — Blocker — V2 does not admit the complete typed transaction checkpoint

The reference admits arbitrary canonical checkpoint JSON and arbitrary
digest-valid owner bytes, then inspects only parent, cursor, slab count and
`finalized`. It does not parse the complete `V11ParentTransactionCheckpoint`,
typed staged V11 state, resource/material receipt prefixes, or owner-specific
payloads. This fails the Version 5 requirement that a typed complete checkpoint
and seven typed owners exist before a continuation capability is returned.

Required disposition: close and execute the checkpoint/owner type admission,
including deny-unknown-fields, schema/model/configuration identity, canonical
order/cardinality, receipt bodies and staged-owner joins.

### RVA-TXN-002 — Blocker — No suffix transaction is resumed

The reference validates a few duplicated cursor fields and returns three
summary values. It does not reconstruct the accepted prefix, restore staged
owners, apply pending event/scheduled work, or execute any remaining slab.
Active segment/regime, controller policy, sequences, event/material/resource
receipts and staged state are not cross-joined. Thus restore equivalence and
replay resistance are unevaluated despite the amendment's claim.

Required disposition: execute before-event, after-event and mid-parent suffixes
from independently persisted inputs and require bit-identical uninterrupted
parent candidate/commit bytes; poison every duplicated cross-wire family.

### RVA-TXN-003 — High — Accepted reduction chronology is underconstrained

An operand passes whenever its end is not in the future. Source receipt,
support ordering, uniqueness, finite value, operator and retained result are
unvalidated. This permits loss/reordering/duplication or replacement of an
accepted peak/total and violates accepted-only diagnostic continuation.

Required disposition: replay the exact typed reduction over authenticated
accepted receipts and add wrong-source, overlap/reorder, duplicate, NaN,
changed-value and lost-pre/post-restart-maximum poisons.

### RVA-TXN-004 — High — Publication/outbox restore is not transactionally authenticated

Pending publication payload digests, record IDs, source reductions, ordering
and uniqueness are not checked. Outbox validation is only set membership of
record ID and does not validate durable state or delivery count chronology.
The one orphan poison does not prove precommit invisibility, rollback clearing,
or exactly-once postcommit continuation.

Required disposition: reconstruct publication and durable outbox transitions
from accepted reductions and parent commit state, with forged payload,
duplicate/order, wrong reduction, premature visibility, rollback retention and
delivery-state poisons.

### Gate evidence

- Ran exact-commit reference: base accepted and all 15 listed poisons rejected
  under their expected labels.
- Ran exact commit-range `git diff --check`: PASS.
- Static: the integration test checks only count/owner summary, so the four
  transaction gaps remain release-blocking.
## Restart V2 corrected transaction re-review at `a38e2cfa12705a6692ced186b5dc4e51d97ab3f3`

Evidence class: `Static + Ran + adversarial inspection`

Superseding verdict: `HOLD`.

The corrected oracle rejects 24/24 listed poisons and now performs a bounded
scalar suffix. It also joins more coupled-time fields and checks event,
scheduled, reduction and publication examples. The prior transaction findings
are improved, not closed:

- `RVA-TXN-001` remains Blocker: checkpoint/owner/receipt bodies have no exact
  deny-unknown-fields/type admission and use generic fixture owner bodies.
- `RVA-TXN-002` remains Blocker: suffix arithmetic uses only staged scalar
  state; it does not consume staged owners, resource/material recurrence,
  event transfer, scheduled state, reduction/publication/outbox, active regime,
  or controller state. Resource receipts are framed but semantically ignored.
- `RVA-TXN-003` remains High: duplicate/reordered/overlapping/nonfinite
  reduction operands, invalid support and changed-prefix aliases remain
  admissible; operator/result receipt identity is absent.
- `RVA-TXN-004` remains High: record/outbox IDs, uniqueness/order, commit
  visibility, rollback clearing and durable delivery transitions are not
  reconstructed.

Ran: exact-commit base continuation PASS and all 24 expected poison labels
PASS. This evidence does not yet authorize the claimed complete V2 transaction
checkpoint or equivalent continuation.
## Restart V2 final transaction re-review at `887d92ec557f22682cc5e4df048a20aa249d2cbf`

Evidence class: `Static + Ran + adversarial inspection`

Superseding verdict: `HOLD`.

The correction closes principal resource/event/material/reduction/publication
joins and rejects all 34 listed poisons. Two transaction residuals remain:

- `RVF-TXN-001` — Blocker: owner `state` bodies remain generic/open; LSE and
  soil-thermal staged state are not consumed, and suffix equivalence compares
  only one scalar vegetation result rather than the complete seven-owner
  candidate, receipts, reductions, publication and outbox bytes.
- `RVF-TXN-002` — High: event body fields/source/receiver/from-participants and
  collection uniqueness/order remain underconstrained; duplicate reduction,
  publication or outbox identities and unreconstructed outbox IDs/transitions
  are not poisoned.

Ran: exact-commit base suffix PASS, 34/34 poison PASS, diff hygiene PASS. Full
typed-owner suffix equality and closed collection chronology are still required
before the V2 amendment merits Review A PASS.

## Restart V2 terminal amendment re-review at `937aadb329ced16f050c676f89769fec2d8f5efe`

Evidence class: `Static + Ran + independent adversarial probes`

Superseding verdict: `PASS`.

The executable V2 reference passes its canonical continuation and rejects
52/52 declared poisons with typed errors. JSON Schema 2020-12, strict BEI (4
rows), unit-compliance, the semantic authority model, and the Rust authority
suite (6/6) pass. Eighteen retained independent probes also reject, including
joint-sequence forgery; nested state, owner and event openness; event custody;
forged LSE/thermal endings; duplicate/reordered receipt/reduction/publication
collections; scheduled/event/resource/material duplication; and forged or
duplicate outbox identities.

The former complete-suffix finding is closed: restoration consumes all seven
closed staged owner bodies and canonical equality covers the ending complete
owner set, prefix resource/material custody, event and scheduled receipts,
reduction operands, publication records, outbox, and the joint next-parent
sequence against uninterrupted continuation. The former event/collection
finding is closed by exact bodies, derived IDs, canonical ordering,
uniqueness/cardinality, event owner-transfer joins, and valid outbox state/count
rules.

No residual Review A finding remains on the transaction/restart amendment.
This verdict supersedes the HOLD at
`887d92ec557f22682cc5e4df048a20aa249d2cbf`.

## Restart V2 v7 regression review at `5918d4dbdfd0a7641d16b1f5f2040289c9893788`

Evidence class: `Static + Ran + independent custody probes`

Verdict: `PASS`.

Version 7 adds the missing exact segment-predecessor and terminal-owner joins:
ordinal zero is rooted in the parent beginning state, subsequent state/support
chronology is contiguous, and terminal ending-owner envelopes equal both
checkpoint and outer staged seven-owner envelopes exactly. This is additive
strengthening and does not weaken any previously closed transaction surface.

Rerun evidence: complete continuation PASS; 54/54 declared poisons and 20
independent custody probes reject; schema, semantic authority, strict BEI (4
rows), unit compliance, Rust authority 6/6, and diff hygiene all PASS. Event,
resource/material, sequence, reduction, publication and durable-outbox custody
remain closed. No Review A regression or new finding remains.

## Sequential-debit amendment Review A — `3065c209c7d5d203a2a06fca793dc8cbc340e26e`

Date: 2026-08-20

Status: `PASS`

Evidence mode: `Static + Ran + independent binary64 probes`

Version 8 cleanly separates two accepted-order folds: authoritative staged
owner subtraction and the exact-`+0.0` cumulative receipt diagnostic. It
explicitly prohibits using the regrouped parent-minus-total value as an owner
ending or acceptance gate, while retaining exact predecessor joins and
overbooking rejection.

Reference evidence passes 49/49 and authority tests pass 7/7. Independent
probes cover nonassociative and reordered operands, wrong sequential endings
for water/NH4/NO3, nonfinite and negative debits, overbooking, and signed-zero
behavior. BEI (4 rows), unit compliance, and diff hygiene pass. No transaction
finding remains; the amendment may proceed to independent verification.

## Resource-custody amendment Review A — `1302b60b9c4d07f28e58c92a30dce6f39cd70c8e`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent actual-shaped custody probes`

Version 9 correctly separates occupancy request/authorization/use receipts
from owner/OFE/layer/source transitions and assigns cross-segment predecessor
authority only to the shared transition. Receipt identity, exact-once debit
links, and aggregate authorization against current shared inventory close the
original two-occupancy alias.

One transaction-authentication blocker remains. The independent oracle checks
only that `owner_candidate_sha256` is shaped like a digest; it does not derive
or join it to an admitted canonical owner candidate, the declared shared
ending, or the next staged owner envelope. Nor does it admit and reconstruct
`other_flux_receipt_ids`. Reframed cases with an unrelated valid digest,
arbitrary but self-chained shared endings, and an unknown other-flux receipt all
accepted. Thus a caller can manufacture a mutually consistent transition
sequence that is unrelated to the actual hydrology/BGC candidate.

The newly added schema is closed JSON, but the V4 reference fixtures use
shorthand decimal fields rather than the schema's canonical identity and bit
fields; no instance gate connects them. Closure requires an actual-shaped
fixture and validator that recompute or exact-join the canonical owner
candidate digest and close the chosen other-flux/candidate lineage branch,
with valid-digest substitution, arbitrary-ending, missing/unknown/duplicate
other-flux, and schema-instance poisons.

Gate evidence: reference PASS 54/54; authority PASS 7/7; strict BEI PASS (4
rows); scoped unit compliance, JSON syntax, and diff hygiene PASS. The direct
forgery probes supersede those self-consistent results.

Verdict: `HOLD / Version 9 has the right custody split but does not yet bind
the shared transition to the actual owner candidate at 1302b60b9`.

## Resource-custody final Review A — `9020f3dcb4cabfde3517f3ee5e23142c8517ce50`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent actual-shaped custody probes`

Version 10 / Restart V3 now rejects every prior Review A forgery: unrelated
candidate digests, arbitrary self-chains, unknown flux IDs, broken/reordered
debit links, identity aliases, and terminal owner substitution. Candidate
bytes/digests and terminal joins are executable rather than format-only.

The transaction oracle still omits aggregate authorization against shared
inventory. It validates each occupancy independently but accepts two 100-unit
water authorizations against a 10-unit shared beginning. Exact debit-link
coverage does not prevent that cross-occupancy overbooking.

It also constructs distinct candidate bytes for NH4 and NO3 transitions of the
same BGC owner in the same slab. Both are described as canonical complete owner
candidates, yet only the last transition encountered is joined to the terminal
BGC envelope. The earlier candidate is neither equal to the same owner/slab
candidate nor terminally joined. This is a contradictory complete-owner
chronology, not a harmless per-resource diagnostic.

Closure requires one canonical complete candidate per owner/slab linked from
all its resource transitions, exact same-owner candidate equality/cardinality,
and summed authorization/final-use validation against the current shared
inventory, with direct poisons for both defects.

Gate evidence: segmented 54/54, Restart V2 54/54, Restart V3 12/12, authority
8/8, strict BEI 4 rows, scoped units, schema syntax, and diff hygiene PASS. V2
is byte-pinned and no production path appears in the amendment range.

Verdict: `HOLD / Version 10 closes prior authentication findings but not
cross-occupancy overbooking or same-owner/slab complete-candidate uniqueness at
9020f3dcb`.

## Resource-custody terminal Review A — `bf2c288c4e1010c47042078c362925db747d46b1`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent composition probes`

Version 11 closes both previous transaction findings: aggregate linked
authorization/final use cannot exceed shared inventory plus typed inflow, and
NH4/NO3 transitions share one canonical complete BGC owner candidate for their
owner/slab. All declared V3 and retained direct custody poisons reject.

The V3 extension is nevertheless not transactionally composed with its
required V2 checkpoint. The validator admits V2 and computes its suffix digest,
then separately admits V3 transitions/candidates. It joins only the parent ID.
The fixture's V3 terminal set is byte- and digest-different from the V2 staged
set for every one of the seven owners, and no check connects V3 resource
transitions to V2 accepted segments or connects the restored V2 suffix to V3
terminal custody. Thus an unrelated, internally valid resource transaction can
be grafted onto an internally valid checkpoint.

Closure requires explicit checkpoint-position semantics and exact V2 staged /
V3 beginning owner joins, followed by complete uninterrupted-versus-restored
V3 suffix equality over seven owners and resource receipts. Add independent
valid-extension substitution and staged/terminal owner mismatch poisons.

Gate evidence: segmented 54/54, V2 54/54, V3 14/14, authority 8/8, BEI 4 rows,
scoped unit/schema/diff PASS. These self-contained gates do not exercise the
missing cross-wire composition.

Verdict: `HOLD / Version 11 closes local debit/candidate custody but does not
compose that custody with the V2 checkpoint and suffix at bf2c288c4`.

## Resource-custody cross-wire Review A — `e97f1683b5de8615e5c45b62aae2e346d3ca8d1c`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent suffix probe`

Version 12 closes the prior checkpoint-prefix transaction finding. V3 position
is derived from V2 support/cursor/slab/event chronology, terminal candidates
equal the seven V2 staged owners, and terminal V11 state equals the decoded V2
staged state. All requested graft and prefix poisons reject.

One transaction verification gap remains. The alleged uninterrupted result and
restored result are both `execute_suffix(c, True)` over the same restored
checkpoint. There is no independently constructed uninterrupted parent
chronology, so equality is by construction and cannot detect lost, duplicated,
or misapplied prefix custody shared by both paths.

Closure requires two independently constructed paths: parent beginning through
accepted prefix and suffix, versus admitted checkpoint terminal through suffix,
with complete owner/resource/receipt/event/reduction/publication equality. A
poison must alter accepted-prefix custody in only the restored path while the
independent uninterrupted path remains fixed.

Gates PASS: V2 54/54, V3 10/10, authority 8/8, BEI 4 rows, scoped unit/schema/
diff hygiene. The suffix-not-consumed poison is not an anti-tautology oracle.

Verdict: `HOLD / Version 12 binds the V2/V3 prefix but still self-compares one
suffix execution instead of proving uninterrupted equivalence at e97f1683b`.

## Resource-custody terminal Review A — `e11b6c15e3daf5daaf9d4143e7ca361a4fde1a87`

Date: 2026-08-20

Status: `PASS`

Evidence mode: `Static + Ran + independent uninterrupted/restored probes`

Version 13 supplies the missing independent transaction chronology. The
uninterrupted path derives accepted prefix custody from frozen beginning
owners/state and frozen operations without reading V3. The restored path uses
the admitted V2/V3 terminal prefix. Applying identical future input produces
byte-identical seven-owner, V11 state, resource/material receipt, slab/event,
reduction, and publication results. A validly reframed restored-only debit
prefix is rejected by the independent comparison.

All prior identity, link, candidate, support/cursor, terminal, overbooking,
multi-component BGC, and cross-wire probes remain closed. Typed other-flux
receipts have derived closed bodies, admitted source/receiver mappings, and
exact-once transition linkage; coordinated parent/segment and flux reframes
reject.

Gates PASS: V2 54/54, V3 13/13, authority 8/8, strict BEI 4 rows, scoped unit,
schema, and diff hygiene. No residual Review A finding or waiver remains.

Verdict: `PASS / SC-VEGETATIONTRANSACTION-001 Version 13 closes complete
resource-custody restart equivalence at e11b6c15e and may proceed to independent
verification`.
## Review A addendum — Version 14 LSE support-admission join

Disposition: **HOLD**, inherited from `SC-LANDSURFACEENERGY-001` findings
`LSE-A-001` through `LSE-A-006`.

The transaction cannot prove receipt-before-owner-operation ordering or prevent
same-duration cross-slab reuse while the receipt omits parent, segment, slab
and absolute support. Add those joins, typed `LSEB-E-042` poisons, exact
rollback/restart bytes, and explicit no-retry-at-minimum evidence. Replace
“reviewed LSE” with prospective/in-review until the authority cycle passes.

Rerun: **HOLD remains**. Absolute-support fields now exist, but no executable
receipt validator proves the joins, byte-exact rollback/restore vectors remain
declarative, slab ordinal is absent from the receipt schema, and “reviewed LSE”
remains premature.

Third rerun: **HOLD remains**. Slab ordinal and canonical fields are now in the
schema, but validation has no independently supplied expected slab context, so
a coordinated rehashed receipt from another slab remains admissible. Rollback/
restore are still declarative, and “reviewed LSE” remains premature.

Terminal addendum: the final LSE authority rerun closes these dependency
findings. The transaction receipt binds the independently supplied slab domain,
below-domain rejection is atomic, and restored continuation covers complete
owner and buffered chronology state. **Addendum verdict: PASS**, subject to
normal lifecycle disposition and dual verification.
