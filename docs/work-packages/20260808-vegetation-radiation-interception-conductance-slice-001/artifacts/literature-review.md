# Independent Literature Review

Status: `research-complete-authority-partial`

Evidence mode: `Static`

Review date: `2026-08-08`

## Verdict

The literature intake supports bounded candidate equations for homogeneous-
canopy radiation receipt, explicit liquid-canopy storage and throughfall,
store-limited wet-canopy evaporation, and normalized layer participation.
It does **not** yet support one internally coherent production chain satisfying
the package's conductance, potential-transpiration, and aerodynamic requirements
without either crossing an excluded photosynthesis/plant-hydraulics boundary or
making an explicit domain-limited empirical-authority decision.

Disposition is `AUTHORITY_PARTIAL`. Do not amend canonical constitutive
authority, create A3 fixtures, or begin production Rust until the conductance
route and its intended domain are selected. The requested primary sources have
now been reviewed. This is not a lack-of-calibration-data hold; it is an
unresolved science-authority choice at the package's contract-first gate.

## Review Boundary

- Independently inspected peer-reviewed articles, official algorithm
  documentation, and institutional report copies only.
- Did not inspect RHESSys source expression, constants, naming, control flow,
  comments, or reversible pseudocode.
- Did not treat citations inherited from RHESSys or the predecessor discovery
  ledger as authority.
- Did not inspect or port any other implementation source code.
- Did not select physiological defaults or calibration bounds.

## Source And Rights Matrix

| Reference | Full text | Rights posture | Review use |
|---|---|---|---|
| R-136 Best et al. (2011), JULES Part 1 | vendored | CC BY 3.0 | explicit interception/storage and layer extraction; conductance incompatibility |
| R-137 Forrester et al. (2014) | vendored | CC BY 4.0 | explicit forest Lambert-Beer receipt and heterogeneity limits |
| R-138 Bonan et al. (2014) | vendored | CC BY 3.0 | mechanistic conductance scope and hydraulic coupling |
| R-131 Martens et al. (2017), GLEAM v3 | vendored | CC BY 3.0 | independent process separation and Gash lineage |
| R-139 Lasch-Born et al. (2020), 4C v2.2 | vendored | CC BY 4.0 | forest process topology and photosynthesis coupling |
| R-129 Shuttleworth and Wallace (1985) | local cache | rights not affirmative | two-source resistance network and Penman-Monteith lineage |
| R-132 MOD16 User Guide v2.2 | local cache | rights not affirmative | empirical no-photosynthesis candidate and incompatibilities |
| R-140 Pereira et al. (2016) | local cache | CC BY-NC-ND 4.0 | wet-canopy aerodynamic-regime evidence |
| R-145 Cain (1998) | local cache | Institute copyright | secondary discovery and parameter-transfer warnings |
| R-144 Misson et al. (2004) | remote repository full text | publisher copyright | ponderosa-pine comparison and counterevidence |
| R-128 Gash (1979) | local PDF and Markdown cache | restricted | primary event-interception equations and assumptions |
| R-141 Jarvis (1976) | local PDF and Markdown cache | Royal Society copyright | original leaf-response equations and parameter-fitting limits |
| R-142 Stewart (1988) | local PDF and Markdown cache | restricted Elsevier article | pine-forest surface-conductance equations and validation |
| R-143 Kelliher et al. (1995) | local PDF and Markdown cache | restricted Elsevier article | leaf/canopy/surface scaling and maximum-conductance synthesis |

Checksums and detailed rights bases are recorded in
`references/annotated_bibliography.md` and
`references/rights_classification_first_pass_2026-05-11.md`.

## Constitutive Coverage

