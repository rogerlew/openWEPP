# Snowpack Localization Evidence

Status: complete
Evidence mode: Static + Ran

## Static: Localized Defect

- HPHYS0283 retained material H1/H7/H39 snowpack after baseline meltout.
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-015` captured corrected routed-melt scaling but did not explicitly capture the companion corrected carried-depth adjustment from `/workdir/wepp-forest/src/winter.for` lines 441-460.
- Pre-fix openWEPP recomputed runtime SWE from routed net melt alone, delaying spring meltout by the same order of magnitude as cumulative negative hourly melt terms.

## Ran: Targeted Trace

- Trace root: `/tmp/hphys0284_springtrace_20260604T182506Z`.
- H1 trace: `/tmp/hphys0284_springtrace_20260604T182506Z/hillslope_output/H1.hphys0284.trace.jsonl`.
- H7 trace: `/tmp/hphys0284_springtrace_20260604T182506Z/hillslope_output/H7.hphys0284.trace.jsonl`.
- H39 trace: `/tmp/hphys0284_springtrace_20260604T182506Z/hillslope_output/H39.hphys0284.trace.jsonl`.

## Ran: Spring 2014 Selected Rows

| Case | Baseline Snow-Water | HPHYS0283 Snow-Water | HPHYS0284 Snow-Water | Baseline Total-Soil | HPHYS0283 Total-Soil | HPHYS0284 Total-Soil |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| H1 J144 | 30.500 | 97.103 | 9.492 | 619.120 | 312.376 | 312.735 |
| H1 J145 | 0.000 | 61.263 | 0.000 | 645.560 | 343.986 | 317.999 |
| H1 J146 | 0.000 | 24.237 | 0.000 | 643.470 | 378.726 | 313.934 |
| H7 J145 | 28.640 | 85.314 | 11.853 | 586.650 | 262.109 | 262.122 |
| H7 J146 | 0.000 | 48.472 | 0.000 | 611.940 | 296.668 | 271.692 |
| H39 J144 | 27.500 | 95.207 | 5.033 | 563.240 | 272.078 | 272.110 |
| H39 J145 | 0.000 | 59.304 | 0.000 | 580.470 | 303.333 | 272.495 |
| H39 J146 | 0.000 | 22.218 | 0.000 | 571.910 | 337.682 | 267.973 |

## Disposition

- Snow meltout timing is materially closer: H1/H39 reach zero by Julian 145 and H7 by Julian 146, matching baseline meltout days for those selected rows.
- Remaining storage residual worsens after snow correction; continuation should focus on runoff/infiltration/soil-storage partition under spring melt/runoff, not another snow-retention compensation.
