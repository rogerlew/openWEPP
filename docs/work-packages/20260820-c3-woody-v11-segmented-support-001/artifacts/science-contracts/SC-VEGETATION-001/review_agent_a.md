# Contract Review Agent A — Time, Numerics, And Chronology

Status: complete / `HOLD`

Date: 2026-08-20

Reviewed exact commit: `5f4d3902065c316309785cc47ac63f766884bbd1`

Evidence class: `Static + Ran`

Scope: V11 time/numerics, exact full-support compatibility authority, temporal
operator semantics, segment/event/restart chronology, and independent
vector/reference anti-alias coverage. Production Rust was not reviewed or
edited.

## Findings

### A-001 — Critical — Required chronology population is mostly unevaluated

The contract requires start/end events, zero-remainder receiver skip,
mid-parent restart, consecutive segmented parents, distinct segment forcing,
and the full poison set (`SC-VEGETATION-001.md:2619-2623`; `package.md`,
Acceptance Population). The frozen vectors contain only support coverage,
scalar debits, one generic rejected attempt, duplicate scheduled receipt, and
one replay flag (`segmented-support-vectors.json:5-18`). The calculator has no
event, participant-set, segment-state, forcing, receipt/digest, checkpoint,
restore, consecutive-parent, or publication model
(`reference_calculator.py:35-60`). Its 600+1200 and 1200+600 cases therefore
cannot observe execution order. Passing 22/22 is self-consistency over a much
narrower model, not the required authority population.

Required disposition: expand frozen vectors and the independent calculator to
execute every required positive and poison chronology with alias-separating
forcing/state/identity operands before authority verification.

### A-002 — Critical — V11 event-transition authority is not implementable

The amendment classifies an event only as “regime/participant and admitted
custody change” (`SC-VEGETATION-001.md:2553`) and later mentions events in
coverage/finalization/restart. It does not define a V11 event input/capability,
precondition, deterministic same-tick order, beginning/ending V11 and owner
digests, transfer ledger, event ordinal, accepted receipt, replay key, failure
rollback, or zero-remainder skip rule. `execute_v11_segment` accepts only an
accepted slab. Importing generic coupled-time rules does not decide which V11
state/custody transitions are legal. This leaves event-boundary restart and
start/end event acceptance non-derivable.

Required disposition: define the V11 event-transition API and receipt/ledger
join, or explicitly define V11 as an inactive byte-identical participant at
events and bind all mutation to a named adopter authority; then add the
required event and replay vectors.

### A-003 — High — Restart wire omits contract-required continuation state

The contract requires current segment/slab/event/participants and
publication/reduction state (`SC-VEGETATION-001.md:2597-2601`), but the closed
restart schema has no explicit fields for those values and no publication or
reduction field (`v11-restart-schema.json:6-19`). A base64 coupled-time blob is
not sufficient authority for adopter-owned reduction/publication state, and
the schema does not specify canonical blob encodings or prove that hashes bind
decoded content. Consequently a mid-parent restore can lose an accepted peak,
publication buffer, or active V11 regime while still satisfying this schema.

Required disposition: make all adopter-owned continuation facts explicit and
closed, specify canonical encodings/hash relationships, and add fresh-object,
pre/post-event, mid-parent, abort, rejected-attempt-absence, and replay vectors.

### A-004 — High — Temporal classification leaves phenology/GSI chronology ambiguous

The table places phase/timers/GSI in sequential state while prose says named
GSI receipts are scheduled once and phenology edge selection runs per physical
segment (`SC-VEGETATION-001.md:2551-2560`). It does not enumerate which imported
V10 GSI calculation, prior-GSI update, threshold edge, timer increment,
preparation/deployment, and material effects occur per slab versus once per
parent/calendar boundary. A literal import of the V10 per-transaction path can
run a daily edge repeatedly when a parent has multiple slabs.

Required disposition: publish an operation-level phenology/GSI ledger with
receipt and state-update order, then test multi-slab and retry cases that would
fail if any scheduled action executes per segment.

