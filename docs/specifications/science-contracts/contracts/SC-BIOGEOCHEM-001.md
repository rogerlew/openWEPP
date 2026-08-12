---
contract_id: SC-BIOGEOCHEM-001
title: Vegetation Biogeochemistry Exchange and Receiving-State Contract
status: approved
maturity: active
owner: openWEPP maintainers + forest biogeochemistry reviewer
contract_version: 1
producer_scope:
  - Layer mineral-nitrogen arbitration and litter/CWD receiving state
consumer_scope:
  - SC-VEGETATION-001 and future soil-biogeochemistry implementation
evidence_level: static
last_reviewed: 2026-08-11
supersedes: []
superseded_by: []
---

# SC-BIOGEOCHEM-001 Vegetation Biogeochemistry Exchange and Receiving-State Contract

Status: `approved`
Maturity: `active`
Evidence mode: `Static`

## Purpose

Own the minimum complete receiving boundary needed by
`OPENWEPP_C3_WOODY_V1`: layer mineral-N requests and receipts, litter and
coarse-woody-debris C/N/dry-material receipts, exact ownership, and atomic
transactions. This contract authorizes an implementation boundary, not a
temporary nutrient source.

## Scientific Scope and Explicit Out-of-Scope Boundaries

In scope are persistent layer `NH4-N` and `NO3-N`; proportional same-snapshot
mineral-N arbitration; litter metabolic/cellulose/lignin and CWD receiving
pools; and exact N/C/dry-matter closure. Decomposition, immobilization,
mineralization, nitrification, denitrification, leaching, gaseous loss and
external deposition equations are an explicitly named successor dependency.
Until admitted, those processes are zero-change—not hidden source/sink—and a
simulation requesting them receives `BGC-E-040`. Vegetation never mutates the
receiving state. No default initial inventory, C:N value, partition, or rate is
authorized.

## Authority Anchors with Top-Down Citations

| Anchor ID | Authority | Use | Evidence |
|---|---|---|---|
| `REF-BGC-001` | CLM5 Technical Note, Chapters 18--21, exact reviewed SHA-256 in bibliography | C/N demand, storage/transfer and receiving-pool reference architecture | `[DIRECT][Static]` `REFERENCE_MODEL_DEFINITION` |
| `REF-BGC-002` | BIOME-BGC v4.2 theoretical framework, exact reviewed SHA-256 in bibliography | litter/CWD pool and elemental architecture | `[DIRECT][Static]` `REFERENCE_MODEL_DEFINITION` |
| `REF-BGC-003` | elemental and dry-material conservation | exact donor/receiver and no-source constraints | `[INFERENCE][Static]` `PHYSICAL_OR_DIMENSIONAL_INVARIANT` |
| `REF-BGC-004` | `SC-VEGETATION-001` | transaction and vegetation donor/request boundary | `[DIRECT][Static]` |

## Variables and Units Using Canonical Symbols First

| Symbol | Units | Meaning / basis | Owner |
|---|---|---|---|
| `Nmin_l,q` | `kg N m^-2` | beginning/end mineral N in layer `l`, species `q in {NH4,NO3}`, ground area | biogeochemistry |
| `D_N,s,l,q` | `kg N m^-2 interval` | vegetation request | vegetation |
| `A_N,s,l,q` | `kg N m^-2 interval` | maximum authorized uptake | biogeochemistry |
| `F_N,s,l,q` | `kg N m^-2 interval` | vegetation-finalized use | vegetation, validated by biogeochemistry |
| `D_X,x,l,q`, `A_X,x,l,q`, `F_X,x,l,q` | `kg N m^-2 interval` | competing-owner request, maximum authorization, and finalized use | competing owner / biogeochemistry validation |
| `L_C,c`, `L_N,c` | `kg C m^-2`, `kg N m^-2` | litter/CWD proposal by class | vegetation |
| `L_DM,c` | `kg dry matter m^-2` | same material proposal, not alias of C | vegetation |
| `B_C,c`, `B_N,c`, `B_DM,c` | corresponding amount units | persistent receiving pool | biogeochemistry |

All are transaction-area interval amounts. `interval^-1` is not a unit.

## Algorithm State Surfaces

