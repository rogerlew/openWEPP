# Observed Greenness And CAL-07C GSI

## Caption

Two Southern Hemisphere PhenoCam lanes compared with the frozen 37-member GSI
ensemble after the Alerce VPD forcing blocker is lifted for bounded execution.

## How to read it

The blue line is the ensemble median and the blue band spans the 5th to 95th
percentiles. The green dashed line is annually normalized raw GCC90 on admitted
camera days.

## Plain-language takeaway

CAL-07C restores the ability to look at Southern Hemisphere timing and shape,
but the camera greenness remains a relative proxy and not absolute LAI,
biomass, or canopy cover.

## Methods and source binding

The figure binds `artifacts/ensemble-daily.csv`, SHA-256
`6e166463681fea4bc0ea8a0347745cf1b59863816dd7be920937a957ca7f805b`, plus `artifacts/source-manifest.csv` SHA-256 `905a6ea0261556da8855a4fbbf98df6a09509490a029832f7d43e8b36c717a6d`, `artifacts/admission-table.csv` SHA-256 `f99c5e9105ddc0f98e1e3bdc23df87546448f460f59c4d2f386ba7af3c41bc38`, and `artifacts/forcing-source-summary.csv` SHA-256 `bd4a4b44b3b2601c46009e4b9e03ea2411fb50b6229f4651e3d7182420c5fd86`.
CAL-04B accepted members are retained without refit or ranking. Alerce uses
the daily mean of POWER hourly paired-product VPD over exact 24-hour LST
days; Beza keeps the CAL-07 daily-summary VPD operator.

## Limitations

Both camera products are provisional and share the PhenoCam processing method.
POWER forcing is gridded/reanalysis evidence, not on-site meteorology. No VPD
value is clipped, and CAL-07C does not replace OBL-PLANT-P-013 in production.

## Accessibility

Median, uncertainty band, and observed proxy use separate line styles and
geometry. Panels are separated by site and include units-free normalized
fraction axes.
