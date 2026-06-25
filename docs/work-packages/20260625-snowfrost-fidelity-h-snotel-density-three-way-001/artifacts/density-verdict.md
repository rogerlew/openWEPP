# Density Fork Routing

Evidence mode: Ran.

All five SNOTEL sites route `STRUCTURAL` under the diagnostic density fork:

| Site | As-built SSD | Observed-density SSD | openWEPP SWE mean residual m | openWEPP depth mean residual m | openWEPP density mean residual kg/m3 | Observed-density arm depth MAE m |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `snotel_mica_creek_st_joe_id` | 250 | 370 | -0.0982399 | -0.177861 | -109.531 | 0.512475 |
| `snotel_paradise_wa` | 250 | 495 | -0.477462 | 0.290776 | -298.888 | 1.78542 |
| `snotel_css_lab_ca` | 250 | 380 | -0.179183 | -0.246599 | -117.316 | 0.695022 |
| `snotel_snowbird_ut` | 250 | 445 | -0.339509 | -0.616503 | -121.205 | 1.13327 |
| `snotel_niwot_co` | 250 | 340 | -0.0737288 | -0.225856 | -57.3536 | 0.373776 |

Interpretation:

- The observed-density SSD arm was derived from peak-SWE-period SNOTEL density
  before residual comparison.
- The arm contrast does not cleanly close the depth residuals as a simple
  low-density input problem.
- The fork route is structural snow-model/forcing/settlement behavior, not an
  authorization to tune SSD or to alter production physics in this package.