### A-005 — High — Resource/reference arithmetic does not bind exact ordered semantics

The reference uses `math.fsum` over all debits after support validation
(`reference_calculator.py:53`), which is neither an ordered per-segment staged
inventory update nor a specified production binary64 operation sequence. It
does not model NH4/NO3 identity, current staged inventory, receipt lineage, or
ending-owner reconstruction. Distinct wrong implementations can therefore
pass the same expected total.

Required disposition: freeze exact amount representation and arithmetic/order,
advance each typed inventory sequentially, independently reconstruct parent
beginning minus ending, and add non-associative/identity-swapped aliases.

### A-006 — Medium — Migration vectors do not exercise the stated rounding authority

The migration rule requires exact rational conversion, ties-to-even, and exact
bit roundtrip (`SC-VEGETATION-001.md:2518-2522`), but the eight vectors include
no halfway tie with even/odd neighbors, one-bit neighbors around an admitted
cadence, large representable cadence/overflow boundary, or a positive duration
that rounds to zero. The calculator and a production implementation could
share common boundary mistakes without detection.

Required disposition: add independently derived tie, neighbor, zero-rounded,
and checked-range vectors with exact expected bits/ticks.

### A-007 — Medium — Full-support compatibility classification is not yet closed

The prose correctly requires a generated, omission-failing field ledger
(`SC-VEGETATION-001.md:2612-2617`), but the current ledger is a prose category
list with result population deferred. It does not freeze the exact V10/V11
types/paths, recursive field inventory, comparison operation, or exhaustive
identity-difference allowlist. In particular, transaction sequence and
diagnostic/receipt fields mix physical and successor identity concerns.

Required disposition: generate and freeze the exhaustive preimplementation
field classification (including branch/diagnostic handling and exact allowed
identity projections). Actual values may be populated during implementation,
but implementation must not choose what compatibility means.

## Gate evidence

- Ran: strict Binding Exposure Index lint — PASS for both amended contracts.
- Ran: science-contract unit compliance lint — PASS for both contracts.
- Ran: `cargo test --test c3_woody_v11_authority_contract` — 3/3 PASS.
- Ran: independent Python calculator — 22/22 reported PASS, subject to A-001
  and A-005 coverage defects.
- Ran: `git diff --check` — PASS.

## Recommendation

`HOLD`. The high-level successor boundary is sound, but the event/restart API
and authority population are not yet complete enough to constrain an
independent production implementation. No production Rust may begin until the
findings are dispositioned and independently verified.

## Re-review at corrected commit

Reviewed exact commit: `675296fdb262efd052be40d32d6730b3d895220a`

Evidence class: `Static + Ran`

Verdict: `HOLD`.

Disposition of original findings:

- `A-002` closed: `VegetationEventTransitionV1` now binds the required event
  identity, ordering, owner joins, custody entries, receipt, ordinal, rollback,
  no-progress/cycle, and no-rate semantics in canonical authority.
- `A-004` closed: the contract now unambiguously makes phenology edge selection
  sequential per positive slab and GSI preparation/calendar/management receipt
  consumption scheduled once.
- `A-006` closed: migration now has independently executable neighbor, tie,
  zero-rounding, nonfinite, and `u128`-range cases.
- `A-007` closed: the compatibility ledger freezes a recursive fail-closed
  projection, narrow successor-identity allowlist, source-path evidence, and
  unknown/mutated-leaf poisons before implementation.
- `A-001`, `A-003`, and `A-005` are improved but remain open for the exact
  defects below.

### RA-001 — Blocker — The 46-case oracle still does not execute event/restart custody

