# Single-OFE Closure Ledger

Status: executed-hold

Evidence mode: Ran

Ran:

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
  annual WAT rows; residuals materially above 1 mm/year are unambiguous
  conservation breaks and are large enough not to depend on display rounding.

Scope caveat:

- Published WAT rows provide end-of-day storage.
- Full-calendar-year residuals are therefore classified only for years `2..6`,
  where the prior year's final row provides start storage.
- Year `1` lacks a pre-day-1 initial storage row in the WAT output and is
  labeled `initial-storage-missing-not-full-year-classified`.

Summary:

| Prefix | Run status | Classification | Max abs full-year R (mm) | First blocker/break |
|---|---|---|---:|---|
| p1 | ran | conservation-break | 49.153 | 2 |
| p2 | fail-closed | runner-domain-blocked |  | CLIM-RUNTIME-E-017 |
| p3 | ran | conservation-break | 51.689 | 2 |
| p4 | fail-closed | runner-domain-blocked |  | CLIM-RUNTIME-E-017 |
| p5 | ran | conservation-break | 58.848 | 2 |
| p6 | fail-closed | runner-domain-blocked |  | CLIM-RUNTIME-E-017 |
| p7 | fail-closed | runner-domain-blocked |  | HKERNEL-WB11-PERC-E-003 |
| p8 | ran | conservation-break | 51.440 | 2 |
| p9 | fail-closed | runner-domain-blocked |  | CLIM-RUNTIME-E-017 |
| p10 | ran | conservation-break | 49.839 | 2 |
| p11 | fail-closed | runner-domain-blocked |  | HKERNEL-WB11-PERC-E-003 |
| p12 | ran | conservation-break | 49.774 | 2 |
| p13 | ran | conservation-break | 47.431 | 2 |
| p14 | fail-closed | runner-domain-blocked |  | CLIM-RUNTIME-E-017 |
| p15 | ran | conservation-break | 51.307 | 2 |
| p16 | ran | conservation-break | 58.961 | 2 |
| p17 | fail-closed | runner-domain-blocked |  | CLIM-RUNTIME-E-017 |
| p18 | fail-closed | runner-domain-blocked |  | HKERNEL-WB11-PERC-E-003 |
| p19 | ran | conservation-break | 78.740 | 2 |
| p20 | fail-closed | runner-domain-blocked |  | HKERNEL-WB11-PERC-E-003 |
| p21 | ran | conservation-break | 54.229 | 2 |
| p22 | ran | conservation-break | 61.861 | 2 |

Full-calendar-year ledger (`years 2..6`):

