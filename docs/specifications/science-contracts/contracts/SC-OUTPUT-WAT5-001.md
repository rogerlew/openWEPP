---
contract_id: SC-OUTPUT-WAT5-001
title: Five-Minute Hillslope Water Diagnostic Output Contract
status: approved
maturity: active
owner: openWEPP maintainers + hydrology/output reviewer
contract_version: 5
producer_scope:
  - Optional hillslope_wat_subhourly Parquet dataset
  - Diagnostic projection of WB14 generation and WB19 saturation return
consumer_scope:
  - End-user subhourly water diagnostics
  - Noninterference and closure verification
evidence_level: static
last_reviewed: 2026-09-02
supersedes: []
superseded_by: []
---

# SC-OUTPUT-WAT5-001 Five-Minute Hillslope Water Diagnostic Output Contract

Status: `approved`
Maturity: `active`
Evidence mode: `Static`

## Purpose

Define the optional version-5 five-minute hillslope water diagnostic dataset
without changing daily water balance, the authoritative 24-hour runoff ledger,
WB16/public peak, erosion execution, HBP, inter-OFE transfer, or routing.

## Scientific Scope and Exclusions

In scope: source-complete WB14 Green-Ampt diagnostic replay on 300-second bins
for callers without an accepted partition ledger, and direct projection of the
sealed Stage-3 WB14 disposition receipts when that authoritative ledger exists,
using local-rain intervals plus exact typed accepted non-rain supply segments,
per-hour closure to authoritative WB14 generation, labeled hourly-resolution
WB19 saturation-return composition, sparse event publication, units, keys,
metadata, and typed failures.

Out of scope: instantaneous or routed discharge, five-minute erosion solves,
erosion forcing adoption, rainfall `effint`, HBP fields/versioning, channel or
watershed routing, and invented timing for runon, routed melt, frost retention,
or any daily/hourly-only or aggregate-only source lacking exact accepted
segment timing and receipt identity.

## Authority Anchors

| Anchor ID | Source | Use | Evidence |
|---|---|---|---|
| `REF-WAT5-WATBAL` | Existing `SC-WATBAL-001#INV-WATBAL-102..104` producer surfaces | Names the unchanged hourly ledger, source-custody, and unit/publication interfaces consumed by this contract; diagnostic replay, closure, and noninterference authority remain local to `SC-OUTPUT-WAT5-001`. | `[DIRECT][Static]` |
| `REF-WAT5-RUNOFFPART` | `SC-RUNOFFPART-001` WB14 Green-Ampt and depression-storage authority | The diagnostic reuses unchanged equations and state order; it does not define new infiltration physics. | `[DIRECT][Static]` |
| `REF-WAT5-OUTPUT-PHYS` | Conservation and dimensional invariants | Depth sums close across time partitions; intensity is depth divided by 300 seconds; omitted support is exact zero. | `[INFERENCE][Static]` |
| `REF-WAT5-PACKAGE` | `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/package.md` | Authorized product boundary, required fields, sparse semantics, protected surfaces, and erosion `NO_ADOPTION`. | `[DIRECT][Static]` |

## Variables and Units

| Symbol | Units | Meaning |
|---|---|---|
| `k` | index `0..287` | Five-minute bin within one simulation day. |
| `h` | index `0..23` | Containing authoritative hour, `floor(k/12)`. |
| `Δt5` | `s` | Exact interval duration, `300`. |
| `R5(k)` | `mm` | Local rainfall depth overlapping bin `k`. |
| `A5(k)` | `mm` | Exact overlap-integrated non-rain supply from typed `SnowTerminalReceiver`, `RoutedRunon`, `LitterPhaseOverflow`, and `CondensationOverflow` accepted segments. |
| `F5(k)` | `mm` | Raw isolated Green-Ampt infiltration increment. |
| `D5(k)` | `mm` | Raw post-infiltration excess retained into depression storage, allocated to the earliest generated bins in WB14 order. |
| `G5raw(k)` | `mm` | Raw WB14 post-depression generation depth. |
| `C5(k)` | `mm` | Bounded closing partition-ledger reconciliation; normally zero and positive only on the canonical latest positive-source piece under `INV-WAT5-011`. |
| `G5closed(k)` | `mm` | Raw shape closed within hour to authoritative WB14 generation, plus the explicitly reconstructible bounded `C5` operand when admitted. |
| `S5(k)` | `mm` | One twelfth of authoritative WB19 saturation return in hour `h`. |
| `Q5(k)` | `mm` | `G5closed(k) + S5(k)` diagnostic closing surface generation. |
| `I5(k)` | `mm h^-1` | `Q5(k) / (300 s / 3600 s)`. |
| `Qh` | `mm` | Authoritative closing surface-runoff depth for hour `h`. |
| `Ih` | `mm h^-1` | Authoritative hourly mean, numerically `Qh` per one-hour bin. |

## Algorithm State Surfaces

