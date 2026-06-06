# Single-OFE Closure Ledger

Status: executed-hold

Evidence mode: Ran

Ran:

- Read WAT parquet outputs under
  `/tmp/wbval04_rocky_mountain_20260606T000000Z/outputs/`.
- WAT emitters: `18/22`.
- Fail-closed before WAT publication: `4/22`.

Residual definition:

`R = inputs - outputs - delta_storage`

Units: `mm` depth over the published hillslope area.

Inputs:

- `P`
- `Irr`

Outputs:

- `Q`
- `Ep`
- `Es`
- `Er`
- `Dp`
- `latqcc`

Storage:

- `SoilWaterTotal`
- `Snow-Water`

Tolerance:

- `1.0 mm/year`
- Rationale: this validation is a conservation characterization pass over
  annual WAT rows; residuals materially above `1 mm/year` are unambiguous
  conservation breaks and are large enough not to depend on display rounding.

Scope caveat:

- Published WAT rows provide end-of-day storage.
- Full-calendar-year residuals are classified only for years `2..6`, where the
  prior year's final row provides start storage.
- Year `1` lacks a pre-day-1 initial storage row in WAT output and is labeled
  `initial-storage-missing-not-full-year-classified`.

Summary:

| Prefix | Run status | Classification | Max abs full-year R (mm) | First blocker/break |
|---|---|---|---:|---|
| p1 | ran | conservation-break | 49.153 | 2 |
| p2 | ran | conservation-break | 64.712 | 2 |
| p3 | ran | conservation-break | 51.689 | 2 |
| p4 | ran | conservation-break | 94.433 | 2 |
| p5 | ran | conservation-break | 58.848 | 2 |
| p6 | ran | conservation-break | 51.591 | 2 |
| p7 | fail-closed | runner-domain-blocked |  | HKERNEL-WB11-PERC-E-003 |
| p8 | ran | conservation-break | 51.440 | 2 |
| p9 | ran | conservation-break | 80.884 | 2 |
| p10 | ran | conservation-break | 49.839 | 2 |
| p11 | fail-closed | runner-domain-blocked |  | HKERNEL-WB11-PERC-E-003 |
| p12 | ran | conservation-break | 49.774 | 2 |
| p13 | ran | conservation-break | 47.431 | 2 |
| p14 | ran | conservation-break | 49.335 | 2 |
| p15 | ran | conservation-break | 51.307 | 2 |
| p16 | ran | conservation-break | 58.961 | 2 |
| p17 | ran | conservation-break | 51.367 | 2 |
| p18 | fail-closed | runner-domain-blocked |  | HKERNEL-WB11-PERC-E-003 |
| p19 | ran | conservation-break | 78.740 | 2 |
| p20 | fail-closed | runner-domain-blocked |  | HKERNEL-WB11-PERC-E-003 |
| p21 | ran | conservation-break | 54.229 | 2 |
| p22 | ran | conservation-break | 61.861 | 2 |

Full-calendar-year ledger (`years 2..6`):

