# Terminal Verification A

Status: `PASS / no open findings`

Evidence mode: `Static + Ran`

The fresh read-only Rust verifier confirmed one authoritative snow calculation,
two immutable linked ledgers, exact Stage-3 handoff, centralized closure
tolerances, capture-independent physics and guards, typed error preservation,
real schema-v4 writer consumption, and no physics/default/fixture/output
change.

Independent evidence reproduced the exact candidate release SHA
`4e0ebd96...da47`, schema-v4 trace SHA `84a64c1b...fb5f`, WAT SHA
`e74b8df2...2b75`, HBP/PASS SHA `d5d3468d...149b`, both closure maxima, all
`8615` Stage-3 links, performance/storage bounds, footprint ceilings,
assurance state, prompt hash, and anticipated `103`-path manifest hash.
Focused current-tree gates passed; retained quick `2172/2172`, frost
`352/352`, full `2221/2221`, and doctest logs were authenticated.

Residual risk is limited to the documented single-fixture performance scope
and 48-byte layout headroom. No missing package-scoped test or closure blocker
was found.
