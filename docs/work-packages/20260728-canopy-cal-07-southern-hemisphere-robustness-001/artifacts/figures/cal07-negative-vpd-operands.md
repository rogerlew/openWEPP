# Temperature Operands on the Three Inadmissible Alerce Days

## Caption

Daily minimum temperature (blue), maximum temperature (orange), and dew-point
temperature (green) for the three Alerce Costero dates that reconstruct to
negative VPD. The contract-defined result is printed beneath each date:
-58.86 Pa, -70.49 Pa, and -1.00 Pa.

## How to read it

Each group is one day. VPD is not a simple temperature difference: the
saturation-vapor-pressure equation is nonlinear, so the three temperature
operands are converted to vapor pressures before subtraction. The green
dew-point marker is therefore contextual, not a visual pass/fail threshold by
itself.

## Plain-language takeaway

The source combination implies actual vapor pressure greater than the
contract's mean saturation-vapor-pressure estimate. Two failures are
substantial and one is very small, but the contract supplies no
tolerance-based clipping rule. Treating the -1 Pa day differently after
seeing it would be post-result canonicalization.

## Methods and source binding

Values are copied without adjustment from the frozen NASA POWER response and
the independently reconstructed VPD diagnostic. The embedded SVG metadata
binds `artifacts/negative-vpd-days.csv`, SHA-256
`a31a5d078922580a920f469d2cfd0d3d1c911f1016c6f7b75a61b642d060eb17`.

## Limitations

This figure cannot determine whether the mismatch arises from gridded-field
aggregation, product precision, temporal averaging, representativeness, or
another source process. It does not authorize changing dew point, deleting a
day, interpolating, or clipping VPD.

## Accessibility

Minimum, maximum, and dew-point operands use distinct colors and consistent
left-to-right offsets within every date group. The legend names each series;
the date and VPD value are printed below every group. The SVG includes title,
description, and source metadata.
