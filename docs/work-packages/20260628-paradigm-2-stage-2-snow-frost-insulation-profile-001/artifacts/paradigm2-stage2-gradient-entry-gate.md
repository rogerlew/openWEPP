# PARADIGM-2 Stage 2 Gradient Entry Gate

Schema: `paradigm2-stage2-snow-frost-insulation-profile-v1`
Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-079 INV-SNOWFREEZE-050 ADR-0029`
Evidence: `Static + Ran (completed outputs reused)`

## Summary

- Gate passed: `True`
- Reason: rows=159986; multi_layer=56831; positive=49548; material_positive=48464; negative=585; max=446.5207296110246; min=-60.66582900192148; material_threshold=10.0
- Candidate trace rows: `159986`
- Multi-layer rows after snow step: `56831`
- Positive gradient rows: `49548`
- Negative gradient rows: `585`
- Max gradient: `446.5207296110246 kg m^-3`
- Min gradient: `-60.66582900192148 kg m^-3`

## Site Matrix

| Site | Corpus | Multi-layer rows | Positive | Material positive | Negative | Max gradient | Min gradient |
|---|---|---:|---:|---:|---:|---:|---:|
| snotel_mica_creek_st_joe_id | snotel_observed | 4998 | 4387 | 4319 | 34 | 446.2620317016145 | -28.732634592815998 |
| snotel_paradise_wa | snotel_observed | 7764 | 7056 | 6955 | 10 | 446.18069924700717 | -14.522355796462136 |
| snotel_css_lab_ca | snotel_observed | 5505 | 4209 | 4088 | 26 | 431.7279085673359 | -20.526493437781426 |
| snotel_snowbird_ut | snotel_observed | 6066 | 5428 | 5328 | 41 | 446.34973157807565 | -36.51607500298621 |
| snotel_niwot_co | snotel_observed | 6812 | 5907 | 5800 | 65 | 446.3665171439347 | -32.54288352506134 |
| harvard_open | cancov_forest | 4536 | 3491 | 3385 | 78 | 446.33656786861127 | -60.66582900192148 |
| harvard_hardwood | cancov_forest | 4366 | 3465 | 3366 | 94 | 446.33922713399613 | -58.322564906978755 |
| marcell_conifer | cancov_forest | 5641 | 5251 | 5118 | 76 | 446.5206273665998 | -47.59928830755521 |
| marcell_deciduous | cancov_forest | 5491 | 5109 | 4992 | 79 | 446.5207296110246 | -47.461085654842435 |
| marcell_open | cancov_forest | 5652 | 5245 | 5113 | 82 | 446.52072790823854 | -47.43868470305492 |
