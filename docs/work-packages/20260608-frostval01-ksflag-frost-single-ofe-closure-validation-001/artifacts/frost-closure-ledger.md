# Frost Closure Ledger

Status: executed-hold
Evidence mode: Ran + Static

Scope coverage:
- Full target set: 43 single-OFE hillslopes.
- Closure evaluated: 6 runnable hillslopes (`p8`, `p13`, `p22`, `p23`, `p26`, `p28`), years 2-7.
- Closure deferred: 37 hillslopes blocked by `HS-RUNTIME-E-062` before hydrology output.

Classification summary:
- Runnable set: 6/6 classified `frost-break`.
- Deferred set: 37/43 classified `frost-closure-undetermined-blocked`.

Runnable prefix summary (`closure_prefix_summary.csv`):

| Prefix | Years evaluated | Max abs residual mm | Worst year | All years clean | Classification |
| --- | --- | ---: | ---: | --- | --- |
| p8 | 6 | 130.843639 | 2 | False | frost-break |
| p13 | 6 | 129.453879 | 2 | False | frost-break |
| p22 | 6 | 133.983394 | 2 | False | frost-break |
| p23 | 6 | 128.351427 | 2 | False | frost-break |
| p26 | 6 | 125.206497 | 2 | False | frost-break |
| p28 | 6 | 117.836737 | 2 | False | frost-break |

Per-year residual ledger (`closure_yearly.csv`):

| Prefix | Year | Inputs mm | Outputs mm | Delta storage mm | Residual mm | Closure class |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| p8 | 2 | 10.386905 | 9.080300 | 132.150244 | -130.843639 | frost-break |
| p8 | 3 | 8.268849 | 8.998575 | -72.395278 | 71.665552 | frost-break |
| p8 | 4 | 10.736111 | 11.145221 | -41.087684 | 40.678574 | frost-break |
| p8 | 5 | 8.139881 | 8.296363 | -15.882672 | 15.726190 | frost-break |
| p8 | 6 | 7.384921 | 7.421127 | -4.389143 | 4.352937 | frost-break |
| p8 | 7 | 7.836310 | 7.084234 | 75.336085 | -74.584010 | frost-break |
| p13 | 2 | 9.381720 | 8.215367 | 130.620232 | -129.453879 | frost-break |
| p13 | 3 | 7.468638 | 8.117744 | -71.248133 | 70.599027 | frost-break |
| p13 | 4 | 9.697133 | 10.127052 | -47.817280 | 47.387361 | frost-break |
| p13 | 5 | 7.352151 | 7.425556 | -8.375561 | 8.302155 | frost-break |
| p13 | 6 | 6.670251 | 6.706602 | -4.787665 | 4.751314 | frost-break |
| p13 | 7 | 7.077957 | 6.405757 | 74.551569 | -73.879369 | frost-break |
| p22 | 2 | 6.803119 | 5.929526 | 134.856986 | -133.983394 | frost-break |
| p22 | 3 | 5.415854 | 5.912887 | -75.399730 | 74.902697 | frost-break |
| p22 | 4 | 7.031839 | 7.134028 | -15.603119 | 15.500930 | frost-break |
| p22 | 5 | 5.331384 | 5.607921 | -42.377994 | 42.101458 | frost-break |
| p22 | 6 | 4.836907 | 4.858681 | -4.115661 | 4.093887 | frost-break |
| p22 | 7 | 5.132554 | 4.619006 | 78.536180 | -78.022632 | frost-break |
| p23 | 2 | 12.648928 | 11.084984 | 129.915371 | -128.351427 | frost-break |
| p23 | 3 | 10.069610 | 10.936455 | -70.559095 | 69.692249 | frost-break |
| p23 | 4 | 13.074184 | 13.708066 | -52.301910 | 51.668028 | frost-break |
| p23 | 5 | 9.912556 | 9.949976 | -3.337993 | 3.300573 | frost-break |
| p23 | 6 | 8.993183 | 9.048250 | -5.282940 | 5.227873 | frost-break |
| p23 | 7 | 9.542874 | 8.642231 | 74.087120 | -73.186478 | frost-break |
| p26 | 2 | 9.303556 | 8.185471 | 126.324582 | -125.206497 | frost-break |
| p26 | 3 | 7.406412 | 8.004378 | -65.967533 | 65.369567 | frost-break |
| p26 | 4 | 9.616340 | 10.231692 | -69.040153 | 68.424801 | frost-break |
| p26 | 5 | 7.290895 | 7.155548 | 14.918999 | -14.783652 | frost-break |
| p26 | 6 | 6.614677 | 6.674957 | -7.424164 | 7.363884 | frost-break |
| p26 | 7 | 7.018986 | 6.376660 | 71.872849 | -71.230523 | frost-break |
| p28 | 2 | 19.390612 | 17.178202 | 120.049148 | -117.836737 | frost-break |
| p28 | 3 | 15.436557 | 16.539473 | -58.262529 | 57.159613 | frost-break |
| p28 | 4 | 20.042522 | 21.438318 | -75.071039 | 73.675244 | frost-break |
| p28 | 5 | 15.195795 | 14.863288 | 17.707335 | -17.374828 | frost-break |
| p28 | 6 | 13.786411 | 13.956773 | -9.764282 | 9.593920 | frost-break |
| p28 | 7 | 14.629078 | 13.280187 | 72.493152 | -71.144261 | frost-break |

