# Canopy Phenology Assurance Roadmap

Status: **active scientific campaign roadmap** (2026-07-28).

Evidence mode: **Ran evidence plus static synthesis** of the completed
`CANOPY-PHENOLOGY-01` GSI kernel, completed `CANOPY-PHENOLOGY-02` native
integration, completed `CANOPY-CAL-01` source ledger, completed
`CANOPY-CAL-02` Elliot reproduction, installed canopy-gradient fixtures, the
commissioned William J. Elliot hardwood and mixed-forest study, and the active
scientific-assurance lifecycle. This roadmap is planning guidance, not equation
or parameter authority. Canonical process authority remains in `SC-PLANT-001`,
`SC-RESIDUE-001`,
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

`CANOPY-PHENOLOGY-02` is mechanically complete, and `CANOPY-CAL-01..02`
completed the Elliot source/reproduction phase. The remaining work is an
empirical calibration, adjudication, and assurance campaign. A passing unit
test, exact mass ledger, legacy match, synthetic hemisphere transform, or
visually plausible canopy curve is necessary evidence but is not sufficient
for the final report.

The campaign now explicitly preserves **Elliot's staged methodology rather
than his numerical outputs**. His useful method is to establish vegetation
stocks and seasonality, quantify gross annual live-to-residue transfer, run
long enough to characterize forest-floor equilibrium, and only then inspect
hydrology and erosion. His reported biomass, residue, runoff, sediment, and
return-period values are not native CP2 calibration or validation targets
unless separately supported by independent observational authority.

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

### 1.4 Elliot source ledger and reproduction

`20260726-canopy-cal-01-source-target-ledger-001` bound the commissioned
report, delivered managements, exact source identities, target classes,
uncertainties, and the Hubbard `dropfc=0.92` report versus `0.95` delivered-file
discrepancy.

`20260726-canopy-cal-02-elliot-reproduction-001` completed two five-arm,
100-year legacy lanes:

- Windows WEPP 2012.800 with WEPPpy SSURGO `2006.2` soils built from exact
  site mukeys; and
- Linux WEPP 260725 with source-native 9002 soils, hourly water balance, and
  fixed-callsite Observe enabled.

Both lanes support `NOT_REPRODUCIBLE`. In the Linux 9002 lane, 9 of 64
predeclared target comparisons pass bounded tolerances and 55 are
contradicted. The delivered Hubbard `dropfc=0.95` branch reproduces the
approximately `19 kg/m2` live-biomass trajectory, but residue and most Santee,
hydrology, sediment, and recurrence targets remain contradicted. The
report-described `dropfc=0.92` branch does not reproduce that live stock.

The Linux release has no litter-specific Observe callsite. Pinned producer
source nevertheless establishes the perennial gross-transfer formula, and the
rounded daily crop output reconstructs gross aboveground
live-to-current-residue transfer within its `0.001 kg/m2` publication
precision. The operator accepted that bound for campaign use. It is legacy
mechanism characterization, not a direct flux observation, exact internal
operand, foliage-only litterfall estimate, or independent field target.

The retained legacy reference values are approximately:

| Lane / arm | 100-year gross transfer | Years 91--100 | Unit |
| --- | ---: | ---: | --- |
| Linux 260725 / Hubbard hardwood 0.95 | 0.82563 | 0.99403 | kg/m2/year |
| Linux 260725 / Hubbard hardwood 0.92 | 0.89957 | 1.00124 | kg/m2/year |
| Linux 260725 / Santee mixed 0.93 | 1.42949 | 1.63683 | kg/m2/year |

