# CAL-07E Terminal Science Review B

Evidence class: `Static + independently Ran`

Verdict:
`PASS / METHOD AUDIT AUTHORIZED / SCIENCE CORRECTION NOT AUTHORIZED`.

## Scope reviewed

- package objective, evidence hierarchy, source-admission rules, write set,
  exit criteria, and delegated-review authority;
- all CAL-07E inputs, artifacts, figure/sidecar, and validation code;
- CAL-07D science summary, solution-route evidence, final disposition,
  observation-support table, and retained CAL-07 transition file;
- canopy-assurance roadmap and work-package catalog edits; and
- full-text or primary-method evidence used for the central direct-site,
  regional, stored-water, and PhenoCam-method claims.

## Scientific review

### Source authority and claim calibration

`PASS`.

The source hierarchy is used correctly. Rasamimanana et al. (2012) is the
strongest direct-site phenology authority; Madagascar regional studies and
outside-Madagascar analogues are not promoted into Bezà parameters or process
equations. Discovery records and the inaccessible 2014 thesis remain
acquisition leads rather than claim-bearing evidence.

The terminal matrix correctly separates measured seasonal
leaf-abundance/rainfall association from causal exclusivity. In particular,
the dry-year/early-defoliation and wet-year/late-defoliation response is now
`UNRESOLVED` because the source frames it as a proposed interpretation rather
than a quantified interannual timing test. Temperature is seasonally
correlated but not independently isolated; VPD is not established as an
observed site driver. Gallery/xerophytic chronology is supported, while
groundwater causation remains unmeasured.

### Observation-product semantics and uncertainty

`PASS`.

The review correctly identifies that CAL-07 retained simplified Data Record 7
`gcc_mean` transitions while CAL-07D paired those dates with
`smooth_gcc_90` daily context. The package does not equate relative GCC
amplitude with GSI, LAI, biomass, canopy cover, or physiological activity.
The exact provisional source rows, archive/member checksums, processing date,
retrieval date, confidence intervals, and provider acknowledgement are
retained.

Independent reconstruction confirms all 12 nominal-date comparisons and
chronologically normalized confidence intervals. Four of six falling
threshold-year comparisons have absolute product offsets of at least 10 days:
12, 29, 43, and 21 days. Rising offsets are zero or one day. The synthesis
therefore calibrates the result correctly: the mismatch does not erase the
broad CAL-07D contradiction, but falling residual magnitude is not
product-invariant.

### Recommended next step

`PASS`.

CAL-07F is justified as a bounded observation-product/operator audit. Separate
`gcc_mean` and `gcc_90` lanes, checksum-frozen provisional inputs, uncertainty
propagation, ROI/daily-curve inspection, and a prospectively frozen operator
are the minimum defensible next evidence. The package correctly forbids
choosing a product because it agrees better with the model.

The literature does not yet justify a forcing correction, threshold refit,
ecotype parameter set, water-cue equation, or Order 7 release. Requests for
the 2014 thesis, post-2011 field phenology, colocated site meteorology, and ROI
correspondence metadata are specific and appropriately prioritized.

### Figure and human interpretation

`PASS`.

The SVG renders at 1200 by 720 pixels, includes an accessible title and
description, and is readable without relying on the sidecar. The revised
`Forest habitat chronology` heading does not imply observed groundwater
access, and all circle labels remain on their solid-color backgrounds. The
sidecar supplies a clear caption, source mapping, causal limits, and the
warning that position and size are not effect estimates or source counts.

### Closure truthfulness

`PASS`.

Roadmap and catalog language matches the bounded evidence. The CAL-07
scientific hold remains explicit, production behavior and science contracts
are untouched, and CAL-07E closes only the literature review plus authority
for a method audit.

## Finding disposition

| Finding | Severity | Disposition |
| --- | --- | --- |
| Direct-site dry/wet defoliation timing was initially described as observed rather than author-proposed | Major | `accepted / resolved`: C15 and synthesis now classify it `UNRESOLVED` |
| Falling-product count said three rather than four material cases | Moderate | `accepted / resolved`: prose now says four of six |
| Derived transition audit initially lacked a retained, checksum-bound source subset and validator binding | Major | `accepted / resolved`: eight exact rows are retained and all dates/CIs are source-validated |
| Provisional provider status was not sufficiently prominent | Moderate | `accepted / resolved`: processing/retrieval dates, checksums, caveat, and CAL-07F freeze requirement are explicit |
| Bibliographic title/name and inaccessible-source locator defects | Minor | `accepted / resolved`: register now matches primary records and gives the thesis catalog locator |
| Figure labels overflowed circles and habitat/water wording could imply measured access | Moderate | `accepted / resolved`: labels and heading were revised and rerendered |
| “Needs correction” prejudged the observation-route outcome | Minor | `accepted / resolved`: synthesis now says “needs reconciliation” and retains audit-before-correction |

No open findings remain.
