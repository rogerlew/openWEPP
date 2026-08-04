# Native-Vegetation Evapotranspiration Process Model

## Status

- `state`: **concept** — high-priority native-vegetation science gap; not yet
  authorized for kernel implementation
- `date`: 2026-08-03
- `owner`: openWEPP maintainers + forest ecohydrology reviewer
- `origin`: Stevens Canyon burned-versus-undisturbed peak-flow inversion and
  ET-partition investigation in WEPPpy
- `promotion trigger`: authorize a contract-first work package after the
  Stevens Canyon fixture is vendored and at least one independent observed
  forest ET-partition dataset is admitted
- `default eligibility`: prohibited until native-process calibration,
  out-of-sample validation, conservation, and hydrologic-response gates pass

## Summary

Implement a native-vegetation evapotranspiration model that represents live
canopy transpiration, mineral-soil evaporation, and wet-canopy/forest-floor
interception evaporation as separate, conservative processes:

\[
ET = T_c + E_s + E_i
\]

The legacy WEPP ET pathways are agricultural models. The original pathway
partitions a common Penman or Priestley-Taylor demand using crop LAI, residue,
and a shallow drying-stage approximation. The optional `pmetpara.txt` pathway
uses FAO-56 dual crop coefficients (`kcb`, `rawp`). Neither pathway is a native
forest ET process model, and neither represents fire as loss of living canopy,
canopy conductance, interception storage, litter storage, and active roots.

This is not a request to retune `kcb`, `rawp`, LAI, residue, or soil parameters
until a desired annual ratio emerges. It is a request to establish an
independently authoritative native-vegetation process family in openWEPP.
Legacy behavior remains a diagnostic comparator and compatibility mode, not
scientific authority for native vegetation.

## Why This Exists

The Stevens Canyon fixture exposed a structural failure that cannot be repaired
by coefficient calibration:

- Current PMET produced burned-to-undisturbed median annual ET ratios of about
  `0.985`, `0.983`, and `0.849` for low, moderate, and high severity.
- Removing PMET and running both sides through legacy Penman produced ratios of
  `0.990`, `0.997`, and `0.862`.
- No severity produced one paired year inside both the diagnostic total-ET and
  `Es/ET` envelopes.
- Legacy Penman assigned undisturbed forest approximately `324 mm/year`
  entirely to plant-side ET, with median `Es=0` and `Er=0`.
- PMET shifted implausibly large post-fire fractions into soil evaporation;
  legacy Penman changed the partition but left low- and moderate-severity total
  ET effectively unchanged.

The shared failure is that canopy loss primarily reallocates atmospheric
demand instead of reducing realized total ET through loss of living leaf area,
stomatal conductance, interception capacity, and active roots. This matters
beyond ET reporting: excessive surface evaporation changes antecedent
shallow-soil water, saturation-excess runoff, hydrograph timing, deep
percolation, lateral flow, plant stress, and erosion.

The current `SC-EVAP-001` correctly documents migrated legacy behavior but is
not sufficient authority for a native forest implementation. Promotion of this
item must amend or supersede the relevant native-mode portions of
`SC-EVAP-001`; it must not silently redefine legacy-mode semantics.

## Scientific Position

No universal burn-severity ET equation was identified. Fire severity should
change physical state variables, not directly choose an arbitrary ET
multiplier. A credible model must allow the following combination:

- transpiration declines with living canopy and active-root loss;
- canopy and litter interception evaporation declines with storage loss;
- exposed-soil potential evaporation may increase immediately after fire;
- actual soil evaporation remains limited by ground-level energy, shallow
  water, and formation of a dry surface layer;
- total annual ET generally declines because increased soil evaporation does
  not replace lost transpiration and interception;
- all component withdrawals remain coupled to explicit water stores and close
  the water and energy budgets.

Burn severity is therefore an input-state transformation. It is not a fourth
ET formula and is not permission to tune a hidden total-ET scalar.