These values may define legacy comparison and sensitivity envelopes. They
must not be fitted as observations. The complete verdict and handoff live in
`../work-packages/20260726-canopy-cal-02-elliot-reproduction-001/artifacts/`.

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
- **Method, not legacy numbers.** `CANOPY-CAL-02` completed the required legacy
  reconstruction and found it not reproducible. Preserve Elliot's staged
  stock-transfer-equilibrium-downstream method, but do not fit native CP2 to
  his model-derived values.
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
- **Separate soil formats.** Bill's delivered managements were intended for
  WEPPcloud 7778/9000-series soils. CAL-02 explicitly separated source-native
  9002 from WEPPpy SSURGO 2006.2. Future comparisons must cross a
  scientifically like-for-like representation explicitly or hold soil fixed;
  no soil-format effect may be attributed to management.
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

The completed source ledger records redistribution permission, exact hashes,
normalized filenames, and the Hubbard Brook discrepancy: the report describes
`92%` biomass retention while the delivered management contains
`dropfc=0.95`.

The completed CAL-01/CAL-02 evidence:

1. binds the exact source hashes into openWEPP campaign evidence;
2. preserves both the `0.92` and `0.95` Hubbard branches;
3. records the missing Windows soil/project and machine-output boundary;
4. retains separate Windows/WEPPpy-2006.2 and Linux/source-native-9002 lanes;
   and
5. labels chart-derived targets and their uncertainty.

The report's unsourced or AI-attributed hydrology and sediment values are
context only and cannot carry calibration or validation. Further recovery of
Bill's missing byte-identical Windows project is not a campaign prerequisite.

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

### 5.1 Completed Elliot reproduction and retained method

CAL-02 reproduced or bounded, for Hubbard Brook and Santee:

- live aboveground biomass trajectories;
- annual biomass-to-residue transfer;
- current, previous, old, and total ground residue;
- time to practical equilibrium;
- LAI and canopy behavior;
- constant-cover versus perennial annual runoff and sediment; and
- daily-runoff and peak-flow return-period tables.

It ran the exact delivered management plus the report-described Hubbard
`dropfc=0.92` branch without editing preserved sources. The resulting
`NOT_REPRODUCIBLE` verdict closes legacy recovery as a campaign objective.

The following analytical sequence remains binding for native work:

1. establish phenology timing and seasonal vegetation stocks;
2. partition total aboveground biomass into persistent structure and foliage;
3. quantify annual gross source transfer separately by leaf, needle, and woody
   material where evidence permits;
4. characterize residue equilibrium, seasonal range, time to equilibrium, and
   drift before tuning decomposition;
5. freeze accepted upstream parameters; and
6. evaluate interception, ET, snow, frost, runoff, and erosion as downstream
   consequences rather than fitting targets.

Every native result must distinguish an independent observation, a fitted
operand, a derived diagnostic, a legacy comparison, and a model output.

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

Cross selected arms with source-native 9002 and WEPPpy SSURGO 2006.2 only
where a scientifically like-for-like comparison is available and useful.
Otherwise hold soil identity fixed. Keep watershed/channel comparisons outside
the hillslope-management verdict.

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

Orders 1 through 6 are complete; Order 7 executed and remains on scientific
hold; CAL-07B completed the hourly VPD aggregation diagnostic; CAL-07C lifted
the Alerce forcing blocker only for bounded research execution and retained
non-forcing contradictions; CAL-07D attributed the Beza transition
contradiction without selecting a correction; CAL-07E completed the
literature-authority review and exposed a transition-product inconsistency;
CAL-07F executed the product-consistent audit, rejected another calibration
round, and adjudicated the remaining Bezà contradiction as an ecosystem-model
limitation; Order 8 remains prospective.
Every prospective row requires its own declared scope, evidence identity, gate
plan, and disposition before execution.

