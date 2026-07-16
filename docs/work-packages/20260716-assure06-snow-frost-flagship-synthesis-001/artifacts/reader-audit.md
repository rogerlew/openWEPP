# ASSURE-06 Rendered Reader Audit

Evidence class: Ran against the final disposable build.

The rendered report answers the scientific claim-envelope questions without
requiring the machine descriptor:

1. **What was evaluated?** Hourly precipitation phase; daily SWE, depth, and
   density response; method-specific frost-depth and soil-temperature response;
   and production partition/storage identities.
2. **At what support?** Hourly station precipitation events, daily site/surface
   snow observations, matched frost-tube dates, soil-profile isotherm bounds,
   and production output rows are stated separately.
3. **In what domain?** Northern Hemisphere phase stations, five mountain
   SNOTEL climates, five New England canopy/open surfaces, three frost-tube
   sites, and two soil-temperature sites are named in the methods.
4. **Which realization?** The report identifies the assessed development
   realization, activated production selectors, versioned science contract,
   and need for exact release-candidate reproduction before publication.
5. **Against what referents?** Observed phase labels, paired SWE/depth records,
   frost tubes, zero-degree-isotherm bounds, conservation identities, and the
   prior fixed-threshold comparator are distinguished.
6. **How was evidence analyzed?** Confusion-matrix reconstruction,
   explicit phase eligibility rules and dual confusion matrices, a site-
   resolved snow profile, residual-family decomposition, site-separated frozen-
   soil comparisons, paired snow controls, and selected-row storage
   reconstruction are described with a reproducible procedure.
7. **What did it show?** The key findings and abstract lead with the bounded
   phase, snow-profile, frost/confounding, and conservation results. Three
   tables and two figures make the principal operands inspectable. The seven
   tables include site-resolved snow, frost-tube, and isotherm results plus the
   four selected conservation rows.
8. **What uncertainty matters?** Observation reuse in development, forcing
   error, spatial support, snow-boundary confounding, referent differences,
   model form, and release transfer are discussed in domain language.
9. **What cannot be inferred?** The report rejects untouched held-out
   validation, isolated frost-physics attribution, a universal snow accuracy
   grade, and transfer to untested sites.
10. **Who decides application fitness?** The conclusion leaves watershed- and
    decision-specific adequacy to the practitioner or institution using the
    evidence.

The title and first scientific section do not use `CANDIDATE`, `PASS`,
`SUPPORTED`, or `INSUFFICIENT_EVIDENCE` as a headline. `DRAFT` appears only as
visible lifecycle metadata and does not substitute for a scientific finding.
The staged package exposes 16 public-safe research objects, including the exact
phase scorer, selected-row conservation log, and a self-contained dataset-
provenance/reacquisition record.
