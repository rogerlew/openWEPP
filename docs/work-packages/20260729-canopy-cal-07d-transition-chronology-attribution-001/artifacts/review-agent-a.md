# CAL-07D Terminal Review A

Evidence class: `Static + Ran`

Verdict: `PASS / ORDER 7 HOLD RETAINED`

## Scope

Independent terminal review covered the exact CAL-07D worktree, package
protocol, both prospective reviews, retained sources and manifests, analysis
and validation code, result tables, scientific synthesis, four figures and
their sidecars, roadmap/catalog changes, `SC-PLANT-001` CP-GSI01 and
OBL-PLANT-P-013 boundaries, and ADR-0042.

## Findings

No closure-blocking scientific or claim-calibration defect was found.

1. Source custody is exact. The dependency manifest binds ten retained objects
   to CAL-07C commit
   `11b1faab37b5d365b0c0c7051ed32a4762821239`; the two source-native Beza CSV
   digests match the prospectively frozen values.
2. The source-level analogy is internally consistent and bounded correctly.
   Falling source dates occur in the order level `0.50`, `0.25`, then `0.10`,
   so `q=p` preserves the observed remaining-state order. The package labels
   that mapping and every event-year relative threshold
   `ASSUMED_FOR_EXECUTION`; it does not equate GCC with GSI, LAI, biomass, or
   canopy cover.
3. CAL-07C is reproduced rather than reinterpreted. All 61,642 BASE
   member-days reconstruct with maximum equation residual `0.000e+00`, and all
   148 absolute event rows reproduce the prior crossing availability,
   selected crossing, residual, and same-direction count, including the same
   11 matches.
4. The attribution predicates match the retained evidence. Observation scale
   recovers 262 previously unmatched member/event/source-level rows.
   Temperature, VPD, and photoperiod substitutions alter 98, 248, and 296
   member/event/operator rows respectively. These are mathematical
   contributors, not identified biological causes or correction authority.
5. The scientific synthesis preserves non-identifiability. POWER is gridded
   forcing, the PhenoCam operator remains diagnostic-only, no tropical
   dry-forest parameter authority is admitted, and no water-status or
   missing-process equation is inferred. The evidence cannot separate forcing
   bias, ecotype transfer, observation semantics, and an omitted seasonal or
   water cue.
6. Counterfactual claims stay below their authority ceiling. Indicator removal
   and the canonical generalized default are sensitivity tests; no threshold,
   forcing correction, parameter refit, production default, or contract
   amendment is recommended.
7. The four SVG plots are plot-only, renderable, source-bound, and
   interpretable with color-independent marker/line distinctions and Markdown
   ancillary information. Regeneration from the bound CSVs produced
   byte-identical SVGs and sidecars.
8. The roadmap and catalog accurately retain Order 7 on hold. CAL-07D explains
   the contradiction but does not resolve it; amplitude, evergreen-floor, and
   decomposition evidence also remain unevaluated.

## Non-deferral and disposition

All prespecified scientific diagnostics have direct current-package evidence.
The remaining on-site meteorology, field/image corroboration, ecotype
authority, water-status evidence, and independent deciduous lane are named
authority/evidence boundaries for possible successor work, not deferred
CAL-07D acceptance gates.

At this review snapshot, Markdown lint, exact terminal-diff hygiene, the
sibling terminal review, and final disposition are still terminal-integration
steps marked pending in `gate-evidence.md`. They must be reduced to explicit
terminal results before the package's `complete` status is finalized. This
does not change Review A's scientific verdict.
