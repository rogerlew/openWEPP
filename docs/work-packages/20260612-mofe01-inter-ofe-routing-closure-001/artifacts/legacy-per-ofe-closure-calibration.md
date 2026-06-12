# legacy per ofe closure calibration

Status: complete for increment M-A calibration

Evidence mode: Ran + Static

## Evidence

Ran:
- Parsed `/wc1/runs/ar/arboreal-dendrite/wepp/output/H1.wat.dat` through `H36.wat.dat` locally with `.venv/bin/python`.
- Parsed OFE counts from `/wc1/runs/ar/arboreal-dendrite/wepp/runs/p*.slp`.
- No comparator subagent was used.
- No legacy files were modified.

Static:
- Legacy WAT headers identify `UpStrmQ`, `SubRIn`, `latqcc`, `QOFE`, `Tile`, `Irr`, `Area`, and profile-storage fields.
- `pw0.slp` declares 15 slope segments, but no `pw0.wat.dat` exists in the legacy output directory; M-A has no 15-OFE WAT closure surface to calibrate.

## Diagnostic formula

Per-OFE row diagnostic residual:

`RM + Irr + UpStrmQ + SubRIn - Q - Ep - Es - Er - Dp - latqcc - Tile - delta(Total-Soil + frozwt + Snow-Water)`.

Area-weighted daily diagnostic residual:

Same row formula, weighted by `Area` and aggregated by H/Y/J before subtracting the area-weighted daily storage delta.

These diagnostics are calibration aids only. They are not acceptance thresholds and do not supersede science contracts.

## Legacy inventory

| OFE count | Hillslopes | Rows per hillslope | Total rows |
| --- | --- | ---: | ---: |
| 1 | H8, H15, H19, H20, H22, H23, H28 | 2,192 | 15,344 |
| 2 | H11, H13, H16, H33, H36 | 4,384 | 21,920 |
| 3 | H6, H12, H14, H29, H30 | 6,576 | 32,880 |
| 4 | H9, H25, H32 | 8,768 | 26,304 |
| 5 | H1, H2, H3, H4, H5, H7, H10, H17, H18, H21, H24, H26, H27, H31, H34, H35 | 10,960 | 175,360 |

Total parsed legacy WAT rows: 271,808.

## Legacy flow-field calibration

| OFE count | Downstream `UpStrmQ` max | Nonzero downstream `UpStrmQ` rows | Downstream `SubRIn` max | Nonzero downstream `SubRIn` rows | `QOFE-Q` max abs | `latqcc` max |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 0.0 | 0 | 0.0 | 0 | 0.0 | 47.0 |
| 2 | 64.79408 | 17 | 44.63 | 9,663 | 84.64425 | 55.43 |
| 3 | 175.5169 | 554 | 61.28 | 19,401 | 179.9954 | 61.28 |
| 4 | 149.1972 | 337 | 49.92 | 18,522 | 193.9773 | 49.92 |
| 5 | 397.8088 | 9,308 | 53.48 | 131,771 | 405.8212 | 53.48 |

Global maxima:
- `UpStrmQ`: 397.8088 mm.
- `SubRIn`: 61.28 mm.
- `QOFE`: 507.2765 mm.
- `Q`: 101.4553 mm.
- `latqcc`: 61.28 mm.
- `Irr`: 0.0 mm in this cohort.

## Legacy adjacency handoff check

Depth-level same-day adjacency checks compare current OFE `UpStrmQ` to previous OFE `QOFE`, and current OFE `SubRIn` to previous OFE `latqcc`.

| OFE count | OFE-pairs | Surface handoff max abs | Surface p95 abs | Subsurface handoff max abs | Subsurface p95 abs |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 2 | 10,960 | 0.0 | 0.0 | 0.0 | 0.0 |
| 3 | 21,920 | 0.000010000000003174137 | 0.0 | 0.0 | 0.0 |
| 4 | 19,728 | 0.000010000000003174137 | 0.0 | 0.009999999999999787 | 0.0 |
| 5 | 140,288 | 0.00010000000003174137 | 0.0 | 0.010000000000001563 | 0.0 |

Interpretation: legacy WAT rows preserve immediate upstream/downstream surface and subsurface handoff to printed-output precision. The nonzero maxima are formatting precision, not material routing drift.

## Per-OFE row residual calibration

| OFE count | Rows after first storage row | Max abs residual | p95 abs residual | Mean abs residual | Worst row |
| ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 15,337 | 77.0 | 24.89 | 4.2395224707309165 | H23 OFE1 2005-J17 residual -77.0 |
| 2 | 21,910 | 132.12983000000003 | 24.900000000000134 | 4.345853895031271 | H11 OFE2 2002-J103 residual 132.12983000000003 |
| 3 | 32,865 | 227.91043999999985 | 25.90999999999996 | 4.669451211106959 | H6 OFE3 2002-J103 residual 227.91043999999985 |
| 4 | 26,292 | 232.47402000000008 | 26.7254894499999 | 5.002818179847416 | H25 OFE4 2002-J103 residual 232.47402000000008 |
| 5 | 175,280 | 438.3034999999999 | 32.70431899999994 | 6.647843416982245 | H21 OFE5 2004-J345 residual 438.3034999999999 |

## Area-weighted daily residual calibration

| OFE count | H-days after first storage day | Max abs residual | p95 abs residual | Mean abs residual | Worst day |
| ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 15,337 | 77.0 | 24.89 | 4.239522470730917 | H23 2005-J17 residual -77.0 |
| 2 | 10,955 | 89.80787499999994 | 24.890000000000057 | 4.342208782297358 | H11 2002-J103 residual 89.80787499999994 |
| 3 | 10,955 | 135.51533666666666 | 26.196666666666648 | 4.660891382827633 | H6 2002-J103 residual 135.51533666666666 |
| 4 | 6,573 | 128.58062749999993 | 27.278633399999812 | 4.981886424773625 | H32 2002-J164 residual 128.58062749999993 |
| 5 | 35,056 | 236.23673400000004 | 34.199742849999915 | 6.589447312078018 | H7 2002-J103 residual 236.23673400000004 |

## Calibration conclusion

Legacy per-OFE WAT output provides a strong routing-shape calibration:
- OFE1 has no upstream surface or subsurface runon.
- Downstream OFEs receive previous OFE `QOFE` as `UpStrmQ` and previous OFE `latqcc` as `SubRIn` to printed precision.
- `QOFE` is not an alias for `Q` on multi-OFE slopes; it is the single-OFE-scaled runoff publication needed for downstream handoff.

The row residual maxima increase with OFE count and runoff events, so they should be used as localization signals, not as openWEPP acceptance tolerances.
