# Prior-Day Snow Carry Divergence Summary

Status: complete

Evidence mode: ran

## Counts

- Affected HPHYS0309 rows represented: `58`
- Hillslope/window/year groups: `7`
- Production edit authorized groups: `0`

## Route Counts

- `density-settling-carry-state-hold`: `1`
- `initial-carry-state-projection-hold`: `6`

## Group Routes

| Hillslope | Window | Year | Rows | First Divergence | Route |
|---|---|---:|---:|---|---|
| H1 | first-abs-storage-ge-10mm | 2013 | 1 | 2013-011 h11 delta_depth=-0.000741865 m | `density-settling-carry-state-hold` |
| H1 | spring-2014 | 2014 | 8 | 2014-001 h01 delta_depth=0.0131443 m | `initial-carry-state-projection-hold` |
| H1 | spring-2016 | 2016 | 15 | 2016-001 h01 delta_depth=0.00261136 m | `initial-carry-state-projection-hold` |
| H7 | spring-2014 | 2014 | 7 | 2014-001 h01 delta_depth=0.0152795 m | `initial-carry-state-projection-hold` |
| H7 | spring-2016 | 2016 | 9 | 2016-001 h01 delta_depth=0.00360337 m | `initial-carry-state-projection-hold` |
| H39 | spring-2014 | 2014 | 9 | 2014-001 h01 delta_depth=0.0147979 m | `initial-carry-state-projection-hold` |
| H39 | spring-2016 | 2016 | 9 | 2016-001 h01 delta_depth=0.00248989 m | `initial-carry-state-projection-hold` |

## Interpretation

HPHYS0310 localizes the HPHYS0309 carry deficit to paired
snow-episode carry-state divergence: six groups diverge at day-1
hour-1 initial carry-state projection, and one group diverges
during early density/settling carry-state evolution. This is still
carry-state producer lineage, not branch-predicate or downstream
water-balance edit authority. The next package should compare the
initial state projection, density, and depth-update equations at
the first divergent hours against fixed-comparator `snowd.for`
source lines before modifying production code.
