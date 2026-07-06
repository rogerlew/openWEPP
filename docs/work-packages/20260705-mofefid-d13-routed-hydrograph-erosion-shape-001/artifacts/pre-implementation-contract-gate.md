# Pre-Implementation Contract Gate

Status: **PASSED** (Static, 2026-07-06).

## Decision

D13 does not close in HOLD. The consumer-path authority can be closed inside
the package by adding an activation-candidate routed-hydrograph shape surface
that Wave-1 erosion consumes only when explicitly selected.

## Contract Amendments

- `SC-SED-001` rev 53:
  - narrows `REF-SED-DC01-SHAPE` to default/off and pre-active authority;
  - adds `REF-SED-LANED-ROUTED-HYDROGRAPH`;
  - updates `INV-SED-013` so active-routed-water Wave-1 consumes finite
    non-negative unit-normalized routed hydrograph weights, with typed
    fail-closed behavior for missing/malformed/non-closing shapes.
- `SC-OFEROUTE-001` rev 23:
  - adds the routed-hydrograph erosion-shape branch/guard row;
  - updates `INV-OFEROUTE-008`, `OBL-OFEROUTE-P-006`, and
    `OBL-OFEROUTE-C-004`;
  - adds the D13 test-vector obligation and
    `OFEROUTE-EROSION-ROUTED-HYDROGRAPH` BEI row.

## Boundary

The amended contracts authorize the erosion consumer switch only. They do not
authorize production/default Lane D activation, the routed-water producer flip,
DC01 disablement, D10 shock-numerics correction, D14 profiling/optimization, or
D15 default-promotion policy.
