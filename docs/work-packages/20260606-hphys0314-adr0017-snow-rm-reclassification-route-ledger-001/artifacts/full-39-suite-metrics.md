# Full 39 Suite Metrics

Status: complete

Evidence mode: Static

Static:

- Metrics are carried forward from the most recent fixed-baseline full H1..H39
  semantic run because HPHYS0314 makes no production runtime edits.
- Source artifact:
  `docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/artifacts/fixed-baseline-semantic-metrics.md`.
- Runtime-impact check: HPHYS0314 touches contracts, tests, package docs, and
  evidence only. It does not touch production Rust kernel/runtime code.

Ran:

- Not rerun in HPHYS0314. Carry-forward is truthfully labeled as static reuse
  of same-runtime metrics.

## H1..H39 Fixed-Baseline Metrics

- Semantic pass hillslopes: `0/39`.
- Structural row/key failures: `0`.

| Column | Hillslope Failures | Row Failures | Mean Abs Diff Mean | Max Abs Diff |
|---|---:|---:|---:|---:|
| RM | 39 | 7097 | 0.256086 | 27.960000 |
| Snow-Water | 39 | 10391 | 2.899432 | 65.506840 |
| Total-Soil | 39 | 52185 | 56.010072 | 317.130129 |
| SoilWaterTotal | 39 | 52185 | 56.010072 | 317.130129 |
| Ep | 39 | 42688 | 0.633657 | 7.100844 |
| Es | 1 | 470 | 0.010140 | 1.828583 |
| Dp | 38 | 10961 | 0.050444 | 0.244800 |
| Q | 0 | 0 | 0.000000 | 0.000000 |
| latqcc | 39 | 38462 | 0.285882 | 3.023092 |

Interpretation: these remain higher-confidence single-OFE daily WAT
investigation signals under ADR-0011. HPHYS0314 does not use them as
implementation acceptance authority.
