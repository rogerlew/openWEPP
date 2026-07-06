# HOLD Legitimacy Audit

Status: **NOT REQUIRED**.

D13 does not close in `HOLD`. The contract-first route was available inside
the declared write set:

- `SC-OFEROUTE-001` rev 23 names the routed-hydrograph erosion-shape consumer
  surface for active routed-water mode.
- `SC-SED-001` rev 53 names the corresponding sediment hourly-shape rule.
- The runtime implements an explicit candidate selector with fail-closed guards
  for missing, malformed, negative, non-finite, and non-closing routed
  hydrograph fractions.
- Contract-derived tests prove the routed candidate supersedes DC01 weights
  for the Wave-1 substrate while default/off remains on the legacy DC01 shape.

No source-authority or interface boundary prevented D13 closure.
