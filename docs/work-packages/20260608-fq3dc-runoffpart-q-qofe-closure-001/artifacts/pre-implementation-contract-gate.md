# Pre-Implementation Contract Gate

Status: complete

Evidence mode: Static + Ran.

## Gate Result

Ran: pre-fix p8/p1 post-FQ1 WAT evidence reproduced the defect:

- `p8`: `sum(Q)=3.930232875259954e-15 mm`
- `p1`: `sum(Q)=3.0643619152587176e-13 mm`

Ran: p8 event trace localized the failure to WB14 over-infiltration of a
`24.6 mm` rain day with exhausted top-two-layer storage.

Static: `SC-RUNOFFPART-001` was amended before production code was corrected.
The new invariant and tests were authored before final production validation.

## Seven-Gate Bar

- Reproduce defect: satisfied.
- Named mechanism: WB14 same-pass infiltration ignored top-two storage limit;
  WB14 later recomputation broke producer-consumer identity.
- Ownership: in runoff partition / same-pass infiltration envelope.
- Authority: `SC-RUNOFFPART-001` v39, physical storage limit, WB18/WB14 producer
  identity.
- Safety: validation requires annual WAT closure preservation.
- Testability: added contract-derived tests.
- Population validation: completed over 42 runnable prefixes.

Conclusion: `HOLD` was invalid once the in-envelope mechanism was proven; the
package proceeded to correction.
