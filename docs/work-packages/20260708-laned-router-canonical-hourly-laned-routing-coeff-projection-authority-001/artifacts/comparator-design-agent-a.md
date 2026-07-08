# Comparator Design Agent A

Status: `GO-WITH-AMENDMENTS`.
Evidence class: Static comparator-design review.

Reviewer: subagent `019f43cc-a8cf-7d20-acf4-166456051633`.

## Findings

1. Medium: the fidelity envelope did not explicitly separate calibration/rule
   development from validation or holdout cohorts. Disposition: accepted. The
   envelope now requires freezing the coefficient-generation rule before
   acceptance runs and reporting a separate untouched acceptance/holdout cohort
   if exploratory data were used.
2. Low: the reference hierarchy needed sharper wording. Disposition: accepted.
   The envelope now distinguishes source-authorized Lane D candidate/reference
   comparisons, legacy/off compatibility controls, and non-negotiable
   conservation/consumer-closure gates.
3. Low: cohort coverage was surface-based but not stratified. Disposition:
   accepted. The envelope now requires stratification across slope,
   cover/residue state, random roughness range, storm intensity/timing, OFE
   count, and low-mass sediment years.

## Verdict

The envelope is adequate as a non-ratifying future-bridge checklist after the
accepted amendments above.
