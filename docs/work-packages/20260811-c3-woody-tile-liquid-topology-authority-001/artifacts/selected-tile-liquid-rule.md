# Selected Tile-Liquid Rule

Status: `selected for canonical admission`

Evidence mode: `Static`

An occupancy is `o=(s,t)`. Each valid occupancy owns exactly one persistent
`S_liq,o`; no non-occupancy lane is allowed. `LAI_s` and `WAI_s` remain
stand-ground stratum state, with uniform conditional density
`LAI_s,t=LAI_s/C_s` and `WAI_s,t=WAI_s/C_s`.

E04 executes independently per occupancy using local incident liquid, beginning
store, conditional plant area, and local canopy temperature. Initial and second
drainage plus free throughfall continue to the next lower occupancy in that tile.
Stemflow routes directly to the same tile's ground recipient. Empty tiles route
rain directly to ground. Aggregation happens only after local routing.
