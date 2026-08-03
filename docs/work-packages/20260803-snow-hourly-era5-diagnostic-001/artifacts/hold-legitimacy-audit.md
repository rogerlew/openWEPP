# HOLD Legitimacy Audit

Ran: the local credential, reader, and custody audit proves that result-bearing
execution crosses an unavailable external-data boundary.

## Current Boundary

The original no-data boundary is superseded. Authenticated hourly
ERA5/ERA5-Land bytes, official grid geopotential, the CDS client, and NetCDF
readers are now available. Direct validation passes; result-bearing comparison
waits only for fresh independent review/verification of this increment.

## Routes Considered

- Authenticated CDS retrieval: completed for eight long-range point series;
  credential values were never recorded.
- Direct content validation: passed for exact identities, time axes, variables,
  units/domains, and the bounded shortwave rule; review remains open.
- Elevation reconciliation: official gridded geopotential and source-project
  target elevations are acquired, hash-bound, and validated.
- Synthetic or alternate forcing: rejected because it would not be ERA5 and
  could not support the requested attribution.

The shortwave-negative disposition is complete. Full attribution advances only
after the current validation/elevation evidence passes review and verification.
