# Final Release Hydrology And Ownership Review

Evidence class: `Static + targeted Ran`, fresh independent exact-worktree
review.

Verdict: **NO-GO / FAIL**.

The release remediation corrected the positive condensation DTO, attempted-
transaction rollback lineage, routed-runon conservation/lineage evidence, and
the shared-source D/A/F transaction. One critical owner-envelope defect remains:
the strict coupled transaction does not contain the five candidate bodies that
its receipts claim were validated, and several purported independent owner
reconstructions are comparisons with caller-supplied copies of those same
candidate states.

## Exact Bytes Reviewed

- `SC-LANDSURFACEENERGY-001.md`:
  `adf1bcd3b95a2e20f55a4f7a449426f31ed14b0595ba79db6d6d4a374b8cee20`;
- `SC-VEGETATION-001.md`:
  `1d7ec3699085fdf5d2f29e01b3c1d76b8a2a5ad8ce22340df2e066cb39f1fb1a`;
- `SC-VEGETATIONTRANSACTION-001.md`:
  `44b77e5d9854ce9c4b64214f2a669e1b09e2fdc5bdb6d1391ef25a909f186e73`;
- `SC-WATBAL-001.md`:
  `c30b7c243a36f7fc2aec316c3ba590c8f7629759d36bf1f91b60c0cf0c419188`;
- joint core:
  `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5`;
- LSE definition:
  `2ee9a7b87c8d22d270900a09312629a0f799475e45c41d0927d7ce18d3679915`;
- top-level calculator:
  `f28d6105077f89a9bfc29b09ee416ce5cd699900d3674842ef4f68ace4f71f31`;
- frozen vectors:
  `3c249fc201896db27b3cdba3fe468c241934e949122060230866b8074486391b`;
- coupled-transaction schema:
  `e9cea670e733cc97c84458ecc10b68d62aaba39caaf31535371ea45d66ddff2c`.

I read the earlier four ownership reviews and their accepted dispositions before
assessing the current bytes. To avoid a duplicate concurrent execution of the
expensive generator, the parent explicitly held this review's independent
regeneration after another release reviewer started that run. This review does
not claim a second regeneration. The closure-blocking finding below is static
and is unaffected by regenerated scalar values.

Targeted execution imported the checksum-bound calculator and used its
registered Draft 2020-12 schema validator. All six strict instances passed; the
positive condensation water protocol and routed forcing also passed their exact
schemas. An independent identity audit confirmed that:

- all five strict receipt records now equal the receipts produced for
  `post_ingress_owner_candidates`, and each receipt's candidate digest matches
  the separate physical candidate dictionary;
- all eleven failure vectors use attempted transaction `20260814001` for the
  five owners and envelope, retain six before/after rollback hashes, expose no
  candidate, and report exact rollback;
- the routed crossing uses accepted upstream transaction `20260814002` and
  downstream transaction `20260814001`; `0.6 kg m^-2` over `120 m2` and
  `0.36 kg m^-2` over `200 m2` both reconstruct `72 kg`, while source and
  destination reconstruct the same extensive energy;
- the shared competition has 19 unique request, authorization, and finalized-
  use keys, one arbitration call, and six source ending-store ledgers.

The first direct `jsonschema` audit attempt omitted the package's local schema
registry and stopped on an unresolved relative `$ref`; it produced no positive
or negative schema conclusion. The successful targeted run used the calculator's
checksum-bound registry and is the evidence reported above.

## Material Finding

### `OWN5-CRITICAL-001` — The strict transaction still omits the candidate bodies and accepts copied owner joins

The remediation makes the five receipt hashes byte-identical to the receipts
created for the physical candidate dictionaries. That closes the earlier
receipt-mismatch symptom, but not the underlying ownership gate.

`lse_v1_coupled_transaction_schema.json` permits exactly eleven root fields.
It contains `candidate_owner_receipts` but no candidate-body collection. Each
receipt contains only transaction/owner identity, beginning and candidate
SHA-256 strings, and the producer-supplied assertion `validated: true`. The
strict instance therefore cannot independently validate, or even inspect, any
ending owner state. A candidate dictionary can be removed or replaced outside
the strict instance while the schema-bound transaction remains valid.

