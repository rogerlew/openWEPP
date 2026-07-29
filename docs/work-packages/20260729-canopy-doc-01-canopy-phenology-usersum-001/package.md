# CANOPY-DOC-01 Native-Forest Canopy-Phenology Narrative

Status: `queued / scaffolded`

Evidence mode: `not run`

Intent: `documentation and scientific communication; no calibration,
implementation, or assurance publication`

## Objective

Author `usersum/openwepp-canopy-phenology.md` as the durable, broad
model-science explanation of openWEPP native forest canopy phenology. Explain
why a first-class native forest land use is needed, the forest dynamics the
model represents, how those dynamics interact with hydrology and sediment,
how the implementation works at an abstract level, and how users should
understand and calibrate its coefficients.

The document must hand detailed methods, quantitative evaluation, uncertainty,
and reproduction material to the later `CANOPY-ASSURE-01` report rather than
duplicating that report.

## Rationale

Legacy WEPP and forest-adapted inputs carry forest, shrub, and grass behavior
through cropland management records because the historical native branches
were not completed for production. That compatibility representation cannot
directly express a mixed forest's persistent structure, evergreen foliage,
climate-driven deciduous foliage, gradual leaf-on and leaf-off, recurring
litter transfer, or the resulting daily canopy and forest-floor state.

The canopy campaign has implemented and evaluated a native forest process
chain, but the repository does not yet contain a reader-facing explanation of
why that chain exists or how to configure it. Users also need coefficient
guidance that separates hard input domains from evidence-supported ranges,
worked examples, calibrated ensembles, and quantities whose typical ranges
are not established.

## Included scope

- A scientific narrative for hydrologists, forest managers, erosion modelers,
  and scientific reviewers.
- The historical limitation of cropland-encoded forest inputs and the purpose
  of the `native_forest` land-use type.
- A referenced science primer covering leaf-on, leaf-off, evergreen and
  deciduous fractions, persistent structure, LAI, canopy cover and height,
  litterfall, decomposition, residue cover, and residue depth.
- The abstract forcing-to-consumer model chain:
  temperature, VPD, latitude and photoperiod, GSI, foliar realization,
  allocation/litter transfer, residue, and downstream consumers.
- Causal interactions with interception, evapotranspiration, snow, frost,
  runoff routing, erosion cover, and sediment delivery.
- A complete user-facing guide to the coefficients that directly control
  native forest phenology, canopy/height realization, leaf transfer, and
  immediate residue behavior.
- Qualitative synthesis of the current evidence envelope and application
  limits, with quantitative assurance results left to `CANOPY-ASSURE-01`.
- Published primary references, a version header, a revision log, and the
  `usersum/README.md` model-science catalog entry.
- Conceptual figures only when they materially improve the narrative. Any
  figure must be accessible, self-contained within `usersum`, and must not
  duplicate a claim-bearing assurance plot.

## Excluded scope

- New calibration, parameter fitting, candidate generation, data acquisition,
  literature-acquisition campaigns, or empirical validation.
- Production Rust, schemas, science contracts, equations, defaults, input
  formats, or runtime behavior.
- The `CANOPY-ASSURE-01` manuscript, supplement, research-object build,
  approval, release transfer, or public assurance-catalog promotion.
- Copying detailed work-package result tables, execution jargon, internal
  verdict machinery, or repository-only links into `usersum`.
- Calling a schema example, search domain, single-site fit, or accepted
  ensemble a universal default, physiological bound, or typical range.
- Recommending downstream compensation: canopy coefficients may not be tuned
  to hide snow, frost, runoff, erosion, litter-source, or decomposition
  residuals.
- Predictive evergreen needle or fine-woody litterfall laws. Their current
  authority gap must remain explicit.

## Deliverables

1. `usersum/openwepp-canopy-phenology.md`.
2. A `usersum/README.md` model-science catalog entry.
3. `artifacts/coefficient-authority-ledger.csv`, containing one row for every
   in-scope user-facing coefficient.
4. `artifacts/source-and-claim-map.md`, binding each narrative claim and
   reference to implementation, contract, literature, or retained evidence.
5. `artifacts/calibration-guidance-audit.md`, checking that calibration advice
   is observable-driven, sequential, identifiability-aware, and does not use
   downstream compensation.
6. Documentation, link, reference, spelling, style, exact-diff, review,
   verification, and disposition artifacts.

## Coefficient-guide contract

Before drafting the public coefficient section, inventory the exact active
native YAML and runtime fields. The inventory must include at least:

- the six temperature, VPD, and photoperiod GSI thresholds;
- summer foliar biomass, evergreen fraction, structural canopy-cover floor,
  and structural biomass;
- maximum LAI, canopy-cover coefficient, canopy-height coefficient, and maximum
  canopy height;
- above-ground and root decomposition controls where they affect the
  documented forest residue chain;
- residue mass-to-depth conversion when it is user-configurable; and
- authenticated external needle/fine-woody litter inputs, described as
  observed exogenous forcing rather than predictive coefficients.

The inventory may add directly coupled user fields after tracing the active
native consumer path. It must distinguish inherited WEPP growth/community or
routing parameters from the coefficients owned by canopy phenology.

For every coefficient, the public guide and authority ledger must record:

- exact field name and user-facing label;
- units and valid hard domain;
- ecological/process meaning and equation location;
- effect direction and the outputs it can influence;
- value status: required input, actual default, worked example, literature
  value, calibrated value, accepted ensemble, or not established;
- range class: `HARD_DOMAIN`, `SOURCE_RANGE`, `CALIBRATION_ENSEMBLE`,
  `TYPICAL_STARTING_RANGE`, `EXAMPLE_ONLY`, or `NOT_ESTABLISHED`;
