[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | always | shared authority | whole chapter |
| terminal-support.md#support | physical solve or receipt interval admission, including snow-free work | physical support, zero support and pre-Newton floor | section (entry extent) |
| soil-custody.md#soil-custody | exact dyadic representation | canonical arithmetic/wire definition | whole chapter |
| litter-phase.md#litter-phase | active phase/fusion/spill or closure | accepted operands and spill | whole chapter |
| water-vapor.md#water-vapor | retained ingress or closure | physical basis and parcel receipts | whole chapter |
| map-custody.md#handoff | accepted native-map identity or physical receipt origin | same-map custody and original owner/receipt boundary | section (entry extent) |
| map-custody.md#pending | converged pending physical-map origin | same-map custody and original owner/receipt boundary | section (entry extent) |
| ../SC-SURFACELIQUID-001.md#version-16-exact-lse-surface-enthalpy-carry-amendment | exact surface owner or closure | frozen mirror and operand custody | whole external contract; frozen protocol scope |
| ../SC-SURFACELIQUID-001.md#exact-surface-parent-local-chronology-amendment | parent-local receipts/restart | partial/final owner chronology | whole external contract; frozen protocol scope |
| ../SC-SURFACELIQUID-001.md#topology-ranked-exact-surface-owner-amendment | topology/receipt reconstruction | opaque topology ordering | whole external contract; frozen protocol scope |
| ../SC-SURFACELIQUID-001.md#purpose-and-scientific-scope | cross-contract surface scope | surface scope | whole external contract; frozen protocol scope |

<a id="surface-custody"></a>
# Surface Custody
Current exact surface high/carry and frozen high-mirror custody. Apply parent-local posture and authenticated topology ordering together with the base receipt rules. The accepted litter spill is an additional negative operand under litter-phase; no original accepted operand is removed.

<a id="exact-surface"></a>
<a id="version-16-lse-surface-enthalpy-exact-carry-amendment"></a>
## Version 16 LSE Surface-Enthalpy Exact-Carry Amendment

Version 16 corrects the analogous representation defect at the retained LSE
surface. The frozen LSE V3 and surface-owner V2 binary64 fields cannot retain a
valid accepted per-tile energy credit below their high-term spacing. Discarding
that credit violates exact energy custody; forcing `nextafter` or one high-term
ULP invents energy. This amendment therefore adds one minimal LSE-owned
companion owner. It does not mutate or reinterpret standalone V1/V2/V3 wire
bytes. On the successor path only, each frozen binary64 field is a
nonauthoritative high mirror joined bit-for-bit to the authoritative exact
owner:

```text
LseSurfaceEnthalpyStateRecordV1 {
    surface_key,
    enthalpy_hi_j_m2_tile: U_hi,
    enthalpy_carry: R_U,
    last_accepted_transaction_id
}
U = exact(U_hi) + R_U                         [J m^-2 tile-ground].
```

`R_U` uses the unchanged canonical `ExactDyadicEnthalpy` wire definition from
version 15. Its unique carry-zero form does not canonicalize the binary64 high
term. `LseSurfaceEnthalpyOwnerEnvelopeV1`,
`LseSurfaceEnthalpyOwnerRestartV1`,
`LseSurfaceEnthalpyOwnerCheckpointV1`, and
`LseSurfaceEnthalpyEnergyCreditReceiptV1` bind the exact schema and definition
digests, frozen LSE V3 and surface-owner V2 parent digests, configuration and
ordered surface keys, transaction/predecessor, half-open support, owner/state
digests, and receipt chain. `SurfaceLiquidCompleteOwnerProjectionV4` joins the
unchanged projection-V3 bytes with this owner; projection V3 bytes remain
frozen. Production downgrade or execution without the exact owner is rejected
after V16 adoption, even when every carry is zero.

Checked adoption requires the frozen LSE V3 and surface-owner V2 high fields to
be bit-identical for every complete ordered surface key, copies those bits to
`U_hi`, and initializes only `R_U` to canonical exact zero. It derives no
enthalpy from temperature and changes no mass, phase, receipt, or high-term
bits. A mixed key set, signed-zero mismatch, stale parent, or partial adoption
rejects before candidate work.

For each immutable candidate and each surface key, the receiver executes:

1. Validate the exact beginning total, complete key order, both frozen high
   mirrors, definition/schema/configuration/state identities, predecessor,
   support, restart/checkpoint lineage, and finite `U_hi`.
2. Validate one canonical ordered list of accepted surface-energy operands.
   The exhaustive kinds are `phase_free_surface_energy`,
   `litter_fusion_energy`, and `retained_ingress_tile_credit`. Each operand is
   the exact dyadic decode of the finite binary64 amount actually accepted by
   its physical receipt and retains kind, ordinal, source owner/receipt,
   transaction, support, OFE/tile/surface key, units, and tile-ground basis.
   A phase-free operand list exposes the primitive accepted radiative,
   sensible, phase-specific vapor, and ground amounts; a producer residual or
   already-rounded aggregate is not an operand. Fusion is exactly the accepted
   binary64 `L_f*(m_frz-m_mlt)` receipt amount and occurs once.
3. For retained ingress, group all accepted retained parcel receipts by the
   complete destination surface key, sort by canonical receipt identity,
   reconstruct the existing finite binary64 OFE-ground group amount and its
   named finite binary64 OFE-to-tile result, and expose each resulting
   `retained_ingress_tile_credit` exactly once. Exact-carry arithmetic begins
   only after this existing physical basis conversion; it neither changes the
   parcel partition nor invents a rational replacement for the accepted
   binary64 tile credit.
4. Compute with exact integer arithmetic
   `U_candidate=U_begin+sum(Q_surface,j)`. Mathematical addition is
   order-independent; canonical operand order remains receipt identity. Round
   the exact total once to binary64 nearest-even `U_hi,candidate`, rejecting a
   nonfinite result or overflow. Compute and normalize
   `R_U,candidate=U_candidate-exact(U_hi,candidate)` and require exact
   independent reconstruction.
5. Require the LSE V3 candidate and surface-owner V2 candidate to carry the
   exact same `U_hi,candidate` bits as nonauthoritative mirrors. Their
   temperatures remain derived by the unchanged V14 heat-capacity equations
   from that high coordinate used by the constitutive solver. The carry is
   energy custody only: it is included in exact beginning/ending storage and
   closure, never converted to a temperature perturbation, flux, phase mass,
   residual tolerance, or solver forcing.
6. Seal the exact owner, credit receipt, both mirror joins, projection V4,
   restart/checkpoint state, and enclosing complete transaction atomically.
   Any current or later failure preserves all beginning V1/V2/V3, exact-owner,
   receipt, restart/checkpoint, and production bytes exactly.

Independent vectors must reconstruct `U_begin`, every named operand,
nearest-even high, carry, exact ending `U`, and both high mirrors without
reading a producer residual. They cover zero credit; positive and negative
sub-ULP credit; exact-halfway even-low and odd-low ties; high-term crossings;
opposite-sign cancellation to canonical carry zero; multiple retained parcels
for one tile; two tiles with distinct fractions and credits; fusion plus
retained ingress; minimum positive/negative subnormal operands; largest-finite
boundary and overflow refusal; and split restart before and after a nonzero
carry. Poisons cover omission, duplication, reorder, wrong kind/ordinal/source,
wrong OFE/tile/surface/support/transaction/predecessor, V2/V3 high-mirror
mismatch, stale projection/restart/checkpoint, noncanonical carry, producer
residual, tolerance repair, zero snap, discarded credit, and forced ULP. Every
poison proves full byte-exact rollback.

The retained `p61` failure support is exactly
`176400000000000..178200000000000 ns`. The retained run identified the
sub-ULP retained-surface-credit predicate but did not preserve the exact
beginning high bits or retained tile-credit operands. No numeric oracle is
invented from that evidence. Implementation must capture those typed operands
from the unchanged fixture, bind them into the receipt, and then prove exact
independent reconstruction plus split-restart equivalence. The unchanged
native-forest fixture carries the second real-consumer gate. Producer-only,
schema-only, synthetic-only, or tolerance-only evidence cannot close adoption.

This amendment changes no radiation, turbulence, vapor, soil heat, bounded
`3300 s` phase law, mass/fusion equation, water authorization, ingress/WB14
chronology, event, topology, physical closure tolerance, or solver rule. The
exact `60000000000 ns` fallback floor remains unchanged, and stable ordinary
supports must still accept steps substantially larger than 60 seconds.
Persistent microstepping/carry diagnostics, a forced ULP, zero snapping,
subnormal flushing, tolerance laundering, and partial commit are prohibited.
`LSEB-E-050` is the typed version-16 schema/domain/identity/operand/mirror/
exact-reconstruction/restart/rollback failure family.

<a id="parent-chronology"></a>
<a id="exact-surface-parent-local-chronology-amendment"></a>
## Exact-Surface Parent-Local Chronology Amendment

`INV-LANDSURFACEENERGY-153` — `LseSurfaceEnthalpyEnergyCreditReceiptV1`
seals exact parent support bounds and one canonical ending posture:
`ParentLocalPartial` when `child_end < parent_end`, or
`PersistentParentFinal` when `child_end == parent_end`. The validator derives
the posture from support; callers cannot select it independently.

A partial child advances the authoritative exact `U_hi + R_U`, frozen-parent
digests, owner-state digest, and exact receipt-chain digest, while every exact
record retains the persistent predecessor transaction marker shared bit-for-
bit by the frozen LSE V3 and surface-owner V2 mirrors. A final child stamps the
child transaction exactly once across all three owners. Beginning exact-record
markers equal the receipt predecessor in both postures. Mixed record markers,
wrong parent bounds or posture, early physical-owner advance, final predecessor
retention, omitted/duplicated exact operands, restart substitution, or replay
rejects with `LSEB-E-050` / `SURFACELIQUID-E-012` and byte-exact rollback.

The current pre-production V1 receipt wire adds the typed posture and parent
bounds to its digest preimage and changes its schema digest. Earlier V1 receipt
bytes fail closed; no implicit migration or compatibility parse is authorized.
This is custody/chronology authority only and changes no energy equation,
tolerance, temporal floor, or physical branch.


| Profile surface | Binding |
| --- | --- |
| algorithm step | Derive partial/final from sealed parent and child supports after authentic V4 physics, then advance exact carry and mirrors atomically. |
| branch/guard | Partial retains predecessor; final stamps transaction once; mirror markers and exact owner remain identical. |
| invariant guard map | `INV-LANDSURFACEENERGY-153` -> exact receipt schema/lineage validator, projection V4, restart, mirror join. |
| test vector | `OBL-LANDSURFACEENERGY-C-008`: first/middle/final, split restart, marker/bounds/posture poisons, rollback. |
| binding exposure | `LSE-V16-PARENT-LOCAL-CHRONOLOGY`, active, `new-INV`, IDs `153/C-008`, dual review/verification. |
| change log | 2026-09-01, contract 18: typed partial/final exact-surface receipt chronology; unchanged physics/tolerances. |

<a id="topology"></a>
<a id="topology-ranked-v16-exact-surface-owner-amendment"></a>
## Topology-Ranked V16 Exact-Surface Owner Amendment

`INV-LANDSURFACEENERGY-158` binds canonical V16 exact-surface record and
accepted-energy operand order to the authenticated configuration topology.
For owner records, compare the rank of `surface_key.ofe_id` in the exact
`SurfaceLiquidConfigurationV2.ofe_topology` first, then use the existing
`DirectSurfaceLiquidStoreKey` order with `ofe_id` held equal. For operands,
compare that owner-record rank first, then the existing operand kind and
ordinal. `OfeId` remains an opaque identity. Its text is never parsed as a
number and never compared lexically to infer topology. Therefore physical
topology `ofe-9 -> ofe-10` is admitted exactly as configured, as are reverse,
nonmonotone, and nonnumeric identifiers.

`LseSurfaceEnthalpyOwnerEnvelopeV1` does not serialize a second topology; it
seals the configuration digest and ordered record bytes. Its context-free
validator therefore proves schema, digest form, unique keys, finite exact
coordinates, state/receipt lineage, and beginning/ending sequence equality,
but does not impose lexical OFE order. The existing authenticated frozen-
parent/configuration join is the canonical-order authority: it validates the
configuration and SurfaceLiquid V2 owner, requires the exact-owner record
sequence to equal their complete topology-ranked sequence, joins LSE V3 keys,
and rejects stale configuration or digest. Construction adopts that sequence;
advance, receipt, restart/checkpoint, projection, and independent replay
preserve it without sorting or rebinding.

No relaxation of cardinality or custody is admitted. Duplicate, omitted,
substituted, foreign, topology-relative reordered, or within-OFE reordered
records; reordered operand groups; LSE/SurfaceLiquid topology disagreement;
stale configuration/digest; or changed beginning/ending sequence rejects with
`LSEB-E-050` / `SURFACELIQUID-E-012`. Failure preserves all exact owner/carry,
high-mirror, physical owner, receipt, restart/checkpoint, WB14, and runner
bytes. No schema field, energy arithmetic, constitutive result, tolerance,
transaction, support, or chronology changes.

`OBL-LANDSURFACEENERGY-C-013` — Prove exact adoption, advance, receipt,
independent replay, restart/checkpoint, and projection for configured topology
`ofe-9 -> ofe-10`; reverse/nonmonotone and nonnumeric opaque identifiers;
multiple within-OFE records and operand kinds/ordinals; and unchanged exact
energy bytes. Poison lexical/numeric-derived reorder, duplicate, omission,
substitution, foreign OFE, within-OFE/operand reorder, stale configuration or
digest, cross-owner topology mismatch, and beginning/ending reorder. Every
failure proves complete rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Validate uniqueness without OFE spelling order; at the authenticated configuration join validate the complete topology-ranked owner sequence and topology-ranked operand groups. |
| branch/guard | Configuration rank is the sole OFE ordering authority. Bare validation cannot promote bytes to installable custody, and lexical/numeric OFE inference is forbidden. |
| invariant guard map | `INV-LANDSURFACEENERGY-158` -> exact-owner adoption/parser, frozen-parent/configuration join, receipt/restart/projection, independent replay, and installer. |
| test vector | `OBL-LANDSURFACEENERGY-C-013`: multi-digit/opaque topology, within-OFE and operand order, duplicate/omission/substitution/reorder/stale-config poisons, rollback. |
| binding exposure | `LSE-V16-TOPOLOGY-RANKED-EXACT-OWNER`, active, `maps-to-existing-INV`, IDs `158/C-013`, dual review/verification. |
| change log | 2026-09-02, contract 23: authenticated configuration-topology rank replaces lexical OFE ordering for V16 exact-owner records and operands; unchanged wire schema, exact arithmetic, physics, and custody. |


<a id="reseal"></a>
<a id="snow-free-final-receipt-reseal-amendment"></a>
## Snow-Free Final-Receipt Reseal Amendment

`INV-LANDSURFACEENERGY-160` admits no new physical result. The provisional
snow-free V11 execution must first complete all existing validation and retain
its exact immutable LSE, surface, hydrology, BGC, vegetation, and soil ending.
A private move-only proof may then authorize the final accepted-slab pass to
reconstruct and reseal only identities whose preimage contains the accepted
slab receipt. Phase-free evaluation, litter phase, resource arbitration,
current ingress, WB14, soil thermal, and every constitutive or conservation
equation are forbidden in that final reseal.

The proof binds the live beginning revision, parent/segment/slab ordinal,
support and duration, configuration/topology, forcing and support receipt,
beginning complete owners, all non-slab inputs, provisional slab identity, and
the validated physical ending. Consumption requires an independently created
final slab whose only permitted binding difference is its ending-owner-derived
accepted-slab identity. The resealed final complete-owner bytes must equal the
provisional physical ending exactly. The provisional pass publishes nothing;
the final pass publishes exactly once. Missing, reused, stale, foreign,
mutated, or restored proofs fail before owner mutation; restart always executes
fresh physics and cannot serialize or recover the proof.

`OBL-LANDSURFACEENERGY-C-015` requires forced direct-versus-reuse byte equality,
one-call counters for every physical provider and adjacent owner operation,
zero provisional and one final publication, exhaustive non-slab identity and
physical-ending poisons, single-use rejection, pre/post-restart vectors, and
complete rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Fully validate one snow-free physical execution, consume one private proof to reseal final accepted-slab identities, compare exact ending owners, then publish once. |
| branch/guard | The final reseal cannot call any physical evaluator; a missing or invalid proof is a typed failure with no replay fallback. Restart invalidates the proof and requires fresh physical execution. |
| invariant guard map | `INV-LANDSURFACEENERGY-160` -> private physical-reuse typestate, accepted-slab-only delta validator, exact ending-owner comparison, provider/publication counters, restart and rollback gates. |
| test vector | `OBL-LANDSURFACEENERGY-C-015`: direct/reuse equality, physical call count one, publication 0/1, non-slab/ending poisons, duplicate use, restart refusal/fresh evaluation, rollback. |
| binding exposure | `LSE-V25-SNOW-FREE-PHYSICAL-REUSE`, active, `maps-to-existing-INV`, IDs `160/C-015`, dual review/verification. |
| change log | 2026-09-02, contract 25: admitted one private single-use snow-free physical ending for final accepted-slab identity resealing; physical equations, exact owners, publication, restart, and rollback are unchanged. |

<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-151"></a> `INV-LANDSURFACEENERGY-151` | On the frozen-litter V16 successor path, every LSE tile retains authoritative exact surface enthalpy `U=exact(U_hi)+R_U`. The LSE V3 and surface-owner V2 binary64 fields remain byte-frozen nonauthoritative high mirrors, every accepted phase-free/fusion/retained-ingress operand is aggregated exactly, the exact total is rounded once to finite nearest-even `U_hi`, and the exact normalized signed-dyadic remainder is retained through receipt, restart/checkpoint, real-consumer adoption, and byte-exact rollback. | `REF-LANDSURFACEENERGY-009/012` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-023` | `[DIRECT][Static] + [INFERENCE][Static]` | runtime/test/real-consumer | `LSEB-E-050` or hard `HOLD` pending `p61` and native-forest adoption |
| <a id="INV-LANDSURFACEENERGY-153"></a> `INV-LANDSURFACEENERGY-153` | The V16 exact-surface receipt seals a typed coupled-parent ending posture. Every accepted partial child advances exact surface energy/carry, frozen-parent digests, owner-state digest, and receipt-chain digest while retaining the exact persistent predecessor marker mirrored by LSE V3 and surface-owner V2. Only the child ending at the sealed parent endpoint uses the persistent-parent-final posture and stamps its transaction marker once across all three owners. Parent bounds, child support, posture, predecessor, mirror markers, and complete rollback are exact. | `SC-COUPLEDTIME-001#INV-COUPLEDTIME-006/024` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-012/013/014/025` | `[DIRECT][Static] + [INFERENCE][Static]` | receipt/runtime/restart/test | `LSEB-E-050` / `SURFACELIQUID-E-012` |
| <a id="INV-LANDSURFACEENERGY-158"></a> `INV-LANDSURFACEENERGY-158` | The V16 exact-surface owner and receipt order every record/operand group by authenticated configuration topology rank of `ofe_id`, then by the existing within-OFE surface key and operand kind/ordinal. OFE IDs are opaque: neither lexical comparison nor numeric parsing is topology authority. Bare envelope validation may prove schema, digest, uniqueness, and lineage, but installable canonical order is established only at the exact configuration join; stale configuration/digest, omission, duplication, substitution, or topology-relative reorder rejects atomically. | `SC-SURFACELIQUID-001#INV-SURFACELIQUID-014/023/030` + exact-owner transaction authority | `[DIRECT][Static] + [INFERENCE][Static]` | topology-ranked owner/receipt/configuration join and multi-digit OFE tests | `LSEB-E-050` / `SURFACELIQUID-E-012` with complete rollback |
| <a id="INV-LANDSURFACEENERGY-160"></a> `INV-LANDSURFACEENERGY-160` | A fully validated snow-free V11 provisional execution may transfer its immutable LSE physical ending through one private move-only proof to the final accepted-slab transaction. The final path may reseal only slab/receipt-dependent identities and must not repeat phase-free evaluation, litter phase, current ingress, WB14, soil evaluation, or any energy equation. Exact physical operands, owner bytes, support, topology, configuration, predecessor, and non-slab lineage must match; the final LSE owner and complete owner set must equal the provisional physical ending byte-for-byte. The proof is single-use and absent from restart, checkpoint, receipt, publication, and wire. | `INV-LANDSURFACEENERGY-151/153/155/156/157/159` + `SC-COUPLEDTIME-001#INV-COUPLEDTIME-029` | `[INFERENCE][Static]` | private snow-free physical-reuse typestate, exact final reseal/reconstruction, physical-provider call counter, restart and poison gates | `LSEB-E-040/047/048/050` with complete rollback |


<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-P-006"></a> `OBL-LANDSURFACEENERGY-P-006` | expose every accepted per-tile phase-free, fusion, and retained-ingress energy amount with exact kind, ordinal, support, transaction, surface key, source receipt, unit, and tile-ground basis; never supply a producer residual, carry, or untyped aggregate. | Producers delivering accepted surface enthalpy operands | v31:L322-L325 | [Local guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#exact-surface); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-006"></a> `OBL-LANDSURFACEENERGY-C-006` | the LSE exact-surface owner alone owns authoritative `U=exact(U_hi)+R_U`; frozen LSE V3 and surface-owner V2 fields are high mirrors only when joined through the V16 successor projection. | Consumers of exact LSE surface enthalpy operands | v31:L336-L338 | [Local guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#exact-surface); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-013"></a> `OBL-LANDSURFACEENERGY-C-013` | prove V16 exact-owner records and operand groups follow authenticated configuration-topology rank for opaque OFE IDs, including `ofe-9 -> ofe-10`, and reject lexical/numeric-derived order, duplicate, omission, substitution, within-OFE/operand reorder, stale configuration/digest, and cross-owner topology mismatch with full rollback. | Topology-ranked exact-surface owner consumers | v31:L361-L365 | [Local guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#topology); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-008"></a> `OBL-LANDSURFACEENERGY-C-008` | Prove first and middle partial children retain the predecessor while exact carry/state/receipt chain advance; the final child stamps the transaction once; mid-parent restart reproduces the same canonical bytes; wrong posture/bounds, mixed markers, early physical advance, final predecessor retention, and injected failure all reject with full rollback. | Exact-surface parent-local chronology consumers | v31:L2135-L2139 | [Local guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#parent-chronology); named fixtures/tests and real consumers |
| <a id="OBL-LANDSURFACEENERGY-C-015"></a> `OBL-LANDSURFACEENERGY-C-015` | prove snow-free provisional/final execution evaluates every LSE, litter-phase, surface-ingress/WB14, and soil physical operator exactly once; the final accepted-slab path reseals identities only, yields byte-identical final owners and one publication, rejects every changed non-slab operand and any reused or post-restart proof, and rolls back exactly. | Snow-free final receipt reseal consumers | v31:L373-L377 | [Local guards/errors](water-vapor.md#errors) | [Tests](common-details.md#tests); [Detail](#reseal); named fixtures/tests and real consumers |
