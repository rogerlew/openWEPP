---
contract_id: SC-OUTPUT-WAT5-001
title: Five-Minute Hillslope Water Diagnostic Output Contract
status: approved
maturity: active
owner: openWEPP maintainers + hydrology/output reviewer
contract_version: 2
producer_scope:
  - Optional hillslope_wat_subhourly Parquet dataset
  - Diagnostic projection of WB14 generation and WB19 saturation return
consumer_scope:
  - End-user subhourly water diagnostics
  - Noninterference and closure verification
evidence_level: static
last_reviewed: 2026-08-11
supersedes: []
superseded_by: []
---

# SC-OUTPUT-WAT5-001 Five-Minute Hillslope Water Diagnostic Output Contract

Status: `approved`
Maturity: `active`
Evidence mode: `Static`

## Purpose

Define the optional version-2 five-minute hillslope water diagnostic dataset
without changing daily water balance, the authoritative 24-hour runoff ledger,
WB16/public peak, erosion execution, HBP, inter-OFE transfer, or routing.

## Scientific Scope and Exclusions

In scope: local-rainfall WB14 Green-Ampt diagnostic replay on 300-second bins,
per-hour closure to authoritative WB14 generation, labeled hourly-resolution
WB19 saturation-return composition, sparse event publication, units, keys,
metadata, and typed failures.

Out of scope: instantaneous or routed discharge, five-minute erosion solves,
erosion forcing adoption, rainfall `effint`, HBP fields/versioning, channel or
watershed routing, and invented timing for runon, routed melt, frost retention,
or any daily/hourly-only source.

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
| `A5(k)` | `mm` | Additional-supply depth; version 1 requires exact zero because no 300-second runon/melt authority exists. |
| `F5(k)` | `mm` | Raw isolated Green-Ampt infiltration increment. |
| `D5(k)` | `mm` | Raw post-infiltration excess retained into depression storage, allocated to the earliest generated bins in WB14 order. |
| `G5raw(k)` | `mm` | Raw WB14 post-depression generation depth. |
| `G5closed(k)` | `mm` | Raw shape closed within hour to authoritative WB14 generation. |
| `S5(k)` | `mm` | One twelfth of authoritative WB19 saturation return in hour `h`. |
| `Q5(k)` | `mm` | `G5closed(k) + S5(k)` diagnostic closing surface generation. |
| `I5(k)` | `mm h^-1` | `Q5(k) / (300 s / 3600 s)`. |
| `Qh` | `mm` | Authoritative closing surface-runoff depth for hour `h`. |
| `Ih` | `mm h^-1` | Authoritative hourly mean, numerically `Qh` per one-hour bin. |

## Algorithm State Surfaces

Required inputs are local hyetograph intervals, unchanged WB14 parameter/state
inputs, authoritative WB14 hourly generation, authoritative WB19 hourly
saturation return, daily/lane identity, and explicit output request state.

Output is zero or one sparse event span per day/OFE, containing typed rows. The
algorithm mutates no water, erosion, transfer, routing, or persistent state.

## Algorithm Specification

1. If output is not requested, do not open a writer and impose no publication
   requirement on otherwise valid runs.
2. If requested, validate all dimensional inputs as finite and nonnegative.
   Reject any positive additional supply without 300-second producer timing.
3. Split local hyetograph intervals at exact multiples of `300 s`. Advance the
   unchanged WB14 Green-Ampt equations in chronological order with continuous
   cumulative infiltration; allocate depression-storage removal from earliest
   raw generated bins, matching WB14 ordering, and retain that removed depth as
   `D5(k)`. Validate `sum(R5) = sum(F5) + sum(D5) + sum(G5raw)` independently
   from the same row operands exposed to consumers.
4. For each hour, let `R_h = Σ G5raw(k)` and `B_h` be authoritative WB14
   generation. If `B_h = 0`, set all twelve `G5closed` values to zero. If
   `B_h > 0` and `R_h = 0`, fail. Otherwise set
   `G5closed(k) = G5raw(k) * B_h / R_h` and validate closure.
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
8. The erosion power-equivalent columns are nullable in version 1. Under the
   frozen erosion `NO_ADOPTION`, publish null exponent/rate/duration and method
   `water_only_no_erosion_adoption`; never fabricate candidate values.
9. Stage every requested run output in its target directory. Close and validate
   every staged writer before publication. Publish the complete output set with
   rollback protection for preexisting targets and use the manifest as the last
   completion marker. Any construction, simulation, close, schema,
   checksum, link, or manifest failure preserves the complete pre-run output set
   and removes incomplete staging files.

## Branch and Guard Table

| Trigger | Guard | Failure behavior |
|---|---|---|
| Output not requested | `WAT5-B-001` | No writer; production behavior unchanged. |
| Fully dry day | `WAT5-B-002` | Valid zero-row day. |
| Positive additional supply without five-minute timing | `WAT5-E-001` | Typed source-incomplete output failure; no uniform/rainfall-shaped fallback. |
| Positive authoritative WB14 hour with zero raw support | `WAT5-E-002` | Typed closure/source failure. |
| Non-finite, negative, invalid time order, or invalid key | `WAT5-E-003` | Typed domain failure before publication. |
| Hour/day closure exceeds tolerance | `WAT5-E-004` | Typed closure failure. |
| Writer/link/publication fails or metadata is incomplete | `WAT5-E-005` | Typed publication failure; preserve the complete pre-run output set and publish no partial replacement set. |

## Invariants and Guard Map

