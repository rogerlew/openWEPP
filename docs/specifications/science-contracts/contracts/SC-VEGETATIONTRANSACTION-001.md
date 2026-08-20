---
contract_id: SC-VEGETATIONTRANSACTION-001
title: Coupled Vegetation Occupancy Owner-Transaction Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + vegetation/hydrology/energy reviewer
contract_version: 9
producer_scope:
  - OPENWEPP_C3_WOODY_V8 occupancy and ground resource/energy candidates
  - OPENWEPP_C3_WOODY_V11 accepted-segment and parent candidates
consumer_scope:
  - Default-off real-hydrology, LSE, BGC, and soil-thermal shadow owners
  - Default-off coupled-time V11 parent coordinator and additive restart
evidence_level: static+independent_oracle
last_reviewed: 2026-08-20
supersedes: []
superseded_by: []
---

# SC-VEGETATIONTRANSACTION-001 Coupled Vegetation Occupancy Owner-Transaction Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static + independent oracle`

## Purpose

Define shared typed identity and independent receiving-owner obligations for
default-off vegetation transactions, including V11 accepted-segment staging
under one coupled-time parent, without activating production selectors.

## Scientific Scope and Explicit Out-of-Scope Boundaries

In scope are occupancy water arbitration, mineral-N and material custody,
component-energy operands, accepted-segment/event chronology, owner candidate
validation, additive restart, independent reconstruction, and atomic parent
commit. Production cutover, snow-covered carrier equations, and soil
transformations are excluded.

## Authority Anchors with Top-Down Citations

| Anchor | Authority | Use | Evidence |
|---|---|---|---|
| `REF-VEGTRANSACTION-001` | `SC-VEGETATION-001#INV-VEGETATION-010..015,073..079` | producer state/area/transaction semantics | `[DIRECT][Static]` |
| `REF-VEGTRANSACTION-002` | `SC-WATBAL-001#INV-WATBAL-101` | hydrology sole mutation and same-layer arbitration | `[DIRECT][Static]` |
| `REF-VEGTRANSACTION-003` | `SC-LANDSURFACEENERGY-001#INV-LANDSURFACEENERGY-010/012/042/043` | independent energy and latent identity | `[DIRECT][Static]` |
| `REF-VEGTRANSACTION-004` | physical conservation/dimensional identity | exact-one debit/conversion | `[INFERENCE][Static]` |
| `REF-VEGTRANSACTION-005` | `SC-COUPLEDTIME-001@2` | parent/slab/event/scheduled/restart/atomic chronology | `[DIRECT][Static]` |

## Variables and Units Using Canonical Symbols First

| Symbol | Units | Meaning |
|---|---|---|
| `tau` | typed transaction ID | immutable owner-set transaction |
| `o=(s,t)` | typed identity | exact stratum/tile occupancy |
| `f_t` | fraction | positive tile fraction within one OFE |
| `D_W,o,l`, `A_W,o,l`, `F_W,o,l` | `kg H2O m^-2 OFE-ground` | request, maximum authorization, final use |
| `Q_o,k` | `J m^-2 tile-ground` | immutable occupancy energy component |

## Algorithm State Surfaces

Inputs are typed vegetation/LSE requests and component operands plus immutable
beginning hydrology, vegetation, LSE and soil-thermal snapshots. Hydrology alone
owns every water mass and forms all candidate debits, condensation credits,
current-ingress partitions, infiltration, and runoff. LSE owns one surface
thermal node per tile; soil thermal owns all soil temperatures/enthalpies and
the conductive/advective receipts. The orchestrator commits the complete owner
set.

For V11, the state surface additionally includes the immutable parent beginning
owner set, current staged complete owner set, accepted slab/event/scheduled
receipts, typed water and NH4/NO3 inventories and cumulative ledgers, ordered
material transfers, accepted-only reductions, buffered publication, and the
current/next parent sequence. None is a live-owner mutation before commit.

