# CAL-07B Prospective Methods And Source Review B

Evidence class: `Static`

Review scope: CAL-07B's frozen intent, methods, source plan, attribution rules,
figure plan, and validation plan; CAL-07's retained incident and daily source
custody; `SC-PLANT-001` OBL-PLANT-P-013; and the official NASA POWER Hourly
API, Daily API, meteorological-data, temporal-processing, and time-standard
documentation. No CAL-07B result-bearing source response or derived result was
reviewed.

Recommendation: `HOLD BEFORE RESULT-BEARING ACQUISITION/EXECUTION`

The proposed diagnostic asks the right bounded question and correctly protects
OBL-PLANT-P-013's fail-closed/no-canonicalization boundary. It should not yet
execute because its terminal attribution classes overlap, its current
operators do not quantify the nonlinear terms that could drive the daily/hourly
difference, and several source-resolution and custody details could turn a
published-operand diagnostic into an unsupported claim about physical hourly
states.

## Authority observations

- OBL-PLANT-P-013 requires
  `1000 * (0.5 * (es(Tmax) + es(Tmin)) - es(Tdew))`, rejects every negative or
  non-finite result, and authorizes neither a zero clamp nor bounded negative
  normalization. CAL-07B preserves that boundary.
- POWER documents the Hourly API values as hourly **average** values, not
  instantaneous measurements. It also documents that a timestamp is the start
  of the represented hour.
- POWER documents that both Hourly and Daily APIs default to Local Solar Time
  (LST), that LST is formed from the UTC archive using longitude-based
  15-degree swath offsets, and that this LST may differ from the site's civil
  time zone.
- POWER documents that daily maximum/minimum temperature is taken from 24
  hourly temperatures and other daily meteorological parameters are based on
  hourly averages or sums. Thus reconstructing `T2M_MAX`, `T2M_MIN`, and mean
  `T2MDEW` from the corresponding LST day is an authority-supported diagnostic.
- The POWER meteorology is gridded assimilation/reanalysis evidence. It is not
  an on-site physical observation of either temperature or dew point.

Official sources reviewed:

- [POWER Hourly API](https://power.larc.nasa.gov/docs/services/api/temporal/hourly/)
- [POWER Daily API](https://power.larc.nasa.gov/docs/services/api/temporal/daily/)
- [POWER temporal processing](https://power.larc.nasa.gov/docs/methodology/data/processing/)
- [POWER meteorological data overview](https://power.larc.nasa.gov/docs/methodology/meteorology/)
- [POWER time-information FAQ](https://power.larc.nasa.gov/docs/faqs/other/)

## Findings

### CAL07B-PRB-001 — Blocker: attribution classes are overlapping and not deterministic

The frozen CAL-07 cases all have negative reported daily contract VPD. If the
hourly reconstruction also has a negative hour and reconstructs the daily
operands, both `HOURLY_STATE_INCONSISTENCY` and `MIXED` apply. If reconstruction
does not agree, `SOURCE_RECONSTRUCTION_MISMATCH` can overlap either of those
classes as well. Conversely, a pure hourly-only terminal class is not reachable
for the selected negative-daily cases once source reconstruction passes.

Before acquisition, define a precedence-based terminal decision:

1. any custody, schema, time-basis, inventory, source-lineage, or reconstruction
   failure yields `SOURCE_RECONSTRUCTION_MISMATCH` and prohibits causal
   attribution;
2. after reconstruction passes, any negative published-hourly VPD plus a
   negative reconstructed contract-daily VPD yields `MIXED`;
3. after reconstruction passes, no negative published-hourly VPD plus a
   negative reconstructed contract-daily VPD yields
   `DAILY_SUMMARY_OPERATOR_MISMATCH`; and
4. retain `hourly_negative_count` and `hourly_min_vpd_pa` as diagnostic fields,
   not a competing terminal verdict.

Define “daily calculation” explicitly as the reconstructed
OBL-PLANT-P-013 value. The unchanged CAL-07 reported value remains the selected
case anchor, not a second attribution rule.

### CAL07B-PRB-002 — Blocker: the method cannot yet identify which nonlinear term drives the difference

Comparing mean hourly VPD with contract-daily VPD shows a difference, but does
not explain it. The prespecified table must include the exact additive
decomposition

```text
mean_hourly_vpd
  = mean(es(T_hour)) - mean(es(Tdew_hour))

contract_daily_vpd
  = 0.5 * (es(Tmin) + es(Tmax)) - es(mean(Tdew_hour))

contract_daily_vpd - mean_hourly_vpd
  = [0.5 * (es(Tmin) + es(Tmax)) - mean(es(T_hour))]
  + [mean(es(Tdew_hour)) - es(mean(Tdew_hour))]
```

with every term reported in pascals. The first bracket is the temperature
extrema-summary term. The second is the dew-point nonlinearity/Jensen term.
For the convex saturation-vapor-pressure curve over these temperatures, the
dew-point term is nonnegative; it therefore cannot by itself drive a daily
value downward relative to mean hourly VPD. Without this decomposition, a
label such as “aggregation mismatch” is descriptive but does not answer what
drives the sign.

Also report `T2MDEW - T2M` and VPD for each co-timed published hour. That
distinguishes negative values already visible in paired hourly operands from
loss of co-temporal structure in the daily extrema/mean operator.

### CAL07B-PRB-003 — Major: “instantaneous” and “hourly physical state” overstate the source

The package objective and claim boundary should use “published hourly-average
POWER operands/states.” POWER calls the returned hourly parameters average
values, and its meteorological fields are gridded assimilation/reanalysis
estimates. A negative value calculated from the published pair does not, by
itself, prove negative instantaneous atmospheric VPD at the grid cell or
camera site.

Rename `HOURLY_STATE_INCONSISTENCY`, if retained as a diagnostic flag, to
`PUBLISHED_HOURLY_OPERAND_NEGATIVITY` or similarly bounded wording. Final
claims must distinguish:

- negativity visible in rounded published hourly operands;
- incompatibility introduced by the contract's daily summary operator; and
- actual atmospheric supersaturation, which this design does not establish.

### CAL07B-PRB-004 — Major: serialized resolution is not source uncertainty or an official precision bound

The retained daily JSON serializes temperature fields at 0.01 C resolution.
That does not establish an official measurement precision or uncertainty of
0.01 C, and the reviewed POWER documentation does not promise a rounding mode.
The hourly and daily services may also round independently after aggregating
their internal values.

Replace “within their 0.01 C publication precision” with “within a
prespecified 0.01 C serialized-resolution reconstruction tolerance.” Record
the observed lexical resolution of each retained field and retain raw response
bytes. Explain why 0.01 C and 2 Pa are conservative round-trip comparison
tolerances; do not portray either as a physical tolerance. Use an explicit
inclusive comparison with a documented floating-point epsilon.

For every negative published-hourly value, optionally report a
resolution-sensitivity interval calculated by perturbing each serialized
temperature operand by half of its displayed unit. This may classify whether
the sign is robust to display resolution, but it must not replace the raw
value, clamp it, or authorize production normalization.

### CAL07B-PRB-005 — Major: time basis and exact 24-hour inventory need stronger binding

The selected LST basis matches CAL-07's retained Daily response header and is
appropriate. Do not rely on API defaults: every Hourly request URL must contain
`time-standard=LST`. Validate that each response header independently reports:

- `time_standard=LST`;
- the exact requested start and end date;
- compatible API/source lineage, units, and `-999` fill semantics;
- the returned grid coordinates/elevation; and
- the expected `T2M` and `T2MDEW` parameter identities.

“24 unique, consecutive keys” is insufficient because 24 consecutive hours
could straddle the wrong date. Require the exact key set
`YYYYMMDD00` through `YYYYMMDD23` for the frozen LST date, with no additional
keys. Parse `HH` as 00–23 and bind both parameter maps to the identical set.
Record that each timestamp denotes the start of the represented hour and that
POWER LST is a longitude swath, not Chilean civil time.

### CAL07B-PRB-006 — Major: source custody must rule out archive-version drift before attribution

Retain the exact response bytes, request URL, retrieval timestamp with offset,
HTTP success status, SHA-256 digest, byte count, API version, source list,
messages, units, fill value, geometry, time standard, and start/end metadata
for each case. Bind the CAL-07 daily source object's existing SHA-256 digest,
API version, source list, geometry, and exact three operand rows into the
CAL-07B manifest.

If the hourly service's current source/API lineage cannot be shown compatible
with the frozen daily object, or if its extrema/mean do not reproduce the
daily operands within the declared serialized-resolution tolerance, report
`SOURCE_RECONSTRUCTION_MISMATCH`. Do not infer a daily aggregation mechanism
from data products that may represent different archive revisions.

Retain retrieval-dated copies or digests of the official methodology pages
used to interpret the responses. A live URL alone is not immutable source
custody.

### CAL07B-PRB-007 — Major: figures and validation must expose, not summarize away, the causal evidence

At minimum, produce these plot-only SVGs:

1. a three-case faceted hourly plot with published `T2M` and `T2MDEW` above
   paired hourly VPD, an explicit zero line, every hour visible, and
   color-plus-line-style encoding; and
2. a per-case additive decomposition from mean hourly VPD to contract-daily
   VPD showing the temperature extrema-summary and dew-point Jensen terms.

Do not connect 23:00 on one case to 00:00 on another. Mark signs and values
directly rather than relying only on color. If serialized-resolution
sensitivity is displayed, label it as a rounding envelope, not uncertainty.
Each Markdown sidecar should explain the source, LST date/key meaning,
equations, units, case-selection bias, gridded-source limitation,
serialized-resolution limitation, and no-canonicalization boundary.

The validator should independently parse retained raw JSON rather than trust
derived CSV columns or reuse the analyzer's helper functions. It should
reconstruct all 72 hourly rows and all daily/decomposition rows, require
round-trip-safe numeric serialization, compare the independent result to each
published table, verify attribution precedence, check SVG XML/accessibility
metadata, bind every figure and sidecar to exact source-table digests, and
prove deterministic regeneration. Python syntax and Markdown lint alone are
not scientific validation.

## Accepted protocol elements

- The three dates, point, variables, and no-normalization boundary are frozen
  prospectively.
- `es(T)=0.6108*exp(17.27*T/(T+237.3))` and the pascal conversion match the
  named contract obligation.
- Finite-value and POWER fill-value rejection are required.
- No CAL-07 forcing, production Rust, science contract, or roadmap advancement
  is authorized.
- The package correctly states that even a demonstrated aggregation mismatch
  is not automatic authority to modify production.

## Conditions for `GO`

1. Make the attribution decision exclusive and precedence-ordered.
2. Add the exact temperature-summary/dew-point-nonlinearity decomposition.
3. Replace instantaneous/physical-state and publication-precision claims with
   bounded published-hourly/serialized-resolution language.
4. Require explicit LST requests, exact same-date 00–23 inventories, complete
   response metadata, and daily/hourly lineage compatibility.
5. Bind raw source and methodology custody, independent reconstruction, and
   the two evidence-bearing figures plus sidecars.

After those prospective amendments, CAL-07B is fit to diagnose the three
published POWER cases. No possible CAL-07B result, by itself, authorizes
clipping, tolerance normalization, a replacement forcing series, resumption of
CAL-07, or advancement of canopy roadmap Order 7.

## Superseding prospective re-review

Evidence class: `Static`

Re-review scope: the amended `package.md` and frozen intent after both initial
prospective reviews. No hourly case response or result-bearing CAL-07B output
was present or inspected.

Superseding recommendation: `GO FOR ACQUISITION AND DIAGNOSTIC EXECUTION`

The amended protocol resolves the original closure-blocking findings before
result acquisition:

- the literal requests now select LST explicitly and freeze one URL for each
  immutable date;
- each response must contain exactly the same-date keys `YYYYMMDD00` through
  `YYYYMMDD23`, not merely 24 consecutive records;
- the claim is bounded to POWER hourly-average published operands and
  expressly excludes instantaneous atmospheric state;
- raw response bytes, URLs, retrieval time, hashes, API/source metadata,
  geometry/elevation, time basis, units, and fill semantics are mandatory
  custody;
- the frozen CAL-07 daily object is retained rather than reacquired, and
  source/geometry/time/unit/reconstruction compatibility precedes scientific
  attribution;
- the operand comparison uses an inclusive `0.01 C` serialized-resolution
  reconstruction tolerance, not a physical uncertainty or production
  tolerance; there is no independent Pa admission threshold, and sign
  disagreement fails reconstruction;
- attribution now consumes explicit primitive fields under exclusive
  precedence, with source reconstruction failure dominant and an exhaustive
  fallback;
- the temperature-extrema-summary and dew-point-nonlinearity terms form a
  mandatory additive decomposition, while every hourly row retains the
  co-timed dew-minus-air operand, raw VPD, and a clearly bounded half-unit
  display-resolution sign range;
- the required hourly and additive-decomposition figures expose all 72
  records, include no between-case connection, and require Markdown sidecars;
  and
- an operationally independent validator must parse raw JSON without
  importing analyzer helpers and must reconstruct the hourly, daily,
  decomposition, attribution, and resolution-sensitivity outputs.

For implementation, apply the stated `1000 Pa/kPa` conversion to each
decomposition component—not only to the two endpoint VPD quantities—and
validate the additive identity in pascals at round-trip-safe precision.

One non-blocking documentation correction remains: the exit criterion saying
the diagnostic distinguishes “hourly physical state” should say “published
hourly-average operand behavior.” The frozen intent, attribution rules, and
other claim boundaries already enforce the narrower meaning, so this wording
does not reopen a result-dependent choice or prevent acquisition. It must be
corrected before terminal disposition.

This `GO` authorizes only the three-case, diagnostic-only execution under the
amended protocol. A source reconstruction mismatch must remain an
unattributed source/product-compatibility result. Any other label remains
evidence about the retained POWER operands and operators, not authority to
modify OBL-PLANT-P-013, canonicalize a negative VPD, replace CAL-07 forcing,
resume CAL-07, or advance roadmap Order 7.
