# CANOPY-CAL-07C Hourly VPD Forcing Reconstruction

Status: `complete / bounded execution / Order 7 hold retained`

Evidence mode: `Ran`

## Objective

Admit, execute, and bound a package-local Alerce Costero forcing
reconstruction that replaces the incompatible CAL-07 daily-summary VPD operand
with the daily mean of published POWER hourly-average paired-product VPD. Then
rerun the frozen CAL-07 Southern Hemisphere evaluation only if the continuous
forcing inventory, custody, and nonnegative VPD gates pass.

## Intent

This package is a forcing-authority and bounded evaluation package. It does
not amend `SC-PLANT-001`, change production Rust, edit CAL-07 evidence, clip
negative VPD, normalize invalid daily contract output, downscale site
meteorology, or declare OBL-PLANT-P-013 replaced. It exercises the already
typed `GsiDailyForcing::vapor_pressure_deficit_pa` input with an explicitly
admitted package-local daily VPD operand derived from hourly POWER products.

CAL-07C may lift CAL-07's immediate input-domain blocker only for this
bounded research execution. It may not claim production readiness or Order 7
completion unless all non-forcing CAL-07 evaluation gates also pass and their
evidence ceilings remain visible.

## Frozen inputs

- CAL-07 source, observation, ensemble, and hold evidence as committed in
  `ab6d84ac`.
- CAL-07B diagnostic attribution as committed in `ab6d84ac`.
- Alerce POWER grid point: latitude `-40.1726`, longitude `-73.4439`, LST,
  parameters `T2M,T2MDEW`, period `2022-01-01` through `2026-07-24`.
- Exact hourly request:
  `https://power.larc.nasa.gov/api/temporal/hourly/point?parameters=T2M,T2MDEW&community=AG&longitude=-73.4439&latitude=-40.1726&start=20220101&end=20260724&format=JSON&time-standard=LST`

## Prespecified forcing operator

For Alerce only:

```text
hourly_product_vpd_pa = 1000 * (es(T2M_hour) - es(T2MDEW_hour))
daily_vpd_pa = mean(hourly_product_vpd_pa over the exact 24 LST hours)
```

For Beza Mahafaly:

```text
daily_vpd_pa = 1000 * (0.5 * (es(T2M_MAX) + es(T2M_MIN)) - es(T2MDEW))
```

All saturation vapor pressures use
`es(T)=0.6108*exp(17.27*T/(T+237.3))` in kPa. No negative value is clipped.
Any missing, duplicate, shifted, nonfinite, `-999`, or negative admitted
daily VPD value fails execution before canopy output.

Hourly-product negatives are not clipped or hidden. They are retained as
signed components of the daily arithmetic mean, counted, plotted, and treated
as a claim-ceiling and review item. Because the typed GSI kernel consumes a
daily `VPD` operand, not subdaily atmospheric state, hourly-product negatives
do not by themselves fail package-local execution when every admitted daily
mean is finite and nonnegative. They do prohibit claims that the hourly POWER
product is everywhere physically admissible or that the result is
production-ready.

The Alerce daily `Tmin` operand remains the frozen CAL-07 daily minimum
temperature because the GSI temperature indicator consumes daily minimum
temperature. Hourly-derived Tmin/Tmax/mean dew point are reconstructed and
compared to the frozen daily operands as source compatibility evidence, but
the admitted correction is only the VPD operand.

## Authority boundary

`SC-PLANT-001` maps the GSI kernel to a typed daily `VPD` operand and the
production runner derives that operand under OBL-PLANT-P-013. CAL-07B proved
the Alerce blocker-date failure is a POWER product/operator mismatch: paired
hourly POWER products are positive on the three CAL-07 negative daily-summary
dates, while the daily summary operator goes negative. CAL-07C's full-period
inventory is broader and retains 349 negative hourly paired-product
components as signed evidence. CAL-07C therefore admits the nonnegative
hourly paired-product daily mean only inside this package-local research
execution, under retained POWER custody and full-period inventory validation.