| Required family | Best available locator | Candidate contribution | Disposition |
|---|---|---|---|
| Component-specific canopy radiation | R-137 Eq. 1 and Sect. 2.2; R-139 Sect. 2.1.3 | absorbed fraction `1 - exp(-k L)` and top-down layer receipt | `CANDIDATE`, domain-limited |
| Canopy liquid input/storage/release | R-136 Sect. 4.1, Eqs. 46-47; R-128 Sects. 3-6 | finite-interval storage candidate plus event-scale wetting/saturation/stemflow authority | `CANDIDATE` |
| Wet-canopy evaporation | R-136 Sect. 2.3; R-140 | evaporation is capped by the available store; Penman-Monteith suitability depends on ventilation | `CANDIDATE`, branch details incomplete |
| Aerodynamic conductance | R-136 Sect. 2.1; R-129; R-140; R-145 Sect. 3.2 | Monin-Obukhov/resistance-network lineage and terrain/ventilation sensitivity | `BLOCKED` |
| Canopy/stomatal conductance | R-141 Eqs. 4-9; R-142 Eqs. 9-24; R-143 Eqs. 3-11; R-138; R-139; R-144 | explicit empirical chain is available but domain-limited; mechanistic alternatives cross scope | `CANDIDATE_DOMAIN_LIMITED`, not admitted |
| Potential transpiration | R-129; R-132 Sects. 2.4-2.5 | Penman-Monteith canopy demand after conductance selection | `BLOCKED_BY_CONDUCTANCE` |
| Layer participation/demand | R-136 Sect. 4.2, Eqs. 50-52 | non-negative normalized root-share times moisture-availability weighting | `CANDIDATE`, semantic review required |

## Radiation Findings

Forrester et al. Eq. 1 supplies the homogeneous-canopy absorbed fraction
`f = 1 - exp(-k L)`. Its layer method proceeds from the highest layer downward,
reducing the incident component by receipt in higher layers. A conservation-
preserving stratum form can therefore be derived by differencing cumulative
transmission at adjacent leaf-area boundaries, but that derivation has not yet
been admitted into `SC-VEGETATION-001`.

The source also prevents three tempting overextensions:

- `k` is not a universal constant. It depends on species, crown architecture,
  canopy structure, and the time/illumination basis over which it was inferred.
- A homogeneous Lambert-Beer canopy is not automatically valid for sparse,
  gapped, disturbed, or strongly mixed canopies.
- PAR receipt cannot silently alias broadband shortwave or net radiation. The
  API and contract must preserve the radiation component and energy basis.

The JULES table value `k = 0.5` and 4C's assumption that 50% of global
radiation is PAR are model settings, not transferable openWEPP defaults.

## Interception Findings

Best et al. Eqs. 46-47 give an explicit finite-timestep relation among rainfall
rate, canopy storage, holding capacity, throughfall, and the next canopy-water
state, all on a mass-per-horizontal-area basis. Section 2.3 states that
wet-canopy evaporation depletes the store, cannot exceed water available in the
timestep, and must hand any remaining atmospheric demand to a resisted
transpiration pathway while retaining energy-balance consistency.

This is materially closer to the package boundary than MOD16, whose wet-canopy
fraction is diagnosed from relative humidity rather than from a prognostic
liquid store. GLEAM independently corroborates separating interception loss
from transpiration and using the Gash lineage.

Gash now supplies the primary event-scale equations. It divides events into
storms that do and do not saturate the canopy, explicitly accounts for free
throughfall, canopy and trunk storage, stemflow, wetting-up, saturated
evaporation, and post-event drying, and derives the rainfall needed for canopy
saturation in Eq. 13. Its central assumption is that discrete storms are
separated by intervals long enough for canopy and trunk stores to dry; mean
rainfall and evaporation rates replace within-storm variation. Gash therefore
strengthens event-scale authority and tests, but it cannot be silently treated
as the update law for an arbitrary finite timestep with carry-over storage.

Pereira et al. establishes a required regime distinction: a Penman-Monteith
wet-canopy calculation is defensible for canopies that are not fully ventilated,
while very sparse, fully ventilated canopies may require a wet-bulb approach.
Canopy cover alone is not sufficient to select the regime. A single universal
wet-canopy evaporation branch would therefore be overbroad.

