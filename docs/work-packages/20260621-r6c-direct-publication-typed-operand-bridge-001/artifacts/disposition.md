# Disposition

Evidence mode: Static + Ran.

Status: executed-hold.

Final disposition:
`HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`.

R6C changed the opt-in cutover path so it no longer builds a skeleton direct
publication frame and then fails parity. It now fails before skeleton frame
construction with a marker naming the missing production producer surface.

Confirmed facts:

- production climate execution publishes WB13/PASS compatibility products;
- production climate execution does not retain direct day/publication operands;
- using WB13/runtime/writeback structures as direct publication authority is
  forbidden by the R6 canonical ledger;
- cutover public writes remain fail-closed.

Next package:

- add retained production direct publication producers to the climate lifecycle;
- feed `DirectRunPublicationFrame` from those producers;
- then close HBP/WAT/PASS/loss/manifest parity, manifest cutover, anti-alias,
  independent reconstruction, no-compatibility, and benchmark gates.