The expanded population is materially better and forcing order is now
observable. However, events are evaluated only as an in-range unique string ID
plus an `integrates_rate` boolean; the oracle never applies participant/custody
changes, checks beginning/ending owner digests, advances an event ordinal,
orders two same-tick events, closes an event ledger, or exercises failure,
cycle, and no-progress semantics. The two “equivalent restart” cases merely
compare caller-provided strings (`restored_digest == uninterrupted_digest`);
they do not checkpoint, restore, continue, or reconstruct any state/receipt/
reduction/publication chain. Thus the executable evidence still cannot detect
an implementation that ignores the corrected event and restart authority.

Required disposition: make the independent oracle construct and consume the
event/restart structures it claims to validate, and add same-tick precedence,
custody/digest/ordinal, event failure/no-progress/cycle, pre/post-event restore,
scheduled/reduction/publication continuation, and replay poisons.

### RA-002 — Blocker — Closed schemas still admit unauthenticated arbitrary embedded objects

Replacing opaque base64 physical payloads is positive, but configuration/state
now accept arbitrary `type: object, minProperties: 1` V10 JSON and restart
receipts accept arbitrary `payload_canonical_json` objects. JSON Schema cannot
enforce the model-definition prose string that claims canonical
parse-reserialize, digest reconstruction, imported schema identity, ordering,
or cross-object joins. No independent semantic validator or poison population
executes those rules. A forged payload/digest, unknown V10 field, reordered or
duplicate owner/receipt, noncanonical object, or mismatched embedded schema can
remain schema-valid and pass the four current tests.

Required disposition: freeze typed imported-schema references/envelopes and an
executable independent semantic validator; poison digest, embedded schema,
canonical reserialization, duplicate/reordered/unknown owner and receipt, and
cross-wire mismatches.

### RA-003 — High — Resource oracle still does not reconstruct staged endings

The authority now correctly freezes ordinary ordered binary64 left-fold
arithmetic and separate water/NH4/NO3 keys, closing the policy ambiguity in
`A-005`. The oracle, however, only folds anonymous amounts and checks
`total > inventory`; it never advances per-segment staged inventories, checks
authorization versus final use, or verifies the required bit-identical
`parent_beginning - cumulative_debit == final_candidate`. No non-associative
order alias, resource identity swap, stale staged inventory, or ending-owner
poison exists.

Required disposition: execute the complete staged resource recurrence and
independent ending reconstruction with alias-separating amount bits and typed
receipt identities.

### RA-004 — High — Mandatory poison surface remains incomplete

The package explicitly requires scaled V10 output, shortened cloned V10
configuration, material reorder/final-state recomputation, rejected staged
owner mutation, wrong active participant set, and post-restart segment/event
replay. The 46 cases do not execute those semantics; several current poisons
are boolean shortcuts rather than wrong-answer structures. Count and case-name
assertions do not establish semantic rejection.

Required disposition: add executable wrong-answer fixtures for every package
poison and assert reconstructed outputs/owner bytes, not only status/count.

Re-review gates run at the corrected commit:

- strict BEI lint: PASS for both contracts;
- science-contract unit lint: PASS for both contracts;
- `cargo test --test c3_woody_v11_authority_contract`: 4/4 PASS;
- independent calculator: 46/46 reported PASS, subject to RA-001/003/004;
- `git diff --check`: PASS.

## Regression release re-review

Reviewed exact commit: `c53adab0a91c0ecbe853c884bfe05591826441c5`

Verdict: `PASS` — prior Review A authority release PASS is confirmed with no
regression or residual finding. The final delta binds migration to the complete
released V10 Rust configuration/state/model surfaces, explicitly limits the
small imported fixture to canonical-framing KAT use, authenticates parent-
beginning owners and accepted receipt payloads during restart, and expands the
semantic population to 36/36 poisons. Strict BEI/unit lint, the 5/5 authority
test, 46/46 chronology oracle, semantic restore/commit oracle, full-surface hash
binding, and diff hygiene all PASS.

## Authority release re-review

Reviewed exact commit: `ab07b1cf62b3da4299baf4ce045ebecccd85911e`

Evidence class: `Static + Ran + adversarial execution`

Verdict: `PASS`.