## Conductance Findings

The decisive literature result is a scope conflict, not a missing coefficient.

JULES, Bonan et al., and 4C all connect defensible forest stomatal conductance
to photosynthesis and, in the more mechanistic cases, to plant hydraulic safety.
Those mechanisms are expressly excluded from this package. Bringing them in
would require a prospective scope amendment and a larger constitutive package.

Two uncoupled empirical alternatives were examined:

1. MOD16 scales a biome-specific potential leaf conductance with piecewise
   minimum-temperature and vapor-pressure-deficit multipliers. It is a daily
   remote-sensing algorithm, uses biome lookup parameters, diagnoses canopy
   wetness from humidity, and omits the explicit soil-water/leaf-water chain
   needed by this slice. Combining it with JULES storage and root weighting
   would mix scales and state definitions without primary compatibility
   evidence.
2. Jarvis Eqs. 4-9 multiply leaf-scale maximum conductance by bounded light,
   temperature, vapour-pressure-deficit, leaf-water-potential, and carbon-
   dioxide responses. Jarvis calls the model descriptive, identifies the
   product interaction as the simplest hypothesis needing further tests, and
   reports parameter differences between Sitka spruce and Douglas fir data.
   This is primary authority for the functional family, not for canopy scaling
   or universal coefficients.

Stewart supplies the missing stand-scale bridge for a dry pine canopy. Its
Eq. 12 is `g_s = L K_1 g(S_t) g(delta q) g(T) g(delta theta)`, with the
nonlinear response functions in Eqs. 17-24. The derived surface conductance can
include litter and understorey evaporation, so it is not automatically equal
to pine stomatal conductance. Alternate-day calibration/validation on the same
three-year sample reproduced summed transpiration within 1%, but fitting 1976
and testing 1974/1975 underestimated totals by 14% and 11% and biased hourly
extremes. The paper attributes this to a missing variable or year-varying
responses.

Kelliher et al. makes the scale distinctions explicit. Canopy conductance is
the parallel integral of leaf stomatal conductance over LAI (Eq. 3); a
light-attenuated hyperbolic response yields Eqs. 4-7; and bulk surface
conductance inferred by Penman-Monteith may exceed canopy conductance at low
LAI because soil evaporation contributes (Eqs. 8-10). Its cross-study maximum
values are observational summaries under optimum conditions, not general
response parameters or defaults.

Consequently, the primary-source gap is closed, and Jarvis-Stewart is now a
real `CANDIDATE_DOMAIN_LIMITED` route. It still cannot be admitted merely
because it fits the package boundary. Admission would have to state that it is
an empirical dry-canopy law, define whether the API represents leaf, canopy, or
bulk surface conductance, bind parameter provenance and calibration domain,
map soil state to Stewart's metre-integrated deficit without aliasing leaf
water potential, and reject unsupported transfer. That is an authority/domain
decision, not a missing-article problem.

## Aerodynamic And Potential-Demand Findings

Shuttleworth-Wallace and the MOD16 documentation support a Penman-Monteith
resistance-network calculation once energy, vapor-pressure deficit,
aerodynamic resistance, and canopy/surface resistance are defined on compatible
scales. The canopy-only slice must not inherit the soil-source terms of the
two-source formulation.

Aerodynamic resistance is not a harmless fixed input. The reviewed sources
tie it to roughness and displacement lengths, wind measurement height,
stability corrections, canopy source height, and sometimes terrain and
ventilation regime. Pereira et al. and related forest evidence show that a
misestimated aerodynamic conductance can dominate wet-canopy evaporation.
The current intake has not selected a domain-bounded aerodynamic formulation
or its parameter authority.

Potential transpiration therefore remains blocked even though the
Penman-Monteith algebra is well established: the equation cannot be made
authoritative while its canopy and aerodynamic resistances are unresolved.

## Layer-Demand Findings