| Order | Prospective package | Outcome | Advancement gate |
| --- | --- | --- | --- |
| 1 | [`CANOPY-CAL-01` source and target ledger](../work-packages/20260726-canopy-cal-01-source-target-ledger-001/package.md) — **complete** | Bind Bill's exact sources, primary literature targets, units, uncertainties, comparison scales, and the 92/95 discrepancy. | Passed: no unsourced or cross-scale target carries calibration or validation. |
| 2 | [`CANOPY-CAL-02` Elliot reproduction](../work-packages/20260726-canopy-cal-02-elliot-reproduction-001/package.md) — **complete / not reproducible** | Reproduce or truthfully bound the Hubbard and Santee biomass, residue, equilibrium, and downstream comparisons. | Passed: machine-readable runs and figures explain the mismatches; legacy recovery is closed. |
| 3 | [`CANOPY-CAL-03` observation corpus, native fixtures, and research outputs](../work-packages/20260726-canopy-cal-03-observation-native-research-001/package.md) — **complete; CAL-04/05 authority blockers retained** | Installed provenance-bound phenology/LAI/canopy/litter observations, native YAML counterparts for selected canopy-gradient lanes, and a stable daily research-output surface. | Passed for CAL-03: immutable roles, protected bindings, real production consumers, mass/cohort closure, and deterministic rebuild. CAL-04 timing fitting and affected CAL-05 source/decay claims remain authority-blocked pending new independent evidence. |
| 4 | [`CANOPY-CAL-04`](../work-packages/20260726-canopy-cal-04-process-calibration-identifiability-001/package.md) — **closed / historical hold retained**; [`CAL-04A`](../work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001/package.md) — **complete / design authority admitted**; [`CAL-04B`](../work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/package.md) — **complete / readiness pass / validation limitation retained** | CAL-04B evaluated all 9,261 frozen Hubbard candidates, independently reconstructed a 37-member accepted ensemble, propagated later-stage uncertainty, passed the freeze and two-verifier barrier, and scored Harvard once without refit. GSI timing and foliar/structural partition are empirically calibrated but partially identifiable; the remaining stages are calibration-ready and data-limited. Harvard aggregate errors span 43.48–72.46 days; 34/37 candidates have zero interval coverage and the maximum positive coverage is 18.81%, so independent transferability is unsupported. | Passed: execution, custody, terminal validation, dual scientific review, and dual verification. Retain the poor Harvard result as contrary evidence; do not refit or claim transferability. |
| 5 | [`CANOPY-CAL-05` litter-source and decomposition readiness](../work-packages/20260728-canopy-cal-05-litter-source-decomposition-readiness-001/package.md) — **complete / readiness pass**; [`CANOPY-LITTER-SOURCE-AUTHORITY-01`](../work-packages/20260728-canopy-litter-source-authority-001/package.md) — **complete / external boundary implemented / predictive authority hold retained** | Raw runtime, reconstruction, recovery, terminal ridge, failures, and Harvard diagnostics pass. The direct-runtime operator is calibration-ready-data-limited and partially identifiable. The successor implemented only authenticated exogenous daily forcing; native predictive source sufficiency remains authority-missing and empirical decomposition remains blocked. | Passed for the authenticated external boundary. Orders 6-8 may use it only with provenance-complete daily inputs; do not infer unavailable predictive source magnitudes or fit decomposition. |
| 6 | [`CANOPY-CAL-06` canopy-gradient congruence](../work-packages/20260728-canopy-cal-06-canopy-gradient-congruence-001/package.md) — **complete / bounded characterization / downstream advancement withheld** | All 261 prespecified runs passed: the complete 37-member timing ensemble across seven native forest lanes plus two open controls. Within-model winter canopy ordering passes 37/37 at Marcell, Harvard, and Hubbard Brook; snow residuals are quantified; Harvard SWE is excluded for a provider unit/identity contradiction; missing predictive litter sources and unavailable erosion output remain explicit. | Passed for bounded characterization: all prespecified cells are `BOUNDED` or `NOT_EVALUATED`, contrary evidence is retained, and unsupported downstream cells are visibly `NOT ADVANCED`. |
| 7 | [`CANOPY-CAL-07` Southern Hemisphere robustness](../work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/package.md) — **hold / forcing authority incompatible / no canopy result**; [`CAL-07B` hourly VPD aggregation diagnostic](../work-packages/20260728-canopy-cal-07b-hourly-vpd-aggregation-diagnostic-001/package.md) — **complete / diagnostic pass**; [`CAL-07C` hourly VPD forcing reconstruction](../work-packages/20260728-canopy-cal-07c-hourly-vpd-forcing-reconstruction-001/package.md) — **complete / bounded execution**; [`CAL-07D` transition chronology attribution](../work-packages/20260729-canopy-cal-07d-transition-chronology-attribution-001/package.md) — **complete / diagnostic attribution**; [`CAL-07E` literature authority review](../work-packages/20260729-canopy-cal-07e-literature-authority-review-001/package.md) — **complete / method audit authorized**; [`CAL-07F` observation product/operator audit](../work-packages/20260729-canopy-cal-07f-observation-product-operator-audit-001/package.md) — **complete / do not calibrate / ecosystem-model limitation adjudicated** | CAL-07F separates `gcc_mean` and `gcc_90`, retains their uncertainty, and reaches the same no-calibration result under both. No member has all 12 seasonal crossings; the best member hits 1/12 and 0/12 confidence intervals, with penalized errors of 59.12 and 65.87 days. | Not passed. Do not execute another Bezà timing calibration round. Defer acquisition and further canopy phenology work; report the tropical dry-forest chronology mismatch as an ecosystem-model limitation. Reopen only with field-corresponded observations, authoritative process science, or an independently testable alternative formulation. |
| 8 | `CANOPY-ASSURE-01` report and supplement | Author, reproduce, review, and publish the complete canopy-phenology assurance product. | Exact research objects, independent reproduction, accountable scientific and publication review, finding disposition, approval, and release transfer pass. |