## Algorithm Specification with Step Sequence

1. Reject duplicate/mismatched `(tau,s,t,l,resource,amount_basis)` identities
   before sorting or summation.
2. Freeze water availability from immutable beginning stores before current
   rain, runon, or canopy release. Hydrology groups all root and ground requests
   by exact source/layer on the OFE-ground basis and returns full or
   equal-status proportional maximum authorization per exact request.
3. Each typed vegetation or LSE requester divides only its own authorization by
   positive `f_t` for its local capped solve and multiplies its local finalized
   use by the same `f_t` on return. No requester may consume another's cap.
4. Hydrology validates `0<=F<=A<=D`, exact identity, aggregate source/layer
   debit, and debits finalized use only. Unused authorization remains in the
   beginning inventory. Negative signed ground vapor is not a withdrawal: it
   creates one typed condensation credit after the capped solve.
5. Energy independently constructs each occupancy candidate from immutable
   direct/diffuse VIS/NIR, leaf/stem shortwave, incident/emitted longwave,
   sensible, finalized transpiration, wet evaporation/condensation, dry-stem,
   ground, storage/conductive, interval, and authority-tagged latent operands.
   It never consumes a producer residual or copied candidate.
6. Each occupancy closes on tile-ground basis; energy then weights accepted
   components once by `f_t` and independently reconstructs the stand ledger.
7. After the capped solve, hydrology accepts current precipitation, runon and
   final canopy releases and partitions them once into retained storage,
   infiltration, routed runoff, and outlet runoff. Current ingress never
   changes the already fixed authorizations. Each crossing retains one mass,
   temperature, liquid enthalpy, source/destination OFE, tile, and interval.
8. Soil thermal independently receives `+G` from the surface and infiltration
   energy at layer 1; LSE independently records `-G`; routed runoff carries its
   mass/enthalpy downstream. No owner calculates a second `G` or advection term.
9. Validate vegetation, hydrology, LSE, BGC, and soil-thermal candidates before
   one atomic shadow commit. Any failure preserves every beginning and
   production byte.
10. For V11, repeat steps 1--8 only through a closed accepted-slab capability,
    with each accepted ending becoming the next staged beginning. Interleave
    admitted zero-duration event receipts in coupled-time order; events may
    change regime/participants/custody but never integrate a rate.
11. Independently reconstruct ordered water and NH4/NO3 debits, material
    transfers, scheduled-once execution keys, reductions, and predecessor
    digests from the parent beginning through the final staged owner set.
12. Consume one authenticated parent candidate to install the complete owner
    set, increment once, and make buffered publication durable. No callable
    per-segment commit or vegetation-only finalize exists.

## Branch and Guard Table

| Trigger | Behavior | Failure |
|---|---|---|
| duplicate/swap/stale identity | reject before arbitration | `VEGTXN-E-001` |
| nonfinite/negative/wrong basis | reject | `VEGTXN-E-002` |
| authorization/use bound failure | reject all candidates | `VEGTXN-E-003` |
| omitted/double/wrong `f_t` | reject dual reconstruction | `VEGTXN-E-004` |
| copied producer energy residual/candidate | reject | `VEGTXN-E-005` |
| wrong tile/owner/interval/digest/component | reject | `VEGTXN-E-006` |
| partial owner set/validation failure | rollback exact bytes | `VEGTXN-E-007` |
| current ingress used for same-interval authorization | reject | `VEGTXN-E-008` |
| missing/duplicate condensation or liquid-enthalpy crossing | reject | `VEGTXN-E-009` |
| wrong OFE, routed destination, or soil-thermal receipt | reject | `VEGTXN-E-010` |
| wrong parent/segment/slab/event/participant/predecessor | reject before staging | `VEGTXN-E-011` |
| stale staged inventory, unordered debit, overbooking, or arithmetic mismatch | reject parent | `VEGTXN-E-012` |
| per-segment/partial/duplicate commit or publication | reject atomically | `VEGTXN-E-013` |
| restart omission, replay, or reconstructed-chain mismatch | reject continuation | `VEGTXN-E-014` |

