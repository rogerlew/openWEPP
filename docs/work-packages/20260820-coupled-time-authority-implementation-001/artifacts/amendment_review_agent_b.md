# SC-COUPLEDTIME-001 restart amendment review B

Reviewer scope: transaction/ownership, restart wire, canonical serialization,
and uninterrupted/restored parent-finalization equivalence. This review did not
edit production Rust.

Reviewed working-tree amendment on 2026-08-20 against the released authority
checkpoint `30e82ab16` and implementation candidate `42f88d644`.

## Verdict

**FAIL / HOLD authority release.** Persisting accepted-slab chronology is the
correct correction, but the proposed wire cannot yet authenticate that
chronology or prove equivalent parent finalization after restore.

## Blocking findings

### B1 — The amendment incompatibly changes a released V1 wire under the same identity

`restart-schema.json` keeps `$id = OPENWEPP_COUPLED_TIME_RESTART_V1` and
`version = 1`, but adds `accepted_slab_receipts` to the closed required shape.
The contract is already `approved`/`active`, and section 8 states that a change
to an existing wire requires a separate authority amendment. Running a review
cycle does not make old V1 bytes valid under the new closed shape: previously
valid V1 checkpoints are now rejected.

Required disposition: either introduce a V2 restart identity with an explicit
V1-to-V2 admission/migration rule, or establish with repository release
governance that V1 was never externally released and explicitly revoke and
replace its checkpoint/identity before implementation resumes. Do not silently
reuse V1 for two incompatible closed schemas.

### B2 — `receipt_id` and accepted-slab lineage are only syntax-checked, not authenticated

The semantic validator checks that slab digest fields have 64 lowercase hex
characters, support is contiguous, and ordinals are sequential. It does not
recompute `receipt_id`, `accepted_slab_id`, `segment_id`, `constraint_digest`,
clock digests, owner-set digests, owner-candidate-set digest, or coupled-ledger
digest from canonical operands. Consequently any well-formed digest can replace
any of those values and restore is still admitted. This contradicts the added
contract text requiring rejection of digest/lineage mismatch and permits a
forged chronology to drive parent finalization.

The frozen model definition also defines `slab-receipt` as only
`accepted_slab_id, begin_clock, end_clock, owner_set, ledger_digest`; that
framing neither matches the restart `slabReceipt` field names nor binds support,
ordinal, segment, duration, constraint, owner-candidate-set, or parent identity.

Required disposition: freeze one exact canonical slab-receipt framing shared by
the model definition, runtime receipt, and restart schema. Recompute every
derived identity on admission and chain consecutive clock/owner digests to the
persisted parent/owner cursor. Add alias-separating poisons that substitute a
different *valid* SHA-256 value for every lineage field and receipt identity.

### B3 — The wire does not establish complete receipt chronology across event boundaries

Slab support is checked as one positive-duration chain, but each receipt's
`segment_id` is not resolved against an authenticated segment/event history.
Beginning/end clock and owner-set digests are not chained between slabs or
joined to accepted event receipts at a shared tick. Thus a slab may claim an
arbitrary segment, skip the owner transition performed by an accepted event,
or begin from an owner digest unrelated to the restored complete owner bytes.
Receipt arrays are also described globally as receipt-ID ordered while slab
receipts are required and validated in ordinal order; these are incompatible
canonical-order rules.

Required disposition: specify and validate the interleaved chronology rule
(`slab -> zero-duration event(s) -> slab`), including segment resolution,
clock continuity, owner digest continuity through events, and an unambiguous
canonical array order. If separate arrays remain, define the deterministic
merge key and prove it reconstructs the same ordered child-receipt list used by
parent finalization.

### B4 — Restored-finalization equivalence evidence is tautological

`reference_model.py::restart_equivalence` compares six caller-supplied values
for equality and hashes the restarted values. It does not admit restart bytes,
resume chronology, independently reconstruct the ordered accepted child
receipts, or construct and compare parent candidate/receipt/publication IDs.
The current vectors therefore demonstrate equality of duplicated fixtures, not
equivalent finalization.

