# Operand Lineage

Status: `MAPPED / CONSERVATION GATE PARTIAL; EXECUTED HOLD`

The implementation run must populate a field-level lineage table for sealed
forcing, projected exposure wind, canopy/snow geometry, shared-air state,
canopy and snow turbulent terms, reciprocal longwave, snow mass, retained and
new liquid, soil/hydrology operands, event ticks, support receipts, and
publication fields. Each row must name units, normalization, authority,
consumer, owner, and rejected aliases.

Acceptance requires independent mass, signed-vapor, energy, longwave,
event-time, and output reconstruction. A producer total restated as its own
closure is supporting evidence only, not acceptance.

| Operand | Units | Authority/source | Normalization | Real consumer/owner | Rejected alias |
| --- | --- | --- | --- | --- | --- |
| sealed exposure wind, height, roughness | m s-1, m, m | `SC-SNOWENERGY-001` exposure receipt | exact receipt identity; no raw 10 m wind or floor | shared carrier → Stage 3 snow/V11 canopy | raw 10 m wind, fixed attenuation |
| reference/canopy/snow temperature and humidity | K, kg kg-1 | shared-carrier equations in `SC-SNOWENERGY-001` | finite domain, one shared node | carrier transaction | independent canopy-air nodes |
| heat/vapor conductances | m s-1 | admitted virtual geometry | finite positive conductance | reference, V11 canopy, Stage 3 snow ledgers | copied or scaled conductance |
| canopy LW components and sky fraction | K, dimensionless, W m-2 | reciprocal LW rule in `SC-SNOWENERGY-001` | component weights sum within released tolerance | canopy/snow shared LW ledger | stale canopy term, single-node LW |
| snow solid/liquid ledgers | kg m-2 | Stage 3/CoE custody and `SC-SNOWENERGY-001` | exact debit-credit join; no dropped remainder | snow and surface-liquid owners | diagnostic melt, rain alias, vapor alias |
| signed vapor export | kg m-2 s-1 / kg m-2 | shared-carrier sign convention | independent reference-minus-surface reconstruction | snow and canopy ledgers | unsigned magnitude |
| event ticks and support | ns / duration bits | `SC-COUPLEDTIME-001` | canonical decimal ticks; max active support | parent chronology receipt | elapsed-float interpolation |
| event identity and replay ordinal | parent/segment IDs, ordinal, digest | terminal event input and receipt | nonempty IDs, candidate-set digest, contiguous ordinal | handoff/restart runtime | replayed or reordered event |
| terminal event errors | kg m-2, kg m-2, J m-2, ns | independently recomputed candidate states | explicit zero-tolerance and deterministic tie rank | event-boundary receipt | producer-reported accepted flag |
| post-event LSE duration | ns | `SC-LANDSURFACEENERGY-001` | `0` or exact admitted support; snow-free operands only | LSE/hydrology/soil-thermal owners | sub-minimum solve, scaled full-bin forcing |
| complete owner bytes | produced typed V11 owner-envelope state bytes | `SC-VEGETATIONTRANSACTION-001` V11 manifest | opt-in stack produces the candidate; receipt joins per-owner digests before staged commit; live custody remains unclosed | one parent transaction | callback-supplied opaque bytes on the legacy opt-in method |
| publication row and transfer | typed direct-run output | existing `DirectFrameExecutor` path | only after parent/day commit | real scheduler consumer | shadow/reference publication |

Conservation acceptance independently reconstructs snow ice end, liquid end,
deposition-minus-sublimation vapor net, energy closure, reciprocal longwave
closure, event-time error, and publication owner/output identity. A producer
total alone cannot close any of these checks.
