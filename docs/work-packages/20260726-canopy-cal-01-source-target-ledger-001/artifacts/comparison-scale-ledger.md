# Comparison-Scale Ledger

Evidence class: `Executed comparison-boundary audit`

## Process boundaries

| Quantity | Bill surface | Valid comparison | Prohibited comparison |
| --- | --- | --- | --- |
| Hillslope surface runoff | WEPP Windows/Cloud profile result | Same climate, soil, slope, run controls, and hillslope boundary | Total gauged watershed discharge without lateral/baseflow accounting |
| Watershed discharge | WEPPcloud watershed and gauged outlet | Same watershed area and full surface/lateral/baseflow boundary | One hillslope’s surface runoff |
| Hillslope sediment delivery | Sediment leaving profile | Same hillslope geometry and soil/management | Channel plus watershed sediment yield |
| Channel loss | WEPPcloud channel network | Observed channel export with matching suspended/bedload definition | Background watershed yield quoted without constituent boundary |
| Forest-floor stock | Oi+Oe+Oa or equivalent | Same material horizons, area basis, and measurement basis | Total fuel including downed coarse wood; loss-on-ignition organic mass silently equated with oven-dry bulk mass |
| Litterfall flux | Trap input per year | Same foliage/needle/wood/fruit material classes | Standing foliage biomass |

## Reported 100-year means

| Site | Surface | Management | Precip (mm/yr) | Surface runoff (mm/yr) | Sediment (kg/ha/yr) | Boundary |
| --- | --- | --- | ---: | ---: | ---: | --- |
| Hubbard | WEPPcloud | mature forest | 1441 | 13.38 | 2.3 | hillslope 1 |
| Hubbard | WEPP Windows | constant cover | 1441 | 11.5 | 4 | hillslope 1 |
| Hubbard | WEPP Windows | hardwood | 1441 | 11.8 | 5 | hillslope 1 |
| Hubbard | WEPPcloud | watershed | 1441 | 625 | 116 | watershed/channel network |
| Santee | WEPPcloud | mature forest | 1321 | 103.4 | 0 | hillslope 2 |
| Santee | WEPP Windows | constant cover | 1321 | 211.4 | 53 | hillslope 2 |
| Santee | WEPP Windows | mixed | 1321 | 177.3 | 27 | hillslope 2 |

The Hubbard `25-50 kg/ha/yr` literature range is reference-watershed stream
sediment and excludes difficult-to-quantify suspended fines. The Santee
`290 mm/yr` is derived total watershed runoff. Neither is a hillslope surface
runoff or hillslope sediment validation target.

## Daily runoff return-period series (mm)

| Return period (yr) | HB constant | HB perennial | HB Cloud hill streamflow | Santee constant | Santee perennial | Santee Cloud hill streamflow |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 2 | 1.4 | 0.1 | 25 | 70 | 67 | 52 |
| 5 | 22 | 25 | 38 | 91 | 87 | 75 |
| 10 | 34 | 40 | 56 | 128 | 126 | 110 |
| 20 | 59 | 63 | 79 | 141 | 137 | 120 |
| 25 | 68 | 64 | 81 | 142 | 140 | 130 |
| 50 | 75 | 78 | 93 | 159 | 152 | 150 |

## Peak-flow return-period series (mm/h)

| Return period (yr) | HB constant | HB perennial | Santee constant | Santee perennial |
| ---: | ---: | ---: | ---: | ---: |
| 2 | 0.1 | 0.1 | 22 | 18 |
| 5 | 3 | 3 | 31 | 27 |
| 10 | 6 | 19 | 48 | 43 |
| 20 | 21 | 45 | 55 | 49 |
| 25 | 37 | 46 | 56 | 52 |
| 50 | 46 | 70 | 71 | 62 |

These series are report-table reproduction targets. Event files, ranking
method, ties, plotting code, and exact machine outputs were not delivered, so
their digit precision is not independently verifiable.

## Channel critical shear

The report’s `19 Pa` WEPPcloud value, Santee `1-2 Pa` judgment, Elliot
`tau_c (Pa) = D50 (mm)` rule, proposed slope formula, and Srivastava relation
are channel-context hypotheses. They are outside canopy calibration and may not
explain a canopy-management difference. Srivastava et al. supports relating
critical shear to observed bed particle size, but application to Hubbard or
Santee requires site-specific channel observations.
