# Implementation Evidence

The explicit `HillslopeClimateRuntimeRequest::snow_free_half_hour_forcing_receipts`
operation now projects an actual parsed climate day to one complete 48-interval
receipt per configured OFE/tile destination. It remains default-off and has no
runner call site.

The projection retains source date/station/version, absolute breakpoint storm
start, event-relative support, and one-day midnight carry. It reconstructs
SIMIMPL28 parent temperature/cloud/horizontal-radiation mechanics, FAO-56
station pressure, repository Harder-Pomeroy hydrometeor phase, dew-point
humidity, Weiss-Norman four-way shortwave, Dilley-O'Brien/Unsworth-Monteith
longwave, liquid enthalpy, and canonical interval/day digests. Parent values
are held into both children. Positive snow/mixed phase, nonpositive wind/VPD,
duplicate destinations, invalid identity, and closure failures reject through
typed errors.

A run-bound provider cursor splits next-day support at child boundaries and
preserves upstream parcel/source identity. Receipt preparation does not mutate
the cursor; an explicit commit applies the candidate transition only after
downstream acceptance. Validated persisted restore atomically owns the combined
shadow, GSI, and provider-cursor checkpoint.

The public provider returns an opaque validated wrapper. The real Child-4
adapter accepts that sealed wrapper (not raw caller DTOs), reconstructs actual
`LandSurfaceForcing` and `SnowFreeForcing`, scopes parcel IDs only at the LSE
aggregation boundary, retains live-owner template fields, recomputes LSE
forcing digests, and validates the resulting real consumer types. Focused
tests execute the explicit provider parser -> provider -> sealed wrapper ->
Child-4 forcing path and complete both zero-radiation and realistic
positive-radiation 48-step days through the public V10/LSE-V2 transaction.

Ordinary strict climate parsing retains its historical 24-hour bound.
`ParserMode::SnowFreeHalfHourProvider` is the explicit, default-off opt-in that
preserves absolute breakpoint support through 48 hours for provider carry.

Independent integration evidence uses strict parser input and the public
provider API. It covers two destinations, 48 receipts, zero-order hold,
horizontal daily-energy closure, breakpoint rainfall, schema validation,
fallback rainfall, midnight carry, authority-vector primitive parity, and
one-bit/unsupported-domain poisons.

The released persisted-restart path now reconstructs the opaque prepared-day
capability only after exact static run/GSI identity, cursor day/configuration,
destination order/cardinality, 48-step WB14/CO2/reference-height, GSI receipt,
and both carry-direction joins. Empty, reordered, wrong-static, and stale-ending
poisons reject before any live-owner mutation.
