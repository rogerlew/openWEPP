# Canopy Sky-View Derivation

Status: `complete / admitted`.

Evidence class: Static + Ran analytical reconstruction.

## Decision

Use:

```text
f_sky = (1 - C)^1.6
```

where `C` is openWEPP's existing effective overhead canopy-cover fraction and
`f_sky` is the hemispherical diffuse sky-view/transmission factor.

## Derivation

FSM2 Eq. 13 gives one-layer beam transmission:

```text
tau_b(theta) = exp(-k_ext VAI_eff / cos(theta))
```

At nadir (`theta=0`), identify the complement of effective overhead
interception with the vertical gap:

```text
1 - C = exp(-k_ext VAI_eff).
```

FSM2 Eq. 14 approximates hemispherical diffuse transmission:

```text
tau_d = exp(-1.6 k_ext VAI_eff).
```

Eliminating the shared unobserved state `k_ext VAI_eff` gives:

```text
tau_d = exp(1.6 ln(1-C)) = (1-C)^1.6.
```

This elimination is an `[INFERENCE][Static]` model-state translation joining
the direct FSM2 equations to openWEPP's canonical effective-cover meaning. It
is not an empirical fit.

The translation is valid only as an equivalent horizontally homogeneous,
random-orientation one-layer canopy beneath an isotropic diffuse sky. It
inverts the native structural floor as effective vertical optical depth; it
does not claim that floor is a measured stem-area index. Directional crowns,
explicit gaps/edges/trunks, terrain horizons, and anisotropic diffuse light
remain outside the admitted regime.

## Why this uses the state we already have

The native canopy realization computes:

```text
LAI = maximum_LAI * foliar_activity
foliar_cover = 1 - exp(-cover_coefficient * live_foliar_biomass)
C = min(0.999, max(structural_cover_floor, foliar_cover))
```

Thus `C` already carries seasonal foliage and a leaf-off structural floor.
Adding LAI again would double count foliage. Adding structural cover to LAI
would add a fraction to an area index and incorrectly treat a cover floor as
stem-area index. Height is not an independent operand in the homogeneous
Beer-law transmission equations.

## Rejected alternatives

| Candidate | Disposition | Reason |
|---|---|---|
| `f_sky = 1-C` | reject | Directly aliases vertical gap to hemispherical sky view and ignores oblique diffuse paths. |
| `VAI_eff = LAI + structural_cover` | reject | Adds quantities with different meanings and double counts canopy state. |
| `VAI_eff = max(LAI, -ln(1-C)/k)` | reject | Unsourced heuristic branch and requires an extinction coefficient. |
| height correction multiplied into sky view | reject | No admitted one-layer source equation defines that operation. |
| new user-entered sky view or remote observation | reject | Unnecessary for the deterministic canonical route and contrary to the product requirement. |

## Analytical checks

`tools/execute.py` compares the evaluator to immutable expected sky-view
values at `C=0`, `0.2`, `0.5`, `0.9`, and `0.999` using the contract
tolerance; invalid, closed, and non-finite cover cases are executed and
rejected. The response is bounded and strictly decreasing on the valid domain.

See `figures/eb02-sky-view-response.svg` and its same-stem Markdown sidecar.
