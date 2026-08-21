# Worker handoff

Status: released / default-off implementation package authorized

Evidence mode: Static + Ran

The subsequent package may implement only the default-off shared carrier and
actual V11 snow-covered segment after consuming:

- `SC-COUPLEDTIME-001@3`, including `EventBoundaryCoalescingReceiptV1`,
  canonical decimal-string ticks, active-participant maximum support, and
  `ERR-CT-021` atomic retry;
- `SC-LANDSURFACEENERGY-001@7`, including
  `LseSupportAdmissibilityReceiptV1`, post-event-only operands, and the
  covered-forest `dt >= 600000000 ns` pre-Newton guard;
- `SC-SNOWENERGY-001@14`, including the shared carrier equations, weighted
  component longwave, sealed exposure receipt, and no raw-10 m/fixed-
  attenuation fallback;
- `SC-VEGETATION-001@26` and `SC-VEGETATIONTRANSACTION-001@15`, including
  exact-once V11 flux lineage and complete-owner-only commit;
- `artifacts/carrier-receipt-schema.json`,
  `event-boundary-receipt-schema.json`, the schema fixtures, the V2 carrier /
  boundary vectors, and restart/rollback vectors; and
- `tests/integration/snow_stage3_shared_carrier_authority_contract.rs`, whose
  focused gate passes 5/5 tests.

The implementation write set must be separately scoped and reviewed. It must
not change V10 behavior, Child 2B receipts, Restart V1/V2/V3 identity, LSE
storage arithmetic, selectors/defaults, canopy-intercepted snow scope, or CoE
ownership. It must prove real-consumer lineage, complete-owner custody,
restart before/after the event, replay protection, rollback, and independent
snow/liquid/vapor/energy closure before any production claim.