## Invariants and Invariant Guard Map

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| `INV-VEGTRANSACTION-001` | Water identity and stand basis persist through request, authorization, final use, debit, and receipt. | REF-001/002/004 | `[INFERENCE][Static]` | dual owner | `VEGTXN-E-001..004` |
| `INV-VEGTRANSACTION-002` | Hydrology arbitrates one same-layer snapshot and debits final use only. | REF-002/004 | `[DIRECT+INFERENCE][Static]` | hydrology | `VEGTXN-E-003` |
| `INV-VEGTRANSACTION-003` | Energy reconstructs each occupancy and weighted stand result independently. | REF-001/003/004 | `[DIRECT+INFERENCE][Static]` | energy | `VEGTXN-E-004..006` |
| `INV-VEGTRANSACTION-004` | Every owner commits together or all beginning bytes remain identical. | REF-001/004 | `[INFERENCE][Static]` | orchestrator | `VEGTXN-E-007` |
| `INV-VEGTRANSACTION-005` | Root and ground withdrawal share immutable beginning stores; current ingress is partitioned only after the capped solve and cannot alter authorization. | REF-002/004 | `[DIRECT+INFERENCE][Static]` | hydrology/orchestrator | `VEGTXN-E-008` |
| `INV-VEGTRANSACTION-006` | Signed condensation has one hydrology mass credit and one paired energy credit; evaporation alone produces a withdrawal. | REF-003/004 | `[INFERENCE][Static]` | dual owner | `VEGTXN-E-009` |
| `INV-VEGTRANSACTION-007` | Surface `-G`, soil `+G`, infiltration energy and routed runoff enthalpy preserve exact OFE/tile/source identity. | REF-003/004 | `[INFERENCE][Static]` | LSE/soil thermal/hydrology | `VEGTXN-E-010` |
| `INV-VEGTRANSACTION-008` | The terminal receiver binds snow and receiver halves by one predecessor chain while preserving independent absolute support, total error precedence, restart custody, atomic commit/rollback, and CoE production invariance. | REF-001/004 + terminal contracts | `[INFERENCE][Static]` | orchestrator | `VEGTXN-E-007` |
| `INV-VEGTRANSACTION-009` | Segment resource identity extends, never replaces, parent/owner/OFE/tile/occupancy/layer/species/basis identity. | SC-COUPLEDTIME-001 + V11 amendment | `[INFERENCE][Static]` | orchestrator | `VEGTXN-E-011` |
| `INV-VEGTRANSACTION-010` | Each occupancy debit authorizes against current staged shared inventory; the parent independently reconstructs ordered debit receipts and typed shared-owner transitions whose endings—not occupancy post-use fields—form cross-segment predecessors. | V11 amendment | `[INFERENCE][Static]` | resource owners | `VEGTXN-E-012` |
| `INV-VEGTRANSACTION-011` | Ordered segment material receipts form one parent batch without final-state recomputation. | V11 amendment | `[INFERENCE][Static]` | vegetation/material owner | `VEGTXN-E-013` |
| `INV-VEGTRANSACTION-012` | Exactly one complete parent commit installs all owners and increments once. | SC-COUPLEDTIME-001 + V11 amendment | `[INFERENCE][Static]` | orchestrator | `VEGTXN-E-014` |
| `INV-VEGTRANSACTION-013` | Restart reconstructs the staged owner/receipt chain and cannot replay accepted work. | SC-COUPLEDTIME-001 + V11 amendment | `[INFERENCE][Static]` | restart owner | `VEGTXN-E-014` |

