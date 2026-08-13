# Pre-Implementation Contract Gate

Status: `PASS`

Evidence mode: `Static + Ran`

- `SC-VEGETATION-001@8` owns the exact V4 state, LAI, cache, migration, guard,
  invariant, and fixture obligations.
- `OPENWEPP_C3_WOODY_V4` imports immutable V3 by exact digest and binds the V8
  amendment plus independent fixture/generator identities.
- V1/V2/V3 definition bytes remain protected.
- Production Rust is outside this authority package and remains fail-closed.
- The reference calculator ran independently and deterministically; exact
  commands/digests are in `gate-results.md`.

No implementation work is authorized from package prose alone; the canonical
contract and V4 definition are the binding authority.

