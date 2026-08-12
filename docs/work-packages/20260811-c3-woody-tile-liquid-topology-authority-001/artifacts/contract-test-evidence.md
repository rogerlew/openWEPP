# Contract Test Evidence

Status: `focused authority suite passing`

Evidence mode: `Ran`

`cargo nextest run --test vegetation_boundary_authority_contract --profile quick`
initially failed three stale V1 lifecycle string assertions, then one line-wrap
sensitive stemflow assertion. These were test reconciliation defects, not
contract defects. After updating assertions to canonical V2 authority and stable
invariant text, the suite passed `14/14`.

The suite now separately proves immutable V1 digest, exact V2 digest/section
hashes, invariants 073--079, committed independent topology vectors, nonlinear
aggregate poison separation, and typed migration disposition. It parses the
fixture, requires the exact 31-case inventory, reconstructs every occupancy
liquid residual and weighted stand closure, verifies water/N keys and bounds,
checks two-pass column evidence, compares all-owner rollback digests, recomputes
canonical state serialization and SHA-256, and rejects aggregate-first
wetness/FvCB and duplicate C/N results.
