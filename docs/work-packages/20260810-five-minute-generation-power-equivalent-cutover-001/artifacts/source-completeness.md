# Source Completeness

Status: `PASS for admitted rain-timed domain; typed rejection outside it`

Evidence mode: `Static + Ran`

V1 admits local rainfall with a WB14 hyetograph clock plus hourly saturation
return labeled `hourly_zero_order_hold`. Every positive hourly-only additional
supply, including values below the closure tolerance, fails `WAT5-E-001`.
Positive authoritative WB14 volume with exact-zero raw support fails
`WAT5-E-002`; out-of-day/nonfinite/negative timing fails `WAT5-E-003`.

Ran: the exact rebuilt release binary executed the p102 multi-OFE control under
`/home/workdir/openwepp-wat5-terminal/p102` and exited 1 at lane 1/day 1 with
`WAT5-E-001 positive additional supply lacks 300-second timing`. It published
no WAT5 target and left no WAT5 temporary file. No uniform, rainfall-shaped,
or tolerance-based zero fallback was used.

Adoption is therefore limited to the diagnostic rain-timed domain. Multi-OFE
runon, routed melt, snowmelt, frost-retention release, HBP, and watershed
routing remain unadopted.
