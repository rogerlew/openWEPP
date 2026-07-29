# Harvard HF237 SWE Authority Contradiction

Evidence class: `Ran + Static source inspection`

Status: `EXCLUDED_FROM_CAL-06 SCORING`

The official [Harvard Forest HF237 detailed
metadata](https://harvardforest1.fas.harvard.edu/exist/apps/datasets/showData.html?id=HF237)
labels snow depth and snow-water equivalent (SWE) as centimeters and density as
kilograms per cubic meter. The checksum-bound raw file is
`target/cancov_stratified_observed/raw/harvard/hf237-01-snow-depth.csv`
(SHA-256
`2c80d505952350879df3993d61262608ee0f5a695f5a9409c1213edd1d0271ff`).

The raw values contradict those declared units and the physical
depth-density-SWE identity. For example, the 2008-12-07 Shaler row reports:

| Quantity | Raw value | Declared unit |
| --- | ---: | --- |
| Depth | 1 | cm |
| Density | 194 | kg/m3 |
| SWE | 2 | cm |

One centimeter of snow at 194 kg/m3 implies 1.94 mm, or 0.194 cm, SWE. The raw
SWE value `2` is consistent with rounded millimeters, but the provider metadata
declares centimeters. The same approximately tenfold conflict recurs across
nonzero rows.

The installed v1 normalizer followed the metadata and multiplied raw SWE by
ten. CAL-06 initially exposed the resulting implausible residual, then excluded
Harvard SWE before terminal scoring. It did not silently relabel the field,
edit the source fixture, or use the contradicted value as model evidence.
Harvard depth and density remain separately scoreable; Harvard hemlock remains
unbound.

Lifting this exclusion requires provider clarification or a formally admitted
source correction. This issue does not affect Marcell SWE, whose normalized
depth, SWE, and derived density identity is internally consistent.