| Prefix | Year | Inputs | Outputs | dStorage | R | Class | Dominant magnitude output term |
|---|---:|---:|---:|---:|---:|---|---|
| p1 | 2 | 934.600 | 821.636 | 86.692 | 26.272 | conservation-break | latqcc |
| p1 | 3 | 667.400 | 617.206 | 21.291 | 28.902 | conservation-break | latqcc |
| p1 | 4 | 951.800 | 1102.235 | -180.179 | 29.745 | conservation-break | latqcc |
| p1 | 5 | 684.600 | 531.928 | 110.229 | 42.443 | conservation-break | Ep |
| p1 | 6 | 1044.200 | 1034.674 | -39.627 | 49.153 | conservation-break | latqcc |
| p2 | 2 | 934.600 | 801.323 | 103.147 | 30.130 | conservation-break | latqcc |
| p2 | 3 | 667.400 | 599.816 | 21.229 | 46.356 | conservation-break | latqcc |
| p2 | 4 | 951.800 | 1100.753 | -179.640 | 30.686 | conservation-break | latqcc |
| p2 | 5 | 684.600 | 510.824 | 109.063 | 64.712 | conservation-break | Ep |
| p2 | 6 | 1044.200 | 1026.499 | -39.028 | 56.729 | conservation-break | latqcc |
| p3 | 2 | 934.600 | 808.268 | 98.999 | 27.333 | conservation-break | latqcc |
| p3 | 3 | 667.400 | 611.190 | 21.229 | 34.981 | conservation-break | latqcc |
| p3 | 4 | 951.800 | 1099.302 | -179.707 | 32.205 | conservation-break | latqcc |
| p3 | 5 | 684.600 | 523.592 | 109.319 | 51.689 | conservation-break | Ep |
| p3 | 6 | 1044.200 | 1035.586 | -39.217 | 47.831 | conservation-break | latqcc |
| p4 | 2 | 934.600 | 785.146 | 110.765 | 38.688 | conservation-break | latqcc |
| p4 | 3 | 667.400 | 567.915 | 20.535 | 78.951 | conservation-break | latqcc |
| p4 | 4 | 951.800 | 1089.230 | -184.688 | 47.258 | conservation-break | latqcc |
| p4 | 5 | 684.600 | 477.629 | 112.538 | 94.433 | conservation-break | Ep |
| p4 | 6 | 1044.200 | 987.630 | -34.893 | 91.463 | conservation-break | latqcc |
| p5 | 2 | 934.600 | 819.096 | 85.390 | 30.114 | conservation-break | latqcc |
| p5 | 3 | 667.400 | 621.626 | 21.281 | 24.494 | conservation-break | latqcc |
| p5 | 4 | 951.800 | 1095.324 | -179.344 | 35.820 | conservation-break | latqcc |
| p5 | 5 | 684.600 | 527.478 | 109.305 | 47.816 | conservation-break | Ep |
| p5 | 6 | 1044.200 | 1024.831 | -39.479 | 58.848 | conservation-break | latqcc |
| p6 | 2 | 934.600 | 807.636 | 100.738 | 26.226 | conservation-break | latqcc |
| p6 | 3 | 667.400 | 609.051 | 21.229 | 37.119 | conservation-break | latqcc |
| p6 | 4 | 951.800 | 1099.566 | -179.595 | 31.828 | conservation-break | latqcc |
| p6 | 5 | 684.600 | 523.790 | 109.219 | 51.591 | conservation-break | Ep |
| p6 | 6 | 1044.200 | 1037.031 | -39.229 | 46.397 | conservation-break | latqcc |
| p8 | 2 | 934.600 | 809.306 | 99.044 | 26.250 | conservation-break | latqcc |
| p8 | 3 | 667.400 | 611.407 | 21.229 | 34.764 | conservation-break | latqcc |
| p8 | 4 | 951.800 | 1099.371 | -179.707 | 32.136 | conservation-break | latqcc |
| p8 | 5 | 684.600 | 523.834 | 109.326 | 51.440 | conservation-break | Ep |
| p8 | 6 | 1044.200 | 1036.127 | -39.225 | 47.298 | conservation-break | latqcc |
| p9 | 2 | 934.600 | 794.122 | 110.247 | 30.231 | conservation-break | latqcc |
| p9 | 3 | 667.400 | 573.012 | 20.535 | 73.854 | conservation-break | latqcc |
| p9 | 4 | 951.800 | 1103.493 | -184.672 | 32.979 | conservation-break | latqcc |
| p9 | 5 | 684.600 | 493.372 | 112.380 | 78.848 | conservation-break | Ep |
| p9 | 6 | 1044.200 | 998.044 | -34.728 | 80.884 | conservation-break | latqcc |
| p10 | 2 | 934.600 | 765.953 | 141.969 | 26.678 | conservation-break | Ep |
| p10 | 3 | 667.400 | 644.318 | -6.054 | 29.136 | conservation-break | Ep |
| p10 | 4 | 951.800 | 1048.702 | -126.737 | 29.835 | conservation-break | Ep |
| p10 | 5 | 684.600 | 590.651 | 48.998 | 44.951 | conservation-break | Ep |
| p10 | 6 | 1044.200 | 942.608 | 51.752 | 49.839 | conservation-break | Ep |
| p12 | 2 | 934.600 | 722.296 | 187.592 | 24.712 | conservation-break | Ep |
| p12 | 3 | 667.400 | 642.006 | -7.849 | 33.243 | conservation-break | Ep |
| p12 | 4 | 951.800 | 1019.164 | -99.879 | 32.516 | conservation-break | Ep |
| p12 | 5 | 684.600 | 625.504 | 9.322 | 49.774 | conservation-break | Ep |
| p12 | 6 | 1044.200 | 919.727 | 80.179 | 44.295 | conservation-break | Ep |
| p13 | 2 | 934.600 | 823.753 | 87.122 | 23.725 | conservation-break | latqcc |
| p13 | 3 | 667.400 | 618.733 | 21.291 | 27.376 | conservation-break | latqcc |
| p13 | 4 | 951.800 | 1098.797 | -180.006 | 33.009 | conservation-break | latqcc |
| p13 | 5 | 684.600 | 527.336 | 109.833 | 47.431 | conservation-break | Ep |
| p13 | 6 | 1044.200 | 1039.810 | -39.414 | 43.805 | conservation-break | latqcc |
| p14 | 2 | 934.600 | 808.194 | 101.068 | 25.338 | conservation-break | latqcc |
| p14 | 3 | 667.400 | 611.979 | 21.229 | 34.192 | conservation-break | latqcc |
| p14 | 4 | 951.800 | 1099.867 | -179.565 | 31.497 | conservation-break | latqcc |
| p14 | 5 | 684.600 | 526.072 | 109.193 | 49.335 | conservation-break | Ep |
| p14 | 6 | 1044.200 | 1038.823 | -39.232 | 44.609 | conservation-break | latqcc |
| p15 | 2 | 934.600 | 707.434 | 201.209 | 25.957 | conservation-break | Ep |
| p15 | 3 | 667.400 | 638.411 | -5.535 | 34.524 | conservation-break | Ep |
| p15 | 4 | 951.800 | 1001.146 | -81.436 | 32.090 | conservation-break | Ep |
| p15 | 5 | 684.600 | 641.773 | -8.480 | 51.307 | conservation-break | Ep |
| p15 | 6 | 1044.200 | 912.675 | 84.677 | 46.848 | conservation-break | Ep |
| p16 | 2 | 934.600 | 817.997 | 86.561 | 30.043 | conservation-break | latqcc |
| p16 | 3 | 667.400 | 619.166 | 21.124 | 27.110 | conservation-break | Ep |
| p16 | 4 | 951.800 | 1096.524 | -179.245 | 34.522 | conservation-break | latqcc |
| p16 | 5 | 684.600 | 514.693 | 120.468 | 49.439 | conservation-break | Ep |
| p16 | 6 | 1044.200 | 1020.945 | -35.706 | 58.961 | conservation-break | latqcc |
| p17 | 2 | 934.600 | 712.586 | 196.100 | 25.914 | conservation-break | Ep |
| p17 | 3 | 667.400 | 638.373 | -7.900 | 36.927 | conservation-break | Ep |
| p17 | 4 | 951.800 | 1007.724 | -87.604 | 31.680 | conservation-break | Ep |
| p17 | 5 | 684.600 | 635.438 | -2.206 | 51.367 | conservation-break | Ep |
| p17 | 6 | 1044.200 | 912.156 | 83.468 | 48.576 | conservation-break | Ep |
| p19 | 2 | 934.600 | 634.962 | 267.890 | 31.748 | conservation-break | Ep |
| p19 | 3 | 667.400 | 605.913 | 5.434 | 56.053 | conservation-break | Ep |
| p19 | 4 | 951.800 | 1038.927 | -119.721 | 32.594 | conservation-break | Ep |
| p19 | 5 | 684.600 | 573.485 | 32.374 | 78.740 | conservation-break | Ep |
| p19 | 6 | 1044.200 | 935.737 | 38.513 | 69.950 | conservation-break | Ep |
| p21 | 2 | 934.600 | 681.506 | 223.893 | 29.200 | conservation-break | Ep |
| p21 | 3 | 667.400 | 628.467 | 1.072 | 37.861 | conservation-break | Ep |
| p21 | 4 | 951.800 | 960.494 | -38.096 | 29.403 | conservation-break | Ep |
| p21 | 5 | 684.600 | 673.431 | -40.602 | 51.772 | conservation-break | Ep |
| p21 | 6 | 1044.200 | 904.448 | 85.523 | 54.229 | conservation-break | Ep |
| p22 | 2 | 934.600 | 758.188 | 145.791 | 30.621 | conservation-break | Ep |
| p22 | 3 | 667.400 | 640.381 | -5.992 | 33.011 | conservation-break | Ep |
| p22 | 4 | 951.800 | 1041.715 | -122.773 | 32.858 | conservation-break | Ep |
| p22 | 5 | 684.600 | 591.275 | 42.500 | 50.825 | conservation-break | Ep |
| p22 | 6 | 1044.200 | 929.670 | 52.669 | 61.861 | conservation-break | Ep |

