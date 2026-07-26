# Frozen CAL-04/CAL-05 Pre-calibration Protocol

Evidence class: `Static: protocol frozen before calibration`

This protocol is immutable for CAL-04 and CAL-05 unless a prospective package
amendment identifies the newly admitted research object, updates checksums and
roles, and receives independent review before any affected fit is run. CAL-03
does not fit a parameter.

## Authority and partitions

The canonical record table is
`tests/fixtures/cancov_forest/observations/canopy_phenology/records.csv`.
`calibration-holdout-ledger.csv` assigns every record exactly one frozen role.
Only `OBSERVATION` records with a calibration role may contribute to an
objective. A calibration screen may reject impossible behavior but may not
create a scalar loss from qualitative wording. Process and downstream holdouts
cannot select parameters, weights, bounds, priors, missing-data policy,
tolerances, or stopping rules.

Bill's values remain `LEGACY_COMPARISON`; his selected controls remain
`FITTED_OPERAND`; CAL-02 results remain `MODEL_OUTPUT`; calculated quantities
remain `DERIVED_DIAGNOSTIC`. None is independent correctness authority.

The current freeze has three hard evidence gaps:

1. no independent quantitative phenology-timing holdout is retained;
2. no retained evidence admits probability priors for native parameters; and
3. no site-matched measured leaf/needle/fine-woody source composition is
   retained.

Consequently CAL-04 parameter fitting must not start from this corpus alone.
CAL-05 may perform source-sufficiency analysis with explicit unknown terms, but
must not fit decomposition or declare source adequacy until its required source
and stock comparisons are jointly admissible.

## Process order and freeze points

CAL-04 must proceed in this order:

1. GSI timing;
2. deciduous foliar maximum and persistent structural biomass, with total
   biomass used only as their partition sum;
3. evergreen fraction;
4. peak LAI;
5. winter canopy floor and summer closure; and
6. downstream evaluation after accepted upstream ranges are frozen.

The intended native operands are GSI thresholds, `Bf,max`, `Bs`, `fe`,
`xmxlai`, `Cs`, and `bb` in that order. An accepted range at one stage is
frozen before the next stage. Reopening it requires a recorded finding and a
prospectively reviewed joint-fit plan. Runoff, erosion, snow, frost,
interception, and ET never select these operands.

CAL-05 consumes the frozen canopy ensemble. It evaluates source sufficiency
before decay: deciduous leaf transfer, recurring evergreen needle turnover,
fine-woody input, then total litter. Only after supported source compositions
are identified may decay be tested against forest-floor storage. A missing
source cannot be compensated by changing foliar mass, LAI, or decay.

## Objective functions and uncertainty

For an admitted scalar observation with standard error `s > 0`, report the
signed residual and standardized residual `(model - observation) / s`. Do not
pool residuals across quantities or units. For a source-reported interval,
loss is zero inside the closed interval and the signed distance to the nearest
bound outside it; report both native-unit and interval-width-normalized
distance when the interval width is positive.

Qualitative calendar phrases and proxy quantities are pass/fail screens in
their source wording. “Early May,” “end of May,” “late June,” “late
September,” “late October,” “a little less than 6,” PAR transmission, and
phenology-index meanings must not be converted to invented dates, canopy
cover, or Gaussian errors. An observation lacking an uncertainty or interval
is reported in native units but receives no invented weight.

No aggregate objective may be used until every component loss, unit,
normalization, missing member, and weight is retained. If aggregation is later
authorized, weights must be fixed prospectively from observation authority,
not chosen to improve fit. Calibration and holdout scores are always separate.

## Bounds and priors

Hard bounds may come only from the controlling science contract, typed schema,
or newly admitted independent evidence. The search record must distinguish
contract domain bounds from scientific plausibility bounds. CAL-01 Bill
operands and current native values are comparison or initialization points,
not priors and not evidence-derived bounds.

No probability prior is admitted by the retained corpus. Uniform sampling
within a domain is a search design, not a uniform scientific prior, and must
be labeled accordingly. A boundary hit is retained as an identifiability
finding; widening a boundary after viewing results requires prospective
amendment.