Required inputs are local hyetograph intervals, exact typed accepted additional-
supply segments, unchanged WB14 parameter/state inputs, authoritative WB14
hourly generation, authoritative WB19 hourly saturation return, daily/lane
identity, and explicit output request state.

Output is zero or one sparse event span per day/OFE, containing typed rows. The
algorithm mutates no water, erosion, transfer, routing, or persistent state.

## Algorithm Specification

1. If output is not requested, do not open a writer and impose no publication
   requirement on otherwise valid runs.
2. If requested, validate all dimensional inputs as finite and nonnegative.
   Admit additional supply only through `Wat5AdditionalSupplySegmentV1`, with
   one exact `Wat5AdditionalSupplySourceKindV1`, accepted source receipt,
   transaction, destination OFE, half-open day support `[start_s,end_s)`, and
   OFE-ground depth. Reject positive aggregate-only or unknown supply.
3. Form the ordered union of local-rain boundaries, additional-segment
   boundaries, and exact multiples of `300 s`. On each elementary support,
   keep local-rain rate and every additional-source rate distinct, set total
   WB14 supply rate to their checked sum, and advance the unchanged WB14
   Green-Ampt equations once in chronological order with continuous
   cumulative infiltration; allocate depression-storage removal from earliest
   raw generated bins, matching WB14 ordering, and retain that removed depth as
   `D5(k)`. Integrate rain to `R5(k)` and non-rain supply to `A5(k)` without
   relabeling. Validate `sum(R5+A5) = sum(F5+D5+G5raw)` independently from the
   same row operands exposed to consumers. Under `INV-WAT5-012`, the accepted
   Stage-3 path instead bins the sealed infiltration, retained-surface, and
   routed/outlet-runoff dispositions from that same source partition directly;
   it must not execute a second day-wide Green-Ampt solve.
4. For each hour, let `R_h = Σ G5raw(k)` and `B_h` be authoritative WB14
   generation. If `B_h = 0`, set all twelve `G5closed` values to zero. If
   `B_h > 0` and `R_h > 0`, set
   `G5closed(k) = G5raw(k) * B_h / R_h` and `C5(k)=0`. If `B_h > 0` and
   `R_h = 0`, apply only the bounded source-supported rule in
   `INV-WAT5-011`; otherwise fail. Validate closure without changing `B_h`.
5. Set `S5(k) = S_h / 12`, explicitly recording the source as
   `hourly_zero_order_hold`. Set `Q5 = G5closed + S5` and validate each hour
   closes to `Qh = B_h + S_h`, then validate the day closes to the unchanged
   authoritative 24-bin total.
6. Find the first and last bin where rainfall, raw/closed generation, or
   saturation return is positive. Emit that inclusive span with
   `event_ordinal = 0`; emit no rows for a fully dry day; omitted leading and trailing bins are exact zero
   by contract. Do not infer multiple storms from
   gaps.
7. Convert `m` runtime depths to `mm` with the named unit helper and derive
   `mm h^-1` intensity with the named 300-second depth-to-rate helper.
8. The erosion power-equivalent columns are nullable in version 2. Under the
   frozen erosion `NO_ADOPTION`, publish null exponent/rate/duration and method
   `water_only_no_erosion_adoption`; never fabricate candidate values.
9. Stage every requested run output in its target directory. Close and validate
   every staged writer before publication. Publish the complete output set with
   rollback protection for preexisting targets and use the manifest as the last
   completion marker. Any construction, simulation, close, schema,
   checksum, link, or manifest failure preserves the complete pre-run output set
   and removes incomplete staging files. WAT5 is always no-replace: a target
   that appears during simulation is preserved and aborts the whole commit.

## Branch and Guard Table

| Trigger | Guard | Failure behavior |
|---|---|---|
| Output not requested | `WAT5-B-001` | No writer; production behavior unchanged. |
| Fully dry day | `WAT5-B-002` | Valid zero-row day. |
| Positive additional supply without exact typed accepted segment timing/receipt | `WAT5-E-001` | Typed source-incomplete output failure; no uniform/rainfall-shaped fallback. |
| Positive authoritative WB14 hour with zero raw generation and no admissible bounded source-supported reconciliation | `WAT5-E-002` | Typed closure/source failure; no redistribution or unsupported placement. |
| Non-finite, negative, invalid time order, or invalid key | `WAT5-E-003` | Typed domain failure before publication. |
| Hour/day closure exceeds tolerance | `WAT5-E-004` | Typed closure failure. |
| Writer/link/publication fails or metadata is incomplete | `WAT5-E-005` | Typed publication failure; preserve the complete pre-run output set and publish no partial replacement set. |

## Invariants and Guard Map