Year-1 observed interval (`day 2..365`, not full-calendar-year classified):

| Prefix | Days summed | Inputs | Outputs | dStorage | R | Note |
|---|---:|---:|---:|---:|---:|---|
| p1 | 364 | 734.400 | 656.039 | 12.407 | 65.954 | initial storage missing |
| p2 | 364 | 734.400 | 627.802 | 1.426 | 105.172 | initial storage missing |
| p3 | 364 | 734.400 | 633.071 | 3.832 | 97.498 | initial storage missing |
| p4 | 364 | 734.400 | 670.691 | -20.240 | 83.948 | initial storage missing |
| p5 | 364 | 734.400 | 641.204 | 19.324 | 73.872 | initial storage missing |
| p6 | 364 | 734.400 | 640.181 | -2.102 | 96.321 | initial storage missing |
| p8 | 364 | 734.400 | 627.067 | 10.460 | 96.873 | initial storage missing |
| p9 | 364 | 734.400 | 680.219 | -21.872 | 76.053 | initial storage missing |
| p10 | 364 | 734.400 | 692.280 | -24.459 | 66.579 | initial storage missing |
| p12 | 364 | 734.400 | 668.907 | -27.252 | 92.744 | initial storage missing |
| p13 | 364 | 734.400 | 642.620 | 30.545 | 61.235 | initial storage missing |
| p14 | 364 | 734.400 | 634.028 | 5.058 | 95.314 | initial storage missing |
| p15 | 364 | 734.400 | 662.262 | -24.028 | 96.166 | initial storage missing |
| p16 | 364 | 734.400 | 657.368 | 0.197 | 76.835 | initial storage missing |
| p17 | 364 | 734.400 | 665.432 | -26.825 | 95.792 | initial storage missing |
| p19 | 364 | 734.400 | 542.785 | 110.003 | 81.612 | initial storage missing |
| p21 | 364 | 734.400 | 651.582 | -2.666 | 85.485 | initial storage missing |
| p22 | 364 | 734.400 | 683.531 | -26.468 | 77.337 | initial storage missing |

No WAT terms were imputed for blocked hillslopes.