Best et al. Eqs. 50-52 allocate a total transpiration flux using a normalized
product of layer root fraction and a piecewise soil-moisture availability
factor. The normalization gives non-negative layer shares summing to one when
at least one layer has a positive weight.

Two package-specific issues require contract review:

- JULES derives root fraction from rooting depth, whereas this package requires
  an explicit caller-supplied layer profile and forbids silently expanding a
  depth into layer participation.
- JULES describes actual extraction. The package may return demand requests
  only; Stage B remains the authority for allocation and withdrawal. Reusing
  the normalized weighting as a request partition is a semantic adaptation,
  not a verbatim source transfer.

If admitted, the zero-denominator case must be a named branch: zero total
demand may return all-zero layer demand, while positive total demand with no
participating/available layer must fail or follow another explicitly sourced
rule. Silent normalization is prohibited.

## Compatibility Risks To Resolve

- PAR, broadband shortwave, absorbed net radiation, and available latent energy
  have different operands and cannot share one scalar without lineage.
- Leaf stomatal conductance, canopy conductance, bulk surface conductance, and
  aerodynamic conductance have different area bases and scaling rules.
- Daily MOD16 multipliers cannot be silently reused at an arbitrary subdaily
  timestep.
- Humidity-diagnosed wet fraction is not equivalent to explicit canopy liquid
  storage.
- Actual layer extraction is not automatically an authority for pre-allocation
  layer demand.
- Sparse-canopy soil evaporation cannot be folded into canopy transpiration;
  it is excluded and belongs to a later coupled package.
- Parameter sets calibrated within one resistance formulation or forest site
  are not transferable by default.

## Authority Decision Needed

The next contract step must choose one of two honest routes:

1. **Empirical bounded slice**: prospectively constrain the slice to a named
   dry-canopy forest domain, then admit the reviewed Jarvis-Stewart equation
   family with an explicit Kelliher-consistent scale definition, parameter
   provenance, state mapping, failure behavior, and transferability limits.
   This route remains empirical and cannot be described as mechanistic forest
   physiology.
2. **Mechanistic forest slice**: amend scope to include photosynthesis and
   plant hydraulics, then select a coupled formulation from the JULES/Bonan/4C
   family. This is scientifically stronger but materially larger than the
   current package.

MOD16 is retained as a comparison source, not a default third route.

## Acquired Primary Articles

The operator supplied the requested Gash (1979), Jarvis (1976), Stewart (1988),
and Kelliher et al. (1995) PDFs and Markdown transcriptions on 2026-08-08. All
eight artifacts were identity- and checksum-verified and placed under the
gitignored `references/copyrighted/` cache because none carries affirmative
redistribution permission. No further article track-down is needed for this
specific empirical-route adjudication.

## Documentation Validation

- Ran `markdown-doc lint --path
  docs/work-packages/20260808-vegetation-radiation-interception-conductance-slice-001
  --format plain`: 40 files validated, 0 errors, 0 warnings.
- Ran `markdown-doc lint` separately on the annotated bibliography and rights
  classification: each validated with 0 errors and 0 warnings.
- Ran `git diff --check`: pass.
- Ran repository-wide `markdown-doc lint --path docs --format plain`: 19,544
  files validated; 15 pre-existing broken-link errors remain in unrelated
  historical/backlog paths. No error names a file changed by this intake.

## Next Permitted Action

Select the intended conductance route and domain, then perform the formal
contract-level compatibility admission, including aerodynamic conductance.
The contract-first production gate remains closed. No production Rust, A3
fixture, or canonical constitutive amendment is authorized by this review
alone.

## Supersession Addendum

On 2026-08-08 the user selected the broader source-aware route based on pinned
MIT-licensed RHESSysEastCoast and GIS2RHESSys repositories, with required
deciduous and mixed-forest support. This resolves the package-level choice by
superseding this narrow implementation scope; it does not retroactively admit
any candidate in this literature matrix. The independent review remains
scientific counterevidence and parameter/domain context for
`../../20260808-rhessys-east-coast-coupled-vegetation-slice-001/package.md`.
