# Temperature-Extrema Summarization Drives the Negative Daily VPD

## Caption

Additive decomposition for each CAL-07 failure date. Blue is mean
hourly-product VPD. Orange is the temperature-extrema summary term, green is
the dew-point nonlinearity term, and red is the resulting reconstructed
contract-daily VPD.

## How to read it

For each date, the red value equals the blue value plus the orange and green
terms. The temperature term is strongly negative: -99.14 Pa, -116.89 Pa, and
-59.96 Pa. Dew-point nonlinearity is positive: +6.81 Pa, +5.89 Pa, and
+1.37 Pa. It therefore moderates rather than causes the negative shift.

## Plain-language takeaway

The driver is the daily temperature summary: averaging saturation pressure at
the daily minimum and maximum produces a value substantially below the mean
of the 24 hourly saturation pressures on these days. That negative shift is
large enough to move otherwise positive mean hourly-product VPD below zero.

## Methods and source binding

The exact identity in Pa is:

`contract daily - hourly mean = temperature-extrema term + dew-point term`.

The three closure residuals are at floating-point noise
(`<= 8.53e-14 Pa`). The embedded SVG metadata binds
`artifacts/daily-decomposition.csv`, SHA-256
`7f23d8c904b9b591d42c7d242ff6f47b282c5b1150299e8b5a3e89b0e47f74c5`.

## Limitations

This decomposition explains the published product/operator combination. It
does not prove that one operator is universally preferable, authorize an
hourly replacement in production, or establish a correction to
OBL-PLANT-P-013. The three dates were selected for negative daily results.

## Accessibility

Every bar has a signed numeric value. The four components use distinct colors
and adjacent text legend labels; signs and positions carry the primary
meaning. The SVG includes title, description, units, and source metadata.