### Invariant Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-VEGTRANSACTION-001` | typed DTO/owner poisons | runtime/test | reject | V2 implementation package |
| `INV-VEGTRANSACTION-002` | real-hydrology shadow owner | runtime/test | reject | V8/LSE implementation package |
| `INV-VEGTRANSACTION-003` | independent energy owner | runtime/test | reject | V2 implementation package |
| `INV-VEGTRANSACTION-004` | all-owner commit/injection | runtime/test | rollback | V2 implementation package |
| `INV-VEGTRANSACTION-005` | immutable-snapshot/order poisons | runtime/test | reject | V8/LSE implementation package |
| `INV-VEGTRANSACTION-006` | signed vapor mass/energy join | runtime/test | reject | V8/LSE implementation package |
| `INV-VEGTRANSACTION-007` | ground/advection cross-owner joins | runtime/test | reject | V8/LSE implementation package |
| `INV-VEGTRANSACTION-008` | terminal all-owner state machine, precedence, and rollback poisons | default-off runtime/test | reject/rollback | terminal handoff package |
| `INV-VEGTRANSACTION-009` | parent/segment/slab/resource identity reconstruction | default-off runtime/test | reject | V11 segmented-support package |
| `INV-VEGTRANSACTION-010` | occupancy/shared-owner alias, missing debit link, forged transition, overbooking, or broken shared predecessor | default-off runtime/test | reject/rollback | V11 segmented-support package |
| `INV-VEGTRANSACTION-011` | ordered material receipt/proposal reconstruction | default-off runtime/test | reject/rollback | V11 segmented-support package |
| `INV-VEGTRANSACTION-012` | consuming complete-owner parent commit and late-failure injection | default-off runtime/test | rollback | V11 segmented-support package |
| `INV-VEGTRANSACTION-013` | fresh restore, event boundary, replay, reduction and publication poisons | default-off runtime/test | reject/rollback | V11 segmented-support package |

## Producer Obligations and Consumer Obligations

- `OBL-VEGTRANSACTION-P-001`: vegetation emits typed occupancy requests and
  immutable energy operands, never owner residuals.
- `OBL-VEGTRANSACTION-C-001`: hydrology and energy independently validate their
  identities, bases, candidates, and receipts.
- `OBL-VEGTRANSACTION-C-002`: no vegetation-only commit API is closure eligible.
- `OBL-VEGTRANSACTION-P-002`: V11 producers emit authenticated slab, event,
  scheduled, resource, material, reduction, and publication receipts in
  accepted chronology; receipt IDs are not caller-selected.
- `OBL-VEGTRANSACTION-C-003`: resource owners reconstruct each ordered debit
  from the current staged inventory and the parent independently reconstructs
  the cumulative ledger and ending owner bytes.
- `OBL-VEGTRANSACTION-C-004`: restart admission authenticates every retained
  owner/receipt/buffer and returns only a closed continuation capability.

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `D_W/A_W/F_W,o,l` | future occupancy resource DTO | diagnostic | stand-ground interval mass | this / SC-WATBAL-001 |
| `Q_o,k` | future energy operands | diagnostic | tile-ground interval energy | this / SC-LANDSURFACEENERGY-001 |
| `D_R/A_R/F_R,k` | `V11ResourceReceiptV1` | parent/segment/slab/resource key | finite binary64 interval amount | this / resource owner |
| `M_k` | `V11MaterialReceiptV1` | ordered accepted material transfer | `kg C`, `kg N`, or dry matter per declared basis | this / BGC-material owner |
| `C_parent` | `V11ParentCommitCandidateV1` | consuming complete owner replacement | identity/ledger only | this / SC-COUPLEDTIME-001 |

## Constants and Parameters with Provenance Anchors

No empirical constant is introduced. `f_t` is caller topology; enthalpy and all
constitutive values retain V2/adjacent-owner authority.

