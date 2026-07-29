# CANOPY-CAL-07E Literature Authority Review

Status:
`complete / method audit authorized / science correction not authorized`

Evidence mode: `Static + externally retrieved literature`

Intent: `science-authority review; no implementation or calibration`

## Objective

Review primary literature capable of interpreting the CAL-07D Bezà Mahafaly
seasonal-transition contradiction. Determine which environmental cues and
observation-method limitations are supported at the site, in southwestern
Madagascar analogues, in separately bounded western-Madagascar evidence, and
in transferable tropical dry-forest studies.

## Questions

1. What field evidence exists for rainfall, soil or groundwater availability,
   temperature, photoperiod, atmospheric demand, and stored plant water as
   controls on leaf flush and leaf fall at Bezà Mahafaly?
2. How do gallery and xerophytic forest phenology differ at the site?
3. What exactly do PhenoCam 10%, 25%, and 50% GCC transition dates represent,
   and what uncertainty or disagreement with in-situ leaf phenology is known?
4. Does admitted literature authorize a production correction, a new
   observation operator, a parameter/ecotype calibration package, or only
   additional discriminating measurements?

## Evidence hierarchy

Sources are classified before interpretation:

1. `DIRECT_SITE_PRIMARY`: peer-reviewed or primary field studies at Bezà
   Mahafaly with explicit phenology or water/climate observations.
2. `DIRECT_SITE_CONTEXT`: primary site descriptions or ecological monitoring
   records that constrain habitat, climate, or sampling but do not isolate a
   phenology mechanism.
3. `REGIONAL_PRIMARY`: primary studies in Madagascar dry forest or spiny
   thicket, with southwestern/southern studies distinguished from more distant
   western-Madagascar evidence.
4. `METHOD_PRIMARY`: primary PhenoCam processing, transition, and validation
   studies.
5. `MECHANISM_ANALOGUE`: primary tropical dry-forest mechanism studies outside
   Madagascar.
6. `DISCOVERY_ONLY`: catalogs, abstracts, conference material, reviews, or
   inaccessible citations used to locate evidence but not to authorize a
   scientific claim.

Direct-site authority outranks regional and biome analogues. Correlation does
not establish an exclusive cue. Evidence from other continents cannot
authorize Bezà parameters or new production equations.

## Included scope

- CAL-07D solution routes: forcing, observation semantics,
  parameter/ecotype transfer, and missing seasonal/water cues.
- Leafing/leaf fall; rainfall and rainfall frequency; temperature;
  photoperiod; VPD/atmospheric demand where observed; soil/groundwater and
  stored-water mechanisms.
- PhenoCam GCC processing, transition-date extraction, uncertainty, and
  comparison with in-situ phenology.
- Exact bibliographic/source register, evidence matrix, claim-level synthesis,
  acquisition-gap inventory, and one accessible evidence-map figure with a
  Markdown sidecar.

## Excluded scope

- Production Rust, contracts, parameter values, forcing corrections, refits,
  or new process equations.
- Treating abstracts or search snippets as full-text mechanism authority.
- Treating GCC as LAI, biomass, canopy cover, or a GSI state.
- Meta-analysis or effect-size pooling across incompatible observations.
- Reclassifying CAL-07D diagnostic observations as calibration or independent
  validation data.

## Write set

- this package;
- `docs/planning/canopy-phenology-assurance-roadmap.md`; and
- `docs/work-packages/README.md`.

All prior packages, production code, science contracts, and ADRs are read-only.

## Source admission

Every included source must record citation, stable locator, access date,
document type, evidence tier, study location, measured variables, temporal and
spatial support, full-text status, and claim ceiling. Full text is required for
claim-bearing evidence unless the package explicitly limits the item to
discovery or acquisition-needed status.

Machine-readable findings must distinguish:

- `SUPPORTED_AT_SITE`;
- `SUPPORTED_AS_CONTRIBUTOR`;
- `PLAUSIBLE_FROM_ANALOGUE`;
- `NOT_SUPPORTED`;
- `UNRESOLVED`;
- `ACQUISITION_NEEDED`; and
- `NOT_APPLICABLE`.

## Execution phases

1. Freeze CAL-07D questions, search strings, inclusion rules, and write set.
2. Search site-specific, regional, method, and mechanism literature.
3. Retrieve and appraise full text where accessible; register inaccessible
   sources without inferring their findings.
4. Build claim-level evidence and source-authority matrices.
5. Synthesize implications for each CAL-07D solution route and identify the
   minimum next evidence.
6. Produce the evidence-map figure and Markdown sidecar.
7. Validate citations, locators, source classifications, claims, Markdown,
   figure rendering, and exact diff.
8. Complete dual terminal review and verification, disposition findings,
   update roadmap/catalog, and close or hold truthfully.

## Exit criteria

- Search protocol and stop rule are explicit.
- Every claim-bearing source has accessible full text and a bounded evidence
  tier.
- Direct-site findings are separated from analogues.
- All four CAL-07D solution routes receive an evidence status and next-step
  disposition.
- Missing or inaccessible high-value sources are listed with an exact request
  for human acquisition.
- No literature result is converted directly into production authority.
- Figure, sidecar, source register, evidence matrix, and synthesis validate.
- Both terminal reviewers/verifiers pass or the package remains on hold.
- Roadmap/catalog and exact-diff reconciliation pass.

## Delegated review authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to two terminal science-review and verification subagents
for independent source-classification, claim-calibration, citation, and
closure review. Expected outputs are `artifacts/review-agent-a.md`,
`artifacts/verification-agent-a.md`, `artifacts/review-agent-b.md`, and
`artifacts/verification-agent-b.md`. Write access is limited to those four
package artifacts.

No heavy comparator or workspace test subagent is required.
