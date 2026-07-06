# Worker Handoff

Status: **COMPLETE — handoff to D10 hold-lift / D15 rerun**.

## First actionable item

Close `SC-OFEROUTE-001#GAP-OFEROUTE-005`: source-authority reconciliation for
the reduced KWE limiter/CFL/dissipation, lateral source and sampled handoff,
and Iwagaki friction mapping.

## After the hold lifts

Rerun D15 or open a D15 hold-lift package. The activation rerun must prove:

- active selector and opt-in surface,
- DC01 daily-lump runon disabled for active lanes,
- active routed path owns surface-water publication,
- runtime closure hard-fail includes `latqcc` bypass,
- D13 erosion consumer receives routed hydrograph weights,
- rev-21 D11 friction operands and D12 melt limb are read by the active
  consumer,
- default/off protected outputs stay byte-identical,
- H2637 active evidence is inside the accepted D10/D14 envelope,
- no default promotion (D16 only).

## D14 budget to carry forward

Use the D14 budget unless the D10 hold-lift materially changes solver
resolution, friction operands, source shape, or handoff policy. If it does,
refresh D14 endpoint timing before activation claims.
