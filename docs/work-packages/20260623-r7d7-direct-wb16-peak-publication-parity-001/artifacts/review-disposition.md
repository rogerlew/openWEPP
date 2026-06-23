# Review Disposition

Status: executed-held.

## Review A

- Static/Ran: accepted. R7D7 confirmed the compatibility PASS `peakro = 0.0`
  behavior was a serialization defect, because compatibility HBP already read
  runtime `peakro` from the same day surface.

## Review B

- Static/Ran: accepted. Direct PASS was also corrected to consume
  `runoff.peak_runoff_m3_s` before the erosion copy so PASS and HBP share
  producer precedence.

## Finding Disposition

- Fixed: PASS `peakro` residual. Compatibility PASS now consumes explicit
  `HillslopePassPublicationScalars` sourced from the day runtime surface.
- Fixed: direct PASS producer precedence. Direct PASS now reads
  `runoff.peak_runoff_m3_s.or(erosion.peak_runoff_m3_s)`.
- Verified: WAT and PASS are byte-identical on the fresh H2637 5-day fixture.
- Held: HBP sediment export aliases remain non-parity:
  compatibility publishes `total_detachment_kg = 0.6` and
  `sediment_concentration_kg_m3 = 6.816136920064195`, while direct publishes
  `0.0` for both. This is a direct EROD15/HBP producer-authority gap and must
  not be hidden by compatibility wrapping or by suppressing typed direct WB16
  peak state.
