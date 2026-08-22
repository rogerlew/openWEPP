# Contract impact and preimplementation gate

Status: `PASS / IMPLEMENTATION MAP; PROSPECTIVE LOWER-BOUNDARY CLARIFICATION OPEN`.

Required gate: prove that Child 2C already binds canopy temperatures and
humidity, leaf/stem/wet-surface conductances, shared canopy-air state, snow
temperature/humidity/roughness/emissivity, reference atmosphere, wind
exposure, reciprocal longwave, support identity, and mass/vapor/energy ledger
operands. A prospective amendment is permitted only for one exact unbound
field.

Disposition: the released Child 2C amendments bind the required physical
operands and consumer obligations. The current LSE contract does not yet
explicitly publish the V11 canopy-only lower-boundary operator mask, so the
new typed seam remains default-off/shadow evidence and a prospective,
single-field contract clarification is open before release. The
implementation must still prove each binding at the actual consumer call and
must not use the package artifact as a physics substitute.

Primary bindings read at this gate:

- `SC-SNOWENERGY-001` Child 2C `INV-SNOWENERGY-036..040` and
  `OBL-SNOWENERGY-P-010/C-017`;
- `SC-VEGETATIONTRANSACTION-001` Child 2C `INV-VEGTRANSACTION-014..017` and
  `OBL-VEGTRANSACTION-P-003/C-005`;
- `SC-COUPLEDTIME-001` Child 2C `INV-COUPLEDTIME-017..020` and the
  `EventBoundaryCoalescingReceiptV1` fields;
- `SC-SNOWFREEFORCING-001` `INV-SFF-001..012` for the 48 exact half-hour
  supports and sealed atmospheric/provider identity;
- `SC-LANDSURFACEENERGY-001` for the snow-free successor only; its
  snow-present branch remains delegated and must not be entered by the
  covered adopter.
