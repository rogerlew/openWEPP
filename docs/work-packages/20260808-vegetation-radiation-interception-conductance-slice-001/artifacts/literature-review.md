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
without crossing an excluded photosynthesis/plant-hydraulics boundary or
admitting an empirical remote-sensing surrogate.

Disposition is `AUTHORITY_PARTIAL`. Do not amend canonical constitutive
authority, create A3 fixtures, or begin production Rust until the conductance
route is selected and its primary sources are reviewed. This is not a lack-of-
calibration-data hold; it is a missing/contradictory science-authority hold at
the package's contract-first gate.

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
| R-128 Gash (1979) | metadata/abstract only | restricted | primary interception track-down item |
| R-141 Jarvis (1976) | remote full text | restricted | original empirical conductance track-down item |
| R-142 Stewart (1988) | metadata/abstract only | restricted | primary pine-forest conductance track-down item |
| R-143 Kelliher et al. (1995) | metadata/abstract only | restricted | maximum-conductance scale/aggregation track-down item |

Checksums and detailed rights bases are recorded in
`references/annotated_bibliography.md` and
`references/rights_classification_first_pass_2026-05-11.md`.

## Constitutive Coverage

| Required family | Best available locator | Candidate contribution | Disposition |
|---|---|---|---|
| Component-specific canopy radiation | R-137 Eq. 1 and Sect. 2.2; R-139 Sect. 2.1.3 | absorbed fraction `1 - exp(-k L)` and top-down layer receipt | `CANDIDATE`, domain-limited |
| Canopy liquid input/storage/release | R-136 Sect. 4.1, Eqs. 46-47 | finite-interval throughfall and storage update using explicit capacity | `CANDIDATE` |
| Wet-canopy evaporation | R-136 Sect. 2.3; R-140 | evaporation is capped by the available store; Penman-Monteith suitability depends on ventilation | `CANDIDATE`, branch details incomplete |
| Aerodynamic conductance | R-136 Sect. 2.1; R-129; R-140; R-145 Sect. 3.2 | Monin-Obukhov/resistance-network lineage and terrain/ventilation sensitivity | `BLOCKED` |
| Canopy/stomatal conductance | R-138; R-139 Sect. 2.1.3; R-132 Sect. 2.5.1; R-144 | mechanistic coupled options and empirical uncoupled options | `BLOCKED` |
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
from transpiration and using the Gash lineage, but its overview paper does not
provide enough equation detail to replace the Gash primary article.

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
2. The Jarvis-Stewart family multiplies a maximum/reference conductance by
   radiation, humidity-deficit, temperature, and water-stress responses.
   Stewart evaluated a stand-scale form for pine forest. However, Cain reports
   weak parameter transfer between forests, and Misson et al. found systematic
   vapor-pressure-deficit error for a Jarvis implementation in drought-stressed
   ponderosa pine. Misson et al.'s better-performing alternative was coupled to
   photosynthesis.

Consequently, neither empirical route can be admitted merely because it fits
the current package boundary. Doing so would be a surrogate-physics decision.

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

1. **Empirical bounded slice**: admit a Jarvis-Stewart-style conductance only
   after reviewing the original Stewart/Jarvis equations, scale conversion,
   parameter provenance, forest/domain limits, failure behavior, and the
   counterevidence. This route remains an empirical constitutive law and cannot
   be described as mechanistic forest physiology.
2. **Mechanistic forest slice**: amend scope to include photosynthesis and
   plant hydraulics, then select a coupled formulation from the JULES/Bonan/4C
   family. This is scientifically stronger but materially larger than the
   current package.

MOD16 is retained as a comparison source, not a default third route.

## Requested Articles

Operator help would materially improve the authority decision. Priority order:

1. Stewart (1988), DOI `10.1016/0168-1923(88)90003-2` - full equation set,
   parameter table, canopy scaling, calibration/validation split, and domain.
2. Kelliher et al. (1995), DOI `10.1016/0168-1923(94)02178-M` - maximum leaf
   versus bulk conductance data and aggregation assumptions.
3. Jarvis (1976), DOI `10.1098/rstb.1976.0035` - original response-function
   definitions and assumptions.
4. Gash (1979), DOI `10.1002/qj.49710544304` - original event-interception
   equations and assumptions; lower urgency because open sources already
   support a finite-timestep storage candidate.

Publisher PDFs or author manuscripts are useful. Unless the supplied artifact
states an affirmative redistribution license, place it under
`references/copyrighted/`; do not add it to `references/vendorable/` or Git.

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

Continue only with source acquisition and equation-level adjudication. The
contract-first production gate remains closed. No production Rust, A3 fixture,
or canonical constitutive amendment is authorized by this review alone.