Deferred closure population:
- Prefixes: `p1`, `p2`, `p3`, `p4`, `p5`, `p6`, `p7`, `p9`, `p10`, `p11`, `p12`, `p14`, `p15`, `p16`, `p17`, `p18`, `p19`, `p20`, `p21`, `p24`, `p25`, `p27`, `p29`, `p30`, `p31`, `p32`, `p33`, `p34`, `p35`, `p36`, `p37`, `p38`, `p39`, `p40`, `p41`, `p42`, `p43`
- Deferred reason for all above: `CLIHILL-E-011 ... HS-RUNTIME-E-062 ... corrected-lineage mapping coverage incomplete`

totalwatsed3 audit evidence:
- Legacy comparator audit ran:
  - Source: `/wc1/runs/al/algebraic-radium/wepp/output/interchange/totalwatsed3.parquet`
  - Summary: `/tmp/frostval01/full/reports/totalwatsed3_legacy_flag_audit/daily_closure_audit_summary.json`
  - Top days: `/tmp/frostval01/full/reports/totalwatsed3_legacy_flag_audit/daily_closure_audit_top_days.csv`
- OpenWEPP runnable-subset audit ran (6 prefixes):
  - Source: `/tmp/frostval01/full/subset_output/interchange/totalwatsed3.parquet`
  - Summary: `/tmp/frostval01/full/reports/totalwatsed3_subset_audit/daily_closure_audit_summary.json`
  - Top days: `/tmp/frostval01/full/reports/totalwatsed3_subset_audit/daily_closure_audit_top_days.csv`
- Key legacy audit metrics:
  - `rows = 2557`
  - `max_reported_runoff_mm = 54.62715183970597`
  - `max_reconstructed_runoff_mm = 54.62715183970597`
  - `runoff_consistency_mm.max_abs = 0.0`
  - `closure_reconstructed_with_storage_total_mm = 23.664437387473882`
  - `closure_reconstructed_with_storage_pct_of_precip = 0.3709392028884864`
  - `soilwatertotal_vs_legacy_max_abs_mm = 0.0034286179463833832`
- Key runnable-subset audit metrics:
  - `rows = 2557`
  - `max_reported_runoff_mm = 0.0`
  - `max_reconstructed_runoff_mm = 0.0`
  - `runoff_consistency_mm.max_abs = 0.0`
  - `closure_reconstructed_with_storage_total_mm = -1.01598909268756`
  - `closure_reconstructed_with_storage_pct_of_precip = -0.015925592399014983`
  - `soilwatertotal_vs_legacy_max_abs_mm = 0.0`