Required disposition: add an independent continuation fixture that starts from
the canonical restart envelope, validates/reconstructs accepted slab and event
receipt chronology, applies the same post-restart operations as the uninterrupted
twin, and independently derives the terminal owner bytes, ordered child receipt
list, parent receipt ID, reduction state, publication record order/digest, and
outbox row. Compare exact bytes/identities. Add omission, reorder, duplicate,
valid-digest substitution, event-boundary splice, and pre/post-restart subset
poisons.

### B5 — Cursor metadata is insufficiently joined to the new receipt list

The validator checks `len(slabs) == next_slab_ordinal` and final slab end equals
the cursor, but does not require `last_accepted_step_ns` to equal the last slab
support duration (or zero for an empty chronology), does not derive/check
`duration_bits` from exact integer support, and does not join the active segment
ordinal/identity to `next_segment_ordinal`. These allow restored controller and
segment state to disagree with the authenticated accepted boundary.

Required disposition: add these joins to normative text, semantic admission,
and wrong-answer fixtures.

## Read-only evidence

- `python3 artifacts/semantic_schema_validator.py --poisons artifacts/semantic-schema-poisons.json` completed with all declared cases matching expectation.
- Inspection confirmed the new poisons cover missing receipts, ordinal/support/cursor errors, and a malformed digest, but no well-formed digest substitution, receipt derivation, event splice, clock/owner chain, or exact parent-finalization reconstruction.
- Existing DirectV10 restart artifacts were not modified by this amendment.

The amendment can proceed once B1–B5 are dispositioned and the corrected wire,
validator, independent continuation oracle, and poison population pass dual
verification before production Rust resumes.

---

## Re-review of corrected V2 amendment

Re-reviewed on 2026-08-20. The correction resolves B1: the original
`restart-schema.json` is byte-identical to checkpoint `30e82ab16` (SHA-256
`71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d`),
and the additive closed wire is now `OPENWEPP_COUPLED_TIME_RESTART_V2` in
`restart-schema-v2.json`. It also materially improves B2 by freezing V2 framed
slab/event receipt domains, recomputing IDs and duration bits, and adding
well-formed wrong-value poisons. `restart_finalization_reference.py` independently
reconstructs its frozen terminal parent and publication receipt KAT.

### Re-review verdict

**FAIL / HOLD authority release.** Two original blockers remain, and the full
oracle population is currently red.

### RB1 — Mid-parent event chronology still cannot be admitted or reconstructed

The V2 validator first chains *all slabs* directly through owner/clock digests,
then starts *all events* from the final slab. This admits only events after the
last accepted slab. It cannot represent the authority's defining chronology:

```text
slab [a,b) -> event at b -> slab [b,c)
```

For such a chronology the second slab must begin from the event's ending
owner/clock digest, but the validator requires it to begin from the first slab's
ending digests before processing the event. Receipt arrays remain separately
ordered and no deterministic merge/replay algorithm resolves events between
slab ordinals. `restart_finalization_reference.py` likewise concatenates all
slab receipt IDs followed by all event receipt IDs and exercises only one slab
with a terminal event. This does not prove restored finalization for terminal
snow meltout followed by snow-free remaining support.

Required correction: define and validate one merged accepted chronology key or
persist an explicit ordered child-receipt sequence. Replay slab/event boundaries
in that order, enforcing tick, segment, owner, and clock continuity at every
edge. Add an independent two-slab/one-interior-event finalization KAT plus
event omission, event moved after slab 2, wrong successor segment, and owner/
clock splice poisons.

### RB2 — Cursor/segment/controller joins remain absent