## Research Lineage

### Atmospheric demand

Penman (1948) combined available energy and aerodynamic drying. Priestley and
Taylor (1972) simplified the potential-evaporation problem for extensive,
saturated, weakly advected surfaces by multiplying equilibrium evaporation by
an empirical coefficient. Neither formula by itself partitions forest ET or
represents living-canopy loss.

Candidate atmospheric-demand implementations may use Penman or
Penman-Monteith meteorology, provided demand is subsequently allocated through
independent canopy and soil source resistances. Priestley-Taylor may be retained
as a low-input forcing option, but its coefficient cannot stand in for fire or
vegetation physiology.

### Two-source energy balance

Shuttleworth and Wallace (1985) developed a resistance-network combination
equation for sparse vegetation. It treats canopy transpiration and soil
evaporation as distinct sources and provides a physically interpretable
transition from bare ground to closed canopy. This is the preferred conceptual
foundation for openWEPP native vegetation.

### Forest interception

Gash (1979) separates canopy wetting, saturation, drainage, and post-storm
drying, with evaporation from a saturated canopy evaluated independently of
soil evaporation. An explicit storage implementation may use Rutter/Gash
lineage while retaining timestep memory. Forest-floor litter requires its own
bounded storage or a coupled implementation of the existing residue-moisture
backlog item.

### Soil evaporation

Ritchie-style stage-one/stage-two drying is a useful starting lineage, but
potential soil evaporation must be based on energy reaching the ground and
actual extraction must be constrained by a shallow surface-water state or
surface resistance. GLEAM provides a practical precedent: calculate potential
evaporation by cover class, use Gash interception, constrain transpiration by
root-zone water, and constrain bare-soil evaporation by surface soil water.

### Integrated precedents

- PT-JPL partitions Priestley-Taylor potential evaporation into canopy
  transpiration, soil evaporation, and interception using independent
  vegetation, temperature, wetness, and moisture constraints.
- NASA MOD16 represents wet-canopy evaporation, dry-canopy transpiration, and
  soil evaporation separately with biome-dependent conductance and stress
  parameters.
- GLEAM represents transpiration, bare-soil evaporation, open-water
  evaporation, interception, and sublimation as separate components.

These are design precedents, not code-vendoring targets. The first openWEPP
implementation should use the smallest process set that is conservative,
identifiable, hourly/daily compatible, and supported by admitted authority.

## Proposed Process Contract

### 1. Partition available energy

Use a Beer-Lambert canopy attenuation relationship as the initial candidate:

\[
A_s = (R_n-G)\exp(-k_R LAI_{live})
\]

\[
A_c = (R_n-G)-A_s
\]

where `A_s` and `A_c` are soil- and canopy-available energy. The final contract
must define net radiation, ground heat flux, extinction coefficient authority,
snow/winter interactions, timestep integration, and how woody area affects
attenuation when foliage is absent.

Required invariants:

- `A_s >= 0`, `A_c >= 0`, and `A_s + A_c = R_n - G` within tolerance;
- decreasing live LAI cannot decrease ground-level available energy under
  otherwise identical conditions;
- energy allocation cannot be counted independently by both sources.

### 2. Live-canopy transpiration

Use a canopy Penman-Monteith or Shuttleworth-Wallace term:

\[
\lambda T_c =
\frac{\Delta A_c + \rho c_p D/r_a^c}
{\Delta + \gamma(1+r_c/r_a^c)}
\]

with canopy resistance represented by living, active foliage:

\[
r_c = \frac{r_{leaf,min}}
{LAI_{active} f_\theta f_{VPD} f_T f_{rad}}
\]

Every stress function must be bounded in `[0,1]`, separately observable, and
supported by a forest or native-vegetation authority. Root-zone extraction must
remain layer-resolved and water-limited. `LAI_active=0` must yield zero
transpiration without numerical singularity or an implicit fallback demand.

The native-mode model must distinguish:

