# Constitutive Authority Admission

Status: `research-complete-admission-blocked`

Evidence mode: `Static`

Admission date: `not-admitted`

## Gate Verdict

The independent literature review is complete for the sources currently
available, but the full constitutive chain is not admissible. Radiation,
interception, wet-canopy depletion, and layer-participation formulations have
bounded candidates. Aerodynamic conductance, canopy/stomatal conductance, and
therefore potential transpiration remain blocked.

The contract-first gate is closed. No canonical contract amendment, authority
fixture, or production implementation may use the candidate rows below until
one coherent conductance route is admitted.

## Admission Matrix

| Family | Primary locator | Units/domain posture | Compatibility result | Disposition |
|---|---|---|---|---|
| Radiation receipt | R-137 Eq. 1 and Sect. 2.2 | component-specific incident radiation; homogeneous-canopy domain; dimensionless `k` and LAI | candidate top-down closure; sparse/heterogeneous domain unresolved | `CANDIDATE_NOT_ADMITTED` |
| Liquid interception | R-136 Sect. 4.1, Eqs. 46-47 | `R`, `TF`: kg m-2 s-1; `C`, `Cm`: kg m-2; explicit timestep | compatible with explicit storage proposal; Gash primary source still desirable | `CANDIDATE_NOT_ADMITTED` |
| Wet-canopy depletion | R-136 Sect. 2.3; R-140 | store-limited interval mass; aerodynamic regime must be explicit | compatible only after aerodynamic branch and energy operands are selected | `CANDIDATE_NOT_ADMITTED` |
| Aerodynamic conductance | R-129; R-136 Sect. 2.1; R-145 Sect. 3.2 | requires wind/reference height, roughness, displacement, stability, and canopy source height | no single bounded parameterization selected | `BLOCKED` |
| Canopy/stomatal conductance | R-138; R-139; R-132 Sect. 2.5.1; R-144 | leaf/canopy area and temporal scales differ among sources | mechanistic sources cross excluded scope; empirical sources lack sufficient compatible authority | `BLOCKED` |
| Potential transpiration | R-129 and R-132 | Penman-Monteith rate/energy conversion requires compatible resistances and interval integration | algebra available, constitutive inputs unresolved | `BLOCKED_BY_CONDUCTANCE` |
| Layer demand | R-136 Sect. 4.2, Eqs. 50-52 | non-negative normalized layer weights; zero denominator needs named behavior | actual-extraction source must be adjudicated for demand-request semantics | `CANDIDATE_NOT_ADMITTED` |

## Blocking Authority Questions

1. Will the package admit an explicitly empirical and domain-bounded
   Jarvis-Stewart conductance law, or widen to photosynthesis and plant
   hydraulics?
2. What primary formulation supplies aerodynamic conductance, including
   roughness/displacement, measurement-height, stability, and sparse/ventilated
   domain behavior?
3. If the empirical route is selected, what source governs leaf-to-canopy
   scaling, maximum conductance, response parameters, and transferability?
4. Can the JULES layer weighting be admitted as a pre-allocation demand
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

See `literature-review.md` for the full synthesis, rights matrix, incompatibility
analysis, and requested primary articles.

RHESSys source expression is prohibited evidence and was not inspected.