`last_accepted_step_ns` is still only parsed. It is not required to equal the
last accepted slab's support length (or zero for an empty chronology).
`active_segment.segment_id`/ordinal and `next_segment_ordinal` are not joined to
the terminal accepted chronology. The new slab `segment_id` is authenticated as
an input to its receipt, but it is not resolved against a segment created by the
event chronology. Therefore well-formed but semantically inconsistent restored
controller and active-regime metadata can still pass admission.

Required correction: enforce the last-step, active-segment, next-segment, and
terminal segment/event joins normatively and in the semantic validator, with
well-formed wrong-value fixtures.

### RB3 — Required all-vector gate fails after the domain amendment

The semantic poison runner passes all 42 declared cases, and the independent
terminal finalization KAT passes. However the canonical oracle command fails at
`slab_receipt_identity_kat`: `coupled-time-vectors.json` still requests the old
`slab-receipt` identity domain, while `reference_model.py` now rejects that
domain in favor of V2. The authority amendment cannot be released with its
mandatory vector population internally inconsistent.

Required correction: preserve explicitly labeled V1 legacy KAT handling or
migrate the affected receipt/event KATs and expected hashes to the V2 domains,
then run the entire oracle population to PASS.

### Re-review evidence

- V1 schema exact hash comparison against `30e82ab16`: PASS.
- V2 semantic poison runner: PASS for all 42 declared cases.
- `restart_finalization_reference.py`: PASS for its one-slab terminal-event KAT.
- Full `reference_model.py --vectors coupled-time-vectors.json`: **FAIL** at
  `slab_receipt_identity_kat` (`InvalidIdentityDomain`).

B1 and the receipt-framing portion of B2 are closed. B3, B5, and the required
full-vector closure remain open as RB1–RB3 above.

---

## Final re-review

Final re-review completed on 2026-08-20.

### Verdict

**PASS / GO for the amended authority release checkpoint.** All findings from
the initial review and first re-review are closed for the contract boundary.
This verdict authorizes resumption of production implementation only after the
required second independent verification and exact amended-authority checkpoint.

### Finding closure

- **B1 closed:** `OPENWEPP_COUPLED_TIME_RESTART_V1` remains byte-identical to
  checkpoint `30e82ab16`; the additive authenticated chronology wire is V2.
- **B2 closed:** V2 slab/event/parent identities use frozen framed domains and
  exact operands. Admission reconstructs parent interval and transaction IDs,
  exact integer-support duration bits, accepted slab/event IDs, receipt IDs,
  and terminal owner lineage. Well-formed substitutions fail.
- **B3 / RB1 closed:** admission deterministically merges slabs by ending tick
  and events by event tick, with a slab-before-event tie rule and event ordinal
  ordering. It replays the merged owner/clock chain from explicit parent-begin
  anchors through the interior B-to-C transition to the terminal accepted owner
  set. The positive fixture now exercises A+B slab, interior B-to-C event, then
  A+C slab.
- **B4 closed:** `restart_finalization_reference.py` independently derives the
  terminal parent and publication receipt KAT from the restored two-slab,
  interior-event chronology; it does not import or invoke production Rust.
- **B5 / RB2 closed:** admission reconstructs the active segment identity and
  validates the parent/cursor boundary, last accepted step, next slab/event/
  segment ordinals, initial owner/clock anchors, and terminal owner chain.
- **RB3 closed:** the legacy `slab-receipt` identity domain remains available
  for its frozen historical KAT, while V2 uses separately named receipt domains.
  The complete oracle population passes.

### Final read-only evidence

- V1 restart schema exact SHA-256 against `30e82ab16`: PASS,
  `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d`.
- V2 semantic validator and poison population: PASS, 47/47.
- Independent restored-finalization KAT: PASS; parent receipt
  `90627286f5cc4b6e341f0162323606013f0c0d8f58b2dd17615459befd6cfda3`,
  publication receipt
  `5faa32af248f6d4badbb0d6b65cf075d18b25f3eaedd23a2d49e53f6ff574602`.
- Complete independent reference oracle: PASS, 108/108.

No production Rust was edited during this review.