## Unit-Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| occupancy water | `kg H2O m^-2 stand-ground` | future typed DTO | one-time `f_t` conversion | none | none |
| occupancy energy | `J m^-2 tile-ground` | future typed DTO | weight after local closure | none | none |
| V11 water debit | owner-declared `kg H2O m^-2` basis | `V11ResourceReceiptV1` | no local conversion after receipt | none | parent operand lineage |
| V11 mineral N debit | owner-declared `kg N m^-2` basis, separate NH4/NO3 | `V11ResourceReceiptV1` | no NH4/NO3 aggregation | none | parent operand lineage |
| V11 material transfer | typed C/N/dry-matter amount and basis | `V11MaterialReceiptV1` | none | none | ordered parent batch lineage |

## Tolerance and Numeric Notes

INV-008 wall support, sequential predecessor identity, restart stage, consumed
marker, error rank, owner bytes, and rollback hashes are exact and admit no
tolerance repair.

Identity and conversion counts are exact. No tolerance repairs an identity,
unit, basis, duplicate, or wrong-owner defect.

## Calibration and Identifiability

`CALIBRATION_NOT_APPLICABLE`: this contract governs ownership and transactions.

## Test-Vector Obligations

Require distinct occupancy/layer requests; duplicate/swap; zero demand/supply;
oversubscription; unused authorization; wrong/double/missing `f_t`;
authorization-as-use; component omission/duplication; tile/ground and
rate/amount aliases; producer-residual poison; and phase-injection rollback.
V11 additionally requires unequal-support order aliases, start/interior/end
events, active-participant changes, water and separate NH4/NO3 overbooking,
scheduled replay, rejected-attempt no-op, mid-parent/event-boundary restart,
publication-before-commit, abort, consecutive parents, and exact-one commit.

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `BEI-VEGTRANSACTION-001` | Native transaction authority | `active` | `maps-to-existing-INV` | `INV-VEGTRANSACTION-001, INV-VEGTRANSACTION-002, INV-VEGTRANSACTION-003, INV-VEGTRANSACTION-004` | `flagged-binding-addition` | Typed water, energy, and atomic owner transaction authority. |
| `BEI-VEGTRANSACTION-002` | V8/LSE shared-hydrology amendment | `active` | `maps-to-existing-INV` | `INV-VEGTRANSACTION-005, INV-VEGTRANSACTION-006, INV-VEGTRANSACTION-007` | `flagged-binding-addition` | Immutable shared hydrology snapshots and reciprocal mass/energy joins. |
| `BEI-VEGTRANSACTION-003` | Terminal receiver amendment | `active` | `maps-to-existing-INV` | `INV-VEGTRANSACTION-008` | `flagged-binding-addition` | Phase-aware predecessor chain, restart, and rollback authority. |
| `BEI-VEGTRANSACTION-004` | V11 segmented parent-transaction amendment | `active` | `maps-to-existing-INV` | `INV-VEGTRANSACTION-009, INV-VEGTRANSACTION-010, INV-VEGTRANSACTION-011, INV-VEGTRANSACTION-012, INV-VEGTRANSACTION-013` | `flagged-binding-addition` | Segment resource identities, staged custody, ordered material accumulation, atomic parent commit, and restart. |

## Gap Register and Promotability Labels

| Gap ID | Gap | Required closure | Label |
|---|---|---|---|
| `GAP-VEGTRANSACTION-001` | typed runtime owners are not V2-complete | Stage-B implementation/tests | `IMPLEMENTATION_MISSING` |

## V8/LSE Shared-Hydrology Amendment

Version 2 extends exact resource identity to ground sources:
`(transaction,OFE,tile,surface_class,requester,resource,optional_layer,basis)`.
Vegetation root and ground/litter evaporation requests are constructed from one
immutable beginning real-hydrology snapshot, before current ingress, and
authorized in one batch. Each
authorization corresponds to one unchanged potential request. A final capped
joint solve produces finalized uses; hydrology alone debits those uses and
credits signed condensation receipts.