Inputs are immutable transaction identity, beginning mineral and receiving
pools, all same-transaction vegetation and competing requests, and immutable
material proposals. Outputs are one receipt/reason per request, candidate
mineral and receiving states, and reconstruction ledgers. Only the
biogeochemistry owner may mutate these candidates; only the orchestrator may
commit them.

## Algorithm Specification with Step Sequence

1. Validate common transaction, area, layer/species IDs and nonnegative finite
   pools/requests. Reject unsupported nitrogen species or receiver class.
2. For each `(l,q)`, let
   `R=sum_s D_N,s,l,q + sum_x D_X,x,l,q`. If `R<=Nmin`, every authorization
   equals its request. Otherwise every vegetation and competing authorization
   is its own request multiplied by the common factor `Nmin/R`. Exact zero
   aggregate demand returns exact zero. No priority, floor, borrowing, or
   cross-layer transfer occurs. Authorization mutates nothing.
3. Vegetation returns finalized use after its carbon/water solve. Validate
   `0<=F_N<=A_N<=D_N`; every competitor likewise returns and validates
   `0<=F_X<=A_X<=D_X`. Form
   `Nmin'=Nmin-sum_s F_N-sum_x F_X`. Unused authorization is never a
   withdrawal. Reject a negative result beyond arithmetic tolerance.
4. Route donor material before receipt. Leaf and fine-root turnover partitions
   each donor's C, N and dry matter among `metabolic`, `cellulose`, and `lignin`
   using that donor's three required caller fractions, which are nonnegative and
   sum exactly one within representation tolerance. Stem/coarse-root mortality
   and standing-dead fall route to `CWD`; livewood turnover first transfers
   internally to the matching deadwood vegetation pool and creates no receiver
   proposal. No other donor/class mapping exists in v1.
5. Validate each litter/CWD proposal after vegetation finalization. Credit
   exactly one receiver class by the same C, N and dry-matter operands that the
   vegetation donor debits. No receiver derives one operand from another.
6. Both owners reconstruct N and material ledgers; atomically commit all or
   none.

## Branch and Guard Table

| Condition | Disposition | Failure |
|---|---|---|
| zero inventory/demand | valid zero receipt | none |
| supply sufficient | full receipt | none |
| competing demand | proportional receipt | none |
| negative/nonfinite/mismatched input | reject before candidate state | `BGC-E-001` |
| unsupported N species/receiver | reject | `BGC-E-002` |
| overdraw or closure mismatch | atomic rejection | `BGC-E-010` |
| decomposition/mineralization requested | explicit unsupported dependency | `BGC-E-040` |

## Invariants and Invariant Guard Map

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| `INV-BIOGEOCHEM-001` | Every finalized mineral-N use is bounded by authorization and request; only finalized use debits same-snapshot inventory and aggregate withdrawal cannot exceed inventory. | `REF-BGC-003` | `[INFERENCE][Static]` | runtime/test | `BGC-E-010` |
| `INV-BIOGEOCHEM-002` | Same-layer oversubscription uses the stated proportional equation without hidden source or priority. | `REF-BGC-001/003` | `[DIRECT+INFERENCE][Static]` | runtime/test | `BGC-E-010` |
| `INV-BIOGEOCHEM-003` | Every accepted litter/CWD transfer has exactly one vegetation debit and one receiver credit for distinct C, N and dry matter. | `REF-BGC-002/003/004` | `[DIRECT+INFERENCE][Static]` | dual reconstruction | `BGC-E-010` |
| `INV-BIOGEOCHEM-004` | Failure or nonconvergence preserves all owner states byte-identically. | `REF-BGC-003/004` | `[INFERENCE][Static]` | atomicity test | `BGC-E-010` |
| `INV-BIOGEOCHEM-005` | Unimplemented soil transformations cannot supply or remove N. | `REF-BGC-003` | `[INFERENCE][Static]` | typed branch | `BGC-E-040` |

### Invariant Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-BIOGEOCHEM-001` | inventory/request poison vectors | test/runtime | typed rollback | coupled package vector ledger |
| `INV-BIOGEOCHEM-002` | competition reconstruction | test/runtime | typed rollback | coupled package oracle |
| `INV-BIOGEOCHEM-003` | independent donor/receiver ledger | test/runtime | typed rollback | coupled package oracle |
| `INV-BIOGEOCHEM-004` | serialized pre/post failure comparison | test | blocked promotion | successor package |
| `INV-BIOGEOCHEM-005` | unsupported-process enum | runtime/test | `BGC-E-040` | successor package |

