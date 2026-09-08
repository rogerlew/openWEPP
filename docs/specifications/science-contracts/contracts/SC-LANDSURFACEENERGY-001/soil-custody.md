[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| terminal-support.md#support | physical solve or receipt interval admission, including snow-free work | physical support, zero support and pre-Newton floor | section |
| soil-coupling.md#soil-coupling | physical operand or closure reconstruction | accepted conduction | whole chapter |
| water-vapor.md#water-vapor | infiltration credit reconstruction | accepted parcel enthalpy | whole chapter |
| map-custody.md#handoff | accepted native-map identity or physical receipt origin | same-map custody and original owner/receipt boundary | section |
| surface-custody.md#reseal | snow-free final identity/receipt reseal | same-map custody and original owner/receipt boundary | section |
| map-custody.md#pending | converged pending physical-map origin | same-map custody and original owner/receipt boundary | section |
| ../SC-SURFACELIQUID-001.md#version-15-exact-soil-thermal-enthalpy-carry-amendment | accepted soil credit or closure | receiver/producer exact credit join | whole external contract; frozen protocol scope |
| ../SC-SURFACELIQUID-001.md#purpose-and-scientific-scope | cross-contract surface scope | surface scope | whole external contract; frozen protocol scope |

<a id="soil-custody"></a>
# Soil Custody
Current exact soil enthalpy representation and candidate-only non-owner beginning. Exact accepted physical operands, one rounding, restart and real-consumer obligations remain binding. Candidate read custody cannot publish an owner.

<a id="exact-soil"></a>
<a id="version-15-receiver-owned-exact-soil-enthalpy-carry-amendment"></a>
## Version 15 Receiver-Owned Exact Soil-Enthalpy-Carry Amendment

Version 15 corrects a representation defect only. A finite accepted energy
credit can be far smaller than the binary64 spacing of a persistent soil-layer
enthalpy high term. Discarding that credit violates exact energy custody, while
forcing a high-term ULP invents energy. Therefore each soil layer's V2 owner
stores the exact total

```text
E_k = exact(H_hi,k) + R_k                         [J m^-2 OFE-ground],
```

where `H_hi,k` is finite binary64 and `R_k` is an exact normalized signed
dyadic. This amendment changes no flux, constitutive equation, heat capacity,
temperature relation, phase behavior, closure tolerance, event, topology,
support, or solver iteration. Version 14's vapor--phase--ingress--liquid-only-
WB14 chronology and exact `60000000000 ns` fallback floor are unchanged;
ordinary stable supports remain substantially larger than 60 seconds.

The canonical wire representation is:

```text
ExactDyadicEnthalpy {
    sign: -1 | 0 | 1,
    coefficient_hex: lowercase hexadecimal nonnegative integer,
    exponent2: signed decimal integer
}
value = sign * coefficient * 2^exponent2 J m^-2 OFE-ground.
```

Zero is uniquely `(0,"0",0)`. Nonzero values require sign `-1` or `1`, a
positive odd coefficient in lowercase hexadecimal without a leading zero, and
the unique exponent remaining after every factor of two is removed. The
coefficient is arbitrary precision. Equivalent noncanonical forms, embedded
signs, uppercase digits, even coefficients, negative zero, and zero with a
nonzero exponent reject. Resource bounds may protect parsing but must accept
every carry reachable from the complete configured finite transaction.
The unique carry zero is new-schema normalization only. It cannot rewrite the
existing binary64 high-term signed-zero representation: migration and no-op
transactions preserve those high-term bits, and this amendment introduces no
new high-term canonical-zero rule.

`SoilThermalOwnerEnvelopeV2` contains ordered
`SoilThermalLayerStateV2` records with `temperature_k`, `H_hi,k`, `R_k`, and
last accepted transaction. It and `SoilThermalOwnerRestartV2`,
`SoilThermalOwnerCheckpointV2`, and `SoilThermalEnergyCreditReceiptV2` bind the
V2 schema and exact-carry definition digests, frozen V1 parent digest,
configuration/model/contract versions, ordered layer set, state digest,
transaction/predecessor, support, and receipt chain. Checked V1-to-V2 migration
copies every V1 field and binary64 bit and adds the canonical exact-zero carry;
it never reconstructs or changes temperature. Production downgrade is always
rejected, including when all carries are zero.

For each immutable candidate, the receiver executes:

1. Validate the complete V2 beginning envelope, exact layer order and
   identities, digest/version lineage, predecessor, support, and finite high
   terms. Decode every canonical `R_k` and reconstruct `E_begin,k` exactly.
2. Validate the layer-keyed credit receipt's canonical ordered list of every
   accepted soil-internal conduction/storage, surface or snow top-boundary, and
   infiltration energy operand. Each operand is decoded exactly from its finite
   binary64 physical receipt and retains source owner/kind, equal-and-opposite
   join where applicable, OFE/layer/support, transaction, units, basis, and
   ordinal. No producer aggregate, residual, or carry is accepted.
3. Compute using exact integer arithmetic
   `E_candidate,k=E_begin,k+sum(Q_soil,k)+sum(Q_top,k)+sum(Q_inf,k)`.
   Exact addition makes the result independent of machine addition order;
   canonical receipt order remains binding custody and digest identity.
4. Correctly round `E_candidate,k` exactly once to binary64 round-to-nearest,
   ties-to-even, producing `H_hi,k`. Reject a nonfinite result or overflow to
   infinity; never clamp to the largest finite value.
5. Compute exactly `R_k=E_candidate,k-exact(H_hi,k)`, normalize it to the sole
   wire form, and require exact independent reconstruction of `E_candidate,k`.
6. Seal the V2 state and credit receipt, then atomically join all owners,
   restart/checkpoint state, and enclosing transaction. Any current or later
   failure preserves all beginning V1/V2 and production bytes exactly.

The canonical WAT5 vector begins at
`H_hi=-34315.42154113602 J m^-2` and adds accepted infiltration credit
`-8.0670339832330148e-19 J m^-2`, only `1.10875e-7` ULP. Correct nearest-even
rounding leaves `H_hi` unchanged and retains the exact negative nonzero carry
`(sign=-1,coefficient_hex="1dc319224e55f",exponent2=-109)`;
independent reconstruction closes exactly. Positive vectors also cover both
signs, exact-halfway even-low and odd-low ties, crossings to adjacent high
terms, opposite-sign and exact-zero cancellation, canonical-order permutation
with identical exact totals, minimum positive/negative subnormal operands,
normal/subnormal boundary crossings, and the largest-finite rounding boundary.
Overflow refuses.

Poison vectors cover NaN, both infinities, every noncanonical dyadic encoding,
coefficient/exponent overflow/resource abuse, wrong schema/definition/parent/
configuration/state/version/owner/transaction/predecessor/support/OFE/layer/
source kind/ordinal/digest, and receipt omission/duplication/reorder/
substitution. Every poison proves byte-exact rollback. Restart tests split
before and after a nonzero credit, produce identical final state/receipt/
checkpoint bytes, and reject replay. The canonical WAT5 transaction plus
unchanged `p61` and native-forest successor consumers must read, persist,
restore, and advance the V2 total. Producer-only, schema-only, diagnostic-only,
or tolerance-only evidence cannot close adoption.

Explicitly prohibited are producer-owned carry/residual, compensated floating
state, tolerance or closure-envelope laundering, `nextafter`, forced-ULP
installation, zeroing a nonzero carry, subnormal flushing, changing high-term
signed-zero/canonical-zero semantics, a process-physics or temperature change,
production downgrade, persistent microstepping or carry diagnostics, and
partial commit. `LSEB-E-049` is the typed version-15 schema/domain/identity/
receipt/exact-reconstruction/restart/rollback failure family.

<a id="soil-beginning"></a>
<a id="candidate-only-v2-soil-beginning-amendment"></a>
## Candidate-Only V2 Soil-Beginning Amendment

`INV-LANDSURFACEENERGY-155` — A
`DirectSoilThermalUnpublishedContinuationV2` may construct one
`SoilThermalUnpublishedPhysicalBeginningV2` only after authenticating the
installed resident against the continuation's original prepared owner, the
complete immutable owner/schema/model/run/configuration/topology identity, the
exact predecessor unpublished trial and its ending-state/seal lineage, and the
exact contiguous positive next-child support. Its transaction is the existing
prepared soil transaction. Neither support nor transaction lineage may be
rebound to the outer surface/LSE transaction.

The constructed value is a borrowed constitutive read surface for one charged
candidate-only LSE/V3 evaluation. A typed V3 soil-beginning discriminator must
keep it distinct from the ordinary publishable owner-plus-restart branch. In
the unpublished branch, no `SoilThermalOwnerEnvelopeV2`, restart, checkpoint,
accepted credit receipt, receipt-free seal, or owner-shaped bytes may be
constructed, serialized into the complete-owner projection, installed,
persisted, or exposed as accepted custody. The candidate projection may retain
only a typed non-owner custody record sufficient to replay the exact original
prepared-owner digest, predecessor-trial seal, physical beginning-state digest,
transaction/predecessor/receipt-chain identities, and child support; it cannot
be accepted or restored as an owner.

Final acceptance remains the sole promotion boundary. It must independently
replay the complete original prepared owner plus every canonically accumulated
accepted operand, require the exact selected physical ending and complete
layer-credit chain, invoke the existing canonical unpublished-composition
path, and seal exactly one `SoilThermalOwnerEnvelopeV2`, energy-credit receipt,
restart/checkpoint identity, and atomic install. A candidate-only projection is
replaced, never accepted beside the publishable projection. Support rebinding,
owner-byte synthesis, private-trial promotion, intermediate or dual acceptance,
proxy physics, receipt repair, or tolerance comparison of any identity rejects
with `LSEB-E-049` and restores every owner byte.

This amendment changes no soil constitutive equation, exact-carry arithmetic,
physical operand, temperature projection, tolerance, Stage-3 budget, temporal
floor, topology, or accepted restart wire. `SC-SOIL-001` process authority is
unchanged; exact soil-thermal representation and custody remain owned here.

`OBL-LANDSURFACEENERGY-C-010` — Prove the exact typed branch and constructor,
contiguous support and predecessor-trial authentication, absence of owner/
restart/checkpoint bytes and intermediate installation, complete accumulated-
operand/physical-ending final replay, one owner/receipt/restart seal, rejection
of rebinding/substitution/dual acceptance, and byte-exact rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Authenticate the continuation and construct a read-only unpublished physical beginning; evaluate privately; compose and seal the publishable V2 owner only at final acceptance. |
| branch/guard | Exactly one of publishable owner+restart or unpublished non-owner beginning; no conversion or fallback between branches. |
| invariant guard map | `INV-LANDSURFACEENERGY-155` -> continuation constructor, typed V3 input/projection, complete outer replay, single install. |
| test vector | `OBL-LANDSURFACEENERGY-C-010`: exact child, support/predecessor/receipt poisons, no owner bytes, final replay, one install, rollback. |
| binding exposure | `LSE-V2-UNPUBLISHED-CANDIDATE-BEGINNING`, active, `maps-to-existing-INV`, IDs `155/C-010`, dual review/verification. |
| change log | 2026-09-01, contract 20: typed candidate-only unpublished V2 soil beginning and single final owner/restart promotion; unchanged physics/tolerances/wire. |


<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-150"></a> `INV-LANDSURFACEENERGY-150` | Every soil layer retains exact receiver-owned enthalpy `E=exact(H_hi)+R`. Each candidate exactly aggregates the beginning total plus every canonical accepted soil-internal, top-boundary, and infiltration operand, rounds once to finite binary64 nearest-even `H_hi`, and stores the exact normalized signed-dyadic remainder `R`; versioned identity, receipt, restart/checkpoint, downgrade refusal, independent reconstruction, and byte-exact rollback are mandatory. | `REF-LANDSURFACEENERGY-009/011` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-022` | `[DIRECT][Static] + [INFERENCE][Static]` | runtime/test/real-consumer | `LSEB-E-049` or hard `HOLD` pending real adoption |
| <a id="INV-LANDSURFACEENERGY-155"></a> `INV-LANDSURFACEENERGY-155` | A V2 unpublished soil continuation may enter a charged candidate-only LSE/V3 evaluation solely through its authenticated read-only `SoilThermalUnpublishedPhysicalBeginningV2`. It remains a non-owner and supplies no restart, checkpoint, accepted receipt, or publishable projection bytes. Final acceptance reconstructs once from the original prepared owner, the complete canonical accumulated operands, and the exact selected physical ending, then seals one V2 owner/receipt/restart bundle. | `INV-LANDSURFACEENERGY-150`, `SC-SURFACELIQUID-001#INV-SURFACELIQUID-027`, `SC-SNOWENERGY-001#INV-SNOWENERGY-084` | `[DIRECT][Static] + [INFERENCE][Static]` | typed candidate/runtime/projection/finalization/test | `LSEB-E-049` with complete rollback |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-P-005"></a> `OBL-LANDSURFACEENERGY-P-005` | expose every accepted soil-internal, top-boundary, and infiltration energy operand with exact layer/support/source identity; never supply a rounded aggregate, residual, or carry. | Producers delivering accepted soil thermal credits | v31:L319-L321 | [Guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#exact-soil); named fixtures/tests/real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-005"></a> `OBL-LANDSURFACEENERGY-C-005` | soil thermal V2 alone owns the exact high/carry representation, credit receipt, restart/checkpoint state, and atomic commit. | Consumers of exact soil thermal enthalpy credits | v31:L334-L335 | [Guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#exact-soil); named fixtures/tests/real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-010"></a> `OBL-LANDSURFACEENERGY-C-010` | prove an unpublished V2 continuation enters LSE/V3 only as its typed authenticated physical read, emits no owner/restart bytes and performs no intermediate install, rejects support/predecessor/ receipt substitution, and accepts only through one complete original-owner replay and one atomic V2 owner/receipt/restart seal with full rollback. | Candidate-only exact-soil non-owner beginning consumers | v31:L345-L349 | [Guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#soil-beginning); named fixtures/tests/real consumers |
