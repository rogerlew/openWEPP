# WA Sediment Attribution

Evidence mode: Ran.

## Surface Confirmation

- Member: `wa_cascades_forest_h1`
- Role: `fine_reference_adequacy_dt75`
- Candidate: `dx2p5_dt75`
- Reference: `dx1p25_dt75`
- Failing surface: `tdep:4`
- Prior annual max relative delta: `0.0221316837961`
- Recomputed year-4 relative delta: `0.0221316837961`
- One-third adequacy threshold: `0.00666666666667`
- Verdict: `FAIL` for the broader mesh-policy gate.

## Daily Attribution

Year 4 has exactly `1`
nonzero daily `tdep` delta day. The whole annual miss is day
`1126` / julian `30`:

| Surface | Candidate | Reference | Delta |
|---|---:|---:|---:|
| `tdep` kg | 0.610706965978 | 0.597483646833 | 0.0132233191451 |
| `tdet` kg | 529.314433146 | 529.444080612 | -0.129647465813 |
| pass `runvol` m3 | 1314.30970261 | 1314.30970261 | 0 |
| pass `sbrunv` m3 | 497.567305858 | 497.567305858 | 0 |
| pass `peakro` m3/s | 1.85463179197e-06 | 1.85463179197e-06 | 0 |

## Routed Trace Comparison On Day 1126

| Surface | Value |
|---|---:|
| aggregate source delta m3 | 0 |
| aggregate outlet delta m3 | 0.0182702561062 |
| aggregate end-storage delta m3 | 0.00335939712086 |
| aggregate tail-fold delta m3 | 0 |
| candidate clamp m3 | 0 |
| reference clamp m3 | 0 |
| max lane routed-shape L1 | 0.000741449015798 |
| terminal outlet delta m3 | -0.00335939708839 |
| terminal routed-shape L1 | 0.000635233567962 |

## Classification

`sediment response to sub-threshold routed-hydrograph shape perturbation`.

The failing annual sediment value is produced by a single low-mass erosion day. Pass-parquet daily water magnitude operands and active source mass are identical on that day. The routed hydrograph shape does differ, and that difference is a consumed water-timing input to the erosion path, but the prior rev-43 mesh-policy routed-water surfaces all passed: terminal outlet, routed shape, end-window storage, tail-fold, uniform-shape, and source-shape-degenerate counters. The package therefore classifies the blocker as a low-denominator annual sediment response to a sub-threshold routed hydrograph timing/shape perturbation, not as active-router numerics or daily water-magnitude drift.

## Follow-On

dx5 promotion remains blocked until a contract-authorized annual pass-sediment adequacy metric policy is adjudicated.

Next package: `20260708-laned-router-annual-sediment-adequacy-metric-authority-001`.

## Provenance

- Prior summary: `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/coupled-spacetime-summary.json`
- Prior mesh ratification: `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/mesh-policy-ratification.json`
- Release binary SHA-256:
  `8876fa04ca520126b958d83a7c5777da6f793e51fba4c346432f065b31647aaa`
