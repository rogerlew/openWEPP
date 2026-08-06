# Independent Reconstruction

Status: `PASS`

Evidence mode: `Ran` from retained exact schema-v5 JSONL and runfiles.

- Every v5 tag, arm, fingerprint, day/lane index, applicability flag, support
  field, carrier term, vapor term, cold/ice/sequential/terminal field, and
  residual was consumed.
- Hourly producer complete energy equals the independent sum of shortwave,
  longwave, sensible, latent, and precipitation advection within
  `1e-6 J m^-2`.
- Daily complete-arm and surface-arm producer totals equal independent operand
  sums within `1e-6 J m^-2`; the applicable producer component residual remains
  within that same frozen tolerance.
- True same-state N/A internal, cold-content, ice, melt, terminal, and closure
  fields are exact zero.
- Daily support equals the 24 hourly operands. Same-state support is either a
  full 24-hour day or explicit zero coverage.
- Water-year sample producer totals equal independently accumulated hourly and
  daily operands within the frozen scale-aware tolerance.
- All `154` non-censored analyzed samples pass the 30-day/0.25-coverage screen;
  four WY2025 samples are analyzed but excluded as right-censored.
- Every observation year has a named analysis or no-window disposition.
- Re-running `--verify-existing` reparsed retained traces, observations,
  runfiles, fixture inputs, and WAT/HBP bytes and reproduced the result object.

No producer total, residual, calendar duration, internal active/lower exchange,
or snow-ground placeholder was used as a reconstructed external-subset operand.
