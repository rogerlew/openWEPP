# Release Hydrology And Ownership Review

Evidence class: `Static + targeted Ran`, fresh independent exact-worktree
review.

Verdict: **NO-GO / FAIL**.

I did not reuse either earlier ownership review's conclusion. I regenerated the
fixture, reconstructed the emitted D/A/F and routed-water joins, and validated
the positive condensation and routed-runon records against the package's own
strict schemas. The shared-layer arithmetic and extensive route conversion are
now substantive, but four release-blocking ownership defects remain.

## Exact Bytes And Commands

Reviewed local `main` at base commit
`0db1960129ad4f8fc4e292b20574dfe7229d5fe1` with the uncommitted Child-1
worktree. The requested exact hashes are:

- LSE definition: `5f7ff9640d67ec7c3b747a8c81332a5906920d49cde58432b6f9de709201d8a5`;
- joint canopy/LSE core: `0e5a7b0e93cd434463c2b4d32e53de762ea5c78026ff47db28ab8d10eca6591e`;
- top-level generator: `f08d010e6984c47a64bc51b457b21bea95de0a119d5e11ff414f8815ea45b589`;
- vectors: `68ebdb09e9344a18fc71c3a284d4f72b345c79e55a4b7b489ee51994eace2744`.

Ran:

```text
.venv/bin/python artifacts/reference_calculator.py \
  --write /tmp/child1_release_review_vectors.json
sha256sum /tmp/child1_release_review_vectors.json
cmp -s /tmp/child1_release_review_vectors.json \
  artifacts/openwepp_snow_free_lse_v1_vectors.json
```

Regeneration produced `68ebdb09...` and was byte-identical. I also ran an
independent JSON/schema/operand audit, retained at
`/tmp/child1_ownership_release_audit.txt`. It counted exact keys and D/A/F,
reconstructed source and route quantities, compared actual versus schema
receipt hashes, validated the positive condensation protocol and routed parcel
against their declared schemas, and reconstructed the rollback-envelope hash.

## Material Findings

### `OWN4-CRITICAL-001` — Actual five-owner candidates are not the schema-bound transaction and several validations are circular

`post_ingress_owner_candidates` does contain five candidate bodies and calls
`validate_owner_candidates()` before `owner_receipts()`. That is a real
improvement. However, the strict coupled-transaction positive instance is a
different object: its five receipt hashes are not equal to
`post_ingress_owner_candidates.owner_receipts`, and the schema has no candidate
body collection at all. The independent audit reported:

```text
strict_receipts_are_actual False
actual_candidate_bodies_in_coupled_schema False
```

The validator independently reconstructs hydrology stores, but vegetation,
soil-thermal, and BGC ending states are accepted by comparison with `joins`
constructed as copies of those same ending states. The material join similarly
compares two caller-supplied empty-list digests. Thus the fixture proves that
five dictionaries can be hashed and cross-copied, not that the schema-bound
five-owner transaction independently reconstructs all candidate bodies before
issuing its receipts.

This leaves `OWN3-CRITICAL-001`, `OWN2-CRITICAL-003`, and the positive part of
`OWN-CRITICAL-009` unresolved. Bind the actual solved candidates and actual
receipts to the strict transaction instance, and reconstruct every body/join
from authoritative operands rather than a copied candidate-side value.

### `OWN4-CRITICAL-002` — The positive condensation owner transaction violates the strict water protocol schema

The accepted condensation solve now produces a positive
`0.05615664698104031 kg m^-2` credit, increases the hydrology source from
`26.0` to `26.05615664698104 kg m^-2`, and carries
`4576.369259294541 J m^-2` into the LSE record. Those mass/enthalpy numbers
join correctly.

But the supposedly typed positive protocol is not an instance of
`lse_v1_water_protocol_schema.json`. Direct Draft 2020-12 validation produced
five errors:

```text
authorizations[0]: required property `reason` is missing
condensation_credits[0]: unexpected property `key`
condensation_credits[0]: required `ofe_id` is missing
condensation_credits[0]: required `tile_id` is missing
condensation_credits[0]: required `surface_id` is missing
```

The positive schema fixture has an empty credit array, so its PASS cannot cover
this branch. `validate_water_protocol()` also does not enforce the declared
credit shape or authorization reason. This leaves `OWN3-HIGH-004` and the
positive evidence for `OWN-CRITICAL-002` unresolved. Use one canonical typed
credit DTO in schema, validator, vector, hydrology debit/credit reconstruction,
and LSE energy join.

### `OWN4-CRITICAL-003` — Natural-failure rollback hashes an unrelated transaction envelope

All three natural numerical failures now carry five owner hashes plus an
envelope hash and have no candidate. Their failure transaction is `73001`, but
`wrap_natural_failures()` is supplied the separate positive owner transaction,
whose envelope transaction is `20260814001`. Independently reconstructing that
positive envelope produced the exact hash stored on all three failures:

```text
failure transaction          73001
rollback envelope transaction 20260814001
digest_matches                True
```

Consequently the bytes shown unchanged are not the owner/envelope snapshot for
the failed transaction. Domain failures use another generic envelope with no
transaction identity. This is not exact atomic rollback evidence for the
attempt that failed, leaving `OWN3-HIGH-005`, `OWN2-CRITICAL-003`, and the
rollback part of `OWN-CRITICAL-009` unresolved. Construct the complete owner
envelope from each attempted transaction and compare that exact envelope and
all five owner bodies before/after.