## Producer Obligations and Consumer Obligations

- `OBL-BIOGEOCHEM-P-001`: vegetation emits immutable typed requests/proposals.
- `OBL-BIOGEOCHEM-P-002`: biogeochemistry returns complete receipts and sole-owner candidates.
- `OBL-BIOGEOCHEM-C-001`: vegetation finalizes no more than authorized N and
  returns unused authorization without debit.
- `OBL-BIOGEOCHEM-C-003`: every competing owner obeys the same request,
  maximum-authorization, finalized-use, validation, and atomic-commit protocol.
- `OBL-BIOGEOCHEM-C-002`: orchestrator commits all cross-owner states atomically.

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `D_N,s,l,q` | future `MineralNRequest.amount` | transaction | `kg N m^-2 interval` exact | this contract |
| `A_N,s,l,q` | future `MineralNAuthorization.maximum` | transaction | same amount/basis | this contract |
| `F_N,s,l,q` | future `MineralNFinalizedUse.amount` | transaction | same amount/basis | this contract |
| `L_C/L_N/L_DM` | future `MaterialTransfer.{carbon,nitrogen,dry_matter}` | transaction | three distinct typed amounts | this / `SC-VEGETATION-001` |

## Constants and Parameters with Provenance Anchors

The proportional arbitration has no empirical constant. Receiver class,
initial inventory, competing demand, and material composition are caller/state
values. Tissue-specific litter chemistry fractions are caller parameters with
three named fields whose sum is one. No decomposition parameter is admitted.

The named successor dependency is
`20260811-soil-biogeochemistry-transformations-implementation-001`, owned by
the soil-biogeochemistry subsystem. It must admit persistent litter/CWD and
mineral pools, moisture/temperature forcing, decomposition, immobilization,
mineralization, nitrification, denitrification, leaching and atmospheric
inputs/outputs; execute after accepted vegetation receipts and before the next
interval's mineral snapshot; and pass C/N closure, no-source, competition,
restart and atomicity gates. Until that package is separately authorized and
completed, `soil_transformations=disabled` is a required model-version field,
all transformation deltas are exactly zero, and requesting `enabled` returns
`BGC-E-040`.

## Unit-Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| N pools/requests | `kg N m^-2` | future BGC registry | none | prohibited | none |
| C pools/transfers | `kg C m^-2` | future BGC registry | none | prohibited | none |
| dry material | `kg dry matter m^-2` | future BGC registry | none | prohibited | none |

## Tolerance and Numeric Notes

Conservation uses `1e-14 kg m^-2 + 64*epsilon*operand_sum`; the absolute term
is a model-version representation tolerance, never a source/sink. Proportional
shares are computed in deterministic stratum-ID order with compensated sums.

## Calibration and Identifiability

`science_implementation_status = NOT_IMPLEMENTED`;
`calibration_evidence_status = NOT_CALIBRATION_READY`;
`identifiability_status = NOT_ASSESSED`. Arbitration has no calibratable
parameter. Soil transformation parameters are outside v1 and cannot be inferred
through this boundary.

## Test-Vector Obligations

Zero, full-supply, two-demand competition, dry layer, NH4/NO3 separation,
wrong-layer alias, C/N/DM poison, missing receiver, double receipt, and
byte-identical rollback vectors are mandatory.

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `BEI-BIOGEOCHEM-001` | coupled C3 authority package | `active` | `maps-to-existing-INV` | `INV-BIOGEOCHEM-001, INV-BIOGEOCHEM-002, INV-BIOGEOCHEM-003, INV-BIOGEOCHEM-004, INV-BIOGEOCHEM-005` | `flagged-binding-addition` | Package artifacts remain evidence. |

## Gap Register and Promotability Labels

| Gap ID | Gap | Required closure | Label |
|---|---|---|---|
| `GAP-BIOGEOCHEM-001` | no runtime owner exists | implement typed state, requests, receipts and atomic tests | `IMPLEMENTATION_MISSING` |
| `GAP-BIOGEOCHEM-002` | soil transformations excluded | new contract/version with exact equations and authority | `EXPLICIT_DEPENDENCY` |

## Change Log

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-11 | 1 | Codex | Initial vegetation mineral-N and material-receiving boundary. |
