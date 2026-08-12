# RHESSys Migration Adapter Disposition

Status: `MILESTONE 1 PASS / offline V2 reporting implemented`

Implement a versioned offline adapter inside `openwepp-vegetation::migration`. Its raw input vocabulary is separate from canonical serde fields. Output records source path/hash, mapping version, per-field provenance, canonical hash when complete, and an exhaustive ordered unresolved-field list. Caller supplements are explicit; neither RHESSys values nor sentinel spellings become runtime defaults or aliases.

Ran: the V2 adapter reports every required occupancy field in deterministic
order and never synthesizes a lane. The typed V1 state migration maps zero
liquid to every occupancy, applies `S_V2 = S_V1 / C_s` only for a single
occupied tile, and returns every unresolved `(stratum,tile,field)` for nonzero
multi-tile liquid. Complete results require caller-supplied warm starts and an
exact pre-bound initial-state digest.
