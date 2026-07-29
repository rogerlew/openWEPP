# Published Hourly-Average POWER Operands Remain VPD-Positive

## Caption

Published POWER hourly-average air temperature (solid blue), dew-point
temperature (dashed green), and paired hourly-product VPD reconstruction
(orange) for the three CAL-07 failure dates. All 72 reconstructed hourly
values remain above the red zero boundary. Daily minima are 17.88 Pa,
30.63 Pa, and 27.47 Pa, respectively.

## How to read it

Each date is a separate Local Solar Time column. The upper panel shows
temperature operands; the lower panel shows
`1000 * (es(T2M) - es(T2MDEW))`. Hour keys `00` through `23` are the starts
of POWER's represented LST hours. Columns are not connected across dates.

## Plain-language takeaway

Negative VPD is not present in any paired published hourly operand record for
these cases. The negative sign appears only after those hourly products are
summarized into separate daily extrema and mean dew point and then passed
through the daily contract operator.

## Methods and source binding

The figure displays all 72 raw source pairs without smoothing or clipping.
POWER describes these as hourly-average gridded/reanalysis values, not
instantaneous measurements. VPD uses
`es(T)=0.6108*exp(17.27*T/(T+237.3))` kPa and the exact Pa conversion.

The embedded SVG metadata binds `artifacts/hourly-reconstruction.csv`,
SHA-256
`ca9bb4c3817fc9c17ce90b101751c79493eaba01ca1fe26bfafbbd9b1da5cf58`.
Raw response URLs, retrieval time, bytes, and hashes are retained in
`artifacts/source-manifest.csv`.

## Limitations

These are three cases selected because their CAL-07 daily VPD was negative;
they are not a representative climatological sample. Published hourly
averages and two-decimal serialization cannot establish instantaneous
atmospheric truth or site conditions at the 840 m camera site. The POWER grid
elevation is 99.4 m.

## Accessibility

Temperature and dew point use color plus solid/dashed line styles. VPD is in a
separate panel with numeric ticks and a labeled zero boundary. Every hour is
visible; the SVG includes a title, detailed description, and exact source
metadata.
