# Canopy Phenology Assurance Roadmap

Status: **prospective scientific campaign roadmap** (2026-07-26).

Evidence mode: **Static synthesis** of the completed `CANOPY-PHENOLOGY-01` GSI
kernel, completed `CANOPY-PHENOLOGY-02` native integration, the installed
canopy-gradient fixtures, the commissioned William J. Elliot hardwood and mixed
forest study, and the active scientific-assurance lifecycle. This roadmap is
planning guidance, not equation or parameter authority. Canonical process
authority remains in `SC-PLANT-001`, `SC-RESIDUE-001`,
`SC-INFILE-MANAGEMENT-YAML-001`, and downstream consumer contracts until an
authorized work package amends them.

## 0. Outcome

The canopy-phenology program culminates in `CANOPY-ASSURE-01`, a reviewed,
reproducible scientific assurance report that explains and demonstrates:

1. the complete forcing-to-consumer phenology chain;
2. deciduous, mixed, and evergreen canopy dynamics;
3. foliar allocation, litter transfer, residue persistence, and mass closure;
4. agreement and disagreement with field-defined biomass, LAI, canopy,
   litterfall, and forest-floor targets;
5. congruence across the installed canopy-gradient fixtures and their
   canopy-stratified snow observations;
6. Southern Hemisphere calendar, forcing, and observational robustness; and
7. consequences for interception, evapotranspiration, snow, frost, runoff, and
   erosion without using downstream compensation to hide an upstream process
   error.

`CANOPY-PHENOLOGY-02` is mechanically complete. The remaining work is an
empirical calibration, adjudication, and assurance campaign. A passing unit
test, exact mass ledger, synthetic hemisphere transform, or visually plausible
canopy curve is necessary evidence but is not sufficient for the final report.

## 1. Completed Foundation

### 1.1 Generalized phenology kernel

`20260717-canopy-phenology-gsi-kernel-001` implemented and verified the
Jolly-Nemani-Running Growing Season Index (GSI): minimum-temperature,
vapor-pressure-deficit, and signed-latitude photoperiod indicators, their
instantaneous product, and the authority-defined 21-day trailing mean.

The kernel provides a continuous, hemisphere-aware foliar activity signal. It
does not by itself define forest biomass, canopy, LAI, or litter.

### 1.2 Native canopy and litter realization

`20260719-canopy-phenology-native-integration-001` completed
`CANOPY-PHENOLOGY-02`. It:

- separates persistent structural biomass, evergreen foliage, and deciduous
  foliage;
- realizes daily foliar biomass and LAI from GSI;
- applies a persistent structural canopy floor;
- publishes positive foliar change as leaf-on allocation and negative foliar
  change as leaf-off litter;
- proves exact daily mass closure and repeated-cycle no-drift;
- routes same-day litter through decomposition, residue depth, and frost; and
- proves snow, ET, WB15 interception, erosion, residue, and frost consume the
  post-phenology state.

The package explicitly excluded site calibration, fitted thresholds,
independent Southern Hemisphere observations, empirical snow-fidelity
promotion, and public-output changes. Those exclusions define this roadmap's
starting boundary.

### 1.3 Ground-side residue coupling

`20260629-frost-residue-cover-implementation-001` connected recurring litter to
dynamic surface-residue mass, residue depth, and the frost thermal path.
`CANOPY-PHENOLOGY-02` subsequently replaced the native forest's fixed
`jdharv` litter window with same-day GSI-derived leaf-off transfer.

The current native production state has one aggregate surface-residue pool.
Bill Elliot's current/previous/old pools are therefore comparison diagnostics,
not native production states.

## 2. Scientific Questions

The campaign must answer the following questions before a canopy assurance
claim can be promoted.

### 2.1 Phenology timing and shape

- Do the GSI indicators reproduce observed green-up, peak canopy, senescence,
  and dormancy timing across climates?
- Does the 21-day signal suppress unrealistic weather-driven shedding and
  refoliation, or does it generate excessive gross allocation/litter turnover?
- Are one parameter set and its declared forest-class interpretation
  transferable, or are ecologically justified parameter strata required?

### 2.2 Canopy and biomass realization

- Are full-leaf foliar biomass, persistent structural biomass, maximum LAI,
  structural canopy cover, evergreen fraction, and `bb` independently
  identifiable from available observations?
- Does `Cc=max(Cs,1-exp(-bb*Bf))` reproduce both summer canopy closure and the
  deciduous winter branch/stem floor?
- Does a mixed forest retain the correct evergreen foliar and canopy floor
  without treating a single NLCD class percentage as a measured foliar
  fraction?

