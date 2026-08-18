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
preserves upstream parcel/source identity. Receipt preparation no longer
mutates the cursor; an explicit commit applies the candidate transition only
after downstream acceptance. Validated persisted restore and combined
shadow/cursor checkpoint ownership remain open requirements.

The public provider returns an opaque validated wrapper. The real Child-4
adapter accepts that sealed wrapper (not raw caller DTOs), reconstructs actual
`LandSurfaceForcing` and `SnowFreeForcing`, scopes parcel IDs only at the LSE
aggregation boundary, retains live-owner template fields, recomputes LSE
forcing digests, and validates the resulting real consumer types. A focused
unit test executes the explicit provider parser -> provider -> sealed wrapper
-> Child-4 forcing path. The real kernel then rejects the canonical midnight
interval with `ci_bracket`; this is recorded in the named hold-lift package and
prevents completion.

Ordinary strict climate parsing retains its historical 24-hour bound.
`ParserMode::SnowFreeHalfHourProvider` is the explicit, default-off opt-in that
preserves absolute breakpoint support through 48 hours for provider carry.

Independent integration evidence uses strict parser input and the public
provider API. It covers two destinations, 48 receipts, zero-order hold,
horizontal daily-energy closure, breakpoint rainfall, schema validation,
fallback rainfall, midnight carry, authority-vector primitive parity, and
one-bit/unsupported-domain poisons.