| Invariant ID | Statement | Authority | Guard | Failure posture | Evidence |
|---|---|---|---|---|---|
| `INV-WAT5-001` | Dataset version `2.0` is optional and separate from WAT, PASS, and HBP; run-file `outputs.wat_subhourly` presence is the sole user-facing opt-in; diagnostics-off preserves protected outputs and state byte-for-byte. | `REF-WAT5-WATBAL` | run-file output config plus noninterference tests | hard-fail/HOLD | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WAT5-002` | Every row uses exact 300-second support, globally ordered and calendar-consistent day/OFE keys, `event_ordinal=0`, `hour_index=floor(subinterval_index/12)`, and an adjacent first-through-last event span with no gap-based event invention or completed-hour re-entry. | `REF-WAT5-OUTPUT-PHYS` | `WAT5-E-003` | typed error | `[INFERENCE][Static]` |
| `INV-WAT5-003` | Raw diagnostic replay uses unchanged WB14 equations and continuous infiltration; closed bins reconcile each hour to authoritative WB14 without changing it. | `REF-WAT5-RUNOFFPART`, `REF-WAT5-WATBAL` | `WAT5-E-002..004` | typed error | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WAT5-004` | Hourly WB19 return is composed only as a labeled twelve-bin zero-order hold, and every hour/day closes to the unchanged 24-bin ledger. | `REF-WAT5-WATBAL` | `WAT5-E-004` | typed error | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WAT5-005` | Positive additional supply without exact producer segment timing and accepted receipt identity is rejected; no fallback may invent timing. | `REF-WAT5-WATBAL` | `WAT5-E-001` | typed error | `[DIRECT][Static]` |
| `INV-WAT5-006` | Sparse output includes first-through-last active bins and metadata declares omitted bins exact zero; a dry day emits no rows. | `REF-WAT5-OUTPUT-PHYS` | writer validation | typed error | `[INFERENCE][Static]` |
| `INV-WAT5-007` | The dataset never claims discharge, peak, routed flow, or erosion adoption; power-equivalent fields are null and method/source codes are exact under `NO_ADOPTION`. | `REF-WAT5-PACKAGE`, `REF-WAT5-WATBAL` | writer-boundary schema/method tests | hard-fail/HOLD | `[DIRECT][Static]` |
| `INV-WAT5-008` | Emitted raw rows expose rainfall, infiltration, depression-storage retention, and post-depression generation so consumers independently reconstruct raw closure. | `REF-WAT5-RUNOFFPART`, `REF-WAT5-OUTPUT-PHYS` | positive-storage Parquet reconstruction | typed error/HOLD | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WAT5-009` | WAT5-enabled run publication is all-or-nothing across every requested output and the manifest; any pre-completion failure preserves preexisting bytes, and the manifest is published last. | `REF-WAT5-PACKAGE` | transactional staging, rollback, and injected-failure tests | typed error/HOLD | `[DIRECT][Static]` |
| `INV-WAT5-010` | Positive additional supply is admitted only as exact accepted typed segments with immutable receipt/source, transaction, destination, support, depth, and area-basis custody. The original closed set is `SnowTerminalReceiver`, `RoutedRunon`, and `LitterPhaseOverflow`; `INV-WAT5-012` adds receipt-complete `CondensationOverflow` and defines which sealed ingress sources retain precipitation lineage. Replay partitions the union of rain, segment, and 300-second boundaries, advances WB14 once on the checked combined rate, exposes rain and additional depth separately, and reconstructs every segment and hourly authoritative additional-supply total exactly once. Aggregate-only, relabeled, retimed, duplicated, omitted, or foreign supply rejects atomically. | `SC-WATBAL-001#INV-WATBAL-103`, `SC-SURFACELIQUID-001#INV-SURFACELIQUID-020/028`, `REF-WAT5-RUNOFFPART` | typed producer segment schema, accepted-source projection, piecewise WB14 replay, independent source/hour closure | `WAT5-E-001..004` / complete publication rollback | `[DIRECT][Static] + [INFERENCE][Static]`; production seam expected-red required |
| `INV-WAT5-011` | When exact source-supported replay yields zero raw generation but the unchanged accepted WB14 hour is finite positive, WAT5 may retain that hour only through one bounded `C5` operand equal to the exact authoritative-minus-raw closing residual. Eligibility requires complete exact positive source support in that same hour, raw supply closure, accepted hourly/daily closure under `TOL-WATBAL-009`, and `C5<=TOL-WAT5-002`. Place `C5` only on the containing 300-second bin of the latest canonical positive-source elementary piece; leave `R5/A5/F5/D5/G5raw`, source supports, and the authoritative hour unchanged. Any missing source, different placement, material residual, negative operand, or second placement hard-fails and rolls back. | `INV-WAT5-003/005/010`, `SC-WATBAL-001#INV-WATBAL-102/103`, `TOL-WATBAL-009` | typed closing-reconciliation operand, canonical piece selector, independent raw/closed closure and rollback validators | `WAT5-E-002..004` / complete rollback | `[DIRECT][Static] + [INFERENCE][Static]`; exact subnormal and boundary vectors required |
| `INV-WAT5-012` | Accepted Stage-3 WAT5 sourcing and raw partition operands are projected once from the exact sealed SurfaceLiquid ingress receipts that close to the accepted OFE ingress ledger. `RawPrecipitation`, canopy throughfall, both canopy drainage limbs, and stemflow retain precipitation lineage in `R5`; snow terminal, routed runon, litter phase overflow, and condensation overflow retain distinct typed non-rain custody in `A5`. The same receipt rows populate `F5`, `D5`, and `G5raw` by their accepted infiltration, retained-surface, and routed/outlet-runoff dispositions. Their checked source sum must reconstruct accepted ingress, their disposition sums must reconstruct the accepted ledger, and their hourly runoff bins must reconstruct the accepted WB14 owner under the existing closure tolerances. This real-consumer path must not execute a second day-wide Green-Ampt solve over already-disposed mass. The pre-ingress snow-terminal custody vector is not a second WAT5 source owner after refreeze and receiver disposition; WAT5 validates the sealed post-disposition `TerminalReceiver` receipt kind and complete ingress closure instead. Segment identity/support and receipt timing remain exact. Forcing-only projection, omission of a receipt kind, disposition-as-source duplication, second-solve substitution, or material source/ingress/disposition mismatch is `WAT5-E-001`. | `INV-WAT5-005/010`, `SC-SURFACELIQUID-001#INV-SURFACELIQUID-020/028`, `SC-WATBAL-001#INV-WATBAL-103` | sealed receipt-group and disposition projector, exact source identity/support, accepted-ingress/disposition/hour closure | `WAT5-E-001` / complete rollback | `[DIRECT][Static] + [INFERENCE][Static]`; heterogeneous and routed multi-OFE vectors required |

