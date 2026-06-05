# HPHYS0305 Full-39 Suite Metrics Context

Static:

- HPHYS0305 adds diagnostic trace/observe surfaces only; it does not change production physics or WAT publication math.
- Full H1..H39 fixed-comparator semantic metrics are carried forward from HPHYS0304 as same-physics context.

Ran:

- HPHYS0305 did not rerun the full H1..H39 semantic suite; targeted H1/H7/H39 paired traces were run.

## Carried HPHYS0304 Metrics

# Fixed-Baseline Semantic Metrics

Status: complete

Evidence mode: ran

Static:

- ADR-0016 makes the fixed `wepp_260430` comparator the active H1..H39 baseline artifact source.
- Candidate openWEPP parquets are reused only after runtime-source diff validation.

Ran:

- Semantic pass hillslopes: `0/39`.
- Structural row/key failures: `0`.

## Focus Columns

| Column | Hillslope Failures | Row Failures | Mean Abs Diff Mean | Max Abs Diff | Fail Delta vs HPHYS0302 |
| --- | ---: | ---: | ---: | ---: | ---: |
| RM | 39 | 7097 | 0.256086 | 27.960000 | 0 |
| Snow-Water | 39 | 10391 | 2.899432 | 65.506840 | 0 |
| Total-Soil | 39 | 52185 | 56.010072 | 317.130129 | 0 |
| SoilWaterTotal | 39 | 52185 | 56.010072 | 317.130129 | 0 |
| Ep | 39 | 42688 | 0.633657 | 7.100844 | 0 |
| Es | 1 | 470 | 0.010140 | 1.828583 | 0 |
| Dp | 38 | 10961 | 0.050444 | 0.244800 | 0 |
| Q | 0 | 0 | 0.000000 | 0.000000 | 0 |
| latqcc | 39 | 38462 | 0.285882 | 3.023092 | 0 |

Interpretation: these are higher-confidence single-OFE daily WAT investigation signals under ADR-0011. They do not by themselves identify term-level melt producer defects.