### 6.1 Completed CANOPY-CAL-03 execution contract

At Order 3, CAL-03 was the evidence and observability package, not a calibration
package. Its retained contract required a fresh executor to complete it without
reopening CAL-02 or inferring requirements from conversation history.

CAL-03 must deliver all of the following:

1. **Observation corpus and target ledger.**
   - Retain primary-source data or exact resolvable research objects for
     phenology dates, seasonal LAI or declared proxy, leaf-on/leaf-off canopy
     cover or gap fraction, foliar and persistent woody biomass, annual litter
     by material class, and forest-floor mass.
   - Record site, coordinates, forest class, period, temporal support, spatial
     support, units, uncertainty, transformations, license, source identity,
     and missing-value semantics.
   - Classify every quantity as `OBSERVATION`, `FITTED_OPERAND`,
     `DERIVED_DIAGNOSTIC`, `LEGACY_COMPARISON`, or `MODEL_OUTPUT`.
   - Assign calibration and holdout roles before any parameter fitting.
     Holdout sites or years may not be selected after viewing fitted
     performance.
   - Carry Bill-derived values only as `LEGACY_COMPARISON`. The CAL-02 values
     and scorecard may test continuity with the legacy mechanism but may not
     carry native acceptance.

2. **Native paired fixtures.**
   - Create native YAML counterparts for the selected Marcell, Harvard, and
     Hubbard Brook canopy-gradient lanes.
   - Preserve climate, soil, slope, simulation period, initial-state, and
     observation bindings across each legacy/native pair. Every intentional
     difference must be machine-readable and reviewed.
   - Include deciduous, mixed, evergreen/conifer, and open controls wherever
     the source site supplies them. Do not manufacture a missing stratum.
   - Keep soil representations fixed within a management comparison. A
     deliberate soil-format crossover is a separately labeled sensitivity
     axis, never a canopy effect.