- live leaf area from standing dead material;
- canopy cover from physiologically active area;
- nominal root depth from active-root distribution;
- atmospheric demand from realized root uptake.

### 3. Mineral-soil evaporation

Calculate soil potential evaporation from ground-level energy and soil-source
resistances rather than as unused canopy demand:

\[
\lambda E_{s,pot} =
\frac{\Delta A_s + \rho c_p D/r_a^s}
{\Delta + \gamma(1+r_{ss}/r_a^s)}
\]

Then constrain it with surface water availability:

\[
E_s = \beta_s E_{s,pot}
\]

An initial bounded GLEAM-style candidate is:

\[
\beta_s = \operatorname{clip}\left(
\frac{\theta_1-\theta_{res}}
{\theta_{crit}-\theta_{res}},0,1\right)^p
\]

where `theta_1` is the water content of the explicitly defined evaporating
layer. The contract must adjudicate whether openWEPP retains a Ritchie drying
memory, uses a dry-layer resistance, or combines them without double counting.

Required behavior:

- soil evaporation responds strongly immediately after wetting;
- dry-period evaporation declines as the evaporating layer dries;
- repeated `10-15 mm/day` soil evaporation cannot occur without matching
  shallow-store recharge and sufficient ground-level energy;
- extraction never silently reaches below the authorized evaporation depth;
- soil evaporation cannot consume unmet transpiration demand merely because
  canopy LAI declined.

### 4. Wet-canopy and forest-floor interception evaporation

Represent interception as one or more explicit stores:

\[
\frac{dS_i}{dt}=I_i-E_i-D_i-X_i
\]

\[
0 \le S_i \le S_{i,max}
\]

Candidate capacity decomposition:

\[
S_{i,max}=s_L LAI_{live}+s_W WAI+S_{litter,max}
\]

where `WAI` is woody area index. Canopy and litter stores may require distinct
drainage and evaporation laws. Interception evaporation is limited by both the
wet-store content and wet-surface energy/aerodynamic demand. Drainage and
overflow return to the soil/runoff input surface explicitly.

This scope overlaps
[`20260512-residue-moisture-storage-full-state.md`](20260512-residue-moisture-storage-full-state.md).
Promotion must either absorb that item or define a single shared storage
contract so native ET does not create a second residue/litter water state.

### 5. Component and energy closure

At every timestep:

\[
ET = T_c + E_s + E_i
\]

All actual fluxes must correspond to withdrawal from a named store. The sum of
latent-energy consumption must not exceed the contract-defined available
energy plus explicitly modeled advection. Water-balance closure and component
closure must be independently reconstructable from diagnostic operands.

## Fire-Severity State Transformation

The initial contract should map fire severity and pre-fire vegetation state to
post-fire states, not directly to ET:

\[
LAI_{live,post}=f_{live}(severity,vegetation)LAI_{pre}
\]

\[
g_{c,post}=f_{conductance}(severity,vegetation)g_{c,pre}
\]

\[
S_{canopy,post}=f_{canopy}(severity)S_{canopy,pre}
\]

\[
S_{litter,post}=f_{litter}(severity)S_{litter,pre}
\]

\[
R_{active,post}(z)=f_{root}(severity,z)R_{active,pre}(z)
\]

Required additional states include albedo, canopy height, woody area, litter
mass/cover, and any soil-surface resistance or repellency state that affects
water availability. The transformation must preserve standing dead canopy
where appropriate while removing its transpiration capacity.

Recovery is separate from stationary first-year-equivalent validation. A
candidate recovery form is:

\[
X(t)=X_{post}+(X_{pre}-X_{post})(1-\exp(-t/\tau_X))
\]

but no recovery curve is authorized until vegetation-type and severity
authority exists. The Stevens Canyon 100-year stationary runs must not be
misinterpreted as a 100-year ecological recovery sequence.

## Diagnostic Output Requirements

