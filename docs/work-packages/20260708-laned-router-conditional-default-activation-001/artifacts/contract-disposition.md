# Contract Disposition

Status: `COMPLETE`
Evidence mode: Static.

## Amendment

`SC-OFEROUTE-001` is amended to rev 46 for conditional Lane D default
activation.

Binding selector policy:

- complete scheduled-lane native `routing_coefficients` -> attach active Lane
  D by default;
- no scheduled-lane native `routing_coefficients` -> remain legacy/off;
- mixed coefficient-present and coefficient-absent lanes -> fail closed before
  streaming;
- explicit `OPENWEPP_LANED_ACTIVE=1` keeps the complete-coefficient
  fail-closed precondition;
- explicit `OPENWEPP_LANED_ACTIVE_DISABLE=1` forces legacy/off and conflicts
  with explicit active.

## Surfaces Updated

- Front matter: `contract_version: 46`.
- Scientific scope: conditional default activation is in scope; universal
  activation without coefficient authority remains out of scope.
- Branch and guard table: added the conditional default activation row and the
  active-disable rollback selector row.
- `INV-OFEROUTE-010`: rewritten from opt-in/default-blocked posture to the
  all/none/mixed default-eligibility invariant.
- Guard map and test-vector obligations: added default all-coeff, no-coeff,
  mixed, explicit active missing-coeff, active+disable, and active+shadow
  vectors.
- BEI: activation validation records rev 46 conditional default posture, and
  active mesh policy records selector-only change.
- Revision history: added rev 46 row.

## Non-Changes

- No mesh-policy change; rev 45 `target_dx_m = 5.0` active default is retained.
- No routed-shape, annual sediment, or active closure tolerance change.
- No sediment process-physics or watershed outlet/HBP re-pointing change.
- Historical D15/D16 text remains as provenance; a rev 46 status note records
  that `INV-OFEROUTE-010` now supersedes old selector-blocked wording.
