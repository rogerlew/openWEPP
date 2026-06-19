# PERFMIG01 Disposition

Evidence: Static + Ran.

## Verdict

`CONTINUE`.

PERFMIG01 is the first production rung of ADR-0023. It is identity-clean, fail
closed for missing migrated ids, and exposes a real transition-boundary cost.
The H2637 endpoint regressed by `+3.15s` (`+0.47%`), but this was explicitly
allowed by the package for a single migrated phase when compatibility
materialization remains at the phase output edge.

## Numbers

| Evidence | Result |
| --- | --- |
| Migrated production branch | WB11 warm-rain runoff writeback |
| Migrated payload | 543 state + 8 flux id-backed updates |
| Focused bit identity | exact map equality and exact `f64::to_bits()` equality |
| H2637 output identity | `.hbp` and `wat.parquet` byte-identical; `pass.parquet` Arrow-equal ignoring metadata |
| H2637 endpoint | `669.97s`, `228144 KB`, rc `0` |
| PERFIDX06 endpoint | `666.82s`, `228508 KB`, rc `0` |
| Delta | `+3.15s`, `+0.47%`, `-364 KB` |
| Legacy no-UI ratio | `73.46x` vs PERFIDX06 `73.12x` |
| Transition apply boundary | `107.531649 us/payload`, projected `25.373275s` over H2637 OFE-days |

## Why This Is Continue, Not Redirect

- Identity is exact on the migrated branch.
- H2637 outputs remain semantically identical.
- The performance regression is small and lies inside the package's expected
  single-rung transition-boundary offset.
- The transition boundary is measured, named, and retireable as adjacent phases
  migrate.
- No production physics, science contract, HBP schema, parquet schema, or
  irrigation activation changed.

## Next Rung Outline

PERFMIG02 should migrate a contiguous WB11-consumer cluster instead of another
single isolated writer. The target is to keep WB11 runoff/infiltration/snow
runtime outputs dense across the immediate downstream hydrology consumers before
materializing back to logical surfaces.

Recommended objective:

- migrate the downstream consumers that read the PERFMIG01 WB11 outputs on
  ordinary warm-rain days;
- preserve exact identity with the existing H2637 and focused 543+8 fixtures;
- measure the endpoint and boundary cost again;
- retire or move outward one internal compatibility materialization boundary.

Do not claim H2637 endpoint closure until the measured endpoint moves toward the
`<=10x` viability gate. PERFMIG01 only proves the production pattern is
correct enough to continue.