### 2.3 Litter sources and forest-floor storage

- Can declining deciduous foliar mass alone reproduce observed annual
  litterfall and equilibrium forest-floor storage?
- Does mixed or evergreen forest require explicit recurring needle turnover?
- Does total litterfall require a separately authorized fine-woody input from
  twigs, branches, bark, or background mortality?
- Can decomposition be calibrated without using an unrealistically large
  foliar pool or an unrealistically slow decay rate to compensate for a missing
  litter source?

### 2.4 Consumer consequences

- Do interception and ET respond coherently to observed seasonal LAI?
- Does leaf-off reduce canopy attenuation while autumn litter increases ground
  insulation in the expected opposing directions?
- Are snow accumulation, melt, residue-depth, frost-onset, and thaw responses
  congruent across paired canopy strata?
- Are runoff and erosion differences attributable to canopy and residue rather
  than soil-format, lateral-flow, baseflow, or channel differences?

### 2.5 Hemisphere robustness

- Does signed latitude produce the correct seasonal ordering without a fixed
  Julian-date branch?
- Does the existing Northern-to-Southern half-year phase transform continue to
  pass for the integrated state and all consumers?
- Do independent Southern Hemisphere observations support leaf-on, leaf-off,
  LAI/canopy amplitude, and mixed/evergreen floor claims?

## 3. Guardrails

- **Contract-first for new physics.** Empirical analysis may challenge the
  current realization, but no needle-turnover, woody-litter, smoothing,
  threshold, canopy, or decomposition law enters production before canonical
  contract amendment and an authorized implementation package.
- **Reproduce before translating.** Reconstruct Bill Elliot's analysis in its
  original perennial WEPP representation before mapping its targets into native
  CP2 operands.
- **Parameters are not observations.** `dropfc`, `oratea`, `bb`, and native YAML
  values remain fitted or interpreted operands. They cannot be presented as
  field measurements.
- **Separate source flux from decay.** Annual foliar, needle, and fine-woody
  inputs are adjudicated before decomposition is tuned to forest-floor stock.
- **Separate calibration from validation.** A site or year used to fit a
  parameter cannot carry the independent performance claim for that parameter.
- **Separate canopy from snow physics.** Canopy-stratum ordering may test
  attenuation and insulation. A canopy parameter must not be tuned to repair a
  canopy-independent snow accumulation or melt residual.
- **Separate hillslope from watershed evidence.** Hillslope surface runoff is
  not directly compared with watershed discharge containing lateral flow,
  baseflow, roads, or channels.
- **Separate soil formats.** The 7777-versus-2006.5 effect identified by Bill is
  crossed explicitly or held fixed; it cannot be attributed to management.
- **No synthetic-only Southern Hemisphere claim.** Phase symmetry proves
  implementation invariance. It does not replace independent observations.
- **No downstream compensation.** Hydrology and erosion are validation
  endpoints, not fitting targets for phenology, biomass, litter source, or
  decomposition.
- **Retain contrary evidence.** Failed sites, seasons, parameterizations, and
  mechanisms remain visible in the report and retained evidence.

## 4. Campaign Data And Evidence

### 4.1 Commissioned Elliot references

The WEPPcloud repository preserves Bill Elliot's April 2026 report and exported
Hubbard Brook hardwood and Santee mixed-forest management files under:

```text
wepppy/docs/work-packages/
  20260626_deciduous_mixed_forest_managements/references/
```

The source ledger records redistribution permission, exact hashes, normalized
filenames, and the unresolved Hubbard Brook discrepancy: the report describes
`92%` biomass retention while the delivered management contains
`dropfc=0.95`.

Campaign intake must:

1. bind the exact source hashes into openWEPP campaign evidence;
2. preserve both the `0.92` and `0.95` Hubbard branches;
3. obtain Bill's original `.run`, climate, soil, slope, constant-cover
   management, and machine outputs if available; and
4. label any chart-digitized values with uncertainty derived from figure
   resolution.

The report's unsourced or AI-attributed hydrology and sediment values are
context only and cannot carry validation.

### 4.2 Canopy-gradient fixtures

`tests/fixtures/cancov_forest/` supplies long-climate forest hillslopes across
conifer, mixed, deciduous, and open/pasture strata. The strongest within-site
comparisons are:

- Marcell conifer, deciduous, mixed, and open;
- Harvard deciduous, mixed, and open; and
- Hubbard Brook deciduous and mixed.

