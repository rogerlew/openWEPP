# Pre-Implementation Contract Gate

Status: `passed`

Evidence mode: `Static`

Before production publication edits, record whether W6 changes public schemas,
process physics, or canonical science-contract obligations. If so, record
contract amendments and tests completed before production edits.

## Gate Result

Static: W6 publication implementation is schema-preserving and does not change
process physics. It does intentionally remove compatibility-stage fake
publication defaults from the public CLI path:

- No canonical `SC-*` amendment is required before the direct writer edit.
- No watershed routing, impoundment, sediment, erosion, runoff-partition,
  water-balance, latest-event, or hillslope physics changes are in scope.
- Public output file set and Arrow/Parquet schema constructors remain unchanged.
- Output column formulas remain the existing writer formulas recorded in
  `publication-operand-lineage.md`, with null emitted when a typed operand is
  unavailable.
- `chanwb` channel outflow, storage, baseflow, loss, and balance operands stay
  null in public watershed CLI frames unless explicit `m^3` channel-balance
  operands are present; W6 does not map impoundment outflow, routed runoff, or
  `cbase` into those fields.
- `area_m2` for W6 supervisor-generated publication frames comes from committed
  source hillslope slope geometry when every contributing hillslope has finite
  positive area; otherwise area-normalized fields are null.
- The implementation removes public CLI dependence on compatibility-shaped row
  seed staging and preserves the existing edge row writer only for current
  non-public aggregation callers.

Production edits may proceed under this gate.