Canopy shortfall is never ground demand, unused root authorization is never a
ground credit, current ingress is unavailable to same-interval ET, and no
request may be inflated or reissued after the final solve. Signed condensation
is credited explicitly rather than clipped. The owner envelope additionally
contains LSE and soil-thermal candidates,
with reciprocal longwave, shared canopy-air H/E, latent mass/energy,
liquid-advection and equal/opposite `G` joins. Production state is outside this
default-off envelope and remains byte-identical.

`GAP-VEGTRANSACTION-001` is closed for the V7 diagnostic owner set and remains
`IMPLEMENTATION_MISSING` for the V8/LSE real-hydrology shadow until the child
runtime packages pass.

## Terminal Receiver All-Owner Amendment

`INV-VEGTRANSACTION-008` extends the existing atomic candidate envelope only
for internal default-off `terminal_receiver_v1`. Its immutable beginning owner
set additionally includes persistent Stage 3 snow/event controller and restart,
surface-liquid/WB14 partial continuation, receiver surface selection, frost,
and routed-water states. Snow advances to `t*`; the selected V10/LSE-V2/direct-
hydrology owners advance only over `dt_remaining`. One transaction ID and exact
predecessor set bind both halves.

Snow-side error precedence applies until a valid terminal receipt exists;
thereafter each receiving owner retains canonical precedence; join/closure and
rollback validation follow. Any failure restores every beginning owner byte and
predecessor identity, including pre/mid/post-event restart state. Production
owners, CoE state, selectors, public output, and protected bytes remain
identical. A partial commit, replaced causative error, simultaneous CoE/Stage 3
liquid generation, or stale post-event snow operand is a hard transaction
failure.

This does not authorize carrier/forest applicability, physical efficacy,
qualification, assurance approval, production activation, or cutover.

| Canonical surface | INV-008 binding |
|---|---|
| Algorithm | validate support/restart; execute snow; join receipt; execute selected receiver; validate all candidates; commit once |
| Branch/guard | total eleven-level precedence in SC-SNOWFREEZE-001; rollback diagnostic is secondary unless itself first; poisoned rollback never commits |
| Alias/unit | absolute wall identity is independent of sequential transaction/predecessor identity; owner bytes and consumed marker are exact |
| Tests | inject every precedence level, rollback-validator failure with earlier cause, rollback-only failure, restart stages, CoE/default byte invariance |

## V11 segmented parent-transaction amendment

Version 4 imports every V3 single-support owner identity and adds a parent /
accepted-segment hierarchy under `SC-COUPLEDTIME-001@2`. The complete parent
owner set is fixed. Each admitted vegetation slab carries parent transaction,
segment, slab, participant, support, duration bits, beginning/ending owner-set
digests, and typed water/N/energy/material receipts. Segment candidates are
staged only and cannot increment or commit the persistent parent transaction.

For each occupancy debit identity, extend the existing tuple with parent transaction,
segment ID, accepted slab ID, occupancy, layer, and source. Water and NH4/NO3
requests authorize against the current staged shared-owner snapshot. Debit
receipts bind request, authorization, and final vegetation use but are not
shared owner predecessor records. The parent validator orders receipts by
accepted chronology and independently reconstructs the ordinary `+0.0`-seeded
ordered vegetation-use fold. It authenticates receipt chronology, not shared
owner state. Missing/duplicate receipts, stale parent beginnings, or a
later segment using an earlier inventory rejects the complete parent.

A distinct shared-resource transition is keyed by owner/OFE/layer/source and
binds segment beginning/ending shared inventory, ordered admitted debit receipt
IDs, and either complete other-flux lineage or a canonical complete-owner
candidate digest. Transition ending is the next segment transition beginning.
Every debit is linked exactly once, unknown/missing/duplicate links reject, and
the sum of admitted authorizations/final uses cannot exceed the current staged
owner authorization. Occupancy post-use amounts cannot be substituted for the
shared hydrology or BGC ending.

