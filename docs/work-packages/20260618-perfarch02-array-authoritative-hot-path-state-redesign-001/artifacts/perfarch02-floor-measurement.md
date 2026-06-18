# PERFARCH02 Floor Measurement

Evidence class: Ran locally on 2026-06-18.

## Inputs

- PERFIDX06 openWEPP H2637 no-UI endpoint: 666.82 seconds.
- PERFIDX06 legacy H2637 no-UI median: 9.12 seconds.
- H2637 OFE-days used for projection: 235,961.
- Prototype iterations per repeat: 200,000.
- Prototype repeats: 5.

## Timing Summary

| Metric | Mean seconds | Min seconds | Max seconds | Mean per iter | Projected H2637 | Ratio vs legacy |
|---|---:|---:|---:|---:|---:|---:|
| `logical_current` | 6.560824278 | 6.503200068 | 6.655355931 | 0.000032804121 | 7.740493287 s | 0.848738x |
| `array_authoritative` | 0.131439351 | 0.127611155 | 0.139069593 | 0.000000657197 | 0.155072804 s | 0.017004x |
| `export_once` | 0.001008919 | 0.001008919 | 0.001008919 | 0.001008919 | 0.001008919 s | 0.000111x |

The measured replacement path for this writeback/guard surface is about 49.9x
faster than the current logical writeback/guard path.

## Per-OFE-Day Budgets

| Quantity | Value |
|---|---:|
| Current openWEPP no-UI per OFE-day | 2,825.975479 microseconds |
| Legacy no-UI per OFE-day | 38.650455 microseconds |
| <=10x target budget per OFE-day | 386.504549 microseconds |
| <=5x target budget per OFE-day | 193.252275 microseconds |
| Prototype array writeback/guard cost per iteration | 0.657197 microseconds |

The candidate writeback/guard surface is well below both the 10x and 5x
per-OFE-day total budgets. That does not prove total H2637 performance because
the prototype does not execute the complete WB11/runner path. It proves that
the representation replacement is not itself the floor.

## Full-Run Extrapolation

The PERFIDX06 endpoint needs large total removal:

| Removal from 666.82 s | Residual seconds | Ratio vs 9.12 s legacy |
|---:|---:|---:|
| 86.323% | 91.200 | 10.000x |
| 89.000% | 73.350 | 8.043x |
| 90.000% | 66.682 | 7.312x |
| 93.000% | 46.677 | 5.118x |
| 93.162% | 45.600 | 5.000x |

PERFARCH01 estimated roughly 89-90% of elapsed time as removable
string-keyed surface machinery. PERFARCH02's prototype supports the direction
of that estimate for the writeback/guard seam. It does not independently prove
that every member of the 89-90% class converts to array cost inside the real
WB11 control flow.

## Interpretation

The measured floor for the prototyped representation class is low enough to
justify a downstream integrated pilot. The realistic target remains <=10x until
the integrated pilot measures actual WB11 daily work, conservation checks,
consumer-boundary validation, persistent scheduler state movement, and
publication materialization together.

The 5x target remains unproven. It requires about 93.16% removal from the
PERFIDX06 endpoint, leaving only 45.6 seconds for all legitimate openWEPP work.
That may be possible only if nearly all profiler-attributed symbol machinery,
scheduler surface movement, and guard formatting exits the hot path.