| Invariant ID | Statement | Authority | Guard | Failure posture | Evidence |
|---|---|---|---|---|---|
| `INV-WAT5-001` | Dataset version `2.0` is optional and separate from WAT, PASS, and HBP; run-file `outputs.wat_subhourly` presence is the sole user-facing opt-in; diagnostics-off preserves protected outputs and state byte-for-byte. | `REF-WAT5-WATBAL` | run-file output config plus noninterference tests | hard-fail/HOLD | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WAT5-002` | Every row uses exact 300-second support, stable day/OFE keys, `event_ordinal=0`, `hour_index=floor(subinterval_index/12)`, and no gap-based event invention. | `REF-WAT5-OUTPUT-PHYS` | `WAT5-E-003` | typed error | `[INFERENCE][Static]` |
| `INV-WAT5-003` | Raw diagnostic replay uses unchanged WB14 equations and continuous infiltration; closed bins reconcile each hour to authoritative WB14 without changing it. | `REF-WAT5-RUNOFFPART`, `REF-WAT5-WATBAL` | `WAT5-E-002..004` | typed error | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WAT5-004` | Hourly WB19 return is composed only as a labeled twelve-bin zero-order hold, and every hour/day closes to the unchanged 24-bin ledger. | `REF-WAT5-WATBAL` | `WAT5-E-004` | typed error | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WAT5-005` | Positive additional supply without 300-second producer timing is rejected; no fallback may invent timing. | `REF-WAT5-WATBAL` | `WAT5-E-001` | typed error | `[DIRECT][Static]` |
| `INV-WAT5-006` | Sparse output includes first-through-last active bins and metadata declares omitted bins exact zero; a dry day emits no rows. | `REF-WAT5-OUTPUT-PHYS` | writer validation | typed error | `[INFERENCE][Static]` |
| `INV-WAT5-007` | The dataset never claims discharge, peak, routed flow, or erosion adoption; power-equivalent fields are null under `NO_ADOPTION`. | `REF-WAT5-PACKAGE`, `REF-WAT5-WATBAL` | schema/method tests | hard-fail/HOLD | `[DIRECT][Static]` |
| `INV-WAT5-008` | Emitted raw rows expose rainfall, infiltration, depression-storage retention, and post-depression generation so consumers independently reconstruct raw closure. | `REF-WAT5-RUNOFFPART`, `REF-WAT5-OUTPUT-PHYS` | positive-storage Parquet reconstruction | typed error/HOLD | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-WAT5-009` | WAT5-enabled run publication is all-or-nothing across every requested output and the manifest; any pre-completion failure preserves preexisting bytes, and the manifest is published last. | `REF-WAT5-PACKAGE` | transactional staging, rollback, and injected-failure tests | typed error/HOLD | `[DIRECT][Static]` |

## Producer and Consumer Obligations

- `OBL-WAT5-P-001`: emit the required schema, metadata, units, method/source
  codes, and atomic file semantics.
- `OBL-WAT5-P-002`: reconstruct closure independently from emitted rows and
  retain exact noninterference anchors for WAT/PASS/HBP/public peak/state.
- `OBL-WAT5-P-003`: stream rows; do not retain the full-run dataset in memory.
- `OBL-WAT5-P-004`: validate public row invariants at the writer boundary so a
  caller cannot serialize non-finite, negative, chronologically inconsistent,
  or non-closing records through the public writer API.
- `OBL-WAT5-C-001`: consumers must treat all quantities as diagnostic depths
  or depth rates, never volumetric discharge or instantaneous/routed peak.
- `OBL-WAT5-C-002`: consumers must honor sparse omitted-zero metadata and the
  nullable erosion fields.

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `R5` | `rainfall_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
| `A5` | `additional_supply_depth_mm` | Parquet | named `m -> mm`; v1 zero | `SC-OUTPUT-WAT5-001` |
| `F5` | `raw_green_ampt_infiltration_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
| `D5` | `depression_storage_retention_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
| `G5raw` | `raw_wb14_post_depression_generation_depth_mm` | Parquet | named `m -> mm` | `SC-OUTPUT-WAT5-001` |
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
4. Positive additional-supply typed rejection.
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

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `BEI-WAT5-001` | `20260810-five-minute-generation-power-equivalent-cutover-001` implementation increment | `active` | `maps-to-existing-INV` | `INV-WAT5-001, INV-WAT5-002, INV-WAT5-003, INV-WAT5-004, INV-WAT5-005, INV-WAT5-006, INV-WAT5-007, INV-WAT5-008, INV-WAT5-009` | `flagged-binding-addition` | Version-2 authority is consolidated in this contract; package artifacts are execution evidence rather than separate binding authority. |

## Gap Register and Promotability

| Gap ID | Statement | Disposition | Promotability |
|---|---|---|---|
| `GAP-WAT5-001` | Runon, routed melt, and partial frost-retention lack 300-second producer timing. | Version 1 fails requested output for affected positive days. | promotable for source-complete local-rain domain only |
| `GAP-WAT5-002` | Hourly saturation return lacks finer timing. | Publish only as labeled hourly zero-order hold. | promotable diagnostic; no instantaneous claim |
| `GAP-WAT5-003` | Erosion fixed exponent was not admitted. | Power-equivalent fields remain null and method records `NO_ADOPTION`. | water output promotable; erosion cutover prohibited |

## Change Log

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-08-10` | `1` | `Codex` | Initial contract for the optional version-1 five-minute diagnostic water dataset, sparse semantics, closure, units, source-completeness failures, noninterference, and erosion `NO_ADOPTION`. |
| `2026-08-11` | `2` | `Codex` | Reopened output-integrity correction: expose per-bin depression-storage retention, rename post-depression generation, validate public rows, bind run-file-only opt-in, and require rollback-safe run-output-set publication with manifest last. |
