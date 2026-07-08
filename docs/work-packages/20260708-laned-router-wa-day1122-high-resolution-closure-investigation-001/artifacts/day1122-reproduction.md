# Day-1122 Reproduction

Status: EXECUTED
Evidence mode: Ran.

## Source Evidence

- Summary JSON: `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/mesh-ladder-summary.json`
- Summary SHA256: `de3c90c61277504428dafb51138e0149a494b930fa1ae9c78f419172f2e48bbf`

## Climate and Mesh

| Surface | Value |
|---|---:|
| Date | 2003-01-26 |
| Precipitation mm | 38.4 |
| Duration h | 9.3 |
| Peak-intensity input | 6.1 |
| OFE lengths m | 108.34, 108.34, 108.34, 108.34, 108.34 |

## Completed Rung Day Books

| Rung | Rows | Source m3 | Clamp m3 | Terminal outlet m3 | Mesh storage m3 | Residual m3 | Relative |
|---|---:|---:|---:|---:|---:|---:|---:|
| baseline_fixed10 | 5 | 4889.32122696 | 25.4271138404 | 4901.81626622 | 12.932074575 | 6.79278855387e-12 | 1.38931116172e-15 |
| dx20 | 5 | 4889.32122696 | 25.4271138404 | 4901.81626622 | 12.932074575 | 6.79278855387e-12 | 1.38931116172e-15 |
| dx10 | 5 | 4889.32122696 | 35.7462846486 | 4912.14021257 | 12.9272990385 | 2.07123207474e-12 | 4.23623643978e-16 |
| dx5 | 5 | 4889.32122696 | 554.609197907 | 5431.03005677 | 12.9003680932 | -3.40705241797e-12 | 6.96835462392e-16 |

## Failing Fine Rungs

| Rung | Day | Guard | Injected m3 | Clamp m3 | Outlet m3 | Mesh storage m3 | Residual m3 | Litres | Relative | Tolerance | Wall | User |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| dx2p5 | 1122 | `laned_active_day_cascade_residual` | 4889.32122696 | 857952342.299 | 544903.223584 | 857412328.397 | -0.000110030174255 | 0.110030174255 | 2.25041818993e-08 | 1e-09 | 2:17.35 | 137.32 |
| dx1p25 | 1122 | `laned_active_day_cascade_residual` | 4889.32122696 | 190055300.17 | 144351896.937 | 45708292.5541 | 1.10864639282e-05 | 0.0110864639282 | 2.26748528346e-09 | 1e-09 | 10:12.63 | 612.53 |

## Interpretation Inputs

- The first failing guard is the active day cascade residual. The code
  returns at that guard, so the logs do not prove whether later seam or
  identity checks would pass on the failed fine rungs.
- The absolute residuals are sub-litre to decilitre scale, but the guard is
  relative to injected source volume while clamp/storage operands are
  eight to nine orders larger than the residual.