The separate Python validator does independently reconstruct the hydrology
resource ledger from finalized uses and condensation credits. It does not do
the equivalent for the other owners:

- vegetation passes when
  `candidate.ending_state == joins.vegetation.ending_state`, but the join is a
  direct deep copy of that candidate ending state;
- soil thermal and biogeochemistry use the same candidate-to-deep-copy
  comparison;
- the material join compares two caller-supplied hashes of the same empty list;
- `owner_receipts()` then sets `validated` from the truthiness of the aggregate
  validation dictionary rather than retaining owner-specific independently
  reconstructed operands and results.

This is circular evidence for vegetation, soil thermal, biogeochemistry, and
material custody. It leaves `OWN4-CRITICAL-001`, `OWN3-CRITICAL-001`,
`OWN2-CRITICAL-003`, and the positive owner-envelope portion of
`OWN-CRITICAL-009` unresolved.

Required correction:

1. Add the exact five physical candidate bodies to the normative coupled-
   transaction schema and strict positive instance, with transaction, owner,
   model/configuration, beginning lineage, and ending-state fields bound.
2. Derive each ending body independently from authoritative operands: V8 final
   candidate operands for vegetation; D/A/F and condensation for hydrology;
   surface energy/enthalpy operands for LSE; Crank--Nicolson ground heat plus
   infiltration enthalpy for soil thermal; and accepted material/mineral-N
   operands for BGC.
3. Construct material receipts independently from vegetation proposals. An
   empty material case may be valid, but two caller-provided equal empty-list
   digests are not independent reconstruction.
4. Issue each owner receipt only after that owner's reconstruction passes, bind
   its digest to the candidate body present in the same strict transaction, and
   add poisons for a mutated/removed body, stale receipt hash, copied join, and
   producer-supplied `validated: true`.

This correction changes the coupled schema bytes and therefore must flow
through its bound schema digest, LSE definition identity, calculator constants,
fixture regeneration, contract tests, and fresh release review.

## Confirmed Corrections

### `OWN4-CRITICAL-002` — Corrected

The positive condensation protocol now has the normative authorization
`reason`, the exact schema-defined positive-credit fields, and no legacy nested
`key`. It passes `lse_v1_water_protocol_schema.json`. The accepted credit is
`0.05615664698104031 kg m^-2`; hydrology reconstructs `26.0` to
`26.05615664698104 kg m^-2`, and LSE receives the paired
`4576.369259294541 J m^-2` enthalpy credit.

### `OWN4-HIGH-003` — Corrected

Every natural and domain failure is rebound to the same attempted transaction
as its complete owner/envelope rollback record. All eleven use transaction
`20260814001`, include the five owners plus envelope, have identical before and
after hashes, carry a null candidate, and pass the normative diagnostics schema.

### `OWN4-HIGH-004` — Corrected

The routed case now distinguishes accepted upstream and downstream transaction
identity, binds the upstream accepted candidate-state digest into the routed
parcel, validates the projected downstream forcing record, and preserves exact
source/destination OFE and tile identities. The nondegenerate `120 m2` to
`200 m2` conversion closes both extensive mass and energy without treating a
tile amount as an OFE amount.

### Earlier custody and transaction findings

The canonical hydrology-only mass ownership, beginning-snapshot-only supply,
one authorization, fixed-cap rebuild, typed water identity, post-ingress
ordering, signed condensation, soil-thermal receipt, V8 successor identity,
and Child-2 real-owner boundary remain correct. The independent Python arbiter
is authority evidence only and is not actual Child-2 production hydrology or
real-consumer evidence.

## Conclusion

The release remediation corrected three of the four ownership release findings
and retained strong shared-source and routed conservation evidence. The strict
positive transaction nevertheless remains receipt-only and cannot validate the
five candidate bodies it claims were accepted. Candidate-to-copy comparisons
are not independent owner reconstruction.

**Result: FAIL. Do not release Child 1 until `OWN5-CRITICAL-001` is corrected,
the identity chain and fixture are regenerated, and a fresh independent
ownership review passes the exact resulting bytes.**
