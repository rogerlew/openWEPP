# CAL-07B Final Disposition

Evidence class: `Ran + Static`

Disposition: `COMPLETE / DIAGNOSTIC PASS / CAL-07 HOLD RETAINED`

CAL-07B answers the diagnostic question that followed CAL-07's fail-closed
forcing rejection. The retained POWER hourly-average operands do not contain
negative paired hourly-product VPD on the three failure dates. The negative
daily values are reproduced when the OBL-PLANT-P-013 daily summary operator is
applied to Tmin/Tmax and mean dew point.

The primary driver is the temperature-extrema summary term. It lowers VPD by
about 59.96 Pa to 116.89 Pa across the three cases. Dew-point nonlinearity is
positive and much smaller, so it offsets rather than causes the negative
shift.

This package does not resume CAL-07, alter production behavior, authorize VPD
clipping, or authorize an hourly replacement operator. Order 7 remains open.
Resume still requires one of:

1. continuous, provenance-complete, contract-admissible meteorology for the
   frozen Alerce site and period; or
2. explicit science-contract authority for bounded input canonicalization or
   a different admitted VPD product/operator, established before rerun.
