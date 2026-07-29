# CAL-07C Pre-Execution Source Inventory

Evidence class: `Ran`

The full-period POWER hourly Alerce response was inspected before
result-bearing CAL-07C execution.

Inventory:

- Hourly `T2M` rows: 39,984.
- Hourly `T2MDEW` rows: 39,984.
- Complete daily groups: 1,666.
- Period: `2022010100` through `2026072423` LST.

Compatibility with CAL-07 daily operands:

- Hourly-derived Tmin maximum absolute residual: `0 C`.
- Hourly-derived Tmax maximum absolute residual: `0 C`.
- Hourly-derived mean dew-point maximum absolute residual:
  `0.005000000000000782 C`.
- Days exceeding the `0.01 C` serialized-resolution tolerance: `0`.

VPD sign inventory:

- Minimum hourly paired-product VPD: `-82.78055165239107 Pa`.
- Negative hourly paired-product rows: `349`.
- Minimum admitted daily mean hourly-product VPD:
  `19.793539669084804 Pa` on `2023-05-12`.
- Negative admitted daily VPD rows: `0`.

Disposition before execution:

The package protocol was amended before CAL-07C result execution. Negative
hourly paired-product components are retained, counted, plotted, and averaged
without clipping. They are a claim ceiling and prospective-review issue, not a
silent correction. Result-bearing execution remains blocked unless
prospective review accepts this daily-operand boundary.
