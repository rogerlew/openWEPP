# Worker Handoff

Status: **HANDOFF-RECORDED**.

## Next Action

Open the next D15 active-owner/optimization follow-on.

Required first decisions/actions:

- Implement an explicit active selector and production execution-order change:
  routed water must be available before downstream runon admission and before
  erosion consumes the hourly shape.
- Optimize/adjudicate the current `91.59 s` user / `1:31.67` wall H2637
  opt-in timing regression as part of D15 readiness.
- Disable DC01 daily-lump runon on active routed lanes.
- Wire active `INV-OFEROUTE-012` closure hard-fail with `ui_SCrunf` and
  `latqcc` bypass operands.
- Feed D13 routed-hydrograph shape to erosion in the real active consumer.
- Preserve subsystem-off/default byte identity.
