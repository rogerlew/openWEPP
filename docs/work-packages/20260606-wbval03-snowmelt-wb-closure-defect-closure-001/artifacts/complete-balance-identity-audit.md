# Complete Balance Identity Audit

Status: complete

Evidence mode: `Ran`

Ran:

- Read saved WBVAL01 WAT parquet files from:
  `/tmp/wbval01_rocky_mountain_20260606T000000Z/nodiscovery/<p>/H<n>.wat.parquet`
- Audited the 12 WBVAL01 prior WAT emitters:
  `p1`, `p3`, `p5`, `p8`, `p10`, `p12`, `p13`, `p15`, `p16`, `p19`, `p21`,
  and `p22`.

Complete identity used:

```text
R = (P + Irr + UpStrmQ + SubRIn)
    - (Q + Ep + Es + Er + Dp + latqcc + Tile)
    - delta(SoilWaterTotal + Snow-Water + InterceptionStorage)
```

Audit findings:

- `UpStrmQ`: zero for all audited rows.
- `SubRIn`: zero for all audited rows.
- `Tile`: zero for all audited rows.
- `InterceptionStorage`: unpopulated (`null`) for all audited rows; treated as
  zero contribution because no start/end storage values exist.
- `SoilWaterTotal` exactly equals `Total-Soil + frozwt`; adding `frozwt` again
  would double-count frozen profile water.
- The complete identity therefore reproduces the WBVAL01 residuals exactly for
  years `2..6`.

Max absolute residual by hillslope under the complete identity:

| Prefix | Max abs full-year `R` (`mm`) |
|---|---:|
| `p1` | 49.153 |
| `p3` | 51.689 |
| `p5` | 58.848 |
| `p8` | 51.440 |
| `p10` | 49.839 |
| `p12` | 49.774 |
| `p13` | 47.431 |
| `p15` | 51.307 |
| `p16` | 58.961 |
| `p19` | 78.740 |
| `p21` | 54.229 |
| `p22` | 61.861 |

Conclusion:

- The emitted-ledger residual is real under the complete declared identity.
- Current post-WBVAL02 execution cannot regenerate WAT ledgers for validation
  because all 12 prior emitters now fail earlier at `radly=486`.
- WBVAL03 cannot safely attribute or fix the residual until the upstream
  climate source boundary is closed.
