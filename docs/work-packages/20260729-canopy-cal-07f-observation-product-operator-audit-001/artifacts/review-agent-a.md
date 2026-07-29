# CAL-07F Terminal Science Review A

Evidence class: `Static + Ran independent reconstruction`

Verdict: `PASS / DO NOT CALIBRATE / ECOSYSTEM-MODEL LIMITATION`

## Scope reviewed

I independently reviewed the package protocol, CAL-07E source and acquisition
disposition, retained PhenoCam inputs, relevant CAL-07D crossing and
counterfactual evidence, all CAL-07F analysis and validation code, result
tables, three figures and sidecars, calibration-readiness and limitation
adjudications, roadmap, catalog, and exact write set.

The package keeps the two observation products separate and does not select
the product that agrees better with the model. Both remain provisional,
checksum-bound, and `DIAGNOSTIC_ONLY`. GCC thresholds are consistently
described as relative fitted chromatic-coordinate levels rather than GSI, LAI,
leaf cover, biomass, or physiological activity.

## Science and method assessment

The observation-product concern found by CAL-07E is resolved sufficiently for
this stop-loss decision:

- all six dependency hashes and byte sizes match;
- the 731 daily rows are consecutive across 2024--2025;
- all 24 product/year/direction/threshold transitions have a corresponding
  same-direction daily-curve crossing;
- 23 crossings select the nominal transition date exactly;
- the remaining 2024 `gcc_90` falling T25 crossing is 4.625 days later and
  remains inside its reported confidence interval; and
- the materially different falling transition dates are retained rather than
  averaged or silently combined.

The seasonal-window rule is deterministic and product-specific. Each calendar
year is divided halfway between that product's falling T10 and rising T10
dates; falling candidates are selected before the midpoint and rising
candidates after it. This prevents the wrong-season recovery crossings
identified in CAL-07D from being counted as valid event matches.

Independent reconstruction from CAL-07D `all-crossings.csv` reproduced all
888 relative and 296 absolute comparisons exactly. It also reproduced all 74
member summaries. Every member has the same product-specific rank under
`gcc_mean` and `gcc_90`, and `GSI-4831` is lowest-error under both. This is not
a viable calibration result: it has only 9/12 required crossings, hits 1/12
and 0/12 confidence intervals, and has penalized mean absolute residuals of
59.12 and 65.87 days.

The chronology limitation is robust across both products:

- every member crosses falling T10, T25, and T50 in both years;
- every member crosses rising T10;
- only 1/37 and 8/37 members cross rising T25 in 2024 and 2025;
- no member crosses rising T50 in either year; and
- no member supplies all 12 required transitions.

The retained direction pattern supports the bounded interpretation that the
current formulation loses active canopy too early and restores it too late or
not at higher relative levels in this tropical dry-forest lane. It does not
identify rainfall, stored water, rooting-zone access, species composition,
another cue, or their interaction as the missing production process.

## Decision and authority assessment

The six prospective criteria reduce mechanically to two passes and four
failures. Operator independence and mechanical year-role separation pass.
Crossing sufficiency, uncertainty fit, direction coherence, and parameter
plausibility fail.

The parameter-plausibility failure is independently supported by the CAL-07D
counterfactual screen:

- `VPD_UNCONSTRAINED` retains 148/148 matches but has rising and falling
  median residuals of about `+44.49` and `-59.50` days;
- the other three constraint-removal scenarios retain only 64 to 93 matches;
  and
- none retains every crossing while bringing both direction medians inside
  the prospective 21-day tolerance.

Accordingly, another threshold or parameter round against the same two
provisional years is not scientifically warranted. The limitation
adjudication is appropriately restricted to the current CP-GSI01 structure,
accepted 37-member ensemble, frozen Bezà products, and tropical dry-forest
chronology claims. It neither invalidates the calibrated northern deciduous
lane nor authorizes surrogate water or seasonal physics.

Deferring the thesis, post-2011 monitoring, and camera/field correspondence
requests follows the user's stop-loss and does not convert missing evidence
into authority. The reactivation triggers require materially different
evidence or a separately authorized ecosystem-phenology formulation.

## Figures and sidecars

All three SVGs render legibly and use titles, descriptions, axis labels,
marker/line distinctions, and explanatory Markdown sidecars. The product
curve figure makes the falling-product divergence visible; the residual
figure preserves missing crossings rather than plotting them as zero; and the
calibration screen labels the 183-day missing-crossing penalty as a ranking
device rather than a physical or observational bound.

## Finding disposition

| Finding | Severity | Disposition |
| --- | --- | --- |
| No closure-blocking scientific, method, figure, claim-calibration, or write-set defect found | `none` | `accepted`: no corrective edit required |

No open finding remains from Review A.

## Terminal integration recheck

After closure integration, I rechecked the complete package state. Reviewer
B's validator finding is accepted, resolved, and accurately summarized in
`finding-disposition.md`. The strengthened validator now checks the
crossing-sufficiency, uncertainty-fit, and direction-coherence flags
biconditionally and independently reconstructs product scores, ranks,
Spearman correlation, top-quartile overlap, CAL-07D scenario predicates, and
the final decision.

The package status, final disposition, roadmap, and catalog consistently state
`complete / do not calibrate / ecosystem-model limitation adjudicated`.
Order 7 remains not passed. The kickoff prompt is archived, no active prompt
remains, the exact write set contains the stated 38 package files plus the
three declared external documentation edits, and no closure claim broadens
the reviewed science verdict.

## Conclusion

CAL-07F passes as a bounded observation-product/operator audit. Its evidence
supports `DO_NOT_RECOMMEND` for another Bezà calibration round and supports
reporting the retained contradiction as an ecosystem-model limitation for the
assessed lane. Order 7 correctly remains not passed, and further canopy
phenology work remains deferred until a stated reactivation trigger is met.
