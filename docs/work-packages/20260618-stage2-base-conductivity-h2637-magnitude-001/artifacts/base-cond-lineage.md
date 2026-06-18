# Base Conductivity Lineage

Evidence class: Static + Ran

Verdict: mixed by surface. The H2637 raw `ksat` parse is correct, and the
hourly lateral `ui_ssh` surface is consistent with source intent for
anisotropy `1.0`. The vertical percolation `ssc` surface is not: openWEPP
currently forms it arithmetically where source intent forms it by inverse
conductivity accumulation.

## Raw H2637 Soil Input

Static:

- `/tmp/openwepp_farpoint01_h2637/without_ui/runs/p2637.sol:1` has
  `solwpv = 9002`.
- `/tmp/openwepp_farpoint01_h2637/without_ui/runs/p2637.sol:29` has
  `ksatadj = 0`, so the REFINTENT001 `ksatadj` correction is not active for
  this fixture.
- Lines 31-34 of the same file provide the layer source:

| Source layer | Interval mm | `ksat` mm/h | `ui_anisrt` |
|---:|---:|---:|---:|
| 1 | 0-200 | 60 | 1.0 |
| 2 | 200-560 | 330.2755 | 1.0 |
| 3 | 560-1140 | 33.0275 | 1.0 |
| 4 | 1140-1600 | 33.0275 | 1.0 |

Ran:

- STAGE2-LATQCC trace rows expose nine 200 mm runtime layers for H2637. The
  ninth layer mirrors the deepest source conductivity; the key mismatch below
  is at the 400-600 mm split layer and does not depend on the tail extension.

## Current openWEPP Projection

Static:

- `legacy_conductivity_source_layers_from_seed_depths` loads
  `layer.ksat_mm_h` and computes
  `lateral_ksat_mm_h = ksat_mm_h * anisotropy_ratio` in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:1196-1255`.
- `legacy_normalize_conductivity_layers_to_200mm` then accumulates both
  `weighted_ksat_mm_h` and `weighted_lateral_ksat_mm_h` arithmetically and
  publishes:
  - `ssc_m_s = weighted_ksat_mm_h / 3.6e6`;
  - `lateral_ssh_m_s = weighted_lateral_ksat_mm_h / 3.6e6`.
- That arithmetic publication is in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs:1332-1379`.

Ran:

- `/tmp/stage2_latqcc/diag3/h2637_trace_selected_days.jsonl` shows
  `wb19_solwpv_mode = 9002`, `wb19_lateral_drain_lane_substeps = 24`, and
  `wb19_lateral_anisotropy_ratio = 1.0`.
- The trace shows `wb18_ssc_layers_m_s` and `wb19_lateral_ssh_layers_m_s`
  identical today:

| Runtime layer | Current `wb18_perc_ssc`, m/s | Current `wb19_lateral_ssh`, m/s |
|---:|---:|---:|
| 1 | 0.000016666666666666667 | 0.000016666666666666667 |
| 2 | 0.00009174319444444445 | 0.00009174319444444445 |
| 3 | 0.0000752294166666667 | 0.0000752294166666667 |
| 4-9 | about 0.00000917430555555556 | about 0.00000917430555555556 |

## Source Intent

Static:

- `/workdir/wepp-forest_260430_baseline/src/input.for:752` says `ssc` uses a
  non-arithmetic mean.
- `/workdir/wepp-forest_260430_baseline/src/input.for:760` and `:843`
  accumulate vertical conductivity as `ksinv += thickness / ssc2`.
- `/workdir/wepp-forest_260430_baseline/src/input.for:761` and `:844`
  separately accumulate hourly horizontal conductivity as
  `ui_ksari += thickness * ssc2 * ui_anisrt`.
- `/workdir/wepp-forest_260430_baseline/src/input.for:926-928` finalizes
  `ssc1 = slayth / ksinv` and, when `ui_run == 1`,
  `ui_ssh1 = ui_ksari / slayth`.
- `SC-SUBHYD-001` HPHYS0257 says hourly modern lanes consume
  `wb19_lateral_ssh_####` as baseline `ui_ssh(i)` and must not substitute
  vertical `wb18_perc_ssc_####`.
- `SC-PERC-001` maps per-layer vertical saturated conductivity `Ksi` to
  `wb18_perc_ssc_####`.

## Normalized Values

Static computation from the H2637 source layers:

| Runtime layer | Interval mm | Current openWEPP `ssc`, mm/h | Source vertical `ssc`, mm/h | Source hourly `ui_ssh`, mm/h | Verdict |
|---:|---:|---:|---:|---:|---|
| 1 | 0-200 | 60 | 60 | 60 | matches |
| 2 | 200-400 | 330.2755 | 330.2755 | 330.2755 | matches |
| 3 | 400-600 | 270.8259 | 117.955408163210 | 270.8259 | vertical `ssc` inflated 2.296x |
| 4-9 | 600-1800 | 33.0275 | 33.0275 | 33.0275 | matches |

The split layer is the discriminating case:

- Current arithmetic value:
  `(160*330.2755 + 40*33.0275) / 200 = 270.8259 mm/h`
  or `0.0000752294166666667 m/s`.
- Source vertical `ssc` value:
  `200 / (160/330.2755 + 40/33.0275) = 117.955408163210 mm/h`
  or `0.0000327653911564473 m/s`.
- Source hourly horizontal `ui_ssh` value with `ui_anisrt = 1.0` remains
  arithmetic: `270.8259 mm/h`.

Lineage conclusion:

- The current equality `wb18_perc_ssc_0003 == wb19_lateral_ssh_0003` is not a
  valid invariant.
- For H2637 hourly lateral flow, the arithmetic `wb19_lateral_ssh_0003` is
  source-intent consistent.
- For H2637 percolation and any daily lateral consumer of vertical `ssc`,
  `wb18_perc_ssc_0003` is source-intent defective.
