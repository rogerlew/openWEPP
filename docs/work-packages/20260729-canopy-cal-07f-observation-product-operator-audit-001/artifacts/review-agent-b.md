# CAL-07F Terminal Science Review B

Evidence class: `Ran + Static`

Terminal verdict: `PASS`

No closure-blocking science, observation-product, anti-cherry-picking,
calibration-role, visualization, or claim-calibration finding remains.

## Scope reviewed

I independently reviewed:

- the root and work-package agent instructions;
- ADR-0042 and the CAL-07F package and kickoff protocol;
- CAL-07E source, product-audit, acquisition-deferral, and authority evidence;
- CAL-07D crossings, counterfactual screen, science summary, and solution-route
  evidence;
- every CAL-07F source, tool, table, synthesis document, figure, and sidecar;
- the changed canopy roadmap and work-package catalog entries; and
- the exact declared write set.

## Scientific review

### Observation products and source limits

The two observation lanes remain separate throughout the audit.
`gcc_mean` and `gcc_90` use their own thresholds, transition dates, confidence
intervals, daily curves, and seasonal windows. No result substitutes one
product for the other, and no product is promoted because it fits the model
better.

The retained evidence is explicitly bounded to the provisional product
processed 26 July 2026, the retained ROI, and exact dependency hashes. GCC is
not represented as GSI, LAI, canopy cover, biomass, or physiological activity.
All source observations remain `DIAGNOSTIC_ONLY`.

Independent source reconstruction found:

- 731 consecutive daily rows spanning 2024–2025;
- 24 product/year/direction/threshold transitions;
- 23 exact nominal daily-curve crossings;
- one `gcc_90` 2024 falling T25 crossing 4.625 days after the nominal date but
  within its reported confidence interval;
- zero retained GCC outlier flags; and
- 21 provider interpolation flags, all disclosed.

These limits support a bounded diagnostic stop-loss, not calibration,
validation, or biological-observation-operator authority.

### Seasonal-crossing semantics

The primary selection is transparent and prospective: for each product and
year, the midpoint between falling T10 and rising T10 divides the calendar
into falling and rising windows. This is an analysis convention that prevents
an opposite-season recovery crossing from satisfying the requested seasonal
event. It is not presented as a biological transition equation.

The retained reduction is exact:

- all 37 members supply each falling T10/T25/T50 crossing in both years and
  products;
- all 37 supply rising T10;
- rising T25 is available for 1/37 members in 2024 and 8/37 in 2025;
- rising T50 is available for 0/37 members in both years; and
- therefore no member supplies all 12 seasonal crossings in either product.

I also ran a deliberately permissive anti-cherry-picking sensitivity that
accepted the nearest same-direction crossing anywhere in the same calendar
year, without the midpoint partition. All 37 members then had 12 crossings,
but zero passed the joint uncertainty and direction screen under either
product. For the best joint member, `GSI-4831`, the permissive result had:

| Product | CI hits | Median absolute residual | Rising median signed residual | Falling median signed residual |
| --- | ---: | ---: | ---: | ---: |
| `gcc_mean` | 1/12 | 23.408 days | -76.847 days | -23.408 days |
| `gcc_90` | 0/12 | 36.094 days | -77.347 days | -26.261 days |

Thus crossing completeness depends, appropriately, on whether
wrong-season recoveries count, but the no-calibration decision does not. This
stress test is reviewer evidence and does not replace the frozen primary
operator.

### Calibration screen and empirical role

The 21-day threshold was frozen before execution and is explicitly described
as an adjudication tolerance tied to the implemented GSI averaging window. It
is not mislabeled as observational uncertainty, a physiological bound, or a
confidence interval. The 183-day missing-crossing penalty affects ranking
only and is likewise disclosed as a ranking device.

The primary screen independently reconstructs:

- zero members complete for all 12 transitions in both products;
- zero members passing the uncertainty criterion in both products;
- zero members passing direction coherence in both products;
- identical product ranks and 100% top-quartile overlap; and
- `GSI-4831` as the lowest penalized-error member, with 1/12 and 0/12
  confidence-interval hits and penalized errors of 59.125 and 65.875 days.

The empirical-role criterion passes only in the narrow mechanical sense that
one year could be excluded from fitting. The package correctly says that such
a holdout would be internal and does not reassign either current year from
`DIAGNOSTIC_ONLY` or call it external validation. This pass cannot rescue the
four failed scientific criteria.

CAL-07D supplies no parameter-only correction direction that meets the frozen
rule. Independent reconstruction found:

| Counterfactual | Complete rows | Falling median | Rising median |
| --- | ---: | ---: | ---: |
| VPD unconstrained | 148/148 | -59.497 days | +44.492 days |
| Photoperiod unconstrained | 93/148 | -21.434 days | -42.886 days |
| Photoperiod and VPD unconstrained | 85/148 | +30.344 days | +7.418 days |
| Temperature unconstrained | 64/148 | -66.589 days | no matched rising rows |

None retains all crossings while bringing both direction medians within
±21 days. These are mathematical counterfactuals, not process authority.

### Limitation adjudication

The no-calibration result follows from both products and four independently
failed criteria; it does not depend on a favorable operator choice or on
`GSI-4831` selection. The ecosystem-model-limitation language is appropriately
scoped to the current CP-GSI01 structure, the frozen 37-member ensemble, the
checksum-bound provisional Bezà ROI and transitions, and tropical dry-forest
chronology/transferability claims. It does not invalidate the northern
calibration or claim to identify a replacement mechanism.

The stop-loss and reactivation boundary match the evidence. Another threshold
sweep, mutable provisional-data refresh, or generic literature search would
not resolve the missing biological operator or process authority. Field-linked
observations, authoritative process science, or an independently testable
alternative formulation are defensible reactivation triggers.

## Figures and human interpretation

I rendered and visually inspected all three SVGs. Labels, axes, direction
encoding, threshold bands, missing-crossing counts, and color contrast are
readable. The figures contain SVG title/description metadata and each has a
complete Markdown caption and ancillary-information sidecar. The sidecars
state the source identities, scoring semantics, missing-value treatment, and
prohibited interpretations.

## Finding disposition

| Finding | Severity | Disposition |
| --- | --- | --- |
| The first validator version checked uncertainty and direction pass flags only one way and did not reconstruct the operator and parameter criteria independently. This could have admitted false-negative decision flags. | Major | `accepted / resolved`: the validator now checks summary flags biconditionally and independently reconstructs product scores/ranks, Spearman correlation, top-quartile overlap, CAL-07D scenario completeness and direction medians, the operator criterion, the parameter criterion, and the final decision. The strengthened validator passes. |

No further finding remains.

## Conclusion

CAL-07F truthfully answers the bounded question. The retained evidence does
not justify another Bezà timing calibration round. Deferring further work and
reporting the scoped tropical dry-forest chronology mismatch as an
ecosystem-model limitation is the scientifically conservative disposition.