Native ET development requires first-class daily and optional subdaily
diagnostics, not only final `Ep`, `Es`, and `Er`:

- above-canopy net radiation, ground heat flux, `A_c`, and `A_s`;
- atmospheric vapor-pressure deficit and aerodynamic resistances;
- live LAI, woody area, canopy cover, and active-root distribution;
- potential and actual `T_c`, `E_s`, and `E_i`;
- canopy and litter storage before/after, interception, drainage, and overflow;
- surface and root-zone moisture-stress factors;
- soil evaporating-layer water before/after and extraction depth;
- component water residual and latent-energy residual;
- reason-coded limitation attribution: energy, conductance, surface moisture,
  root supply, interception storage, snow, or frozen water.

Diagnostics must be available through openWEPP-owned outputs. A hidden
calibration-only trace cannot close the science or publication gates.

## WEPPpy Stevens Canyon Fixture Intake

### Upstream investigation authority

The source investigation is:

`/home/workdir/wepppy/docs/investigations/2026-08-03-stevens-canyon-peak-flow-inversion/`

Key committed artifacts are:

- `artifacts/hillslope-fixtures.md` — fixture contract and replay commands;
- `artifacts/setup_hillslope_fixtures.sh` — production-to-isolated-fixture
  intake recipe;
- `artifacts/add_high_severity_hillslope_fixture.py` — deterministic
  high-severity counterfactual builder;
- `artifacts/run_hillslope_fixtures.sh` — 39-run validator;
- `artifacts/et-calibration-targets.md` — diagnostic severity envelopes;
- `artifacts/legacy-et-ablation-results.md` and compact CSV outputs;
- `artifacts/pmet-calibration-results.md` and compact CSV outputs;
- `artifacts/soil-evaporation-code-trace.md` — legacy mechanism trace;
- `artifacts/water-balance-attribution.md` — hillslope flux attribution.

The generated local source root is:

`/wc1/ablation/stevens-canyon-peak-flow-20260803-hillslopes`

It contains H49-H61 for `undisturbed`, `burned`, and `high_severity`, including
management, soil, slope, climate, run controls, and the runtime sidecars
`gwcoeff.txt`, `snow.txt`, `pmetpara.txt`, `wepp_ui.txt`, `chntyp.txt`, `tc.txt`,
and `chan.inp`. H49 and H57 are unchanged controls. The native-ET severity
cohort is H50-H56 and H58-H61.

### Proposed openWEPP vendor destination

Vendor the minimal redistributable test cohort under:

`tests/fixtures/native_vegetation_et/stevens_canyon/`

The intake package must follow
[`reference-vendoring-policy.md`](../governance/reference-vendoring-policy.md)
and `tests/fixtures/AGENTS.md`. It must establish that public WEPPcloud project
inputs and derived fixture metadata are legally redistributable before
committing them. Ambiguous artifacts remain local and receive metadata-only
tracking.

The committed fixture should include:

1. The eleven treated hillslopes for undisturbed and their actual low/moderate
   burned counterparts, plus high-severity counterfactuals.
2. H49 and H57 only if retained as byte-identical intake controls.
3. Required `.run`, `.man`, `.slp`, `.cli`, and `.sol` inputs and all sidecars
   needed to preserve runtime selection, including `wepp_ui.txt`.
4. A machine-readable manifest recording upstream wepppy commit, source public
   run identifiers, hillslope role, area, soil texture, severity, file hashes,
   generation command, units, and rights classification.
5. Compact expected summaries for current PMET and legacy Penman behavior,
   explicitly labeled **diagnostic comparator outputs, not native truth**.
6. The target matrix and citations, labeled provisional until external
   authority promotion.

Do not vendor:

- the `wepp_260803_hill` executable or build artifacts;
- all 36,525-row outputs for every lane when compact annual/event evidence is
  sufficient;
- credentials, production NoDb state, unrelated watershed files, or absolute
  host paths as runtime dependencies;