Material transfers are amount-bearing segment receipts whose source chronology
is immutable. Parent finalization concatenates accepted transfers in slab and
within-segment canonical order, assigns stable parent-scoped proposal IDs, and
validates receiving BGC/residue candidates. It may not rerun turnover or derive
transfers from only the final vegetation state.

The V11 parent commit candidate contains exactly one ending candidate for every
complete owner, one successor V11 state, ordered accepted slab/event/scheduled
receipts, cumulative ledgers, and one parent receipt. A consuming atomic commit
checks the live parent clock and beginning owner set, installs all candidates,
increments once, and releases buffered publication. No public segment commit or
vegetation-only finalize capability exists.

`V11CompleteOwnerManifestV1` fixes canonical owner-class order as
`vegetation`, `snow`, `land_surface_energy`, `surface_liquid`, `hydrology`,
`bgc`, then `soil_thermal`; a parent configuration may mark any non-vegetation
class inactive, but its manifest entry remains present and its bytes must carry
unchanged. Version 1 admits exactly one aggregate owner envelope for each of
these seven classes; per-OFE/tile/occupancy/layer identities remain typed inside
that envelope. A future multiple-envelope class requires a successor manifest.
Owner IDs are strict ascending canonical UTF-8 bytes and unique. The manifest
binds expected count, each owner schema/model/configuration ID,
beginning digest, active/inactive segment disposition, and ending digest. A
candidate with a missing, extra, reordered, duplicate, or unknown owner is not
constructible.

`V11MaterialProposalV1` frames parent transaction, source segment/slab receipt,
within-slab ordinal, source/destination owner and pool, element/species, amount
bits, basis, and beginning/ending transfer digests. Its ID is SHA-256 over
domain `openwepp-v11-material-proposal-v1` plus the length-framed fields in that
order. Parent material order is accepted chronology then within-slab ordinal;
reordering changes the parent receipt and fails the receiving-owner ledger.

Its canonical closed wire binds schema/model/authority/configuration IDs;
parent interval and transaction; current and successor sequence; parent
beginning and ending complete-owner digests; ordered slab, event, scheduled,
resource and material receipt IDs; reconstructed water/N/material ledger
digests; successor V11 state digest; accepted-only reduction digest; ordered
publication-record IDs; and parent-receipt ID. All decimal `u128`, digest,
binary64-bit, byte, array-order, cardinality, and canonical-JSON rules import
`SC-COUPLEDTIME-001@2`. The parent receipt is reconstructed from this exact
field sequence. Atomic commit consumes this candidate and the live clock; it
does not accept caller-supplied IDs, owner sets, a boolean commit flag, or an
independent publication transition.

`OPENWEPP_C3_WOODY_V11_RESTART_V1` retains the parent beginning set, current
staged complete owner set, accepted receipt chronology, and scheduled/material
state. Restore reauthenticates the hierarchy and returns only a continuation
capability; rejected attempts and live-owner partial installs are impossible.

The restart wire also retains checkpoint phase; parent interval/current and
next sequence; coupled-time cursor, segment/event ordinal and active
participants; exact staged V11/water/BGC/energy/thermal owner bytes and digests;
accepted slab/event/scheduled/resource/material receipts; ordered reduction
operands and values; pending publication records; durable outbox state; and
controller/authority/configuration identities. Fresh restore reconstructs the
merged slab/event predecessor chain, resource folds, scheduled execution keys,
reductions, parent/publication receipts, and outbox state before releasing a
continuation. DirectV10 restart V1 and coupled-time restart V2 remain embedded
or referenced byte-identically, never extended in place.

Admission must execute `OPENWEPP_C3_WOODY_V11_SEMANTIC_VALIDATOR_V1` as bound by
`SC-VEGETATION-001`; schema validation or caller-claimed hashes alone are never
sufficient. `OPENWEPP_C3_WOODY_V11_PARENT_CANDIDATE_V1` is the closed candidate
schema and its authenticated parent receipt is the sole consuming commit
capability.

