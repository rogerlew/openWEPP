# Source Completeness

Status: `PASS for admitted rain-timed domain; typed rejection outside it`

Evidence mode: `Static + Ran`

V1 admits local rainfall with a WB14 hyetograph clock plus hourly saturation
return labeled `hourly_zero_order_hold`. Every positive hourly-only additional
supply, including values below the closure tolerance, fails `WAT5-E-001`.
Positive authoritative WB14 volume with exact-zero raw support fails
`WAT5-E-002`; out-of-day/nonfinite/negative timing fails `WAT5-E-003`.

Ran: the p102 multi-OFE control exited at lane 1/day 1 with `WAT5-E-001
positive additional supply lacks 300-second timing`. A dedicated two-day p61
regression succeeds on day 1, fails on day 2 with that same typed error, and
publishes no HBP, loss, PASS, WAT, WAT5, manifest, staging file, or backup.
No uniform, rainfall-shaped, or tolerance-based zero fallback was used.

Adoption is therefore limited to the diagnostic rain-timed domain. Multi-OFE
runon, routed melt, snowmelt, frost-retention release, HBP, and watershed
routing remain unadopted.