- legacy values as pass/fail truth for the new native model.

### Intake provenance hashes

At backlog creation, the upstream recipes and summaries had these SHA-256
hashes:

| Artifact | SHA-256 |
|---|---|
| `setup_hillslope_fixtures.sh` | `77c8e865ee78b7a828691d9f4fbedc1aa7b19ba6e528bb9ed00f0481d2d180a9` |
| `add_high_severity_hillslope_fixture.py` | `6b229710f38dd6a6186fc358a9519b274e244c54525bbee110713131591b261e` |
| `run_hillslope_fixtures.sh` | `ce77df1a1ac665c88d9232594059fce23549be115050df458568bb21be48e604` |
| `legacy-et-ablation-summary.csv` | `989990b799159b438cfac15658a53444e2f4b888862394007cc0cf68febce7a3` |
| `pmet-calibration-summary.csv` | `8eceb31dbdecdc91b5695c73f6d6dbb2bc2714318fec0a8b58ccdb0a0a0de65d` |
| `et-calibration-targets.md` | `409988808efa00c011cd2f2b1d868f5dbf082f8590b5d8d89dea72e4aa57a3bb` |

These hashes identify the intake evidence only. Vendoring must recompute and
record every imported file hash at the source commit used by the intake
package.

## Calibration and Validation Strategy

### Calibration quantities

Calibration may use physically measurable quantities:

- minimum leaf stomatal resistance or maximum canopy conductance;
- radiation extinction coefficient;
- canopy and litter storage capacities;
- surface-soil critical moisture or dry-layer resistance parameters;
- active-root fraction and root-density profile;
- live-LAI and conductance survival fractions by severity;
- recovery timescales when recovery enters scope.

Do not calibrate soil conductivity, rooting, residue, or LAI to compensate for
an ET-process defect unless those quantities are independently observed or
constrained.

### Stevens Canyon diagnostic matrix

Retain the provisional first-year-equivalent screening envelopes:

| Severity | Burned / undisturbed ET | Burned `Es/ET` |
|---|---:|---:|
| Low | `0.65-0.80` | `0.15-0.30` |
| Moderate | `0.50-0.70` | `0.25-0.40` |
| High | `0.40-0.60` | `0.30-0.45` |

These are screening targets, not universal observations. The native model must
also produce nonzero and independently plausible undisturbed soil and
interception evaporation. Passing ratios with a physically false denominator
is a failure.

### Required independent authority

At least one calibration dataset and one held-out validation dataset must
provide multiple ET components or enough direct observations to constrain
them. Candidate sources include:

- concurrent above- and below-canopy eddy covariance;
- sap flow plus eddy covariance;
- throughfall/stemflow plus canopy-storage observations;
- lysimeter or soil-heat/soil-water constrained soil evaporation;
- pre-/post-fire energy and water balance with documented severity and canopy
  loss;
- watershed water-balance ET only as a total-ET constraint, not a partition
  authority.

The Sierra Nevada evidence that first-year ET declined approximately 31% after
low-severity fire and 50% after high-severity fire is an important regional
total-ET authority. The Hualo forest study supplies a component example in
which post-fire soil evaporation rose while transpiration and interception
loss fell enough for total ET to decline. Neither alone identifies every
process parameter.

## Falsifiable Acceptance Gates

### Constitutive gates

1. Dense-canopy shading reduces `A_s` monotonically without violating energy
   closure.
2. Zero live LAI yields zero transpiration while dead canopy may retain
   radiation/interception effects.
3. Soil evaporation pulses after wetting and becomes moisture-limited during
   dry-down without unaccounted deep extraction.
4. Interception evaporation cannot exceed intercepted storage plus same-step
   intake; drainage and overflow reach the named downstream water surface.
5. All stress factors are bounded and monotone in their declared driver over
   their applicability domain.

### Component and conservation gates