3. **Stable daily research-output surface.**
   - Publish date/year/day, site/arm identity, GSI temperature/VPD/photoperiod
     indicators, instantaneous GSI, GSI21, structural biomass, evergreen and
     deciduous foliar biomass, total foliar and aboveground live biomass, LAI,
     canopy cover, leaf-on allocation, leaf-off transfer, aggregate surface
     residue, decomposition loss, residue depth, and the values actually
     consumed by interception/ET, snow, frost, runoff, and erosion.
   - Preserve units, chronology, producer identity, null/overflow semantics,
     and exact producer-to-consumer lineage. Test-only traces are insufficient.
   - Prefer a campaign-confined diagnostic artifact unless a public output is
     independently justified. A public schema or new production physics
     requires its own contract authority and package.
   - Emit machine-readable manifests and deterministic tidy outputs suitable
     for independent rebuilding of every later table and figure.

4. **Bill-method diagnostics for native CP2.**
   - Compute annual gross leaf-on and leaf-off, net foliar change, seasonal
     amplitude, phenology churn ratio, residue equilibrium mass, seasonal
     range, time to practical equilibrium, and year-over-year drift.
   - Add an analysis-only current/previous/old cohort ledger driven from the
     exact native daily litter source and declared decay. Its sum must
     reconcile to the aggregate production residue whenever the assumed decay
     equations are equivalent.
   - Keep leaf, needle, fine-woody, and total litter distinct. CAL-03 may
     inventory missing source terms and support shadow diagnostics; it may not
     add unratified production turnover.

5. **Pre-calibration protocol.**
   - Freeze objective functions, uncertainty-aware tolerances, parameter
     bounds/priors, calibration/holdout partitions, missing-data rules,
     equilibrium rule, and failure classifications for CAL-04 and CAL-05.
   - Declare the process order: GSI timing; foliar/structural partition;
     evergreen fraction; LAI; canopy floor/closure; litter source; then
     decomposition. Downstream hydrology and erosion remain evaluation-only.
   - Specify identifiability outputs, including parameter correlations,
     profile or ensemble uncertainty, boundary hits, failed runs, and
     equifinality. A single best-fit parameter vector is insufficient.

CAL-03 advances only when independent review verifies the observation
authority and role assignments, paired fixtures preserve all protected
forcings, daily outputs exercise real production consumers, the native mass
and cohort ledgers close, deterministic rebuild passes, and no calibration or
new physics has been smuggled into the evidence package.

### 6.2 CAL-04 and CAL-05 execution discipline

CAL-04B must calibrate or demonstrate readiness in process order rather than
jointly fitting every
operand:

1. fit GSI thresholds to independent timing observations;
2. constrain `Bf,max` and `Bs` from foliar and persistent structural biomass,
   using total biomass only as a partition sum;
3. constrain `fe` from evergreen/mixed seasonal persistence;
4. constrain `xmxlai` from peak LAI;
5. constrain `Cs` and `bb` from winter floor and summer canopy closure; and
6. freeze accepted upstream ranges before downstream evaluation.

Each stage must report the ADR-0042 science-implementation,
calibration-evidence, and identifiability fields plus retained failures. It may
revisit an earlier stage only through an explicit finding and renewed joint
fit plan. When admitted data cannot separate an operand, CAL-04B must complete
the readiness matrix, sensitivity/synthetic-recovery evidence, equifinality,
and additional-data inventory rather than stopping solely for data scarcity.
Runoff, erosion, snow, and frost residuals cannot select canopy parameters.

CAL-05 then applies Elliot's source-to-equilibrium method to the frozen canopy
ensemble. It must adjudicate leaf-only CP2, recurring needle turnover,
fine-woody input, and decomposition separately. Source-flux sufficiency is
decided before decay is fitted to forest-floor stock. If no supported source
composition can reproduce both annual litter inputs and equilibrium storage,
CAL-05 must recommend a contract package rather than compensate with
unrealistic foliar mass or decay.

`CANOPY-CAL-05` identified missing predictive source physics. The bounded
`CANOPY-LITTER-SOURCE-AUTHORITY-01` successor subsequently implemented only
the supported authenticated external boundary and retained the predictive
authority hold.

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
- Calibration and holdout scores are reported separately; no holdout member
  contributes to fitting, parameter selection, or tolerance relaxation.