`TA-001` is closed. Restore now authenticates the V11 authority/configuration,
parent and successor sequence, cursor and embedded coupled-time cursor,
slab/event ordinals, active participants, complete owner manifest, closed staged
V11 state and enclosing digest, accepted slab/event/resource/material prefixes,
scheduled-once state, reduction state, publication/outbox posture, staged
resource bits, and staged complete-owner digests. It replays the accepted prefix,
then executes the event (when pending), slab-1 vegetation transition, and each
suffix resource transition from the authenticated staged owners/resources before
comparing the reconstructed complete ending to uninterrupted execution.

All prior adversarial probes now reject:

- bad participant set -> `V11-RESTART`;
- missing accepted material -> `V11-RESTART`;
- forged reduction -> `V11-RESTART`;
- outer/embedded cursor mismatch -> `V11-RESTART`;
- forged staged physical state/digest -> `V11-RESTART`;
- forged staged vegetation owner -> `V11-RESTART`;
- prior unknown-body, forged-ending, and forged-live-beginning probes remain
  rejected by their typed guards.

No remaining Review A authority finding was identified. This PASS authorizes
the independent verification checkpoint; it is not production implementation,
full-support V10 runtime compatibility, or final package acceptance evidence.

Release re-review gates:

- strict BEI lint: PASS for both contracts;
- science-contract unit lint: PASS for both contracts;
- authority contract test: 5/5 PASS;
- chronology oracle: 46/46 PASS;
- semantic validator: valid transaction, before/after-event restore equivalence,
  seven-owner consuming commit, and 34/34 poisons PASS;
- all prior and new adversarial restart probes: rejected as required;
- `git diff --check`: PASS.

## Terminal authority re-review

Reviewed exact commit: `205e0ad4e628044093e42eb99388fbbac6942d2c`

Evidence class: `Static + Ran + adversarial execution`

Verdict: `HOLD`.

The exact three prior adversarial probes now fail correctly:

- unknown receipt field -> `V11-SCHEMA-BODY`;
- forged hydrology ending -> `V11-OWNER-ENDING`;
- independently forged live beginning -> `V11-BEGINNING-OWNER`.

Closed receipt bodies, imported canonical fixtures, the owner descriptor
manifest, complete owner-ending reconstruction, independently supplied live
store, actual suffix resource/event/slab replay, and the five new poisons close
`FA-001`, `FA-002`, and the self-seeded-store portion of `FA-003`.

### TA-001 — Blocker — Restart admission ignores authoritative persisted fields

The restart object now has the correct V11 shape, but `restore_and_continue`
does not join several required persisted fields to the accepted prefix or use
them as continuation operands. Adversarial executions independently changed
each of the following and restore still returned the accepted final commit:

- `active_participant_ids` to `["bogus"]`;
- `accepted_material_receipts` to an empty list;
- `reduction_state` to a fabricated peak and operand count;
- `accepted_until_ns` to `42` while embedded coupled time retained the original
  cursor;
- staged V11 physical canopy liquid to `999.0`, with its physical digest
  recomputed but the enclosing state digest unchanged.

The same gap applies to scheduled receipts, pending publication/outbox state,
parent/next sequence joins, and the embedded coupled-time cursor/ordinals unless
separately checked. Suffix vegetation is reconstructed from the original parent
candidate prefix rather than `staged_v11_state`, so a schema-valid but physically
different checkpoint can be silently ignored. This directly violates
`INV-VEGETATION-127` and the contract requirement to reconstruct identity,
predecessor, owner debit, receipt, reduction, and publication chains before
returning a continuation.

Required disposition: semantically validate and cross-join every restart field,
derive suffix execution from authenticated staged V11/owner/controller state,
and add poisons for participant, cursor/coupled-time mismatch, material,
scheduled-once, reduction, publication/outbox, next sequence, staged-state
digest, and staged-state continuation changes.

Terminal gates:

- strict BEI and science-contract unit lint: PASS for both contracts;
- authority contract test: 5/5 PASS;
- chronology oracle: 46/46 PASS;
- semantic validator: valid transaction, prefix restore cases, seven-owner
  commit, and 26/26 listed poisons PASS;
- exact former adversarial probes: all three now reject as required;
- new restart-field adversarial probes: five invalid checkpoints accepted;
- `git diff --check`: PASS.

## Final re-review at typed-validator candidate

Reviewed exact commit: `c7ec8e73096f9816ffbe812ac15deeba1d2b8574`

Evidence class: `Static + Ran + adversarial execution`

Verdict: `HOLD`.

The seven-owner manifest, framed receipts, event record, sequential resource
receipts, prefix checkpoint objects, consuming store, 21 poisons, and mandatory
validator contract are substantial corrections. They close the earlier
case-count/outer-framing gaps, but three authority defects remain executable.

### FA-001 — Blocker — Receipt payloads are canonical but not closed or typed

`validate_receipt` authenticates base64, canonical JSON, digest, kind, and
ordinal, but every receipt body remains an unconstrained JSON object. Each
kind-specific validator reads selected keys and never rejects unknown keys or
enforces an exact field set/schema. Adversarial execution added
`unknown_field` to a resource body, rebuilt its authenticated receipt and parent
receipt, and `validate` accepted it. Therefore the claimed mandatory semantic
validator still admits unknown receipt semantics, contrary to the canonical
contract's explicit unknown-object rejection.

Required disposition: freeze exact closed body schemas/field sets for slab,
event, scheduled, resource, material, and publication receipts and reject
missing/extra/wrong-typed fields before digest/ledger admission. Add one poison
per receipt kind.

### FA-002 — Blocker — Resource/event receipts do not authenticate complete-owner endings

The resource recurrence now checks request, authorization, final use, staged
beginning, and ending bits in exact order. However, its reconstructed water and
N endings are never joined to the hydrology/BGC digests in
`ending_owner_sha256`. Likewise, the event checks snow beginning/ending but
does not bind the receiving surface-liquid ending to `transfer_bits`.
Adversarial execution replaced the hydrology ending digest with 64 `f` bytes,
recomputed the parent receipt, and `validate` accepted it. This leaves the
complete-owner candidate independent of the custody ledgers it claims to
install.

Required disposition: deterministically reconstruct and compare every affected
owner ending (water/hydrology, NH4/NO3/BGC, event donor and receiver,
vegetation/material receiver) and poison each cross-owner join.

### FA-003 — Blocker — Restore/commit does not join an external accepted owner set or continue from the prefix

`restore_and_continue` authenticates the accepted resource prefix, then calls
`validate(c)` on the already complete uninterrupted candidate and constructs
`AtomicStore(c)`, whose live owners and clock are initialized from that same
candidate. It does not rebuild suffix slabs/events/resources from restored
state. Thus it is a prefix check followed by replay of a prebuilt final
candidate, not restore-and-continuation. The same self-seeding makes the commit
join tautological: adversarial execution forged the candidate's hydrology
beginning digest, recomputed the parent receipt, and the store committed it.
The `restart_event_replay` and `rejected_attempt_leakage` poisons also raise
directly in `mutate` rather than being detected by restore/validation.

Required disposition: supply an independently held accepted live clock/owner
set to restore and commit; reconstruct the suffix from checkpoint cursor,
ordinals, participants, receipts, and staged owners; compare the resulting
candidate/commit bytes with uninterrupted execution; route replay/leakage
poisons through that validator.

Final re-review gates:

- strict BEI lint: PASS for both contracts;
- science-contract unit lint: PASS for both contracts;
- authority contract test: 5/5 PASS;
- chronology calculator: 46/46 reported PASS;
- semantic validator: valid case, 21/21 poisons, two checkpoint digests,
  seven-owner commit and publication assertions PASS;
- adversarial semantic probes: unknown receipt payload accepted; forged
  hydrology ending accepted; forged beginning self-seeded and committed;
- `git diff --check`: PASS.