1. `T_c + E_s + E_i` reproduces published ET at output precision.
2. Each flux has an independently reconstructable water-store withdrawal.
3. Water-balance residual remains within the ratified tolerance at timestep,
   event, annual, and full-run scales.
4. Latent-energy use respects the ratified energy ceiling.
5. Diagnostics identify the controlling limitation without hidden fallback or
   clamping.

### Native-vegetation behavior gates

1. Undisturbed forest has nonzero plausible transpiration, soil evaporation,
   and interception evaporation; no component is forced to zero merely by an
   LAI threshold.
2. Fire severity reduces living-canopy transpiration and interception storage
   monotonically for fixed pre-fire vegetation.
3. Soil evaporation may increase after fire but cannot replace all lost
   transpiration/interception over the annual balance without observational
   authority.
4. Stevens Canyon low, moderate, and high cohorts are evaluated over all 100
   paired climate years with area weighting and reported uncertainty.
5. At least 80% of paired years satisfy the wider target tolerance defined by
   the upstream target artifact, or the target artifact is formally revised by
   stronger external authority before implementation tuning continues.
6. One or more external forest datasets pass without site-specific retuning.

### Hydrologic consequence gates

After ET calibration is frozen, rerun—not retune against—the following:

- antecedent shallow and full-profile soil water;
- runoff, percolation, and lateral-flow partition;
- the Stevens Canyon year-34/day-203 inversion event;
- multi-event runoff/peak timing behavior;
- erosion and sediment consequences where runoff changes materially.

Runoff is a validation consequence, not part of the ET calibration objective.

### Compatibility and publication gates

- Legacy ET and PMET compatibility modes remain explicitly selectable and
  unchanged unless a separate authorized retirement package says otherwise.
- Native vegetation uses an explicit model selector and typed parameter
  schema; absence of required native parameters fails closed.
- No new native process is inferred solely from the presence or absence of
  `pmetpara.txt`.
- Output schemas evolve additively until an explicit versioned cutover.
- The real hillslope CLI and output consumers must read and publish native
  component diagnostics before the feature can be called wired.

## Proposed Promotion Sequence

1. **Fixture intake:** vendor the Stevens Canyon input cohort and compact
   diagnostic evidence with provenance and rights review.
2. **Authority review:** admit primary literature and observed component
   datasets into `references/annotated_bibliography.md` and external-authority
   suites.
3. **Contract amendment:** author a native-vegetation amendment or successor to
   `SC-EVAP-001`, including state, units, equations, ordering, guards, outputs,
   and failure posture.
4. **Isolated kernels:** implement and test energy partition, canopy
   transpiration, soil dry-down, and interception storage independently.
5. **One-way diagnostic replay:** compute native ET beside existing runs
   without water-balance feedback to expose equation and parameter behavior.
6. **Conservative coupling:** enable native flux withdrawal in an experimental
   hillslope mode and prove water/energy/component closure.
7. **Calibration and holdout validation:** calibrate only admitted parameters,
   freeze them, and run held-out forest and post-fire data.
8. **Hydrologic consequence study:** evaluate soil moisture, runoff, lateral
   flow, routing, and erosion after ET parameters are frozen.
9. **Default consideration:** require independent science review, dual code/QA
   review, generated-output evidence, and explicit default-adoption authority.

## Non-Goals

- Do not fix legacy Fortran `evap` or `evappm` in this item.
- Do not make `kcb`, `rawp`, Priestley-Taylor alpha, or an ET-ratio multiplier
  the native fire model.
- Do not port PT-JPL, MOD16, GLEAM, or Shuttleworth-Wallace wholesale without
  an openWEPP-specific contract and dependency/parameter assessment.
- Do not tune runoff or peak flow until ET calibration is frozen.
- Do not treat legacy comparator parity as native-vegetation correctness.
- Do not claim universal severity coefficients from one watershed.
- Do not model chronological recovery in the stationary Stevens Canyon
  100-year climate ensemble.

