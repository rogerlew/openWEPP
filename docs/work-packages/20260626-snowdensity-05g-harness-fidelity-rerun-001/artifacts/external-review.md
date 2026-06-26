# External Review

Evidence class: Static.

Review disposition: endorsed.

Summary:

- 05G is accepted as a model-correction package. It fixed the diagnostic
  harness instead of defending the 05E promotion-candidate result.
- The old `cancov = 0.0` replay artifact is resolved: diagnostic replay now
  consumes the generated openWEPP runtime surface `cancov`, and all five
  coniferous SNOTEL fixtures replay at `0.9`.
- The legacy diagnostic gap is resolved: `legacy_coe` at representative canopy
  reproduces the as-built profile (`fail=9`, `marginal=8`, `pass=8`,
  `strong=20`, `unavailable=15`), showing the prior diagnostic-vs-production
  gap was caused by the hard-coded canopy.
- The shortwave bridge is accepted for 05G adjudication: the PySnobal forcing
  CSV transports openWEPP native hourly shortwave through
  `net_solar = hrrad * 1000000 / 3600 * 0.8`, and `coe-melt` inverts that
  transport with `/ 0.8`. The `0.8` factor is a round-trip format bridge in the
  CoE replay, not an independent snow-only radiation product.
- The representative result is correctly deflating: at coniferous canopy
  `coe_shortwave_albedo_v1` is neutral on robust failures and should not be
  promoted by 05E's low-canopy artifact.

Carry-forward:

- Do not retire `coe_shortwave_albedo_v1` based on conifer neutrality. Hold it
  opt-in pending low-canopy/mixed-forest adjudication.
- Mixed-forest fixtures such as Marcell/Harvard become load-bearing for the
  open-vs-deciduous-vs-conifer test.
- Mixed-forest adjudication must use real per-day seasonal canopy, not the
  single representative `0.9` canopy used for evergreen fixtures.
- The PySnobal comparison arm consumes exported `net_solar` directly and
  therefore sees `0.8 * hrrad`; that remains acceptable only because PySnobal
  is an ADR-0017 diagnostic flag profile, not the CoE melt decision basis.
- Brock-2000 constant review against the in-repo `brock2000.pdf` remains open
  for a later albedo-focused check.

