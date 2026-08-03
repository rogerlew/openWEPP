# Frozen Comparison Protocol

Static: this protocol is preregistration, not a result.

1. Validate complete UTC hourly inventory and source units; consume the point
   time-series products' de-accumulated radiation without further differencing.
   Apply and publish the checksum-bound `[-4, 0) J m^-2 -> 0` shortwave
   ingress disposition; reject any lower value or identity mismatch.
2. Compare downward shortwave and longwave independently against retained
   hourly forcing: correlation, mean signed error, MAE, fixed-local-standard
   peak-hour offset, daily-energy closure, and winter-event chronology. The
   earlier `local-solar` label was corrected after independent review found
   that differing source-grid longitudes prevent exact cancellation.
3. Examine cloud fraction, dewpoint, vector wind, and pressure jointly while
   holding precipitation bytes and calibrated multipliers unchanged.
4. Compare raw 2 m temperature before a separate fixed `-6.5 K km^-1`
   elevation sensitivity: `T_site = T_grid + lapse_rate * (z_site - z_grid)`,
   with elevations in km. A separately acquired compatible grid-orography or
   geopotential ancillary is mandatory for each dataset/site; geopotential is
   converted by `z / 9.80665`. Without it, temperature sensitivity is blocked.
   The lapse rate is `ASSUMED_FOR_EXECUTION`.
5. Only after isolated lanes close, run snow chronology as radiation-only,
   temperature-only, then combined. Reject compensation that improves timing
   while degrading magnitude, mass closure, or energy closure.

Site results remain separate; no pooled score may conceal a site failure.
