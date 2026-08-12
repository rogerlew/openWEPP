---
contract_id: SC-VEGETATIONTRANSACTION-001
title: Coupled Vegetation Occupancy Owner-Transaction Contract
status: approved
maturity: active
owner: openWEPP maintainers + vegetation/hydrology/energy reviewer
contract_version: 1
producer_scope:
  - OPENWEPP_C3_WOODY_V2 occupancy resource and energy candidates
consumer_scope:
  - Diagnostic hydrology and energy owners in atomic vegetation transactions
evidence_level: static
last_reviewed: 2026-08-12
supersedes: []
superseded_by: []
---

# SC-VEGETATIONTRANSACTION-001 Coupled Vegetation Occupancy Owner-Transaction Contract

Status: `approved`
Maturity: `active`
Evidence mode: `Static`

## Purpose

Define shared typed identity and independent receiving-owner obligations for the
default-off V2 diagnostic transaction without activating production selectors.

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
| `f_t` | fraction | positive stand-area fraction |
| `D_W,o,l`, `A_W,o,l`, `F_W,o,l` | `kg H2O m^-2 stand-ground` | request, maximum authorization, final use |
| `Q_o,k` | `J m^-2 tile-ground` | immutable occupancy energy component |

## Algorithm State Surfaces

Inputs are typed vegetation requests/component operands and immutable beginning
water/energy snapshots. Hydrology alone forms soil candidate debits; energy
alone forms its candidate ledger; the orchestrator commits the complete owner set.

## Algorithm Specification with Step Sequence

1. Reject duplicate/mismatched `(tau,s,t,l,resource,amount_basis)` identities
   before sorting or summation.
2. Hydrology groups all occupancy and competitor requests by exact layer on the
   stand-ground basis and returns full or equal-status proportional maximum
   authorization per exact request.
3. Vegetation alone divides authorization by positive `f_t` for its local capped
   solve and multiplies local finalized use by the same `f_t` on return.
4. Hydrology validates `0<=F<=A<=D`, exact identity, aggregate layer debit, and
   debits finalized use only. Unused authorization remains in inventory.
5. Energy independently constructs each occupancy candidate from immutable
   direct/diffuse VIS/NIR, leaf/stem shortwave, incident/emitted longwave,
   sensible, finalized transpiration, wet evaporation/condensation, dry-stem,
   ground, storage/conductive, interval, and authority-tagged latent operands.
   It never consumes a producer residual or copied candidate.
6. Each occupancy closes on tile-ground basis; energy then weights accepted
   components once by `f_t` and independently reconstructs the stand ledger.
7. Validate vegetation, water, BGC, and energy candidates before one atomic
   commit. Any failure preserves every beginning byte.

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

## Invariants and Invariant Guard Map

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| `INV-VEGTRANSACTION-001` | Water identity and stand basis persist through request, authorization, final use, debit, and receipt. | REF-001/002/004 | `[INFERENCE][Static]` | dual owner | `VEGTXN-E-001..004` |
| `INV-VEGTRANSACTION-002` | Hydrology arbitrates one same-layer snapshot and debits final use only. | REF-002/004 | `[DIRECT+INFERENCE][Static]` | hydrology | `VEGTXN-E-003` |
| `INV-VEGTRANSACTION-003` | Energy reconstructs each occupancy and weighted stand result independently. | REF-001/003/004 | `[DIRECT+INFERENCE][Static]` | energy | `VEGTXN-E-004..006` |
| `INV-VEGTRANSACTION-004` | Every owner commits together or all beginning bytes remain identical. | REF-001/004 | `[INFERENCE][Static]` | orchestrator | `VEGTXN-E-007` |

### Invariant Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-VEGTRANSACTION-001` | typed DTO/owner poisons | runtime/test | reject | V2 implementation package |
| `INV-VEGTRANSACTION-002` | diagnostic water owner | runtime/test | reject | V2 implementation package |
| `INV-VEGTRANSACTION-003` | independent energy owner | runtime/test | reject | V2 implementation package |
| `INV-VEGTRANSACTION-004` | all-owner commit/injection | runtime/test | rollback | V2 implementation package |

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

## Change Log

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-12 | 1 | Codex | Initial shared V2 occupancy water/energy owner identity, reconstruction, and atomicity authority. |
