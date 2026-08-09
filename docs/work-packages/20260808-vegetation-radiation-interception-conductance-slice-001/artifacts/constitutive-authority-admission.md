# Constitutive Authority Admission

Status: `research-complete-admission-blocked`

Evidence mode: `Static`

Admission date: `not-admitted`

## Gate Verdict

The independent literature review, including the four requested primary
articles, is complete, but the full constitutive chain is not admissible.
Radiation, interception, wet-canopy depletion, layer participation, and an
empirical domain-limited conductance formulation have bounded candidates.
Aerodynamic conductance and the authority decision governing conductance scale,
domain, and state mapping remain blocked; therefore potential transpiration is
also blocked.

The contract-first gate is closed. No canonical contract amendment, authority
fixture, or production implementation may use the candidate rows below until
one coherent conductance route is admitted.

## Admission Matrix

| Family | Primary locator | Units/domain posture | Compatibility result | Disposition |
|---|---|---|---|---|
| Radiation receipt | R-137 Eq. 1 and Sect. 2.2 | component-specific incident radiation; homogeneous-canopy domain; dimensionless `k` and LAI | candidate top-down closure; sparse/heterogeneous domain unresolved | `CANDIDATE_NOT_ADMITTED` |
| Liquid interception | R-136 Sect. 4.1, Eqs. 46-47; R-128 Sects. 3-6 | `R`, `TF`: kg m-2 s-1; `C`, `Cm`: kg m-2; explicit timestep; Gash uses discrete dry-separated storms | explicit storage candidate is compatible; Gash supports event wetting/saturation terms but not arbitrary timestep carry-over | `CANDIDATE_NOT_ADMITTED` |
| Wet-canopy depletion | R-136 Sect. 2.3; R-140 | store-limited interval mass; aerodynamic regime must be explicit | compatible only after aerodynamic branch and energy operands are selected | `CANDIDATE_NOT_ADMITTED` |
| Aerodynamic conductance | R-129; R-136 Sect. 2.1; R-145 Sect. 3.2 | requires wind/reference height, roughness, displacement, stability, and canopy source height | no single bounded parameterization selected | `BLOCKED` |
| Canopy/stomatal conductance | R-141 Eqs. 4-9; R-142 Eqs. 9-24; R-143 Eqs. 3-11; R-138; R-139; R-144 | Jarvis is leaf scale; Stewart is dry-canopy bulk surface scale for one pine stand; Kelliher distinguishes leaf, canopy, and surface scales | empirical equations are primary-source supported but require a named domain, scale, state mapping, and parameter provenance; mechanistic sources cross excluded scope | `CANDIDATE_DOMAIN_LIMITED_NOT_ADMITTED` |
| Potential transpiration | R-129 and R-132 | Penman-Monteith rate/energy conversion requires compatible resistances and interval integration | algebra available, constitutive inputs unresolved | `BLOCKED_BY_CONDUCTANCE` |
| Layer demand | R-136 Sect. 4.2, Eqs. 50-52 | non-negative normalized layer weights; zero denominator needs named behavior | actual-extraction source must be adjudicated for demand-request semantics | `CANDIDATE_NOT_ADMITTED` |

## Blocking Authority Questions

1. Will the package narrow to a named dry-canopy domain and admit the explicitly
   empirical Jarvis-Stewart conductance family, or widen to photosynthesis and
   plant hydraulics?
2. What primary formulation supplies aerodynamic conductance, including
   roughness/displacement, measurement-height, stability, and sparse/ventilated
   domain behavior?
3. If the empirical route is selected, will the API expose leaf, canopy, or
   bulk surface conductance; how will Kelliher's scale distinctions be enforced;
   and what calibration provenance and transfer limits bind each parameter?
4. What explicit mapping, if any, relates openWEPP soil-layer observations to
   Stewart's integrated soil-moisture deficit without pretending that it is
   Jarvis leaf water potential?
5. Can the JULES layer weighting be admitted as a pre-allocation demand
   partition without claiming that it is actual extraction?

## Prohibited Interim Choices

- Do not use `k = 0.5`, a MOD16 biome row, or a literature-average maximum
  conductance as a production default.
- Do not mix MOD16 humidity-diagnosed wetness with JULES prognostic canopy
  storage.
- Do not treat a daily remote-sensing multiplier as timestep-independent.
- Do not relabel leaf conductance as canopy or bulk surface conductance.
- Do not implement a surrogate Penman-Monteith chain while either resistance
  remains unresolved.

See `literature-review.md` for the full synthesis, rights matrix, primary-source
adjudication, and incompatibility analysis.

RHESSys source expression is prohibited evidence and was not inspected.

## Supersession Addendum

This admission verdict remains historically accurate for the narrow package:
no candidate was admitted and its contract-first gate never opened. The user
subsequently selected the MIT-licensed RHESSysEastCoast/GIS2RHESSys source-aware
successor. Source inspection and coupled scope are authorized only in that new
package; this artifact must not be read as prohibiting its pinned licensed
sources or as admitting their behavior without contract adjudication.