## Missing-data and matching rules

- Missing, non-finite, inapplicable, and zero are distinct states.
- Never interpolate an observation unless its retained source already did so
  and flags the operation.
- Match site, canopy class, date or stated temporal window, spatial support,
  material class, stock/flux boundary, and measurement basis before scoring.
- Do not compare PAR transmission directly to canopy cover.
- Do not compare standing foliage to annual litterfall.
- Do not merge leaf, needle, fine-woody, fruit, or aggregate litter.
- Do not merge Hubbard loss-on-ignition forest-floor mass with Santee oven-dry
  bulk mass in one objective.
- Harvard hemlock remains unbound until a pure-conifer fixture exists.
- A failed or missing model day never causes the corresponding observation to
  disappear; it is reported as a failed member.

## Diagnostics and equilibrium

Annual diagnostics use calendar-year daily records:

```text
gross_leaf_on = sum(nonnegative daily leaf-on allocation)
gross_leaf_off = sum(nonnegative daily leaf-off transfer)
net_foliar_change = year_end_foliar - year_start_foliar
seasonal_amplitude = max(daily deciduous foliar) - min(daily deciduous foliar)
phenology_churn_ratio = gross_leaf_off / seasonal_amplitude
```

Churn is null, not zero, when amplitude is zero. The independent foliar ledger
must reconcile start stock plus leaf-on minus leaf-off to end stock within the
declared numeric accumulation tolerance.

The frozen practical-equilibrium screen follows CAL-02: over years 91–100,
`(maximum year-end stock - minimum year-end stock) / mean year-end stock <=
0.02`, with a positive finite mean. Also report seasonal range, first year
meeting the rule for every subsequent ten-year window, and signed
year-over-year drift. This numerical screen is not proof that flux composition
or decay is correct.

The analysis-only current/previous/old cohort ledger is driven by the exact
native daily litter source and its declared decay. Its sum must reconcile to
aggregate production residue only when all cohort and production equations,
chronology, and environmental modifiers are equivalent. Otherwise report the
first divergent operand and classify the comparison inapplicable rather than
forcing closure.

## Identifiability record

Every stage must retain all attempted vectors, deterministic seed or
deterministic grid identity, bounds, objective components, convergence state,
failed-run reason, boundary hits, and calibration/holdout membership. Report
parameter correlations and either profile likelihood or ensemble uncertainty.
Retain materially equifinal solutions; a single best vector is insufficient.

## Failure classifications

| Code | Meaning | Required disposition |
| --- | --- | --- |
| `AUTHORITY_BLOCKED` | Required observation, role, bound, or prior is absent. | Stop fitting; admit evidence prospectively or hold. |
| `INPUT_INVALID` | Fixture/schema/units/binding is invalid. | Correct input provenance before rerun. |
| `MODEL_FAILED` | Typed runtime, non-finite value, or incomplete chronology. | Retain failed member and error; do not impute. |
| `LEDGER_NONCLOSURE` | Independent foliar, mass, or equivalent cohort ledger fails. | Stop scientific scoring and diagnose first divergence. |
| `NONIDENTIFIABLE` | Retained range/profile does not constrain the operand. | Report range/equifinality; do not select a point. |
| `BOUNDARY_HIT` | Accepted solution touches a search bound. | Report; do not widen retrospectively. |
| `CALIBRATION_FAIL` | Calibration authority misses its frozen envelope. | Do not score downstream consequences as validation. |
| `HOLDOUT_FAIL` | A frozen holdout misses after calibration is frozen. | Preserve result; do not refit or relax tolerance. |
| `PROCESS_INAPPLICABLE` | Quantity or equation is not comparable. | Report mismatch in basis; do not coerce. |
| `PASS_BOUNDED` | Every applicable frozen criterion passes. | Advance with limitations and retained uncertainty. |

No status can be converted to `PASS_BOUNDED` from ordering, visual agreement,
legacy parity, or self-consistency alone.