- range authority, ecological/geographic scope, and scale;
- calibration target and minimum observation needs;
- identifiability, correlation, equifinality, and boundary warnings; and
- transfer and downstream-compensation cautions.

If no defensible typical range exists, say `not established`. Do not convert
the CAL-04B search grid, one Hubbard fit, schema examples, or the accepted
ensemble into a broader ecological range without separate authority.

Calibration guidance must establish an observation-driven order:

1. classify forest composition and persistent structure;
2. constrain full-leaf biomass, LAI, canopy cover, and height from matching
   observations and scales;
3. constrain seasonal timing from leaf-on/leaf-off observations while
   retaining threshold covariance and equifinality;
4. assess litter source and decomposition separately from canopy timing; and
5. reserve independent sites or years for transfer evaluation without refit.

The guidance must explain that the retained temperate parameter ensemble is
partially identifiable, Harvard transferability is unsupported, and the
tropical dry-forest contradiction does not authorize another timing
calibration round.

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/usersum-authoring-style-guide.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/decisions/0034-management-file-lanuse-input-authority.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-YAML-001.md`
- `crates/openwepp-management-schema/src/lib.rs`
- completed canopy work packages from `CANOPY-PHENOLOGY-01` through
  `CANOPY-CAL-07F`
- `usersum/README.md`
- `usersum/snow-frost-modeling-and-validation.md`

## Intended write set

- `usersum/openwepp-canopy-phenology.md`
- `usersum/README.md`
- optional conceptual figure assets under `usersum/` referenced only by the
  canopy narrative
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/README.md`
- this `CANOPY-DOC-01` package tree

Production code, tests, schemas, contracts, ADRs, completed predecessor
evidence, `usersum/assurance/**`, assurance sources, generated exports, and
release tooling are read-only.

## Execution phases

1. Declare documentation intent and reconcile the exact write set.
2. Build the source/claim map and coefficient-authority ledger from canonical
   contracts, active schema/runtime fields, primary literature, and retained
   canopy evidence.
3. Freeze the narrative argument, audience, section plan, coefficient range
   classes, and assurance handoff before prose drafting.
4. Author the self-contained `usersum` narrative and catalog entry.
5. Audit every coefficient, range, calibration recommendation, reference,
   equation, limitation, link, and capability/default statement.
6. Run affected documentation, spelling-preview, link/path, and diff checks.
7. Complete two independent scientific/editorial reviews, disposition every
   finding, and correct accepted findings.
8. Complete two independent verifications, reconcile the terminal diff, update
   roadmap/catalog status, archive the kickoff prompt, and close truthfully.

## Validation plan

This is an editorial documentation increment with no executable or production
impact. Run and retain:

- `markdown-doc lint --path usersum`
- `markdown-doc lint --path docs/work-packages/20260729-canopy-doc-01-canopy-phenology-usersum-001`
- `markdown-doc lint --path docs/planning/canopy-phenology-assurance-roadmap.md`
- `markdown-doc lint --path docs/work-packages/README.md`
- `git diff --check`
- a repository-relative Markdown-link check over the changed files;
- a negative check that `usersum/openwepp-canopy-phenology.md` contains no
  links outside `usersum`;
- a spelling-normalization preview with `uk2us`, applying only reviewed safe
  prose changes; and
- package-local checks that every public coefficient row has units, domain,
  status, range class, authority, calibration guidance, identifiability, and
  transfer cautions.

Rust, workspace, comparator, empirical, and assurance-publication commands are
not applicable because no executable, schema, contract, protected evidence,
or public assurance surface is changed.

## Exit criteria

- The narrative satisfies the Order 8 contract and the usersum narrative style
  guide.
- The framing problem explains the native forest need without reducing the
  document to an implementation feature list.
- Leaf-on/off, mixed/evergreen/deciduous structure, litter/residue, and all
  named hydrologic and sediment interactions are scientifically sourced and
  explained at the intended audience level.
- Every in-scope coefficient is present in the public guide and authority
  ledger with units, hard domain, effect, value/range status, authority,
  calibration target, identifiability, and transfer warning.
- Unsupported typical ranges are labeled `NOT_ESTABLISHED`; no example, search
  domain, site fit, or ensemble is silently promoted.
- Calibration guidance is observation-driven and does not tune canopy
  parameters to compensate downstream residuals.
- The document is self-contained within `usersum`, uses published references,
  carries a matching version/revision log, and closes with interpretation.
- Quantitative assurance results and reproduction details are not duplicated.
- All current-scope validation requirements have direct evidence.
- Two independent reviews and two independent verifications pass; every
  finding is dispositioned and every accepted finding is corrected and
  verified.
- Exact terminal diff matches the declared write set, kickoff is archived,
  roadmap/catalog are current, and final disposition is truthful.

## Security and production impact

No secrets, protected data, dependencies, runtime behavior, schemas,
serialization, defaults, or security boundaries change. The tracked public
assurance catalog and generated assurance exports remain unchanged.

## Delegated review authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to two independent terminal scientific/editorial
review-and-verification subagents. Their scope is the narrative argument,
coefficient authority and calibration guidance, citations, usersum style,
exact write set, validation evidence, and closure legitimacy. Expected outputs
are
`artifacts/review-agent-a.md`, `artifacts/review-agent-b.md`,
`artifacts/verification-agent-a.md`, and
`artifacts/verification-agent-b.md`. Write access is limited to those four
package artifacts.

No heavy batch, comparator, or full-workspace subagent is required.