- Parameter uncertainty, correlations, boundary hits, and materially
  equifinal solutions are retained rather than collapsed into one optimum.

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

CAL-06 must publish the human-interpretation artifacts specified in the
[`CANOPY-CAL-06 figure contract`](canopy-cal-06-figure-contract.md). The
prospective [`experiment map`](figures/cal06-experiment-map.svg) shows the
current site/stratum matrix and gated consumer sequence; it is not a result.

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
4. **Bill Elliot method and reproduction** - source integrity, 92/95
   resolution, `NOT_REPRODUCIBLE` result, retained staged methodology, biomass
   and residue trajectories, equilibrium comparison, and claim limitations.
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

Order 4 is complete with its poor Harvard transferability result retained and
no refit.

Order 5, `CANOPY-CAL-05`, is complete. Under explicit operator direction,
`CANOPY-LITTER-SOURCE-AUTHORITY-01` completed authority synthesis and
implemented the supported identity-only prescribed/exhaustive-measured-daily
external boundary on 2026-07-28. Initial terminal findings were corrected;
both final independent re-reviews and both re-verifications passed, and the
package is closed.

Current literature and state still do not authorize predictive needle or
fine-woody ground-deposition laws. The external interface is a separate
exogenous input and retains predictive-source incompleteness. Orders 6-8 must
not infer unavailable source magnitudes, tissue partitions, carbon
conversions, turnover laws, or fitted decay. No additional article is
currently needed; lifting the predictive hold requires new species/site
process evidence and branch/crown/stand state, not another broad turnover
citation.

Order 6, `CANOPY-CAL-06`, is complete. All 261 prespecified runs passed and the
six required plot-only human-interpretation figures bind retained tidy tables,
with captions and ancillary scientific information in paired Markdown
sidecars. Winter
canopy ordering is stable across all 37 members for every source-supplied
within-site forest gradient. This is bounded model-response evidence, not
empirical canopy-amplitude validation. Harvard and Marcell snow residuals are
retained without canopy retuning; Harvard SWE is excluded because provider
centimeter metadata contradict the same-row depth-density identity.

Predictive needle and fine-woody sources remain null, residue/frost adequacy
does not advance, and ET/runoff/erosion consequences remain model-response or
not-evaluated evidence. No downstream residual selected a canopy parameter.

Order 7, `CANOPY-CAL-07`, froze two independent Southern Hemisphere
observational lanes and the complete accepted ensemble before result-bearing
execution. Execution then failed closed because the NASA POWER Alerce series
reconstructs three negative VPD days under OBL-PLANT-P-013: 2022-07-22,
2022-09-15, and 2025-09-09. The last occurs inside a scoring year. No clipping,
dew-point correction, day deletion, interpolation, or partial member/site
result was admitted.

Order 7 remains on scientific hold. Its immediate prerequisite is continuous,
contract-admissible meteorology for the Alerce lane or explicit science
authority defining a bounded canonicalization with threshold, units,
provenance, and tests. An article alone does not fill this data/authority gap.
Even after forcing admission, quantitative canopy amplitude/evergreen-floor
evidence and a phase-transformed real-consumer trajectory gate remain open.

`CANOPY-CAL-07B` fills the immediate source/operator diagnostic gap without
changing that hold. Exact hourly POWER products for the three Alerce failure
dates reconstruct positive paired hourly-product VPD for all 72 hourly rows,
while hourly-derived Tmin/Tmax/mean-dew operands reproduce the frozen CAL-07
daily operands within the prespecified serialized-resolution tolerance. The
additive decomposition attributes the sign change to the daily temperature
extrema summary term: -99.14 Pa, -116.89 Pa, and -59.96 Pa. Dew-point
nonlinearity is positive and smaller: +6.81 Pa, +5.89 Pa, and +1.37 Pa.
This supports `DAILY_SUMMARY_OPERATOR_MISMATCH` for the published POWER
product/operator combination, not clipping, canonicalization, production
operator replacement, or CAL-07 resumption.