### `OWN4-HIGH-004` — Routed mass/energy conversion closes, but the emitted runon is not a strict lineage record

The new route math is nondegenerate and correct as far as it goes: upstream
`0.6 kg m^-2` over `120 m2` becomes downstream `0.36 kg m^-2` over `200 m2`;
both sides reconstruct `72 kg` and `5,952,940.00850654 J` exactly. The
downstream ingress mass and energy also close.

The emitted `downstream_runon_parcel` nevertheless fails the declared forcing
parcel schema because its load-bearing `route_id`, interval, amount basis, and
source/destination areas are forbidden additional properties. Those fields are
therefore neither admitted nor validated by the strict DTO. In addition, the
upstream crossing belongs to transaction `20260814002`, while downstream
ingress is applied through the main transaction `20260814001`; the parcel has
no source transaction/accepted-receipt identity with which to establish the
handoff. The route validator checks state hash and scalar conservation but not
this transaction lineage.

This leaves `OWN3-HIGH-003`, the routed part of `OWN-CRITICAL-005`, and the
strict multi-OFE evidence for `OWN-HIGH-006` unresolved. Bind route sequence,
source accepted transaction/receipt, interval, basis, and both OFE areas into
one strict parcel schema, then validate the exact upstream-to-downstream
transaction ordering.

## Confirmed Corrections

- The shared competition has exactly 19 unique request, authorization, and
  finalized-use keys, one arbitration call, identical D/A/F key sets, and
  `F<=A<=D` for every record.
- Five `soil-1` competitors retain vegetation-root versus ground-surface,
  occupancy, request tile, source, OFE, layer, and stand-ground identity.
- Six source ledgers independently debit finalized use. One ending value is
  `-3.469446951953614e-18 kg m^-2`, inside the declared mass closure envelope
  but not valid as a schema-bound nonnegative owner state; binding actual
  candidate bodies under `OWN4-CRITICAL-001` must resolve that representation
  explicitly. The decisive shared vector has no positive unused authorization,
  though a separate valid protocol/poison pair exercises `F<A`.
- Post-ingress retained liquid changes LSE enthalpy, and infiltration carries a
  soil-thermal enthalpy receipt.
- The positive condensation mass/enthalpy arithmetic is correct; its DTO/schema
  identity is not.
- Multi-OFE extensive mass and energy conversion is correct; its strict lineage
  is not.
- The one-authorization and rebuild-from-beginning evidence is present for the
  coupled solve.

## Prior-Finding Reassessment

| Finding family | Fresh release assessment |
|---|---|
| `OWN-CRITICAL-001` | Corrected in canonical custody/state authority. |
| `OWN-CRITICAL-002` / `OWN3-HIGH-004` | Numerical mass/enthalpy join corrected; strict positive credit still fails its schema (`OWN4-CRITICAL-002`). |
| `OWN-CRITICAL-003` | Corrected: immutable beginning authorization precedes ingress. |
| `OWN-CRITICAL-004` / `OWN2-CRITICAL-002` / `OWN3-CRITICAL-002` | Corrected for the 19-record shared D/A/F and per-source debit evidence, subject to actual candidate binding in `OWN4-CRITICAL-001`. |
| `OWN-CRITICAL-005` / `OWN-HIGH-006` / `OWN2-HIGH-006` / `OWN3-HIGH-003` | Extensive route arithmetic corrected; strict route/transaction lineage remains unresolved (`OWN4-HIGH-004`). |
| `OWN-HIGH-007` / `OWN2-CRITICAL-001` / `OWN2-HIGH-005` | Ordinary water identity and six strict positive schemas improved; actual candidates, condensation, and routed runon are not the strict positive instances. |
| `OWN-HIGH-008` | Provider selection and missing-enthalpy rejection are corrected for ordinary forcing; routed accepted-transaction lineage remains incomplete. |
| `OWN-CRITICAL-009` / `OWN2-CRITICAL-003` / `OWN3-CRITICAL-001` / `OWN3-HIGH-005` | Not corrected: actual candidate bodies are not schema-bound/independently reconstructed, and numerical rollback uses an unrelated envelope. |
| `OWN2-CRITICAL-004` | Corrected: retained ingress and infiltration energy change their respective owner candidates. |

## Child-2 Boundary

The Python arbiter is independent **authority evidence only**. It is not the
actual openWEPP hillslope hydrology owner, despite the fixture label
`hydrology-real-owner`. Nothing in this review may be borrowed as Child-2
real-owner, production candidate, scheduler, or production-byte-invariance
evidence.

## Conclusion

The release candidate has materially stronger source-keyed arbitration,
post-ingress thermodynamics, condensation arithmetic, and multi-OFE extensive
conservation. It still cannot receive a PASS because the exact positive
condensation and route records disagree with the strict schemas, the five
actual candidate bodies are not the schema-bound independently reconstructed
transaction, and natural failures prove rollback of a different transaction
envelope.

**Result: FAIL. Do not release Child 1 until all four findings are corrected,
the fixture is re-frozen, and a fresh independent review confirms the new exact
bytes.**
