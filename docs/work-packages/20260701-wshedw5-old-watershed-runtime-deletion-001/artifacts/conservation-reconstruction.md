# Conservation Reconstruction

Status: `not-applicable`

Evidence mode: `static + ran`

W5 does not introduce new watershed conservation formulas or publication
operands. It deletes the retired request/writeback carrier and keeps the direct
typed kernel as the only production route.

Supporting checks:

- WS11 typed branch closure is asserted in
  `typed_frame_dispatch_executes_ws11_ipeak_branches_with_closure`.
- WS12 inactive outflow and active drop-spillway min-controller composition are
  asserted through typed frame state.
- Transport capacity is verified to respond to particle diameter and not
  collapse to the deleted surrogate identity.

No independent output-operand reconstruction artifact is required for W5
because no new conservation-sensitive publication equation is introduced.
