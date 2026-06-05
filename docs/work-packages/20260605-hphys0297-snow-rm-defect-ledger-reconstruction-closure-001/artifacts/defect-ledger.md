# Snow/RM Defect Ledger

Status: executed-hold
Evidence mode: Ran

Ran:
- Run root: `/tmp/hphys0297_full_20260605T000000Z`.
- Ledger JSON:
  `/tmp/hphys0297_full_20260605T000000Z/reports/hphys0297_defect_ledger.json`.
- Reconstruction tolerance: `2.000 mm` window-sum absolute residual.

Verdict counts:
- `UNRESOLVED`: `9` windows.
- `LEGACY-DEFECTIVE`: `0` windows.
- `OPENWEPP-DEFECTIVE`: `0` windows.

Ledger:

| Hill | Window | Year | Days | Verdict | Observed baseline-candidate RM mm | Reconstructed baseline-branch RM mm | Baseline RM mm | Reconstruction-baseline mm | Negative raw melt mm | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | first-abs-storage-ge-10mm | 2013 | 112-127 | UNRESOLVED | 14.672576 | 168.698510 | 176.290000 | -7.591490 | -11.533388 | Pinned-baseline branch reconstruction does not reproduce baseline RM to the named tolerance. |
| H1 | spring-2014 | 2014 | 120-146 | UNRESOLVED | 63.562583 | 507.214450 | 550.900000 | -43.685550 | -28.308587 | Pinned-baseline branch reconstruction does not reproduce baseline RM to the named tolerance. |
| H1 | spring-2016 | 2016 | 104-111 | UNRESOLVED | 15.276407 | 69.363458 | 90.920000 | -21.556542 | -0.224814 | Negative raw melt is immaterial; the window remains a snow/winter producer magnitude/timing hold. |
| H7 | first-abs-storage-ge-10mm | 2013 | 112-127 | UNRESOLVED | 11.427268 | 189.242793 | 194.240000 | -4.997207 | -12.330078 | Pinned-baseline branch reconstruction does not reproduce baseline RM to the named tolerance. |
| H7 | spring-2014 | 2014 | 120-146 | UNRESOLVED | 61.799024 | 532.542050 | 577.160000 | -44.617950 | -30.016628 | Pinned-baseline branch reconstruction does not reproduce baseline RM to the named tolerance. |
| H7 | spring-2016 | 2016 | 104-111 | UNRESOLVED | 16.885426 | 129.535968 | 152.610000 | -23.074032 | -0.255930 | Negative raw melt is immaterial; the window remains a snow/winter producer magnitude/timing hold. |
| H39 | first-abs-storage-ge-10mm | 2013 | 97-112 | UNRESOLVED | 10.689298 | 44.886462 | 52.280000 | -7.393538 | -10.581207 | Pinned-baseline branch reconstruction does not reproduce baseline RM to the named tolerance. |
| H39 | spring-2014 | 2014 | 120-146 | UNRESOLVED | 65.755222 | 504.318219 | 549.600000 | -45.281781 | -29.319266 | Pinned-baseline branch reconstruction does not reproduce baseline RM to the named tolerance. |
| H39 | spring-2016 | 2016 | 104-111 | UNRESOLVED | 15.940163 | 77.041250 | 99.230000 | -22.188750 | -0.243386 | Negative raw melt is immaterial; the window remains a snow/winter producer magnitude/timing hold. |

Interpretation:
- HPHYS0297 does not authorize any residual acceptance or re-tiering.
- The six HPHYS0296 corrected-negative-melt candidates remain unresolved because
  reconstruction does not close to tolerance.
- The three spring-2016 windows remain producer magnitude/timing holds.