Normalized Harvard and Marcell observations currently emphasize snow depth,
SWE, and density. The fixtures use legacy `.man` inputs, so the campaign must
add or generate native YAML counterparts without silently changing climate,
soil, slope, or observation bindings.

### 4.3 Additional canopy and litter observations

An authorized data-intake package must identify and retain primary-source
observations for:

- leaf-on, peak, senescence, and leaf-off timing;
- seasonal LAI or an explicitly declared canopy-activity proxy;
- leaf-on and leaf-off canopy cover or gap fraction;
- full-leaf foliar biomass and persistent aboveground woody biomass;
- annual litterfall separated into leaves, needles, and woody material where
  available;
- forest-floor litter/duff mass and, where available, seasonal or age-class
  information; and
- at least one independent deciduous and one evergreen or mixed Southern
  Hemisphere evaluation lane, spanning at least two Southern Hemisphere
  climate regions and multiple observed seasons.

Remote-sensing or camera greenness may test timing and relative activity. It
does not automatically provide absolute LAI, foliar biomass, or canopy cover;
each mapping requires its own authority and uncertainty statement.

The campaign also needs a stable research-output surface for daily GSI
indicators, GSI21, foliar and structural pools, LAI, canopy, allocation, litter,
aggregate residue, residue depth, and consumed downstream values. Test-only
traces proved integration but cannot be the final assurance report's sole data
source. An output package may add a campaign-confined diagnostic artifact or a
contract-governed public surface; the choice must preserve units, chronology,
identities, and exact producer-to-consumer lineage.

## 5. Analytical Design

### 5.1 Bill Elliot reproduction

For Hubbard Brook and Santee, reproduce:

- live aboveground biomass trajectories;
- annual biomass-to-residue transfer;
- current, previous, old, and total ground residue;
- time to practical equilibrium;
- LAI and canopy behavior;
- constant-cover versus perennial annual runoff and sediment; and
- daily-runoff and peak-flow return-period tables.

Run the exact delivered management plus the report-described Hubbard
`dropfc=0.92` branch. Determine which, if either, reproduces the report figures.
Do not reconcile them by editing the preserved sources.

### 5.2 Native parameter translation

Map field quantities into native operands with an explicit lineage table:

| Native operand | Required interpretation |
| --- | --- |
| `Bf,max` | Full-leaf foliar biomass, not total aboveground live biomass. |
| `Bs` | Persistent woody biomass excluded from seasonal foliar transfer. |
| `fe` | Evergreen fraction of the summer foliar pool. |
| `xmxlai` | Observed or independently sourced full-leaf maximum LAI. |
| `Cs` | Effective persistent branch/stem canopy-cover floor. |
| `bb` | Canopy closure coefficient constrained by paired foliar mass and cover. |
| GSI thresholds | Climate-response operands evaluated against observed timing. |
| `oratea` / `orater` | Declared decomposition rates evaluated after source flux closure. |

Bill's total biomass target constrains `Bs+Bf`; it does not authorize setting
`Bf,max` to 15 or 19 kg m^-2. His added twig/branch transfer is not foliar
biomass and must remain a separate analytical term unless new production
authority is ratified.

### 5.3 Controlled crossover

Run these management arms under identical climate, soil, slope, initial state,
and period:

1. constant-cover WEPP baseline;
2. current WEPPcloud fixed-date deciduous or mixed management;
3. Bill Elliot perennial management;
4. native CP2 with field-derived foliar/structural partitioning; and
5. native CP2 plus analysis-only needle and fine-woody litter shadow fluxes.

Cross the selected arms with 7777 and 2006.5 soil representations where a
scientifically like-for-like conversion is available. Keep watershed/channel
comparisons outside the hillslope-management verdict.

### 5.4 Residue cohort shadow ledger

Build an analysis-only current/previous/old cohort ledger driven by the exact
daily native litter input and decay factor. The ledger must sum independently
to the aggregate production surface-residue pool when all cohorts share the
same linear decay law. Any divergence is either an implementation defect or an
explicit difference in age-dependent physics.

This diagnostic reproduces Bill's analytical view without adding unratified
production pools.

### 5.5 Gross-turnover diagnostic

For each year, publish:

```text
seasonal_amplitude =
    Bf,max * (1 - fe) * (max(GSI21) - min(GSI21))

gross_leaf_off =
    Bf,max * (1 - fe) * sum(max(GSI21[d-1] - GSI21[d], 0))

phenology_churn_ratio = gross_leaf_off / seasonal_amplitude
```

Publish the corresponding gross leaf-on allocation and their difference. A
ratio materially above one indicates repeated decline/recovery turnover and
must be explained against observations before litterfall is considered
credible.