Version 5 supersedes that unimplemented V1 wire with
`OPENWEPP_C3_WOODY_V11_RESTART_V2` for production restoration. V2 binds the
complete typed vegetation parent checkpoint and separate beginning/staged
seven-owner bytes, plus coupled-time V2, event/scheduled lineage, reductions,
publication and outbox state. V1 remains immutable evidence and is never an
accepted production checkpoint. All duplicated cursor, parent, participant,
owner and receipt facts must join bit-for-bit before a continuation exists.
The seven owner payloads use seven closed owner-specific canonical schemas;
suffix equivalence covers every owner byte and all receipt/resource/material/
event/reduction/publication/outbox state. Event source, receiver, prior/next
participants, custody amount, tick, and ordinal are reconstructed rather than
trusted. All persisted collections are exact-cardinality, canonically ordered,
and identity-unique. Publication and outbox IDs are independently derived, each
record has exactly one durable row when buffered for delivery, and the outbox
row binds parent, record, state, and delivery count. Current/next parent
sequences also join the checkpoint's last accepted sequence, so a coordinated
sequence reframe remains invalid.
Each accepted segment beginning digest equals the prior ending-state digest,
with ordinal zero rooted in the parent beginning state, and its support begins
at the predecessor end. The terminal segment ending complete-owner envelopes
equal checkpoint and outer staged seven-owner envelopes byte-for-byte and
digest-for-digest.

| Failure ID | Typed failure |
|---|---|
| `VEGTXN-E-011` | Wrong parent/segment/slab/participant/support/resource identity rejects before staging. |
| `VEGTXN-E-012` | Cross-segment predecessor, inventory, overbooking, or cumulative-ledger mismatch rejects atomically. |
| `VEGTXN-E-013` | Per-segment commit, duplicate finalization/increment, incomplete owner set, or stale live clock rejects. |
| `VEGTXN-E-014` | Restart hierarchy, receipt, scheduled-once, material, or replay mismatch rejects continuation. |

Version 4 is default-off and adds no snow-carrier, constitutive, selector,
publication, deployment, or production-cutover authority.

## Change Log

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-20 | 4 | Codex | Drafted V11 accepted-segment staging, cumulative resource/material custody, additive restart, and one atomic parent commit. |
| 2026-08-20 | 5 | Codex | Added production V11 restart V2 complete typed checkpoint/owner custody after implementation inventory proved reviewed V1 insufficient. |
| 2026-08-20 | 6 | Codex | Required owner-specific V2 state admission, full seven-owner suffix equality, authenticated event custody, canonical collections, and reconstructed durable outbox identity. |
| 2026-08-20 | 7 | Codex | Added exact accepted-segment predecessor chaining and terminal complete-owner equality across restart V2 layers. |
| 2026-08-20 | 8 | Codex | Separated authoritative sequential resource-owner subtraction from the nonassociative ordered cumulative diagnostic fold and rejected regrouped ending aliases. |
| 2026-08-20 | 9 | Codex | Split occupancy-scoped vegetation debit receipts from typed shared-owner transitions and required exact debit links, owner-candidate lineage, authorization, and shared predecessor continuity. |
| 2026-08-19 | 3 | Codex | Added default-off terminal receiver all-owner transaction authority (`INV-VEGTRANSACTION-008`) with phase-aware error precedence, exact rollback, restart membership, and CoE production invariance. |
| 2026-08-14 | 2 | Codex | Extended the transaction to V8/LSE source-keyed ground water, one real-hydrology authorization, coupled final solve, LSE/soil-thermal owner joins and production-isolated atomic shadow commit. |
| 2026-08-12 | 1 | Codex | Initial shared V2 occupancy water/energy owner identity, reconstruction, and atomicity authority. |
