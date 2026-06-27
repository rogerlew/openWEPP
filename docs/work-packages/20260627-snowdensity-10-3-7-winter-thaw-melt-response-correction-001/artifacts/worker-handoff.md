# Worker Handoff

Evidence mode: Static/Ran.

SNOWDENSITY-10.3.7 is complete.

## Result

- Disposition: `WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES`.
- Contract: `SC-SNOWFREEZE-001` v94.
- New opt-in selector: `coe_winter_thaw_state_loss_v1`.
- Package-bound coupled WAT selector:
  `OPENWEPP_SNOWDENSITY1037_MELT_MODEL=coe_winter_thaw_state_loss_v1`.
- Paired Sleepers/Harvard evidence:
  - Under-ablation windows: `132 -> 108`.
  - Aggregate depth-loss deficit: `24.105 m -> 17.629 m`.
  - Modeled depth loss: `15.868 m -> 26.400 m`.
  - Routed melt: `5.895 m -> 11.235 m`.
  - Snowpack SWE loss: `4.628 m -> 10.615 m`.
- Conservation/routing evidence:
  - Candidate active-ledger SWE balance residual: `0`.
  - Candidate routed state-loss residual: `0`.
  - State loss never exceeds prior SWE plus same-day snow/rain input.
- Coupled WAT evidence:
  - Disposition: `WINTER-THAW-COUPLED-WAT-IMPROVES`.
  - Paired snow-control failures: `1147 -> 978`.
  - Paired surfaces improved/worse: `4/0`.
  - Candidate direct trace selected rows: `112502`.

## Boundaries

- `legacy_coe` remains default and rollback.
- `coe_shortwave_albedo_v1` behavior is unchanged.
- No parser/runfile/user CLI activation, public schema, fixture, coefficient,
  radiation, canopy, phase, density-constant, frost, rain-heat, sub-canopy
  longwave, Qwet/frzftp, or compatibility-runtime change was made.

## Next Route

The candidate improves the thaw-response defect but leaves 108 under-ablation
windows and 978 coupled WAT snow-control failures. The next package should
evaluate the next single lever from the 10.3 sequence, likely sub-canopy
longwave / forest energy, using the same contract-first, opt-in, paired-event
and coupled WAT gates. Do not default-activate
`coe_winter_thaw_state_loss_v1` without a later activation package that also
reruns SNOTEL plus non-SNOTEL snow/frost profiles and clears or explicitly
bounds the remaining snow-control failures.
