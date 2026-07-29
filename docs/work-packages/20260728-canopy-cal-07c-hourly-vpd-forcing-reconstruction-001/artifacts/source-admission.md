# CAL-07C Source Admission

Evidence class: `Ran`

## Admitted source object

CAL-07C retained the full-period Alerce Costero NASA POWER hourly response:

- path: `inputs/source/power_hourly_alerce_20220101_20260724_lst.json`;
- SHA-256: `ad6fad499f1c4db3a10252dcf0b71182cef7b41360b9aade041423e2dd71f6df`;
- rows: 39,984 `T2M` and 39,984 `T2MDEW`;
- period: `2022010100` through `2026072423` LST;
- API: `v2.9.6`;
- source list: `GEOSIT;MERRA2`;
- fill value: `-999.0`;
- grid geometry: longitude `-73.444000`, latitude `-40.173000`,
  elevation `99.400 m`; and
- units: `T2M:C;T2MDEW:C`.

The complete source and POWER documentation custody table is
`artifacts/source-manifest.csv`.

## Admission operator

For Alerce only:

```text
hourly_product_vpd_pa = 1000 * (es(T2M_hour) - es(T2MDEW_hour))
daily_vpd_pa = mean(hourly_product_vpd_pa over exact 24-hour LST day)
```

For Beza Mahafaly, CAL-07C retained the CAL-07 daily-summary operator.

No negative value was clipped, deleted, normalized, or hidden.

## Admission result

`artifacts/admission-table.csv` records every Alerce day. The table has 1,666
rows, each with 24 hourly keys and `daily_admission_pass=true`.

Observed source facts:

- negative hourly paired-product components retained: `349`;
- negative admitted daily VPD rows: `0`;
- minimum admitted daily VPD: `19.793539669084804 Pa` on `2023-05-12`;
- original negative daily-summary dates rejected for CAL-07C execution:
  `2022-07-22`, `2022-09-15`, and `2025-09-09`; and
- hourly-derived Tmin/Tmax/mean dew-point residuals against frozen CAL-07
  daily operands remain within the declared `0.01 C` serialized-resolution
  tolerance.

Disposition: `ADMITTED_FOR_PACKAGE_LOCAL_BOUNDED_EXECUTION`.

This admission does not replace `SC-PLANT-001` OBL-PLANT-P-013 in production.