## Producer and Consumer Obligations

- `OBL-WAT5-P-001`: emit the required schema, metadata, units, method/source
  codes, and atomic file semantics.
- `OBL-WAT5-P-002`: reconstruct closure independently from emitted rows and
  retain exact noninterference anchors for WAT/PASS/HBP/public peak/state.
- `OBL-WAT5-P-003`: stream rows; do not retain the full-run dataset in memory.
- `OBL-WAT5-P-004`: validate public row invariants at the writer boundary so a
  caller cannot serialize non-finite, negative, chronologically inconsistent,
  or non-closing records through the public writer API. Derived closure/rate
  arithmetic must remain finite, and `(year, julian)` must advance when a
  simulation day advances for the same WEPP identity.
- `OBL-WAT5-P-005`: preserve exact accepted additional-segment source kinds,
  receipts, transaction, destination, support, depth, and OFE basis through
  producer construction and replay; prove per-source, per-bin, per-hour, and
  day closure with no precipitation alias, retiming, omission, or duplicate.
- `OBL-WAT5-P-006`: prove bounded zero-raw-generation reconciliation retains
  the accepted positive hour on exactly the latest canonical positive-source
  piece, exposes an exact `C5` operand, preserves every raw/source operand,
  rejects missing/foreign support or residual above `TOL-WAT5-002`, and rolls
  back the complete output set and runtime state on failure.
- `OBL-WAT5-P-007`: prove the receipt-group source projector preserves every
  precipitation and non-rain source kind exactly once, reconstructs the
  accepted ingress ledger, projects each accepted disposition into the exact
  raw WAT5 operand/hour without a second physical solve, and rejects omitted,
  duplicated, foreign, or disposition-substituted receipts before publication.
- `OBL-WAT5-C-001`: consumers must treat all quantities as diagnostic depths
  or depth rates, never volumetric discharge or instantaneous/routed peak.
