# Implementation And Test Evidence

Status: `EXECUTING / Increment 2A internal column engine pass; public E04 pending`

Evidence mode: `Static + Ran`

The executable state surface now implements V2 identity, configuration,
occupancy-local state, canonical digest binding, transaction lineage, and
offline V1/RHESSys migration. Historical shared liquid and hydraulic warm
starts are reachable only through the explicitly named V1 migration DTO.

The public candidate validates complete V2 state and then returns a typed
implementation-incomplete error before mutation or publication. Disabled V1
transaction physics was removed rather than retained beside the V2 state.
Internal E04 tile-column routing is now implemented through an
`OccupancyPassSolver` seam. It consumes immutable V2 state, derives conditional
plant area, routes only within a tile, exposes authoritative water operands,
and independently rejects closure/identity/basis poisons. It also proves that
an injected descendant failure cannot mutate any beginning lane.

The callback in this increment is controlled test machinery, not production
physiology. Exact potential and capped E11--E15 occupancy solves, hydrology
arbitration, owner candidates, the public E04 path, and commit remain pending.
