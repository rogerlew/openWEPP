# Operand Lineage

Static:

R5A introduces no new physical operands. It changes lifecycle ownership for
existing typed direct-frame state.

| Operand group | Source before R5A | R5A source | Authority |
|---|---|---|---|
| day water state | `DirectWaterState::zero()` in `DirectDayFrame::seed` | persistent `DirectLaneFrame.water` handoff | lifecycle plumbing only |
| day transfer buffers | `DirectTransferBuffers::zero()` in `DirectDayFrame::seed` | persistent `DirectLaneFrame.transfer` handoff | lifecycle plumbing only |
| day publication frame | `DirectPublicationFrame::empty()` | compatibility-shadow placeholder from lane/published direct frame | lifecycle plumbing only |
| direct span downstream operands | existing R3/R4 direct spans | unchanged | existing package authority |
| end-of-day lane state | none | `DirectLaneFrame::commit_day` from day water/transfer/publication | lifecycle plumbing only |

No WB13/WAT/PASS/loss fields become direct-authoritative in this package.
