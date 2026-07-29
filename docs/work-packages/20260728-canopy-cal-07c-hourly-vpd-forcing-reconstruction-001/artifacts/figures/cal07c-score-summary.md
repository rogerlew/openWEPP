# Timing And Shape Score Summary

## Caption

Median normalized shape correlations by site/year and Beza deciduous midpoint
timing residuals for every frozen ensemble member.

## How to read it

Positive bars indicate same-direction annual normalized shape agreement.
Purple points are modeled minus observed transition dates; zero means exact
agreement with the provisional PhenoCam midpoint. Blank member/event rows do
not appear as points because no same-direction modeled crossing was found.

## Plain-language takeaway

The result is descriptive bounded evidence. It evaluates whether the frozen
ensemble has Southern Hemisphere timing/shape support after the forcing
blocker is removed; it does not refit or choose members. Only
11 of 148 Beza member/event rows found a
same-direction crossing, so transition chronology remains contradicted.

## Methods and source binding

The figure binds `shape-scores.csv`, `transition-residuals.csv`, and
`site-summary.csv`, plus `artifacts/source-manifest.csv` SHA-256 `905a6ea0261556da8855a4fbbf98df6a09509490a029832f7d43e8b36c717a6d`, `artifacts/admission-table.csv` SHA-256 `f99c5e9105ddc0f98e1e3bdc23df87546448f460f59c4d2f386ba7af3c41bc38`, and `artifacts/forcing-source-summary.csv` SHA-256 `bd4a4b44b3b2601c46009e4b9e03ea2411fb50b6229f4651e3d7182420c5fd86`. Alerce uses the POWER
hourly-product daily-mean VPD operand; Beza keeps the CAL-07 daily-summary VPD
operand. Summary: SH-DB-BEZA median r 0.682151, RMSE 0.375275; SH-EN-ALERCE median r 0.478301, RMSE 0.404096.

## Limitations

No pass threshold is invented for timing residual magnitude. Shape scores use
camera-supported days only and annual min-max normalization. No VPD value is
clipped, and CAL-07C does not replace OBL-PLANT-P-013 in production.

## Accessibility

Bars carry signed numeric labels. Timing residuals are plotted against a
visible zero line with days as the unit.
