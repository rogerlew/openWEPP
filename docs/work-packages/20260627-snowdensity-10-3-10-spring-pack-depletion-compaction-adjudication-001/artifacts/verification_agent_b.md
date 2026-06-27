# Verification B

Evidence class: Static + Ran

## Checks

- `wc -l` line-count check for the new diagnostic tool, guard test, and package.
- Boundary scan for `subprocess.run`, `OPENWEPP_SNOWDENSITY10310`,
  `production_physics_changed`, `density_cap_changed`, and
  `SNOW_DENSITY_CAP_KG_M3`.

## Result

PASS.

## Notes

- No production runner invocation exists in the diagnostic tool.
- No new runtime selector is introduced.
- The only cap constant is the existing `522.0 kg m^-3` authority used for
  feasibility classification.
