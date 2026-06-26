# Albedo Authority Ledger

Static:

- Brock et al. (2000) is the authority selected for the first opt-in
  temperature/age albedo state because it gives a compact temperature-only
  snow-albedo decay model compatible with the current openWEPP forcing seam.
- Accepted model id: `brock2000_temperature_age_v1`.
- Age symbol: `Ta`, accumulated positive air-temperature age since the latest
  fresh-snow reset.
- Deep-snow formula: `0.713 - 0.112 * log10(Ta)`.
- Shallow-snow formula: `underlying_albedo + 0.442 * exp(-0.058 * Ta)`.
- Depth transition: `d_star = 0.024 m water equivalent`.
- Upper albedo cap: `0.85`.
- Fresh-snow reset threshold: `0.001 m` water equivalent.

No SNOTEL site fitting was used. SNOTEL, frost-site, legacy WEPP, and PySnobal
profiles may classify future residuals, but they did not fit the 05C constants
or select a site-specific albedo model.

Copyright posture: the package records formula provenance and paraphrased
authority only; it does not reproduce long copyrighted passages.