`CANOPY-CAL-07C` then executed the bounded correction lane authorized by that
diagnostic. It retained the full-period Alerce POWER hourly object, counted 349
negative hourly paired-product components without clipping, and admitted the
daily signed mean only because all 1,666 admitted daily VPD values were finite
and nonnegative. The package-local executor consumed explicit `vpd_pa` and
`vpd_source` fields, proving zero VPD residual against the admitted source and
rejecting the three negative CAL-07 daily-summary dates. This lifts the
immediate Alerce input blocker only for CAL-07C research execution. Order 7
still does not pass: Beza transition chronology is contradicted, the combined
Southern Hemisphere seasonal-direction cell is contradicted, and quantitative
amplitude/floor and downstream litter/decomposition cells remain unevaluated.

`CANOPY-CAL-07D` investigated the retained Beza chronology contradiction
without refitting or changing production science. It independently
reconstructed all 61,642 Beza member-days with zero equation residual and
reproduced the same 11 of 148 absolute transition matches. Source-aligned
relative levels recovered 262 previously unmatched member/event/level rows,
showing that absolute `GSI21=0.5` exaggerates missing crossings, but no
date-50 rising transition matched and residuals remained directionally
displaced. At the 2024 and 2025 date-50 rises, median `GSI21` was only
`0.003` and `0.006`; photoperiod was the minimum constraint for 36/37 and
34/37 members, with VPD limiting the remainder. Counterfactual removal
establishes photoperiod/VPD sensitivity but cannot distinguish gridded-forcing
bias, tropical dry-forest threshold transfer, or a missing water/seasonal cue.
Those routes require new admitted evidence; Order 7 remains held.

`CANOPY-CAL-07E` reviewed direct-site, Madagascar regional, tropical dry-forest
mechanism, and PhenoCam method literature. Direct Bezà evidence supports
rainfall as a contributor and later leaf retention in gallery than xerophytic
forest. The paper proposes, but does not quantify, earlier dry-year and later
wet-year defoliation. It does not isolate VPD,
groundwater, soil water, temperature, or a unique trigger. Regional and
analogue evidence supports multiple species- and habitat-dependent water and
seasonal cues but cannot authorize a Bezà equation or parameter set.

The method review also found that CAL-07's simplified transition dates are
`gcc_mean`, while CAL-07D's ancillary daily GCC context is `smooth_gcc_90`.
The products agree within one day for rising transitions but differ by 12 to
43 days in material falling cases in the checksum-retained provisional record
processed 26 July 2026. This does not erase the broad chronology contradiction,
but falling residual magnitude is not product-invariant. A bounded CAL-07F
observation-product/operator audit is the next package.
Production correction, parameter fitting, and the Order 7 hold remain
unchanged.

`CANOPY-CAL-07F` executed the product-consistent stop-loss audit. The retained
daily curves reproduce all 24 product transitions; member ranks are identical
under `gcc_mean` and `gcc_90`. After excluding wrong-season recovery crossings,
no frozen member supplies all 12 seasonal transitions. Falling medians are
about 38 to 92 days early, rising T10 medians are about 45 to 83 days late,
and rising T50 is absent for every member in both years and products. The best
joint member hits one `gcc_mean` confidence interval and no `gcc_90`
intervals, with penalized errors of 59.12 and 65.87 days.

Only operator independence and mechanical year-role separation pass the six
prospective calibration criteria. Another parameter round is not recommended.
Further acquisition and canopy phenology work are deferred, and the retained
Bezà contradiction is adjudicated as an ecosystem-model limitation for this
tropical dry-forest lane. Order 7 remains not passed; reactivation requires
field-corresponded evidence, authoritative process science, or an independently
testable alternative ecosystem phenology formulation.
