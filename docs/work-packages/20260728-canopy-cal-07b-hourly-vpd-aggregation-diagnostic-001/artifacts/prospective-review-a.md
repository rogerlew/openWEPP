# CAL-07B Prospective Scientific Review A

Evidence class: `Static: contract, frozen predecessor evidence, and official
NASA POWER documentation`

Review verdict: `HOLD before acquisition`

## Scope and authority checked

This review examined the frozen CAL-07B protocol, the CAL-07 negative-VPD
incident and retained daily-source lineage, and `SC-PLANT-001`
OBL-PLANT-P-013. It also checked the current official NASA POWER descriptions
of:

- [temporal processing](https://power.larc.nasa.gov/docs/methodology/data/processing/);
- [meteorological data processing](https://power.larc.nasa.gov/docs/methodology/meteorology/);
- the [Hourly API](https://power.larc.nasa.gov/docs/services/api/temporal/hourly/);
  and
- the [Daily API](https://power.larc.nasa.gov/docs/services/api/temporal/daily/).

NASA states that initial meteorological data are hourly and daily values are
formed by the mean, maximum, or minimum of 24 hours. It specifically states
that daily temperature extrema are obtained from the 24 hourly temperature
values while other meteorological parameters are based on hourly averages or
sums. The API documentation also states that hourly parameters are hourly
**average values**, not instantaneous observations. Both APIs default to
Local Solar Time, but both allow the time standard to be selected explicitly.

OBL-PLANT-P-013 authorizes only the existing daily contract calculation:

`1000 * (0.5 * (es(Tmax) + es(Tmin)) - es(Tdew))`

and requires every negative result to hard-fail. CAL-07 did so correctly. The
hourly calculation in CAL-07B is diagnostic and is not a replacement
production operator.

## Ranked findings

### P0-1 — “Instantaneous hourly” overstates the source and the attainable claim

The intent calls the diagnostic quantities “instantaneous hourly saturation
deficits.” NASA's Hourly API describes its parameters as values averaged by
hour. Consequently:

- `es(hourly T2M) - es(hourly T2MDEW)` is a reconstruction from two published
  hourly-average operands;
- a negative result establishes a negative deficit reconstructed from the
  rounded hourly POWER product;
- it does not establish an instantaneous negative atmospheric VPD, physical
  supersaturation at the site, or even a negative mean of sub-hourly VPD,
  because the nonlinear saturation function does not commute with temporal
  averaging.

Rename “instantaneous hourly” throughout the frozen intent and claim chain to
“POWER hourly-product” or “hourly-average-operand reconstruction.” Rename
`HOURLY_STATE_INCONSISTENCY` to a similarly bounded label such as
`REPORTED_HOURLY_OPERAND_NEGATIVE`. Figures and sidecars must carry the same
ceiling. The package may attribute the sign to a published product/operator
combination, not to the physical atmospheric state.

Disposition: `closure-blocking; amend before acquisition`.

### P0-2 — The attribution rules are overlapping and not exhaustive

The four labels do not presently define one deterministic partition:

- hourly negatives plus a negative reconstructed daily result satisfy
  `MIXED`, but may simultaneously satisfy
  `SOURCE_RECONSTRUCTION_MISMATCH`;
- “the daily summary calculation” in `MIXED` does not say whether it means
  reconstructed daily VPD, CAL-07 reported-operand VPD, or both;
- the 2025 CAL-07 value is only `-1.00224 Pa`. The reconstructed hourly
  operands could pass the stated `0.01 C` and `2 Pa` source tolerances while
  producing a small positive reconstructed daily VPD. That case would satisfy
  none of the current labels;
- a reprocessed hourly object could reconstruct no negative at either scale
  and still require a deterministic source-mismatch disposition.

Freeze primitive Boolean fields first:

1. all 24 hourly source records valid;
2. any hourly reconstructed VPD negative;
3. reconstructed contract-daily VPD negative;
4. CAL-07 reported-operand contract VPD negative;
5. every daily operand within its declared reconstruction tolerance;
6. reconstructed versus CAL-07 contract-VPD residual within tolerance; and
7. reconstructed and CAL-07 contract-VPD signs agree.

Then either publish those axes without a forced exclusive label or specify
precedence. A defensible precedence is:

1. `SOURCE_RECONSTRUCTION_MISMATCH` whenever an operand, VPD tolerance, or
   sign check fails;
2. `MIXED` when source reconstruction passes and both an hourly negative and
   a negative reconstructed daily result exist;
3. `REPORTED_HOURLY_OPERAND_NEGATIVE` when source reconstruction passes,
   hourly negatives exist, and reconstructed daily VPD is nonnegative;
4. `DAILY_SUMMARY_OPERATOR_MISMATCH` when source reconstruction passes, all
   hourly reconstructions are nonnegative, and reconstructed daily VPD is
   negative; and
5. an explicit `NO_REPRODUCED_NEGATIVE` fallback.

The sign check is essential for the near-zero 2025 case. Without it, a
precision-induced sign reversal can be mislabeled as successful source
reconstruction and then silently escape attribution.

Disposition: `closure-blocking; amend before acquisition`.

### P1-1 — The exact time standard must be encoded in each frozen URL

The package says Local Solar Time and relies on the current API default.
Because day membership is the central diagnostic variable, an implicit
default is not adequate custody. Freeze each literal request URL before
acquisition with `time-standard=LST`, along with the exact `community`,
coordinates, parameters, date, and format. Retain response API version,
header/time-standard metadata, retrieval timestamp, byte count, and SHA-256.

The CAL-07 daily URLs omitted the explicit time-standard argument but their
retained responses report LST. CAL-07B should compare against those retained
objects, not reacquire or silently replace them. If current hourly data do not
reconstruct the frozen daily object, the result is a source-version/product
compatibility finding; it cannot be attributed solely to an aggregation
operator.

Disposition: `must fix before acquisition`.

### P1-2 — The temperature tolerance is plausible; the 2 Pa tolerance needs a
frozen derivation or should be secondary

The `0.01 C` operand threshold is defensible only as a reconstruction
tolerance for two-decimal published fields. Define it as inclusive
(`absolute residual <= 0.01 C`) and verify from response metadata that both
hourly and daily fields are actually published at that precision. Do not call
it measurement uncertainty.

The `2 Pa` threshold is not source publication precision: POWER publishes
temperature/dew-point operands, while the contract VPD is locally derived.
Its value must therefore be:

- justified prospectively by a rounding-error envelope through the exact
  `es(T)` equation over each frozen day's known daily temperature
  neighborhood; or
- removed as an independent source-admission rule and retained only as a
  reported diagnostic residual after operand and sign agreement.

The protocol must also state inclusive comparison semantics and calculate the
CAL-07 comparison VPD afresh from the frozen CAL-07 operands with the exact
specified equation. A copied value from a prior diagnostic table is useful
cross-check evidence, not an independent operand.

Disposition: `must fix before acquisition`.

### P1-3 — “Consecutive LST hour keys” requires an exact machine rule

For each one-day request, require keys `YYYYMMDD00` through `YYYYMMDD23`
exactly once, in addition to count 24. “Consecutive” alone could admit a
shifted 24-hour interval. Parse the response-declared time standard and reject
UTC or missing/contradictory metadata. Retain `-999`, missing, non-finite, and
duplicate-key failures as package execution failures, not scientific
attributions.

Disposition: `must fix before acquisition`.

### P2-1 — Daily-mean VPD and contract-daily VPD must remain distinct

The proposed operators correctly distinguish:

- mean hourly-product VPD:
  `mean(es(T2M_h) - es(T2MDEW_h))`; and
- reconstructed contract-daily VPD:
  `0.5 * (es(max(T2M_h)) + es(min(T2M_h)))
  - es(mean(T2MDEW_h))`.

This distinction is the diagnostic's scientific value. It must be retained
in tables and figures. Because of nonlinearity and the use of daily extrema,
the two quantities are not expected to be equal. “Mismatch” should mean the
daily contract operator yields a negative value despite no negative
hourly-product reconstruction; it must not mean the two legitimate
aggregations merely differ numerically.

Disposition: `required claim calibration`.

## Evidence ceilings

Even after the findings above are corrected, CAL-07B can establish only:

- whether the retained/current POWER hourly product contains negative
  reconstructed deficits at the three frozen dates;
- whether its 24 hourly records reconstruct the retained CAL-07 daily
  operands within declared publication-rounding tolerances; and
- whether combining daily extrema and mean dew point through
  OBL-PLANT-P-013 creates a negative value when the hourly-product
  reconstructions are all nonnegative.

It cannot establish site-observed atmospheric truth, authorize a clamp or
dew-point adjustment, validate an alternative production VPD operator, or
resume CAL-07. If hourly and retained daily products fail source
reconstruction, it also cannot identify aggregation as the cause; source
release/version or service-product differences remain live explanations.

## Prespecification verdict

`HOLD before acquisition`.

The equations and frozen dates are suitable, and NASA's published processing
hierarchy supports the intended daily-from-hourly comparison. Execution is
not yet free of post-result choices because the source is described at the
wrong temporal evidence level, attribution labels overlap and omit a
near-zero sign-disagreement case, the literal LST URLs are not frozen, and
the `2 Pa` threshold lacks a prospective derivation. Amend those items before
retrieving results; after amendment, this reviewer would support `GO` for the
diagnostic-only execution.

## Superseding review after protocol amendment

Evidence class: `Static: amended protocol reinspection`

Superseding verdict: `HOLD before acquisition`

The amendment resolves the original P0 findings and most P1 findings:

- hourly evidence is now correctly bounded to published hourly-average
  operands;
- all three literal requests explicitly select LST;
- exact `00` through `23` keys and response metadata are required;
- primitive axes and source-mismatch-dominant exclusive precedence now cover
  source drift, near-zero sign disagreement, mixed negatives, and the
  no-negative fallback;
- the ungrounded Pa admission tolerance is gone;
- the inclusive temperature reconstruction threshold is explicitly a
  serialized-field tolerance;
- CAL-07 VPD is recomputed from its retained raw operands;
- half-unit sign sensitivity is kept separate from raw-sign attribution; and
- independent raw-JSON reconstruction and the required figures are binding.

Those changes make the main attribution decision mechanical. Three narrow
issues remain in the exact current text.

### Amendment finding A — decomposition units omit the required `1000`

The protocol says the additive decomposition is “in Pa,” but each displayed
equation uses `es(...)` directly. The package has already defined `es(T)` in
`kPa`, so the displayed decomposition terms are in kPa unless every equation
is multiplied by `1000`. This is not merely typography because the required
machine-readable fields and figure will label the terms in Pa.

Specify one of these equivalent conventions before execution:

- define `E_pa(T) = 1000 * es_kpa(T)` and use `E_pa` throughout the
  decomposition; or
- multiply the right-hand side of the mean VPD, contract VPD, temperature
  term, and dew-point term equations by `1000`.

The validator should require the additive residual in Pa to close within a
prospectively declared floating-point tolerance.

Disposition: `closure-blocking protocol correction`.

### Amendment finding B — source compatibility cannot require MERRA-2 for
every frozen date

The daily CAL-07 object spans 2022--2026 and reports the aggregate source list
`GEOSIT, MERRA2, POWER`. NASA describes MERRA-2 as retrospective and GEOS
forward processing as the near-real-time source. Requiring “a shared MERRA2
source lineage” can therefore force the 2025 case to source mismatch even if
the daily and hourly products are correctly aligned on the same GEOS/GEOS-IT
lineage.

Require compatible source lineage for the case date, not MERRA-2
specifically. Preserve the exact source lists and API versions from both
objects. Where the retained multi-year daily response exposes only an
aggregate source list and cannot prove the date-specific upstream source,
label date-specific lineage `NOT_RESOLVED_FROM_RESPONSE_METADATA`; do not
silently equate an intersecting aggregate list with proof of identical
processing. Geometry, LST, units, operand reconstruction, and sign comparison
can still support product compatibility at the declared serialized level.

Disposition: `closure-blocking protocol correction`.

### Amendment finding C — two stale physical-state claims remain

The objective still asks whether negative values are present in hourly
temperature/dew-point “states,” and the exit criterion says the diagnostic
distinguishes “hourly physical state” from daily aggregation. The amended
operator section correctly disclaims that evidence level.

Replace those phrases with “published hourly-average operands/product” and
“hourly-product reconstruction,” respectively. Otherwise a package can
satisfy its literal exit criterion only by exceeding the evidence ceiling.

Disposition: `closure-blocking claim correction`.

One additional ambiguity should be removed while making these corrections:
the operator section says reported CAL-07 contract VPD is “read unchanged”
from the diagnostic table, while the attribution section correctly requires
it to be recalculated from raw frozen daily operands. State that the operands
are read unchanged, VPD is recalculated, and the prior table is cross-check
only.

After these exact corrections, this review supports `GO` without another
scientific redesign. The case set, operators, attribution precedence,
serialized-resolution sensitivity, independent reconstruction, and evidence
ceilings are otherwise adequately prespecified for diagnostic-only
execution.

## Final superseding review after targeted corrections

Evidence class: `Static: final pre-acquisition protocol inspection`

Final prospective verdict: `GO`

No hourly response or result artifact was present at this inspection. The
remaining corrections were made before acquisition:

- the objective and exit criterion now refer only to reconstructions from
  published hourly-average operands;
- the decomposition defines `E_pa(T) = 1000 * es_kpa(T)` and consistently
  uses it for every term;
- its mandatory additive identity has a prospectively frozen `1e-9 Pa`
  numerical closure gate;
- source custody preserves exact source lists and distinguishes exact match,
  aggregate overlap, disjoint sources, and lineage not resolved from response
  metadata without assuming MERRA-2 for 2025;
- overlap is expressly not evidence of identical processing;
- unchanged CAL-07 daily operands are the comparison input, CAL-07 VPD is
  recomputed from them, and the prior table is cross-check only.

Read with the earlier amendments, the protocol now fixes the three cases,
literal LST requests, exact hour inventory, equations, serialized-resolution
semantics, primitive evidence axes, exclusive attribution precedence,
decomposition, sign sensitivity, figures, independent raw-JSON validation,
and claim ceilings before observing results. No result-dependent scientific
choice remains apparent.

`GO` authorizes the diagnostic-only acquisition and execution exactly as
specified. It does not authorize production changes, a replacement VPD
operator, any normalization of negative VPD, resumption of CAL-07, or
advancement of roadmap Order 7.
