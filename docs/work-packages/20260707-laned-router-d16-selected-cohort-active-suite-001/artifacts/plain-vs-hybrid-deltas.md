# Plain-vs-Hybrid Deltas

Status: PARTIAL. Evidence mode: Ran.

Only `h2637` reached both active plain and explicit active hybrid publication.
The selected suite stopped before `mn_corn_h4` could produce active plain
outputs, so the selected-cohort suite evidence hold remains open.

## H2637

| Metric | Plain | Hybrid | Delta | Relative delta |
|---|---:|---:|---:|---:|
| `total_source_m3` | `374423.3526212723` | `374423.35262127244` | `0.0000000001` | machine scale |
| `total_routed_outlet_m3` | `374463.0826831916` | `372817.05470593827` | `-1646.0279772533` | `-0.439570%` |
| `total_end_window_storage_m3` | `3167.32249029797` | `3261.408449926588` | `94.0859596286` | `2.97178%` |
| `total_clamp_m3` | `3207.052552219107` | `1655.1105346072773` | `-1551.9420176118` | `-48.3916%` |
| `total_tail_fold_m3` | `36426.08375542731` | `36681.65010117003` | `255.5663457427` | `0.701601%` |
| `max_day_cascade_residual_rel` | `2.4765580376695655e-13` | `4.579976970630865e-13` | `2.1034189329612994e-13` | closure scale |
| `max_day_identity_residual_rel` | `2.4507313136493173e-13` | `4.443101533649709e-13` | `1.9923702200003918e-13` | closure scale |
| `max_day_seam_residual_rel` | `5.0415846159888125e-14` | `4.082921815102614e-14` | `-9.586628008861987e-15` | closure scale |

Publication hashes differ for H2637:

- Plain HBP: `efd8c4255fbe976ecafb2bc89defb7bebd4e2054c9e65c89cd5353c4c31c3790`
- Hybrid HBP: `bfb2b002f8b67cd3c4b42504ae9cbc02189c13651f658b0c035c51cd23f50621`
- Plain pass parquet: `21c54bf2b045c3fb2f79f39ca174e36a4d188b39f7064f2a75f1170be6bb1656`
- Hybrid pass parquet: `44e3da28ed5a2c4b310507d8d2f03e65c3a902e2f01e59f08e11e732d80e1f34`

H2637 pass-surface sediment deltas remain in the previously observed tolerance
hold range:

| Metric | Plain | Hybrid | Relative delta |
|---|---:|---:|---:|
| `tdet_sum` | `23.0500822964053` | `22.614819809849912` | `-1.88833%` |
| `sedcon_1_sum` | `0.003291965085541466` | `0.003078835252837724` | `-6.47424%` |
| `sedcon_2_sum` | `0.04811333586560604` | `0.04499836138762825` | `-6.47424%` |
| `sedcon_3_sum` | `0.022790527515287068` | `0.02131501328887655` | `-6.47424%` |
| `sedcon_4_sum` | `0.05718536337767179` | `0.05348304375608286` | `-6.47424%` |
| `sedcon_5_sum` | `0.2906656139945431` | `0.27184721462710326` | `-6.47424%` |

## External Members

No selected external member reached a plain-vs-hybrid comparison. The first
external member, `mn_corn_h4`, failed active plain on the Rev-21 `canhgt`
guard before its output manifest was written.
