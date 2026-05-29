# HPARITY01 Always-Fail Column Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Baseline Evidence Source
- Semantic run root:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/semantic`
- Summary:
  - `total_hillslopes = 39`
  - `semantic_fail_count = 39`
  - `semantic_pass_count = 0`
  - `total_common_rows = 56979`

## Always-Fail Column Matrix

| WB13 column | Hillslopes failing | Aggregate fail_count | max_abs_diff | max_abs locus `(OFE,J,Y)` | Canonical authority anchors | Runtime writer lineage (current) | Guard families | Follow-on closure wave |
|---|---:|---:|---:|---|---|---|---|---|
| `Dp` | 39 | 44447 | 0.24 | `H9 (1,1,2013)` | `SC-PERC-001` WB13 coupling addendum; `SC-WATBAL-001` WB13 schema + HPARITY01 lineage register | `D` -> `Dp` via `crates/openwepp-runner/src/hillslope/mod.rs` WB13 row assembly | `HKERNEL-WB11-PERC-E-001..003`, `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY04` |
| `Ep` | 39 | 56834 | 7.78 | `H34 (1,202,2015)` | `SC-EVAP-001` WB13 ET coupling addendum; `SC-WATBAL-001` WB13 schema + HPARITY01 lineage register | `Ep` runtime flux -> `Ep` publication | `HKERNEL-WB11-ET-E-001..003`, `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY03` |
| `Es` | 39 | 56973 | 10.0289189528 | `H6 (1,179,2015)` | `SC-EVAP-001` WB13 ET coupling addendum; `SC-WATBAL-001` WB13 schema + HPARITY01 lineage register | `Es` runtime flux -> `Es` publication | `HKERNEL-WB11-ET-E-001..003`, `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY03` |
| `ProfileDepth` | 39 | 56979 | 200.0 | `H9 (1,1,2013)` | `SC-PERC-001` WB13 profile coupling addendum; `SC-WATBAL-001` WB13 invariants | `solthk` -> `ProfileDepth` (`*1000`) in WB13 row assembly | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY02` |
| `ProfilePorosityCap` | 39 | 56979 | 752.384 | `H6 (1,1,2013)` | `SC-PERC-001` WB13 profile coupling addendum; `SC-WATBAL-001` HPARITY01 lineage register | currently synthesized as `max(ProfileFCStore, ProfileWPStore) + 20.0` in WB13 row assembly (non-baseline-authoritative placeholder lineage) | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY02` |
| `ProfileFCStore` | 39 | 56979 | 216.7970000000 | `H7 (1,1,2013)` | `SC-PERC-001` WB13 profile coupling addendum; `SC-WATBAL-001` WB13 invariants | `sum(thetfc_i * dg_i)` in WB13 row assembly | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY02` |
| `ProfileWPStore` | 39 | 56979 | 87.1809000000 | `H6 (1,1,2013)` | `SC-PERC-001` WB13 profile coupling addendum; `SC-WATBAL-001` WB13 invariants | `sum(thetdr_i * dg_i)` in WB13 row assembly | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY02` |
| `RM` | 39 | 20732 | 45.7399999999 | `H23 (1,143,2014)` | `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-011`; `SC-WATBAL-001` `INV-WATBAL-026` | `prcp + SWE_before - SWE_after + Irr` -> `RM` in WB13 row assembly | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY03` |
| `Snow-Water` | 39 | 27358 | 562.47 | `H9 (1,114,2014)` | `SC-SNOWFREEZE-001` runtime SWE publication; `SC-WATBAL-001` `INV-WATBAL-026/027` | `snow.runtime_swe` -> `Snow-Water` in WB13 row assembly | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY03` |
| `latqcc` | 39 | 56979 | 205.5085 | `H9 (1,45,2016)` | `SC-SUBHYD-001` WB13 coupling addendum; `SC-WATBAL-001` WB19 lateral coupling | runtime lateral symbol `q` -> `latqcc` (`*1000`) in WB13 row assembly | `HKERNEL-WB11-LAT-E-001..003`, `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY04` |
| `Total-Soil` | 39 | 56505 | 354.7904481621 | `H12 (1,146,2014)` | `SC-SOIL-001` `INV-SOIL-013`; `SC-WATBAL-001` `INV-WATBAL-029` + HPARITY01 alias policy; `SC-SYSTEM-001` `INV-SYSTEM-027` | `wb11_soil_water` -> `Total-Soil` in WB13 row assembly | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY04` |
| `SoilWaterTotal` | 39 | 56505 | 354.7904481621 | `H12 (1,146,2014)` | `SC-WATBAL-001` WB13 invariant `SoilWaterTotal = Total-Soil + frozwt`; `SC-SYSTEM-001` `INV-SYSTEM-027` | `Total-Soil + frozwt` -> `SoilWaterTotal` in WB13 row assembly | `HKERNEL-WB13-HWAT-E-001..003`, `HS-SIMOUT-E-001` | `HPARITY04` |

## Alias Continuity Check (MEASURE-HP01-002)
- Baseline alias source in semantic evidence maps canonical `Total-Soil` from
  baseline field `Total-Soil Water`.
- Candidate alias source maps canonical `Total-Soil` from candidate field
  `Total-Soil`.
- `SoilWaterTotal` remains distinct and coupled by closure relation
  `SoilWaterTotal = Total-Soil + frozwt`.

## Implementation Readiness Notes
1. All 12 columns are now explicitly mapped to contract authority + runtime
   writer ownership + guard families.
2. `ProfilePorosityCap` currently uses non-authoritative placeholder lineage in
   runtime row assembly and is explicitly queued for closure under `HPARITY02`.
3. `Dp` symbol ambiguity is explicitly disambiguated in contracts:
   - climate `Dp` (time-to-peak, `h`) vs WB13 `Dp` (deep percolation, `mm`).
