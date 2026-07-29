# Alerce VPD Reconstruction Audit

## Caption

Original CAL-07 daily contract VPD and CAL-07C admitted hourly-product daily
mean VPD for Alerce Costero. Red points are the three original negative daily
contract values. The lower panel shows days with negative hourly paired-product
components; 103 days contain at least one such hour, but every
admitted daily mean is nonnegative.

## How to read it

The blue line is the VPD operand passed to the CAL-07C package-local GSI
executor. The dashed gray line is the original OBL-PLANT-P-013 daily-summary
reconstruction from CAL-07. Orange bars are counts of retained negative hourly
components inside the daily signed mean.

## Plain-language takeaway

The three CAL-07 daily failures disappear at the daily operand level when VPD
is reconstructed as the mean of paired hourly POWER products. The hourly source
is still not physically clean at every hour, so the result is bounded research
forcing evidence, not production authority.

## Methods and source binding

The figure binds `artifacts/daily-vpd-reconstruction.csv`, SHA-256
`f99c5e9105ddc0f98e1e3bdc23df87546448f460f59c4d2f386ba7af3c41bc38`, plus
`artifacts/source-manifest.csv` SHA-256 `905a6ea0261556da8855a4fbbf98df6a09509490a029832f7d43e8b36c717a6d`, `artifacts/admission-table.csv` SHA-256 `f99c5e9105ddc0f98e1e3bdc23df87546448f460f59c4d2f386ba7af3c41bc38`, and `artifacts/forcing-source-summary.csv` SHA-256 `bd4a4b44b3b2601c46009e4b9e03ea2411fb50b6229f4651e3d7182420c5fd86`. VPD uses `1000*(es(T2M)-es(T2MDEW))` at each
hour and an arithmetic daily mean over exact 24-hour LST days. Units are Pa.

## Limitations

No hourly negative value is clipped, deleted, or hidden; 349 negative hourly
components remain a claim ceiling. The retained POWER grid elevation remains
99.4 m while the camera site is approximately 840 m. This package does not
replace OBL-PLANT-P-013 in production.

## Accessibility

The lines, points, and bars use color plus distinct geometry. Zero is a
visible horizontal reference, axes carry units, and the SVG includes title,
description, and source metadata.
