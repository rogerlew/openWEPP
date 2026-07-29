# CANOPY-CAL-07B Hourly VPD Aggregation Diagnostic

Status: `complete / diagnostic pass / CAL-07 hold retained`

Evidence mode: `Ran + Static`

## Objective

Determine whether CAL-07's three negative daily VPD values are already present
in reconstructions from NASA POWER's published hourly-average operands or are introduced when
separately summarized daily temperature extrema and mean dew point are
combined by OBL-PLANT-P-013.

## Intent

This package is `DIAGNOSTIC_ONLY`. It characterizes source/operator
compatibility and may identify a defect-shaped authority question. It does not
alter production physics, authorize input normalization, resume CAL-07,
publish canopy results, or advance roadmap Order 7.

## Frozen cases

The three Alerce Costero dates diagnosed by CAL-07 are immutable:

- 2022-07-22;
- 2022-09-15; and
- 2025-09-09.

Hourly-average-product evidence will use the same POWER grid point
`(-40.1726, -73.4439)`, Local Solar Time, parameters `T2M,T2MDEW`, and one
exact API request per case date.

Frozen literal requests:

- `https://power.larc.nasa.gov/api/temporal/hourly/point?parameters=T2M,T2MDEW&community=AG&longitude=-73.4439&latitude=-40.1726&start=20220722&end=20220722&format=JSON&time-standard=LST`
- `https://power.larc.nasa.gov/api/temporal/hourly/point?parameters=T2M,T2MDEW&community=AG&longitude=-73.4439&latitude=-40.1726&start=20220915&end=20220915&format=JSON&time-standard=LST`
- `https://power.larc.nasa.gov/api/temporal/hourly/point?parameters=T2M,T2MDEW&community=AG&longitude=-73.4439&latitude=-40.1726&start=20250909&end=20250909&format=JSON&time-standard=LST`

## Prespecified operators

POWER describes the hourly fields as hourly averages. For every retained
hourly-product record:

`hourly_vpd_pa = 1000 * (es(T2M) - es(T2MDEW))`

For every case day:

- `hourly_mean_vpd_pa` is the arithmetic mean of all 24 hourly-product VPD
  reconstructions.
- `hourly_min_vpd_pa` is their minimum; no negative value is clipped.
- `reconstructed_tmin_c` and `reconstructed_tmax_c` are the minimum and
  maximum of the 24 hourly `T2M` values.
- `reconstructed_tdew_mean_c` is the arithmetic mean of the 24 hourly
  `T2MDEW` values.
- `reconstructed_contract_vpd_pa` applies OBL-PLANT-P-013 to those three
  reconstructed daily operands.
- Reported daily operands are read unchanged from CAL-07's frozen forcing.
  CAL-07 comparison VPD is recalculated from those operands; the prior
  diagnostic table is only a cross-check.

Define `E_pa(T) = 1000 * es_kpa(T)`. The additive driver decomposition is
mandatory, in Pa:

```text
mean_hourly_product_vpd
  = mean(E_pa(T_hour)) - mean(E_pa(Tdew_hour))

contract_daily_vpd
  = 0.5 * (E_pa(Tmin) + E_pa(Tmax)) - E_pa(mean(Tdew_hour))

contract_daily_vpd - mean_hourly_product_vpd
  = temperature_extrema_summary_term
  + dewpoint_nonlinearity_term

temperature_extrema_summary_term
  = 0.5 * (E_pa(Tmin) + E_pa(Tmax)) - mean(E_pa(T_hour))

dewpoint_nonlinearity_term
  = mean(E_pa(Tdew_hour)) - E_pa(mean(Tdew_hour))
```

Every term is published separately. The additive identity must close within
`1e-9 Pa`; this is a floating-point reconstruction gate, not a physical
tolerance. Hourly rows also retain
`T2MDEW - T2M`, raw reconstructed VPD, and a half-serialized-unit sign range
formed from the four corners `T2M +/- 0.005 C` and
`T2MDEW +/- 0.005 C`. That range describes sensitivity to displayed
resolution only; it is not uncertainty and never replaces the raw sign.

All saturation vapor pressures use
`es(T)=0.6108*exp(17.27*T/(T+237.3))` in kPa. Every source field must be
finite, must not equal the POWER `-999` fill value, and each case must have
exactly the 24 unique LST keys `YYYYMMDD00` through `YYYYMMDD23`. Missing,
duplicate, UTC, shifted, or contradictory response metadata fails execution.
The response API version, sources, coordinates/elevation, time standard,
retrieval timestamp, byte count, URL, and SHA-256 are retained. The frozen
CAL-07 daily response is not reacquired or replaced.

Daily/hourly product compatibility requires identical grid
coordinates/elevation, LST, Celsius units, non-disjoint reported source lists,
and operand reconstruction within the serialized-resolution tolerance.
Different retained API versions are reported explicitly and prohibit claims
of bit-identical service processing, but do not alone force mismatch when
these product checks pass. Upstream lineage is separately classified as
`EXACT_SOURCE_LIST_MATCH`, `AGGREGATE_OVERLAP_ONLY`, `DISJOINT`, or
`NOT_RESOLVED_FROM_RESPONSE_METADATA`. The CAL-07 multi-year daily source list
cannot establish a date-specific MERRA2 or GEOS-IT lineage; overlap is not
reported as identical processing.

