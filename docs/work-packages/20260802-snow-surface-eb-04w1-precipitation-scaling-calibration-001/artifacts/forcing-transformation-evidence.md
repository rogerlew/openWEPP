# Forcing Transformation Evidence

Status: `PASS`

Evidence mode: **Ran**.

## Transformation Contract

The package copied each source fixture beneath
`target/snow_surface_eb04w1_precipitation_scaling/` and multiplied only the
daily CLIGEN precipitation-depth token. Event duration, time-to-peak fraction,
dimensionless peak intensity, maximum and minimum temperature, solar
radiation, wind, dew-point-related fields, all non-daily lines, and every
non-climate input were protected.

## Audit Result

| Check | Result |
|---|---:|
| Unique transformed lane/multiplier fixtures | `32 / 32` |
| Maximum `scaled - source * multiplier` residual | `2.842170943040401e-14 mm` |
| Protected daily-token mismatches | `0` |
| Non-daily-line mismatches | `0` |
| Execution return-code failures | `0` |
| `1.0` climate files changed | `0` |
| Non-`1.0` changed-file inventory | lane climate file only |

The residual is decimal rendering roundoff, many orders below any hydrologic
materiality. Each cell retains an individual transformation/provenance record
under `target/.../runs/<multiplier>/<lane>/B/eb04w1-cell-provenance.json`.

## Synthetic Fail-Closed Checks

The transformer self-test passed zero, decimal, and high-precipitation rows and
rejected malformed climate input. These tests establish transformation
mechanics only; they are not scientific calibration or validation evidence.
