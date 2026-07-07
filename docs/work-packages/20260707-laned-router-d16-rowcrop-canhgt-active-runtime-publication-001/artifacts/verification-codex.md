# Verification - Codex

Status: COMPLETE

Ran:

- Verified contract text and code agree on the source of Lane D dynamic
  `h_c`: post-growth PL state carried through
  `DirectDayFrame.evapotranspiration_compute_inputs.canopy_height_m`.
- Verified active route rejects positive LAI with missing/non-positive
  post-growth canopy height and accepts a positive day-frame value even when
  static lane configuration canopy height is zero.
- Verified shadow operand source guards reject stale static-authority
  consumption.
- Verified the selected-cohort active plain/hybrid suite completes for all
  four selected members.
- Verified the full workspace test suite, clippy, fmt, deny, markdown lint, and
  touched-contract unit compliance gates.

Conclusion:

- The `mn_corn_h4` active-run canopy-height publication hold is lifted.
