# PERFARRAY01 Review B

Evidence class: Static.

## Findings

1. A benchmark would be misleading without a new array request authority. The
   current `HillslopeKernelRequest` stores logical `BTreeMap` references, so
   an array benchmark would need to export maps before kernel execution.
   Disposition: accepted; no Stage B timing was run.

2. Existing WB11 accessor functions are not array-complete. The core scalar
   accessors read `request.state_surface` and `request.flux_surface` directly,
   while indexed helpers cover only selected symbol families. Disposition:
   accepted; next package must port the accessor layer or introduce a view
   abstraction.

3. The Stage A API should remain inert until an authority path exists.
   Disposition: accepted; no scheduler call site was added.

## Risk

The main residual risk is carrying a shell API that is unused in production.
That risk is bounded because it is tested, exported from the contract crate,
and default-unwired. The next package must either consume it in a real
array-authoritative path or remove/revise it if the authority split changes.