## Frozen attribution rules

Publish these primitive fields before applying an exclusive label:

1. `hour_inventory_valid`;
2. `any_hourly_product_vpd_negative`;
3. `reconstructed_contract_daily_vpd_negative`;
4. `cal07_contract_daily_vpd_negative`;
5. `daily_operands_within_serialized_resolution`;
6. `contract_daily_signs_agree`; and
7. the numeric reconstructed-versus-CAL-07 VPD residual.

CAL-07 comparison VPD is recalculated afresh from its frozen raw daily
operands; the prior negative-day table is only a cross-check.

Daily operand reconstruction passes when every absolute residual is
`<= 0.01 C`, inclusively. This is a serialized-field reconstruction tolerance,
not measurement uncertainty. No independent Pa tolerance is used for
admission; the VPD residual is reported diagnostically. Labels use this
deterministic precedence:

1. `SOURCE_RECONSTRUCTION_MISMATCH` when inventory/metadata/lineage fails, any
   operand exceeds `0.01 C + 1e-12 C`, or reconstructed and CAL-07
   contract-daily signs disagree.
2. `MIXED_PRODUCT_NEGATIVES` when reconstruction passes, at least one
   hourly-product VPD is negative, and reconstructed contract-daily VPD is
   negative.
3. `REPORTED_HOURLY_OPERAND_NEGATIVE` when reconstruction passes, at least one
   hourly-product VPD is negative, and reconstructed contract-daily VPD is
   nonnegative.
4. `DAILY_SUMMARY_OPERATOR_MISMATCH` when reconstruction passes, every
   hourly-product VPD is nonnegative, and reconstructed contract-daily VPD is
   negative.
5. `NO_REPRODUCED_NEGATIVE` otherwise.

These classifications concern published POWER product/operation combinations,
not instantaneous atmospheric state. They do not authorize production
canonicalization or treat a small negative VPD as zero.

## Included scope

- Acquire and digest-bind three exact POWER Hourly API responses.
- Retain the official POWER hourly/daily processing-method references and
  exact request URLs, including retrieval-dated local copies and digests of
  the methodology pages.
- Reconstruct hourly-product and daily quantities independently in package-local
  tooling.
- Produce machine-readable hourly, daily, attribution, and manifest tables.
- Produce accessible plot-only SVG figures with detailed Markdown sidecars.
- Update the CAL roadmap and work-package catalog with the diagnostic result.

## Excluded scope

- No production Rust, science-contract, fixture, test, or CAL-07 input edit.
- No hourly-to-daily replacement forcing for CAL-07.
- No clipping, tolerance normalization, dew-point adjustment, interpolation,
  downscaling, or site-elevation correction.
- No judgment that an aggregation mismatch is automatically a production
  defect; contract authority must decide that separately.

## Authority and dependencies

- CAL-07 package, frozen forcing, negative-day table, incident, and final
  disposition.
- `SC-PLANT-001`, especially OBL-PLANT-P-013.
- NASA POWER Daily and Hourly API/methodology documentation.
- ADR-0042 evidence/claim calibration.

## Intended write set

- This package directory.
- `docs/planning/canopy-phenology-assurance-roadmap.md`.
- `docs/work-packages/README.md`.

All production paths and CAL-07 evidence are protected.

## Phase plan

1. Freeze cases, equations, attribution logic, sources, and write set.
2. Complete two independent prospective reviews.
3. Acquire the exact hourly source objects and build custody manifests.
4. Execute independent hourly/daily reconstruction and attribution.
5. Render and validate figures and Markdown sidecars.
6. Complete dual terminal review, finding disposition, dual verification,
   exact-diff reconciliation, and final disposition.

## Validation requirements

- Exact hourly URL, response metadata, source digest, and 24-hour inventory.
- Independent calculation from retained raw hourly JSON, explicitly bounded
  as hourly-average-product evidence.
- Exact linkage to CAL-07 daily operands and negative values.
- Deterministic table and SVG regeneration.
- SVG XML/accessibility and figure-sidecar/source-binding checks.
- A three-case hourly operand/VPD figure and an additive driver-decomposition
  figure, with all 72 hours visible and no between-case line connection.
- A validator that independently parses raw JSON and reconstructs all hourly,
  daily, decomposition, attribution, and serialized-resolution fields without
  importing analyzer helpers.
- Markdown lint, Python syntax checks, and exact write-set reconciliation.
- Two independent prospective reviews and two independent terminal
  review/verifications.

## Review authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two read-only prospective reviewers and two read-only
terminal review/verifier agents for source authority, operator legitimacy,
attribution, figure integrity, validation, and final claim calibration.
Expected outputs are the four named package review/verification artifacts;
write access is limited to those files.

## Security-impact gate

No credentials, user-controlled command construction, production interfaces,
or runtime behavior are in scope. Network acquisition uses three fixed public
HTTPS URLs and retained response hashes.

## Exit criteria

- Every frozen case has exactly 24 source hours and a deterministic
  attribution under the prespecified rules.
- The diagnostic distinguishes hourly-product reconstruction from the daily
  aggregation operator without changing either source or production behavior.
- All tables, figures, sidecars, validation, reviews, verifications, roadmap,
  catalog, and exact-diff artifacts pass.
- Final wording states explicitly whether CAL-07 remains held and what
  authority/evidence would be required next.
