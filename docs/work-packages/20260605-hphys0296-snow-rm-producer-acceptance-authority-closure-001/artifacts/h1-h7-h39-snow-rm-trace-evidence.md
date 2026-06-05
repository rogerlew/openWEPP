# H1/H7/H39 Snow/RM Trace Evidence

Status: executed-hold
Evidence mode: Ran

Ran:
- Targeted traces:
  `/tmp/hphys0296_full_20260605T070000Z/hillslope_output/H1.hphys0296.trace.jsonl`
- Targeted traces:
  `/tmp/hphys0296_full_20260605T070000Z/hillslope_output/H7.hphys0296.trace.jsonl`
- Targeted traces:
  `/tmp/hphys0296_full_20260605T070000Z/hillslope_output/H39.hphys0296.trace.jsonl`
- Classification report:
  `/tmp/hphys0296_full_20260605T070000Z/reports/hphys0296_snow_rm_acceptance.md`

| Hill | Window | Year | Days | ΔRM sum | ΔSnow start | ΔSnow end | Neg raw melt | Neg days | Routed melt | Post rain | Released rain | RM identity abs | SWE closure abs | Class |
|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| H1 | first-abs-storage-ge-10mm | 2013 | 112-127 | `-14.672576` | `-7.485938` | `0.000000` | `-11.533388` | `7` | `161.349242` | `0.268182` | `12.417542` | `0.000000` | `0.000000` | corrected-negative-melt-candidate |
| H1 | spring-2014 | 2014 | 120-146 | `-63.562583` | `-12.270901` | `0.000000` | `-28.308587` | `13` | `480.813939` | `6.523478` | `18.094466` | `0.000000` | `0.510000` | corrected-negative-melt-candidate |
| H1 | spring-2016 | 2016 | 104-111 | `-15.276407` | `-14.483710` | `0.000000` | `-0.224814` | `1` | `75.643593` | `0.000000` | `8.890000` | `0.000000` | `0.000000` | producer-magnitude-timing-hold |
| H7 | first-abs-storage-ge-10mm | 2013 | 112-127 | `-11.427268` | `-3.497309` | `0.000000` | `-12.330078` | `7` | `182.544550` | `0.268182` | `11.936326` | `0.000000` | `0.000000` | corrected-negative-melt-candidate |
| H7 | spring-2014 | 2014 | 120-146 | `-61.799024` | `-7.326839` | `0.000000` | `-30.016628` | `14` | `511.967497` | `3.393478` | `20.784398` | `0.000000` | `0.000000` | corrected-negative-melt-candidate |
| H7 | spring-2016 | 2016 | 104-111 | `-16.885426` | `-16.034749` | `0.000000` | `-0.255930` | `1` | `135.724574` | `0.000000` | `8.890000` | `0.000000` | `0.000000` | producer-magnitude-timing-hold |
| H39 | first-abs-storage-ge-10mm | 2013 | 97-112 | `-10.689298` | `3.777880` | `-5.836295` | `-10.581207` | `6` | `40.902415` | `0.688287` | `30.318638` | `0.000000` | `0.000000` | corrected-negative-melt-candidate |
| H39 | spring-2014 | 2014 | 120-146 | `-65.755222` | `-12.559468` | `0.000000` | `-29.319266` | `14` | `477.193800` | `6.650978` | `18.068037` | `0.000000` | `0.382500` | corrected-negative-melt-candidate |
| H39 | spring-2016 | 2016 | 104-111 | `-15.940163` | `-15.127710` | `0.000000` | `-0.243386` | `1` | `83.289837` | `0.000000` | `8.890000` | `0.000000` | `0.000000` | producer-magnitude-timing-hold |

First-divergence note:
- First H1/H7/H39 residual rows are small early-2013 `Snow-Water` differences
  around `0.070795` to `0.078712 mm`, with candidate post-winter rain
  `0.074545 mm` on day 11 and closed snow-state accounting. These are not the
  storage-collapse owner.
