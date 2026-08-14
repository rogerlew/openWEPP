---
contract_id: SC-VEGETATIONTRANSACTION-001
title: Coupled Vegetation Occupancy Owner-Transaction Contract
status: approved
maturity: active
owner: openWEPP maintainers + vegetation/hydrology/energy reviewer
contract_version: 2
producer_scope:
  - OPENWEPP_C3_WOODY_V8 occupancy and ground resource/energy candidates
consumer_scope:
  - Default-off real-hydrology, LSE, BGC, and soil-thermal shadow owners
evidence_level: static+independent_oracle
last_reviewed: 2026-08-14
supersedes: []
superseded_by: []
---

# SC-VEGETATIONTRANSACTION-001 Coupled Vegetation Occupancy Owner-Transaction Contract

Status: `approved`
Maturity: `active`
Evidence mode: `Static`

## Purpose

Define shared typed identity and independent receiving-owner obligations for the
default-off V8/LSE real-hydrology shadow transaction without activating
production selectors.

## Scientific Scope and Explicit Out-of-Scope Boundaries

In scope are occupancy water arbitration, component-energy operands, owner
candidate validation, independent reconstruction, and atomic commit. Production
cutover, canopy snow, and soil transformations are excluded.

## Authority Anchors with Top-Down Citations

| Anchor | Authority | Use | Evidence |
|---|---|---|---|
| `REF-VEGTRANSACTION-001` | `SC-VEGETATION-001#INV-VEGETATION-010..015,073..079` | producer state/area/transaction semantics | `[DIRECT][Static]` |
| `REF-VEGTRANSACTION-002` | `SC-WATBAL-001#INV-WATBAL-101` | hydrology sole mutation and same-layer arbitration | `[DIRECT][Static]` |
| `REF-VEGTRANSACTION-003` | `SC-LANDSURFACEENERGY-001#INV-LANDSURFACEENERGY-010/012/042/043` | independent energy and latent identity | `[DIRECT][Static]` |
| `REF-VEGTRANSACTION-004` | physical conservation/dimensional identity | exact-one debit/conversion | `[INFERENCE][Static]` |

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

## Producer Obligations and Consumer Obligations

- `OBL-VEGTRANSACTION-P-001`: vegetation emits typed occupancy requests and
  immutable energy operands, never owner residuals.
- `OBL-VEGTRANSACTION-C-001`: hydrology and energy independently validate their
  identities, bases, candidates, and receipts.
- `OBL-VEGTRANSACTION-C-002`: no vegetation-only commit API is closure eligible.

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `D_W/A_W/F_W,o,l` | future occupancy resource DTO | diagnostic | stand-ground interval mass | this / SC-WATBAL-001 |
| `Q_o,k` | future energy operands | diagnostic | tile-ground interval energy | this / SC-LANDSURFACEENERGY-001 |

## Constants and Parameters with Provenance Anchors

No empirical constant is introduced. `f_t` is caller topology; enthalpy and all
constitutive values retain V2/adjacent-owner authority.

## Unit-Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| occupancy water | `kg H2O m^-2 stand-ground` | future typed DTO | one-time `f_t` conversion | none | none |
| occupancy energy | `J m^-2 tile-ground` | future typed DTO | weight after local closure | none | none |

## Tolerance and Numeric Notes

Identity and conversion counts are exact. No tolerance repairs an identity,
unit, basis, duplicate, or wrong-owner defect.

## Calibration and Identifiability

`CALIBRATION_NOT_APPLICABLE`: this contract governs ownership and transactions.

## Test-Vector Obligations

Require distinct occupancy/layer requests; duplicate/swap; zero demand/supply;
oversubscription; unused authorization; wrong/double/missing `f_t`;
authorization-as-use; component omission/duplication; tile/ground and
rate/amount aliases; producer-residual poison; and phase-injection rollback.

## Binding Exposure Index

No earlier sidecar exists; all binding authority is in this contract.

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

## Change Log

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-14 | 2 | Codex | Extended the transaction to V8/LSE source-keyed ground water, one real-hydrology authorization, coupled final solve, LSE/soil-thermal owner joins and production-isolated atomic shadow commit. |
| 2026-08-12 | 1 | Codex | Initial shared V2 occupancy water/energy owner identity, reconstruction, and atomicity authority. |
