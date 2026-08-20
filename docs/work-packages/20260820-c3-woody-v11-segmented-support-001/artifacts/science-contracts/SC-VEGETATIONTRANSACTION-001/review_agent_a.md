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
