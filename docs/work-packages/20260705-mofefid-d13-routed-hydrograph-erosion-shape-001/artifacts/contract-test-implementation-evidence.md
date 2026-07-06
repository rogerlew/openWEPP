# Contract-Test Implementation Evidence

Status: **COMPLETE** (Ran).

## Contract-Derived Tests

- `wave1_span_routed_hydrograph_shape_supersedes_dc01_weights`
  - Derives from `SC-SED-001#INV-SED-013` rev 53 and
    `SC-OFEROUTE-001#INV-OFEROUTE-008` rev 23.
  - Fixture supplies DC01 excess hours at `3,4,10,11` but a routed
    hydrograph shape with nonzero weights only at `3,4`.
  - Expected pre-D13 failure mode: Wave-1 would publish DC01-derived weights,
    not the routed candidate.
- `wave1_span_routed_hydrograph_shape_fails_closed_when_missing`
  - Derives from the missing-surface fail-closed clause in `INV-SED-013`.
  - Expected pre-D13 failure mode: no routed-hydrograph authority existed, so
    the missing-surface condition could not be represented.
- `wave1_span_routed_hydrograph_shape_fails_closed_when_nonclosing`
  - Derives from the unit-normalized positive-runoff shape requirement.
  - Expected pre-D13 failure mode: no routed-hydrograph authority existed, so
    non-closing routed shapes could not be rejected before erosion consumed
    them.

## Execution

Ran:
`cargo test -p openwepp-hillslope-orchestrator wave1_span_routed_hydrograph_shape -- --nocapture`

Result: pass, 3 passed.
