# Migration and Compatibility Disposition

Status: `selected`

Evidence mode: `Static`

- Every migration requires caller-supplied complete V2 numerical-state lanes
  with null transaction identity; no warm start is copied or synthesized.
- With those complete lanes, V1 zero liquid expands to exact zero for every
  occupancy.
- With those complete lanes, V1 with exactly one occupied tile maps
  `S_V2=S_V1/C_s`.
- Nonzero V1 store across multiple occupied tiles has no unique mapping and
  returns an exhaustive unresolved-lane report; callers must supply V2 lanes.
- No silent liquid distribution, warm-start copy/broadcast, reset, or parser
  default exists.
- V1 remains immutable and historical. V1 state cannot execute as V2 except
  through the explicit migration operation.
- The RHESSys adapter reports missing occupancy state and does not synthesize it.