## Risks and Open Questions

1. Can hourly WEPP climate forcing support stable two-source aerodynamic
   resistances without additional canopy-air temperature state?
2. Should the first implementation use full Shuttleworth-Wallace coupling or a
   simpler independently capped two-source Penman-Monteith approximation?
3. Which forest plant-functional types are required initially, and which
   conductance parameters have redistributable authority?
4. How should live LAI, dead foliage, woody area, and canopy cover be separated
   in existing management inputs?
5. Is the existing `cancov`/phenology machinery sufficiently authoritative to
   drive radiation and interception, or must it be extended first?
6. Should litter water state be delivered by the existing residue-storage
   backlog item as a prerequisite?
7. What evaporating-layer depth and moisture-resistance relationship is valid
   for ash-covered, water-repellent, and recovering post-fire soil?
8. How should snow-covered ground suppress soil evaporation and expose canopy
   sublimation without double counting the snow energy-balance program?
9. How should advective energy be diagnosed so an energy ceiling does not
   incorrectly reject real oasis or edge effects?
10. Which observed sites provide pre-fire, post-fire, and component-resolved ET
    with sufficient metadata for external-authority admission?

## References

- Penman, H. L. (1948). *Natural evaporation from open water, bare soil and
  grass.* Proceedings of the Royal Society A 193, 120-145.
  DOI: `10.1098/rspa.1948.0037`.
- Priestley, C. H. B., Taylor, R. J. (1972). *On the assessment of surface heat
  flux and evaporation using large-scale parameters.* Monthly Weather Review
  100, 81-92. DOI: `10.1175/1520-0493(1972)100<0081:OTAOSH>2.3.CO;2`.
- Shuttleworth, W. J., Wallace, J. S. (1985). *Evaporation from sparse crops—an
  energy combination theory.* Quarterly Journal of the Royal Meteorological
  Society 111, 839-855. DOI: `10.1002/qj.49711146910`.
- Gash, J. H. C. (1979). *An analytical model of rainfall interception by
  forests.* Quarterly Journal of the Royal Meteorological Society 105, 43-55.
  DOI: `10.1002/qj.49710544304`.
- Fisher, J. B., Tu, K. P., Baldocchi, D. D. (2008). *Global estimates of the
  land-atmosphere water flux based on monthly AVHRR and ISLSCP-II data,
  validated at 16 FLUXNET sites.* Remote Sensing of Environment 112, 901-919.
  DOI: `10.1016/j.rse.2007.06.025`.
- Martens, B., et al. (2017). *GLEAM v3: satellite-based land evaporation and
  root-zone soil moisture.* Geoscientific Model Development 10, 1903-1925.
  DOI: `10.5194/gmd-10-1903-2017`.
- Mu, Q., Zhao, M., Running, S. W. *MODIS Global Terrestrial
  Evapotranspiration Algorithm Theoretical Basis Document* and NASA MOD16 user
  guide: `https://modis-land.gsfc.nasa.gov/ET.html`.
- Zhang, L., Dawes, W. R., Walker, G. R. (2001). *Response of mean annual
  evapotranspiration to vegetation changes at catchment scale.* Water Resources
  Research 37, 701-708. DOI: `10.1029/2000WR900325`.
- Roche, J. W., Goulden, M. L., Bales, R. C. (2020). *Wildfire controls on
  evapotranspiration in California's Sierra Nevada.* Journal of Hydrology 590,
  125364. DOI: `10.1016/j.jhydrol.2020.125364`.
- White, D. A., et al. (2020). *The effect of wildfire on the structure and
  water balance of a high conservation value Hualo forest in central Chile.*
  Forest Ecology and Management 472, 118219.
  DOI: `10.1016/j.foreco.2020.118219`.

Rights status for full-text reference artifacts is `restricted` until reviewed
under the repository vendoring policy. Citations and DOI metadata may be
tracked immediately; PDFs must not be committed without redistribution rights.
