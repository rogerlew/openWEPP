# Fresh Hydrology and Ownership Re-Review

Evidence class: `Static + targeted Ran` exact-worktree review.

Reviewed the same exact contract/oracle candidate as the fresh science
re-review. Verdict: **NO-GO / FAIL**.

## Findings

### `OWN2-CRITICAL-001` — Frozen owner vectors do not instantiate strict schemas

Fixture water keys omit required requesting owner/component, occupancy,
surface, soil-layer and typed source identities, and use a different amount
field. Typed occupancy/layer custody is therefore not exercised.

### `OWN2-CRITICAL-002` — Arbitration loses complete resource identity

The calculator groups only by `source_id`; root layers are concatenated strings
without occupancy identity, and final root use is a caller-supplied scalar
minimum rather than the accepted joint capped V8 result. Same-named sources in
different OFEs can collide.

### `OWN2-CRITICAL-003` — Complete owner transaction and rollback are absent

Boundary owner candidates leave hydrology and soil thermal null, infiltration
energy names the wrong receiver, the five-owner envelope is incomplete, and
rollback hashes do not cover all owners and transaction state.

### `OWN2-CRITICAL-004` — Current-ingress thermal state is not updated

The rain/runon case reports a mixed liquid temperature while retaining the
same LSE enthalpy and temperature as the no-ingress case. Retained ingress must
update `U_s`; infiltration enthalpy must reach soil thermal.

### `OWN2-HIGH-005` — Schema ownership conditions are incomplete

The water key omits surface class; shared layer stores are incorrectly forced
into tile-ground identity; condensation credit has the wrong transaction
basis; and `minItems=5` does not require exactly one receipt for each owner.

### `OWN2-HIGH-006` — Multi-OFE routing is not executed

The heterogeneous fixture contains multiple tiles in one OFE, not a routed
multi-OFE owner transaction.

## Initial-Finding Reassessment

`OWN-CRITICAL-003` is corrected canonically, and the V8 model-identity impact
is correct. `OWN-CRITICAL-001`, `OWN-CRITICAL-002`, `OWN-CRITICAL-004`,
`OWN-CRITICAL-005`, `OWN-HIGH-006`, `OWN-HIGH-007` and
`OWN-CRITICAL-009` remain unresolved. `OWN-HIGH-008` is corrected in
contract/schema provider posture but lacks a complete executed positive
fixture. Equal/opposite ground-heat prose and a primitive receipt exist, but no
soil-thermal owner candidate closes the transaction.
