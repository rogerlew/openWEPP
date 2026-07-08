# Contract Design

Status: queued placeholder.

Execution must decide whether to create `SC-GWBASEFLOW-001` or amend an
existing contract. The default expectation is a new process contract because
`SC-INFILE-GWCOEFF-001` owns parsing, not groundwater/baseflow process physics.

## Required Contract Surfaces

- State:
  - groundwater storage.
  - deep-percolation recharge.
  - baseflow volume/flux.
  - deep seepage volume/flux.
- Coefficients:
  - `igwstrd`.
  - `bfcoeff`.
  - `dscoeff`.
  - `bftharea`.
  - `lr_bf`.
- Boundaries:
  - single-OFE execution.
  - Lane D MOFE aggregation.
  - active surface-router ownership boundary.
  - HBP/watershed handoff.
  - `chan.inp` `cbase` separation.
- Guards:
  - missing/malformed `gwcoeff.txt`.
  - coefficient domain violations.
  - mixed authority across Lane D MOFE lanes.
  - double-feed between routed surface water and groundwater/baseflow export.
- Obligations:
  - contract-derived unit tests.
  - single-OFE daily reservoir vector.
  - Lane D MOFE groundwater/baseflow accounting vector.
  - watershed/channel consumer proof in M-T2B or later M-T3.

## Non-Goals

- Do not tune coefficients.
- Do not infer default groundwater parameters.
- Do not add production formulas not backed by literature plus baseline code.
- Do not let `latqcc`, groundwater baseflow, and `cbase` collapse into one
  "baseflow" bucket.