- `OBL-WAT5-C-002`: consumers must honor sparse omitted-zero metadata and the
  nullable erosion fields.

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `R5` | `rainfall_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
| `A5` | `additional_supply_depth_mm` | Parquet | named `m -> mm`; exact overlap sum of typed non-rain segments | `SC-OUTPUT-WAT5-001` |
| `F5` | `raw_green_ampt_infiltration_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
| `D5` | `depression_storage_retention_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
| `G5raw` | `raw_wb14_post_depression_generation_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
| `C5` | exactly reconstructible as `G5closed-G5raw*B_h/R_h` when `R_h>0`, otherwise `G5closed`; not a new schema column | diagnostic closing ledger | named `m -> mm`; zero except `INV-WAT5-011` | `SC-OUTPUT-WAT5-001` |
| `G5closed` | `closed_wb14_generation_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
| `S5` | `saturation_return_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
| `Q5` | `closing_surface_generation_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
| `I5` | `closing_surface_generation_intensity_mm_h` | Parquet | named 300-second depth-rate conversion | `SC-OUTPUT-WAT5-001` |
| `Qh`, `Ih` | `hourly_authoritative_runoff_depth_mm`, `hourly_mean_generation_intensity_mm_h` | Parquet | `mm` and `mm h^-1` | `SC-OUTPUT-WAT5-001` publication; unchanged hourly producer surface |

## Constants and Parameters

| Name | Value | Units | Authority |
|---|---|---|---|
| `WAT5_INTERVAL_SECONDS` | `300` | `s` | dataset resolution specification |
| `WAT5_INTERVALS_PER_HOUR` | `12` | count | exact hour partition |
| `WAT5_INTERVALS_PER_DAY` | `288` | count | exact day partition |
| `WAT5_DATASET_VERSION` | `2.0` | schema version | this contract |

No empirical or calibratable parameter is introduced.

## Unit-Governance Map

| Symbol | Declared units | Boundary registry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| depth columns | `mm` | required `hillslope_wat_subhourly.*` entries | named `m -> mm` | none | `units=mm` |
| intensity columns | `mm h^-1` | required entries | named 300-second depth-to-hourly-rate | none | `units=mm h^-1` |
| interval times | `s` | required entries | identity | none | `units=s` |
| indexes/codes | dimensionless/text | publication-only rationale | none | allowed | role metadata |

## Tolerance and Numeric Notes

`TOL-WAT5-001 = 1e-12 m * max(1, authoritative depth)` controls each
per-hour and daily diagnostic closure. It adjudicates numerical residuals
only: it cannot supply missing raw support, retime additional supply, or
authorize mutation. Runtime Green-Ampt tolerances remain owned by
`SC-RUNOFFPART-001`.

`TOL-WAT5-002 = 1e-12 m * max(1, S_h, F_h, D_h, B_h)` bounds the positive
closing partition-ledger operand in `INV-WAT5-011`, where all terms are finite
nonnegative depths for the same hour. It applies only when `B_h>0`,
`R_h=0`, exact positive source support and raw supply closure exist, and the
accepted hourly/daily ledger independently satisfies `TOL-WATBAL-009`. The
exact boundary is accepted; a larger residual hard-fails. It cannot change an
authoritative hourly bin, add supply, alter infiltration/depression/raw
generation, select a source-free bin, or repair missing custody.

## Calibration and Identifiability

`CALIBRATION_NOT_APPLICABLE`: this is a deterministic diagnostic projection
with fixed resolution and no fitted parameters.

- `science_implementation_status = IMPLEMENTED` only after real output closure.
- `calibration_evidence_status = NOT_APPLICABLE`.
- `identifiability_status = NOT_APPLICABLE`.

## Test-Vector Obligations

1. Dry, nonponding, delayed-ponding, and high-intensity ponded cases.
2. Pulse crossing five-minute and hourly boundaries.
3. Saturation-return-only and rain-plus-saturation cases.
4. Exact snow-terminal, routed-runon, and litter-phase-overflow segments alone
   and overlapping rain; aggregate-only/unknown/relabel/retime/duplicate/
   omission/source-receipt substitution typed rejection.
5. Positive authoritative hour with zero raw support typed rejection.
6. Per-hour/day closure and independent reconstruction from Parquet rows,
   including positive depression-storage retention.
7. Sparse first/last support and omitted-zero metadata.
8. Diagnostics-off and diagnostics-on identity for WAT/PASS/HBP/public peak,
   hourly runoff, erosion rows, and persistent rill state.
9. Real p61 output round trip and p102 non-adoption/source-completeness guard.
10. Unit registry/schema metadata and null erosion-candidate fields.
11. Existing-target, day-2 source, forced close/schema/link, and manifest
    failures preserve all sibling bytes and publish no partial replacement set;
    success publishes every requested output and the manifest.
12. Public writer-boundary rejection of invalid numeric, key, chronology,
    duration, and closure records.
13. Exact `2.9989032090949053e-19 m` accepted positive hour over twelve exact
    litter-overflow-supported bins whose raw replay infiltrates all supply:
    retain raw closure, place one exact `C5` on the latest canonical positive-
    source piece/bin, close the accepted hour, and reject first/uniform/source-
    free/duplicate placements and the first value above `TOL-WAT5-002`.

## Exact Accepted Additional-Segment Source Amendment

`INV-WAT5-010` closes `GAP-WAT5-001` only where the accepted Stage-3/Lane-D
path now owns exact subhourly support and receipt identity. The producer surface
is `Wat5AdditionalSupplySegmentV1`:

```text
(source_kind, source_receipt_sha256, transaction_id, destination_ofe_id,
 start_s, end_s, depth_m_ofe_ground)
