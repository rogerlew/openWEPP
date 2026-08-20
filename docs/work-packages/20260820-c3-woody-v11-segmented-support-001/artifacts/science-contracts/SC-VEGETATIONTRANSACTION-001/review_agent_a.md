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