## 6. Ordered Work Packages

Each row is prospective. It requires its own declared scope, evidence identity,
gate plan, and disposition before execution.

| Order | Prospective package | Outcome | Advancement gate |
| --- | --- | --- | --- |
| 1 | [`CANOPY-CAL-01` source and target ledger](../work-packages/20260726-canopy-cal-01-source-target-ledger-001/package.md) | Bind Bill's exact sources, primary literature targets, units, uncertainties, comparison scales, and the 92/95 discrepancy. | No unsourced or cross-scale target carries calibration or validation. |
| 2 | [`CANOPY-CAL-02` Elliot reproduction](../work-packages/20260726-canopy-cal-02-elliot-reproduction-001/package.md) | Reproduce or truthfully bound the Hubbard and Santee biomass, residue, equilibrium, and downstream comparisons. | Machine-readable runs and figures explain any mismatch with Bill's report. |
| 3 | `CANOPY-CAL-03` observation corpus, native fixtures, and research outputs | Install provenance-bound phenology/LAI/canopy/litter observations, native YAML counterparts for selected canopy-gradient lanes, and a stable daily research-output surface. | Calibration/holdout assignments and unchanged forcing/soil/slope bindings are explicit; assurance values do not depend solely on test-only traces. |
| 4 | `CANOPY-CAL-04` process calibration and identifiability | Evaluate GSI thresholds, `Bf,max`, `Bs`, `fe`, `xmxlai`, `Cs`, and `bb` in process order. | Timing, amplitude, biomass partition, and canopy closure pass without hydrologic fitting. |
| 5 | `CANOPY-CAL-05` litter-source and decomposition adjudication | Test leaf-only CP2, cohort reconstruction, needle turnover, fine-woody input, and decomposition equifinality. | Decide whether CP2 is sufficient or a contract amendment is required; no missing source is hidden in decay. |
| 6 | `CANOPY-CAL-06` canopy-gradient congruence | Run conifer-mixed-deciduous-open within-site comparisons through canopy, ET/interception, snow, residue, frost, runoff, and erosion consumers. | Prespecified ordering and quantitative cells pass or are bounded with retained contrary evidence. |
| 7 | `CANOPY-CAL-07` Southern Hemisphere robustness | Combine synthetic phase invariance with independent SH observational lanes and real downstream consumers. | Timing, phase, amplitude, mass closure, and consumer chronology pass without fixed dates or NH-only tuning. |
| 8 | `CANOPY-ASSURE-01` report and supplement | Author, reproduce, review, and publish the complete canopy-phenology assurance product. | Exact research objects, independent reproduction, accountable scientific and publication review, finding disposition, approval, and release transfer pass. |

If `CANOPY-CAL-05` identifies missing production physics, insert a bounded
contract-and-implementation package before Orders 6-8. The roadmap does not
pre-authorize that change or presume its outcome.

## 7. Acceptance Rubric

### 7.1 Process cells

- GSI indicator and GSI21 trajectories are finite, bounded, chronological, and
  reproducible.
- Leaf-on, peak, senescence, and leaf-off timing errors are reported by site,
  year, and canopy class.
- Summer/winter LAI and canopy amplitude are evaluated independently.
- Structural, evergreen, and deciduous biomass partitions reconcile to declared
  field quantities and units.
- Daily and annual foliar allocation/litter ledgers close.
- Phenology churn is reported and adjudicated.

### 7.2 Litter and residue cells

- Annual leaf, needle, woody, and total litter quantities remain distinct.
- Aggregate residue and the shadow cohort sum reconcile when their equations
  are equivalent.
- Equilibrium mass, seasonal range, time to equilibrium, and year-over-year
  drift are reported.
- Decomposition is summarized both by its raw operand and realized
  environment-weighted decay.
- Residue mass, residue depth, and the exact frost-consumed depth remain
  traceable on leaf-off days.

### 7.3 Canopy-gradient cells

- Within-site canopy strata preserve expected deciduous < mixed < evergreen
  winter canopy ordering where those classes exist.
- Summer canopy closure and winter structural/evergreen floors are evaluated
  quantitatively, not by ordering alone.
- Paired open/deciduous/conifer snow responses report accumulation, peak,
  melt-out, density, and timing without attributing canopy-independent residuals
  to phenology.
- Frost onset/thaw and residue insulation are evaluated with the same daily
  canopy/litter state.
- ET, interception, runoff, and erosion consequences are reported only after
  their upstream process cells pass.

### 7.4 Southern Hemisphere cells

