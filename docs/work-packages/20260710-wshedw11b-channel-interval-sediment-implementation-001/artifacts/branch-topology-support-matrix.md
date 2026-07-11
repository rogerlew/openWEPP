# Branch and Topology Support Matrix

Status: `EXECUTED`

Evidence mode: `Static` from `SC-ROUTE-001` v53 and pinned source.

| Branch/topology | Interval-lane predicate/result | Required W11B behavior |
|---|---|---|
| `ipeak = 1` Rational | False: no wave grid | Preserve event-scalar/minor-0 behavior; hourly paired input remains non-activated. |
| `ipeak = 2` CREAMS | False: no wave grid | Preserve event-scalar/minor-0 behavior. |
| `ipeak = 3` kinematic | True when all inlet/dependency authority is complete | Route water and sediment on normalized `dtchr` grid. |
| `ipeak = 4` static MC | Same | Route with static Muskingum-Cunge coefficients. |
| `ipeak = 5` variable MC | Same | Refresh dynamic coefficients per interval. |
| `ipeak > 5` | Same canonical `ipeak >= 4` family | Static MC semantics; supersedes stale held-W11 fail-closed row. |
| Leaf channel, all paired HBP | Active for `ipeak >= 3` | Exact-overlap project `V_h/S_h`; publish typed interval water/classes. |
| Active channel -> channel | Downstream dependency has hourly authority | Consume upstream same-index water and per-class egress directly. |
| Local paired HBP + active upstream channel | Active | Additive same-grid intake; preserve source lineage. |
| Any impoundment dependency | Inactive/excluded | Do not claim interval sediment routing; existing typed branch applies. |
| Mixed paired/unpaired hillslopes | Invalid | Typed domain failure; never collapse to scalar. |
| Active upstream + downstream unpaired contributor | Invalid mixed authority | Typed domain failure. |
| Malformed pair or non-covering grid | Invalid | Typed `WKERNEL-WS10-CHANNEL-E-003` family failure. |
| No paired hourly authority | Inactive | Existing event-scalar lane remains protected. |

Activation is evaluated in dispatch order; an active upstream channel's typed
same-grid output is the only dependency form satisfying `INV-ROUTE-005(a)`.