| Prefix | Year | Inputs | Outputs | dStorage | R | Class | Dominant magnitude output term |
|---|---:|---:|---:|---:|---:|---|---|
| p1 | 2 | 934.600 | 821.683 | 86.692 | 26.225 | conservation-break | latqcc |
| p1 | 3 | 667.400 | 617.218 | 21.291 | 28.890 | conservation-break | latqcc |
| p1 | 4 | 951.800 | 1102.235 | -180.179 | 29.745 | conservation-break | latqcc |
| p1 | 5 | 684.600 | 531.928 | 110.229 | 42.443 | conservation-break | Ep |
| p1 | 6 | 1044.200 | 1034.674 | -39.627 | 49.153 | conservation-break | latqcc |
| p3 | 2 | 934.600 | 808.319 | 98.999 | 27.282 | conservation-break | latqcc |
| p3 | 3 | 667.400 | 611.196 | 21.229 | 34.975 | conservation-break | latqcc |
| p3 | 4 | 951.800 | 1099.302 | -179.707 | 32.205 | conservation-break | latqcc |
| p3 | 5 | 684.600 | 523.592 | 109.319 | 51.689 | conservation-break | Ep |
| p3 | 6 | 1044.200 | 1035.586 | -39.217 | 47.831 | conservation-break | latqcc |
| p5 | 2 | 934.600 | 819.113 | 85.390 | 30.097 | conservation-break | latqcc |
| p5 | 3 | 667.400 | 621.626 | 21.281 | 24.494 | conservation-break | latqcc |
| p5 | 4 | 951.800 | 1095.324 | -179.344 | 35.820 | conservation-break | latqcc |
| p5 | 5 | 684.600 | 527.478 | 109.305 | 47.816 | conservation-break | Ep |
| p5 | 6 | 1044.200 | 1024.831 | -39.479 | 58.848 | conservation-break | latqcc |
| p8 | 2 | 934.600 | 809.357 | 99.044 | 26.199 | conservation-break | latqcc |
| p8 | 3 | 667.400 | 611.414 | 21.229 | 34.758 | conservation-break | latqcc |
| p8 | 4 | 951.800 | 1099.371 | -179.707 | 32.136 | conservation-break | latqcc |
| p8 | 5 | 684.600 | 523.834 | 109.326 | 51.440 | conservation-break | Ep |
| p8 | 6 | 1044.200 | 1036.127 | -39.225 | 47.298 | conservation-break | latqcc |
| p10 | 2 | 934.600 | 765.930 | 142.039 | 26.631 | conservation-break | Ep |
| p10 | 3 | 667.400 | 644.310 | -6.034 | 29.124 | conservation-break | Ep |
| p10 | 4 | 951.800 | 1048.714 | -126.749 | 29.835 | conservation-break | Ep |
| p10 | 5 | 684.600 | 590.639 | 49.010 | 44.951 | conservation-break | Ep |
| p10 | 6 | 1044.200 | 942.599 | 51.762 | 49.839 | conservation-break | Ep |
| p12 | 2 | 934.600 | 722.177 | 187.761 | 24.662 | conservation-break | Ep |
| p12 | 3 | 667.400 | 642.008 | -7.845 | 33.237 | conservation-break | Ep |
| p12 | 4 | 951.800 | 1019.197 | -99.913 | 32.516 | conservation-break | Ep |
| p12 | 5 | 684.600 | 625.488 | 9.338 | 49.774 | conservation-break | Ep |
| p12 | 6 | 1044.200 | 919.683 | 80.222 | 44.295 | conservation-break | Ep |
| p13 | 2 | 934.600 | 823.797 | 87.122 | 23.681 | conservation-break | latqcc |
| p13 | 3 | 667.400 | 618.745 | 21.291 | 27.364 | conservation-break | latqcc |
| p13 | 4 | 951.800 | 1098.797 | -180.006 | 33.009 | conservation-break | latqcc |
| p13 | 5 | 684.600 | 527.336 | 109.833 | 47.431 | conservation-break | Ep |
| p13 | 6 | 1044.200 | 1039.810 | -39.414 | 43.805 | conservation-break | latqcc |
| p15 | 2 | 934.600 | 707.310 | 201.383 | 25.906 | conservation-break | Ep |
| p15 | 3 | 667.400 | 638.391 | -5.509 | 34.518 | conservation-break | Ep |
| p15 | 4 | 951.800 | 1001.141 | -81.431 | 32.090 | conservation-break | Ep |
| p15 | 5 | 684.600 | 641.794 | -8.501 | 51.307 | conservation-break | Ep |
| p15 | 6 | 1044.200 | 912.625 | 84.728 | 46.848 | conservation-break | Ep |
| p16 | 2 | 934.600 | 818.015 | 86.561 | 30.025 | conservation-break | latqcc |
| p16 | 3 | 667.400 | 619.170 | 21.124 | 27.106 | conservation-break | Ep |
| p16 | 4 | 951.800 | 1096.524 | -179.245 | 34.522 | conservation-break | latqcc |
| p16 | 5 | 684.600 | 514.693 | 120.468 | 49.439 | conservation-break | Ep |
| p16 | 6 | 1044.200 | 1020.945 | -35.706 | 58.961 | conservation-break | latqcc |
| p19 | 2 | 934.600 | 634.769 | 267.796 | 32.035 | conservation-break | Ep |
| p19 | 3 | 667.400 | 605.956 | 5.394 | 56.050 | conservation-break | Ep |
| p19 | 4 | 951.800 | 1038.933 | -119.727 | 32.594 | conservation-break | Ep |
| p19 | 5 | 684.600 | 573.526 | 32.334 | 78.740 | conservation-break | Ep |
| p19 | 6 | 1044.200 | 935.647 | 38.603 | 69.950 | conservation-break | Ep |
| p21 | 2 | 934.600 | 681.400 | 224.028 | 29.172 | conservation-break | Ep |
| p21 | 3 | 667.400 | 628.401 | 1.144 | 37.856 | conservation-break | Ep |
| p21 | 4 | 951.800 | 960.479 | -38.082 | 29.403 | conservation-break | Ep |
| p21 | 5 | 684.600 | 673.452 | -40.624 | 51.772 | conservation-break | Ep |
| p21 | 6 | 1044.200 | 904.396 | 85.574 | 54.229 | conservation-break | Ep |
| p22 | 2 | 934.600 | 758.163 | 145.838 | 30.599 | conservation-break | Ep |
| p22 | 3 | 667.400 | 640.337 | -5.944 | 33.006 | conservation-break | Ep |
| p22 | 4 | 951.800 | 1041.715 | -122.773 | 32.858 | conservation-break | Ep |
| p22 | 5 | 684.600 | 591.270 | 42.505 | 50.825 | conservation-break | Ep |
| p22 | 6 | 1044.200 | 929.660 | 52.678 | 61.861 | conservation-break | Ep |

Year-1 observed interval (`day 2..365`, not full-calendar-year classified):

| Prefix | Days summed | Inputs | Outputs | dStorage | R | Note |
|---|---:|---:|---:|---:|---:|---|
| p1 | 364 | 734.400 | 656.097 | 12.407 | 65.895 | initial storage missing |
| p3 | 364 | 734.400 | 633.123 | 3.832 | 97.446 | initial storage missing |
| p5 | 364 | 734.400 | 641.241 | 19.324 | 73.834 | initial storage missing |
| p8 | 364 | 734.400 | 627.119 | 10.460 | 96.821 | initial storage missing |
| p10 | 364 | 734.400 | 692.438 | -24.559 | 66.520 | initial storage missing |
| p12 | 364 | 734.400 | 669.175 | -27.453 | 92.678 | initial storage missing |
| p13 | 364 | 734.400 | 642.712 | 30.545 | 61.144 | initial storage missing |
| p15 | 364 | 734.400 | 662.571 | -24.266 | 96.095 | initial storage missing |
| p16 | 364 | 734.400 | 657.397 | 0.197 | 76.806 | initial storage missing |
| p19 | 364 | 734.400 | 543.196 | 110.083 | 81.121 | initial storage missing |
| p21 | 364 | 734.400 | 651.875 | -2.925 | 85.450 | initial storage missing |
| p22 | 364 | 734.400 | 683.671 | -26.579 | 77.309 | initial storage missing |

Blocked single-OFE hillslopes:

- `p2`, `p4`, `p6`, `p9`, `p14`, `p17`: `CLIM-RUNTIME-E-017`, hourly
  radiation value out of physical bound.
- `p7`, `p11`, `p18`, `p20`: `HKERNEL-WB11-PERC-E-003`, WB11 percolation
  domain violation at `sim_day_index=95`, calendar year `1990`, Julian day `95`.

No WAT terms were imputed for blocked hillslopes.
