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
