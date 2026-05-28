# WSHEDIMPL40 Muskingum-Cunge Gap Matrix

Status: complete  
Evidence mode: static  
Date: 2026-05-28

## Static
- Baseline authority references used in this package:
  - `/workdir/wepp-forest_260430_baseline/src/wshchr.for:307`
    (`q1(0,ichan) = q1(ntchr,ichan)` prior outflow continuity),
  - `/workdir/wepp-forest_260430_baseline/src/wshchr.for:255`
    (`qinich(ichan) = qin(ntchr)` prior inflow continuity),
  - `/workdir/wepp-forest_260430_baseline/src/wshchr.for:518` and `:563`
    (`c4 = 2.*qlavg*dxchr*dtchr*c0`),
  - `/workdir/wepp-forest_260430_baseline/src/wshchr.for:521-563`
    (`ipeak == 5` MVPMC3 dynamic coefficient refresh block).

| Gap ID | Baseline-authoritative behavior | Pre-WSHEDIMPL40 openWEPP behavior | Resolution in this package | Status |
| --- | --- | --- | --- | --- |
| `GAP-ROUTE-010` / `GAP-SYSTEM-009` (A) | Prior wave-state memory continuity is carried forward across events/runs (`qinich`, previous `q1`). | WS10 MC branch did not ingest prior published `ws10_channel_{id}_{qin,q1}` payloads; deterministic branch memory continuity was not explicit. | WS10 MC runtime now reads optional prior `qin/q1` symbols and uses them as previous-state terms with typed non-finite guards. | closed |
| `GAP-ROUTE-010` / `GAP-SYSTEM-009` (B) | Single-segment MC lateral term follows baseline lineage reduction: `c4 = 2 * qlat * dtchr * c0`. | WS10 MC runtime used `c4 = qlat`, missing baseline scaling lineage for current lane. | WS10 MC runtime updated to `c4 = 2 * qlat * dtchr * c0` and covered by contract-derived assertion. | closed |
| `GAP-ROUTE-010` / `GAP-SYSTEM-009` (C) | MC coefficient validity is finite/defined; coefficient sign is not itself a domain error. | Publication bounds forced `c1/c2/c3 >= 0`, introducing non-physical clamp semantics. | Runtime/publication now enforces finite-only coefficient validity and permits signed `c1/c2/c3` surfaces. | closed |
| `GAP-ROUTE-011` / `GAP-SYSTEM-010` | `ipeak = 5` branch recomputes MC coefficients during segment/time-step progression (`MVPMC3`). | WS10 MC branch executes one coefficient set per step and does not perform full dynamic refresh parity. | Not implemented in WSHEDIMPL40; captured as explicit follow-on gap. | promotable-with-risk |

## Ran
- not-applicable
