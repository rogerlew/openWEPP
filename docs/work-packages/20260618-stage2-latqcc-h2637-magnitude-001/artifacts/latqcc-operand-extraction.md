# H2637 `latqcc` Operand Extraction

Package:
`20260618-stage2-latqcc-h2637-magnitude-001`

## Evidence

Static:

- `SC-SUBHYD-001` makes WB19 lateral flow authoritative for the lateral
  generation term behind WAT `latqcc`.
- `hydrology_phase_lateral_drainage/00_lateral_transfer.rs` computes WB19
  potential as:
  `q = fcdep_before * anisotropy * Ke * sin(atan(avgslp)) / slplen`,
  where `Ke = (86400 / lane_substeps) *
  conductivity_depth_sum / saturated_depth_sum`, with the legacy saturation
  multiplier only when `solwpv < 2006`.
- MAGPARITY01 already closed transfer, area scaling, and outlet export
  identities. This package therefore extracts the generation operands, not
  WAT/PASS aliases.

Ran:

- Built a temporary diagnostic source copy under `/tmp/stage2_latqcc/src` at
  source commit `85631941817a227f2fdce3657a153418c533762f`.
- The temporary patch only emitted persistent multi-OFE HPHYS0245 rows and WB19
  per-substep diagnostic operands. It was not committed to this repository.
- Ran H2637 with legacy sidecar discovery from a copied run directory:
  `/tmp/stage2_latqcc/diag3/runs`.
- Trace:
  `/tmp/stage2_latqcc/diag3/h2637_trace_selected_days.jsonl`,
  SHA-256 `3f5bc681ee69394a8b647eca39259b4a05be41a38170723e97b7878aaca167c5`,
  114 rows.
- Output WAT:
  `/tmp/stage2_latqcc/diag3/owepp_output/H2637.wat.parquet`,
  SHA-256 `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`,
  235,961 rows.
- Output PASS:
  `/tmp/stage2_latqcc/diag3/owepp_output/H2637.pass.parquet`,
  12,419 rows, numeric `runvol` sum `14,085,670.078744758 m3`,
  `sbrunv` sum `884,949.9416133772 m3`.

The selected trace days were simulation days `1, 4384, 4385, 5506, 5507,
5508`, each with all 19 OFEs. These include the global maximum `latqcc` row
and the dominant H2637 high-magnitude windows.

## Extracted Lane

All traced OFE-days used the same WB19 lane controls:

| Operand | Value |
| --- | ---: |
| `solwpv` | `9002` |
| `wb19_lateral_drain_lane_substeps` | `24` |
| `wb19_lateral_anisotropy_ratio` | `1.0` |
| `slplen` | `26.11 m` |
| `avgslp` range across 19 OFEs | `0.28647296..0.79066738` |

This is the 24-substep lateral lane, not a one-step daily lane.

## Static Layer Operands

The first traced OFE row exposes the H2637 layer values that are stable across
the traced OFE-days:

| Layer | `dg` m | `fc` m | `ul` m | `drfc` m | `por` | `ssc` m/s | `ssh` m/s |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 0.200000 | 0.019138 | 0.127054 | 0.023283 | 0.683646 | 1.666666667e-05 | 1.666666667e-05 |
| 2 | 0.200000 | 0.019138 | 0.127054 | 0.023283 | 0.683646 | 9.174319444e-05 | 9.174319444e-05 |
| 3 | 0.200000 | 0.021389 | 0.114999 | 0.025902 | 0.632195 | 7.522941667e-05 | 7.522941667e-05 |
| 4 | 0.200000 | 0.021510 | 0.072501 | 0.028173 | 0.426801 | 9.174305556e-06 | 9.174305556e-06 |
| 5 | 0.200000 | 0.021510 | 0.072501 | 0.028173 | 0.426801 | 9.174305556e-06 | 9.174305556e-06 |
| 6 | 0.200000 | 0.018980 | 0.074600 | 0.025605 | 0.426883 | 9.174305556e-06 | 9.174305556e-06 |
| 7 | 0.200000 | 0.013519 | 0.078933 | 0.020059 | 0.427072 | 9.174305556e-06 | 9.174305556e-06 |
| 8 | 0.200000 | 0.013519 | 0.078933 | 0.020059 | 0.427072 | 9.174305556e-06 | 9.174305556e-06 |
| 9 | 0.200000 | 0.013519 | 0.078933 | 0.020059 | 0.427072 | 9.174305556e-06 | 9.174305556e-06 |

Across the trace, `wb19_lateral_ssh_* == wb18_perc_ssc_*` for every layer.
No separate lateral-conductivity multiplier was observed in H2637.

## Operand Ranges

From the 114 traced OFE-days:

| Operand | Range |
| --- | ---: |
| `q` / `latqcc` | `0.00030490386850942446..0.07162409876710504 m` |
| WB19 potential | `0.00030490386850942446..0.07162409876710504 m` |
| WB19 target | `0.00030490386850942446..0.07162409876710504 m` |
| WB19 capacity total `tdv` | `0.25765010110832304..12.912830680839656 m` |
| minimum available pool by substep | `0.1744486720952016..0.5062229379024412 m` |
| `watyld` from `avpora - (avfca + (1-avcoca))` | `0.1706334131690198..0.339395182626328` |
| capacity-active layer-substep count | `24..215` |
| conductivity-active layer-substep count | `24..212` |
| positive withdrawal layers | `1..3` |

In the peak rows, `q == potential == target`; the available pool and capacity
are much larger than the realized `q`, so the magnitude is driven by the WB19
potential equation, not by a storage cap.

## Peak Rows

| Sim day | OFE | Year | Julian | `latqcc` mm | WAT `Q` mm | `SubRIn` mm |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 5507 | 7 | 16 | 28 | 71.624099 | 39.214739 | 71.425711 |
| 5507 | 6 | 16 | 28 | 71.425711 | 33.846244 | 69.213478 |
| 5507 | 4 | 16 | 28 | 70.184839 | 15.919787 | 24.442550 |
| 4384 | 11 | 13 | 1 | 69.967127 | 34.546224 | 62.506156 |
| 4384 | 12 | 13 | 1 | 69.920453 | 37.497966 | 69.967127 |
| 4385 | 1 | 13 | 2 | 69.321512 | 0.000000 | 0.000000 |
| 5507 | 5 | 16 | 28 | 69.213478 | 26.772797 | 70.184839 |
| 4384 | 2 | 13 | 1 | 68.924196 | 9.308141 | 18.616281 |

## Extraction Verdict

`latqcc` was traced back to WB19 operands for all 19 OFEs on the selected
H2637 high-magnitude days. The extracted term is WB19 lateral `q`; it is not a
WAT/PASS alias and not a downstream transfer artifact.