- The complete cyclic phase-transform gate passes for GSI, canopy, allocation,
  litter, and all real consumers.
- Independent SH sites exercise actual signed latitude, chronology,
  multi-season climate, and at least two climate regions.
- At minimum, deciduous seasonal reversal and a persistent evergreen or mixed
  floor are observed and evaluated in separate independent lanes.
- Calibration on NH timing alone cannot carry the SH verdict.
- Any missing biome, latitude band, or observation type is a named report
  limitation rather than an implied global claim.

### 7.5 Verdict classes

Each material claim receives one of:

- `SUPPORTED`: process and independent evidence satisfy the prespecified cell;
- `BOUNDED`: evidence constrains behavior but is insufficient for a broad
  claim;
- `CONTRADICTED`: retained evidence materially disagrees with the claim; or
- `NOT_EVALUATED`: required authority, observation, or consumer evidence is
  absent.

No aggregate pass may erase a contradicted site, class, season, or material
source term.

## 8. CANOPY-ASSURE-01 Report Contract

The final assurance report is a scientific communication product for
hydrologists, forest managers, model developers, and WEPP users. It must include:

1. **Scope and claims** - evaluated domains, scales, versions, exclusions, and
   verdict vocabulary.
2. **Process formulation** - forcing, GSI indicators, trailing state,
   foliar/structural realization, LAI, canopy, allocation, litter,
   decomposition, residue depth, and consumer chronology.
3. **Parameter authority** - units, ecological meaning, defaults, fitted
   values, priors or bounds, correlations, and identifiability.
4. **Bill Elliot reproduction** - source integrity, 92/95 resolution, biomass
   and residue trajectories, equilibrium comparison, and limitations.
5. **Mass and state assurance** - daily closure, annual transfer, no-drift,
   cohort reconstruction, and churn.
6. **Canopy-gradient evaluation** - site/stratum design, canopy and LAI results,
   snow/frost congruence, and open-versus-forest controls.
7. **Southern Hemisphere evaluation** - synthetic phase proof plus independent
   observational results.
8. **Downstream consequences** - interception, ET, snow, frost, runoff, and
   erosion with causal boundaries and competing explanations.
9. **Negative and sensitivity evidence** - failed parameterizations,
   equifinality, soil-format effects, forcing limits, and alternative litter
   hypotheses.
10. **Limitations and application guidance** - supported forest classes,
    climates, scales, missing processes, and judgments retained by users.
11. **Reproduction supplement** - exact inputs, manifests, code/software
    identity, commands, tidy result objects, tables, figures, and finding
    disposition.

Minimum figures:

- forcing indicators, instantaneous GSI, and GSI21 through representative
  leaf-on and leaf-off seasons;
- deciduous, mixed, and evergreen foliar/structural biomass, LAI, and canopy;
- daily allocation/litter and annual mass closure;
- Bill-style live biomass plus current/previous/old/total residue trajectories;
- observed-versus-modeled phenology timing and amplitude by site;
- within-site canopy-gradient snow and frost comparisons; and
- NH/SH seasonal-phase and independent SH observational comparisons.

Minimum tables:

- operand and observation lineage;
- calibration/holdout assignment;
- Bill reproduction scorecard;
- process, litter, gradient, and hemisphere acceptance cells;
- sensitivity/identifiability summary; and
- supported, bounded, contradicted, and not-evaluated claims.

## 9. Publication And Relationship To Other Assurance Work

`CANOPY-ASSURE-01` uses the scientific-assurance v2 lifecycle. Generated
figures, a valid schema, or internal review do not authorize publication.
Advancement requires:

- an exact report root and complete transitive evidence graph;
- independent reproduction of every material result;
- accountable human scientific review;
- independent reproduction/publication review;
- complete finding disposition;
- assurance-steward and release-owner approval; and
- deterministic release transfer and public build.

The canopy report is a distinct plant/coupled-process assurance product. Its
reviewed evidence may support the canopy-dependent portions of `ASSURE-06`
snow/frost synthesis, but neither report inherits the other's conclusions.
Snow/frost residuals that are demonstrably canopy-independent remain owned by
the snow/frost campaign.

## 10. Immediate Next Action

Scaffold `CANOPY-CAL-01` as a documentation-and-evidence intake package. Its
first deliverable is the exact target ledger and reproduction manifest, not a
parameter change. It should bind the commissioned Elliot sources, enumerate
missing run artifacts, audit every field target back to a primary source, and
predeclare which targets are observations, Bill-derived assumptions,
management operands, or model outputs.
