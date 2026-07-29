# CAL-06 Snow Observation Operator Disposition

Evidence class: `Static source inspection + Ran`

CAL-06 compares like measurement scales only:

- Marcell stratum means are compared with the corresponding modeled stratum.
- Harvard HF237-01 daily bulk snow depth and bulk density are compared with
  daily WAT snow depth and bulk density for the bound open and deciduous lanes.
- Harvard vertical density-profile layers are
  `NOT_EVALUATED_SCALE_MISMATCH`; a layer-at-depth measurement is not a bulk
  snowpack-density observation.
- Harvard SWE is
  `INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION`. The provider metadata declares
  depth and SWE in centimeters, while installed rows conflict with the
  `SWE = depth × density / water-density` identity by approximately one order
  of magnitude under those units. CAL-06 does not reinterpret or modify the
  installed source fixture.
- Harvard hemlock observations remain unbound because the native mixed lane is
  not a pure-hemlock counterpart.

The scoring tables therefore quantify residuals without creating a support
threshold, silently pooling incompatible scales, or treating excluded data as
zero.