```

`Wat5AdditionalSupplySourceKindV1` was initially closed to
`SnowTerminalReceiver`, `RoutedRunon`, and `LitterPhaseOverflow`; version 5
extends it with `CondensationOverflow` under `INV-WAT5-012`. Precipitation-
lineage sources are prohibited from this enum and remain in `R5`. Snow terminal
segments derive only from accepted LSE forcing parcels of that exact kind;
routed-runon segments derive only from accepted ingress receipts whose routed
source/destination identity and sent/received volume closure validate; litter-
overflow segments derive only from the exact phase-receipt-bound internal
ingress receipt. The same physical mass cannot appear on two surfaces.

Each segment has finite `0<=start_s<end_s<=86400`, finite positive depth, the
accepted transaction and destination OFE, exact half-open source support, OFE-
ground area basis, and immutable source receipt digest. Segments sort by
`(start_s,end_s,source_kind,source_receipt_sha256,transaction_id,destination)`.
Duplicate identity, overlap under the same receipt, foreign support or
destination, missing receipt, unknown kind, and positive hourly/daily
additional supply without the complete segment set reject as `WAT5-E-001`.
The canonical segment sum must reconstruct the accepted per-source and hourly
additional-supply owners with the same checked operation order and exact
binary64 bits; `TOL-WAT5-001` does not repair custody mismatch.

Replay takes the sorted union of rain boundaries, segment boundaries, and
300-second bin boundaries. For each elementary interval `j`:

```text
i_rain,j = sum active local-rain rates
i_add,j  = sum active segment depth/(end-start), in canonical segment order
i_wb14,j = checked(i_rain,j + i_add,j)
```

WB14 advances exactly once on `i_wb14,j`; it is not run separately per source.
Overlap integration records rain only in `R5`, additional sources only in
`A5`, and their combined partition in `F5`, `D5`, and `G5raw`. Independent
per-bin/event closure is `sum(R5+A5)=sum(F5+D5+G5raw)`. Existing hourly closing
scales `G5raw` only to the already-authoritative WB14 generation and cannot
move source support. The source-completeness code is
`rainfall_and_exact_typed_additional_segments_saturation_hourly_zero_order_hold`.

WAT5 remains diagnostic-only. It neither changes accepted WB14 owners nor
feeds WAT, PASS, HBP, peak, erosion, routing, or manifest values. Publication
for every 1-, 10-, and 19-OFE required case remains one atomic requested-output
set with the manifest last. Missing/invalid segments, replay or closure failure,
writer failure, or injected publication failure preserves all pre-run outputs,
accepted owners, receipts, and runtime state byte-for-byte.

`OBL-WAT5-P-005` — Prove each admitted source alone, overlapping sources and
rain, boundary-crossing support, canonical order, exact per-source/hour/day
reconstruction, distinct `R5/A5`, one WB14 replay, unknown/aggregate/relabel/
retime/duplicate/omission/receipt-substitution poisons, diagnostics-off
noninterference, 1/10/19 complete-output-set success, and rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Project exact accepted non-rain receipts to typed segments; form the union partition with rain and 300-second boundaries; advance WB14 once on the combined rate while recording distinct rain/additional depths. |
| branch/guard | Only the four closed non-rain source kinds with exact receipt/support custody are admitted; precipitation-lineage and aggregate-only supplies cannot enter the segment surface. |
| invariant guard map | `INV-WAT5-005/010` -> typed segment schema, accepted-source projector, source/hour closure, piecewise replay, writer and rollback validators. |
| test vector | `OBL-WAT5-P-005`: closed sources, overlaps/boundaries, exact closures, identity/source poisons, one replay, 1/10/19 publication, rollback. |
| binding exposure | `BEI-WAT5-002`, active, `maps-to-existing-INV`, IDs `005/010/P-005`, dual review/verification. |
| change log | 2026-09-02, contract 3: exact typed accepted non-rain segment timing and one combined piecewise replay; unchanged WB14 physics, WAT5 resolution/tolerance, and protected-output authority. |

## Bounded Source-Supported Closing Reconciliation Amendment

`INV-WAT5-011` covers the narrow representational case where the accepted
post-partition hourly WB14 owner retains a finite positive depth but the
source-complete diagnostic replay produces exact zero raw generation because
all segment supply is admitted to infiltration. `TOL-WATBAL-009` validates the
accepted hourly/daily owner relationship but does not mutate or create hourly
bins. This amendment likewise leaves the accepted hour unchanged; it only
gives its already-owned positive depth one deterministic WAT5 closing location.

For hour `h`, reconstruct in canonical bin order:

```text
S_h = sum_k(R5(k)+A5(k))
F_h = sum_k F5(k)
D_h = sum_k D5(k)
R_h = sum_k G5raw(k)
B_h = accepted authoritative WB14 generation
epsilon_h = checked(B_h-R_h).
```

The special branch requires exact `B_h>0`, exact `R_h=0`, exact positive
source support `S_h>0`, ordinary raw closure under `TOL-WAT5-001`, complete
segment/receipt reconstruction under `INV-WAT5-010`, accepted hourly/daily
closure under `TOL-WATBAL-009`, finite `epsilon_h>0`, and
`epsilon_h<=TOL-WAT5-002`. It does not apply when raw generation is positive;
that case retains the ordinary proportional closing shape.

Let the source-complete piecewise partition already constructed by
`INV-WAT5-010` contain elementary pieces `j`. Select
`j* = arg max(end_s,start_s)` among pieces in hour `h` with exact positive
combined source rate `i_rain,j+i_add,j>0`. Because 300-second boundaries are
members of the partition, `j*` lies in exactly one WAT5 bin `k*`. Set:

```text
C5(k*)       = epsilon_h
C5(k != k*)  = +0
G5closed(k)  = C5(k).
```

The latest supported piece is the causal canonical location for a residual of
the completed ordered partition; caller/source order cannot change it. `C5`
is a closing-ledger operand bound to the accepted hourly owner and the complete
positive-source piece, not rainfall, additional supply, raw runoff, or a debit
from a particular parcel. Therefore `R5`, `A5`, `F5`, `D5`, and `G5raw` remain
bit-identical to the unchanged replay, including
`sum(R5+A5)=sum(F5+D5+G5raw)`. Closed closure separately requires
`sum(G5closed)=B_h` exactly under the declared operation order. Public rows
make `C5` independently reconstructible from existing raw, closed, and hourly
authority fields; no schema column or dataset-version change is required.

First-positive, uniform, rainfall-shaped, largest-source, caller-last, or
source-free placement is prohibited. So are splitting `epsilon_h`, subtracting
it from source, infiltration, or depression fields, changing any segment
support, using a daily scalar, applying the operand twice, or accepting a value
above `TOL-WAT5-002`. Missing/foreign receipt or support, nonunique selected
piece/bin, raw/source/accepted-ledger closure failure, nonfinite arithmetic,
negative result, or later publication failure rejects and preserves every
accepted owner, receipt, WAT/PASS/HBP/peak/erosion value, staged output, and
runtime state byte-for-byte.

`OBL-WAT5-P-006` — Prove the exact subnormal litter-overflow vector, last-piece
selection under reordered and overlapping sources, unchanged raw operands and
source supports, exact raw and closed closure, retained authoritative hour,
zero/foreign/missing source refusal, first/uniform/duplicate poison, exact
tolerance boundary and first-above rejection, and complete rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | After exact raw replay and accepted-hour validation, compute the closing residual; for the eligible zero-raw case place it once on the bin containing the latest positive-source elementary piece. |
| branch/guard | Requires positive accepted hour, exact zero raw generation, complete positive source support, raw and accepted-ledger closure, and `epsilon_h<=TOL-WAT5-002`; otherwise `WAT5-E-002/004`. |
| invariant guard map | `INV-WAT5-011` -> typed closing-reconciliation operand, canonical piece/bin selector, independent raw/closed closure and rollback validators. |
| test vector | `OBL-WAT5-P-006`: exact subnormal case, raw/closed/source closure, deterministic placement, boundary/above-bound and placement poisons, rollback. |
| binding exposure | `BEI-WAT5-003`, active, `maps-to-existing-INV`, IDs `011/P-006`, dual review/verification. |
| change log | 2026-09-02, contract 4: bounded source-supported WAT5 closing-ledger placement for an accepted positive hour with exact zero raw generation; unchanged raw replay, source timing, hourly owner, and protected outputs. |

## Accepted SurfaceLiquid Receipt-Complete Source Amendment

`INV-WAT5-012` makes the sealed accepted SurfaceLiquid ingress receipts the
single WAT5 source-custody authority for the Stage-3 direct path. For each OFE
and exact support, group disposition receipts that share one immutable source
parcel. The group contributes that parcel's source depth once; infiltration,
retention, runoff, and overflow dispositions are evidence about its accepted
fate and are not additional WAT5 sources.

The projector retains `RawPrecipitation`, `CanopyThroughfall`,
`CanopyInitialDrainage`, `CanopySecondDrainage`, and `CanopyStemflow` in `R5`.
It maps sealed `TerminalReceiver`, `RoutedRunon`, `LitterPhaseOverflow`, and
`CondensationOverflow` receipts to the corresponding typed `A5` segments.
Before replay, the canonical checked sum of those groups must reconstruct the
accepted OFE ingress ledger under the SurfaceLiquid closure tolerance. This
post-disposition closure supersedes any forcing-only WAT5 source projection;
in particular, a pre-ingress snow-terminal vector cannot be counted again
after refreeze and receiver disposition.

Missing, duplicate, foreign, prefix-invalid, or disposition-substituted
receipts reject as `WAT5-E-001` before publication. Receipt identity, support,
transaction, destination, area basis, and precipitation/non-rain lineage remain
immutable through the projector. Accepted infiltration, retained-surface, and
routed/outlet-runoff dispositions populate `F5`, `D5`, and `G5raw` directly on
their exact receipt supports. Their per-support, per-hour, and day sums must
reconstruct the accepted ingress ledger and WB14 hourly owner. Re-solving the
already-disposed mass through a day-wide Green-Ampt state is prohibited because
it would create a second partition owner with different chronology.

`OBL-WAT5-P-007` — Prove all nine accepted source kinds, disposition custody,
accepted-ingress and accepted-hour reconstruction, heterogeneous and routed
multi-OFE order independence, no second physical solve, and omission/
duplication/foreign/disposition-substitution poisons.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Preserve precipitation/non-rain lineage, project exact sealed receipt dispositions to `F5/D5/G5raw`, and close source, disposition, and hourly runoff sums to the accepted ledgers without a second Green-Ampt solve. |
| branch/guard | Only authenticated Stage-3 ingress receipt kinds are admitted; disposition receipts do not create a second source parcel. |
| invariant guard map | `INV-WAT5-012` -> receipt-group projector, source/support identity validators, accepted-ingress closure, and atomic rollback. |
| test vector | `OBL-WAT5-P-007`: all source kinds and dispositions, heterogeneous/routed OFEs, exact source/disposition/hour closure, second-solve absence, and omission/duplicate/foreign/disposition-substitution poisons. |
| binding exposure | `BEI-WAT5-004`, active, `maps-to-existing-INV`, IDs `012/P-007`, dual review/verification. |
| change log | 2026-09-02, contract 5: receipt-complete Stage-3 WAT5 source projection with unchanged WB14 physics and protected outputs. |

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `BEI-WAT5-001` | `20260810-five-minute-generation-power-equivalent-cutover-001` implementation increment | `active` | `maps-to-existing-INV` | `INV-WAT5-001, INV-WAT5-002, INV-WAT5-003, INV-WAT5-004, INV-WAT5-005, INV-WAT5-006, INV-WAT5-007, INV-WAT5-008, INV-WAT5-009` | `flagged-binding-addition` | Version-2 authority is consolidated in this contract; package artifacts are execution evidence rather than separate binding authority. |
| `BEI-WAT5-002` | Exact Accepted Additional-Segment Source Amendment | `active` | `maps-to-existing-INV` | `INV-WAT5-005, INV-WAT5-010, OBL-WAT5-P-005` | `flagged-binding-addition` | Admits only exact accepted non-rain source segments and one piecewise WB14 replay; aggregate-only timing and rain/source relabeling remain rejected. |
| `BEI-WAT5-003` | Bounded Source-Supported Closing Reconciliation Amendment | `active` | `maps-to-existing-INV` | `INV-WAT5-011, OBL-WAT5-P-006` | `flagged-binding-addition` | Retains a bounded accepted positive hour on the latest exact positive-source piece only when raw replay closes with exact zero generation; raw operands and source supports remain unchanged. |
| `BEI-WAT5-004` | Accepted SurfaceLiquid Receipt-Complete Source Amendment | `active` | `maps-to-existing-INV` | `INV-WAT5-012, OBL-WAT5-P-007` | `flagged-binding-addition` | Projects WAT5 source custody once from the sealed accepted SurfaceLiquid ingress receipts, retains precipitation/non-rain lineage, and requires source sum to close to the accepted ingress ledger. |

## Gap Register and Promotability

| Gap ID | Statement | Disposition | Promotability |
|---|---|---|---|
| `GAP-WAT5-001` | Runon, routed melt, condensation overflow, and partial frost-retention lacked subhourly producer timing. | `CLOSED` for receipt-complete accepted Stage-3 ingress sources under `INV-WAT5-010/012`; aggregate-only and partial frost-retention inputs still fail `WAT5-E-001`. | promotable for source-complete typed-segment domain |
| `GAP-WAT5-002` | Hourly saturation return lacks finer timing. | Publish only as labeled hourly zero-order hold. | promotable diagnostic; no instantaneous claim |
| `GAP-WAT5-003` | Erosion fixed exponent was not admitted. | Power-equivalent fields remain null and method records `NO_ADOPTION`. | water output promotable; erosion cutover prohibited |

## Change Log

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-08-10` | `1` | `Codex` | Initial contract for the optional version-1 five-minute diagnostic water dataset, sparse semantics, closure, units, source-completeness failures, noninterference, and erosion `NO_ADOPTION`. |
| `2026-08-11` | `2` | `Codex` | Reopened output-integrity correction: expose per-bin depression-storage retention, rename post-depression generation, validate public rows, bind run-file-only opt-in, and require rollback-safe run-output-set publication with manifest last. |
| `2026-09-02` | `3` | `Codex` | Admit exact accepted snow-terminal, routed-runon, and litter-overflow segments to one piecewise WAT5 replay while preserving distinct rain/additional custody and aggregate-only rejection. |
| `2026-09-02` | `4` | `Codex` | Admit one bounded, explicit closing-ledger operand on the latest exact positive-source piece when an accepted positive hour has zero raw diagnostic generation; preserve raw replay and hourly ownership unchanged. |
| `2026-09-02` | `5` | `Codex` | Project Stage-3 WAT5 sources once from the sealed SurfaceLiquid ingress receipts, retain precipitation/non-rain lineage, add condensation-overflow custody, and require the projected source sum to reconstruct accepted ingress before replay. |