This is an observed/source-product admission for a bounded evaluation. It is
not a canonical contract amendment. A production change would require a
separate contract-first package.

## Included scope

- Retain and digest-bind the full-period Alerce hourly POWER source object and
  POWER method pages.
- Reconstruct full-period Alerce hourly-product and daily admitted VPD.
- Publish source/admission tables, comparison diagnostics, figures, and
  Markdown sidecars.
- Generate complete package-local CAL-07C inputs, including Beza unchanged
  and Alerce with explicit `vpd_source` provenance.
- Run a package-local executor that passes the admitted `vpd_pa` directly to
  the typed GSI kernel.
- Run the original CAL-07 analysis gates, focused phase/consumer checks,
  terminal validation, dual reviews/verifications, and roadmap/catalog
  updates.

## Excluded scope

- No production Rust, canonical science-contract, fixture, or test edit.
- No CAL-07 or CAL-07B artifact mutation.
- No clipping, deletion, interpolation, lapse-rate correction, or
  site-elevation correction.
- No claim that POWER grid forcing is on-site meteorology.
- No absolute LAI, biomass, canopy-cover, or evergreen-floor calibration.
- No predictive litter/decomposition consequence claim.

## Intended write set

- This package directory.
- `docs/planning/canopy-phenology-assurance-roadmap.md`.
- `docs/work-packages/README.md`.

## Phase plan

1. Freeze authority boundary, source objects, operators, and write set.
2. Complete two independent prospective reviews before result-bearing rerun.
3. Build full-period hourly VPD reconstruction and source/admission manifests.
4. Generate corrected package-local forcing and execute CAL-07C.
5. Analyze observational timing/shape, mass closure, and evidence ceilings.
6. Produce accessible plot-only SVG figures with Markdown sidecars.
7. Complete validation, finding disposition, terminal review/verification,
   exact-diff reconciliation, roadmap/catalog updates, and final disposition.

## Validation requirements

- Full hourly response has exactly 39,984 `T2M` and 39,984 `T2MDEW` LST
  keys, spanning 1,666 complete days from `2022010100` through `2026072423`.
- Every Alerce retained day has exactly 24 contiguous LST hours.
- Every admitted daily VPD is finite and nonnegative.
- Every hourly-product VPD is finite; negative hourly-product rows are
  counted, retained without clipping, and carried into source/claim
  disposition.
- Hourly-derived daily operands reproduce frozen CAL-07 daily Tmin/Tmax/mean
  dew point within the declared serialized-resolution tolerance or the
  mismatch is published and adjudicated before execution.
- The package-local executor publishes complete 37-member, two-site daily
  output only after forcing admission passes.
- Independent validation reconstructs source hashes, hourly/daily VPD,
  package-local executor VPD operands, inventory, mass closure, figures,
  sidecars, and result manifests.
- Markdown lint, Python syntax, package-local Rust `cargo fmt --check` and
  `cargo check`, SVG XML/render checks, and exact diff hygiene pass.
- Two independent prospective reviews, two terminal reviews, and two
  terminal verifications complete with finding disposition.

## Review authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to two read-only prospective reviewers and two read-only
terminal review/verifier agents for source authority, operator legitimacy,
claim calibration, validation, figure integrity, and final disposition.
Expected outputs are the package-local review and verification artifacts;
write access is limited to those files.

## Security-impact gate

No credentials, user-controlled command construction, production interfaces,
or runtime behavior are in scope. Network acquisition used one fixed public
HTTPS URL and retained response hashes.

## Exit criteria

- The Alerce forcing blocker is either lifted for this bounded research
  execution by full-period hourly-product admission, or the package closes on
  hold with the exact authority/inventory blocker.
- If execution proceeds, every CAL-07C daily output row, score, figure, and
  verdict is regenerated from retained sources.
- Final wording states whether Order 7 advances, remains held, or remains
  bounded with named non-forcing blockers.
