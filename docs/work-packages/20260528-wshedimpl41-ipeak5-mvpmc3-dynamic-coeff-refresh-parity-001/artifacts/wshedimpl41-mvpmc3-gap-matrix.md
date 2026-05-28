# WSHEDIMPL41 MVPMC3 Gap Matrix

Status: complete  
Evidence mode: static  
Date: 2026-05-28

## Static
- Baseline authority references used for WSHEDIMPL41 parity closure:
  - `/workdir/wepp-forest_260430_baseline/src/wshchr.for:521-563`
    (`MVPMC3` dynamic reference-flow and coefficient refresh lineage),
  - `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
    (`ipeak` branch selector lineage),
  - `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
    (WS11 orchestration call-chain lineage).

| Gap ID | Baseline-authoritative behavior | Pre-WSHEDIMPL41 openWEPP behavior | Resolution in this package | Status |
| --- | --- | --- | --- | --- |
| `GAP-ROUTE-011` | `ipeak=5` executes MVPMC3 dynamic coefficient refresh (`qref` lineage, geometry/depth solve, dynamic `c0..c4` recomputation). | `ipeak=5` reused static `ipeak=4` coefficient path in the current single-segment lane. | Added dedicated `ipeak=5` branch with dynamic reference-flow lineage, geometry/depth solve, dynamic `ckref/tk/cx`, and per-step `c0..c4` refresh under typed fail-closed guards. | closed |
| `GAP-SYSTEM-010` | System integration expects `ipeak=5` coefficient publication to reflect dynamic MVPMC3 refresh behavior, not static carryover. | Integration boundary emitted coefficients equivalent to static MC branch for `ipeak=5`. | Routed WS11 `ipeak=5` to dedicated runtime branch and verified coefficient divergence/sensitivity via contract-derived vectors. | closed |

## Ran
- not-applicable
