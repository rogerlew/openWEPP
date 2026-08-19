# Current caller injection surface

Evidence class: `Static` at intake commit `09bbd5d844456c3c11b3bd9c88909dbe0d5f8ae3`.

The public scheduler callback returns both `PreparedSnowFreeGsiDayV1` and
`DirectV10ShadowDayInput`. The latter aliases the caller-constructible V9 day
template. Provider projection overwrites meteorology, precipitation, GSI, CO2,
and reference height, but retains these caller-controlled physical operands:

- LSE snow/domain flags, transaction, cadence, and runon parcels;
- vegetation ground VIS/NIR albedo and upward longwave;
- every `SoilLayerForcing` field except water and temperature after the current
  partial live-owner projection;
- every per-OFE WB14 parameter.

The closure API therefore remains capable of changing accepted science without
changing the sealed meteorological receipt. This surface must not be narrowed
piecemeal before the missing root-hydraulic authority is admitted, because a
partially sealed API would still be non-closure-eligible.
