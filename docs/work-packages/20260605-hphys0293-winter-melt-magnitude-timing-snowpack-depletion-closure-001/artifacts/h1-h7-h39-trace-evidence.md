# H1/H7/H39 Trace Evidence

Status: complete
Evidence mode: Ran

Ran:

- Trace command: `.venv/bin/python docs/work-packages/20260605-hphys0293-winter-melt-magnitude-timing-snowpack-depletion-closure-001/artifacts/hphys0293_diagnostics.py --run-root /tmp/hphys0293_full_20260604T212429Z --trace-max-days 1800`
- Target rows Markdown: `/tmp/hphys0293_full_20260604T212429Z/reports/hphys0293_target_depletion_rows.md`
- Target rows JSON: `/tmp/hphys0293_full_20260604T212429Z/reports/hphys0293_target_depletion_rows.json`
- Trace status: `/tmp/hphys0293_full_20260604T212429Z/reports/hphys0293_target_trace_status.tsv`

Representative target rows:

| Hill | Date | ΔQ mm | ΔRM mm | ΔSnow mm | ΔTotal-Soil mm | SWE before mm | SWE after mm | ΔSWE mm | S mm | Routed melt mm | WB12 infiltrated mm | WB12 residual before Q mm | SWE closure mm |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | 2014-132 | 0.000000 | -5.096264 | -27.366889 | 58.586514 | 369.200133 | 353.593111 | -15.607022 | 5.163736 | 5.163736 | 5.163736 | 0.000000 | 0.000000 |
| H1 | 2014-133 | 0.000000 | -4.111735 | -31.591426 | 54.342302 | 353.593111 | 332.428574 | -21.164536 | 12.828265 | 12.828265 | 12.828265 | 0.000000 | 0.000000 |
| H1 | 2014-145 | 0.000000 | -21.008350 | 0.000000 | 19.768662 | 9.491650 | 0.000000 | -9.491650 | 9.491650 | 10.001650 | 10.001650 | 0.510000 | 0.000000 |
| H7 | 2014-133 | 0.000000 | -4.277466 | -27.980960 | -15.404526 | 388.200477 | 367.189040 | -21.011437 | 12.342534 | 12.342534 | 12.342534 | 0.000000 | 0.000000 |
| H7 | 2014-146 | 0.000000 | -16.787045 | 0.000000 | -43.473561 | 11.852955 | 0.000000 | -11.852955 | 11.852955 | 14.472955 | 14.472955 | 0.000000 | 0.000000 |
| H7 | 2016-111 | 0.000000 | -15.906187 | 0.000000 | -8.387465 | 5.263813 | 0.000000 | -5.263813 | 5.263813 | 5.263813 | 5.263813 | 0.000000 | 0.000000 |
| H39 | 2014-132 | 0.000000 | -5.255619 | -28.364899 | -39.221034 | 366.211770 | 350.235101 | -15.976669 | 5.214381 | 5.214381 | 5.214381 | 0.000000 | 0.000000 |
| H39 | 2014-133 | 0.000000 | -4.197007 | -32.666437 | -42.312396 | 350.235101 | 328.863563 | -21.371538 | 12.872993 | 12.872993 | 12.872993 | 0.000000 | 0.000000 |
| H39 | 2014-145 | 0.000000 | -22.466657 | 0.000000 | -57.541480 | 5.033343 | 0.000000 | -5.033343 | 5.033343 | 5.415843 | 5.415843 | 0.637500 | 0.000000 |

Interpretation:

- Target `ΔQ` is zero within floating tolerance for representative rows.
- Target `SWE closure` is zero within trace tolerance, so the snow-state trace is internally closed.
- H1/H39 terminal depletion days show small post-partition residual-before-`Q` values while WB13 `Q` remains zero; this does not reopen HPHYS0292 WB14 capacity ownership.
