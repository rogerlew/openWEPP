# Disposition

Status: complete / defect corrected / review and verification pass

Evidence mode: Static + Ran

`SNOW-WETCOMPACT-DUP-001` is closed with a landed, validated correction. The
retired driver combined a bounded snowpack state loss with routed liquid even
though routed liquid already contained that state loss. Canonical authority now
requires positive hourly generated melt plus interval-start snow-contact rain,
counted once before runoff. Production carries that value through one private,
typed scalar, and the offline CoE-bound replay requires the same three exact
lineage columns.

The correction is materially different. Across the four canonical sites, the
new accumulated driver is `55.7%` to `61.9%` of the old driver, changing
`24,046` driver-days and `22,392` density-days. Maximum density and depth
deltas are `174.016 kg m^-3` and `0.367071 m`. Upstream mass remains invariant
within `2.443e-15 m`; operand, density, layer, and Stage-3 closures all pass by
orders of magnitude. Density-mediated routing/store/refreeze disposition may
change and did so by at most `0.002363 m`.

This does not explain or fix early melt. It removes an upstream density/geometry
confounder and therefore invalidates pre-21K density, depth, and loss baselines
for causal attribution. It changes neither generated melt nor the energy
equations, phase partition, forcing, radiation, canopy, frost, density
coefficients/cap, or defaults.

The derived Snowbird CLI is verified and remains `DEVELOPMENT_ONLY`. It raises
median peak SWE by `0.122877 m` (`1.291661` ratio) over 39 matched water years
without shifting median peak timing. That confirms input sensitivity only; it
does not establish precipitation truth or validate the physics correction.

Both independent reviews return `GO`; both terminal verifications pass with
nonblocking notes. Quick (`2181/2181`), frost (`358/358`), Critical full
(`2270/2270`), doctest,
format, warnings-denied Clippy, dependency policy, assurance validation,
fixture reproducibility, anti-evasion, and AUTH11 gates pass.

21L is admitted with one strict entry condition: first regenerate canonical
and scaled Snowbird plus Mica, Niwot, and Paradise corrected-state baselines,
then attribute warm/mixed pre-peak loss while preserving forcing, snowfall,
storage, loss, and energy operands separately. Canonical lanes own acceptance;
the scaled Snowbird lane owns development sensitivity only.
