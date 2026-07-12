# Pre-Implementation Gate

Static authority: `SC-PLANT-001#INV-PLANT-021` explicitly requires required
growth symbols, including `bbb`, to be finite or fail as a typed boundary error.

Ran reproduction: `cqr_growth_crop_authority_preserves_schedule_precedence_and_integral_guards`
with `bbb = NaN` failed its expected-error assertion because production returned
`DirectProductionTypedGrowthCropAuthority { bbb: NaN, ... }`.

Mechanism: `direct_growth_projection_required_scalar` checks presence only and
returns a present non-finite value unchanged. The correction is in the declared
production helper and does not change physics, thresholds, or finite inputs.

Disposition: `OPENWEPP-DEFECTIVE`; proceed to correction.
